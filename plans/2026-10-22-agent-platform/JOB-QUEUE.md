# 提交幂等与持久队列

2026-10-22，JOB-EXEC-02 已发布于 harness-ops 58fc0dc，接续 JOB-EXECUTION。此批完成节点宿主队列；phonefarm 的设备 lease 仍必须由实际命令持有，调度声明不代替设备资格或实验还原。验证与限制见 BATCHES。

## 提交与找回

POST /jobs 的 X-Request-ID 为1到128位字母、数字、点、下划线、冒号或短横线，首位字母或数字。新CLI/网页自动生成，网络重试保持同一ID、参数与上传内容。SQLite唯一约束持久保存request→job映射，一次规范化spec加上传字节的内容指纹仅用于重复提交判定；同ID同内容返回原job，不同内容409。没提供ID的旧客户端仍可提交，但跨请求不具备幂等保证。

GET /requests/:id 只读找回原job，不存在404；不得用不确定结果生成新ID重跑手机写入。CLI在发送前报告ID，网页不确定提交保留ID并提供查询。刷新后只保留ID、不保存参数；可以重新填写此请求，提交仍用原ID，内容冲突返回409并查询原任务。CLI先核对jobs.idempotency-v1能力，旧节点不能默默多次POST。上传内容在客户端以固定gzip时间戳打包，避免同一输入仅因封装时间变化产生冲突。

## 存储与调度

作业目录仍保存输入、日志、metadata、原件及独立worker收据。目录内.queue.sqlite3保存request、规范化spec、资源声明、提交顺序与preparing/queued/claimed/blocked/done状态。SQLite写事务处理唯一登记、原子领取、原子取消；同数据根只有持有dispatcher flock的服务执行调度。intake flock覆盖登记及输入准备，准备进程消失后将未完成输入记为失败，不执行半份输入。

输入准备完成才进入queued。领取后先写starting记录，再启动上一批worker。launch.json的只发布一次标记存在时绝不再次执行：有活worker就接续读取；没worker且无终态则interrupted，不盲重试。标记尚未产生的claimed项可以继续启动，因为命令尚未发出。客户端断线和HTTP服务重启不删除队列。

resources为独占字符串数组。未声明资源的phonefarm/GOL/adb/fetch_apk保守使用legacy-device，其余默认无独占资源；显式声明支持不同设备/宿主资源分开排队。调度按提交顺序寻找第一个满足容量与资源条件的任务，无关任务可越过忙的队首。max-running默认2，可由节点参数修改。claimed与blocked均占容量；blocked保留声明资源。当前仍可能受旧global devlock串行限制，不能将宿主队列并行写成物理设备lease并行已完成。

## 取消、阻断和释放

准备/等待中的cancel在事务内标为done，命令不执行，outcome=cancelled、recovery=not_needed、exit=null。已领取项沿用独立worker取消通道。worker丢失或终态recovery.required转blocked；不从exit0判断恢复完成。POST /jobs/:id/queue-release 必须管理/掌舵权限，提供非空note，保存核查说明后允许再次调度；这是管理员对队列资源可复用的确认，不自动执行设备恢复，也不将其包装成平台已验证还原。

## 同级界面

GET /queue 与 /dash/api/queue提供同一容量、非终态队列、资源、等待原因和阻塞job列表。job.status保留state=running|finished，新增phase=preparing|queued；queue记录state、提交时间、资源、等待原因，request_id可找回。blocked不覆盖实际执行终态。网页提供队列视图、节点已有命令提交、查询不确定请求、撤销等待及显式解除资源保留；CLI/API具备相同核心操作。输入包/游戏分发页面属于DIST后续。

## 验收

并发重复提交只执行一次；同ID不同内容409。真实HTTP服务进程重启后，运行任务不重复、排队任务仍能依次执行；等待取消不执行。资源冲突等待且无关任务能推进。网页/CLI展示同一原因和真实phase，权限及不确定提交反馈可核查。全部先用无害宿主子进程，实际手机与强lease联动另行验收。
