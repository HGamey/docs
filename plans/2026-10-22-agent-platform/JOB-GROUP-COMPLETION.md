# 作业原进程组的结束证据

2026-10-23，JOB-FINISH-01已完成源码、宿主与Web/CLI验证并发布main。仅源码与隔离宿主验证，不部署或接设备。

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

## 交付记录

harness [eb3e69d](https://github.com/HGamey/harness-ops/commit/eb3e69dcf407b86b9b08308cd8efc469d85f3794)经实际code-only120165766193（run39877260415）、protocol3.11/3.14 120165766110/120165766509（run39877260474）completed/success后同SHA推main并核远端。

本机5个既有worker/CLI测试一次通过；新增1条实际Jobs/worker回归一次通过，分别覆盖host-none与targeted recovery的同组后代、原命令exit0、资源阻断和无关任务继续，8个已知PID全退出。网页任务详情定向DOM一次通过；整合发现维护页按exit0显示完成，追加公开字段投影和维护页DOM各一次，未重跑任务页或浏览器。CLI保留真实非零命令退出码，exit0但组未结束返回125。CI运行完整协议回归的两Python版本。

本机证据在qa/job-group-{existing-tests,ui-validation,source-review,publish}.json、job-group-host-66569-1792714523811573000.json及原始日志。初始CLI测试AST括号错误在执行前修正并记入source-review，不计验证。无PF/ADB/真实设备、部署或旧实验链重跑。JOB-FINISH-01仅关闭本声明范围，JOB-EXEC-03仍未完成。
