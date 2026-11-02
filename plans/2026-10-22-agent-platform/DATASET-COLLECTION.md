# 高低清（LR/HR）数据集采集方案

更新日期：2026-11-01。对应 2026-10-30 会议待办第 1 项：完善高低清数据集采集方案，写清原理、软硬件需求和操作流程。也对应 [todo](todo.md) 的 METHOD-06。

本文只整理方案。依据是各仓库现有代码和已有记录，没有重新操作手机，也没有重新采集。每种方法是否在真机上跑通，按记录逐项标明。

## 0 结论先行

| 用途 | 推荐方法 | 状态 |
|---|---|---|
| 预训练、快速迭代、覆盖大量场景 | **原生帧模拟降采样**（方法 C）：先采原生高清帧，再在电脑上缩小得到低清 | 管线已在用；数据要重新采 |
| 微调、真实低清的评测真值 | **RenderDoc 同帧双分辨率回放**（方法 A） | 一个游戏 18 对已在真机完成；换游戏要重新摸资源 |
| 检查真实低清与模拟低清的差距 | **Game Mode 真实低清帧**（方法 B），不配对使用 | 截图路径已在真机跑过；只能当不配对数据 |
| 更好的采帧工具 | **层内 framedump**（方法 D），在送显处无损逐帧读回 | 代码已有，**未上机** |
| 管线自检 | **refbench 同帧双分辨率**（方法 E） | 只过了宿主测试，**未上机** |

两次独立冷启动得到的截图不是同一画面，不能当训练对。模拟低清必须标成模拟，不能说成真实低清。

## 1 原理

### 1.1 为什么要配对数据

学习型超分（小 CNN、NPU 模型）用监督训练：输入一张低清帧，输出与同一时刻的高清帧比较，算损失。两张图必须是**同一游戏状态**：同一相机、同一时刻、同一光照，只差渲染分辨率。状态不同，模型学到的是“猜另一帧”，而不是“补细节”。

低清帧最好是游戏**真的按低分辨率渲染**出来的。真实低清有锯齿、LOD 切换、随分辨率变化的后处理和低分辨率 UI。把高清图缩小得到的低清没有这些问题，只是更模糊。只用模拟数据训练的模型，到真机上可能表现变差。

### 1.2 超分接在哪里，决定要什么数据

| 接入点 | 现有实现 | 需要的数据 |
|---|---|---|
| 送显前，处理整张画面（含 UI） | 灰层 selfsr：Catmull-Rom + RCAS；离线对照 SGSR1、FSR1 | 整帧、sRGB、含 UI 的配对 |
| 替换游戏自己的放大 pass（UI 之前） | 灰层 upop / npuop（NPU 超分） | 场景颜色（UI 之前）的配对 |

RenderDoc 回放拿到的是场景 HDR，在 UI 和完整后处理之前，适合第二种接入点。给第一种用时，要另做色调映射和 UI 处理，见 §1.4。

### 1.3 当前分辨率与倍率

以主测机和原神为例（[STEP3](../../experiments/2026-10-24-selfsr/STEP3.md)）：

| 项 | 值 |
|---|---|
| 屏幕 | 2688×1216 |
| 游戏送显（原生） | 2141×969 |
| Game Mode 0.56 时游戏实际渲染 | 1505×681，约为送显的 0.70 倍 |
| selfsr 输出 | 2141×969（省 GPU 用法）或 2688×1216（提画质用法） |
| 之前 NPU 模型 | 2 倍，部署输入 1072×488 或 1134×514 |

截图（`adb exec-out screencap`）拿到的是 HWC 放大后的屏幕图（2688×1216），不是游戏送显的原图。现有离线管线先把截图缩回 2141×969 当真值，这本身是近似。

数据集要按部署时的实际倍率采。2 倍训练的模型不能直接当 1.42 倍用。

### 1.4 可用的采集方法

| | A RenderDoc 双分辨率回放 | B Game Mode 低清 + 原生截图 | C 原生帧模拟降采样 | D 层内 framedump | E refbench 同帧双分辨率 |
|---|---|---|---|---|---|
| 做法 | 抓一帧 RDC，在原设备 GPU 上用同一份几何按低、高两种分辨率重新栅格化 | 不同次冷启动，一次开 Game Mode 降分辨率，一次原生，分别截图 | 采原生帧，电脑上缩小得到低清 | 在 `vkQueuePresentKHR` 处把送显图无损读回、落盘 | 同一 frame index 下，直接渲染 1280×720 和 1920×1080 两张 |
| 怎么对齐 | 同一捕获；固定像素中心映射；用几何和深度建有效掩码。不靠平移搜索 | **对不齐**。冷启动后时辰、云、角色、镜头都会变 | 构造即对齐；需固定缩放核和像素中心约定 | 本身只采单路；配对要配合 B 或 C | 同一命令缓冲，同一相位 |
| 低清是否真实 | 是，真实 GPU 渲染 | 是，游戏真实低分渲染（含低分 UI），但截图经过 HWC 放大 | **否**，合成 | 取决于开不开 Game Mode | 是，但内容是程序化场景，不是游戏 |
| 覆盖范围 | 场景 HDR、depth、aux；不含透明物、完整后处理和 UI | 整帧含 UI | 整帧含 UI | 整帧含 UI，连续帧有帧号 | 白盒程序化内容 |
| 成本 | 高：约 620 MB/帧；每个游戏要从 RDC 重新找 pass、资源和常量；原设备回放一帧 VmSize 约 6.38 GB | 低：已有脚本 | 最低 | 低：送显图 2141×969 RGBA8 约 8.3 MB/帧 | 低 |
| 真机状态 | **已验证**：终末地 18 对（2026-10-13） | 截图采集**已在真机跑过**（STEP3、baseline_cmp）；作为训练对**已证明不可用** | 管线**已在用**：NPU 模型、SR_LOOP 数据集 A、离线画质评估 | **未上机** | **未上机**，只过了宿主测试 |
| root | 商业包不可调试时要 root；可调试包按 RenderDoc 普通流程 | 现有脚本按 root 设计 | 取决于怎么采原生帧：截图不要 root | 要挂层：root，或可调试包走 GPU 调试层 | 不要 root（自编可调试 APK） |
| 依据 | [Vulkan 数采手册](VULKAN-DATA-COLLECTION.md) §2、§7 | [STEP3](../../experiments/2026-10-24-selfsr/STEP3.md)、`selfsr/baseline_cmp.sh` | `selfsr/baseline_quality.py`、`sr_quality.py`、[NPU 训练脚本](../../experiments/2026-10-20-npu-sr/train_sr.py)、[SPEC_SR_LOOP](../../repos/phonefarm/docs/SPEC_SR_LOOP.md) §4 | `selfsr/gray/layer/framedump.cpp` | [NATIVE_PAIRED_CAPTURE](../../repos/refbench/NATIVE_PAIRED_CAPTURE.md) |

各方法的补充说明：

**A RenderDoc 回放。** 是目前唯一在真机上做成的“真实低清 + 同状态高清”路线。已有 18 对是 1782×810 / 2376×1080 的场景 HDR 先导数据，三段轨迹都在同一区域。scene01、scene02 训练，scene03 整段留出。格式细节（R11G11B10 HDR、D24S8 depth）只适用于那批捕获。原始 RDC 当时留在手机上，没有备份到电脑；训练包（451.5 MB，含 18 对和加载工具）在研究工作区。历史脚本绑定旧设备、旧 boot 和旧包，只能复用流程。

**B Game Mode 截图。** STEP3 记录：Game Mode 缩小窗口后，进世界的点击落点不同，6 个超分臂的角色和镜头都和原生臂不同；同为原生的相邻两臂整屏 SSIM 只有 0.49–0.73。所以 `baseline_quality.py` 默认的“hwc 截图当低清输入”只适合看效果，主指标用的是同帧模拟口径（`same_frame_sim`）。B 的价值是提供**真实低清的样子**，用来检查模拟低清的域差。改游戏内渲染精度（如 `genshin_prefs.py`）也要停游戏，问题相同。

**C 模拟降采样。** 已有三处用法，缩放核各不相同：

| 用法 | 缩放 | 规模与结果 |
|---|---|---|
| NPU 模型 `train_sr.py` | 2×2 平均，2 倍 | 原神截帧 `ref_*.png`，最后 4 张做测试；测试 PSNR：双线性 36.84、双三次 39.69、模型 fp32 42.09、int8 41.77 dB。原始截帧没有入库 |
| SR_LOOP 数据集 A | PIL 抗混叠 bicubic，2 倍 | 150 帧，切 256×256 块，丢弃率 0%，bicubic 基线 37.286 dB |
| `sr_quality.py`、`baseline_quality.py --truth-only` | BOX | 离线评估 selfsr、SGSR1、FSR1，不用于训练 |

C 的低清不含真实低分渲染伪影。数据要标 `source.kind=hr_downsampled`，并写明缩放核（见 [SPEC_SEQUENCE_QUALITY](../../repos/phonefarm/docs/SPEC_SEQUENCE_QUALITY.md) 的来源枚举）。

**D framedump。** 解决截图的两个问题：截图经过 HWC 放大、一次只能截一张。它在 present 处把每张送显图拷到主机可见缓冲，由写盘线程落盘，丢帧会在 `index.csv` 记 gap。开 Game Mode 时采到的是真实低清送显图，不开时是原生送显图，但两次仍然不是同一状态。它本身不是配对方法，是让 B 和 C 的原料更干净。**风险**：STEP1 记录原神会杀层里的线程，而 framedump 用了写盘线程，上机前不能假设可用。

**E refbench。** 同帧对齐最严格，适合当已知答案来检验采集、导入和评测管线，不能代表真实游戏画质。

不在本方案内的路线：给自编或可调试的游戏（如可调试版 Megacity）在引擎里同一帧渲染两种分辨率。仓库里没有实现，列入 §5 待定。

## 2 软硬件需求

### 2.1 硬件

| 项 | 要求 | 用于 |
|---|---|---|
| 手机 | **可 root 的 Android 手机**，Vulkan，最好高通 GPU（已有记录都在高通平台）；支持 Android Game Mode 降分辨率 | A、B、D |
| 手机（可选） | 不 root 的 Android 手机，只用可调试包 | C 的截图、E；D 走 GPU 调试层 |
| 手机散热 | 机身风扇或外置风扇；电池温度控制在 40–42 °C 以下 | 过热会降帧率档、弹高温窗，影响 B 和 D |
| 控制电脑 | macOS 或 Linux，USB 直连手机 | 全部 |
| RenderDoc 控制端 | Linux x86_64（历史 SDK 客户端是 linux-x86_64 构建） | A |
| 训练机 | 有 GPU 的 Linux 服务器 | 训练与批量评估 |
| 存储 | 节点大容量盘或存储服务器，见下表 | 原件与派生数据 |

### 2.2 存储估算

| 数据 | 单帧 | 一段 / 一批 | 依据 |
|---|---|---|---|
| RenderDoc RDC | 约 620 MB | 6 帧约 3.7 GB；18 帧 11.17 GB | 实测平均 |
| RDC 派生（低清、高清回放捕获）与暂存 | 再加数 GB / 帧 | 按 RDC 的 1–2 倍预留 | 估算 |
| framedump 原生送显图 2141×969 RGBA8 | 约 8.3 MB | 600 帧（60 帧下 10 秒）约 5.0 GB | 计算值 |
| framedump 低清送显图 1505×681 RGBA8 | 约 4.1 MB | 600 帧约 2.5 GB | 计算值 |
| 屏幕截图 PNG 2688×1216 | 无压缩约 13 MB，PNG 更小 | 按首批实测更新 | 计算值 |
| 模拟低清 | 可从高清加缩放参数重建 | 默认不长期保存，只存配方 | — |
| 先导训练包（18 对） | — | 451.5 MB | 实测 |

framedump 每段开始前会检查剩余空间：需要本段预计大小再加 `framedump_reserve_mb`（缺省 4096 MB）。

建议首版数据集按 **原生高清 10,000 帧左右、RenderDoc 真实对 100 帧左右** 规划，原件约 150–200 GB，派生和暂存另算。

### 2.3 软件

| 软件 | 版本 / 位置 | 部署说明 |
|---|---|---|
| phonefarm（设备层：adb、设备锁、`capture`、`gray-layer session`） | HGamey/hgamey 的 `phonefarm/` | [M1 设备层](../../guide/01-device-layer.md) |
| 节点服务与 `pf-node` | HGamey/hgamey 的 `harness-ops/` | [M2 节点服务](../../guide/02-node-service.md)、[M4 Agent 接入](../../guide/04-agent.md) |
| 灰层（selfsr、framedump） | HGamey/experiments 的 `selfsr/gray/`，NDK 现编 | [M6 自研超分](../../guide/06-selfsr.md) |
| Android NDK | r28c 已验证 | 同上 |
| RenderDoc | 历史成功组合为 1.46（设备侧组件、layer、SDK） | [Vulkan 数采手册](VULKAN-DATA-COLLECTION.md) §3、§4 |
| refbench | HGamey/hgamey 的 `refbench/` | [NATIVE_PAIRED_CAPTURE](../../repos/refbench/NATIVE_PAIRED_CAPTURE.md) |
| 离线管线 | Python 3 + numpy + Pillow：`selfsr/baseline_quality.py`、`sr_quality.py`、`sgsr1.py`、`fsr1.py` | — |
| 训练 | TensorFlow（`train_sr.py` 用 Keras，导出 int8 TFLite） | — |

每种方法要不要 root：

| 方法 | 要 root | 不 root 能否做 |
|---|---|---|
| A | 商业包（不可调试）要 | 只能用可调试包 |
| B | 现有 `selfsr_ab.py` 按 root 设计 | 未验证 |
| C | 不要（截图即可） | 能 |
| D | 挂层要 root；读游戏私有目录要 root | 可调试包可用 GPU 调试层挂层（`noroot_smoke.sh` 的方式），未与 framedump 一起验证 |
| E | 不要 | 能 |

## 3 操作流程

### 3.1 总流程

```text
定场景清单 → 采原生高清（C 原料） → 生成模拟对 + QA → 发布 v0（模拟）
        ↘ RenderDoc 抓帧 → 双分辨率回放 → 掩码 + QA → 加入真实对（A）
        ↘ Game Mode 真实低清帧（B/D） → 域差检查（不配对）
```

每一批都在持有设备租约的平台任务里做，产物写任务输出目录（`PF_JOB_OUT`，即 job `@out`）。不在任务外直接改手机设置。

### 3.2 准备（每批一次）

1. 按 [M1](../../guide/01-device-layer.md)、[M6](../../guide/06-selfsr.md) 装好 phonefarm、NDK 和实验仓库。
2. 记录：游戏包名与版本、游戏内画质设置、屏幕和送显分辨率、系统和 GPU 驱动、是否 root。
3. 先在手机上手动进一次游戏，确认能进大世界。
4. 只读确认节点和设备：

```sh
pf-node node
pf-node devices
pf-node queue
```

### 3.3 方法 C：采原生高清帧，生成模拟对

**采帧。** 原神可以用 phonefarm 自带的巡航截图。它只保留横屏、大世界探索态、小地图和技能栏都在的帧，并去掉画面冻结的帧（64×36 灰度缩略图平均差小于 1.0）：

```sh
phonefarm capture --serial <串号> --out <目录> --frames 200 --settle-ms 800 --json
```

- 输出 `manifest.jsonl`（每帧的文件、时间、状态、尺寸、字节数、sha256、segment）和 `capture.json`。
- 每 10 张保留帧记为一个 segment，划分数据集时同一 segment 不跨集。
- 退出码：0 收满，1 步数用尽，2 用法或设备错误。
- 这条命令只认原神。其他游戏先用 `selfsr_ab.py` 的每臂截图，或等 framedump 上机后改用它。

**生成离线对照与模拟对。** 单张原生截图：

```sh
python3 -I selfsr/baseline_quality.py --truth-only --native <原生截图.png> --out <目录>
```

输出真值 `gt.png`、模拟低清 `low_input.png`（BOX 缩到 `--low-size`，缺省 1505×681），以及双线性、SGSR1、FSR1、selfsr 四种放大结果和 `baseline_quality.json`。

多张截图批量比较 selfsr 算子：

```sh
python3 -I selfsr/sr_quality.py --scale 0.7 --sharp 0.8 --out q.json <原生截图.png>...
```

**训练用的切块和划分。** SR_LOOP 用的 `sr_loop/dataset.py` 不在 HGamey 的仓库里。要先把 §3.7 的 QA 规则写成实验仓库里的脚本，再批量生成训练集。之前的 NPU 训练脚本只按文件名把最后 4 张留作测试，没有按场景划分，不再沿用。

### 3.4 方法 B：采 Game Mode 真实低清帧（不配对）

用已有的配对运行脚本。`hwc` 组是“原生 vs Game Mode 0.56 + 系统放大”，每臂窗口结束时截图，可选录屏：

```sh
# 本机
sh selfsr/baseline_cmp.sh <串号> SETS=hwc ROUNDS=6 VIDEO=20

# 经平台
pf-node --experiment main --research <研究ID> --device-access write --detach \
  experiment selfsr/baseline_cmp.sh <串号> SETS=hwc ROUNDS=6 VIDEO=20
```

- 产物：`shots/<臂>.png`（屏幕分辨率截图）、`traces/<臂>_video.mp4`、`report.md`、`restore.json`。
- 用前核对：`cmd game list-configs` 里确有 downscale；HWC 层表里游戏图层的 Source Crop 确实变小。
- 平台任务结束后停在 blocked。核对 `restore.json` 后再释放手机。
- 这些帧只用于 §3.8 的域差检查和人工评价，不进训练对。

### 3.5 方法 D：framedump（未上机，先做验证）

framedump 的开关都是系统属性，挂层前设置：

| 属性 | 作用 |
|---|---|
| `debug.knobs.framedump=1` | 总开关，游戏建 VkInstance 时读 |
| `debug.knobs.framedump_go=<标签>` | 运行中改一次，开始一段；只取 `[A-Za-z0-9_-]` |
| `debug.knobs.framedump_n` | 每段帧数，缺省 600，上限 36000 |
| `debug.knobs.framedump_slots` | 读回槽数，缺省 8 |
| `debug.knobs.framedump_dir` | 输出目录，缺省 `/data/data/<包名>/files/i3dump` |
| `debug.knobs.framedump_reserve_mb` | 剩余空间余量，缺省 4096 |

每段输出：`session.json`（宽高、格式、preTransform）、`f<10 位帧号>.raw`、`index.csv`（seq、frame、时间、gap）、`summary.json`（这段完成的标志）；空间不够时写 `refused.json`。

现在没有封装好的采集脚本。上机验证的最小步骤：

1. 用 `gray-layer session` 挂层，只开 framedump，不开 selfsr 和插帧。
2. 进世界后设一次 `framedump_go`，等 `summary.json` 出现。
3. 拉回整段目录，核对帧数、gap、格式；取一帧转 PNG，与同时刻截图缩回送显尺寸后比较。
4. 确认游戏没有杀掉写盘线程，帧率下降在可接受范围内（采集时的帧率不作为性能结果）。
5. 收尾：层已摘、`debug.knobs.*` 已清、输出目录已拉回并删掉。
6. 再验证开 Game Mode 时采到的是低清送显图（1505×681 左右），以及 selfsr 同时开着时 framedump 读到的是哪一张图。

验证通过后，再写成实验仓库里的脚本，替代 §3.3 的截图采帧。

### 3.6 方法 A：RenderDoc 双分辨率回放

完整步骤见 [Vulkan 数采手册](VULKAN-DATA-COLLECTION.md)，这里只列顺序：

1. §4：先采一个可回放的单帧，SDK 能打开，原设备能回放。
2. §5：扩到短序列（一次 6 帧），覆盖静止、转视角、平移、遮挡变化。
3. §7：从当前 RDC 找场景 pass、附件和分辨率常量；导出母捕获的原生 color、depth、aux；改渲染尺寸；先做同尺寸低清控制回放，确认与母捕获一致，再导出高清。派生 RDC 另存，母捕获不动。
4. 固定像素中心映射，用几何和深度建有效掩码。
5. §8：按手册的目录布局和清单交付。

注意：

- 不能把已有 810p 图片缩放后当 600p 真实输入。新倍率要重新回放。
- 场景 HDR 的预览变换 `(HDR/(1+HDR))**(1/2.2)` 不是游戏的色调映射。给整帧 sRGB 模型用时，要另定统一的色调映射，并在清单里写明。
- aux、相机、曝光的语义没证明前，按原名保存，不改名成光流或线性 HDR 标签。

### 3.7 场景覆盖

每个游戏按下表组合选场景。每个组合至少一段，每段 6–10 秒（或 6 帧 RDC）。

| 维度 | 取值 |
|---|---|
| 场景类型 | 城镇和建筑（直线边、文字招牌）；植被（细密高频）；水面和反射；岩石地形；室内；战斗特效和粒子；远景天空 |
| 运动 | 静止；慢转视角；快转视角；角色奔跑；飞行、滑翔或载具 |
| 光照 | 白天；黄昏；夜晚；阴天或雨；室内暗光；强逆光 |
| HUD | 标准 HUD；HUD 区单独标注（小地图、技能栏、队伍栏、顶栏） |

- 菜单、对话、加载页、过场单独存放，不进主训练集。
- 一个画面里 HUD 和场景要分开评价。HUD 区域在 SR_LOOP 里有原神的实测排除框，可作为起点，换游戏要重新标。
- 游戏：原神（主测）；Megacity（可调试，可走不 root 路径）；终末地（已有 RDC 先导数据）。新增游戏先采 1 段确认流程。
- 记录游戏内时辰、天气和位置，方便以后回到同一地点补采。

### 3.8 质量检查（QA）

| 检查 | 规则 | 不通过时 |
|---|---|---|
| 来源 | `source.kind` 必填：真实对为 `real_game_capture`，模拟对为 `hr_downsampled` 并写缩放核 | 不入库 |
| 完整性 | 尺寸、格式、字节数、sha256 与清单一致；framedump 有 `summary.json`；refbench 有 `completion.json` | 整段作废 |
| 有效画面 | 横屏、在大世界、非冻结、非菜单或加载页 | 丢帧 |
| 对齐（模拟对、截图类） | 低清用 bicubic 放大回高清尺寸，在中央区做相位相关，`|dx|,|dy| ≤ 0.25 px` | 整帧丢弃 |
| 对齐（RenderDoc 对） | 同尺寸低清控制回放与母捕获一致；固定像素映射；有效掩码覆盖率记录在清单里。不靠平移搜索或曝光拟合 | 该对不入库 |
| PSNR 下限 | 逐块 bicubic 重建 PSNR > 28 dB（SR_LOOP 规则，2 倍、模拟对）。真实对的下限由先导批次定 | 丢块 |
| 熔断 | 丢弃块（含对齐丢弃帧的块）占比 > 5% | 整批不发布，查原因 |
| 基线锁定 | 每个版本记录验证集上双线性、bicubic 的 PSNR / SSIM，以后的模型都和它比 | — |
| 域差 | 方法 B / D 的真实低清帧与同场景的模拟低清，比较锐度（拉普拉斯方差）、边缘和锯齿统计；人工看图 | 记录差距，决定真实对的权重 |
| 设备收尾 | 层已摘、属性已清、Game Mode 已复原，`restore.json` 通过 | 不释放手机，按原任务恢复 |

### 3.9 训练、验证、测试的划分

- **按场景或路线划分，不按帧划分。** 相邻帧、同一帧的裁块、重复回放都不算独立样本。
- 测试集：整段留出若干场景（每种场景类型至少一个）；有条件时整款游戏留出一款，测泛化。
- 验证集：沿用 SR_LOOP 的做法，按 segment 编号固定抽取（如 `segment % 5 == 4`），同段不跨集。
- 真实对和模拟对分开报告：至少给出“模拟测试集”和“真实测试集”两组数字。真实测试集只用 RenderDoc 对。
- `split.json` 写清每帧属于哪一集、块坐标、阈值和数据指纹，版本发布后不改，只出新版本。

### 3.10 命名与清单

目录沿用 [Vulkan 数采手册](VULKAN-DATA-COLLECTION.md) §8 的布局，增加超分专用的字段：

```text
dataset/<数据集名>-v<版本>/
  manifest.json          # 版本、来源批次、全部帧的索引
  split.json             # 划分、阈值、指纹、锁定基线
  qa_report.json         # 每项 QA 的计数和丢弃原因
  batches/<batch-id>/    # 一批采集一个目录，batch-id = <游戏简称>-<YYYYMMDD>-<序号>
    <scene-id>/          # 如 s03-forest-pan-day
      hr/f<10 位帧号>.png
      lr/f<10 位帧号>.png       # 真实低清；模拟低清不长期保存
      mask/f<10 位帧号>.png     # RenderDoc 对的有效掩码
      meta.json
```

`meta.json` 每帧至少包括：

| 类别 | 字段 |
|---|---|
| 来源 | batch / scene / 帧号，node ID、job ID，游戏包名与版本，系统和 GPU 驱动 |
| 方法 | `source.kind`、`capture_method`（如 `renderdoc_replay`、`framedump`、`screencap`）、`derivation`（模拟时写缩放核和倍率） |
| 尺寸 | LR、HR 宽高，倍率，色彩空间（sRGB8 / 场景 HDR），preTransform |
| 场景标签 | 场景类型、运动、光照、HUD 状态、时辰天气 |
| 校验 | 每个文件的字节数和 sha256，对齐结果，PSNR，掩码覆盖率 |

字段不知道就写 `null`，不填猜测值。

### 3.11 数据放在哪

- **数据不进 git。** 实验仓库和文档仓库只放代码、方案和报告（[文档仓库规则](../../README.md)、HGamey/experiments 的 README）。
- 原件随平台任务产物留在节点的任务输出目录。大文件（RDC、framedump 整段）放节点大容量盘或存储服务器，清单里写清取回方式。
- 采集完成后，把清单和报告通过现有结果登记关联到研究（见 [结果索引](RESULT-INDEX.md)），否则别人找不到。
- 数据集版本（`manifest.json`、`split.json`、`qa_report.json`）和原件分开存。训练机按清单取数据，不在电脑上重复全量下载、解压。
- 游戏画面受版权约束，只在项目内部使用；公开材料只放脱敏预览。

## 4 验收

| 阶段 | 内容 | 验收标准 |
|---|---|---|
| P0 模拟集 v0 | 方法 C，原神 + 一款其他游戏 | 至少 8 个场景组合；对齐和熔断规则通过；`split.json` 和锁定基线已出；清单字段齐全；数据已登记到研究 |
| P1 framedump 上机 | 方法 D，§3.5 的 6 步 | 一段 600 帧完整落盘，gap 有记录；与截图一致；游戏不崩，层摘净；开 Game Mode 时确认是低清送显图 |
| P2 RenderDoc 复现 | 方法 A，换一个游戏或新场景 | 单帧低清控制回放与母捕获一致；至少 3 个场景、每个 6 帧的真实对；掩码和格式记录齐全 |
| P3 数据集 v1 | 模拟对 + 真实对 + 不配对真实低清 | 同一模型在留出场景上，模拟测试集和真实测试集都比 bicubic 高 0.5 dB 以上（沿用 SR_LOOP Gate 2 的门槛）；域差检查结果写进报告 |
| 每一批 | 设备收尾 | `restore.json` 或收尾记录通过后才释放手机 |

## 5 风险与待定

| 项 | 说明 | 处理 |
|---|---|---|
| framedump 在原神里可能不能用 | 原神会杀层里的线程，framedump 有写盘线程 | P1 先验证；不行就改为单线程分帧写，或只在其他游戏用 |
| 真实对少 | RenderDoc 每帧约 620 MB，每个游戏都要重新摸资源，人工多 | 真实对只做微调和评测，量以百计；训练主力用模拟对 |
| 真实对只到场景 HDR | 不含 UI 和完整后处理，与送显前超分的输入不一致 | 先服务替换游戏放大 pass 的路线；整帧路线用模拟对 + 真实测试集 |
| 模拟对的域差 | 模拟低清没有锯齿、LOD 等真实伪影 | 用 B / D 的真实低清做域差检查；可在模拟时加锯齿或降质增广，效果待验证 |
| 截图不是送显原图 | 截图经过 HWC 放大，缩回送显尺寸是近似 | framedump 上机后改用送显原图 |
| 跨冷启动无法配对 | 时辰、云、镜头都会变 | 不用 B 做训练对；写进 QA 规则 |
| 防作弊和用户协议 | 挂层、RenderDoc 抓商业游戏有风险；之前挂层未被拦，不代表以后也不会 | 只用测试账号和测试手机；被拦就停，换可调试游戏 |
| 游戏更新 | 更新后库目录、pass 结构、资源都可能变 | 清单记录游戏版本；RenderDoc 流程每次从当前 RDC 重新找资源 |
| 原始 RDC 未备份 | 18 个 RDC 当时只在手机上 | 使用前由当前手机负责人核对是否还在；新采的 RDC 采完就转存 |
| 倍率不统一 | selfsr 是约 1.42 倍或 1.79 倍，NPU 模型是 2 倍 | 先定部署倍率，再采数据；不同倍率分开出集 |
| 色彩空间 | 场景 HDR 与 sRGB 截图不能混在一起训练 | 统一色调映射前分开存放 |
| 切块脚本不在仓库 | `sr_loop/dataset.py` 在外部仓库 | 在实验仓库重写 QA 和切块脚本，P0 前完成 |
| 待定：引擎内双渲染 | 自编或可调试游戏（如 Megacity）可在引擎里同帧渲染两种分辨率，得到含 UI 的真实对 | 未实现，评估工作量后再定 |
| 待定：refbench 同帧对 | 代码已有，没在真机跑过 | 上机后作为管线的已知答案，不当游戏数据 |
| 待定：时序标签 | 插帧和时序超分要相机、运动矢量、jitter 等标签，现在没有 | 不在本方案范围；需要时另写方案 |

## 6 资料索引

| 内容 | 位置 |
|---|---|
| RenderDoc 与 frame-audit 采集、18 对数据范围、交付布局 | [VULKAN-DATA-COLLECTION.md](VULKAN-DATA-COLLECTION.md) |
| refbench 同帧双分辨率采集（未上机） | [NATIVE_PAIRED_CAPTURE.md](../../repos/refbench/NATIVE_PAIRED_CAPTURE.md)、[SPEC_NATIVE_PAIRED_CAPTURE.md](../../repos/refbench/docs/SPEC_NATIVE_PAIRED_CAPTURE.md) |
| `phonefarm capture`、对齐和切块规则、数据集 A | [SPEC_SR_LOOP.md](../../repos/phonefarm/docs/SPEC_SR_LOOP.md) §4 |
| 质量评测的来源枚举和配对账本 | [SPEC_SEQUENCE_QUALITY.md](../../repos/phonefarm/docs/SPEC_SEQUENCE_QUALITY.md) |
| 灰层、Game Mode、A/B 测量与截图 | [HARNESS_GUIDE.md](../../experiments/2026-10-24-selfsr/HARNESS_GUIDE.md)、[STEP1](../../experiments/2026-10-24-selfsr/STEP1.md)、[STEP3](../../experiments/2026-10-24-selfsr/STEP3.md) |
| NPU 超分模型与训练脚本 | [experiments/2026-10-20-npu-sr](../../experiments/2026-10-20-npu-sr/) |
| 离线画质管线、SGSR1 / FSR1 移植、framedump、`baseline_cmp.sh` | HGamey/experiments 的 `selfsr/`（`baseline_quality.py`、`sr_quality.py`、`sgsr1.py`、`fsr1.py`、`baseline_cmp.sh`、`gray/layer/framedump.cpp`） |
| 部署与模块安装 | [guide/](../../guide/README.md) |
