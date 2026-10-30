# 手机 Vulkan 指令与渲染数据采集手册

更新日期：2026-10-30。面向 HGamey 的采集执行者、外部 Agent 和数据使用者。

目标是采集**可分析的 Vulkan 调用、渲染状态与资源**，用于定位渲染开销、离线回放和生成同状态图像数据。推荐先采一个可回放帧，确认场景和输出正确，再扩展为短序列；需要排查多余渲染操作时，先用较轻量的 frame-audit。

本手册依据 2026-10-11～13 的真机采集记录，以及 2026-10-26 已交付的 phonefarm/harness 接口整理。本次仅整理文档，未重新采集或检查手机上的原始文件。示例命令需要部署节点实际支持对应入口。

## 1 先选择采什么

“Vulkan 指令”在本项目指应用与 Vulkan API 之间的调用及其相关状态，不是 Adreno 等 GPU 的硬件机器指令。SPIR-V 也是着色器中间表示，不能直接当作硬件 ISA。

| 路线 | 核心输出 | 可以做什么 | 使用边界 |
|---|---|---|---|
| RenderDoc 抓帧 | `.rdc`，捕获帧相关的 API 调用、资源内容、管线与 Shader 状态 | 在兼容设备回放，检查 draw/dispatch、资源依赖，导出中间缓冲，研究同帧替换 | 文件较大，抓帧会干扰运行；不能据抓帧速度报告游戏 FPS |
| phonefarm frame-audit | `fa_*.json`，pass/draw 计数、附件、引用、拷贝与清屏等摘要；可选时间戳 | 筛选疑似无效 STORE/LOAD、重复清屏、可疑 pass 等优化方向 | 不是完整 API trace，不包含重建任意图像所需的所有资源，不能替代 RDC |
| PNG、录屏、普通 `phonefarm capture` | 屏幕像素与观察记录 | 确认场景、视角、交互和人工评价 | 没有 Vulkan 调用流；截图成功不代表抓帧成功 |

**需要“收集指令并能回放”时选择 RenderDoc；需要“先找值得优化的渲染操作”时选择 frame-audit。两条路线分别采，不同时向同一游戏叠加两个采集层。**

## 2 之前实际采到了什么

### 终末地 RenderDoc 数据

| 项目 | 已有结果 |
|---|---|
| 平台 | 已 root 的一加 Android，Adreno 750；当时游戏包为 `com.hypergryph.endfield`，版本 1.5.3 / code 50 |
| 原始捕获 | 三段移动轨迹，每段六帧，共 18 个 RDC；三段都在同一区域，不是三个独立地图 |
| 捕获编号 | scene01：10185～10190；scene02：18411～18416；scene03：6203～6208 |
| 同帧配对 | 18 对 LOW 1782×810 / HIGH 2376×1080；HIGH 在原设备 GPU 用同一捕获的几何重新栅格化，不是缩放 LOW 图片 |
| 原生输出 | 场景 HDR、depth、aux 和相关相机/常量缓冲；主通路的空间配对已完成，包含有效掩码 |
| 数据适用范围 | 带掩码的场景 HDR 空间先导数据；不是包含透明物、完整后处理和 UI 的最终画面真值，也不是已认证的完整动态光流标签 |
| 数据划分 | scene01/02 用于训练，scene03 整条轨迹留出；相邻帧、裁块和重复回放不能算作独立场景 |

原始 RDC 合计 **11,167,684,391 字节，约 11.17 GB**。2026-10-13 最后确认存于手机：

```text
/storage/emulated/0/Android/media/com.hypergryph.endfield/files/RenderDoc/
```

当时没有把这 18 个 RDC 下载到 Mac，也没有收入训练 ZIP。因此“训练包已保存/已迁移”不等于“原始 Vulkan 捕获已备份或已进入 phonefarm 后台”。再次使用前应由当前手机 owner 核对原件是否仍在。

可独立读取的先导训练包仍在本机，2026-10-30 仅核对文件存在及大小为 **451,537,667 字节**，未重新解压：

```text
phonefarm/tasks/endfield_temporal_sr_20261013/
  standalone-dataset-001/build-attempt-002/endfield-sr-pilot-v1.zip
```

包内含 18 对数据与加载工具，**不含原始 RDC**。原记录中的 SHA256 为：

```text
9ba1472ebedaa1743c8cf905434ba431696ca292231cb2bf645900edf204e3cd
```

该摘要沿用已有存储索引，本次没有重复计算。

### 轻量 Vulkan 摘要

已有原神 frame-audit 原件。一个历史样本记录每帧 33 个 pass，并包含 draw、dispatch、附件和时间戳可用性信息。它证明这条摘要采集路径实际运行过；不表示其它游戏、Vulkan 扩展或当前部署均已验证。该样本有一帧 `passes_timed=0`，所以分析时不能把打印出的 `0.0 ms` 当作真实零耗时。

依据：文末资料索引中的最终结果、18 对方法附录、存储索引，以及 `knobs/evidence/10_frameaudit/` 原件。

## 3 采集前准备

首次采集只需要把以下信息明确下来，不必重做整套环境审计。

| 准备项 | 要记录或确认的内容 |
|---|---|
| 目标 | 游戏包名、版本、场景、画质设置、窗口/渲染分辨率；区分 Vulkan 和 GLES 路径 |
| 手机 | 节点 ID、设备绑定、当前连接 serial、系统/GPU/驱动、root 或应用调试能力、当前 boot |
| 工具 | 实际 phonefarm/harness 版本，RenderDoc APK/layer/SDK 的版本与构建标识；历史成功组合为 RenderDoc 1.46 |
| 所有权 | 采集任务持有效 phonefarm lease，手机只由一个 owner 控制；分析 Agent 可以并行处理已落盘文件 |
| 场景准备 | 有可进入的场景或存档；先确认确实移动/渲染，避免菜单、加载页、黑屏或角色被障碍挡住 |
| 输出 | 新的采集批次目录、期望帧数、超时、文件总量预算、保存位置及恢复范围 |

节点入口可先进行只读查询。URL、node ID 和凭据文件由运行环境提供，不把凭据写进采集清单：

```sh
pf-node node
pf-node devices
pf-node queue
pf-node leases
```

`devices` 是登记信息，不等于设备已在线；必须结合实际设备观察。进入采集后，手机操作统一沿持有 lease 的任务执行。

**平台边界：**普通 RenderDoc Android 工作流要求应用可调试。之前终末地是非 debuggable 商业包，使用的是已授权 root 设备上的专用 layer 生命周期适配。不能据此认为任意非 root 手机都可抓取；nova16 鸿蒙无 root 的条件也没有被该实验验证。

## 4 RenderDoc 单帧采集流程

### 4.1 有可调试构建时

1. 在 RenderDoc 中选择 Android Remote Context，确认 adb 可访问目标设备。
2. 安装/启动匹配版本的设备侧 RenderDoc 组件，选择目标包并启动。
3. 进入固定场景，确认目标使用 Vulkan，再触发一帧捕获。
4. 等待实际 NewCapture 记录，保存 RDC 的真实路径、frame number 和文件大小。
5. 先在兼容设备打开并回放该帧，确认场景、关键 pass 和原生纹理输出，再扩采。

这部分遵循项目留存的 RenderDoc Android 上游说明。不要通过修改 APK 后仍把它记录成未修改的原版构建。

### 4.2 之前 root 终末地的实际路径

历史实现分为“手机侧 layer 生命周期”和“SDK 控制端”。步骤可以复用，旧脚本中的设备和安装路径不能直接复用。

1. **保存原状态。** 保存本次涉及的 Vulkan/RenderDoc 属性、GPU debug settings、目标安装路径、已有 layer 文件和端口映射。记录本轮创建的文件。
2. **停止目标并放置 layer。** 当时将匹配 ABI 的 RenderDoc layer 放入目标应用实际 native library 目录，沿原安装目录的权限和 SELinux 上下文处理。应用升级后该目录会变化。
3. **短暂启用并启动。** 当时的启用值为 `debug.vulkan.layers=VK_LAYER_RENDERDOC_Capture`、`debug.rdoc.IGNORE_LAYERS=0`。这两个值只是历史成功配置，不能在不检查其它 owner/layer 的情况下直接全局覆盖。
4. **确认连接的是本次游戏进程。** 检查新 PID 与启动时间、进程确实加载了目标 layer，并找到属于该进程的 `renderdoc_<实例号>` socket。不能把 socket 后缀假设成 PID。
5. **建立受管转发。** adb forward 连接该 socket；历史 Linux 控制端经受管 SSH 转发接入。保存本任务的映射，结束时仅移除这些映射。
6. **核 API 并触发。** SDK TargetControl 确认 PID、目标 API 为 Vulkan、连接可用，再执行一次 `TriggerCapture(N)`；不要靠反复点击来补齐一个结果未知的请求。
7. **接收并封存。** 为每个 NewCapture 保存 capture ID、frame number、API、时间、路径和 byte size。文件写完后记录实际 size/SHA256，再做 SDK 打开检查。
8. **结束与恢复。** 释放输入、关闭控制连接、停止本任务启动的目标、恢复原属性和本任务安装范围，保存收尾结果。采集完成与恢复完成分别记录。

当时连接建立后便恢复了短暂全局属性，最终仍执行完整清理。恢复应回到本次记录的原值，不能把旧实验“原值为空”推广到新手机。

已有代码入口位于研究工作区：

```text
phonefarm/tasks/oneplus_live_20261012/livephase/
  endfield-renderdoc-layer-c5de-002/capture-runner.py

phonefarm/tasks/endfield_temporal_sr_20261013/
  capture-pilot-001/driver.v7.py
  motion_and_replacement/continuous-control-client-002/
    continuous_control.v3.cpp
    continuous-control-supervisor.v3.py
```

这些是**绑定旧设备、boot、包和私有目录的实验实现**，不是可以换一个包名立即执行的通用 CLI。接续采集时只复用其流程，在当前 owner 下填写新目标；不要直接运行历史 `manifest.armed*.json`。

SDK 控制端的既有调用形式如下，仅在当前连接和 manifest 已准备好后，由受管 runner 调用：

```sh
continuous_control_v3.linux-x86_64 connect-probe \
  --manifest /owned/current-target.json

continuous_control_v3.linux-x86_64 trigger-metadata \
  --manifest /owned/current-target.json --capture-count 6 --execute
```

该历史客户端限定终末地、localhost 连接和最多六帧。`current-target.json` 的字段为 `schema`、`host`、`package`、`port`、`expected_pid`、`capture_count`；schema 为 `hgamey/endfield_capture_control_manifest@1`。端口和 PID 必须来自本次连接，外层 supervisor/lease 仍不可省略。

## 5 从单帧扩展到短序列

先完成一个单帧的打开、回放和资源导出，再采一组六帧。建议依次覆盖静止细节、相机旋转、角色平移、遮挡变化，最后扩展地图、材质和光照。

- 一次触发 `N` 帧，记录实际收到的每个捕获，不用旧帧填补缺失。
- 保存输入轨迹、采集顺序、触发/接收时间以及场景前后截图。
- 移动输入被接收不代表角色或相机真的移动。之前曾遇到角色被围栏挡住，需换轨迹后才得到有效移动。
- 连续 frame number 只说明捕获编号连续。抓帧会阻塞游戏，秒级时间戳也不足以证明稳定的 30/60 FPS 时间间隔。
- 场景切换、丢帧、失败和部分捕获保留在原批次中，训练资格另行标注。

时序训练还需要确认 previous/current 相机状态、jitter、历史缓冲、曝光、运动矢量方向/单位和遮挡有效区。仅有连续 RDC 或一张“像光流”的纹理，不足以提供这些标签。

## 6 phonefarm 轻量 frame-audit 采集

使用这条路线时，采集器记录的是**已有 hook 覆盖到的渲染步骤与资源引用**。目标是发现优化候选，后续仍需验证候选的实际性能和画质。

下面是通过既有 Node CLI 提交一次有限采集的示例。假定已在环境中配置 `PF_NODE_URL`、`PF_NODE_ID` 和 `PF_NODE_TOKEN_FILE`，并准备好匹配目标平台的 layer；占位符需替换。

```sh
pf-node --request-id vk-audit-scene01-001 --detach \
  phonefarm frame-audit capture \
  --serial SERIAL --pkg com.example.game \
  --captures 1 --gap 20 --settle 15 --out @out --json
```

命令会启动目标并按入口的场景准备策略执行；`settle` 只是等待，不保证任意游戏已经进入目标场景。先采用已完成游戏准备的可复现场景，确有需要时使用该入口已有的 `--tap`/`--extras` 参数。一轮 capture 可能包含多个记录帧，数量以输出 JSON 的 `frames` 为准。

需要 GPU pass 时间戳时，在单独批次添加 `--ts`。保留是否启用时间戳的标记，不把两种采集条件混在同一性能基线。

提交后使用返回的 job ID：

```sh
pf-node status JOB_ID
pf-node log JOB_ID
pf-node artifacts JOB_ID
pf-node out JOB_ID
```

请求返回不确定时，用 `pf-node request vk-audit-scene01-001` 找回原任务，不换 ID 重投。这个有限入口使用自身会话与 lease；不要把其它命令支持的 `--device-binding` 参数直接套上来。

结果在实际 session 子目录中，主要包括采集 JSON、截图、会话和恢复记录。对保存的 JSON 进行离线分析：

```sh
phonefarm frame-audit analyze /path/to/fa_1.json \
  --md /path/to/analysis.md --json /path/to/analysis.json
```

解释结果时看三点：

1. **覆盖范围：**记录 `diag` 中的 `cb_unseen`、`ev_overflow`、`ts_skipped` 等；存在漏记或溢出时限定结论，不当成完整帧。
2. **时间资格：**核对 `ts_enabled`、`ts_available` 与每帧 `passes_timed`。缺时间戳就是未知，不填零；各 pass 时间相加也不自动等于端到端帧时间。
3. **模型与实测：**LOAD/STORE 字节是基于尺寸、格式和采样数的带宽模型，未包含实际压缩等因素；描述符绑定也不等于 Shader 一定采样。疑似无效操作需要后续改动实验确认。

已有长期灰层 session 也支持 `frameaudit`/`frameaudit-ts` 模式及受管动作，详见 [灰层抓帧会话](GRAY-CAPTURE-SESSION.md) 和当前 `pf-node` 帮助。不要绕过会话直接改全局属性触发采集。

## 7 需要高低清训练对时再做的工作

如果只交付 Vulkan 指令捕获，到“RDC 可打开并回放”即可结束。生成训练数据是下一阶段，不应把它设成每次抓帧的前置条件。

同帧 LOW/HIGH 的关键步骤是：

1. 从**当前 RDC**发现目标 scene pass、attachments、viewport/scissor、Shader 和实际绑定常量，不沿用旧资源 ID、event ID 或内存偏移。
2. 导出母捕获的原生 scene color、depth 和 aux，保存格式、尺寸、event、mip/slice/sample 与原始字节。
3. 修改真实渲染尺寸及相关 framebuffer、内存分配和有证据支持的分辨率常量；处理实际 InitialContents 与后续 coherent/flush 写入。
4. 做同尺寸 LOW 控制回放，确认所选初始化策略没有改变基准输出，再在原 GPU 导出 HIGH。所有派生 RDC 另存，母捕获保持原样。
5. 固定像素中心映射与方向，用几何/深度构建有效掩码，检查同状态配对；不靠平移搜索、曝光拟合或挑裁块掩盖错配。

终末地已验证的格式细节只适用于对应捕获：HDR 为 R11G11B10；depth SDK 返回 `5N` 字节，前 `4N` 为打包 D24S8，尾部 `N` 字节语义未认证，应保留。固定预览变换 `(HDR/(1+HDR))**(1/2.2)` 不是游戏 tone mapping。

aux、相机和曝光语义未证明时按原名保存，不能直接改名为“光流真值”“无抖动相机”或“线性 HDR 标签”。600→1080 等新分辨率训练也应使用真实低清渲染与同状态高清参考，不能把已有 810p 图片 resize 后当作真实 600p 输入。

## 8 每批交付内容

下面是建议的交付布局，不要求迁移或改写历史目录，也不是 Node 自动扫描协议：

```text
collection/<batch-id>/
  manifest.json                 # 身份、版本、场景、文件索引、各阶段状态
  capture/                      # RDC 或 fa JSON 原件；远置时保留明确引用
  metadata/                     # SDK inventory、actions、资源/Shader/绑定信息
  preview/                      # 场景前后截图、回放预览
  raw/                          # 可选：原生颜色、depth、aux、常量缓冲
  derived/                      # 可选：派生RDC、高低清数据、mask；不覆盖原件
  logs/                         # 触发/接收、失败、退出与恢复记录
```

清单至少包含以下信息；未知字段保留 `null` 或明确状态，不填猜测值：

| 类别 | 必需信息 |
|---|---|
| 来源 | batch/scene/sequence ID，node ID、job ID、request ID，游戏包与版本，设备/boot/进程身份 |
| 环境 | GPU/驱动、操作系统、实际图形 API、采集工具版本、画质/分辨率与场景说明 |
| 触发 | 期望/实际帧数，触发次数、顺序、时间，输入轨迹（有则提供） |
| 原件 | 每个文件的路径或可取回 URI、bytes、SHA256；RDC 的 capture ID、frame number 和 API |
| 回放与导出 | SDK 打开结果、实际回放设备、event/resource、格式/尺寸/mip/slice/sample，输出文件索引 |
| 状态 | 捕获完成、容器可读、原生回放完成、配对通过、时序标签资格、恢复完成分别记录 |

原件生成完和跨机传输时确认 size/hash，复用已有索引；无需为每个展示层重新计算全量摘要。公开共享操作手册和脱敏预览；原始 RDC、进程原始转储与账户相关内容按项目数据权限保存。

**phonefarm 中的可发现性需要显式登记。** 文件在手机、Mac 或某个 job 输出目录里，不会自动成为 results 记录。新采集把清单和报告写入 job `@out`，通过现有结果登记关联研究/方向与原件引用；大 RDC 可留在设备宿主的大容量目录，记录取回方式。不要把数百 MB 的 RDC 塞进只面向小工件的 `--inputs` 清单，旧原件也不因登记而重复上传。

## 9 最小验收与常见问题

一轮采集只做与交付目标相关的检查：

- **指令摘要交付：**JSON 可读、目标与场景明确、实际有帧和步骤、覆盖诊断已记录。
- **RDC 交付：**实际文件完整、SDK 可打开并报告 Vulkan/正确帧，至少完成代表帧的原设备回放；逐帧是否回放在清单中逐项记录。
- **图像交付：**指定 event 的原生尺寸/格式/字节数正确，预览能确认目标场景；截图不替代原生纹理数据。
- **配对/时序交付：**分别执行对应检查，不因“RDC 已生成”自动标为训练合格。
- **收尾：**输入已释放、本任务的 layer/属性/映射已按范围恢复；未确认的恢复保留阻断，不清旧 lease 掩盖问题。

| 现象 | 处理方式 |
|---|---|
| 手机可见，但抓不到游戏 | 核实际 Vulkan 路径、目标进程和 layer 加载；普通 debuggable 路线与 root 专用路线不能混用 |
| 有截图，没有 RDC | 还没有完成 API 捕获；检查 TargetControl 的实际 NewCapture 记录 |
| 连续帧里没有运动 | 核相机/角色实际变化，排除障碍、菜单和输入映射问题 |
| CPU 能读 RDC，GPU 回放失败 | 容器可读与 GPU 兼容是两阶段；优先原设备 native replay，保留失败，不靠换桌面 GPU 宣称成功 |
| native replay 内存不足 | 依据实际地址空间/RSS调整预算。历史一帧成功回放 VmSize 约6.38 GB、采样RSS约1.24 GB；4 GiB虚拟地址上限不足，不能把它误判成需要同等物理内存 |
| HIGH 仍读到旧分辨率常量 | 检查后续 coherent/flush 写入是否覆盖 InitialContents；以实际绑定缓冲的 GPU 回读为准 |
| frame-audit 时间显示为0 | 先核时间戳是否可用；缺测值不当零耗时 |
| 收尾失败或状态未知 | 保存原 job/session 与收据，沿原上下文恢复；不重放整段采集、不直接清锁 |

容量按实际样本估算：历史 18 个 RDC 平均约 620 MB/帧，一组六帧约 3.7 GB，派生捕获和回放暂存还会增加空间。先单帧、后短序列；优先在手机或大容量 Linux 宿主处理，仅回传必要缓冲和索引，避免 Mac 重复全量下载/解压。

采集运行与性能测量分开。RenderDoc 抓取、纹理读回、文件写入与 Shader 替换回放的墙钟耗时，都不能直接作为游戏帧时间、实时 FPS、功耗或每帧能耗收益。

## 10 资料与现有实现入口

研究文件路径以 `phonefarm/` 为根。Mac 研究树在 `/Users/kamico/work/GOSIM/HGamey/phonefarm`；2026-10-26 同步的 rhodes1 副本在 `/data2/kamico/HGamey-dev-20261025/HGamey/phonefarm`。本次没有重新检查远端副本或手机原件。

| 内容 | 文件 |
|---|---|
| 18帧原件身份与逐文件状态 | `tasks/endfield_temporal_sr_20261013/source-files-003/source-manifest.json` |
| 最终结果与适用范围 | `tasks/endfield_temporal_sr_20261013/dataset-bundle-001/FINAL-RESULTS.md` |
| ZIP与原始RDC的存储区分 | `tasks/endfield_temporal_sr_20261013/standalone-dataset-001/STORAGE-INDEX-20261014.json` |
| 同帧高低清方法 | `tasks/endfield_temporal_sr_20261013/methodology/METHOD.md`；`methodology/appendices/ACTUAL_18_NATIVE_PAIR_AND_PIXEL_MASKED_PILOT.md` |
| Android原设备回放与内存经验 | `tasks/endfield_temporal_sr_20261013/methodology/appendices/ANDROID_LOCAL_ROUTE.md` |
| 虚拟手柄注册与历史故障 | `tasks/endfield_temporal_sr_20261013/methodology/appendices/HID_BOOTSTRAP.md` |
| RenderDoc Android上游说明的本地存档 | `tasks/media_algorithm_research_20261012/blackbox-capture-audit-001/source/docs/how/how_android_capture.rst` |
| frame-audit实际原件 | `tasks/developer_transition_20261022/phonefarm-main/knobs/evidence/10_frameaudit/{capture_meta.json,fa_1.json,analysis.md}` |

公开代码入口：[frame-audit 实现](https://github.com/HGamey/phonefarm/blob/7aa39c5a5ebc401430c828b0c8d1d08d57e77e85/src/frameaudit.rs)、[pf-node CLI](https://github.com/HGamey/harness-ops/blob/be8279c270739c7280afbe8e4b6af267c9bbfeb5/node/pf-node)。平台相关合同：[灰层抓帧会话](GRAY-CAPTURE-SESSION.md)、[结果索引](RESULT-INDEX.md)。

本手册新增的是可复用操作说明；平台开发 TODO、旧研究负结果和原始恢复义务保持原记录，长期自动开发仍按此前要求暂停。
