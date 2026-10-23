# 作业执行范围与脱组后代

2026-10-23，JOB-CONTAINMENT-01分两批推进。第一批显式策略与能力边界已实现，源码/隔离宿主验证完成并发布；第二批内核后端尚未实现。完整条目不由协议批次勾完。

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


## 第一批已实现合同与验收

/node.execution.process_policies为按策略名索引的对象，capabilities另声明jobs.execution-policy-v1。每种策略含available、backend、completion_scope、reason；当前process_group可用，后端posix_process_group，完成范围original_process_group；contained不可用，不因Linux名称推断支持。

显式请求在X-Job.execution_policy传字符串，缺失不加入原规范化spec，因此旧幂等指纹保持不变。元数据以execution_policy和execution_policy_source分别记录请求与explicit/legacy_default来源；缺省请求与新显式请求是不同的冻结内容，同ID改字段仍409。直接API在读取上传body前拒绝不可用策略（422，code=execution_policy_unavailable），Dashboard共用Jobs.start，CLI在pack之前核能力。排队启动与独立worker再按冻结请求校验，未执行失败沿既有持久终态逻辑处理。

worker仅在实际Popen成功后写execution.process_management={policy,backend,completion_scope,detached_descendants}；当前detached_descendants为unverified。已有owned_process_group继续提供完成事实。旧记录不补造这个对象，取消前未启动/启动失败也没有实际后端证明。网页新旧节点、未决请求原body重试与CLI同语义；无新数据库或调度状态。

63项本地协议测试一次通过（17.856秒），定向DOM一次通过。宿主QA三组通过，共11次HTTP、2个实际job、6个已知PID退出、零设备调用：不发送声明的4096字节body即可收到contained拒绝，零job/prepare/spawn；缺省与显式策略、同ID重试/冲突与只读拒绝；实际setsid子进程在父命令exit0、原组absent后仍活，记录如实为unverified，随后自然退出。

第一次宿主脚本过严地拒绝lease的只读快照，导致已创建legacy job的状态HTTP断开；保留原result/log/清理补充。只修fixture，在同数据根按原request ID找回旧job，不重放；剩余explicit job才新增并完成现场脱组验证。该故障不归为产品失败，也不隐去首次结果。证据保留在developer_transition_20261022/qa/job-execution-policy-*。没有部署、设备访问、真实cgroup后端或任意进程树完成证明。


第一批harness [fb4dc56](https://github.com/HGamey/harness-ops/commit/fb4dc56375252f175732619c11d959a3f1dfbb52)包含功能提交5468e0f与文件末尾空行修正。最终SHA通过code-only120184129840（run39882870707）、protocol3.11/3.14 120184130255/120184129669（run39882870737）后直推main。空白修正没有重跑本地全协议；最终SHA的必需CI仍重新通过。无PR、force push或部署。


## JOB-CONTAINMENT-01B：独立特权worker的内核范围（实施合同，未实现）

当前非特权HTTP node既不能切换UID/GID，也不能读取另一UID写入的0600原件。不能把整个HTTP服务或Python解释器提权来绕过边界。因此本批只交付独立worker内可显式启用的Linux cgroup v2后端及隔离CI；所有普通node/API/Web/CLI的contained仍不可用，不新增对外可用能力。生产的固定特权启动边界、跨UID实时输出与PF控制会话单列后续事项。

worker从管理员提供的固定配置获得空的专用cgroup根及专用非root UID/GID，请求spec不能指定或改写这些值。worker处于域外；作业无补充组并设置no_new_privs，以清洁环境运行。配置中的身份不能同时用作登录、其他作业或服务账号；首次内核验收创建临时账号和专属域，不操作真实节点。普通同UID委派不是本合同的contained实现。

每次新建独占子域，目标在可信启动门等待；worker加入并确认实际成员身份后才允许目标执行。fork/exec/setsid后代留域；作业不可写域及祖先迁移入口。只依据cgroup.events populated确认宿主成员结束；主命令已退出而域仍活时worker继续持锁，force在原信号流程后使用cgroup.kill，未知保持恢复义务。域空不证明设备恢复；worker丢失、重启和重新取得域的生产恢复仍待后续。

保持管理记录0600、job根归worker且只有遍历权限；输入归worker只读，输出归作业可写，日志使用worker已打开的FD。既有私有祖先不自动chmod，实验配置必须选可遍历的隔离目录。原始输出不迁移。只复用现有worker/flock/持久收据，不添加数据库表、资源控制器、容器或daemon。

精简验证：保留普通节点contained拒绝回归；新增一个非矩阵Linux真实内核集成过程，覆盖输入/输出与身份权限、setsid脱组、迁出拒绝、直接命令退出后的保留、force后的域空及收据。必需接口/权限不可用时明确失败，不跳过后报通过。只清理本次账户和域，原始失败和诊断保留。
