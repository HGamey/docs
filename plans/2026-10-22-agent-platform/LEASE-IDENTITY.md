# 外层队列的实例身份

2026-10-23，LEASE-IDENTITY-01实施合同，尚未实现。只做源码与隔离宿主验证；不部署、不操作真机或真实租约目录。

## 问题与范围

普通devlock的renew/release及PF/GOL借锁只凭label，nested强Lease也凭同名owner决定目录。不同任务同名可被误认同一owner，状态则因外层没有typed身份而unknown。本批统一既有队列owner身份，不新建任务调度器。label只展示与排队，不授予任何持锁资格。

## 共享实现与接口

PF新增小型queue_owner模块作为唯一typed身份实现；隐藏入口`phonefarm __queue-owner`仅操作宿主元数据，shell仍管已有队列/qmutex，删除其Python进程身份副本。操作create/check/renew/release/reconcile以JSON回执；create成功回传schema与instance_id。PF/GOL直接acquire后保存该实例，经DEVLOCK_INSTANCE_ID传给子命令，续期/释放显式带同一实例。已有继承实例校验失败就拒绝，不换实例重试；没有实例不能靠同label借用。工具或协议不兼容明确失败，不回退旧label逻辑。

typed marker放既有devlock.d/phonefarm-lease.json，schema=phonefarm/devlock-owner@1；记录instance_id、label、实际持续holder_pid、holder_start、host_boot_id、精确owner文本。旧参与者已将该文件作为不可偷取屏障。create只发布在既有排队取得的目录，原子持久写后才能执行子命令。helper核实际祖先关系，不相信任意传入PID；进程实例/启动身份及host启动身份由同一模块读取。变更在共享短临界区内再次比较实例，不能按TTL接管typed或未知记录。

PF gate的holder是监督父进程，GOL是Guard所在进程，devlock run是持续wrapper；直接acquire的holder是调用shell。acquire成功stdout是typed JSON，诊断走stderr；run向子命令传ID并保留原退出码。正常release只删除本实例外层元数据，不删除nested或其恢复屏障。

## nested与恢复

新强marker可带严格可选outer_instance_id，旧marker无该字段仍按旧强恢复合同读取。创建nested必须核活外层身份、继承实例和调用祖先；其他目录检查仅允许本次核验的queue marker，foreign/unknown仍阻断。强Lease持久保存不可变outer引用，后续核自身marker未被替换；不以外层holder仍存活作为有界收尾/恢复的前提。父进程退出后不允许新借用，但已有强owner可继续恢复。

recovery_dir按强marker类型选择，不把queue JSON当作强恢复上下文；原outer引用随接续记录保留。普通外层退役提供窄入口`devlock reconcile INSTANCE --participants-stopped --confirm-device-idle`：须核原holder已退出或进程实例变化、无nested/强记录、bridge可独占，再比同实例并先写审计归档。只退休queue元数据，不生成设备恢复证明；损坏/旧无实例记录不自动升级或按超时删除。

## 状态与网页

lease status维持既有schema_version/status，entries补role=outer_queue|device_lease、instance_id、outer_instance_id及holder核验事实。只有完整匹配且活的新外层显示active，旧/坏/身份未核保持unknown，已知pending/recovery/abandoned仍优先blocked，只有原clear门槛能放新任务。状态读取不写文件或授予权限。

node沿现有LeaseState与fleet lock.lease透传，不再实现宿主身份判断；任务/节点页展示真实关联，label仅显示。maint/home不得用job或goal label==held_by认定持锁任务，也不得将任意busy描述成正在测量。无新HTTP路由，不新增浏览器恢复捷径。

## 分工与最小验证

platform独占PF queue_owner.rs、tools/devlock及其定向unit/脚本fixture适配；root主系统整合：PF devlock/devlease/main、GOL Guard、AGENTS、Git/Cargo/docs；frontend独占harness相关网页与服务显示标签（不改调度准入）；project只准备三组实际宿主验证，冻结签名后一次执行。模块接口先同步再并行实现。

验证聚焦正常PF/GOL/run传实例与nested关联；同label/旧实例/进程身份变化拒绝；父退出后强owner可收尾与孤立queue显式退役且不能绕过nested。复用受影响既有fixture，避免全实验链回归。所有失败和未知保持原件，B1/B2隔离负租约不动。
