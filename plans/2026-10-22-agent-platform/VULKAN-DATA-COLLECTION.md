# 手机 Vulkan 数据采集教程

本教程有两种采法，按需要选一种：

| 采法 | 最后得到 | 手机在哪 |
|---|---|---|
| **一、frame-audit** | `fa_1.json`：一帧里每个 pass 的 draw、dispatch、附件、读写、清屏；`analysis.md`：可疑操作清单 | 平台中心上的手机，用 `pf-node` 远程操作 |
| **二、RenderDoc** | `.rdc`：一帧完整的 Vulkan 调用、资源和 Shader，可以回放 | 手机用 USB 接在你自己的电脑上 |

两种采法不要同时用在同一个游戏上，原因见「原理」。

采到 `.rdc` 以后，按第三部分回放、导出图像，或者做同一帧的高低清两版。

---

## 原理

### 为什么能采到：Vulkan 层

游戏画每一帧，都要调用 Vulkan 函数：创建图像和缓冲、上传数据、绑定 Shader、`vkCmdDraw`、提交、`vkQueuePresentKHR` 把画面送上屏幕。

这些调用并不直接进显卡驱动，而是先经过 Vulkan loader。loader 允许在游戏和驱动之间插入**层**（layer）：

```text
游戏  →  层（可以有多个）  →  显卡驱动  →  GPU
```

层是一个 `.so` 文件。插进去以后，游戏的每一次 Vulkan 调用都会先经过它，它可以记录下来，再原样交给驱动。游戏代码一行不用改。

在 Android 上，loader 从两个地方决定要加载哪些层：

- **系统属性 `debug.vulkan.layers`**：指定层的名字。
- **游戏自己的 native library 目录**：loader 在这里找层文件。

所以采集的准备工作就是两件事：把层文件放进游戏的库目录，再设置这个属性。

- 可调试版本的游戏，RenderDoc 会自动完成这两步。
- 商业包需要 root，这就是第二部分 B 路要手动做的事。

层只在游戏**启动时**加载，所以每次都要先把游戏停掉，放好层，再启动游戏。

本教程的两种采法都是这样工作的：

- frame-audit 插入的是平台自己的层 `libVkLayer_refknobs.so`。
- RenderDoc 插入的是它自己的层 `libVkLayer_GLES_RenderDoc.so`。

两者的区别只在于层里记了什么。

### frame-audit 记什么

frame-audit 的层只记一帧的**结构**，不记数据：

- 每个 render pass 用了哪些附件（颜色、深度），各是什么尺寸和格式。
- 每个附件开始时是读入、清空还是丢弃（loadOp），结束时是写回还是丢弃（storeOp）。
- pass 里有几个 draw、几个 dispatch，有哪些拷贝和清屏。
- 加了 `--ts` 时，还会在每个 pass 前后插入 GPU 时间戳查询，测出这个 pass 在 GPU 上花了多久。

这样记下来的文件很小，几百 KB。分析工具按这些记录找出"白操作"，例如：

- **白存**：写回了，但后面没有人读。
- **白读**：读入了，但马上被整个覆盖。
- **白清**：清空了，但马上被画满。
- **白画**：画了，但结果没有用到。

frame-audit 不保存图像和缓冲的内容，所以**不能回放**。

### RenderDoc 怎么采

RenderDoc 的层平时只做轻量记录：游戏创建了哪些资源、分配了哪些内存。

你点 Trigger Capture 后，它在下一次 `vkQueuePresentKHR`（上一帧结束）时开始完整捕获：

1. **拍下起点。** 把这一帧会用到的每张图像、每个缓冲当前的内容都读出来存下，这部分叫 initial contents。
2. **逐条记录调用。** 从这一刻起，游戏的每一次 Vulkan 调用，连同全部参数，按顺序写进文件。包括 Shader（SPIR-V）、管线状态、描述符，以及 CPU 往 GPU 内存里写的数据。
3. **在终点结束。** 到下一次 `vkQueuePresentKHR`，这一帧结束，捕获停止。

`.rdc` 文件就是"起点时所有资源的内容 + 这一帧的全部调用"。因为要读出大量 GPU 内存并写文件，采集时游戏会卡顿，所以采集过程中测到的帧率没有意义。

### `.rdc` 里有什么

`.rdc` 是一个分段的容器文件，主要有三部分：

| 部分 | 内容 |
|---|---|
| 元数据 | 采集用的驱动名（这里是 Vulkan）、机器标识、帧号、时间戳频率 |
| 缩略图 | 这一帧最终画面的小图，打开文件列表时显示 |
| 帧捕获数据 | 一串按顺序排列的"块"（chunk）。每个块对应一次 Vulkan 调用或一份数据，比如 `vkCreateImage`、`vkCmdBeginRenderPass`、`vkCmdDraw`、`vkQueueSubmit`，以及资源的 initial contents、CPU 写进映射内存的数据 |

RenderDoc 可以只用 CPU 把帧捕获数据解析成一棵树，叫**结构化数据**（structured data，下文简称 SD）：每个块是一个节点，下面是这次调用的全部参数。比如一次 `vkCreateImage` 节点下有 `extent.width`、`extent.height`、`format`、`usage`。

读 SD 不需要 GPU，也不用回放，在任何电脑上都能做。**改 SD 再写回成一个新的 `.rdc`**，就能得到一份"改过参数的捕获"。高低清版本就是这样做出来的，见第三部分 D。

### 为什么商业包要 root

可调试（debuggable）的游戏，Android 允许调试工具把层放进它的目录，所以 RenderDoc 能自动完成准备。

商业包都是不可调试的。它的库目录 `/data/app/…/lib/arm64` 属于系统，只有 root 才能往里放文件。层文件放不进去，loader 就找不到它，所以商业包必须用 root 手机，按第二部分 B 路手动放。

### 怎么重放

重放时**不运行游戏**，只运行这一帧的 GPU 工作：

1. 按记录重新创建这一帧用到的全部资源，把起点时的内容写回去。
2. 按顺序重新执行记录下来的每一条 Vulkan 调用，交给真实的显卡驱动去画。

输入数据完全来自文件，所以每次重放画出来的结果都一样。重放可以停在任意一条调用（event）上，查看：

- 这一刻绑定了什么 Shader 和资源；
- 渲染目标里此时的图像；
- 某个中间缓冲（深度、法线、运动矢量等）的内容。

这就是为什么能从 `.rdc` 里导出游戏屏幕上看不到的中间数据。

重放最好在**采集用的同一台手机**上做。`.rdc` 里的内存类型、扩展和 Shader 都按那块 GPU 记录，换到别的 GPU 上可能放不出来，或者结果不同。

### 回放在哪里跑

回放需要真实的 GPU 和驱动，所以真正执行回放的是**手机**，电脑只负责显示和下指令。有两种方式：

| 方式 | 怎么工作 | 适合 |
|---|---|---|
| **qrenderdoc 远程回放** | 在 qrenderdoc 里把 Replay Context 选成手机。RenderDoc 在手机上启动回放服务（`org.renderdoc.renderdoccmd.arm64`），把 `.rdc` 传过去在手机上回放，再把你要看的图像和数据传回电脑显示 | 手动查看、导出几张图、找 event |
| **手机上直接运行回放程序** | 用 NDK 编译一个调用 RenderDoc 回放 API 的小程序，放到手机上以 root 运行。它在手机本地打开 `.rdc`、回放、把数据写成文件 | 批量导出、处理很多帧、生成派生捕获 |

两种方式用的是同一套回放代码：RenderDoc 的 Android 库 `libVkLayer_GLES_RenderDoc.so` 里既有采集用的层，也有完整的回放实现。

回放一帧要把这一帧的全部资源重新建出来：进程的虚拟地址空间会用到 6 GB 以上，实际占用内存约 1.2 GB。如果给回放进程设了地址空间上限（`ulimit -v`），至少要 8 GiB，设成 4 GiB 会因为分配失败而崩溃。回放时手机上不要开着游戏。

### 为什么能得到高低清两个版本

截两张不同分辨率的图，得到的是两个不同的瞬间：角色动了，镜头动了，粒子和动画也不一样，没法逐像素对应。

重放不一样：`.rdc` 里存的不是像素，而是**画这一帧的全部指令和数据**，包括几何、相机矩阵、材质、动画时刻。只要在重放前改掉跟分辨率有关的部分，同一组指令就会在另一个分辨率下重新画一遍。需要改的有：

- 渲染目标图像的尺寸，以及对应的 framebuffer 和内存分配；
- viewport 和 scissor；
- Shader 常量里写死的分辨率值。CPU 后面写进去的值也要一起改，否则重放时会读到旧值。

这样得到的低清和高清，是**同一个瞬间、同一份几何、同一个相机**，在两种像素网格上各画一遍。它们是真正对齐的一对，不是把一张图放大或缩小得来的。

具体做法是改 SD 再写成新的 `.rdc`，然后回放这份新捕获，见第三部分 D。有两个容易漏的地方：

- **分辨率常量不一定只写了一次。** Shader 读的分辨率常量（比如 `1/宽, 1/高`）在 initial contents 里有一份，CPU 在这一帧里往映射内存写的数据（SD 里叫 `Internal::Coherent Mapped Memory Write`）还可能把它覆盖掉。只改前一份，GPU 实际读到的仍然是旧值。
- **改了尺寸，内存也要够。** 图像变大后需要更大的内存分配。新尺寸需要多少内存，要在原手机上用驱动实际查询（`vkGetImageMemoryRequirements`），不能自己估算。

做配对时还要注意三点：

- **像素中心对齐。** 低清的第 `x` 个像素对应高清的位置是 `(x + 0.5) × 倍数 − 0.5`，不是 `x × 倍数`。
- **有效区域掩码。** 有些效果依赖分辨率，比如屏幕空间反射、时间性抗锯齿的历史帧、UI。这些区域两边不一定一致，要用深度和几何算出可信区域。
- **改尺寸后先验证。** 用原尺寸重放一次，确认结果和原始捕获一样，再改尺寸。原始 `.rdc` 不动，改过的另存一份。

### 导出的数据是什么样的

回放导出的是 GPU 里的原始字节，格式和游戏用的一样，不是截图：

- **场景 HDR** 常见格式是 `R11G11B10_FLOAT`：每像素 4 字节，装着三个无符号小浮点数。R、G 各 11 位（5 位指数、6 位尾数），B 10 位（5 位指数、5 位尾数）。值可以大于 1，不能当 8 位 RGB 看。
- **深度** 常见格式是 `D24S8`：每像素 24 位深度加 8 位模板。RenderDoc 的回放接口会返回「5 × 像素数」个字节：前 4N 字节每像素一个 32 位整数，低 24 位是深度（除以 16777215 得到 0 到 1），高 8 位是模板；最后 N 字节不用。
- **PNG 只用来看。** PNG 是 8 位的，HDR 大于 1 的部分会被截掉，深度精度也会丢。数据一律存原始字节或 DDS。
- 看 HDR 时用固定的预览变换 `(x / (1 + x)) ^ (1 / 2.2)`。它只是为了让人能看清，不是游戏自己的色调映射。

解码代码见第三部分 E。

### 两种采法为什么不能同时用

两个层都插在游戏和驱动之间，一前一后。靠近驱动的那个层，会把另一个层自己发出的调用也当成游戏的调用记下来。比如 frame-audit 加的时间戳查询会进 RenderDoc 的捕获，RenderDoc 采集时读回资源的操作会进 frame-audit 的记录。两边的结果都不干净。

### 连续采的几帧之间隔了多久

RenderDoc 连续采 N 帧时，每采一帧游戏都要停下来，等它把数据写完。大多数游戏的逻辑按真实时间推进，所以两帧之间角色和镜头的移动会比正常游戏大，也不均匀。

帧号连续只说明中间没有漏帧，不说明两帧之间隔了 1/60 秒。做时序数据时，帧间运动要用每一帧自己的相机矩阵算，不能按固定帧率推算。

### 每个捕获都要重新找

event 号、资源编号、缓冲里常量的偏移，每个 `.rdc` 都可能不一样，哪怕是同一个游戏、同一个场景、连着采的下一帧。所以导出用的 event、要改的块和字节，每个捕获都要重新找、重新核对，不能照抄别的帧的数字。

---

## 准备：装好 pf-node（只做一次）

**第 1 步：下载代码。**

```sh
git clone https://github.com/HGamey/hgamey.git ~/hgamey
alias pf-node='python3 ~/hgamey/harness-ops/node/pf-node'
```

**第 2 步：设置中心地址。**

```sh
export PF_NODE_URL=https://gameharness.liyao.space
export PF_NODE_ID=d944a2c6ec60405db5de445506d59360
```

**第 3 步：拿到你自己的 token。** 向管理员要一个邀请码，然后运行：

```sh
pf-node join --invite <邀请码> --name <你的名字> --token-file ~/.config/hgamey/<你的名字>.token
export PF_NODE_TOKEN_FILE=~/.config/hgamey/<你的名字>.token
```

把第 1、2、3 步里的 `alias` 和 `export` 写进 `~/.bashrc`，以后就不用再设。

**第 4 步：检查能不能连上。**

```sh
pf-node checkin
```

看到一段 JSON，里面有你的名字和待办，就说明连上了。

**第 5 步：编译 phonefarm。** 用于第一部分的离线分析。

```sh
cd ~/hgamey/phonefarm && cargo build --release
```

完成后会生成 `~/hgamey/phonefarm/target/release/phonefarm`。

---

## 一、用 frame-audit 采一帧

### 第 1 步：找一台能用的手机

```sh
pf-node phones | python3 -c "
import json, sys
for p in json.load(sys.stdin)['items']:
    if p['node_name'] == 'local':
        print(p['ref'], p['model'], 'online' if p['online'] else 'offline',
              'idle' if p['idle'] else 'busy', 'root' if p['root'] else 'no-root')"
```

你会看到类似：

```text
d944a2c6ec60405db5de445506d59360/REDMAGIC NX809J online idle root
d944a2c6ec60405db5de445506d59360/ONEPLUS PLC110 online idle root
```

选一台同时是 `online idle root` 的手机，复制第一列，下面用 `<手机>` 代替。

只列 `local`（中心本机）的手机，是因为 frame-audit 要用的注入层目前只装在中心上。

### 第 2 步：确认游戏已经装在这台手机上

```sh
pf-node --device <手机> adb -s @serial shell pm list packages <游戏名关键字>
```

输出里有 `package:<游戏包名>`，就说明装了。记下包名，下面用 `<包名>` 代替。

### 第 3 步：拿到研究 ID

在网页「任务」里打开你的任务，地址栏里 `id=` 后面那一串就是研究 ID，下面用 `<研究ID>` 代替。

### 第 4 步：确定怎么进到要采的场景

采集时，命令会**重启游戏**，等一段时间再采。如果游戏启动后要点几下才能进场景，需要先找出这几下的坐标。

1. 启动游戏：

   ```sh
   pf-node --device <手机> adb -s @serial shell monkey -p <包名> -c android.intent.category.LAUNCHER 1
   ```

2. 等游戏停在要点的画面，截图并取回：

   ```sh
   pf-node --device <手机> adb -s @serial shell screencap -p /sdcard/step.png
   pf-node --device <手机> adb -s @serial pull /sdcard/step.png step.png
   ```

   第二条命令跑完会自动把图片下载到 `pf-node-out/<作业ID>/step.png`，屏幕上会显示这个路径。

3. 打开图片，记下要点的位置的像素坐标 `x,y`。
4. 需要点好几下，就重复第 2、3 步，按顺序记下来，比如 `1200,650;1800,900`。

### 第 5 步：提交采集

```sh
pf-node --research <研究ID> --device <手机> --request-id fa-<场景名>-001 --detach \
  phonefarm frame-audit capture \
    --serial @serial --pkg <包名> \
    --settle 60 --tap "<第 4 步的坐标>" \
    --captures 1 --out @out --json
```

| 参数 | 填什么 |
|---|---|
| `--settle 60` | 游戏启动后等 60 秒再开始点击和采集。加载慢就加大 |
| `--tap "x,y;x,y"` | 第 4 步记下的坐标。不需要点就去掉这个参数 |
| `--captures 1` | 采 1 轮。采多轮时加 `--gap 20`，表示每轮之间隔 20 秒 |
| `--request-id` | 自己起一个名字。每次新采集用一个新名字 |

命令会打印一个作业 ID，下面用 `<作业ID>` 代替。

### 第 6 步：等它完成

```sh
pf-node wait <作业ID>
```

看到 `outcome=succeeded` 就是成功了。如果失败，运行下面的命令看原因：

```sh
pf-node log <作业ID>
```

如果第 5 步的命令断网了，不知道有没有提交上，运行 `pf-node request fa-<场景名>-001` 找回作业 ID。**不要换一个 request-id 重新提交。**

### 第 7 步：下载结果

```sh
pf-node out <作业ID>
ls pf-node-out/<作业ID>/gray-session-*/
```

你会看到：

```text
fa_1.json          # 这一帧的渲染步骤
fa_1.png           # 采集时的画面
collection.json    # 采集记录
capture_meta.json  # 设备、游戏、参数
```

先打开 `fa_1.png`，确认画面就是你要的场景。不是的话，回到第 4 步调整坐标或 `--settle`。

### 第 8 步：分析

```sh
~/hgamey/phonefarm/target/release/phonefarm frame-audit analyze \
  pf-node-out/<作业ID>/gray-session-*/fa_1.json \
  --md analysis.md --json analysis.json
```

打开 `analysis.md`，里面有每个 pass 的概览，以及白存、白读、白清、白画的可疑清单。

### 第 9 步：检查这一帧记全了没有

```sh
F=$(ls pf-node-out/<作业ID>/gray-session-*/fa_1.json)
grep -oE '"(cb_unseen|ev_overflow|ts_skipped)":[0-9]+' $F
```

三个数都是 `0`，说明这一帧记全了。不是 `0` 的话，在报告里注明"这一帧没记全"。

### 第 10 步（可选）：加上 GPU 时间

把第 5 步的命令加上 `--ts`，换一个新的 `--request-id`，重新做第 5 到第 8 步。然后检查时间是不是真的测到了：

```sh
grep -oE '"(ts_enabled|ts_available|passes_timed)":[a-z0-9]+' $F
```

`passes_timed` 为 `0` 的帧，时间显示为 `0.0 ms`，意思是**没测到**，不是零耗时。

### 第 11 步：交付

```sh
pf-node research deliver <研究ID> --summary analysis.md --artifact job:<作业ID>
```

任务页上会显示交付报告，`fa_1.json` 和截图出现在成果文件里。

---

## 二、用 RenderDoc 抓一帧

### 准备

**第 1 步：接好手机。** 用 USB 把手机接到你的电脑，运行：

```sh
adb devices
adb shell su -c id
```

第一条要能列出手机。第二条要输出 `uid=0(root)`。游戏如果是可调试版本，不需要 root，可以直接跳到下面的「A」。

**第 2 步：安装 RenderDoc 1.46。** 从 renderdoc.org 下载，解压后运行 `qrenderdoc`。

**第 3 步：让 RenderDoc 认到手机。** 在 qrenderdoc 窗口左下角的 Replay Context 下拉框里选你的手机。RenderDoc 会自动往手机上装一个 `org.renderdoc.renderdoccmd.arm64`，等它装完。

### A. 游戏是可调试版本

1. 切到 **Launch Application** 页，点 Executable Path 右边的 `…`，在手机上的应用列表里选游戏。
2. 点 **Launch**，游戏会在手机上启动。
3. 在手机上进到要采的场景。
4. 在 qrenderdoc 弹出的游戏连接窗口里，点 **Trigger Capture**。
5. 窗口里出现一张缩略图，就是抓到的帧。右键点它，选 **Save**，存到电脑上。
6. 跳到下面的「检查」。

### B. 游戏是商业包（不可调试），手机已 root

**第 1 步：设好包名。**

```sh
PKG=<游戏包名>
```

**第 2 步：找到游戏的库目录。**

```sh
LIBDIR=$(adb shell dumpsys package $PKG | sed -n 's/.*legacyNativeLibraryDir=//p' | tr -d '\r')/arm64
echo $LIBDIR
```

应该输出 `/data/app/…/lib/arm64` 这样的路径。

**第 3 步：确认没有别的层在用。**

```sh
adb shell getprop debug.vulkan.layers
```

应该是空的。如果不是空的，说明有别的采集正在用这台手机，先停下，不要继续。

**第 4 步：取出 RenderDoc 的层文件。** 在 RenderDoc 的解压目录里运行：

```sh
unzip -j plugins/android/org.renderdoc.renderdoccmd.arm64.apk lib/arm64-v8a/libVkLayer_GLES_RenderDoc.so
```

当前目录会多出 `libVkLayer_GLES_RenderDoc.so`。

**第 5 步：把层放进游戏的库目录。**

```sh
adb shell am force-stop $PKG
adb push libVkLayer_GLES_RenderDoc.so /data/local/tmp/
adb shell su -c "cp /data/local/tmp/libVkLayer_GLES_RenderDoc.so $LIBDIR/"
adb shell su -c "chmod 755 $LIBDIR/libVkLayer_GLES_RenderDoc.so"
adb shell su -c "chown system:system $LIBDIR/libVkLayer_GLES_RenderDoc.so"
adb shell su -c "chcon u:object_r:apk_data_file:s0 $LIBDIR/libVkLayer_GLES_RenderDoc.so"
adb shell su -c "ls -lZ $LIBDIR/libVkLayer_GLES_RenderDoc.so"
```

最后一条应该显示 `-rwxr-xr-x`、`system system`、`u:object_r:apk_data_file:s0`。

**第 6 步：打开层，启动游戏。**

```sh
adb shell su -c "setprop debug.vulkan.layers VK_LAYER_RENDERDOC_Capture"
adb shell su -c "setprop debug.rdoc.IGNORE_LAYERS 0"
adb shell monkey -p $PKG -c android.intent.category.LAUNCHER 1
```

**第 7 步：确认层加载上了。** 等游戏起来以后运行：

```sh
PID=$(adb shell pidof $PKG | tr -d '\r' | awk '{print $1}')
adb shell su -c "grep -c libVkLayer_GLES_RenderDoc /proc/$PID/maps"
```

输出大于 `0` 就是加载上了。输出 `0` 的话，回到第 5 步，检查文件和权限。

**第 8 步：马上把全局属性关掉。** 层已经加载进游戏进程，关掉属性不影响这次采集，但能避免其他应用也加载这个层。

```sh
adb shell su -c "setprop debug.vulkan.layers ''"
```

**第 9 步：进场景，然后抓帧。**

1. 在手机上进到要采的场景，确认角色和镜头确实能动。
2. 在 qrenderdoc 里点 **File → Attach to Running Instance**，选你的手机，列表里会出现游戏进程，点 **Connect to App**。
3. 在弹出的连接窗口里设置帧数（第一次先抓 1 帧），点 **Trigger Capture**。
4. 出现缩略图后，右键点它，选 **Save**，存到电脑上。

手机上也保留了一份，在 `/sdcard/Android/media/<包名>/files/RenderDoc/`。

**第 10 步：收尾。**

```sh
adb shell am force-stop $PKG
adb shell su -c "rm $LIBDIR/libVkLayer_GLES_RenderDoc.so"
adb shell rm /data/local/tmp/libVkLayer_GLES_RenderDoc.so
adb shell su -c "setprop debug.rdoc.IGNORE_LAYERS ''"
adb shell getprop debug.vulkan.layers
adb shell su -c "ls $LIBDIR"
```

最后两条：第一条应该是空的，第二条的列表里不应该再有 `libVkLayer_GLES_RenderDoc.so`。

### 检查

1. 在 qrenderdoc 里打开存下来的 `.rdc`，Replay Context 选同一台手机。
2. 在 **Event Browser** 里能看到 draw 和 dispatch 的列表。
3. 选中最后几个 draw，在 **Texture Viewer** 里看输出图像，确认就是你要的场景。
4. 能看到场景，这一帧就采成功了。

### 采一段连续帧

确认一帧没问题以后，在第 9 步里把帧数改成 6，其他步骤不变。注意：

- 采之前先让角色或镜头动起来。输入发出去了不代表角色真的动了，要看画面。
- 实际收到几帧就记几帧，缺的不要拿别的帧补。
- 每个 `.rdc` 大约 600 MB，6 帧大约 3.7 GB。准备好空间。
- 在手机上回放一帧，大约需要 6.4 GB 虚拟地址空间和 1.2 GB 内存。

### 交付

1. 把 `.rdc` 放在大容量机器上，不要上传到平台。
2. 写一个 `manifest.json`，内容包括：
   - 游戏包名和版本、手机型号、场景
   - RenderDoc 版本
   - 每个 `.rdc` 的文件名、大小、SHA256（用 `sha256sum *.rdc` 计算）、帧号
   - 存放位置
3. 交付到研究任务：

   ```sh
   pf-node research deliver <研究ID> --summary manifest.md
   ```

---

## 三、回放和导出 RenderDoc 捕获

按要做的事选一种：

| 要做什么 | 用什么 | 在哪跑 | 见 |
|---|---|---|---|
| 看一帧：找 event、看中间缓冲、看 Shader 和参数 | qrenderdoc 图形界面 | 电脑上看，手机上回放 | A |
| 导出几张图到电脑 | qrenderdoc 的 Python Shell | 电脑上看，手机上回放 | B |
| 批量导出、处理很多帧 | 自己编译的回放程序 `rdexport` | 手机上 | C |
| 做同一帧的高低清两版 | 改 SD，写出新的 `.rdc`，再按 C 导出 | 手机上 | D |
| 把导出的原始字节变成数组和预览图 | Python + numpy | 电脑上 | E |

不管用哪种，回放都要在**采集用的那台手机**上做。

### A. 用 qrenderdoc 回放和查看

1. 用 USB 接好采集用的手机，`adb devices` 能看到它。
2. 打开 qrenderdoc，在左下角的 Replay Context 下拉框里选这台手机，等它显示已连接。
3. 点 **File → Open Capture**，选 `.rdc` 文件。qrenderdoc 会把文件传到手机上去回放。600 MB 的文件走 USB 要传一会儿，等进度条走完。
4. 在 **Event Browser** 里展开 pass，点一个 event。之后所有窗口显示的，都是「回放到这个 event 刚执行完」时的状态。
5. 在 **Texture Viewer** 里看图像：
   - **Outputs** 页：这个 event 写的颜色目标和深度。
   - **Inputs** 页：这个 event 读的纹理。
   - HDR 图默认可能很暗或一片白，拖动右上角的显示范围（Range）看清。
   - 鼠标放在图上，下面会显示这个像素的原始值。
6. 找到你要的 event。场景数据通常取「场景主 pass 的最后一个 draw」：从后往前点，找 Outputs 里是完整场景、还没有 UI 的那个 pass，记下它最后一个 draw 的 event 号。
7. 看这个 event 的其他信息：
   - **Pipeline State**：Shader、绑定的资源、viewport 和 scissor、render pass 和 framebuffer。点 Shader 名字可以看它的 SPIR-V 反汇编，点常量缓冲可以看里面的值。
   - **API Inspector**：这个 event 的调用和全部参数，就是 SD 里对应块的内容。
   - **Resource Inspector**：选一个资源，看创建它的调用（比如 `vkCreateImage` 的全部参数）和用到它的 event。
   - **Mesh Viewer**：draw 的顶点输入和输出。
8. 导出一张图：在 Texture Viewer 里选中这张图，点工具栏上的保存按钮（Save Texture），选格式：
   - **DDS**：保留原始格式，HDR 和深度都不丢精度。要当数据用，选这个。
   - **EXR**：转成浮点。
   - **PNG**：8 位，只用来看。

   文件存在电脑上。

### B. 用 Python Shell 导出

1. 按 A 的第 1 到 3 步打开捕获。
2. 在 Event Browser 里点中要导出的 event。
3. 点菜单 **Window → Python Shell**，把下面的代码贴进去，改好 `OUT`，运行：

   ```python
   import renderdoc as rd

   OUT = '/home/me/rd-out'          # 电脑上的输出目录，要先建好
   EID = pyrenderdoc.CurEvent()     # Event Browser 里当前选中的 event
   action = pyrenderdoc.GetAction(EID)

   def export(ctrl):
       ctrl.SetFrameEvent(EID, True)
       targets = [('c%d' % i, r) for i, r in enumerate(action.outputs)] + [('depth', action.depthOut)]
       for name, rid in targets:
           if rid == rd.ResourceId.Null():
               continue
           ts = rd.TextureSave()
           ts.resourceId = rid
           ts.destType = rd.FileType.DDS
           path = '%s/e%d_%s.dds' % (OUT, EID, name)
           ctrl.SaveTexture(ts, path)
           print(path)

   pyrenderdoc.Replay().BlockInvoke(export)
   ```

4. 每导出一张会打印一个路径。`c0` 到 `c7` 是颜色输出，`depth` 是深度。

### C. 在手机上批量回放：rdexport

`rdexport` 是一个调用 RenderDoc 回放接口的小程序，在手机上直接运行：

- `rdexport <x.rdc>`：列出这个捕获的全部 event。
- `rdexport <x.rdc> <event> <输出目录>`：回放到这个 event，把它的颜色输出和深度按原始字节写成文件。

第 1 到 5 步只做一次。

**第 1 步：装 Android NDK r28。**

Linux：

```sh
curl -LO https://dl.google.com/android/repository/android-ndk-r28c-linux.zip
unzip -q android-ndk-r28c-linux.zip
export NDK=$PWD/android-ndk-r28c
export NDK_BIN=$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin
```

要用 `unzip` 解压。Python 的 zipfile 会把里面的符号链接解成普通文件，之后编译会报错。

macOS：在 Android Studio 的 SDK Manager 里装 NDK 28，然后：

```sh
export NDK_BIN=~/Library/Android/sdk/ndk/<版本号>/toolchains/llvm/prebuilt/darwin-x86_64/bin
```

**第 2 步：取 RenderDoc 1.46 的头文件。**

```sh
git clone --depth 1 --branch v1.46 https://github.com/baldurk/renderdoc.git
```

头文件在 `renderdoc/renderdoc/api/replay/`。版本必须是 1.46，和手机上用的库一致。

**第 3 步：取手机版 RenderDoc 库。** 和第二部分 B 第 4 步一样，在 RenderDoc 1.46 的解压目录里运行：

```sh
unzip -j plugins/android/org.renderdoc.renderdoccmd.arm64.apk lib/arm64-v8a/libVkLayer_GLES_RenderDoc.so
```

这个库既是采集用的层，也包含完整的回放功能。把它放到和下面的 `rdexport.cpp` 同一个目录。

**第 4 步：把下面的代码存成 `rdexport.cpp`。**

```cpp
// rdexport：在手机上回放一个 .rdc。
//   rdexport <x.rdc>                    列出全部 event
//   rdexport <x.rdc> <event> <输出目录>  导出这个 event 的颜色输出和深度（原始字节）
#include <cstdio>
#include <cstdlib>
#include <string>
#include "renderdoc_replay.h"

REPLAY_PROGRAM_MARKER()

// ResultDetails::Message() 链接不上（依赖库里没导出的 ToStr），直接读 code 和 internal_msg
static int fail(const char *what, const ResultDetails &r)
{
  fprintf(stderr, "%s 失败: code=%u %s\n", what, (unsigned)r.code, r.internal_msg ? r.internal_msg->c_str() : "");
  return 1;
}

static void list(const rdcarray<ActionDescription> &actions, const SDFile &sd, int depth)
{
  for(const ActionDescription &a : actions)
  {
    printf("%*s%u  %s\n", depth * 2, "", a.eventId, a.GetName(sd).c_str());
    list(a.children, sd, depth + 1);
  }
}

static const ActionDescription *findAction(const rdcarray<ActionDescription> &actions, uint32_t eid)
{
  for(const ActionDescription &a : actions)
  {
    if(a.eventId == eid)
      return &a;
    if(const ActionDescription *c = findAction(a.children, eid))
      return c;
  }
  return nullptr;
}

static const TextureDescription *find(IReplayController *ctrl, ResourceId id)
{
  for(const TextureDescription &t : ctrl->GetTextures())
    if(t.resourceId == id)
      return &t;
  return nullptr;
}

int main(int argc, char **argv)
{
  if(argc != 2 && argc != 4)
  {
    fprintf(stderr, "用法: rdexport <x.rdc> [<event> <输出目录>]\n");
    return 2;
  }

  GlobalEnvironment env;
  env.enumerateGPUs = false;
  RENDERDOC_InitialiseReplay(env, {});

  ICaptureFile *file = RENDERDOC_OpenCaptureFile();
  ResultDetails res = file->OpenFile(argv[1], "", nullptr);
  if(!res.OK())
    return fail("OpenFile", res);

  ReplayOptions opts;
  rdcpair<ResultDetails, IReplayController *> rc = file->OpenCapture(opts, nullptr);
  if(!rc.first.OK())
    return fail("OpenCapture", rc.first);
  IReplayController *ctrl = rc.second;

  if(argc == 2)
  {
    list(ctrl->GetRootActions(), ctrl->GetStructuredFile(), 0);
  }
  else
  {
    uint32_t eid = (uint32_t)strtoul(argv[2], nullptr, 10);
    std::string outdir = argv[3];

    const ActionDescription *action = findAction(ctrl->GetRootActions(), eid);
    if(!action)
    {
      fprintf(stderr, "没有 event %u\n", eid);
      return 1;
    }

    // 回放到这个 event 执行完
    ctrl->SetFrameEvent(eid, true);

    // 8 个颜色输出，加上深度（手机版库没有导出 PipeState 的方法，所以用 action 里的记录）
    ResourceId targets[9];
    for(int i = 0; i < 8; i++)
      targets[i] = action->outputs[i];
    targets[8] = action->depthOut;

    for(int i = 0; i < 9; i++)
    {
      ResourceId id = targets[i];
      const TextureDescription *tex = find(ctrl, id);
      if(id == ResourceId::Null() || !tex)
        continue;

      bytebuf data = ctrl->GetTextureData(id, Subresource(0, 0, 0));
      bool depth = i == 8;
      std::string name = outdir + "/e" + std::to_string(eid) + (depth ? "_depth" : "_c" + std::to_string(i)) +
                         "_" + std::to_string(tex->width) + "x" + std::to_string(tex->height) + "_" +
                         std::string(tex->format.Name().c_str()) + ".raw";
      FILE *f = fopen(name.c_str(), "wb");
      fwrite(data.data(), 1, data.size(), f);
      fclose(f);
      printf("%s  %zu 字节\n", name.c_str(), (size_t)data.size());
    }
  }

  ctrl->Shutdown();
  file->Shutdown();
  RENDERDOC_ShutdownReplay();
  return 0;
}
```

**第 5 步：编译。**

```sh
$NDK_BIN/aarch64-linux-android33-clang++ \
  -std=c++17 -DRENDERDOC_PLATFORM_ANDROID -O2 -rdynamic -static-libstdc++ \
  -I renderdoc/renderdoc/api/replay \
  rdexport.cpp libVkLayer_GLES_RenderDoc.so \
  -Wl,-rpath,'$ORIGIN' -o rdexport
file rdexport
```

最后一条应该输出 `ELF 64-bit LSB pie executable, ARM aarch64`。`-Wl,-rpath,'$ORIGIN'` 让程序运行时在自己所在的目录里找 RenderDoc 库。

**第 6 步：把程序和库放到手机上。**

```sh
adb shell mkdir -p /data/local/tmp/rd/out
adb push rdexport libVkLayer_GLES_RenderDoc.so /data/local/tmp/rd/
adb shell chmod 755 /data/local/tmp/rd/rdexport
```

**第 7 步：指定要回放的 `.rdc`。** 在这台手机上采的，直接用手机上那份（`PKG` 是第二部分 B 第 1 步设的包名）：

```sh
adb shell ls /sdcard/Android/media/$PKG/files/RenderDoc/
RDC=/sdcard/Android/media/$PKG/files/RenderDoc/<文件名>.rdc
```

捕获在电脑上的，先传上去：

```sh
adb push frame.rdc /data/local/tmp/rd/
RDC=/data/local/tmp/rd/frame.rdc
```

**第 8 步：列出 event。**

```sh
adb shell "su -c 'cd /data/local/tmp/rd && ./rdexport $RDC'" > events.txt
```

`events.txt` 里每行是一个 event 号和调用名，缩进表示层级。要导出哪个 event，可以先在 qrenderdoc 里找（A 第 6 步），两边的 event 号是一样的。

**第 9 步：导出。**

```sh
adb shell "su -c 'cd /data/local/tmp/rd && ./rdexport $RDC <event号> out'"
```

每导出一个文件打印一行。文件名里有 event 号、槽位（`c0` 到 `c7` 是颜色，`depth` 是深度）、尺寸和格式。检查字节数：`R11G11B10_FLOAT` 应该是「宽 × 高 × 4」，`D24S8` 应该是「宽 × 高 × 5」。

**第 10 步：取回电脑，清掉手机上的输出。**

```sh
adb pull /data/local/tmp/rd/out ./rd-out
adb shell "su -c 'rm -rf /data/local/tmp/rd/out/*'"
```

`rdexport` 和库留在手机上，下次从第 7 步开始。

多个捕获要一个一个处理：每个 `.rdc` 都先做第 8 步列出 event，确认 event 号，再做第 9 步。

### D. 做同一帧的高低清两版

这一步要自己写 C++ 程序改 SD（编译方式和 C 一样）。下面按顺序列出每一步做什么、用什么接口、怎么检查。下文用低清 1782×810、高清 2376×1080（4/3 倍）举例；实际数字每个捕获都要重新查。

原始 `.rdc` 不动，所有改动都写进新文件。

**第 1 步：导出母版。** 用 C 的 `rdexport` 导出原始捕获在场景主 pass 最后一个 draw 的颜色和深度。同一个 event 导两次，两次的 `sha256sum` 必须一样，证明回放可以重复。

**第 2 步：读 SD，找到要改的块。** 用 `ICaptureFile::OpenFile` 打开，`GetStructuredData()` 拿到 SD。`chunks` 里每个块有名字（`ch->name`），参数用 `FindChild` 一层层往下找。场景 pass 用到的每张渲染目标，都要找到下面这些块：

| 块 | 要改的字段 |
|---|---|
| `vkCreateImage` | `CreateInfo.extent.width`、`CreateInfo.extent.height`、`memoryRequirements.size` |
| `vkAllocateMemory`（这张图单独用的那块内存） | `AllocateInfo.allocationSize` |
| `vkCreateFramebuffer` | `CreateInfo.width`、`CreateInfo.height` |
| `vkCmdBeginRenderPass`（场景主 pass） | `RenderPassBegin.renderArea.extent.width`、`.height` |
| `vkCmdSetViewport` | `pViewports.0.width`、`pViewports.0.height` |
| `vkCmdSetScissor` | `pScissors.0.extent.width`、`pScissors.0.extent.height` |
| `Internal::Initial Contents`（这张图的） | `ContentsSize`，以及 `Contents` 指向的缓冲 |

可以先在 qrenderdoc 的 Resource Inspector 和 API Inspector 里看一遍这些调用的参数，再写程序。

如果一张图和别的资源共用一块内存（`vkBindImageMemory` 的 offset 不是 0，或者同一块内存绑了几个资源），改大以后会压到别的资源。这种帧先跳过。

**第 3 步：在原手机上查新尺寸要多少内存。** 写一个小的 Vulkan 程序在原手机上运行：按 SD 里 `vkCreateImage` 的 `CreateInfo` 各建一张原尺寸和新尺寸的图，调用 `vkGetImageMemoryRequirements`，记下 `size`、`alignment`、`memoryTypeBits`。新尺寸的 `size` 填进第 2 步表里的 `memoryRequirements.size` 和 `allocationSize`。

格式、flags、usage 要和回放时实际创建这张图用的一致。RenderDoc 回放时可能会给图像多加 usage 位（比如传输用的位），照抄 SD 不一定对。

检查：原尺寸查到的 `size` 必须等于 SD 里的 `memoryRequirements.size`。不相等，说明程序建图的条件和回放时不一样，它查出的新尺寸也不能用。不要自己估算内存大小。

**第 4 步：处理新尺寸图像的初始内容。** 原来的初始内容是按旧尺寸存的，新尺寸没有现成的内容。把这些图的初始内容换成新尺寸的全 0，同时 `ContentsSize` 改成新的大小。

再按同样的方法做一份**零值对照**：尺寸不变，只把同样这些图的初始内容换成全 0。回放后在同一个 event 导出，和第 1 步的母版逐字节比较。完全一样，说明这些图的初始内容不影响输出，高清版的全 0 初始化也就没有问题。不一样的话，这一帧不能这样做。

**第 5 步：找到并改掉分辨率常量。**

1. 在 qrenderdoc 里打开原始捕获，点场景主 pass 的 draw，在 Pipeline State 里看它绑定的常量缓冲，找跟分辨率有关的值，比如 `1782`、`810`，或者 `1/1782`、`1/810`。
2. 看这个 draw 的 Shader 的 SPIR-V 反汇编，确认 Shader 真的读这个字段（`OpAccessChain` 取到这个成员），并且拿它跟像素坐标（`FragCoord`）或采样坐标一起计算。光是数值看起来像分辨率还不够。
3. 找这个常量在 SD 里的所有出处：常量缓冲所在内存的 `Internal::Initial Contents`，以及这一帧里所有覆盖到这个范围的 `Internal::Coherent Mapped Memory Write`。GPU 读到的是 draw 之前最后写进去的那一份，每一份都要改。
4. 只改证明过的那几个字节，其余字节保持原样。

**第 6 步：写出新的 `.rdc`。** 要点：

- 改 SD 的副本，不改原来那份。块用 `Duplicate()` 深拷贝；缓冲先共用，要改哪个再单独复制。
- 写出时用一个**新的** `ICaptureFile`：先 `SetMetadata`，元数据照抄母捕获，再 `Convert`。直接用打开母捕获的那个对象 `Convert`，可能写出的还是旧数据。
- `SetMetadata` 要的 `machineIdent` 从母捕获的文件头读：第 36 到 39 字节是缩略图长度 L，从第 40 + L 字节开始的 8 字节就是它。

```cpp
// 1. 打开母捕获，读出 SD
ICaptureFile *src = RENDERDOC_OpenCaptureFile();
src->OpenFile(mother, "rdc", nullptr);
const SDFile &sd = src->GetStructuredData();

// 2. 复制一份 SD：块用 Duplicate() 深拷贝；缓冲先共用，要改的再单独复制
SDFile copy;
copy.version = sd.version;
for(SDChunk *c : sd.chunks)
  copy.chunks.push_back(c->Duplicate());
for(bytebuf *b : sd.buffers)
  copy.buffers.push_back(b);

// 3. 改一个标量：例如第 i 个块是要放大的那张图的 vkCreateImage
SDChunk *ch = copy.chunks[i];
SDObject *w = ch->FindChild("CreateInfo")->FindChild("extent")->FindChild("width");
if(w->data.basic.u != 1782) return;    // 先核对旧值
w->data.basic.u = 2376;
ch->metadata.length = 0;               // 改过的块都这样做

// 4. 改缓冲里的几个字节：先复制这个缓冲，只改证明过的字段
bytebuf *b = new bytebuf(*copy.buffers[k]);
copy.buffers[k] = b;
memcpy(b->data() + offset, newBytes, n);

// 5. 用一个新的 ICaptureFile 写出，元数据照抄母捕获
Thumbnail th = src->GetThumbnail(FileType::JPG, 0);
ICaptureFile *dst = RENDERDOC_OpenCaptureFile();
dst->SetMetadata(src->DriverName(), machineIdent, FileType::JPG, th.width, th.height, th.data,
                 src->TimestampBase(), src->TimestampFrequency());
ResultDetails r = dst->Convert(high, "rdc", &copy, nullptr);
dst->Shutdown();

// 6. 收尾：copy 析构时会 delete 所有缓冲，先把共用的那些拿掉
for(size_t j = 0; j < copy.buffers.size(); j++)
  if(j != k)
    copy.buffers[j] = nullptr;
src->Shutdown();
```

每个要改的值都照第 3 段那样：先核对旧值，再写新值。

**第 7 步：检查新文件。** 用新的 `ICaptureFile` 打开刚写出的 `.rdc`，`GetStructuredData()` 后逐项检查：

- 块的数量和母捕获一样；
- 改过的字段是新值；
- 没改的块和缓冲，内容和母捕获完全一样（缓冲可以逐个算 sha256 比较）；
- `RecordedMachineIdent()`、`TimestampBase()`、`TimestampFrequency()` 和母捕获一样。

**第 8 步：回放新文件，导出高清版。** 按 C 的第 6 到 10 步回放新 `.rdc`，在同一个 event 导出。检查：

- 输出尺寸是新尺寸，字节数对得上；
- 在 qrenderdoc 里打开新 `.rdc`，在这个 draw 的 Pipeline State 里看常量缓冲，分辨率常量是新值。

**第 9 步：配对检查。** 用 E 的代码把低清和高清都解码，然后：

1. **按像素中心对应。** 低清像素 `(x, y)` 的中心在高清图里的坐标是 `((x + 0.5) × s − 0.5, (y + 0.5) × s − 0.5)`，`s` 是倍数（这里是 4/3）。在这个位置对高清图做双线性采样，得到和低清同尺寸的图。
2. **算可信区域。** 一个低清像素要同时满足三条才算可信：
   - 深度大于 0；
   - 和上下左右四个邻居的深度差都不超过 1e-4（不在物体边缘上）；
   - 高清图在对应位置的深度（取最近的像素），和低清的深度差不超过 1e-4。
3. **只在可信区域里比较 HDR**，算平均绝对误差（MAE）。

参考（一帧上的实测）：常量全部改对时，可信区域里的 HDR MAE 约 0.01；只改了 Initial Contents、漏了后面的 Coherent Mapped Memory Write 时约 0.066。

比较时不要平移、翻转或拟合颜色去「对齐」两张图。对不上就说明哪里没改对，回到第 5 步查。

### E. 解码导出的原始字节

C 和 D 导出的 `.raw` 文件，用下面的 Python 代码读成 numpy 数组（需要 `numpy` 和 `Pillow`）：

```python
import numpy as np
from PIL import Image

def read_hdr(path, w, h):
    """R11G11B10_FLOAT：每像素 4 字节，返回 (h, w, 3) 的 float32"""
    word = np.fromfile(path, '<u4')
    assert word.size == w * h
    word = word.reshape(h, w)
    out = []
    for shift, mbits in ((0, 6), (11, 6), (22, 5)):    # R、G 各 6 位尾数，B 5 位
        v = (word >> shift) & ((1 << (mbits + 5)) - 1)
        e = (v >> mbits).astype(np.int32)
        m = (v & ((1 << mbits) - 1)).astype(np.float64) / (1 << mbits)
        assert not (e == 31).any(), '有 Inf 或 NaN'
        out.append(np.where(e == 0, m * 2.0 ** -14, (1 + m) * 2.0 ** (e - 15)))
    return np.stack(out, -1).astype(np.float32)

def read_depth(path, w, h):
    """D24S8：文件是 5 × 像素数 字节，前 4N 字节每像素一个 uint32，低 24 位是深度"""
    raw = np.fromfile(path, np.uint8)
    assert raw.size == 5 * w * h
    packed = raw[:4 * w * h].view('<u4').reshape(h, w)
    return (packed & 0xffffff) / 16777215.0

def preview(hdr):
    """只用来看，不是游戏自己的色调映射"""
    return (hdr / (1 + hdr)) ** (1 / 2.2)

hdr = read_hdr('rd-out/e1234_c0_1782x810_R11G11B10_FLOAT.raw', 1782, 810)
Image.fromarray((np.clip(preview(hdr), 0, 1) * 255 + 0.5).astype(np.uint8)).save('preview.png')
```

把最后两行的文件名、宽和高换成你导出的那个文件。


---

## 常见问题

| 现象 | 怎么办 |
|---|---|
| frame-audit 失败，日志里说找不到层 | 这台手机不在中心上。按第一部分第 1 步，只选 `local` 的手机 |
| `fa_1.png` 不是要的场景 | 加大 `--settle`，或者重新做第 4 步，检查点击坐标 |
| `analysis.md` 里时间全是 0 | 没加 `--ts`，或者这台手机不支持时间戳；按第 10 步检查 `passes_timed` |
| RenderDoc 的 Attach 列表里没有游戏 | 第 7 步的结果是 0：层没加载上。回到第 5 步检查 `ls -lZ` 的输出 |
| 有截图，没有 `.rdc` | 没有点 Trigger Capture，或者抓帧还没完成。等缩略图出现再保存 |
| 电脑能打开 `.rdc`，但回放失败 | 把 Replay Context 换成采集用的那台手机再回放 |
| 连续几帧画面都一样 | 角色或镜头没动。先让角色动起来再抓 |
| `rdexport` 报 `OpenCapture 失败` | 换成采集用的那台手机；确认手机上的 `libVkLayer_GLES_RenderDoc.so` 和编译时用的是同一个 1.46 |
| `rdexport` 运行一半被杀掉 | 内存不够。关掉游戏和其他应用；检查有没有设过小的 `ulimit -v` |
| 自己改 `rdexport` 后链接报 `undefined symbol: PipeState::…` 或 `ToStr` | 手机版库没有导出这些函数。用 `ActionDescription` 里的 `outputs`、`depthOut`，错误信息读 `internal_msg` |
| 导出的 PNG 一片白或很暗 | HDR 数据不能直接存 PNG。存 DDS 或原始字节，按 E 解码再看 |
| 高清版整体错位或发糊 | 分辨率常量没改到 GPU 实际读的那一份。回到 D 第 5 步，查有没有后来的 `Internal::Coherent Mapped Memory Write` |

---

## 参考

- [灰层抓帧会话](GRAY-CAPTURE-SESSION.md)：在一个会话里反复采 frame-audit
- 命令帮助：`pf-node --help`、`pf-node phonefarm gray-layer --help`
- RenderDoc 文档：<https://renderdoc.org/docs/>，Python 接口在 <https://renderdoc.org/docs/python_api/>
- RenderDoc 回放接口的头文件：RenderDoc 源码 `renderdoc/api/replay/renderdoc_replay.h`（v1.46）
