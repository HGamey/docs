# 手机 Vulkan 数据采集教程

本教程有两种采法，按需要选一种：

| 采法 | 最后得到 | 手机在哪 |
|---|---|---|
| **一、frame-audit** | `fa_1.json`：一帧里每个 pass 的 draw、dispatch、附件、读写、清屏；`analysis.md`：可疑操作清单 | 平台中心上的手机，用 `pf-node` 远程操作 |
| **二、RenderDoc** | `.rdc`：一帧完整的 Vulkan 调用、资源和 Shader，可以回放 | 手机用 USB 接在你自己的电脑上 |

两种采法不要同时用在同一个游戏上。

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

### 为什么能得到高低清两个版本

截两张不同分辨率的图，得到的是两个不同的瞬间：角色动了，镜头动了，粒子和动画也不一样，没法逐像素对应。

重放不一样：`.rdc` 里存的不是像素，而是**画这一帧的全部指令和数据**，包括几何、相机矩阵、材质、动画时刻。只要在重放前改掉跟分辨率有关的部分，同一组指令就会在另一个分辨率下重新画一遍。需要改的有：

- 渲染目标图像的尺寸，以及对应的 framebuffer 和内存分配；
- viewport 和 scissor；
- Shader 常量里写死的分辨率值。CPU 后面写进去的值也要一起改，否则重放时会读到旧值。

这样得到的低清和高清，是**同一个瞬间、同一份几何、同一个相机**，在两种像素网格上各画一遍。它们是真正对齐的一对，不是把一张图放大或缩小得来的。

做配对时还要注意三点：

- **像素中心对齐。** 低清的第 `x` 个像素对应高清的位置是 `(x + 0.5) × 倍数 − 0.5`，不是 `x × 倍数`。
- **有效区域掩码。** 有些效果依赖分辨率，比如屏幕空间反射、时间性抗锯齿的历史帧、UI。这些区域两边不一定一致，要用深度和几何算出可信区域。
- **改尺寸后先验证。** 用原尺寸重放一次，确认结果和原始捕获一样，再改尺寸。原始 `.rdc` 不动，改过的另存一份。

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

---

## 参考

- [灰层抓帧会话](GRAY-CAPTURE-SESSION.md)：在一个会话里反复采 frame-audit
- 命令帮助：`pf-node --help`、`pf-node phonefarm gray-layer --help`
