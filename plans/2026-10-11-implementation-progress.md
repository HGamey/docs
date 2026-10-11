# 本轮实现与验收进度

更新：2026-10-12（交接与原始验收批次始于 2026-10-11）。需求与五仓主线审计见 [系统交接计划](2026-10-11-system-handoff.md)，跨仓验收见 [系统 epic #11](https://github.com/HGamey/.github/issues/11)。所有下列 PR 当前均为 draft、未合并；本文件不将离线夹具或历史红魔结果作为一加真机验收。

实际运行入口、结果含义与页面启动方式见 [Harness 运行说明](harness-runbook.md)。

## 当前目标：优化收益、代价与边界

用户已明确本阶段的交付：回答具体改动能优化多少、付出什么代价、在哪些设备与游戏场景有效。设备连接、root、读取、控制回读和编译只作为实验前置。当前一加尚没有合格的游戏优化效果量，不能把前置能力验证计作收益。

首个真实案例使用终末地，保持游戏画质与帧率设置，优先评估系统 GPU 下界或 DDR 下界的单变量改动。先记录负载频率与瓶颈信号，再固定候选和重复对照；统计提交节奏代理时明确其来源与适用范围，不改称显示 FPS。报告同时列出有效收益、温度等代价、无效/退化候选及置信范围。功率单位或放电窗口未验证时保留 UNKNOWN，不换算成手机功耗结论。

五页阶段 PPT 暂作过程档案。后续汇报以真实效果、代价和边界组织内容，前置能力与回归数量仅占辅助位置。

## 可审阅的代码交付

| 系统位置 | PR | 已交付的机制 | 尚未验收的范围 |
|---|---|---|---|
| 交接与状态 | [docs #5](https://github.com/HGamey/docs/pull/5)、[game_opt_loop #26](https://github.com/HGamey/game_opt_loop/pull/26) | 会议需求、五仓职责、历史证据边界、验收顺序 | 整个系统 epic 仍开放 |
| 评测证据与档案 | [game_opt_loop #27](https://github.com/HGamey/game_opt_loop/pull/27) | 两种评测入口保留完整 receiver 扩展；离线资格回放不能进入前沿；旧报告明确未资格化 | 旧报告可读不等于重新获得真实游戏资格 |
| 方向与专家决策 | [game_opt_loop #28](https://github.com/HGamey/game_opt_loop/pull/28) | shader/optimize 共享 campaign；冻结请求、工件、条件、决定和负结果；pending/reject 不评测，恢复不重复下发 | 模型阶段与动态灰层仍需实际连接到候选调度 |
| 实验资格 | [phonefarm #18](https://github.com/HGamey/phonefarm/pull/18) | 逐目标必需指标、代理/显示语义、root/冷机/稳定基线/放电窗口/还原门禁，入口实际消费；私有 probe 修复并发 root 误判，530 项回补测试通过 | 真机资格复核；连续画质仍需真实捕获与消费端桥接 |
| 连续画质 | [phonefarm #17](https://github.com/HGamey/phonefarm/pull/17) | 配对 PNG、来源与 hash 账本，空间/色偏/相邻残差/坏帧检查，错配或解码异常不获资格 | hash 一致性不证明真实捕获；尚无光流、遮挡或完整鬼影检测；报告始终不授予真实游戏来源资格 |
| 条件与设备独占 | [phonefarm #19](https://github.com/HGamey/phonefarm/pull/19) | shader v2 条件解析、请求/实际分离、载体参数回读；完整租约生命周期已回补原草稿，533 项通过，旧入口保持兼容；主进程退出而子组存活及未知恢复字段保留屏障；真实 GOL fan/Inject 兼容；质量采集不进入性能样本 | 无端到端放电窗口或真实游戏收益资格；不支持的注入/控制明确 ABORT；实际远端恢复待验收 |
| 独立确认 | [game_opt_loop #29](https://github.com/HGamey/game_opt_loop/pull/29) | 事先注册轮数/顺序/收益及非劣效界与基线准入 hash；只消费版本化有效单轮观测；原始工件去重、同设备/boot 的完整 ENV 和 capture 窗不重叠、单侧置信上界裁决 | 采集 driver 见 PF #25；真机行为、设备身份、采集来源和注册时间尚未验收或认证；不改前沿或授予游戏优化证明 |
| 单轮原始观测验收 | [phonefarm #22](https://github.com/HGamey/phonefarm/pull/22)（依赖 #18） | 从原始记录重算七项门禁、至少三轮稳定基线、冻结绑定、控制与恢复完整快照和整窗放电；542 项源码测试，实际跨仓 CLI 冻结→10 观测→judge 跑通 | 离线入口没有采集设备；当前仅支持 Android root/sysparam/KGSL 提交代理/电池；旧目录缺 cold/root/boot/退出/恢复 receipts 不能补成有效观测 |
| 持租约的确认采样 | [phonefarm #25](https://github.com/HGamey/phonefarm/pull/25)（依赖 #22，另引入 #19 租约） | 实际 prepare/collect/reconcile 命令源、冻结触控负载、三基线、五组交替 A/B、控制/采样/完整恢复与永久关闭旧会话；568 项分支测试；实际 CLI 重建十份观测与 GOL 裁决 | fake transport 未执行 Android worker，观测与裁决均 synthetic/BLOCKED；真实 root/setsid/procfs、节点持续生效、KGSL/放电窗口、场景重复性与实际来源待验收；HDC/无 root 明确拒绝 |
| 模型工件阶段 | [game_opt_loop #30](https://github.com/HGamey/game_opt_loop/pull/30) | 五阶段 driver 请求与角色/格式/工件冻结、逐阶段专家审批、预算声明门禁、原子恢复与显式 reconcile | 345 项测试使用假驱动；没有真实训练、量化、Android 推理或 phonefarm 资格接入；完成协议不授予模型语义或推广资格 |
| 动态 SR 灰层工件 | [phonefarm #20](https://github.com/HGamey/phonefarm/pull/20) | sampled-image ABI 反射、冻结候选/构建/层 hash、有界构建子进程、实际 native 记录/提交计数 | 原分支为 host 证据；本轮实际 Android 编译与 SPIR-V 校验见 #26 及下节，GPU 未验证；所有权见 #23，deployable=false |
| 有限 SPIR-V 适配与工件消费 | [phonefarm #21](https://github.com/HGamey/phonefarm/pull/21)（依赖 #20） | storage input 转 sampled input，保留 storage output；`build --adapted-artifact` 使用已校验快照；530 项离线测试及 6 个实际 campaign 工件适配 | GOL #31 新增独立的本地编译绑定；PF 的原始关联仍标为 caller_supplied_unverified；自动准备 driver 见 GOL #34；本轮实际 SPIR-V 校验与 Android 构建见 #26 及下节；GPU 像素等价仍缺 |
| 候选编译证据 | [game_opt_loop #31](https://github.com/HGamey/game_opt_loop/pull/31) | 实际 Naga 编译与安全 gate、源代码/SPIR-V/编译输入/可执行文件冻结；外部 expected identity 约束重放；340 项测试及实际跨仓四 CLI | 本地完整性与编译重放不等于可信来源、算法等价、GPU 验证或部署资格；自动准备 driver 见 #34；仍不授予部署资格 |
| 原始证据的只读展示 | [game_opt_loop #32](https://github.com/HGamey/game_opt_loop/pull/32)（依赖 #27） | 展示完整 receiver/原始 A/B/资格与来源；矛盾 summary PASS 或缺项不显示有效通过；339 项测试、严格 clippy 与实际浏览器检查 | 历史前沿只作档案展示；证明/晋级始终 false；统一案例入口见 #33；展示不授予证明或晋级资格 |
| native 资源所有权 | [phonefarm #23](https://github.com/HGamey/phonefarm/pull/23)（依赖 #20） | 按录制代与每次 dispatch 保留不可改写的完整资源；合法边界退役、失败输出句柄清理；518 项测试，每种 fixture 39 类实际 hook 场景，独立反例复核 | 仅 host/mock 下层验证；生产拒绝旧启发式注入；真实图像布局/目标/计算状态合同、Android/GPU 与真实游戏验证未完成 |
| 统一案例页面 | [game_opt_loop #33](https://github.com/HGamey/game_opt_loop/pull/33)（依赖 #32） | 固定根目录内有界只读扫描，独立确认/模型阶段及关联原始 JSON、exact bytes/SHA、未知/负例/恢复/成本同页；351 项测试、严格 clippy 和实际浏览器检查 | 保存的 PASS/completed 仅是原报告声明；页面不运行 driver，证明和晋级始终 false；关联缺失/越界/符号链接/预算不足明确显示 |
| 原生构建进程监督 | [phonefarm #24](https://github.com/HGamey/phonefarm/pull/24)（依赖 #21） | 显式 inherited-group 接口，所有 helper 继承独立外层进程组；超时/溢出先终止再日志；535 项测试与独立满 stderr 管道反例复核；GOL 自动桥接见 #35 | 该 PR 以 controlled synthetic SDK 验证进程机制；真实 NDK 六阶段另见 #26，GPU 未执行；成功输出仍需外层证明进程组消失再核验，deployable=false |
| 自动候选准备 | [game_opt_loop #34](https://github.com/HGamey/game_opt_loop/pull/34)（依赖 #31） | 冻结两端私有工具和候选，实际 compile/verify/adapt/verify 四阶段、有界组监督、原子账本、只读 inspect；355 项测试、严格 clippy，独立工具/SDK竞争/后代/重哈希反例复核 | 此版本只接受 PREPARED_OFFLINE；缺失 SDK 通过私有空 SDK 留下实际拒绝，完整链 false；native host 桥接见 #35；有限真实 NDK 编译见 #26，GPU、游戏 eval 和质量仍未验收 |
| 自动原生准备 | [game_opt_loop #35](https://github.com/HGamey/game_opt_loop/pull/35)（依赖 #34，消费 PF #24） | 完整有界 SDK 快照、固定四/五源文件合同、实际六阶段与构建/验证两组停止证据；367 项及严格 clippy，通过独立 receipt 篡改与遗留子进程反例 | 该 PR 的 synthetic SDK 只验证 host 机制；完整真实 NDK 含链接或超过规模上限时仍拒绝，有限真实 profile 六阶段见 #26；GPU 未验收；所有部署/证明标记仍 false |
| 原生白盒配对 | [refbench #2](https://github.com/HGamey/refbench/pull/2) | 独立 native LR/HR 与候选输出分离、固定同帧参数、raw 导出及严格 raw→PNG 质量消费桥；质量模式不能用于性能 | 11 项 host 测试；本轮 APK 构建/安装见下节，GPU 配对采集未执行；native 来源暂映射 unknown，指标可 PASS 而总体 UNKNOWN；不是实际游戏、没有 FrameGen 中间真值 |
| 捕获完成合同 | [refbench #3](https://github.com/HGamey/refbench/pull/3)（基于 #2） | 新 run-scoped 输出、写入失败保持失败、全部 raw/out/manifest 完成后创建唯一 completion 标记；15 项 host、syntax、实际 NDK 构建及独立窄审通过 | 新 APK 未部署，GPU/LR/HR 实采与来源未验证；旧无 marker 的捕获只能归档，不能导入成新合同成功 |
| 单个确定性系统案例 | [game_opt_loop #37](https://github.com/HGamey/game_opt_loop/pull/37)（基于 #29） | 一次 case plan/run 冻结工具、输入与预算，串接 prepare → freeze → collect → judge；持久 intent/UNKNOWN 停止；355 项及严格 clippy，九项实际案例 CLI，真实 PF + 明确 fake transport 拒绝复核 | 真机载体/温度/功耗/控制资格及真实采样尚待完成；probe 不替代 plan；host 退出不代表远端已停止；loaded preset 仅透传 ID，不放宽数值 |
| 加载态确认消费 | [game_opt_loop #38](https://github.com/HGamey/game_opt_loop/pull/38)（基于 #37） | 旧冻结指纹保持不变；新温度合同、规则版本、profile/reference/hash 和八项门禁逐项绑定，独立温度窗口原始工件去重；原版本 360 项；`9c6b393` 明确 goal 透传和同字节条件绑定，362 项与严格 clippy | host 合同验收；PF producer/原始重建和真机整窗仍待完成，不按 true cold 标记升级旧观测；缺省目标仍是功耗效率 |
| 有限真实 NDK 导出 | [phonefarm #26](https://github.com/HGamey/phonefarm/pull/26)（基于 #24） | 官方 r28c/darwin/arm64/API30 明确文件与别名映射、逐块/枚举预算、NOFOLLOW；3490 项/483,954,086 bytes；21 Python 与 535 Rust/default clippy；真实六阶段见下节 | 只支持注册的有限 profile；编译完成不授予 GPU、部署、图像等价或游戏资格 |
| 加载态 producer 与控制探测 | [phonefarm #27](https://github.com/HGamey/phonefarm/pull/27)（base 为已冻结 #17–#25 依赖分支） | `e4e059e` 的 669 项旧 gate 保留；新 `796ea31` 使用设备 uptime 的 start-to-start deadline，671 项完整本地测试及 default clippy exit 0（保留 52 条警告）；`311439c` 只追加证据。三次真实探测均在写入前拒绝，第三次实际间隔已符合规则 | 不包含 #26；三次均未执行参数写入、11s 保持或完整恢复；仍需合格独立真机验证及正式 A/B，fake 十臂继续 BLOCKED |

单轮观测重建和消费端已通过实际 CLI 合同验证。早期夹具由假设性 driver 声明和转换过的历史 trace 片段构成；#25 新夹具实际执行 host 调度与明确的 fake transport，十份 raw 观测和最终裁决始终 BLOCKED。两者均没有真实设备采集认证。真实训练器、量化器和 Android runtime driver 尚未运行。

## PR 依赖与审阅顺序

这些功能分支共享文件，审阅和后续整合需遵循实际依赖：

- game_opt_loop 的档案/页面链为 #27 → #32 → #33；候选准备链为 #31 → #34 → #35。#35 消费 PF #24，五输入版本另需 #23 所有权头文件；#26、#28、#29、#30 分别以 main 为基线。
- 单系统案例 #37 以 #29 为基线，消费 PF #25 的版本化 prepare/collect 协议；更换温度 profile 还需要新的 PF producer/观测合同与消费者验收。
- 加载态确认消费 #38 以 #37 为基线；有限 NDK 导出 PF #26 以 #24 为基线，原 SDK 快照规则保持严格。 PF #27 以 `codex/loaded-thermal-integration-base` / `6025119` 为冻结依赖基线，包含尚未合并的 #17–#25，未包含 #26；不得当成可直接合入 main 的独立补丁。refbench #3 基于 #2。
- phonefarm 的观测链为 #18 → #22 → #25；#25 在 #22 上另外引入 #19 的设备租约并扩展持久恢复状态，集成时保留 #19 完整入口与 #25 更新后的租约模块。
- phonefarm 的工件/构建链为 #20 → #21 → #24；资源所有权 #23 以 #20 为基线。#17 和 #19 分别以 main 为基线。
- 本地集成分支同时检查这些功能；各 PR 的独立测试数与集成测试数绑定各自提交。当前没有合并任何 PR。

## 集成验证如何解释

本轮各功能从远端基线或明确依赖的 draft 分支建立独立分支，再 cherry-pick 到本地集成分支，检查功能同时存在时的实际 CLI 行为。当前共 26 个 draft PR；新增 #37/#38/PF #26 独立分支通过。GOL 加载态整合 `d4e5208` 已通过 441 项及严格 clippy；明确 goal 最终整合 `291b0f1` 的默认 harness 443 项及严格 clippy 通过，本次八项可选 native 测试体和可选实际 PF adapter 路径未启用，实际 native 验证仍绑定此前 `d4e5208`。PF `e4e059e` / 669 与同源集成 `ef0e7b5` 保留为旧检查点；新 `796ea31` 完整本地 gate 为 671 项、default clippy exit 0，集成 `61530ea` 的 1049 项源文件、测试及规格 SHA 全部一致，`src/loop_v1/knobs` 差异为空，继承该同字节 gate。root 未重复执行 Cargo，也不将同源比较冒充第二份测试。测试数是对应提交的回归总数，不能相加成更多独立实验证据，也不能换算为真实游戏效果。

- game_opt_loop 集成分支 `codex/handoff-game-integration` 在 `d7e5960` 上共 **427 项测试通过**，`clippy --all-targets --all-features -- -D warnings` 通过。包含 #26–#35、实际候选准备及 campaign/证据兼容测试：资格不足不进入前沿、原始扩展与专家决定不丢失、恢复零重复评测。单轮独立确认仍由夹具供数，模型阶段由假驱动执行；八项 native CLI 测试显式启用并实际执行。日志在 `integration_game_native_bounded_workers/`，可执行文件 SHA 为 `dbcc34d5d4621998cdd2145eb1ba6956dd46f680eefcff17099af4f93dad4fe2`。旧 `5149a33` 的 415 项仍作为对应提交的历史记录。
- phonefarm 集成分支 `codex/handoff-phonefarm-integration` 在生产源码 `f88188c` 上共 **634 项测试通过**，包含 #17–#25；default clippy exit 0，有 67 个告警标题，未宣称 strict clean。日志在 `integration_phone_schema_bounded_workers/`，PF 二进制 SHA 为 `5abd65372e3da6dae3a932d97617489b8f4f86cbe1d55a059d5785ff3cc5b397`；之后 `c27d1cc` / `6025119` 仅同步 #19/#25 证据文档。测试调用固定 `5149a33` / SHA `589667fa…` 的 GOL，完整 driver 夹具保留 synthetic/BLOCKED；ADB/HDC 真机调用 0。此前 `cd71c4e` 的 634 项、`8e819ff` 的 602 项及历史片段 freeze→10 观测→judge PASS 保留为对应旧提交的离线证据，不能用于证明真实采集或游戏收益。native host/hook 验证仍不证明 GPU 执行或完成。
- `75dd2bb` / `8e819ff` 对应的两个集成可执行文件实际执行了 compile→verify→adapt-storage→verify-adaptation，四条 CLI 均 exit 0。原始 source/SPIR-V 的 bytes/hash 一致，编译身份和执行前后工具 hash 冻结；结果仍 `deployable=false`、`gpu_validated=false`。证据目录为 `integrated_compilation_adapter_v2/`；前一次聚合脚本失败的目录保留为未验收记录。
- #34 的此前集成完整测试在独立 target 运行，通过 415 项，GOL 二进制 SHA 为 `589667fa4675a2bc73c36eebe239cdeaa637c52604a357bd79b260d7647890d3`；仍用于上述 PF driver gate 的固定消费者。#35 的新 427 项 native 验证使用固定 PF `cd71c4e` 快照（SHA `2065e6dd…`）及 legacy PF `86f3ab9`（SHA `db1721a7…`），前后工具 hash 不变；未将新 PF schema 二进制冒充 native 验证输入。此前 `134e0df` 的 400 项日志和旧二进制也保留。
- 最终 phonefarm 回归已通过。前两轮失败记录继续保留：路径别名夹具对照失败、已知 spawn 失败却留下恢复 marker。独立反例还复现共享 probe 输出将非 root 误判为 root、远端不完整账本漏掉存活组、主机 marker 错误布尔类型或缺失进程组被清除。修复后，远端账本 17 项、master 13 项及主机锁 6 项独立反例通过；未知状态仍保留恢复门禁。#24 独立构建分支 535 项通过；满未读 stderr 的 1 秒工具超时在 1.027 秒 SIGKILL 后确认进程组消失，无需外层强杀；其 present SDK 自动桥接已由 GOL #35 在 synthetic SDK 下完成。
- #25 的 `f526b31` 历史 gate 通过 568 项与 default clippy；其独立实际 CLI 重建十个 bundle，与 driver JSON 完全一致。十份报告均 exit 1 / BLOCKED，实际 GOL judge 读取十个报告 hash 并 exit 1 / BLOCKED。fake prepare/collect 的 exit 0 仅表示机械完成；synthetic、proof/promotion=false 全程保留。完整 raw/argv/stdout/stderr 留在 worktree，紧凑摘要与复现脚本进入 PR；新 schema gate 与这批历史 CLI 证据分别标注。
- #19 原草稿回补通用进程组/持久恢复生命周期，`28a4868` 上 533 项/default clippy exit 0，发布 `4e171f7` 仅加证据；不导入确认 driver。未知顶层恢复字段/format 也拒绝，旧无扩展 idle marker 兼容，旧 PID-only marker 无法补猜 PGID。#25 同步该窄补丁的新源码 `22641c3` 已通过新的 **568 项/default clippy exit 0**，发布 `ff22a1b` 仅加证据；仍有既有告警。历史十臂独立重建仍绑定 `f526b31`，没有翻标为新源码。
- #35 在 `70141d5` 上 367 项与严格 clippy 通过，发布 `afce270` 为证据归档。实际四/五输入正例均完成六阶段、八 helper 同属私有构建组；missing SDK 仍 BLOCKED。独立审查证实 rehashed nested receipt 必须与原 stdout 一致，遗留关闭管道子进程保持 UNKNOWN，验证阶段不执行。根集成首次已过八项 native 测试，但两项旧额外耗时断言失败；PF 新 schema 首轮也有三秒 stdin 测试限时失败，单独同预算复验通过。保留失败记录后，两仓以 **`RUST_TEST_THREADS=4`** 重新完整回归，分别通过 **427/634 项**；这次复验未改源码、生产预算或测试限时。
- #23 的独立审查实际复现了失败 Vulkan create 返回未定义输出句柄时误销毁的反例；修复后同一反例 exit 0。生产 hook 拒绝没有 source/layout/target/compute 合同的旧启发式路线，离线 ownership 通过不能用来开启注入。
- 测试日志与原始 JSON 保存在本机 `phonefarm/tasks/system_handoff_20261011/` 及各 worktree 的 `tasks/`。集成分支目前仅本地存在；这些记录不代表 GitHub CI 已通过。
- 上述已完成的离线验收批次无真机部署、真实训练或模型 API 调用；**0 模型 API 调用**。随后的一加真机阶段独立记录在下节，不能将这条 0-device 声明扩大到新阶段。内部代码开发与评审不作为被测模型的实验成本。

## 2026-10-12 一加与真实 Android 工具链阶段

用户已接回一加并授权并行推进。设备操作只由 measurement owner 执行，其他 agent 并行进行载体构建、案例入口和接收端温度模块。新证据在本机 `phonefarm/tasks/oneplus_live_20261012/`；未合并 PR，未调用模型 API，未重新 root/刷机或删除用户应用。

| 工作 | 实际已完成 | 尚待完成 |
|---|---|---|
| 设备身份 | PKG110，Android 16，Magisk uid0、SELinux Enforcing；实际 serial/boot 与 KGSL/控制节点只读记录 | 节点持续生效、完整恢复及真实采样资格 |
| 游戏状态 | 终末地 1.5.3 / code 50，显式 activity 启动并点击继续后进入可玩的 3D 营地；采样窗外实际执行相机滑动与反向滑动，三张 2376×1080 原图及六条带起止时间的命令已存档 | 近似视角返回不能证明场景精确复位；尚无正式游戏性能采样或优化收益；后台原神下载状态仍需排除或明确记录 |
| Mac Android 工具链 | 官方 command-line tools 校验 SHA 后安装 NDK 28.2.13676358、build-tools 35.0.0、android-35；原始 NDK 8897 项/约 2.96GB/23 个链接，确实超过原快照合同；[PF #26](https://github.com/HGamey/phonefarm/pull/26) 导出已审阅/发布/本地整合；新 Rust producer 的同字节完整 gate 已通过 | 有限 profile 之外的 SDK 仍拒绝；没有放宽原 regular-only 快照门禁 |
| refbench APK 与运行自检 | PR #2 的 `ad251d8` 实际构建成功，10.66s；arm64 APK 1,405,348 bytes，SHA `cb59df213d2e3ff9ea433702aba36e9c10d6ebb4078a9c994a4dcf3d62858db7`；签名和 ELF 核验、安装 4.198s/exit 0。两轮独立 BW600 自检有不同 run_id；第二轮原 PNG 看见带宽场景，真实线程检查见 RefbenchDrv；每轮自报 frames_submitted=600/clean_exit=true，停止后实际 pidof 为空并返回终末地 | 自报提交数不等于独立 GPU 完成或显示 FPS；未采集正式 trace/ENV、功耗或 A/B；LR/HR 导出和来源核验仍未执行 |
| 六阶段真实 native 编译 | GOL `70141d5` + PF `f88188c` + 明确的有限真实 NDK profile，ownership5 六阶段完成并确认组停止；130.96s，arm64 层 3,085,992 bytes，SHA `b74ce25c577ae3ae0b363e54739f166ac13244cacaa2d4fdb8254626bdd9f21b`；三份 SPIR-V 实际 `spirv-val` exit 0 | GPU 执行、像素等价、布局/目标/计算状态、真实游戏挂点；全部 deployable/proof/promotion 仍 false |
| 温度规则 | 只读取得实际 typed sensor/trip/HAL；HAL CPU/GPU severe 95°C、skin 60°C，所采 battery 约 41°C；按域上限和稳定窗口判读。PF `796ea31` 保留原 caps、HAL 和稳定参考规则，以 uptime deadline 避免读取耗时累加；671 项完整 gate/default clippy exit 0，完整十臂 fake 链仍 BLOCKED | 三次控制探测均未通过写入前准入；实际整窗性能与控制/恢复资格尚未验证，详见 [一加验收顺序](oneplus-acceptance.md) |
| 单 GPU 参数准入实测 | 新签名 PF 执行独立 control-only 请求，24.039s 后在无候选暖机阶段 exit 83/BLOCKED；系统报告 HAL thermal_status=3/SEVERE，框架拒绝。未创建 initial.snapshot/restore.tsv/state/inputs，未安装 worker 或写控制/ftrace；独立正常租约检查成功，GPU min 前/后均 11；操作前与拒绝后原 PNG 已保留 | 请求的 11→10、11s 保持与完整恢复实验均未执行，不能授予参数持续生效或恢复资格；需新的合格温度窗口后独立重试 |
| HOME 后独立重试 | 自然冷却后 HAL 降至 2；配置探测明确不属于终末地性能试验，未修改原神下载。58.860s 后仍在写入前 BLOCKED：七个完整样本中实际间隔有 6.07–6.12s 超过冻结上限 6s；CPU zone17 的跨度 3.9°C，全域最大 zone19 为 6.6°C，也超稳定性限 3°C | 已按 start-to-start deadline 窄修并通过新完整 gate，未提高间隔/温度阈值。原始失败不重标成功；旧 `e4e059e`/669 记录仅属旧源 |
| deadline 修复后独立探测 | signed PF `796ea31` / SHA `87669fbc…`，driver 53.15s；七个实际点跨度 31.89s，间隔 5.27–5.35s 均不超过 6s，HAL 全为 0。仍在写入前 BLOCKED：CPU zone26 波动 7.7°C，zone25/27 为 5.4°C，超过冻结的 3°C 稳定规则 | 没有 initial snapshot、restore graph、remote state/inputs 或 worker 安装；实际控制写入为 0，11s 保持/完整恢复 NOT_RUN。正常租约检查成功、GPU min 11→11；两张原图纯黑，不能作为可见游戏或桌面。原神下载仍 active_unknown |
| 目标游戏归因与固定计数尺度 | [组织 issue #19](https://github.com/HGamey/.github/issues/19)记录 parser 只按 comm/context 接收、SPF 各臂独立推断的缺口；已安排独立实现与原始所有权 witness 核验 | 目标 package/TGID/TID/starttime/context 所有权及事前冻结 SPF 尚未实现或真机验收；不能据当前 proxy 指标声称目标游戏优化，显示 FPS 仍未验证 |
| 电池功耗 | 发现原 sampler 固定 µA 假设不适用于直接解释本机读数；同型号家族 Android 16 官方源和 charge_counter 变化支持 mA/充电负号方向的待验证解释 | 实际内核/驱动绑定、明确单位 adapter、停充与完整放电窗口；`power_now=0` 不当有效功耗 |
| 单案例入口 | [GOL #37](https://github.com/HGamey/game_opt_loop/pull/37) `a06c751`，355 项与严格 clippy 通过；[GOL #38](https://github.com/HGamey/game_opt_loop/pull/38) `9c6b393` 加载态消费及明确 goal，362 项通过；最终整合 `291b0f1` 默认 harness 443 项/严格 clippy 通过，保留此前 `d4e5208` 的实际 native 八项记录；明确 fake transport 的实际 CLI 仍为 SIMULATED_BLOCKED | 真实 prepare/collect/judge、三基线和五对独立确认；首个真实目标明确选择 `render_cadence`，功耗数值不作为该目标的合格指标 |

真实 native 编译证据保存在额外的 `real-host-build-summary.json`。历史 journal 的 `android_codegen_validated=false` / `real_ndk_validation=not_run` 未被改写；新增观察只证明工具确实生成 Android 工件，不把历史保守字段改成已部署。温度 preset 的提高也不能替代稳定性、放电、控制与恢复证据。

新 refbench 捕获完整性修复 `bfffeb2` 已发布 [draft #3](https://github.com/HGamey/refbench/pull/3)，基于 PR #2：run-scoped 输出、sticky 写入失败、create-only `refbench/native_capture_completion@1` 完成标记，全部文件与 fence 完成后才发布，桥接器拒绝没有标记的旧捕获。15 项 host 测试、syntax 与官方 NDK 构建通过；新 APK SHA `3303595eed5bfafdf6e24a1632bebfaf3134d059cdf039ea217acc8660a04400`，1,409,444 bytes，未部署。两轮已运行的 BW 自检仍绑定旧 `ad251d8` / `cb59df…`；新 native 质量捕获的 GPU、来源与性能资格均未验证。独立的两帧 LR/HR 捕获计划已固定新版身份与 run-scoped 完成对账，实际执行仍为 false；无候选时仅归档，不生成画质 PASS 或性能结果。

实验过程按实际检查点追加到 `oneplus_live_20261012/reporting/`。保留原 PNG、命令、失败记录、工具及源文件 hash；有实际起止时钟的截图绑定命令，旧截图仅有文件修改时间时明确标记采集时间未认证。截图在性能窗口外获取，能力自检、离线机制、真实性能三类结果分别解释。后续 PPT 使用该索引，只有实际有效采样才能生成性能收益图表。

## 2026-10-12 后续隔离与连接恢复

- GOL #38 追加旧格式与控制能力目的隔离：源码 `0dd011e` 在原 #38 基线通过 364 项完整本地测试及严格 all-targets/all-features Clippy，31 个外层重新计算 hash 的矛盾报告均 BLOCKED。历史 PF 省略 version/policy 的有效旧格式保持兼容，显式版本、温度绑定或控制目的须与性能合同一致。发布 `923bf7e` 仅追加紧凑证据，PR 仍为 draft。
- 本地整合 `6b7d45c` 在独立 COW target 通过 **445 项默认 harness 测试及严格 Clippy**，源文件与历史 accepted 工具前后 hash 不变。默认测试中的八项可选 native 测试体和可选实际 PF adapter 仍未启用，不扩大此前 `d4e5208` 的实际 native 范围；旧 `291b0f1` / 443 与 `5afa43…` 工具保留。
- 控制探测专用 `capability_absolute_v1` 源码 `19bfed5` 已通过独立 **676 项完整本地测试和 default Clippy exit 0**（保留 53 条警告），证据提交 `c3671f0` 已追加至 draft PF #27。保持绝对安全上限、HAL、30s/至少六点/间隔完整性及停止、保持、恢复屏障，温度波动只作为配置读回检查的诊断。性能 prepare/collect/worker/consumer 明确隔离该目的，正式 loaded/cold 的稳定性规则不变；真机尝试结果见下节，控制能力不等于收益。
- queued-origin 与 baseline 冻结 SPF 在独立 PF 分支实现。独立审查发现三项直接影响真实指标的缺陷：测量窗 queued 而 tail 才 submitted 的 context/caller 未全部冻结；事件 print fmt 的参数映射仅按 token 校验；task witness 与 lifecycle tracing 间存在采集空隙。旧 `ff71181` 全 gate 正常停止并标为未接受；修复后另行冻结源码和完整回归，不复用旧测试数。设备事件格式清单待实采，显示帧与真实游戏效果仍未验证。
- 用户重新接 USB 后曾出现 Mac 已识别 USB ADB 接口但 transport 无法通信的失败。NATIVE 恢复、LIBUSB 临时对照和重新开关 USB 调试的各次失败独立留档，根因 UNKNOWN。随后用户正常重启手机，**USB `c531e0a` 已在线、root uid0 可用、SELinux Enforcing，实际画面已解锁**，新 boot 为 `b03328bb-9647-4cc1-8e6c-8f534df69dad`。历史失败不代表当前仍断连，也未改密钥或重新刷机。
- 本机已生成并逐页检查五页阶段 PPT，包含五仓职责、终末地原图、可编辑采样间隔图、白盒自检及验收缺口，PPT SHA `71a78aae5f1f773f0e2de2888d843e56afeb130940aa784d8d88e6d642b3167f`。这是过程档案，不能作为完整鸿蒙测试能力或优化收益证明；后续按用户明确的效果/代价/边界重点重写。归档此前检查点有 18 张原 PNG，重启后的新截图另行追加，所有旧失败与未执行项保留。

这些增量未合并 PR，未完成正式 A/B，没有模型 API 调用。源代码回归、连接诊断、控制能力与真实性能各自保留范围。

## 终末地的优化空间与实际阻断

当前 USB 调试与 root 已确认可用。下列记录的重点是候选是否作用于实际负载，以及为什么还不能给出收益百分比。

| 固定场景或候选 | 实际观察 | 能回答的范围 |
|---|---|---|
| 高画质、原 30 帧设置、营地 | UID10382 与唯一终末地 SurfaceView 图层匹配；900 个 present2present 桶中 16ms:1、33ms:890、50ms:9，图层 vote=30，报告桶均值约 30.165Hz | 当前约 30Hz 呈现节奏的单次诊断；50ms 桶比例 1%，不是官方卡顿率。Timeline 分母为 0，不能由 jankyFrames=0 推出零卡顿 |
| 相同画质的新 60 帧负载条件 | 目标 vote=60；30 个频率点中 GPU 全为 578MHz，DDR 25 点为 1555MHz、2 点为 1708MHz、3 点为 2092MHz，LLCC 全为 806MHz | 仅用于寻找瓶颈。30→60 是测试负载配置变化，不能算候选优化。瞬时频率采样不能证明采样点之间的完整驻留 |
| GPU 下限 310MHz 或 DDR 下限 768MHz | 上述全部频率采样点已高于这些下限 | 在本段场景中没有观察到作用机会，先不为低效候选运行完整 A/B；不外推其他场景 |
| GPU 下限 629MHz（合法档位 5） | 独立绿色签名工具执行唯一控制准入；两个暖机点相隔 5.26s，HAL 从 2 升至 3，exit83，在 initial snapshot、安装和控制写入前拒绝 | 候选未执行，收益/11s 保持/完整恢复实验均 NOT_RUN。原 GPU min 仍为 11。这是当前 60 帧负载与系统热策略的边界，不是“root 无法调频”的结论 |

60 帧诊断脚本的 30 次 sleep-after-read 实际覆盖 42.89s；目标直方图加权仅约 29.094s、1336 个间隔。`averageFPS=45.920` 是**有效记录内的桶均值**，不能当整窗 FPS。官方 Android16 的 statsd layer pull 可清空 layer 统计而保留 global 窗口，tracker 失效也可丢失连接；目前原件不足以区分实际原因。桶量化约 1ms/间隔不足以解释 13.796s 的差距，因此该路径暂作诊断，不能用来冻结正式优化主指标。

GPU 候选的写入前拒绝保留为真实负结果；不提高热阈值或重试同一候选。正式对照仍需完整来源/窗口、固定场景、至少三基线和五对 A/B。功耗单位与放电窗口仍未验证，不给 W 或节能百分比。

两个真实 typed 暖机点的 CPU/GPU 最大值分别为 69.0/67.4°C 与 68.6/67.0°C，skin 55.585→55.649°C、battery 42.1→42.2°C；global HAL 仍从 2 升至 3。全局热状态不能仅根据 CPU/GPU 温度推断。拒绝后已目视截图确认原 30 帧设置恢复，并返回 HOME；正常租约检查成功、GPU min=11、瞬时 GPUclk=231MHz。恢复游戏设置不构成候选收益或完整性能恢复实验。

queued-origin 的原三项静态缺陷已修；旧 shell collector v3/v4 在终末地约 190 个线程下仍超出冻结的 20s，实际 fork 事件字段和第三份 baseline raw 的 manifest 也需修正。本地组合 `f36bab1` 的完整 gate 已停止并保留 exit−15/520.34s 记录，**未接受、未部署**。新的固定只读 native reader 正在做实际完整 witness 验证，不能用 host 测试数代替真实采集成功。

## 剩余系统接口与完成条件

| 接口 | 当前缺口 | 验收所需实际证据 |
|---|---|---|
| game_opt_loop → phonefarm shader v2 | 已核对真实序列化的 fan/Inject；支持范围限于现有白盒载体，完整条件/采样仍待真机 | 同一请求被明确接受或明确拒绝；requested 字段保留，observed 未知不虚构；未知条件不能授予资格 |
| storage-image 算子 → 灰层 sampled-image ABI | #31/#34 已提供独立编译绑定及自动四阶段准备，#21 可实际适配；#35 接入 PF #24 六阶段 host 构建/验证与四/五输入合同 | 有限真实 NDK 的 Android 构建和 SPIR-V 校验已完成；尚需 Vulkan 运行验证、GPU 等价与实际工件身份贯通，host 编译不能替代设备证据 |
| 动态层 → 真实游戏试验 | #23 已修复 host 可验证的完整录制代/dispatch 所有权；一加挂点、参数、图像用途、布局/计算状态及命中仍未验证 | Android/Vulkan 验证、实际 GPU 完成、copy 等价基线、目标/非目标 pass 与具体管线合同、实际命中；已录制的注入不能通过关闭开关撤回 |
| refbench → 配对连续画面 | 原生 producer 和 raw→PNG 桥已发布；尚无 Android/GPU 实采及来源核验 | 真机构建和相同场景/相机/时间的独立渲染与读取，帧 ID/格式/色彩/提交完成与捕获账本，不把候选输出作为参考；单独进行性能运行 |
| 序列质量 → 资格/候选裁决 | CLI 的合成 PASS 不能自动授予真实来源资格 | 捕获来源核验、候选工件绑定、冻结阈值、坏帧证据；缺失或未知阻断晋级 |
| 模型工件 → 实际训练/量化/runtime | 五阶段协议完成不证明权重、模型格式或部署语义有效 | 可信数据/格式检查、真实 driver、预算计量、租约与恢复、实际 Android 推理和性能/质量报告 |
| 探索 → 独立确认 | #25 已发布 host 调度与 Android worker 命令源；三基线、十臂 fake 全链实际执行与重建，保持 SIMULATED_BLOCKED/BLOCKED；真机 worker 尚未执行 | 实际 cold/root/boot/完整快照/采样与恢复退出 receipts；远端 worker/子组持久身份与互斥，超时后先确认停止再恢复；预冻结顺序、探索隔离、每臂唯一工件和不重叠整窗、完整轮数与实际来源 |
| 档案 → 统一案例页面 | #32/#33 已展示完整评测、独立确认和模型阶段，浏览器与有界文件扫描机制已验证 | 仍需真实案例数据；保存声明、hash 匹配或页面可见不授予实验资格；原始来源/恢复/成本未知继续保留 |

上述接口的验收证据齐备后，才安排冻结预算的整链路候选与国内模型比较。接回一加时先执行 [只读资格和基线](oneplus-acceptance.md)，不从 root 成功直接跳到收益声明。
