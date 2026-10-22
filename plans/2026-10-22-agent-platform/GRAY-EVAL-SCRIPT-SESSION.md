# 有限触控脚本的 gray eval 会话

状态：2026-10-23，PRODUCER-RECOVERY-03B2C 已发布，源码 2fb0b16；仅宿主验证。03B2B已发布。复用 GRAY-EVAL-LAUNCH-SESSION 的真实Lease、恢复身份、资源登记、采集/恢复独立和冻结判据；不改阈值，不部署或跑手机。

## 输入与测量合同

灰档无workload.launch时走本有限路线；非法或未支持脚本在任何设备写入前拒绝，不回退旧execute_script。保留旧script命令供其原场景使用。本路线不调用onboard/HID，不部署共享pf_t1/pf_t2，不隐式停充或锁频。

- 输入接受动作数组或steps数组包装；有限loop/repeat按既有count/steps语义展开，最多10层、10000步。每一步字段类型/范围显式检查，不把错误当默认值。
- 支持sleep/wait（ms，或sec/s/seconds；缺省1000ms，单步0..60000ms），tap/click（显式非负整数x/y），swipe（显式x/y及to_x/to_y或x2/y2；duration_ms缺省350，范围1..60000）。总声明等待/拖动时长checked累加，最多600000ms；所有坐标必须位于本次设备原始截图实际宽高内，不自动缩放或猜横竖。
- 兼容仅一个精确旧形式：exec的cmd按空白分词必须恰好是input swipe X Y X2 Y2 DURATION，所有参数为十进制非负整数，随后变成同一typed swipe。拒绝其余exec/元字符/后台化和未知动作。保留默认spin_touch三次12000ms拖动与40秒总长，不改变旧script的语义。
- workload.seconds为1..594整数，trace lead固定6秒，sampler窗口为lead+seconds+4秒；声明输入时长必须覆盖lead+seconds，且最多600秒。每个子进程记录实际uptime/退出状态；计划窗口与实际窗口分开。启动开销不伪装成同步，实际trace必须落在输入已开始且正常结束的区间，不能满足时本轮无效。

## 所有权和执行

新gray_script_sample模式登记三个独立child：有限输入workload、trace、sampler；原gray_sample两个child合同保持。全部用已有permit/boot/PID开始时间/PGID监督，生成的输入脚本只有数值同步sleep/input，不另起后台进程。trace仍仅用本session独占实例；输入/trace/sampler原件与自然完成收据保留。每个新脚本child在自身已登记PGID内运行共享guard，按设备uptime独立截止（计划时长+30秒），不依赖主机ADB连接；超时终止本组且不产生输入自然成功收据。正常退出停止并回收guard，master核验整个组消失。旧gray_sample行为不变。

资格检查也走相同受控输入。可在资格轮由已登记trace child采输入前后两张PNG并留原件，不将该轮截图开销计入性能baseline。三基线及ABBA采样期间不额外截图；每臂原有比较截图在采样前进行，保存PNG并复用原PSNR/噪声底算法。loadop与storeop按现有请求施加，候选命中必须来自真实effective证据，不能用普通renderpass计数补足。

父会话新脚本身份与launch身份显式区分；旧launch会话仍可恢复，独立frame-audit recover均不能结束父会话。可共用资源结构，但恢复报告scope/schema必须对应真实owner，不允许改manifest身份绕过恢复。eval recover接续同serial/context；原失败报告不可覆盖。

输入完成必须有自然成功receipt与全部child当前消失证明。输入中断/失败且无自然完成证据时，不能以PGID消失声称触点释放：保持恢复待核、停止后续候选。已知从未启动输入的失败可以正常还原其它资源。游戏视角/存档/状态始终不在还原范围。

## 完整交付范围与验证

本批一起完成有限输入解析、资格/三基线/ABBA、同状态截图原件、fan/层/属性/历史文件/trace/受控进程恢复和eval recover。不扩展旧任意shell/HID能力；gray-ab及其额外停充义务仍是03B3/03B4。

使用少量有辨识力的parser/worker与实际CLI+fakeADB验证：默认12秒语义、危险/超限/越界输入拒绝、三child正常与输入中断缺receipt、完整流程恢复及同context续接。复用已通过launch与统计证据，仅重验共享改动影响。网页仍读既有job产物与恢复收据；没有新页面协议就不重复布局测试。模拟值不作性能/真机结论。

## 实现与当前验证进度

所有gray eval统一到owned评测循环，移除旧无父会话入口与未使用的readonly命中兜底；required_metrics必传。新script owner/schema/scope与旧launch分开，旧launch父manifest缺completion_owner仍可识别；frame-audit不能结束这两种父会话。

有限输入由数值计划生成专用模板。共享guard在已登记child PGID内提供独立deadline；同组watchdog没有新owner。trace先完成配置及资格首帧，再启动sampler/input，避免准备时间消耗尾窗；原sample_env在子shell显式保留其非errexit语义，可选传感器缺失仍按原未知字段处理。trace-time的start/end对应实际采集区间，child_start/end另记生命周期；自然输入时段必须覆盖采集窗。

恢复先由worker核当前账本与PGID。未产生输入permit的准备失败无需输入自然收据；已permit的输入即使组已消失，也必须自然完成才可结束会话。目标场景始终不声称已还原。每轮采样前复用PNG实际宽高校验坐标，比较图尺寸改变明确拒绝；保留原PSNR/噪声底判据。

统一构建/签名与10项相关unit通过。3项唯一worker行为共执行5次：真实输入模板替换只重跑受影响项；trace资格测试发现POSIX awk三元表达式括号问题，修复后仅重跑该项，原失败日志保留。4次实际CLI一次通过，共1868次lease门控传输、410次base写前pending和41次父pending核验。完整资格+3baseline+6对ABBA共16采样及18对比PNG后注入fan恢复差异，正确保屏障；同context恢复成功且旧失败报告不变；输入中断无自然收据保屏障；非法exec零transport拒绝。全部模拟进程已退出，仅输入中断的隔离fixture屏障按预期保留。

最终审读保留旧launch collection schema的raw image_check，并在质量判定报错前保存负结果。此增量仅影响launch collection，不改变已验收script路径；最终重新构建签名，追加一次既有live-check单测，未重复完整CLI。源码经code-only check120059560881 completed/success（run39846075471）后同SHA直推main并核对远端。未部署或验证真机能力；假数据不得用作帧率、功耗或Android恢复结论。
