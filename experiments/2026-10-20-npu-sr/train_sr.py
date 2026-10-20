# 用原神真机截帧训练 2 倍超分 (ESPCN 型), 导出给 NPU 的 int8 TFLite
# 低分辨率输入 = 原帧 2x2 平均下采样 (模拟半分辨率渲染), 目标 = 原帧
# 用法: python -I train_sr.py <帧目录> <输出目录> [c1 c2 步数]
import glob, os, sys, json
import numpy as np
import tensorflow as tf
from PIL import Image

src, out = sys.argv[1], sys.argv[2]
c1, c2, steps = (int(x) for x in (sys.argv[3:6] if len(sys.argv) > 5 else (32, 32, 4000)))
os.makedirs(out, exist_ok=True)
H, W = 488, 1072                      # 部署输入
frames = sorted(glob.glob(f"{src}/ref_*.png"))
test_names = frames[-4:]              # 最后 4 张留作测试
train_names = frames[:-4]

def load(p):
    a = np.asarray(Image.open(p).convert("RGB"), np.float32) / 255.0
    h, w = a.shape[0] // 2 * 2, a.shape[1] // 2 * 2
    return a[:h, :w]

def down(a):
    return a.reshape(a.shape[0] // 2, 2, a.shape[1] // 2, 2, 3).mean((1, 3))

train = [load(p) for p in train_names]
test = [load(p) for p in test_names]

def model(h, w):
    x = inp = tf.keras.Input((h, w, 3))
    x = tf.keras.layers.Conv2D(c1, 5, padding="same", activation="relu", name="c1")(x)
    x = tf.keras.layers.Conv2D(c2, 3, padding="same", activation="relu", name="c2")(x)
    x = tf.keras.layers.Conv2D(12, 3, padding="same", name="c3")(x)
    x = tf.keras.layers.Lambda(lambda t: tf.nn.depth_to_space(t, 2))(x)
    # 残差: 模型只学双线性放大缺的细节
    up = tf.keras.layers.UpSampling2D(2, interpolation="bilinear")(inp)
    x = tf.keras.layers.Add()([up, x])
    return tf.keras.Model(inp, x)

rng = np.random.default_rng(0)
P = 48                                 # 低分辨率块大小
def batch(n=32):
    xs, ys = [], []
    for _ in range(n):
        a = train[rng.integers(len(train))]
        y0 = rng.integers(0, a.shape[0] // 2 - P) * 2
        x0 = rng.integers(0, a.shape[1] // 2 - P) * 2
        hr = a[y0:y0 + 2 * P, x0:x0 + 2 * P]
        xs.append(down(hr)); ys.append(hr)
    return np.stack(xs), np.stack(ys)

m = model(None, None)
opt = tf.keras.optimizers.Adam(tf.keras.optimizers.schedules.CosineDecay(1e-3, steps))
m.compile(opt, "mae")
for s in range(steps):
    x, y = batch()
    loss = m.train_on_batch(x, y)
    if s % 500 == 0:
        print("step", s, "loss", round(float(loss), 5), flush=True)

def psnr(a, b):
    return float(10 * np.log10(1.0 / np.mean((np.clip(a, 0, 1) - b) ** 2)))

def crop(a):                           # 测试统一用部署尺寸的中心区域
    y0 = (a.shape[0] - 2 * H) // 2; x0 = (a.shape[1] - 2 * W) // 2
    return a[y0:y0 + 2 * H, x0:x0 + 2 * W]

res = {"bilinear": [], "bicubic": [], "model_fp32": []}
lows = []
for a in test:
    hr = crop(a); lr = down(hr); lows.append(lr)
    res["bilinear"].append(psnr(tf.image.resize(lr[None], (2 * H, 2 * W), "bilinear")[0].numpy(), hr))
    res["bicubic"].append(psnr(tf.image.resize(lr[None], (2 * H, 2 * W), "bicubic")[0].numpy(), hr))
    res["model_fp32"].append(psnr(m.predict(lr[None], verbose=0)[0], hr))

# 部署: 固定输入尺寸, uint8 进出, 全 int8
d = model(H, W)
d.set_weights(m.get_weights())
def rep():
    for a in train:
        y0 = (a.shape[0] - 2 * H) // 2; x0 = (a.shape[1] - 2 * W) // 2
        yield [down(a[y0:y0 + 2 * H, x0:x0 + 2 * W])[None].astype(np.float32)]
conv = tf.lite.TFLiteConverter.from_keras_model(d)
conv.optimizations = [tf.lite.Optimize.DEFAULT]
conv.representative_dataset = rep
conv.target_spec.supported_ops = [tf.lite.OpsSet.TFLITE_BUILTINS_INT8]
conv.inference_input_type = tf.uint8
conv.inference_output_type = tf.uint8
tfl = conv.convert()
name = f"genshin_sr_{c1}_{c2}"
open(f"{out}/{name}_int8.tflite", "wb").write(tfl)

# int8 模型在主机上的画质 (CPU 参考实现), 并导出测试输入给真机
it = tf.lite.Interpreter(model_content=tfl, experimental_op_resolver_type=tf.lite.experimental.OpResolverType.BUILTIN_WITHOUT_DEFAULT_DELEGATES); it.allocate_tensors()
ii, oo = it.get_input_details()[0], it.get_output_details()[0]
res["model_int8_host"] = []
for k, (a, lr) in enumerate(zip(test, lows)):
    (si, zi), (so, zo) = ii["quantization"], oo["quantization"]
    q = np.clip(np.round(lr / si + zi), 0, 255).astype(np.uint8)
    q.tofile(f"{out}/test_{k}_in.u8")
    it.set_tensor(ii["index"], q[None]); it.invoke()
    o = (it.get_tensor(oo["index"])[0].astype(np.float32) - zo) * so
    res["model_int8_host"].append(psnr(o, crop(a)))
    np.save(f"{out}/test_{k}_hr.npy", (crop(a) * 255).round().astype(np.uint8))
summary = {k: round(float(np.mean(v)), 2) for k, v in res.items()}
summary.update({"per_frame": {k: [round(x, 2) for x in v] for k, v in res.items()},
                "train_frames": [os.path.basename(p) for p in train_names],
                "test_frames": [os.path.basename(p) for p in test_names],
                "in_quant": [float(x) for x in ii["quantization"]], "out_quant": [float(x) for x in oo["quantization"]],
                "c1": c1, "c2": c2, "steps": steps, "model": f"{name}_int8.tflite"})
json.dump(summary, open(f"{out}/{name}_quality.json", "w"), indent=1, ensure_ascii=False)
print(json.dumps({k: summary[k] for k in res}, ensure_ascii=False))
