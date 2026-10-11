# 本轮实现与验收进度

更新：2026-10-12（交接与原始验收批次始于 2026-10-11）。需求与五仓主线审计见 [系统交接计划](2026-10-11-system-handoff.md)，跨仓验收见 [系统 epic #11](https://github.com/HGamey/.github/issues/11)。所有下列 PR 当前均为 draft、未合并；本文件不将离线夹具或历史红魔结果作为一加真机验收。

实际运行入口、结果含义与页面启动方式见 [Harness 运行说明](harness-runbook.md)。

## 可审阅的代码交付

| 系统位置 | PR | 已交付的机制 | 尚未验收的范围 |
|---|---|---|---|
| 交接与状态 | [docs #5](https://github.com/HGamey/docs/pull/5)、[game_opt_loop #26](https://github.com/HGamey/game_opt_loop/pull/26) | 会议需求、五仓职责、历史证据边界、验收顺序 | 整个系统 epic 仍开放 |
| 评测证据与档案 | [game_opt_loop #27](https://github.com/HGamey/game_opt_loop/pull/27) | 两种评测入口保留完整 receiver 扩展；离线资格回放不能进入前沿；旧报告明确未资格化 | 旧报告可读不等于重新获得真实游戏资格 |
| 方向与专家决策 | [game_opt_loop #28](https://github.com/HGamey/game_opt_loop/pull/28) | shader/optimize 共享 campaign；冻结请求、工件、条件、决定和负结果；pending/reject 不评测，恢复不重复下发 | 模型阶段与动态灰层仍需实际连接到候选调度 |
| 实验资格 | [phonefarm #18](https://github.com/HGamey/phonefarm/pull/18) | 逐目标必需指标、代理/显示语义、root/冷机/稳定基线/放电窗口/还原门禁，入口实际消费；私有 probe 修复并发 root 误判，530 项回补测试通过 | 真机资格复核；连续画质仍需真实捕获与消费端桥接 |
| 连续画质 | [phonefarm #17](https://github.com/HGamey/phonefarm/pull/17) | 配对 PNG、来源与 hash 账本，空间/色偏/相邻残差/坏帧检查，错配或解码异常不获资格 | hash 一致性不证明真实捕获；尚无光流、遮挡或完整鬼影检测；报告始终不授予真实游戏来源资格 |
| 条件与设备独占 | [phonefarm #19](https://github.com/HGamey/phonefarm/pull/19) | shader v2 条件解析、请求/实际分离、载体参数回读；本机设备写入口自动租约与异常退出恢复门禁；真实 GOL fan/Inject 兼容；legacy/v2 都拒绝质量采集污染性能样本 | 无端到端放电窗口或真实游戏收益资格；不支持的注入/控制明确 ABORT |
| 独立确认 | [game_opt_loop #29](https://github.com/HGamey/game_opt_loop/pull/29) | 事先注册轮数/顺序/收益及非劣效界与基线准入 hash；只消费版本化有效单轮观测；原始工件去重、同设备/boot 的完整 ENV 和 capture 窗不重叠、单侧置信上界裁决 | 采集 driver 见 PF #25；真机行为、设备身份、采集来源和注册时间尚未验收或认证；不改前沿或授予游戏优化证明 |
| 单轮原始观测验收 | [phonefarm #22](https://github.com/HGamey/phonefarm/pull/22)（依赖 #18） | 从原始记录重算七项门禁、至少三轮稳定基线、冻结绑定、控制与恢复完整快照和整窗放电；542 项源码测试，实际跨仓 CLI 冻结→10 观测→judge 跑通 | 离线入口没有采集设备；当前仅支持 Android root/sysparam/KGSL 提交代理/电池；旧目录缺 cold/root/boot/退出/恢复 receipts 不能补成有效观测 |
| 持租约的确认采样 | [phonefarm #25](https://github.com/HGamey/phonefarm/pull/25)（依赖 #22，另引入 #19 租约） | 实际 prepare/collect/reconcile 命令源、冻结触控负载、三基线、五组交替 A/B、控制/采样/完整恢复与永久关闭旧会话；568 项分支测试；实际 CLI 重建十份观测与 GOL 裁决 | fake transport 未执行 Android worker，观测与裁决均 synthetic/BLOCKED；真实 root/setsid/procfs、节点持续生效、KGSL/放电窗口、场景重复性与实际来源待验收；HDC/无 root 明确拒绝 |
| 模型工件阶段 | [game_opt_loop #30](https://github.com/HGamey/game_opt_loop/pull/30) | 五阶段 driver 请求与角色/格式/工件冻结、逐阶段专家审批、预算声明门禁、原子恢复与显式 reconcile | 345 项测试使用假驱动；没有真实训练、量化、Android 推理或 phonefarm 资格接入；完成协议不授予模型语义或推广资格 |
| 动态 SR 灰层工件 | [phonefarm #20](https://github.com/HGamey/phonefarm/pull/20) | sampled-image ABI 反射、冻结候选/构建/层 hash、有界构建子进程、实际 native 记录/提交计数 | 缺 Android NDK/glslc 与 GPU 验证；资源所有权修复见 #23；全部 manifest 的 deployable=false |
| 有限 SPIR-V 适配与工件消费 | [phonefarm #21](https://github.com/HGamey/phonefarm/pull/21)（依赖 #20） | storage input 转 sampled input，保留 storage output；`build --adapted-artifact` 使用已校验快照；530 项离线测试及 6 个实际 campaign 工件适配 | GOL #31 新增独立的本地编译绑定；PF 的原始关联仍标为 caller_supplied_unverified；自动准备 driver 见 GOL #34；仍缺 spirv-val、GPU 像素等价和实际 Android 构建 |
| 候选编译证据 | [game_opt_loop #31](https://github.com/HGamey/game_opt_loop/pull/31) | 实际 Naga 编译与安全 gate、源代码/SPIR-V/编译输入/可执行文件冻结；外部 expected identity 约束重放；340 项测试及实际跨仓四 CLI | 本地完整性与编译重放不等于可信来源、算法等价、GPU 验证或部署资格；自动准备 driver 见 #34；仍不授予部署资格 |
| 原始证据的只读展示 | [game_opt_loop #32](https://github.com/HGamey/game_opt_loop/pull/32)（依赖 #27） | 展示完整 receiver/原始 A/B/资格与来源；矛盾 summary PASS 或缺项不显示有效通过；339 项测试、严格 clippy 与实际浏览器检查 | 历史前沿只作档案展示；证明/晋级始终 false；统一案例入口见 #33；展示不授予证明或晋级资格 |
| native 资源所有权 | [phonefarm #23](https://github.com/HGamey/phonefarm/pull/23)（依赖 #20） | 按录制代与每次 dispatch 保留不可改写的完整资源；合法边界退役、失败输出句柄清理；518 项测试，每种 fixture 39 类实际 hook 场景，独立反例复核 | 仅 host/mock 下层验证；生产拒绝旧启发式注入；真实图像布局/目标/计算状态合同、Android/GPU 与真实游戏验证未完成 |
| 统一案例页面 | [game_opt_loop #33](https://github.com/HGamey/game_opt_loop/pull/33)（依赖 #32） | 固定根目录内有界只读扫描，独立确认/模型阶段及关联原始 JSON、exact bytes/SHA、未知/负例/恢复/成本同页；351 项测试、严格 clippy 和实际浏览器检查 | 保存的 PASS/completed 仅是原报告声明；页面不运行 driver，证明和晋级始终 false；关联缺失/越界/符号链接/预算不足明确显示 |
| 原生构建进程监督 | [phonefarm #24](https://github.com/HGamey/phonefarm/pull/24)（依赖 #21） | 显式 inherited-group 接口，所有 helper 继承独立外层进程组；超时/溢出先终止再日志；535 项测试与独立满 stderr 管道反例复核 | controlled fake NDK 验证进程机制；真实 NDK、实际 GOL native 构建和 GPU 均未执行；成功输出仍需外层证明进程组消失再核验，deployable=false |
| 自动候选准备 | [game_opt_loop #34](https://github.com/HGamey/game_opt_loop/pull/34)（依赖 #31） | 冻结两端私有工具和候选，实际 compile/verify/adapt/verify 四阶段、有界组监督、原子账本、只读 inspect；355 项测试、严格 clippy，独立工具/SDK竞争/后代/重哈希反例复核 | 只接受 PREPARED_OFFLINE；缺失 SDK 通过私有空 SDK 留下实际拒绝，完整链 false；present SDK native 构建、GPU、游戏 eval 和质量仍未验收 |
| 原生白盒配对 | [refbench #2](https://github.com/HGamey/refbench/pull/2) | 独立 native LR/HR 与候选输出分离、固定同帧参数、raw 导出及严格 raw→PNG 质量消费桥；质量模式不能用于性能 | 11 项 host 测试；尚无 APK/GPU 采集；native 来源暂映射 unknown，指标可 PASS 而总体 UNKNOWN；不是实际游戏、没有 FrameGen 中间真值 |

单轮观测重建和消费端已通过实际 CLI 合同验证。早期夹具由假设性 driver 声明和转换过的历史 trace 片段构成；#25 新夹具实际执行 host 调度与明确的 fake transport，十份 raw 观测和最终裁决始终 BLOCKED。两者均没有真实设备采集认证。真实训练器、量化器和 Android runtime driver 尚未运行。

## PR 依赖与审阅顺序

这些功能分支共享文件，审阅和后续整合需遵循实际依赖：

- game_opt_loop 的档案/页面链为 #27 → #32 → #33；候选准备链为 #31 → #34。#26、#28、#29、#30 分别以 main 为基线。
- phonefarm 的观测链为 #18 → #22 → #25；#25 在 #22 上另外引入 #19 的设备租约并扩展持久恢复状态，集成时保留 #19 完整入口与 #25 更新后的租约模块。
- phonefarm 的工件/构建链为 #20 → #21 → #24；资源所有权 #23 以 #20 为基线。#17 和 #19 分别以 main 为基线。
- 本地集成分支同时检查这些功能；各 PR 的独立测试数与集成测试数绑定各自提交。当前没有合并任何 PR。

## 集成验证如何解释

本轮各功能从远端基线或明确依赖的 draft 分支建立独立分支，再 cherry-pick 到本地集成分支，检查功能同时存在时的实际 CLI 行为。当前共 20 个 draft PR；测试数是对应提交的回归总数，不能相加成更多独立实验证据，也不能换算为真实游戏效果。

- game_opt_loop 集成分支 `codex/handoff-game-integration` 在 `5149a33` 上共 **415 项测试通过**，严格 clippy 通过。包含 #26–#34、实际候选准备及 campaign/证据兼容测试：资格不足不进入前沿、原始扩展与专家决定不丢失、恢复零重复评测。单轮独立确认仍由夹具供数，模型阶段由假驱动执行。发现并修复了两项将 Python 启动速度误当成生产预算行为的时序测试；生产超时规则未放宽。
- phonefarm 集成分支 `codex/handoff-phonefarm-integration` 在 `cd71c4e` 上共 **634 项测试通过**，包含 #17–#25 源码；default clippy exit 0，有 67 个告警标题，未宣称 strict clean。测试调用固定 `5149a33` 的 GOL 二进制，完整 driver 夹具保留 synthetic/BLOCKED；ADB/HDC 真机调用 0。之后 `2d2ee03` 仅加入 #25 的证据文档和复现脚本。此前 `8e819ff` 的 602 项、6 个保存的实际 CLI 报告及历史片段 freeze→10 观测→judge PASS 保留为对应旧提交的离线证据，不能用于证明真实采集或游戏收益。native host/hook 验证仍不证明 GPU 执行或完成。
- `75dd2bb` / `8e819ff` 对应的两个集成可执行文件实际执行了 compile→verify→adapt-storage→verify-adaptation，四条 CLI 均 exit 0。原始 source/SPIR-V 的 bytes/hash 一致，编译身份和执行前后工具 hash 冻结；结果仍 `deployable=false`、`gpu_validated=false`。证据目录为 `integrated_compilation_adapter_v2/`；前一次聚合脚本失败的目录保留为未验收记录。
- #34 的最终集成完整测试在独立 target 运行，通过 415 项。实际 PF 四阶段编译/适配及 missing-SDK 拒绝一并执行；最新 GOL 二进制 SHA 为 `589667fa4675a2bc73c36eebe239cdeaa637c52604a357bd79b260d7647890d3`。此前 `134e0df` 的 400 项日志和旧二进制保留为历史证据，不能套到新源码。
- 最终 phonefarm 回归已通过。前两轮失败记录继续保留：路径别名夹具对照失败、已知 spawn 失败却留下恢复 marker。独立反例还复现共享 probe 输出将非 root 误判为 root、远端不完整账本漏掉存活组、主机 marker 错误布尔类型或缺失进程组被清除。修复后，远端账本 17 项、master 13 项及主机锁 6 项独立反例通过；新的 634 项全量日志绑定 `cd71c4e`，PF 二进制 SHA 为 `2065e6dd948109f00be3e860b792267e6275f812523f24a4275f45e2a78219de`。未知状态仍保留恢复门禁。#24 独立构建分支 535 项通过；满未读 stderr 的 1 秒工具超时在 1.027 秒 SIGKILL 后确认进程组消失，无需外层强杀；其 present SDK 自动桥接正在单独实现。
- #25 分支最终源码 `f526b31` 通过 568 项与 default clippy，仍有既有告警；发布 head `25f5676` 仅增加证据文档。独立实际 CLI 重建十个 bundle，与 driver JSON 完全一致；十份报告均 exit 1 / BLOCKED，实际 GOL judge 读取十个报告 hash 并 exit 1 / BLOCKED。fake prepare/collect 的 exit 0 仅表示机械完成；synthetic、proof/promotion=false 全程保留。完整 raw/argv/stdout/stderr 留在 worktree，紧凑摘要与复现脚本进入 PR。
- #23 的独立审查实际复现了失败 Vulkan create 返回未定义输出句柄时误销毁的反例；修复后同一反例 exit 0。生产 hook 拒绝没有 source/layout/target/compute 合同的旧启发式路线，离线 ownership 通过不能用来开启注入。
- 测试日志与原始 JSON 保存在本机 `phonefarm/tasks/system_handoff_20261011/` 及各 worktree 的 `tasks/`。集成分支目前仅本地存在；这些记录不代表 GitHub CI 已通过。
- 本轮无真机部署、真实训练或模型 API 调用；**0 模型 API 调用**。内部代码开发与评审不作为被测模型的实验成本。

## 剩余系统接口与完成条件

| 接口 | 当前缺口 | 验收所需实际证据 |
|---|---|---|
| game_opt_loop → phonefarm shader v2 | 已核对真实序列化的 fan/Inject；支持范围限于现有白盒载体，完整条件/采样仍待真机 | 同一请求被明确接受或明确拒绝；requested 字段保留，observed 未知不虚构；未知条件不能授予资格 |
| storage-image 算子 → 灰层 sampled-image ABI | #31/#34 已提供独立编译绑定及自动四阶段准备，#21 可实际适配；prepare/inspect 实际 CLI 已验证 | 统一冻结准备账本；PF #24 已提供显式监督合同，准备 driver 仍未启用有 SDK 的 native 构建；实际 Android 构建、Vulkan 验证、GPU 等价与工件身份贯通 |
| 动态层 → 真实游戏试验 | #23 已修复 host 可验证的完整录制代/dispatch 所有权；一加挂点、参数、图像用途、布局/计算状态及命中仍未验证 | Android/Vulkan 验证、实际 GPU 完成、copy 等价基线、目标/非目标 pass 与具体管线合同、实际命中；已录制的注入不能通过关闭开关撤回 |
| refbench → 配对连续画面 | 原生 producer 和 raw→PNG 桥已发布；尚无 Android/GPU 实采及来源核验 | 真机构建和相同场景/相机/时间的独立渲染与读取，帧 ID/格式/色彩/提交完成与捕获账本，不把候选输出作为参考；单独进行性能运行 |
| 序列质量 → 资格/候选裁决 | CLI 的合成 PASS 不能自动授予真实来源资格 | 捕获来源核验、候选工件绑定、冻结阈值、坏帧证据；缺失或未知阻断晋级 |
| 模型工件 → 实际训练/量化/runtime | 五阶段协议完成不证明权重、模型格式或部署语义有效 | 可信数据/格式检查、真实 driver、预算计量、租约与恢复、实际 Android 推理和性能/质量报告 |
| 探索 → 独立确认 | #25 已发布 host 调度与 Android worker 命令源；三基线、十臂 fake 全链实际执行与重建，保持 SIMULATED_BLOCKED/BLOCKED；真机 worker 尚未执行 | 实际 cold/root/boot/完整快照/采样与恢复退出 receipts；远端 worker/子组持久身份与互斥，超时后先确认停止再恢复；预冻结顺序、探索隔离、每臂唯一工件和不重叠整窗、完整轮数与实际来源 |
| 档案 → 统一案例页面 | #32/#33 已展示完整评测、独立确认和模型阶段，浏览器与有界文件扫描机制已验证 | 仍需真实案例数据；保存声明、hash 匹配或页面可见不授予实验资格；原始来源/恢复/成本未知继续保留 |

上述接口的验收证据齐备后，才安排冻结预算的整链路候选与国内模型比较。接回一加时先执行 [只读资格和基线](oneplus-acceptance.md)，不从 root 成功直接跳到收益声明。
