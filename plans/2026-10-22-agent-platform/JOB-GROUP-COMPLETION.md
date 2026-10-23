# 作业原进程组的结束证据

2026-10-23，JOB-FINISH-01实施合同，尚未实现。仅源码与隔离宿主验证，不部署或接设备。

## 触发问题

node worker目前直接子命令退出就发布finished。若它留下同一进程组的后代，host-none或recovery任务可能被调度标done并让出资源。退出码必须保留，但不能充当整个原进程组已结束的证据。

## 窄合同

继续由现有worker持有Popen与runner.lock。仅在直接子进程被回收后，用killpg(pgid,0)有界核查原进程组；最多等待0.5秒自然收尾，不发送额外信号、不遍历PPID、不从历史记录恢复PGID控制权。ESRCH记absent，仍存在记present，权限/系统错误记unknown。

execution增加可选owned_process_group：scope=original_process_group、status=not_started|pending|absent|present|unknown、pgid、checked_at、error。未启动时not_started，已运行尚未收尾pending；时间与PGID只作诊断。旧记录缺字段为未提供，不能补造absent。

present/unknown时保留直接子exit和execution.returncode，phase/outcome=interrupted，recovery.status=required，附进程组原因。既有dispatcher据required保留声明资源，其他资源任务仍能继续；不新增调度状态或自动放行。absent不代表设备恢复，沿原有not_reported/required规则。

Web任务详情与pf-node status消费同一记录。wait/follow遇命令exit0但进程组未证实结束时返回125，明确保留命令退出码但作业完成未确认；已有非零退出码仍保留。网页明确原进程组范围、事实状态与时间，旧记录未提供，不因exit0显示作业成功。

## 验证及未覆盖范围

一条实际隔离宿主链覆盖主命令exit0而同组后代仍活、资源保持blocked、无关资源继续；正常/取消/未启动复用已有worker用例，CLI已有退出码用例追加新记录。网页定向渲染检查即可，不重跑旧完整浏览器旅程。全部后代由fixture自然退出并核PID，不触旧负租约。

setsid脱组后代不在此证据范围内，absent不能描述成整个进程树终止。内核containment/跨OS能力合同另列JOB-CONTAINMENT-01。关键metadata与目录fsync另列JOB-DURABILITY-01，不混成同组修复；强lease的宿主boot/start身份另列LEASE-IDENTITY-02。真实断电/宿主重启/两节点与手机验收继续待办，完整JOB-EXEC-03保持未完成。
