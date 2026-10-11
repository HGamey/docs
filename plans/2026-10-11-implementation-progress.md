# 本轮实现与验收进度

更新：2026-10-11。需求与五仓主线审计见 [系统交接计划](2026-10-11-system-handoff.md)，跨仓验收见 [系统 epic #11](https://github.com/HGamey/.github/issues/11)。所有下列 PR 当前均为 draft、未合并；本文件不将离线夹具或历史红魔结果作为一加真机验收。

## 可审阅的代码交付

| 系统位置 | PR | 已交付的机制 | 尚未验收的范围 |
|---|---|---|---|
| 交接与状态 | [docs #5](https://github.com/HGamey/docs/pull/5)、[game_opt_loop #26](https://github.com/HGamey/game_opt_loop/pull/26) | 会议需求、五仓职责、历史证据边界、验收顺序 | 整个系统 epic 仍开放 |
| 评测证据与档案 | [game_opt_loop #27](https://github.com/HGamey/game_opt_loop/pull/27) | 两种评测入口保留完整 receiver 扩展；离线资格回放不能进入前沿；旧报告明确未资格化 | 旧报告可读不等于重新获得真实游戏资格 |
| 方向与专家决策 | [game_opt_loop #28](https://github.com/HGamey/game_opt_loop/pull/28) | shader/optimize 共享 campaign；冻结请求、工件、条件、决定和负结果；pending/reject 不评测，恢复不重复下发 | 模型阶段与动态灰层仍需实际连接到候选调度 |
| 实验资格 | [phonefarm #18](https://github.com/HGamey/phonefarm/pull/18) | 逐目标必需指标、代理/显示语义、root/冷机/稳定基线/放电窗口/还原门禁，入口实际消费 | 真机资格复核；连续画质仍需真实捕获与消费端桥接 |
| 连续画质 | [phonefarm #17](https://github.com/HGamey/phonefarm/pull/17) | 配对 PNG、来源与 hash 账本，空间/色偏/相邻残差/坏帧检查，错配或解码异常不获资格 | hash 一致性不证明真实捕获；尚无光流、遮挡或完整鬼影检测；报告始终不授予真实游戏来源资格 |
| 条件与设备独占 | [phonefarm #19](https://github.com/HGamey/phonefarm/pull/19) | shader v2 条件解析、请求/实际分离、载体参数回读；本机设备写入口自动租约与异常退出恢复门禁；真实 GOL fan/Inject 兼容；legacy/v2 都拒绝质量采集污染性能样本 | 无端到端放电窗口或真实游戏收益资格；不支持的注入/控制明确 ABORT |
| 独立确认 | [game_opt_loop #29](https://github.com/HGamey/game_opt_loop/pull/29) | 事先注册轮数/顺序/收益及非劣效界与基线准入 hash；只消费版本化有效单轮观测；原始工件去重、同设备/boot 的完整 ENV 和 capture 窗不重叠、单侧置信上界裁决 | 仍缺真机单轮采集 driver；设备身份、采集来源和注册时间尚无认证；不改前沿或授予游戏优化证明 |
| 单轮原始观测验收 | [phonefarm #22](https://github.com/HGamey/phonefarm/pull/22)（依赖 #18） | 从原始记录重算七项门禁、至少三轮稳定基线、冻结绑定、控制与恢复完整快照和整窗放电；542 项源码测试，实际跨仓 CLI 冻结→10 观测→judge 跑通 | 离线入口没有采集设备；当前仅支持 Android root/sysparam/KGSL 提交代理/电池；旧目录缺 cold/root/boot/退出/恢复 receipts 不能补成有效观测 |
| 模型工件阶段 | [game_opt_loop #30](https://github.com/HGamey/game_opt_loop/pull/30) | 五阶段 driver 请求与角色/格式/工件冻结、逐阶段专家审批、预算声明门禁、原子恢复与显式 reconcile | 345 项测试使用假驱动；没有真实训练、量化、Android 推理或 phonefarm 资格接入；完成协议不授予模型语义或推广资格 |
| 动态 SR 灰层工件 | [phonefarm #20](https://github.com/HGamey/phonefarm/pull/20) | sampled-image ABI 反射、冻结候选/构建/层 hash、有界构建子进程、实际 native 记录/提交计数 | 缺 Android NDK/glslc；共享 descriptor、旧 clone 提前释放、resize 销毁仍被录制引用的图像，需一起修复资源所有权；全部 manifest 的 deployable=false |
| 有限 SPIR-V 适配与工件消费 | [phonefarm #21](https://github.com/HGamey/phonefarm/pull/21)（依赖 #20） | storage input 转 sampled input，保留 storage output；`build --adapted-artifact` 使用已校验快照；530 项离线测试及 6 个实际 campaign 工件适配 | source/SPIR-V 关联仍未核验；缺 GOL driver/安全 gate 认证、spirv-val、GPU 像素等价和实际 Android 构建；不能授予部署资格 |
| 原生白盒配对 | [refbench #2](https://github.com/HGamey/refbench/pull/2) | 独立 native LR/HR 与候选输出分离、固定同帧参数、raw 导出及严格 raw→PNG 质量消费桥；质量模式不能用于性能 | 11 项 host 测试；尚无 APK/GPU 采集；native 来源暂映射 unknown，指标可 PASS 而总体 UNKNOWN；不是实际游戏、没有 FrameGen 中间真值 |

单轮观测重建和消费端已通过实际 CLI 合同验证；夹具由假设性 driver 声明和转换过的历史 trace 片段构成，没有设备采集认证。真实训练器、量化器和 Android runtime driver 不属于本轮假驱动验证已经完成的内容。

## 集成验证如何解释

本轮各功能从同一远端基线建立独立分支，再 cherry-pick 到本地集成分支，检查功能同时存在时的实际 CLI 行为。测试数是对应提交的回归总数，不能相加成更多独立实验证据，也不能换算为真实游戏效果。

- game_opt_loop 集成分支 `codex/handoff-game-integration` 在 `9782a76` 上共 **376 项测试通过**，严格 clippy 通过。包含 #26–#30、实际 campaign/证据兼容测试：资格不足不进入前沿、原始扩展与专家决定不丢失、恢复零重复评测。单轮独立确认仍由夹具供数，模型阶段由假驱动执行。发现并修复了两项将 Python 启动速度误当成生产预算行为的时序测试；生产超时规则未放宽。
- phonefarm 集成分支 `codex/handoff-phonefarm-integration` 在 `7c03487` 上共 **602 项测试通过**，包含 #17–#22；clippy exit 0，仍有历史告警。既有 6 个保存的实际 CLI 报告由真实 GOL parser 消费；新增实际 freeze→10 个单轮观测→judge 的夹具判定 PASS，hash 一致的恢复门禁篡改及 synthetic 来源 BLOCKED，证明/晋级标记全程 false。ADB/HDC 调用 0；native counter host 测试仍不验证 GPU 生命周期。
- 测试日志与原始 JSON 保存在本机 `phonefarm/tasks/system_handoff_20261011/` 及各 worktree 的 `tasks/`。集成分支目前仅本地存在；这些记录不代表 GitHub CI 已通过。
- 本轮无真机部署、真实训练或模型 API 调用；**0 模型 API 调用**。内部代码开发与评审不作为被测模型的实验成本。

## 剩余系统接口与完成条件

| 接口 | 当前缺口 | 验收所需实际证据 |
|---|---|---|
| game_opt_loop → phonefarm shader v2 | 已核对真实序列化的 fan/Inject；支持范围限于现有白盒载体，完整条件/采样仍待真机 | 同一请求被明确接受或明确拒绝；requested 字段保留，observed 未知不虚构；未知条件不能授予资格 |
| storage-image 算子 → 灰层 sampled-image ABI | 有限转换与工件构建消费已接通；仍缺 GOL 调度、源代码安全认证及 GPU 等价 | 确认支持矩阵、上游编译/gate 与 source/SPIR-V 绑定；Vulkan 验证、算法等价、候选/层 hash 与命中身份贯通 |
| 动态层 → 真实游戏试验 | 一加挂点、参数、图像用途及命中尚未验证；[资源生命周期审查](https://github.com/HGamey/.github/issues/14#issuecomment-6109047301) 已冻结完整修复与验收范围 | 每次 dispatch 不可改写的完整资源组、按录制代合法退役、copy 等价基线、目标/非目标 pass 约束、实际命中；已录制的注入不能通过关闭开关撤回 |
| refbench → 配对连续画面 | 原生 producer 和 raw→PNG 桥已发布；尚无 Android/GPU 实采及来源核验 | 真机构建和相同场景/相机/时间的独立渲染与读取，帧 ID/格式/色彩/提交完成与捕获账本，不把候选输出作为参考；单独进行性能运行 |
| 序列质量 → 资格/候选裁决 | CLI 的合成 PASS 不能自动授予真实来源资格 | 捕获来源核验、候选工件绑定、冻结阈值、坏帧证据；缺失或未知阻断晋级 |
| 模型工件 → 实际训练/量化/runtime | 五阶段协议完成不证明权重、模型格式或部署语义有效 | 可信数据/格式检查、真实 driver、预算计量、租约与恢复、实际 Android 推理和性能/质量报告 |
| 探索 → 独立确认 | 离线 raw producer 与 judge 已连接；聚合 eval 报告不能替代单轮记录，仍缺持租约的真机采样 driver | 原始 cold/root/boot/完整快照/采样与恢复退出 receipts，预冻结顺序、探索隔离、每臂唯一工件和不重叠整窗、完整轮数与实际来源 |

上述接口的验收证据齐备后，才安排冻结预算的整链路候选与国内模型比较。接回一加时先执行 [只读资格和基线](oneplus-acceptance.md)，不从 root 成功直接跳到收益声明。
