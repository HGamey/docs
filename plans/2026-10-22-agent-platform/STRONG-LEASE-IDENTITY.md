# 强 lease 的宿主进程身份

2026-10-23，LEASE-IDENTITY-02源码与隔离宿主验收完成。PF 201719b、harness 66a532e；仅此范围完成，不代表部署、真实重启或设备恢复已验收。

## 具体问题与复用范围

queue_owner已经能核对宿主boot和进程start；强devlease marker仍只有pid/child_pid/child_pgid。acquire_recovery与recover_confirmed_idle据数字PID和PGID探测存活，宿主重启后号码复用可能误挡旧上下文的显式恢复。现有代码只探测，不按历史PID发终止信号，本批不引入这种操作。

把queue_owner已有macOS/Linux进程/boot读取实现抽到共享host_process模块，queue owner沿用原schema、祖先校验与行为。共享接口必须能捕获{pid,start,host_boot_id}并区分同实例存活、previous boot、已退出/zombie/PID复用、读取失败；读取失败不能当死亡。直接子进程已退出但尚未reap时仍允许捕获身份，不能把短命命令变成启动失败。不另造身份算法或hash。

最小源码范围是共享模块/main注册、queue_owner、devlease、observation。bench/capture/gray/confirmation已共用Lease::acquire_recovery；这些实验上下文里的boot_id是设备boot，不能当宿主boot或批量换schema。

## Marker与恢复顺序

- 强marker增加成组host_boot_id、pid_start；存在child_pid/child_pgid时再要求child_start。新owner/child放行前持久化其真实实例。Lease::check及通用/observation worker验证身份不可变和与当前实际进程匹配。
- 恢复接管保留完整previous_owner，写入新owner身份并删除旧child身份。设备上下文、receipt、外层queue实例和既有锁顺序继续保留，不降低既有恢复要求。
- 宿主boot明确不同，旧宿主owner/child/PGID已消失；不让当前重用的数字阻挡原上下文显式恢复。这个事实不证明设备状态、远端动作或恢复已经完成。
- 同boot下leader退出或PID复用只证明旧leader消失；原PGID仍在或探测失败必须保留屏障，不能由leader身份变化推断全部后代结束。setsid脱组仍属独立containment待办。
- command_pending、无法确认的设备写入和recovery_required保持原有阻断/显式恢复规则。宿主boot变化不得绕过这些条件。
- 旧记录身份字段全缺时保持保守旧逻辑，不用当前复用PID回填历史身份；部分缺失/损坏/身份读取失败为unknown。明确previous boot/PID复用可以报告abandoned，但仍保留恢复屏障。已有pending/recovery显示优先级保持。

网页/API仅需呈现与旧记录兼容的宿主身份事实、观察原因和范围；若现有通用状态已足够，不为了这批重做界面。不能把旧数据升级成已验证身份。

## 验证与未完成边界

精简回归覆盖身份变化、同boot残留组拒绝、previous boot仅进入显式恢复、旧/不完整字段兼容，以及短命child。复用已有queue identity和Lease合同用例，只跑受影响范围及仓库规定检查；原失败保留。隔离fixture的不同boot字段证明分支行为，不能写成真实主机重启或实际手机恢复已经验收。完整JOB-EXEC-03与总goal保持开放。


## 实现、发布与验证

新marker的host_boot_id/pid_start成组保存，child_pid/child_pgid存在时要求child_start；Lease.check、Drop和两种worker验证实例。恢复保留previous_owner并写新身份，清旧child字段。只读状态新增host_process对象（status、pid、start、host_boot_id、reason），七种status为alive/previous_boot/exited/zombie/pid_reused/legacy/unknown；Jobs与Fleet呈现同一事实，既有整体状态和恢复动作门禁不变。损坏marker仍直接unknown，不从损坏内容推导身份。

本机11项devlease用例中10项首次通过，真实未reap短命child首次失败：macOS原PROC_PIDTBSDINFO调用arg=0不查询zombie。依据[Apple XNU实现](https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/kern/proc_info.c)，改为arg=1纳入未回收进程，同一用例复测通过；未引入第二种身份算法。另1项queue身份和3项命令监督测试通过。失败日志保留，没有盲重跑整个实验链。

3组实际宿主CLI一次通过：22条PF命令、23个已知PID全部退出、0设备/传输调用。覆盖真实owner/child、短命命令、模拟前次boot但数字仍活、同boot PID复用且原PGID仍活、旧记录保守恢复、不完整身份拒绝。pending、required、partial三份负例marker原样保留；前批B1/B2负例未触碰。定向DOM一次通过七种显示、缺字段/legacy区分、文本转义与既有门禁；未重复浏览器整链。

证据位于本机developer_transition_20261022/qa/strong-lease-identity-*，原始宿主目录strong-lease-identity-host-cli-20261023T095823557908。PF code-only 120179340708（run39881389533），harness code-only 120179343980（run39881390580）、protocol3.11/3.14 120179343947/120179344082（run39881390532）均completed/success后才同SHA推main，无PR。

macOS本机验收不等于Linux运行验证；伪造fixture boot字段不等于真实主机重启；owner实例结束不等于脱组后代、远端活动或手机恢复。JOB-CONTAINMENT-01、完整JOB-EXEC-03、端到端流程与整体goal保持开放。
