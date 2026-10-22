# 项目地图与可复用资产

审计日期：2026-10-22。以下是源码与已有记录审计；本轮没有手机操作或新的真机验收。固定提交链接用于区分上游能力、本地分叉与未来提案。

## 仓库职责

| 仓库 | 审计版本 | 当前职责 | 本轮建议 |
|---|---|---|---|
| [phonefarm](https://github.com/HGamey/phonefarm/tree/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff) | `0cd6571` | 设备后端、操作、knobs、测量、统计与恢复 | 保留执行底座；补完整能力查询、操作回执和统一 ownership |
| [harness-ops](https://github.com/HGamey/harness-ops/tree/07b0aa7beee4f256834b752fa00273a5e6868560) | `07b0aa7` | 节点服务、pf-node/pf-fleet、部署、网页 | 第一批远程产品主干：节点、分发、队列、索引、异步协作 |
| [game_opt_loop](https://github.com/HGamey/game_opt_loop/tree/d090d27ceb62c8dc15331e5121d1cf0ffd048ad0) | `d090d27` | 优化策略、候选生成/演化、goal、campaign、模型、确认与报告 | 保留算法和方法；作为可选客户端，与外部 Agent 使用相同执行合同 |
| [refbench](https://github.com/HGamey/refbench/tree/b53c7856825b2a0de9358677d2836ecd04ad73e6) | `b53c785` | 确定性 Vulkan 负载、开关、已知答案 | 保留标尺和集成靶场；其收益与真实游戏收益分开 |
| [docs](https://github.com/HGamey/docs/tree/a28b0a1e671d11c4a06bc2742c44f77e143192fc) | `a28b0a1` | 长期设计、规格、计划、报告 | 当前计划、能力账本与长期合同的入口 |
| [internal](https://github.com/HGamey/internal/tree/497695f71c35d8fc471008f1fd4e555dc754ad57) | `497695f` | 内部会议、运维、实验资料 | 保持私有边界；与产品产物协议分开 |
| [knobs](https://github.com/HGamey/knobs/tree/3f812a93d4892a8066eafcac818b49ab2d56ffa7) | `3f812a9` | 已归档，功能并入 phonefarm/knobs | 不再单独恢复开发线 |
| [sr_loop](https://github.com/HGamey/sr_loop/tree/b3b90dbbad7a9d991cfbc49fe8d80278658f9238) | `b3b90db` | README 明确为结项前序；已有模型/演化经验 | 按需抽取算法和负结果，不恢复整套重复基础设施 |
| [.github](https://github.com/HGamey/.github/tree/4ad34aee426cf3bc47f76e0dcbe8391db97388ff) | `4ad34ae` | 组织说明和入口 | 更新为外部 Agent 优先定位，链接统一文档 |

这份职责划分不要求立即合仓或更换语言。先统一安装、调用和数据合同，再根据实际维护成本决定是否合并 harness-ops 与 phonefarm。

## 已经有的能力

- 节点已有异步 `/jobs`、增量日志、文件级输出、SHA/etag、Range/续传、进程组及退出码恢复。见 [node_service.py](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/node_service.py#L8)。不能把任务系统描述成完全不存在。
- fleet 已有 registry、指定节点/标签/空闲节点选择与 goal 迁移。它是客户端选择，不是持久调度中心。见 [pf-fleet](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/pf-fleet#L203)。
- phonefarm 已有 adb、hdc、iOS/WDA 后端及能力声明。iOS 源码明确尚未真机验收；HDC 不具备 Android root 注入/HID 的等价能力。见 [platform.rs](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/platform.rs#L118)、[ios.rs](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/ios.rs#L602)。
- Android 已有 HID 手柄、摇杆和按钮保持；点击/滑动/文本也已实现。缺的是可靠的通用指针生命周期、多点触控、统一远程会话和操作回执。见 [gamepad.rs](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/gamepad.rs#L90)。
- 已有强 lease、token/心跳、设备 flock 与 recovery 屏障；不应另造不兼容的锁或仅凭超时抢锁。见 [devlease.rs](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/devlease.rs#L181)。
- 上游已有 `phonefarm screen` 截图观战与动作叠加。它尚未成为 node 统一认证下的人机接管会话。见 [screen.rs](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/screen.rs#L15)。
- GOL 已有候选级 Pending/Deferred/Ready/InFlight/Completed、运行中读取人类指导、分层失败后继续，以及外部 proposer 协议。应把可复用状态与方法抽成工具，不把每个外部 Agent 限定为某一代候选 proposer。见 [campaign.rs](https://github.com/HGamey/game_opt_loop/blob/d090d27ceb62c8dc15331e5121d1cf0ffd048ad0/src/campaign.rs#L63)、[goal/mod.rs](https://github.com/HGamey/game_opt_loop/blob/d090d27ceb62c8dc15331e5121d1cf0ffd048ad0/src/goal/mod.rs#L689)、[现有集成协议](../../repos/game_opt_loop/docs/INTEGRATION.md)。
- 已有云端构建与 SHA 资产发布。不要再造一套下载/编译部署通路。见 [node-build.yml](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/.github/workflows/node-build.yml#L1)。

## 使用中暴露的确定缺口

| 现象 | 代码/实际证据 | 影响 |
|---|---|---|
| 看起来“接入了平台”，别人仍找不到数据 | 首页绑定 goal；实验解析只认 optimize/autoloop/eval，扫描近200 jobs。[dash.py:295](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/dash.py#L295)、[703](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/dash.py#L703)；Doom自定义作业实际遗漏 | 合法自定义实验也应注册和被索引，不再按命令名猜测结果 |
| 只读凭据拿不到原件 | 只读 token 只允许 MCP；GET jobs 仍走 admin。[node_service.py:370](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/node_service.py#L370) | 结果发现、日志和产物应有一致的只读授权 |
| 状态查询自己起任务 | fleet status 经 POST jobs 查询 devlock/fleet。[pf-fleet:38](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/pf-fleet#L38) | 访客不能查询；状态污染任务列表，并产生额外调度负担 |
| 选中了空闲设备也可能马上冲突 | fleet 先探测再直接启动，无资源预留 | 需要队列与执行端 lease 原子取得，不能把探测视为占有 |
| 服务重启后可查询却不能取消 | status 有磁盘退出码恢复；kill 只查内存 procs。[node_service.py:218](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/node_service.py#L218)、[294](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/node_service.py#L294) | 持久进程身份、恢复和取消必须使用同一合同 |
| 作业只有 running/finished | 直接 Popen，少了准备、队列、采集、恢复、阻断状态。[node_service.py:169](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/node_service.py#L169) | 外部 Agent 无法判断该重试、做别的还是等待恢复 |
| 安装 APK 被误当成游戏就绪 | 上游 fetch_apk 固定默认 serial、basename缓存、可选hash，缺资源/存档/场景合同。[fetch_apk.sh:8](https://github.com/HGamey/harness-ops/blob/07b0aa7beee4f256834b752fa00273a5e6868560/node/fetch_apk.sh#L8) | 必须分开下载、安装、资源齐全、登录/场景可复现等阶段 |
| 大输入传输成本高 | 输入整包tar、512MiB限制、两端驻内存；路径仅改写显式argv | 需要流式/续传/缓存与声明式文件引用，避免任务自己写搬运脚本 |
| 暂停看屏仍在后台抓图 | widget暂停只停前端tick；screen paused初始化后没有修改路由。[screen_widget.html:224](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/screen_widget.html#L224)、[screen.rs:191](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/screen.rs#L191) | 无人观看/性能采样时应停止或明确计入观察开销。源码确认，本轮未真机复现 |
| 输入成功没有统一回执 | Backend基础输入返回 `()`，部分坐标硬编码、HDC键值未验证 | “命令发出”与“实际输入成功/画面变化”必须区分 |
| 入口域名被当成存储位置 | jobs在执行checkout内，域名可能只是转发 | node ID、执行host、数据根、入口URL、代码版本分别展示 |

## 本地成果不是单纯过期版本

本机研究分支已做 job 唯一锁名、raw adb/fetch_apk 包锁、fleet队列解析、SHA内容缓存、partial校验/隔离、显式serial等增强。它们没有全部出现在当前上游 main。上游同时增加了新后端、screen 等能力。

开发前要逐项对照并回收，不能直接覆盖本机、全量推历史分支或将所有功能再实现一次。原始收据在本机迁移任务的 ACCEPTANCE 与 harness-ops 测试中；共享规划不搬运这些实验数据。

## 维护边界

- 执行节点、优化策略、文件保存、结果索引和展示页面是不同职责；一个公开网页或 fleet registry 不能证明数据已经统一。
- GOL 的内置 goal、现有多个网页和外部 Agent 应消费同一个结果/任务合同；先改数据入口，再决定合并哪些页面。
- 当前 docs STATUS、迁移后的旧 STATUS 与各研究报告有不同日期/范围。建立带版本和验证等级的能力账本，保留旧报告历史，不把历史限制当当前代码事实。
- 构建成功、离线测试通过和真机验证要分开。当前 phonefarm 有构建工作流；harness-ops 上游缺专门的远程作业/分发/故障回归，首批应补齐。
