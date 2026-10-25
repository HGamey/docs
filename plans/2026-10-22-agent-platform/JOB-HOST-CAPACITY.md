# JOB-HOST-CAPACITY-01：宿主调度容量

2026-10-25，已发布 harness `1dd4e68501f4e4b7a4604a8a6959f2ee53daefb3`；属于 JOB-03，不是操作系统硬限额或手机 GPU 配额。

## 资源声明与配置

可选 job `host_slots` 是封闭三键对象 `{cpu:N,gpu:N,io:N}`。cpu 为1–65535整数，gpu/io为0–65535整数；bool/float/null/未知键非法。缺省不改原canonical/指纹，调度按固定兼容需求 `{cpu:1,gpu:0,io:0}`，公开 `source:legacy_default`。0表示没有预留，不能说程序不使用该资源。cap为 `jobs.host-capacity-v1`。typed及普通device_access=recovery本批不接受显式host_slots，内部准备/恢复使用固定兼容需求，不继承父计算需求；旧typed精确spec不变。

节点管理员启动参数 `--host-capacity FILE`，文件为 `{schema:"phonefarm/host-capacity@1",slots:{cpu:N,gpu:N,io:N}}`，同上范围。配置保存到原队列SQLite的单条scheduler配置；未传文件且首次初始化时使用cpu=max_running、gpu=io=0；未传且已有配置时保留原配置。显式文件在启动时更新配置（相同值不更新），所有新版本dispatch在同一claim事务读取当前配置，避免同根服务各持一份旧总量。公开source（initial_default/file）与updated_at、slots，不探硬件、不在线提供配置写API。旧版本调度器不得与新版共用数据根并行运行，按现有停服务升级规则部署；本批不部署。

CLI `--host-slots FILE`读取闭合三键小JSON（最多4KiB）；Web可选三整数，缺省整体省略。cap/静态格式在pack或读upload body前核对。首次请求超过总量返回422 `host_capacity_exceeded`（requested/total/shortages），零prepare/零job；已有ID仍先按原请求查回，读body后核原指纹，不受新容量缩小影响。reserve前在原intake临界区再次核容量，最终INSERT事务也检查当前总量，旧ID匹配在检查之前。

## 持久占位与宿主收尾

原SQL原子claim保留max_running、独占resources、设备scope/FIFO/lease约束，额外按三维容量累计claimed/blocked且未释放的需求。恢复只能借祖先设备/独占资源，不能借祖先计算slots。资源不足时继续扫描可行候选，不加优先级/抢占。

SQLite jobs增加nullable `host_capacity_release`事实（{at,basis}）；未释放时为null，只有受控null→事实，查询不改写。claimed/blocked的null均占位。queued/preparing不占位；done不得绕过未知host释放；旧done不伪造宿主证明。旧claimed/blocked迁移保留null，按固定legacy_default计数。

已有worker结束收据和runner flock为依据：not_started的持久SQL事实；或supervised、worker_false、finished/nonlost、原process_group absent；或既有contained completion+private output handback资格。只证明原受管范围，setsid后代仍未验证。把Node会话恢复里现有普通组判断提成共享只读资格，与容量释放共用；不读旧PID猜结束。

finalize同步meta/关键记录后，以同一SQL事务提交state及容量释放事实。设备recovery.required/not_reported仍可blocked，但已确认宿主结束的slots可释放。保存失败/worker lost/组域unknown或present不释放。只有未启动的持久SQL事实可独立证明无执行，不能凭缺文件推断。

显式execution reconcile后域空且handoff returned时，调度可仅同步/提交host_capacity_release，保持原lost/outcome/queue blocked/设备义务。dispatcher在已有tick中对未释放blocked核宿主资格，不在GET改历史；无queued时启动或reconcile完成后安排一次tick/现有dispatcher继续。无新监控服务。retry-finalize可补提交同一释放事实。queue-release对所有未释放宿主占位要求同一资格，不能凭note/lease clear把未知host变done。没有忽略未知按钮。

## 重启与可读状态

重启不重置占位。容量缩小到reserved以上显示overcommitted和available=0，不杀旧worker；已queued请求超过当前总量保持原声明，waiting说明request_exceeds_current_total，改回管理员配置后可继续。原ID重试不创建新job或改需求。max_running仍只限制claimed作业数，blocked计算保留与设备恢复保留独立。

`/node.execution.host_capacity`返回配置（schema/slots/source/updated_at/basis:declared_scheduler_slots）；`/queue.host_capacity`加reserved、available、overcommitted（逐维超额数量）。`job.queue.host_capacity`返回 `slots`、`source:explicit|legacy_default`、`state:not_claimed|reserved|released|historical_done`、`release`。旧done且无释放事实用historical_done，不冒充已核宿主。

`waiting.kind=capacity`，reason为max_running、slots_unavailable或request_exceeds_current_total；slots等待携shortages（resource/requested/total/reserved/available）及相关占位blocking_jobs。保现resources/device_lease先行解释顺序。Web/CLI消费同一事实，不用局部列表重算容量；原pending显式/省略/旧未知三态冻结，与budget/device_binding并列，不自动推需求。

## 验收

一次本地协议；聚焦SQL原子多维claim、超量零prepare、同ID冲突/配置缩放、未知host保留/已结束host但设备blocked可释放、durability不提前释放。复用既有实际宿主/contained reconcile链，不做CPU压力/大文件/真机。单390原ID冻结及容量等待/宿主释放与设备屏障分离；CLI一个集中case。首次失败保留，仅修受影响链。JOB-09磁盘/研究预算及生产部署仍待后续。


## 发布证据

一次本地136协议72.847s：134项首次通过；旧device scope fixture使用CPU2而声明claim3，以及旧HTTP fixture允许失联host凭lease clear释放，两项修正后定向2/2通过0.594s。保留首次失败日志，没有重复本地整套。真实host组残留与fsync故障链证明未知占位、无关任务结束归还及retry后归还，所记录PID全退出；未知/设备恢复结论不因进程退出改写。

单390px浏览器4步首过，41HTTP=40GET+1合成截断提交，0cancel/release/recovery/device；服务/浏览器退出。CLI新case进入统一协议。最终同SHA五项actual required CI全部成功，包括Linux真实contained失联reconcile：普通核查保留slots，force核空并handback后仅归还host，原lost/blocked仍保留；两Python协议和PF shared/recovery链通过。源码同SHA直推main并ls-remote核对；未部署或运行真实手机。

本地索引 `tasks/developer_transition_20261022/qa/job-host-capacity-{validation,checks,publish,ui-validation}.json`；[Linux CI](https://github.com/HGamey/harness-ops/actions/runs/40148901983)。父JOB03仍有全producer/真机与独立磁盘预算边界，JOB04状态revision继续实施。
