# 作业结束记录的持久化边界

2026-10-23，JOB-DURABILITY-01下一批设计，尚未实现。只做源码与隔离宿主故障注入，不部署、真实断电或操作手机。

## 已确认的问题

SafeRoot.write_json只close+rename，worker先exitcode后meta。dispatcher仅看meta.finished就提交SQL done，甚至可能在worker仍持flock时放行。简单给rename后加目录fsync仍不够：新meta已可见而fsync尚未完成或失败时，HTTP/dispatcher能提前宣告完成。

准备阶段的job/in/out、输入与prepared也需明确同步完成才queued。publish_json_once遇已存在发布返回False不能证明原目录项此前已同步。当前SQLite事务已synchronous=FULL，复用它作为资源状态的提交边界，不新增终态镜像或完成标记。

## 结构与次序

1. SafeRoot提供通用持久写及同步屏障：文件flush/fsync、原子replace、父目录fsync，目录创建同步父目录。调用方给关键文件/目录集合，基础层不按任务文件名猜语义，不新增hash。失败要允许“目标已可见但持久化未确认”的结果。
2. dispatcher私有读取候选与HTTP公开status分开。取得已释放的runner flock后读取稳定终态候选，完成固定关键记录与job/祖先目录同步，再提交SQL done或既有恢复规则需要的blocked。HTTP在queue仍claimed时将terminal meta呈现为finalizing/running，不公开成功；读状态不写文件。
3. 单job同步失败用同一SQLite事务改blocked，并存最小state_reason={kind:durability,stage,message,at}。保资源但释放claimed名额，其他方向继续。SQLite自身无法提交时保持原reservation、报告节点存储错误，不能声称blocked已持久化。
4. 显式retry-finalize只取得该job的flock、读取原候选、重新同步和提交SQL；不启动worker、不重放命令。重试成功仍按原设备恢复义务决定done/blocked。普通queue-release不能绕过durability block；活跃恢复子任务仍遵循既有互斥关系。
5. 输入与准备目录同步后才queued；人工release说明持久化后才SQL done。cancel_pending保持与claim竞争的原子事务，在同一SQL事务记cancelled_before_launch事实；公开status从这一事实派生取消，不以文件缺席推断。meta只作兼容输出，写失败不否认数据库已经确认的未执行取消。

公开queue可选state_reason只表达提交失败或未执行取消依据；旧行无该字段保持兼容，不复制整份终态metadata。旧任务和历史结果不能由本次升级补造持久化证明。

## Web、CLI/API与实现前需收束的细节

拟新增管理权限POST /jobs/:id/retry-finalize及dashboard同义路由，pf-node retry-finalize ID；返回是否已提交存储状态和同一job，不称为设备恢复成功。网页显示finalizing、保存失败原因及重试入口；权限/能力缺失明确，原解除资源保留入口在durability错误时禁用。精确capability/回包字段在实现前一并固定。

准备尚未生成可读候选就发生存储失败时，必须依持久化的未执行事实处理，不能把缺文件当未执行，也不能在重试中执行原命令；需要先决定是从已记录的失败意图重建诊断候选，还是明确留待人工核验。不得为了让重试返回成功而捏造执行结果。

已取消未执行、运行结束、worker丢失、存储失败、实际设备恢复required分别有独立依据。只读候选/已提交结果的分界不得造成dispatcher调用公开status后永远等自己的循环。

## 最小验证

聚焦rename已可见后目录fsync失败、失败job保资源且无关job运行、存储恢复后显式retry-finalize不重放三条故障链；准备目录与取消SQL事实扩既有合同用例。Web定向检查和API同字段，精简测试，不重跑灰层或Doom。原失败日志保留。

本合同仅覆盖指定job执行记录和输入元数据的持久化顺序。任意producer工件内容、真实断电、文件系统/存储硬件兑现同步语义、setsid脱组与强lease跨启动身份另列验收；不能以错误注入通过关闭完整JOB-EXEC-03。
