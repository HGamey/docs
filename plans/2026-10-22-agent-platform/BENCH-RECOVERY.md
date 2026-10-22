# bench 执行与恢复范围

2026-10-23，PRODUCER-RECOVERY-01宿主验证完成，发布状态见BATCHES。接续HWCOND-RECOVERY和JOB-RECOVERY；仅宿主/模拟传输验证，不部署或操作手机。

## 所有权

bench在任何部署、旧锁频恢复或新锁频之前持久写独立session，然后hold实际强lease。每个ADB命令使用同一个lease，通过本地子进程门控先记录PID/PGID再放行；命令退出码、输出、超时及组残留均明确记录。本地连接结束不等同远端负载停止。不复制/反复hash ADB二进制。

每次bench使用独立远端目录与token/boot绑定，避免覆盖其他运行的模型、runner或恢复文件。默认本地session位于持久state根；显式out下也建立独立session，保留命令原件、恢复证据和报告引用。已有模型内容hash继续用于实验身份，不新增无用hash链。

远端复用confirmation worker/bootstrap的先记录后放行和进程组回收，增加bench模式；不触碰ftrace或其他confirmation配置。每轮运行workload和sampler，二者均有界；结束后必须确认所有已记录组消失，才允许host恢复频率或开始下一轮。root可用时worker及其负载以root运行并在报告注明UID范围；这与旧bench的shell UID负载不同，不直接混用旧基线。无root的no-lock继续使用shell UID，不采root频率。

## 恢复证据

正常结束先永久关闭远端session、只读复核关闭与组消失，再确认hwcond状态已经恢复、输入文件按keep策略保留或删除，持久写`phonefarm/bench-recovery@1`后调用complete_recovery。scope只涵盖本session的频率控制、受控进程、临时输入清理；不证明整台设备或整个job无其他影响。单个round的锁频回读不能解除整个session屏障。

不能确认恢复的失败保留有原因的证据和屏障；能够明确完成清理的实验失败可留下失败实验报告与成功恢复证据，两者不混同。未处理的旧损坏/未知状态也保留，不能因为没启动新负载便假设原设备正常。

成功恢复原件先记`verified`，只有lease完成收据发布成功后，CLI报告才记`reported_complete`；保存原件不提前等同解除屏障。无部署的旧锁频恢复只声明`frequency_controls_only`。局部命令退出码保留在journal；hwcond可选节点/无root探测仍按输出和严格恢复回读判定，门控、超时、残留子进程等生命周期错误单独传播。

硬件写入与只读探测通过`Shell::shell_write`区分。在每次硬件写入前持久设置`control_command_pending=true`，仅明确成功结束后清除。超时/非零/崩溃保持该事实，后续探测、写入与恢复完成均拒绝，包括重新启动`--recover`；不能靠新一次回读抹掉可能仍在执行的远端su。当前这类未绑定到远端worker的控制命令需要独立核验，不自动续接。其它hwcond消费者的默认Shell行为不变，须按todo逐个接入。

新增`bench --recover <session目录> --serial S`，只允许接管同serial、同context的已结束旧owner，沿用acquire_recovery核查本地进程及组。它绕过普通排队入口但绝不跳过强恢复资格。读取已有session后核对设备boot/token，关闭/核查该session，恢复原值并写新的恢复证据；原失败证据保持不变。`--unlock`保留旧锁频记录兼容用途，只能声明frequency_controls_only，不替代新session恢复。

远端关闭中崩溃留下无法证明归属/结束的worker.lock、未知pending或逃离组的后代，明确保留屏障；不按超时猜空闲。真实设备能力和断电验收仍待做。

所有install_pending均拒绝自动完成恢复，避免未知push或部署命令稍后继续；当前需独立核对session，不能凭部分文件已存在自动清屏障。默认仅删除模型/runner输入；session关闭记录、控制脚本和进程账本保留用于后续核验。现有通用恢复lease的显示label仍为`confirmation-reconcile`，实际恢复范围以typed scope和session为准，后续统一命名另行处理。

## 接口与验证

CLI JSON增加recovery/session引用；node现有job lease回执API/网页展示scope、origin及evidence，同一事实来源。不从bench exit0自动清job调度保留。

验证集中在正常session恢复、恢复失败阻断新写、子进程超时/死亡、恢复接管上下文不匹配及成功恢复链。复用现有实际CLI/fake ADB、远端shell fixture和job收据测试，不重复整个phonefarm测试集。

本批通过43项相关Rust单测（bench5、hwcond28、本地命令2、receipt1、devlock7）、3项远端shell宿主fixture和9项已签名实际CLI用例。CLI共93次fake ADB，均核对PID/PGID/serial/pending，写操作均已有hold。正常rootless/no-lock完成；恢复失败后普通任务、错serial/context均零ADB；同context恢复新增成功证据并保留旧失败原件；旧有损serial别名错误不panic且保留屏障。硬件锁频命令真实超时后没有后续transport/恢复写，snapshot与pending保留；再次recover仍exit2且零transport。

shell测试运行实际宿主进程组，但Linux /proc身份和kill错误文本由fixture提供；CLI的远端worker回应/频率均为模拟。它们不证明Android内核、真实断电、真实功耗或性能。没有node协议/页面布局改动，本批沿已有job回执接口展示scope/origin/evidence，未重复网页布局验收。QA自身目录重名导致首轮中断，修正后通过，原日志保留；无root兼容和旧记录错误分支借用冲突均在发布前修正。
