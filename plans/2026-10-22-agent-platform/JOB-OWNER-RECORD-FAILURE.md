# 活跃 worker 保存故障与停止意图

2026-10-25，JOB-OWNER-RECORD-FAILURE-01，源码与隔离宿主/CI范围已交付（harness764c4bb），未部署或操作真机。作为运行期磁盘软保护的前置修复，归JOB-06/09；不增加磁盘配额或新调度器。

现有worker在Popen/contained GO之后，写入身份、期限、取消和已发信号记录可能因ENOSPC/EDQUOT/EIO失败而退出，目标仍存活。原deadline也先写盘再请求停止。这批使已持flock、Popen/绑定domain的当前owner在记录故障时保留监督，按原信号链请求停止；HTTP仍不能按历史PID补发信号。

在现job_runtime中共享一个内存停止状态，普通组和cgroup循环复用。已有人工意图与期限意图合并，force只能增强，原信号列表和单调升级时钟不重置。RunBudget先记录内存触发事实，再由owner请求并尝试发布；不以写盘成功作为发信号前提。不新增monitor/线程/事件库，固定安装模块集合不变。

只有目标已放行后的管理记录I/O失败进入此降级。没有已有停止意图时锁存owner_record_error/escalate；已有graceful或escalate意图保持原模式。取消读取失败不抹掉已知意图；身份/触发/信号写入失败不丢掉内存信号历史。仅处理记录I/O异常与已定义的记录解析错误，不吞编程错误。已可信结束不补发迟到信号；域状态unknown仍保原中断/恢复边界，不猜PID或域身份。

最近成功保存的`execution.stop_intent`：schema=phonefarm/job-stop-intent@1，requested_at、mode、causes（execution_deadline/owner_record_error等本批固定原因）、record_errors（null或有限first_at/last_at/count/errno/message）。只记录意图与保存错误历史；实际信号仍仅在原cancel.signals中。元数据保存失败时该新对象也可能不可见，GET不能声称实时或已经持久化。保存失败后至多每秒重试一次小记录，不刷日志、不删产物腾空间。未发生本批原因的旧job不回填。

无既有意图时由owner_record_error发起的停止，即使真实exit0也以interrupted保留harness中断原因，CLI沿原interrupted/未知125规则。期限停止仍沿原124条件；真实退出码、宿主完成、设备恢复和容量资格独立。目标终结后的_finish、输出交还和fsync仍沿原机制；最终保存失败继续lost/storage_blocked，不伪造成功或自动释放设备。

新源码worker提供修复；固定安装需管理员安装新prefix才包含修复，不热更新旧prefix。公共contained准入不改变。网页和CLI状态展示最近保存的停止原因/记录错误及原信号，不把错误历史当当前磁盘剩余量。

最小验证复用现有期限/取消实际worker链，在活跃阶段注入meta/cancel读写失败，核原owner仍持锁、INT/TERM/KILL时序不重置、独立worker监督与原日志保留；永久最终保存失败不提交成功。原Linux固定worker链补同类降级与域空/输出归还断言。仅隔离宿主/CI，无实际填盘或设备操作；运行期log/space阈值在后续JOB-DISK-GUARD单列。

## 验证与发布

源码764c4bb61971a37a781dc143afcf9e4b78a9d225。唯一本地完整协议139项/75.733秒，138首过；人工先graceful再期限升级且记录I/O失败时，内存cancel缺少escalation_source。修为锁存升级原因再保存，仅原方法定向重跑1/1通过1.434秒。首次失败原件保留，不重跑本地全套。

实际worker注入signal-meta ENOSPC、cancel-read EDQUOT和期限intent发布失败：原flock/owner保持，INT/TERM/KILL各一次，升级间隔不被1秒保存重试重置；已有graceful不会因记录失败擅自升级。无既有请求的记录故障导致实际exit0仍interrupted；永久final-meta ENOSPC保留非终态rawmeta、真实exitcode0和worker非零退出，不冒称持久成功。5条故障执行共10个已知worker/child PID全退出，无fixture强杀，0设备。

实际390px synthetic HTTP页面4项有效检查通过，共2个browser context；首轮QA误等标题“执行中断”，实际优先正确显示资源保留，保留失败原件仅续余下3项。39GET/0POST/0device，pageerror/consoleerror为0，fixture和浏览器已退出，root目视人工取消截图。

五项actual CI（code-only、两Python、cgroup-kernel、pf-shared-kernel）均success，同SHA直推main并ls-remote核对。Linux原manualforce链注入活跃meta ENOSPC，核同worker/flock/域populated、实际SIGKILL唯一、causes=[]和原manual时间、域空及输出returned；保原固定安装/独立期限链。固定MODULES未改，旧prefix未热更新。证据索引为qa/job-owner-record-failure-{validation,checks,publish,ui-validation}.json。所有旧QA未知恢复义务保留，下一运行期软阈值独立见JOB-DISK-GUARD。
