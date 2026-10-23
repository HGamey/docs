# 作业结束记录的持久化边界

2026-10-23，JOB-DURABILITY-01源码与隔离宿主验收完成，harness 8381c19已推main。只做源码与隔离宿主故障注入，不部署、真实断电或操作手机。

## 已确认的问题

SafeRoot.write_json只close+rename，worker先exitcode后meta。dispatcher仅看meta.finished就提交SQL done，甚至可能在worker仍持flock时放行。简单给rename后加目录fsync仍不够：新meta已可见而fsync尚未完成或失败时，HTTP/dispatcher能提前宣告完成。

准备阶段的job/in/out、输入与prepared也需明确同步完成才queued。publish_json_once遇已存在发布返回False不能证明原目录项此前已同步。当前SQLite事务已synchronous=FULL，复用它作为资源状态的提交边界，不新增终态镜像或完成标记。

## 结构与次序

1. SafeRoot提供通用持久写及同步屏障：文件flush/fsync、原子replace、父目录fsync，目录创建同步父目录。调用方给关键文件/目录集合，基础层不按任务文件名猜语义，不新增hash。失败要允许“目标已可见但持久化未确认”的结果。
2. dispatcher私有读取候选与HTTP公开status分开。取得已释放的runner flock后读取稳定终态候选，完成固定关键记录与job/祖先目录同步，再提交SQL done或既有恢复规则需要的blocked。HTTP在queue仍claimed时将terminal meta呈现为finalizing/running，不公开成功；新supervised作业的读状态不写文件。
3. 单job同步失败用同一SQLite事务改blocked，并存最小state_reason={kind:durability,stage,message,at}。保资源但释放claimed名额，其他方向继续。SQLite自身无法提交时保持原reservation、报告节点存储错误，不能声称blocked已持久化。
4. 显式retry-finalize只取得该job的flock、读取原候选、重新同步和提交SQL；不启动worker、不重放命令。重试成功仍按原设备恢复义务决定done/blocked。普通queue-release不能绕过durability block；保存阻断时新recovery_for需先重试保存，活跃恢复子任务仍遵循既有互斥关系。release说明若写后同步失败，只留候选，不展示为已放行收据；retry不自动重做管理员放行。
5. 输入与准备目录同步后才queued；输入树同步拒绝链接/特殊文件，不跟随外部路径；人工release说明持久化后才SQL done。cancel_pending保持与claim竞争的原子事务，在同一SQL事务记cancelled_before_launch事实；公开status从这一事实派生取消，不以文件缺席推断。meta只作兼容输出，写失败不否认数据库已经确认的未执行取消。

作业列表以SQL与磁盘ID并集读取，早期缺目录/meta的准备失败仍可发现和重试。

公开queue可选state_reason只表达提交失败或未执行取消依据；旧行无该字段保持兼容，不复制整份终态metadata。旧任务和历史结果不能由本次升级补造持久化证明。

## Web、CLI/API与未执行事实

能力名jobs.durability-v1；管理权限POST /jobs/:id/retry-finalize及网页同义/dash/api/jobs/:id/retry-finalize，pf-node retry-finalize ID。空JSON请求，回包{storage_committed:bool,job:status}；true只表示结果的SQL提交完成，设备恢复仍可能blocked。claimed的终态候选公开为finalizing/running；durability阻断公开为storage_blocked/running，保留原命令诊断字段，禁止取消和普通解除资源保留。Web显示保存失败原因与“重试保存结果”；CLI等待遇保存阻断明确非零退出，旧节点能力缺失明确说明。

准备或已确认worker未创建的启动失败，先在SQL事务中记blocked与最小unstarted事实{outcome,note,at}，再构造failed/not_started诊断候选并同步。这是调用点对未执行的事实记录，不以文件缺失判断；重试只可据这份事实重建同一诊断候选，不重放命令。若取消已在SQL获胜，保留cancelled_before_launch与done，不被随后准备错误覆盖。新取消完全从SQL事实派生；旧无字段行保留既有兼容行为，不补造新证明。

已取消未执行、运行结束、worker丢失、存储失败、实际设备恢复required分别有独立依据。只读候选/已提交结果的分界不得造成dispatcher调用公开status后永远等自己的循环。

## 最小验证

聚焦rename已可见后目录fsync失败、失败job保资源且无关job运行、存储恢复后显式retry-finalize不重放三条故障链；准备目录与取消SQL事实扩既有合同用例。Web定向检查和API同字段，精简测试，不重跑灰层或Doom。原失败日志保留。

本合同仅覆盖指定job执行记录和输入元数据的持久化顺序。任意producer工件内容、真实断电、文件系统/存储硬件兑现同步语义、setsid脱组与强lease跨启动身份另列验收；不能以错误注入通过关闭完整JOB-EXEC-03。


## 2026-10-23 验收记录

harness 8381c19aa86816f20021f00e6457b2d344cfea75经过实际code-only、protocol (3.11)/(3.14)全部completed/success后同SHA直推main并核远端。本地规定协议集61项通过（17.514秒）；包含真实独立worker的rename可见后fsync故障、同资源保留/无关作业继续、显式重试计数仍1、SQL缺meta取消/失败事实和HTTP权限。SafeRoot三项定向回归与两条host故障链先各一次通过，未在其后额外重复旧实验链。原始worker异常、日志及断言收据存本机qa/job-durability-*。

网页定向DOM首次通过：exit0不遮盖保存中/失败、权限/旧能力/忙态、单次重试及迟到响应隔离、保存成功仍保留恢复义务。CLI缺日志时先读状态；保存阻断返回125，明确未执行的终态不依赖不存在的日志。未重跑浏览器或操作真实设备；未部署，实际断电与任意producer数据持久性仍不在本声明中。
