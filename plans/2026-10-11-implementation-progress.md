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
| 条件与设备独占 | [phonefarm #19](https://github.com/HGamey/phonefarm/pull/19) | shader v2 条件解析、请求/实际分离、载体参数回读；完整租约生命周期已回补原草稿，533 项通过，旧入口保持兼容；主进程退出而子组存活及未知恢复字段保留屏障；真实 GOL fan/Inject 兼容；质量采集不进入性能样本 | 无端到端放电窗口或真实游戏收益资格；不支持的注入/控制明确 ABORT；实际远端恢复待验收 |
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
| 原生构建进程监督 | [phonefarm #24](https://github.com/HGamey/phonefarm/pull/24)（依赖 #21） | 显式 inherited-group 接口，所有 helper 继承独立外层进程组；超时/溢出先终止再日志；535 项测试与独立满 stderr 管道反例复核；GOL 自动桥接见 #35 | controlled synthetic SDK 验证进程机制；真实 NDK 和 GPU 均未执行；成功输出仍需外层证明进程组消失再核验，deployable=false |
| 自动候选准备 | [game_opt_loop #34](https://github.com/HGamey/game_opt_loop/pull/34)（依赖 #31） | 冻结两端私有工具和候选，实际 compile/verify/adapt/verify 四阶段、有界组监督、原子账本、只读 inspect；355 项测试、严格 clippy，独立工具/SDK竞争/后代/重哈希反例复核 | 此版本只接受 PREPARED_OFFLINE；缺失 SDK 通过私有空 SDK 留下实际拒绝，完整链 false；native host 桥接见 #35，真实 NDK/GPU、游戏 eval 和质量仍未验收 |
| 自动原生准备 | [game_opt_loop #35](https://github.com/HGamey/game_opt_loop/pull/35)（依赖 #34，消费 PF #24） | 完整有界 SDK 快照、固定四/五源文件合同、实际六阶段与构建/验证两组停止证据；367 项及严格 clippy，通过独立 receipt 篡改与遗留子进程反例 | controlled synthetic SDK 只验证 host 机制；普通真实 NDK 包含链接或超 512MiB 时明确拒绝，真实兼容/Android codegen/GPU 未验收；所有部署/证明标记仍 false |
| 原生白盒配对 | [refbench #2](https://github.com/HGamey/refbench/pull/2) | 独立 native LR/HR 与候选输出分离、固定同帧参数、raw 导出及严格 raw→PNG 质量消费桥；质量模式不能用于性能 | 11 项 host 测试；尚无 APK/GPU 采集；native 来源暂映射 unknown，指标可 PASS 而总体 UNKNOWN；不是实际游戏、没有 FrameGen 中间真值 |
| 单个确定性系统案例 | [game_opt_loop #37](https://github.com/HGamey/game_opt_loop/pull/37)（基于 #29） | 一次 case plan/run 冻结工具、输入与预算，串接 prepare → freeze → collect → judge；持久 intent/UNKNOWN 停止；355 项及严格 clippy，九项实际案例 CLI，真实 PF + 明确 fake transport 拒绝复核 | 真机载体/温度/功耗/控制资格及真实采样尚待完成；probe 不替代 plan；host 退出不代表远端已停止；loaded preset 仅透传 ID，不放宽数值 |
| 加载态确认消费 | [game_opt_loop #38](https://github.com/HGamey/game_opt_loop/pull/38)（基于 #37） | 旧冻结指纹保持不变；新温度合同、规则版本、profile/reference/hash 和八项门禁逐项绑定，独立温度窗口原始工件去重；原版本 360 项；`9c6b393` 明确 goal 透传和同字节条件绑定，362 项与严格 clippy | host 合同验收；PF producer/原始重建和真机整窗仍待完成，不按 true cold 标记升级旧观测；缺省目标仍是功耗效率 |
| 有限真实 NDK 导出 | [phonefarm #26](https://github.com/HGamey/phonefarm/pull/26)（基于 #24） | 官方 r28c/darwin/arm64/API30 明确文件与别名映射、逐块/枚举预算、NOFOLLOW；3490 项/483,954,086 bytes；21 Python 与 535 Rust/default clippy；真实六阶段见下节 | 只支持注册的有限 profile；编译完成不授予 GPU、部署、图像等价或游戏资格 |

单轮观测重建和消费端已通过实际 CLI 合同验证。早期夹具由假设性 driver 声明和转换过的历史 trace 片段构成；#25 新夹具实际执行 host 调度与明确的 fake transport，十份 raw 观测和最终裁决始终 BLOCKED。两者均没有真实设备采集认证。真实训练器、量化器和 Android runtime driver 尚未运行。

## PR 依赖与审阅顺序

这些功能分支共享文件，审阅和后续整合需遵循实际依赖：

- game_opt_loop 的档案/页面链为 #27 → #32 → #33；候选准备链为 #31 → #34 → #35。#35 消费 PF #24，五输入版本另需 #23 所有权头文件；#26、#28、#29、#30 分别以 main 为基线。
- 单系统案例 #37 以 #29 为基线，消费 PF #25 的版本化 prepare/collect 协议；更换温度 profile 还需要新的 PF producer/观测合同与消费者验收。
- 加载态确认消费 #38 以 #37 为基线；有限 NDK 导出 PF #26 以 #24 为基线，原 SDK 快照规则保持严格。
- phonefarm 的观测链为 #18 → #22 → #25；#25 在 #22 上另外引入 #19 的设备租约并扩展持久恢复状态，集成时保留 #19 完整入口与 #25 更新后的租约模块。
- phonefarm 的工件/构建链为 #20 → #21 → #24；资源所有权 #23 以 #20 为基线。#17 和 #19 分别以 main 为基线。
- 本地集成分支同时检查这些功能；各 PR 的独立测试数与集成测试数绑定各自提交。当前没有合并任何 PR。

## 集成验证如何解释

本轮各功能从远端基线或明确依赖的 draft 分支建立独立分支，再 cherry-pick 到本地集成分支，检查功能同时存在时的实际 CLI 行为。当前共 24 个 draft PR；新增 #37/#38/PF #26 独立分支通过。GOL 加载态整合 `d4e5208` 已通过 441 项及严格 clippy；明确 goal 最终整合 `291b0f1` 的默认 harness 443 项及严格 clippy 通过，本次八项可选 native 测试体未执行，实际 native 验证仍绑定此前 `d4e5208`。PF 新 producer 的完整 gate 继续执行。测试数是对应提交的回归总数，不能相加成更多独立实验证据，也不能换算为真实游戏效果。

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
| Mac Android 工具链 | 官方 command-line tools 校验 SHA 后安装 NDK 28.2.13676358、build-tools 35.0.0、android-35；原始 NDK 8897 项/约 2.96GB/23 个链接，确实超过原快照合同；[PF #26](https://github.com/HGamey/phonefarm/pull/26) 导出已审阅/发布/本地整合 | 新整体 gate；没有放宽原 regular-only 快照门禁 |
| refbench APK | PR #2 的 `ad251d8` 实际构建成功，10.66s，源文件前后 hash 不变；arm64 APK 1,405,348 bytes，SHA `cb59df213d2e3ff9ea433702aba36e9c10d6ebb4078a9c994a4dcf3d62858db7`；签名和 ELF 检查完成；持设备租约安装 4.198s、exit 0 | 运行、Vulkan/GPU 完成、LR/HR 导出和实际来源核验；安装成功尚无运行证明 |
| 六阶段真实 native 编译 | GOL `70141d5` + PF `f88188c` + 明确的有限真实 NDK profile，ownership5 六阶段完成并确认组停止；130.96s，arm64 层 3,085,992 bytes，SHA `b74ce25c577ae3ae0b363e54739f166ac13244cacaa2d4fdb8254626bdd9f21b`；三份 SPIR-V 实际 `spirv-val` exit 0 | GPU 执行、像素等价、布局/目标/计算状态、真实游戏挂点；全部 deployable/proof/promotion 仍 false |
| 温度规则 | 只读取得实际 typed sensor/trip/HAL；HAL CPU/GPU severe 95°C、skin 60°C，当前 battery 约 41°C；新增模块按域上限和稳定窗口判读；PF `2a49bd2` 先冻结规则、无候选暖机，再冻结初始快照；针对性温度 11 项、qualification 14 项、观测 CLI 18 项通过，完整十臂 fake 链仍 BLOCKED | 新源码完整 gate、控制 probe 实测和真实 ENV/整窗消费者；详见 [一加验收顺序](oneplus-acceptance.md) |
| 电池功耗 | 发现原 sampler 固定 µA 假设不适用于直接解释本机读数；同型号家族 Android 16 官方源和 charge_counter 变化支持 mA/充电负号方向的待验证解释 | 实际内核/驱动绑定、明确单位 adapter、停充与完整放电窗口；`power_now=0` 不当有效功耗 |
| 单案例入口 | [GOL #37](https://github.com/HGamey/game_opt_loop/pull/37) `a06c751`，355 项与严格 clippy 通过；[GOL #38](https://github.com/HGamey/game_opt_loop/pull/38) `9c6b393` 加载态消费及明确 goal，362 项通过；最终整合 `291b0f1` 默认 harness 443 项/严格 clippy 通过，保留此前 `d4e5208` 的实际 native 八项记录；明确 fake transport 的实际 CLI 仍为 SIMULATED_BLOCKED | 真实 prepare/collect/judge、三基线和五对独立确认；首个真实目标明确选择 `render_cadence`，功耗数值不作为该目标的合格指标 |

真实 native 编译证据保存在额外的 `real-host-build-summary.json`。历史 journal 的 `android_codegen_validated=false` / `real_ndk_validation=not_run` 未被改写；新增观察只证明工具确实生成 Android 工件，不把历史保守字段改成已部署。温度 preset 的提高也不能替代稳定性、放电、控制与恢复证据。

实验过程按实际检查点追加到 `oneplus_live_20261012/reporting/`。保留原 PNG、命令、失败记录、工具及源文件 hash；有实际起止时钟的截图绑定命令，旧截图仅有文件修改时间时明确标记采集时间未认证。截图在性能窗口外获取，能力自检、离线机制、真实性能三类结果分别解释。后续 PPT 使用该索引，只有实际有效采样才能生成性能收益图表。

## 剩余系统接口与完成条件

| 接口 | 当前缺口 | 验收所需实际证据 |
|---|---|---|
| game_opt_loop → phonefarm shader v2 | 已核对真实序列化的 fan/Inject；支持范围限于现有白盒载体，完整条件/采样仍待真机 | 同一请求被明确接受或明确拒绝；requested 字段保留，observed 未知不虚构；未知条件不能授予资格 |
| storage-image 算子 → 灰层 sampled-image ABI | #31/#34 已提供独立编译绑定及自动四阶段准备，#21 可实际适配；#35 接入 PF #24 六阶段 host 构建/验证与四/五输入合同 | 真实 NDK 布局与 Android 构建、Vulkan 验证、GPU 等价和工件身份贯通；有界 regular-only SDK 的 synthetic 正例不能替代这些证据 |
| 动态层 → 真实游戏试验 | #23 已修复 host 可验证的完整录制代/dispatch 所有权；一加挂点、参数、图像用途、布局/计算状态及命中仍未验证 | Android/Vulkan 验证、实际 GPU 完成、copy 等价基线、目标/非目标 pass 与具体管线合同、实际命中；已录制的注入不能通过关闭开关撤回 |
| refbench → 配对连续画面 | 原生 producer 和 raw→PNG 桥已发布；尚无 Android/GPU 实采及来源核验 | 真机构建和相同场景/相机/时间的独立渲染与读取，帧 ID/格式/色彩/提交完成与捕获账本，不把候选输出作为参考；单独进行性能运行 |
| 序列质量 → 资格/候选裁决 | CLI 的合成 PASS 不能自动授予真实来源资格 | 捕获来源核验、候选工件绑定、冻结阈值、坏帧证据；缺失或未知阻断晋级 |
| 模型工件 → 实际训练/量化/runtime | 五阶段协议完成不证明权重、模型格式或部署语义有效 | 可信数据/格式检查、真实 driver、预算计量、租约与恢复、实际 Android 推理和性能/质量报告 |
| 探索 → 独立确认 | #25 已发布 host 调度与 Android worker 命令源；三基线、十臂 fake 全链实际执行与重建，保持 SIMULATED_BLOCKED/BLOCKED；真机 worker 尚未执行 | 实际 cold/root/boot/完整快照/采样与恢复退出 receipts；远端 worker/子组持久身份与互斥，超时后先确认停止再恢复；预冻结顺序、探索隔离、每臂唯一工件和不重叠整窗、完整轮数与实际来源 |
| 档案 → 统一案例页面 | #32/#33 已展示完整评测、独立确认和模型阶段，浏览器与有界文件扫描机制已验证 | 仍需真实案例数据；保存声明、hash 匹配或页面可见不授予实验资格；原始来源/恢复/成本未知继续保留 |

上述接口的验收证据齐备后，才安排冻结预算的整链路候选与国内模型比较。接回一加时先执行 [只读资格和基线](oneplus-acceptance.md)，不从 root 成功直接跳到收益声明。
