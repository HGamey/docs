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
| 条件与设备独占 | [phonefarm #19](https://github.com/HGamey/phonefarm/pull/19) | shader v2 条件解析、请求/实际分离、载体参数回读；本机设备写入口自动租约与异常退出恢复门禁 | 正在核对真实 game_opt_loop 请求格式；无端到端放电窗口或真实游戏收益资格 |
| 独立确认 | [game_opt_loop #29](https://github.com/HGamey/game_opt_loop/pull/29) | 事先注册轮数/顺序/收益及非劣效界；逐臂单轮绑定、原始工件去重、单侧置信上界裁决 | phonefarm 尚需实现单轮确认 driver；独立性、采集来源和注册时间尚无认证；不改前沿或授予游戏优化证明 |

动态 upop 工件构建和模型五阶段账本正在独立 worktree 中实现与复核；代码尚未发布为 PR 时，不计为可审阅交付。真实训练器、量化器和 Android runtime driver 不属于本轮假驱动验证已经完成的内容。

## 集成验证如何解释

本轮各功能从同一远端基线建立独立分支，再 cherry-pick 到本地集成分支，检查功能同时存在时的实际 CLI 行为。测试数是对应提交的回归总数，不能相加成更多独立实验证据，也不能换算为真实游戏效果。

- game_opt_loop 集成分支 `codex/handoff-game-integration` 在 `76a9f18` 上共 **359 项测试通过**，严格 clippy 通过。包含 #26–#29、实际 campaign/证据兼容测试：资格不足不进入前沿、原始扩展与专家决定不丢失、恢复零重复评测。单轮独立确认仍由夹具供数。
- phonefarm 集成分支 `codex/handoff-phonefarm-integration` 在合并序列质量与资格审查后共 **540 项测试通过**；加入 #19 的集成复核正在进行。验证对象包含实际离线 CLI 的 synthetic 来源阻断、资格回放 ABORT、正常序列与异常序列判定。
- 测试日志与原始 JSON 保存在本机 `phonefarm/tasks/system_handoff_20261011/` 及各 worktree 的 `tasks/`。集成分支目前仅本地存在；这些记录不代表 GitHub CI 已通过。
- 本轮无真机部署、真实训练或模型 API 调用；**0 模型 API 调用**。内部代码开发与评审不作为被测模型的实验成本。

## 剩余系统接口与完成条件

| 接口 | 当前缺口 | 验收所需实际证据 |
|---|---|---|
| game_opt_loop → phonefarm shader v2 | 两端必须按真实序列化请求核对必填字段与注入对象，不能各自测试自造格式 | 同一请求被明确接受或明确拒绝；requested 字段保留，observed 未知不虚构 |
| storage-image 算子 → 灰层 sampled-image ABI | 当前生成器和 upop 层接口不同；固定候选不能代替任意演化候选 | 有限支持的转换或专用生成路线；编译、SPIR-V 反射、算法等价验证、候选/层 hash 与命中身份贯通 |
| 动态层 → 真实游戏试验 | 一加挂点、参数、图像用途及命中尚未验证；不能外推红魔路径 | copy 等价基线、目标/非目标 pass 约束、实际 dispatch/submit 命中、失败时停止与还原 |
| refbench → 配对连续画面 | 现有 LR 与上采样输出不是独立原生 HR；缺同步 native LR/HR 导出 | 相同场景/相机/时间的独立渲染与读取，帧 ID/格式/色彩/提交完成与捕获账本，不把候选输出作为参考 |
| 序列质量 → 资格/候选裁决 | CLI 的合成 PASS 不能自动授予真实来源资格 | 捕获来源核验、候选工件绑定、冻结阈值、坏帧证据；缺失或未知阻断晋级 |
| 模型工件 → 实际训练/量化/runtime | 五阶段协议完成不证明权重、模型格式或部署语义有效 | 可信数据/格式检查、真实 driver、预算计量、租约与恢复、实际 Android 推理和性能/质量报告 |
| 探索 → 独立确认 | 聚合 eval 报告不是预分配单轮样本，尚缺采样 driver | 预冻结顺序、探索隔离、每臂唯一 trace/ENV/报告、完整轮数、实际条件与来源、注册后独立运行 |

上述接口的验收证据齐备后，才安排冻结预算的整链路候选与国内模型比较。接回一加时先执行 [只读资格和基线](oneplus-acceptance.md)，不从 root 成功直接跳到收益声明。
