# 活跃 worker 保存故障与停止意图

2026-10-25，JOB-OWNER-RECORD-FAILURE-01，下一批合同，尚未实现。作为运行期磁盘软保护的前置修复，归JOB-06/09；不增加磁盘配额或新调度器。

现有worker在Popen/contained GO之后，写入身份、期限、取消和已发信号记录可能因ENOSPC/EDQUOT/EIO失败而退出，目标仍存活。原deadline也先写盘再请求停止。这批使已持flock、Popen/绑定domain的当前owner在记录故障时保留监督，按原信号链请求停止；HTTP仍不能按历史PID补发信号。

在现job_runtime中共享一个内存停止状态，普通组和cgroup循环复用。已有人工意图与期限意图合并，force只能增强，原信号列表和单调升级时钟不重置。RunBudget先记录内存触发事实，再由owner请求并尝试发布；不以写盘成功作为发信号前提。不新增monitor/线程/事件库，固定安装模块集合不变。

只有目标已放行后的管理记录I/O失败进入此降级。没有已有停止意图时锁存owner_record_error/escalate；已有graceful或escalate意图保持原模式。取消读取失败不抹掉已知意图；身份/触发/信号写入失败不丢掉内存信号历史。仅处理记录I/O异常与已定义的记录解析错误，不吞编程错误。已可信结束不补发迟到信号；域状态unknown仍保原中断/恢复边界，不猜PID或域身份。

最近成功保存的`execution.stop_intent`：schema=phonefarm/job-stop-intent@1，requested_at、mode、causes（execution_deadline/owner_record_error等本批固定原因）、record_errors（null或有限first_at/last_at/count/errno/message）。只记录意图与保存错误历史；实际信号仍仅在原cancel.signals中。元数据保存失败时该新对象也可能不可见，GET不能声称实时或已经持久化。保存失败后至多每秒重试一次小记录，不刷日志、不删产物腾空间。未发生本批原因的旧job不回填。

无既有意图时由owner_record_error发起的停止，即使真实exit0也以interrupted保留harness中断原因，CLI沿原interrupted/未知125规则。期限停止仍沿原124条件；真实退出码、宿主完成、设备恢复和容量资格独立。目标终结后的_finish、输出交还和fsync仍沿原机制；最终保存失败继续lost/storage_blocked，不伪造成功或自动释放设备。

新源码worker提供修复；固定安装需管理员安装新prefix才包含修复，不热更新旧prefix。公共contained准入不改变。网页和CLI状态展示最近保存的停止原因/记录错误及原信号，不把错误历史当当前磁盘剩余量。

最小验证复用现有期限/取消实际worker链，在活跃阶段注入meta/cancel读写失败，核原owner仍持锁、INT/TERM/KILL时序不重置、独立worker监督与原日志保留；永久最终保存失败不提交成功。原Linux固定worker链补同类降级与域空/输出归还断言。仅隔离宿主/CI，无实际填盘或设备操作；运行期log/space阈值在后续JOB-DISK-GUARD单列。
