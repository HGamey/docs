# 作业执行范围与脱组后代

2026-10-23，JOB-CONTAINMENT-01实施前合同。只读审计已完成，以下尚未实现。先交协议与能力边界，再实现有真实内核验收的后端；完整条目不能由协议批次勾完。

## 当前事实

job_runtime只控制实际子进程及其原PGID。直接命令退出后，原组absent可以形成该范围完成证据；setsid脱组后代不在其中。JOB-FINISH已有回执与网页说明，不能把它扩写成整棵进程树结束。node身份目前列协议能力，没有执行保证的运行时声明。Mac launchd同组清理与瞬时child PID枚举也不能提供持久脱组约束。

## 第一批：可选择、可审计的执行保证

复用现有job spec/幂等指纹、metadata、队列、worker、Web和CLI；不增加调度状态、数据库表、常驻代理或另一份进程所有权账本。

- 请求增加execution_policy，值为process_group或contained。缺失继续已有process_group行为，记录来源legacy_default；显式输入来源explicit。策略进入冻结spec和现有幂等比较，重复请求不能悄悄改策略。
- /node区分协议支持与实际执行范围，当前后端明确original_process_group。contained当前未实现，返回明确不可用原因；不能根据操作系统名称或存在/sys/fs/cgroup目录宣布支持。
- contained要求在接收输入、准备作业或启动命令前拒绝；直接API、CLI、网页使用相同准入。请求不支持的保证不能静默降级。已入队spec在执行前仍须满足其冻结策略，旧任务缺字段继续原行为。
- job的请求策略、实际后端与完成范围分别记录。实际执行信息由worker写入，不能把请求字符串当执行证据；旧record没有这些字段就显示未提供，不回填整树终止证明。
- Web/CLI同批提供策略选择和不可用原因；原PGID回执仅表示原组状态，setsid后代未验证，设备恢复是另一义务。无需为本批重做布局。
- 保持现有取消、恢复、持久化顺序：原组未知/仍活仍沿现有required/blocked保资源，无关方向继续。没有新能力时不会为已有普通host任务强加不可实现的内核保证。

最小源码范围：node_service校验、job_dispatch初始metadata、job_runtime执行事实、node_read能力、pf-node及现有Web表单/详情。先读实际代码再确定最小字段位置，避免同一含义重复状态。

## 第二批：Linux后端及真实内核验收

后续显式配置delegated cgroup v2根，核权限与必需接口；不自动修改系统服务或委派。每job独占域，child在启动门控内进入后才执行，worker保留域外。用cgroup.events的populated状态确认域内进程结束；未知/仍有成员保资源。强制结束只在已确认支持时对所拥有域使用cgroup.kill，不能凭历史PID恢复信号权限。

必须先冻结权限保证：普通fork/exec/setsid成员留域，不等于同UID或特权任务不能主动迁出。可写域本身不足以宣称防逃逸；如使用contained的强保证，需明确实际权限隔离及不支持条件，不能只给有限后端换名称。worker丢失、主机重启后的域核查和设备恢复仍分别验证。

当前只有Darwin隔离宿主验收环境，不推断Linux可写委派域。Mac没有已验的对应实现时如实不可用；不能用ps轮询或launchd同组清理冒充。

## 精简验收与交付边界

第一批：一个实际setsid的有界宿主负例说明有限范围，contained不可用时零准备/启动，策略幂等冲突与旧请求兼容，Web/CLI同字段反馈。测试仅使用自建无害进程并回收，不碰设备、真实lease或系统cgroup。节点协议变更按AGENTS运行必需协议测试。

第二批另验真实Linux内核：setsid仍留域、取消和域空、权限/查询失败保资源。仅filesystem fixture或本机模拟不能勾这批。整体JOB-EXEC-03、两节点与真机流程、总goal继续开放。
