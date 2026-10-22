# 作业执行与停止合同

2026-10-22，已发布 harness-ops main 225cb5c，尚未部署节点。此批解决 HTTP 服务重启后的进程所有权和取消回执，持久排队、资源调度与设备恢复屏障仍在 JOB 待办。

## 所有权

每个新作业由独立 Python worker 持有实际子进程及其进程组。HTTP 服务先写初始 metadata，再以独立 session 启动 worker。runner.lock 的 flock 从 launcher 经 pass_fds 交给 worker，父进程仅关闭描述符，实际命令不继承锁。服务重启不会成为另一个执行者；读取和取消均不向历史 PID 发信号。

launch.json 固定本次 command 和 cwd，环境只由进程继承，不把完整环境与凭据持久化。worker 是启动后 meta.json/exitcode 的唯一写入者。HTTP 进程只回收自己启动的 worker，不替它重写终态。

## 状态与证据

兼容 state=running|finished；新增 phase=starting|running|cancelling|finished|interrupted，终态 outcome=succeeded|failed|cancelled|interrupted。execution.kind=supervised-v1，worker/child PID 只供诊断，存活证据来自持有的 flock。启动有30秒窗口，避免初始记录发布到锁交接间的误判。

worker 消失且没有终态时返回 interrupted/exit=-1/lost，明确子进程是否停止未知；不自动重跑、不从旧PID推断资源已经释放。历史任务仍能读取 exitcode 和产物，但没有可验证 worker 所有权的在运行任务不支持信号取消。

## 停止操作

POST /jobs/:id/cancel，JSON {"force": false}：持久化柔性停止请求，由 worker 向自己持有的子进程组发一次 SIGINT。返回 accepted 仅代表请求已记录，不代表已停止或已还原。重复请求保留第一次时间。force=true 单调升级：SIGINT 后15秒仍运行则 SIGTERM，再15秒则 SIGKILL。原 /kill 路由保留升级语义，回执附带同一作业状态。

cancel 包含 mode=graceful|escalate、requested_at、status=requested|signaled|not_applied、signals（name/at）。任务已结束时为幂等 no-op；不支持旧作业时返回明确限制。真实 exitcode 与取消 outcome 分别记录：进程成功退出或收到信号不证明设备还原。

recovery.status 为 not_reported|required|not_needed：普通/柔性结束未上报恢复；强制 TERM/KILL 或失去 worker 要求检查；未执行就取消不需要本次恢复。此字段只是恢复证据状态，此批尚未实施设备恢复屏障。

## 网页与 CLI

只读 token 不能写。网页复用查看密码与掌舵 cookie，写入要 JSON + X-HG。GET /dash/api/access 只读宿主授权状态，不采手机。网页柔性停止与确认后的升级终止分开，显示请求、信号、终态和恢复证据，迟到响应不覆盖其他任务。CLI cancel 与 Ctrl-C 同一合同，未知退出码不能返回成功；取消后可继续 status/wait。

## 本批验收

真实无害子进程在 HTTP 服务进程重启后继续执行，新服务持久化取消并能读到真实终态；柔性停止、显式升级、启动失败与重复所有权有针对性测试。权限/只读/旧任务兼容、CLI 中断与网页反馈分别验证。全部是宿主模拟，不替代手机测量和恢复验收。

这不是整机断电持久性保证，也不保证脱离进程组的后代进程已退出；worker 丢失不自动补发信号或重放任务。后续设备 lease、恢复屏障和步骤收据必须补上，才能判断资源何时可再次写入。
