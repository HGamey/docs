# gray-ab loadop 的完整会话与停充恢复

状态：2026-10-23，PRODUCER-RECOVERY-03B3 实施前规格，主 Agent 已审核冻结。基于已发布的有限输入与 owned sampler/trace 生命周期；本批只迁移 `gray-ab --mode loadop`，不部署、不操作真机、不产生新的性能结论。

## 保留实验本身

保留现有 `--pairs`、`--serial`、`--out` 入口、默认有限 spin-touch 工作负载、场景进入检查和统计。每对顺序仍为 `ctrl1 → knob1 → ctrl2 → knob2 → …`，两臂均启用层，仅 loadop 开关不同；两臂 `passdump=0`。不得直接使用当前 eval layered restart 的 `passdump=1` 默认值，必须由本 owner 显式传递这项配置。

本路线不增加 eval 的资格轮、三基线、ABBA、PSNR 门槛或优化判决。原约 16.7/33.3 ms 与 SPF2 自检继续是原用途，不因迁移而升级成新资格证明。没有新增 fan、锁频或温控动作；原 `gray-ab` 未执行这些动作，不应由复用 eval Parent 隐式引入。

“整轮停充”明确指一个停充区间覆盖整个 loadop 采集 session：完成 prepare 和受控工具部署后、首个 arm 前确认放电，跨 ctrl/knob 及准备间隔保持，全部 child 结束后恢复充电控制，再恢复层和原属性。只有一条 charge owner/record，不逐 arm 反复停充恢复。每 arm 记录实际电池状态和 sampler 窗口。不得延续旧路径在采样约第 10 秒才调用独立停充线程的行为。旧 `power.json` 的 8 个、约 200 ms 间隔样本与本协议不同，旧、新结果不能直接混合汇总。

## 单一 owner 与可复用边界

整次 loadop 命令持有一个真实 Lease。新增明确的 `gray_ab_loadop` completion owner、`phonefarm/gray-ab-loadop-session@1` manifest 和对应恢复 scope；不能用 `gray_eval_script` 身份冒充，也不能由独立 frame-audit recover 完成这个父会话。

复用已有原始属性/文件快照、受控层安装与 restart、唯一远端目录和 trace instance、有限 Plan、三 child ledger/permit、自然输入收据、实际 uptime 窗口和同 context 恢复。原始属性与历史文件只在首次写入前保存一次，ctrl/knob 切换不得把当前实验状态覆盖成原始值。场景进入的 marker 轮询、计数读取与固定 tap 也经过 Session 受控 transport；固定坐标按原始截图实际尺寸校验，失败不猜坐标或回退 raw ADB。

不得保留 `sample_perf_under_lease` 的并行宿主线程或另一条 raw Device 采样通道。每 arm 继续由已登记的输入、trace、sampler 三个 child 执行；不拍 eval 资格图，`QUALIFY=0`。保留 lead=6 秒、原 trace 时长和 sampler lead/tail；当前默认有限输入仍需覆盖实际 trace 窗口。所有设备写均进入同一 pending/lease-command 生命周期，不能仅凭持有 Lease 就另开无登记命令。

## 充电状态的归属与证据

使用既有严格 `ChargeSuspend` 原节点保存、候选回滚与读回语义，不另造停充实现。充电控制记录仍由统一持久 state resolver 定位；写前解析路径。任何已存在而不能证明属于本 parent 的旧记录必须拒绝，不能因路径相同自动接管、覆盖或恢复。

父 manifest 在第一项充电写前登记本 session 的停充义务。若复用 `ChargeSuspend::apply`，需要在其独占创建并持久保存候选记录后、实际写节点前调用 before-write 绑定：保存 serial、session context、记录路径，以及记录中实际的原节点和原值。必须核验记录内容一致且属于当前 parent，不能只保存全局路径或只依靠 label。候选失败后，只有原值回写、读回和记录清除均明确成功，才能转下一个候选；父义务跟随已验证的候选状态，不遗失失败原件。

Session 的硬件 Shell adapter 区分可选节点缺席与执行不确定性。可在 apply 前以受控 presence 筛候选，或使用保留远端退出码的专用读包装；不能把 `cat` 可选节点不存在直接套入旧 `eval_transport` 的非零 sticky-stop，也不能把 transport、权限、身份/boot 不符等错误转换成“候选不存在”。写操作沿用写前 hold + durable pending，只有明确执行成功及要求的读回完成后才清除对应阶段；超时、断链或结果未知不能由下一次普通探测清 pending。

充电 obligation 至少区分未修改、已记录待写、已停充、待恢复、已读回恢复。进入采样前保存实际电池状态、选中控制节点、原值/实验值及确认时间；sampler 本身不负责改变充电控制。恢复必须向相同已绑定节点写回原值并读回一致，再按原 hwcond 语义清除持久记录、更新父 manifest。`Drop`、子命令 exit 0、record 文件消失或 Lease release 均不是充电恢复证明。

设备本来已经放电时，受控读取并保存 `observed_no_mutation` 条件，不调用会拒绝这一条件的 `ChargeSuspend::apply`，也不创建伪造的充电控制记录。各 arm 同样保存实际电池状态；不能把“观测到放电”标成“本 session 修改了充电控制”。只有本 session 实际修改且已绑定的控制需要恢复。状态未知或采样前不再满足放电条件时停止采集，不根据旧观测继续假定条件成立。

## 失败、pending 与 recover

任何 arm 的采集、停充或恢复失败均停止后续 arm，并先保存已取得的原始数据和负结果。采集与恢复结果分开：采集失败但全部已登记资源恢复成功，可以报告有限 scope 的恢复完成；采集成功但任一恢复义务未清，必须保留 Lease 恢复屏障并返回失败。

收尾按真实依赖推进：确认当前受控命令完成、已登记远端 groups 消失，再恢复本 parent 明确拥有的充电控制。输入已获 permit 却缺自然完成收据时，不能声称触点释放；即使 groups 已消失，也不能结束整个会话。但在 transport、身份和 group quiescence 均明确时，允许独立恢复已知充电控制并记录该项成功，避免因输入收据缺失无谓持续停充。此时父会话仍 required，后续 arm 不执行。若受控写仍 pending、组状态未知或身份不符，则不再写设备，保留充电记录与明确原因。

完成其它资源恢复仍复用既有规则：核当前 worker/permit，停并读回本 session trace instance、移除 owned instance，恢复原层/属性/历史文件，按既有政策保留已 closed 的远端工具目录。输入、游戏视角、角色/存档/场景始终不声称被还原；有限输入自然完成只证明本次受控输入正常结束。

新增 `gray-ab recover --session DIR --serial SERIAL --json`，只接受 `gray_ab_loadop` owner，校验原 session、serial、boot、远端目录和记录绑定，接续未完成的资源义务，不重新执行 arm。恢复报告独立落盘，不覆盖原 collection/失败报告。pending 不自动清空；旧记录属于其它 session、原值不符或恢复证据不足时明确 required，并保留精确原因和原件路径。

恢复 scope 为本会话实际拥有的有限输入生命周期、trace/sampler、层/属性/历史文件与充电控制；不声明整个 node job 或所有设备状态已恢复。正常闭环先持久写恢复报告，再给 Lease 提交同 owner 的 scope、证据来源和报告引用。发布恢复收据失败仍保留屏障。

## 输出兼容与新协议标记

保留 `--out` 下 `ctrlN`、`knobN` 的 arm 命名，`summary.json`、`power.json`、`env.txt`、`trace.txt`、`knobs_layer_out.json` 等原始产物用途，以及根目录唯一 collection 报告。会话 manifest/恢复报告放独立 session 目录并由 collection 显式引用；已有 arm 结果不得被静默覆盖。仍保留 `phonefarm/gray-ab-collection@1` 的 collection_ok、arms 与原有关键字段，新增 session/protocol 和独立恢复信息；未执行旧 deactivation 操作时不伪造其旧格式成功收据，改为引用真实受控切换/最终恢复证据。

协议标记为 `gray_ab_loadop_whole_round_charge_v1`。`power.json` 保留 PerfSnapshot 的顶层关键字段和单位，复用已有功耗汇总算法；样本来自本 arm 的 owned sampler。明确记录这是整个 sampler 的 lead+测量+tail 窗口，含实际开始/结束、有效样本数、缺失字段与来源，不能冒充旧 8 样本窗口，也不能把 trace 窗口与功耗窗口默认为相同。若提取 trace 同窗子集，另加显式子集字段，不替换全窗原件。未知指标保留未知，required 样本不足使 collection 失败。

充电条件使用父 manifest 的绑定、实际读回与时序证据；旧 env 头的 `charge_suspended` 仅由旧全局文件推断，不能拿它证明新 parent 控制或整窗停充。保留原 env 文本，但在新 collection/power 元数据中注明该字段不作为本协议依据。

保留 `game_state_restored:false`，不把局部恢复映射成 `job_recovery_complete:true`。新的 scoped recovery 状态/报告引用单独表达。命令退出统一为：采集与恢复均成功 0；采集失败但有限 scope 已恢复 1；恢复未完成、身份/pending 不确定或报告持久化失败 2。JSON 应能分别读取 collection 与 recovery，不用一个布尔值吞掉负结果。

## 实现拆分与有限验证

在现有结构上增加有限 owner 路线即可：base 的 `Session::new_loadop(...)`、`install_loadop()`、`restart_loadop(label, bool)` 显式固定 passdump=0；父层提供 `Parent::new_loadop(..., plan)`、`resume_loadop`、`prepare_charging()`、`charging_observation()`。复用现有 script sample/finish 和三 child worker，不新造 worker 或通用 producer 框架。父/base/charge 实现与 gray-ab CLI/采样证据共享分别推进，但本批须一起交付可执行和可恢复的完整闭环。

仅补能区分本批行为的宿主验证，不重跑已验收的有限 parser/三 child 全套。实际 CLI 限三次：一对 ctrl/knob 完整 collection 后在末尾注入充电恢复读回差异；同 context recover；不属于本 parent 的旧记录零写拒绝。第一条同时核顺序、两臂 passdump=0、首臂 permit 前已确认停充、两臂间不切换充电控制、最后所有 child 结束后才恢复充电且先于层/属性恢复，以及协议/全 sampler 窗口和既有字段；前两条核记录/屏障保留、续接成功且原失败报告不变。候选缺席可继续、权限/transport 错误必须停止和已放电不写控制用窄测试验证，不增加整套时序调用。所有假数据明确 synthetic，不能形成真机功耗/FPS结论。

## 下一 mode 边界

`gray-ab --mode copyprobe` 留给下一独立批次；它不是本 loadop 会话的别名，不得绕过新的 owner 合同。当前实现中仍可到达的旧 copyprobe 路线应明确标为未迁移，并保持其已有行为边界；本批不恢复、扩展或顺带接管 copyprobe/upop 注入。下一 mode 必须单独明确其真实写入资源和恢复义务，再决定复用范围。

## 已冻结的对外入口

采用 `gray-ab recover --session DIR --serial S [--json]`，只接本 loadop owner；0/1/2 按上述采集与恢复组合返回。loadop 支持 `--json` 和 `--evidence-origin device|synthetic`，copyprobe 不因新增选项改变测量能力。`power.json` 保留 PerfSnapshot 顶层字段，新增协议、采样窗口与父充电证据；功耗只用经过放电及完整窗口核验的电池样本。以上由主 Agent 按既有产品准则确认，无待人批准项。
