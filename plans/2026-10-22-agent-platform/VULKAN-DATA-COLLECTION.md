# 手机 Vulkan 数据采集教程

本教程教你在 HGamey 平台上采集手机游戏的 Vulkan 渲染数据。有两种采法：

| 采法 | 得到什么 | 用来做什么 |
|---|---|---|
| **frame-audit**（先用这个） | `fa_*.json`：每帧的 pass、draw、dispatch、附件、读写、清屏摘要；可选 GPU 时间戳 | 找可疑的多余渲染操作（白存、白读、白清、白画），定优化方向 |
| **RenderDoc 抓帧** | `.rdc`：一帧完整的 API 调用、资源内容、管线和 Shader 状态 | 回放这一帧，导出中间缓冲，做同帧替换实验和高低清训练对 |

截图和录屏只有画面，没有 Vulkan 调用。这两种采法不能同时挂在同一个游戏上，要分开采。

## 准备

1. 配好 `pf-node`：设置 `PF_NODE_URL`、`PF_NODE_ID`、`PF_NODE_TOKEN_FILE`。检查能连上：

   ```sh
   pf-node checkin
   ```

2. 选手机：

   ```sh
   pf-node phones
   ```

   要找的手机必须满足：
   - 在线（`online=true`）、空闲、有 root。注入层要 root。
   - 在**中心**上。目前只有中心编好了注入层（`libVkLayer_refknobs.so`）。

   记下它的 `ref`，比如 `d944a2c6…/REDMAGIC`。

3. 确认游戏已经装好，并且能进到你要采的场景：
   - 不能停在菜单、加载页或黑屏。
   - 角色不能被挡住，要能动起来。

4. 在任务页认领或创建一个研究任务，记下研究 ID。采集作业都带上它，任务页才能显示。

## 采法一：frame-audit

### 1. 提交采集

```sh
pf-node --research <研究ID> --device <手机ref> --request-id fa-<场景>-001 --detach \
  phonefarm frame-audit capture \
    --serial @serial --pkg <游戏包名> \
    --captures 1 --gap 20 --settle 60 \
    --out @out --json
```

| 参数 | 作用 |
|---|---|
| `--captures N` | 采几轮，每轮之间隔 `--gap` 秒 |
| `--settle 秒` | 启动游戏后等多久再采。只是等待，不会自动进场景；原神默认 15 秒，其他游戏默认 60 秒 |
| `--tap "x,y;x,y"` | 等待时依次点击这些坐标，用来点过开场 |
| `--extras "…"` | 启动时带的 intent 参数，适用于能直接进场景的测试包 |
| `--ts` | 额外记录每个 pass 的 GPU 时间戳 |

命令返回作业 ID。

时间戳批次单独采：不要把加了 `--ts` 和没加的结果放在一起比。

### 2. 等结果，取回文件

```sh
pf-node status <作业ID>
pf-node wait <作业ID>
pf-node out <作业ID>        # 下载到 ./pf-node-out/<作业ID>/
```

如果网络断了、不确定有没有提交成功，用 `pf-node request fa-<场景>-001` 找回原作业，不要换一个 request-id 重新提交。

你会得到：
- `fa_*.json`：每帧的渲染步骤
- 采集前后的截图
- 会话和收尾记录

### 3. 分析

```sh
phonefarm frame-audit analyze pf-node-out/<作业ID>/…/fa_1.json \
  --md analysis.md --json analysis.json
```

你会得到：`analysis.md`，里面是步骤总览和可疑操作清单。

### 4. 读结果时检查三件事

1. **记全了没有。** 看 `diag` 里的 `cb_unseen`、`ev_overflow`、`ts_skipped`。不为 0，说明这一帧没记全，结论只适用于记到的部分。
2. **时间可不可信。** 看 `ts_enabled`、`ts_available` 和每帧的 `passes_timed`。`passes_timed=0` 时显示的 `0.0 ms` 表示没测到，不是零耗时。各 pass 的时间加起来也不等于整帧时间。
3. **是模型还是实测。** LOAD/STORE 的字节数是按尺寸和格式算出来的带宽估计，没算硬件压缩。描述符绑定了，也不代表 Shader 真的读了。可疑操作要再做一次改动实验才能确认。

需要在一个会话里反复采时，用灰层会话的 `frameaudit` 或 `frameaudit-ts` 模式，见 [灰层抓帧会话](GRAY-CAPTURE-SESSION.md)：

```sh
pf-node --research <研究ID> --device <手机ref> --detach \
  phonefarm gray-layer session --serial @serial --package <游戏包名> \
    --mode frameaudit --out @out --max-seconds 900 --power-policy battery
```

## 采法二：RenderDoc 抓帧

**先采一帧，确认能回放，再采短序列。**

### A. 游戏有可调试版本

1. 在电脑上打开 RenderDoc，选 Android Remote Context，确认 adb 能看到手机。
2. 让 RenderDoc 在手机上装好配套组件，选中游戏包名并启动。
3. 进入要采的场景，确认游戏走的是 Vulkan，然后触发一帧捕获。
4. 等 RenderDoc 收到新捕获，记下 `.rdc` 路径、帧号和文件大小。
5. 在同一台手机上打开这个 `.rdc` 并回放，确认画面对、关键 pass 在、纹理能导出。

### B. 商业包（不可调试），只限 root 手机

以下操作都放在一个持有这台手机的作业里做（`pf-node --device <手机ref> …`），不要绕开平台直接操作手机。

1. **记下现状。** 记录要改的系统属性（`debug.vulkan.layers` 等）、GPU 调试设置、游戏安装目录里已有的层文件、已有的 adb forward。收尾时按这份记录恢复。
2. **放层。** 停掉游戏，把 ABI 匹配的 RenderDoc 层（`libVkLayer_GLES_RenderDoc.so`）放进游戏自己的 native library 目录，权限和 SELinux 标签照原目录设置。游戏更新后这个目录会变，要重新放。
3. **打开层，启动游戏。** 设置 `debug.vulkan.layers=VK_LAYER_RENDERDOC_Capture` 和 `debug.rdoc.IGNORE_LAYERS=0`，然后启动游戏。设置前确认没有别的作业也在用这些属性。
4. **确认连上的是这次的游戏进程。** 核对新进程的 PID 和启动时间，确认它加载了 RenderDoc 层，再找到它对应的 `renderdoc_<编号>` socket。编号不等于 PID。
5. **建立转发。** 用 `adb forward` 把这个 socket 转到本机端口，记下这条映射。
6. **触发捕获。** 用 RenderDoc 的 TargetControl 连上，确认 PID 正确、API 是 Vulkan，然后触发一次 `TriggerCapture(N)`。结果不确定时，先查有没有收到，不要重复触发。
7. **收文件。** 每收到一个捕获，记下捕获 ID、帧号、API、时间、路径、大小和 SHA256，再用 RenderDoc 打开检查一遍。
8. **收尾。** 断开连接，停掉这次启动的游戏，删掉这次放的层文件和 adb forward，把属性恢复成第 1 步记下的值。"采集完成"和"恢复完成"分开记录。

### 从一帧扩展到短序列

- 按这个顺序扩：静止细节 → 镜头旋转 → 角色平移 → 遮挡变化 → 换地图、材质、光照。
- 一次触发 N 帧，实际收到几帧就记几帧，缺的不要拿别的帧补。
- 保存输入轨迹、触发和接收时间，以及每段前后的截图。
- 输入发出去了，不代表角色真的动了，要看截图确认。
- 抓帧会卡住游戏，所以帧号连续不代表帧间隔是正常的 1/30 或 1/60 秒。

### 容量和回放资源

- 每个 `.rdc` 大约 600 MB，6 帧一组大约 3.7 GB。导出和回放还要额外的空间。
- 在手机上回放一帧，大约需要 6.4 GB 虚拟地址空间、1.2 GB 常驻内存。4 GiB 地址上限不够用。
- 尽量在手机或大容量 Linux 机器上处理，只把需要的缓冲和清单传回来。

## 进阶：生成同帧高低清训练对

只交付 `.rdc` 的话，到"能回放"就完成了。需要训练数据时再做这一步：

1. 从这一份 `.rdc` 里找出目标场景 pass、附件、viewport/scissor、Shader 和实际绑定的常量。不要沿用别的捕获里的资源 ID 或 event ID。
2. 导出原生的场景颜色、depth 和 aux，记下格式、尺寸、event、mip/slice/sample。
3. 改渲染尺寸，以及对应的 framebuffer、内存分配和分辨率常量。要连带处理后面写入的 coherent/flush 数据，否则还会读到旧值。
4. 先用原尺寸回放一次，确认输出和原来一样，再在原手机上导出高清版。派生的 `.rdc` 另存，不改原件。
5. 用几何和深度生成有效掩码，固定像素中心映射，逐对检查高低清是否对齐。

低清输入要真实渲染出来，不能把高清图缩小当低清。aux 缓冲的含义没有确认前，按原名保存，不要叫它"光流"。

## 交付

### 目录

```text
collection/<批次ID>/
  manifest.json      # 身份、版本、场景、文件清单、各步状态
  capture/           # .rdc 或 fa_*.json 原件；放不下就写明取回位置
  metadata/          # RenderDoc 导出的资源、Shader、绑定信息
  preview/           # 场景截图、回放预览
  raw/               # 可选：原生颜色、depth、aux、常量缓冲
  derived/           # 可选：派生 .rdc、高低清对、掩码
  logs/              # 触发、接收、失败和收尾记录
```

### manifest.json 至少要有

| 类别 | 内容 |
|---|---|
| 来源 | 批次、场景、序列 ID；node ID、作业 ID、request ID；游戏包名和版本；手机 |
| 环境 | GPU、驱动、系统、图形 API、采集工具版本、画质和分辨率 |
| 触发 | 期望帧数、实际帧数、触发顺序和时间、输入轨迹 |
| 原件 | 每个文件的路径、大小、SHA256；`.rdc` 的捕获 ID、帧号、API |
| 回放与导出 | 能否打开、在哪台手机回放、导出了哪些 event 和资源 |
| 状态 | 捕获完成、能打开、回放完成、配对通过、收尾完成，各记一项 |

不知道的字段填 `null`，不要猜。

### 在平台上登记

1. 把 `manifest.json` 和报告写进作业的 `@out`。
2. 交付研究任务：

   ```sh
   pf-node research deliver <研究ID> --summary report.md --artifact job:<作业ID>
   ```

3. 很大的 `.rdc` 留在手机或大容量机器上，在 manifest 里写清怎么取回。不要用 `--inputs` 上传。

## 验收清单

- **frame-audit：** JSON 能读；游戏和场景写清楚；确实有帧、有步骤；`diag` 已记录。
- **RenderDoc：** 文件完整；能打开，显示 Vulkan 和正确的帧；至少一帧在原手机上回放成功。
- **图像：** 导出的纹理尺寸、格式、字节数对；预览能看出是目标场景。截图不能代替原生纹理。
- **训练对：** 单独检查对齐和掩码。生成了 `.rdc` 不等于训练数据合格。
- **收尾：** 层、属性、端口转发都已恢复。恢复没确认就保持阻断，不要清锁了事。

## 常见问题

| 现象 | 怎么办 |
|---|---|
| 手机在线，但抓不到游戏 | 确认游戏走的是 Vulkan、连的是这次的进程、层已加载；可调试版本的做法和 root 的做法不能混用 |
| 有截图，没有 `.rdc` | 捕获没有完成；查 TargetControl 有没有收到新捕获 |
| 连续几帧画面没变化 | 看镜头和角色是否真的动了；排除菜单、障碍物和按键映射问题 |
| 电脑能打开 `.rdc`，手机回放失败 | 能打开和能回放是两回事；在原手机上用 native replay，失败就如实记录 |
| 回放内存不够 | 按"容量和回放资源"一节准备地址空间 |
| 高清版还读到旧分辨率常量 | 检查后面的 coherent/flush 写入是否覆盖了初始内容；以 GPU 回读的实际值为准 |
| frame-audit 时间是 0 | 先看时间戳是否可用；没测到不能当零耗时 |
| frame-audit 报找不到层 | 这台手机所在的节点没有编注入层；换中心上的手机 |
| 收尾失败或状态不明 | 保留原作业和会话记录，用 `pf-node session-recovery` / `session-recover` 恢复；不要重跑整段采集，也不要直接清锁 |

性能要另外测：抓帧、读回纹理、写文件、替换 Shader 回放所花的时间，都不能当作游戏帧时间、FPS 或功耗。

## 参考

- [灰层抓帧会话](GRAY-CAPTURE-SESSION.md)
- [结果索引](RESULT-INDEX.md)
- 命令帮助：`pf-node --help`、`pf-node phonefarm gray-layer --help`
- 代码：[frame-audit 实现](https://github.com/HGamey/phonefarm/blob/7aa39c5a5ebc401430c828b0c8d1d08d57e77e85/src/frameaudit.rs)
