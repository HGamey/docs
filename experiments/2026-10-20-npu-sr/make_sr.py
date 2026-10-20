# 生成 NPU 测速用的 2 倍超分模型 (随机权重, 只测速度不测画质)
# 输入按原神半分辨率 1072x488, 输出 2144x976
import sys, os
import numpy as np
import tensorflow as tf

H, W = 488, 1072
OUT = os.path.dirname(os.path.abspath(__file__))

def espcn(c1, c2):
    x = inp = tf.keras.Input((H, W, 3), batch_size=1)
    x = tf.keras.layers.Conv2D(c1, 5, padding="same", activation="relu")(x)
    x = tf.keras.layers.Conv2D(c2, 3, padding="same", activation="relu")(x)
    x = tf.keras.layers.Conv2D(12, 3, padding="same")(x)
    x = tf.keras.layers.Lambda(lambda t: tf.nn.depth_to_space(t, 2))(x)
    return tf.keras.Model(inp, x)

def rep():
    for _ in range(8):
        yield [np.random.rand(1, H, W, 3).astype(np.float32)]

for name, c1, c2 in [("espcn_s", 16, 16), ("espcn_m", 32, 32), ("espcn_l", 64, 32)]:
    m = espcn(c1, c2)
    conv = tf.lite.TFLiteConverter.from_keras_model(m)
    open(f"{OUT}/{name}_fp32.tflite", "wb").write(conv.convert())
    conv = tf.lite.TFLiteConverter.from_keras_model(m)
    conv.optimizations = [tf.lite.Optimize.DEFAULT]
    conv.representative_dataset = rep
    conv.target_spec.supported_ops = [tf.lite.OpsSet.TFLITE_BUILTINS_INT8]
    conv.inference_input_type = tf.uint8
    conv.inference_output_type = tf.uint8
    open(f"{OUT}/{name}_int8.tflite", "wb").write(conv.convert())
    macs = H * W * (5 * 5 * 3 * c1 + 9 * c1 * c2 + 9 * c2 * 12)
    print(name, "params", m.count_params(), "GMAC", round(macs / 1e9, 2))
