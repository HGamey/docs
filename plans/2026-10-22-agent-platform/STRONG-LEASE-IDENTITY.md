# 强 lease 的宿主进程身份

2026-10-23，LEASE-IDENTITY-02下一批合同。只读审计完成，尚未实现、构建或测试。本阶段继续源码与隔离宿主；不部署或操作真实设备/lease根。

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
