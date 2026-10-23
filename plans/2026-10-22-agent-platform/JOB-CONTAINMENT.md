# 作业执行范围与脱组后代

2026-10-23，JOB-CONTAINMENT-01分批推进。策略/API/Web/CLI与隔离Linux内核worker已发布；普通节点启用所需的特权入口、跨UID工件/控制与失联恢复仍未完成，完整条目保持开放。

## 当前事实

job_runtime只控制实际子进程及其原PGID。直接命令退出后，原组absent可以形成该范围完成证据；setsid脱组后代不在其中。JOB-FINISH已有回执与网页说明，不能把它扩写成整棵进程树结束。/node现已区分协议支持与实际可用策略，普通节点仍仅支持原进程组范围。Mac launchd同组清理与瞬时child PID枚举也不能提供持久脱组约束。

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

本机为Darwin；新增Linux内核后端已在隔离Ubuntu24.04 CI实跑，不能据此推断任何部署节点有可写委派域。Mac没有已验的对应实现时如实不可用；不能用ps轮询或launchd同组清理冒充。

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


## JOB-CONTAINMENT-01B：独立特权worker的内核范围（隔离内核验收完成）

当前非特权HTTP node既不能切换UID/GID，也不能读取另一UID写入的0600原件。不能把整个HTTP服务或Python解释器提权来绕过边界。因此本批只交付独立worker内可显式启用的Linux cgroup v2后端及隔离CI；所有普通node/API/Web/CLI的contained仍不可用，不新增对外可用能力。生产的固定特权启动边界、跨UID实时输出与PF控制会话单列后续事项。

worker从管理员提供的固定配置获得空的专用cgroup根及专用非root UID/GID，请求spec不能指定或改写这些值。worker处于域外；作业无补充组并设置no_new_privs，以清洁环境运行。配置中的身份不能同时用作登录、其他作业或服务账号；首次内核验收创建临时账号和专属域，不操作真实节点。普通同UID委派不是本合同的contained实现。

每次新建独占子域，目标在可信启动门等待；worker加入并确认实际成员身份后才允许目标执行。fork/exec/setsid后代留域；作业不可写域及祖先迁移入口。只依据cgroup.events populated确认宿主成员结束；主命令已退出而域仍活时worker继续持锁，force在原信号流程后使用cgroup.kill，未知保持恢复义务。域空不证明设备恢复；worker丢失、重启和重新取得域的生产恢复仍待后续。

保持管理记录0600、job根归worker且只有遍历权限；输入归worker只读，输出归作业可写，日志使用worker已打开的FD。既有私有祖先不自动chmod，实验配置必须选可遍历的隔离目录。原始输出不迁移。只复用现有worker/flock/持久收据，不添加数据库表、资源控制器、容器或daemon。

精简验证：保留普通节点contained拒绝回归；新增一个非矩阵Linux真实内核集成过程，覆盖输入/输出与身份权限、setsid脱组、迁出拒绝、直接命令退出后的保留、force后的域空及收据。必需接口/权限不可用时明确失败，不跳过后报通过。只清理本次账户和域，原始失败和诊断保留。


01B源码以管理员显式传入的contained_config连接独立worker，HTTP node没有提权路径。每作业目录通过SafeRoot固定，输入只读、输出属于专用身份，启动门不继承管理凭据；主命令退出后phase为collecting，worker仍持锁，force可对域kill。结束前恢复job根0700并同步，避免后续同UID作业读写旧证据；任何遗留子域（含空域）均阻断身份复用。门控进程回收超时进入worker-lost，保留遗留域，不伪造not_needed。后续生产恢复设计不在本批。

本机4项既有worker回归一次通过（1.834秒）。首次真实内核CI b232f55/run39884495270因fixture中tuple与list比较提前失败：原件显示UID/GID、空组、NoNewPrivs、零cap与7键环境正确，但剩余生命周期未验，不能算通过。临时域和账号均清理；保留完整raw和失败日志，修正fixture后再验证。CI使用Ubuntu24.04 VM，不等同任何部署节点。

实现核对了Linux的[cgroup v2成员继承、迁移权限与域结束接口](https://docs.kernel.org/admin-guide/cgroup-v2.html)和[no_new_privs继承语义](https://docs.kernel.org/userspace-api/no_new_privs.html)；实际可用仍以隔离内核结果为证据。


最终harness [487e5fb](https://github.com/HGamey/harness-ops/commit/487e5fb0076e8a78a7fd4aa34523768a8238264e)经code-only120190073130、protocol3.11/3.14 120190072430/120190072703、cgroup-kernel120190072687全部成功后同SHA直推main。内核run39884713585确认真实输入读取/日志/输出、身份/环境、管理记录及迁出拒绝、setsid后代在leader exit0后仍在域且worker持锁、force清空域、同UID不能再读取封存输出。4个已知PID全部退出，专属域/临时账号清理完毕；原件下载保存在本机qa/job-cgroup-kernel-attempt-2。首次失败原件保留在attempt-1，未以重跑删除或覆盖。

只勾01B这一隔离范围。普通node的contained仍false，生产特权启动、跨UID PF会话和worker失联/重启恢复属于JOB-CONTAINMENT-02；没有部署、手机访问、真实lease修改或原数据迁移。


## JOB-CONTAINMENT-02A：可信入口与有限作业结果（源码与隔离CI已验收）

源码核查确认run_node.sh仍是用户态macOS/Homebrew循环；update_node.sh将用户clone同步到~/node/harness-ops。没有Linux installer/systemd/sudoers可直接复用。node_session还需要以node身份读session-control.json、遍历原件并启动PF publisher；PF使用显式0600/0700。因此禁止把当前可自更新源码树直接sudo执行，也不能用umask声称实时权限已解决。

下一窄批只覆盖有限宿主作业：管理员独立安装root-owned可信worker入口、固定解释器/导入路径和配置，普通node更新不能覆盖。入口仅接受job ID和固定锁FD，实际job根、cgroup根、node/job身份由固定配置选择，请求不得注入其他root路径或解释器。sudo只能授权该入口，不能授权通用Python或shell。复用真实runner.lock的同一打开描述符，不另造owner；要实际核FD对应当前job且锁可用。未配置/权限不足继续明确不可用，不自动修改机器权限或sudo策略。

生产管理树归node身份，worker原子发布管理记录时也保持node可读写；job身份仍不能修改meta、launch、cancel和锁文件。配置/程序归root。输出运行中归job身份；域空且根封存后，限定在固定job/out内、不跟链接地交还可归档原件给node读取，再完成持久化。输出交还失败须保留资源并可恢复，不能把无法读取的结果标成已完成。stdout仍可经预打开FD实时读取；不扫描修改运行中producer文件权限。此模型必须先与SafeRoot和终态同步边界一起实现，而非只在命令前加sudo。

只用一条隔离Linux验收链证明非root launcher→固定特权入口→worker，launcher退出后runner锁仍存续，node可取消、读日志和最终原件，job不可写管理记录。可信安装只在隔离CI演练，不部署任何已有节点。public contained仍不可用；为未来启用补齐真实安装基础，不伪装现有节点已就绪。

02B随后处理PF实时会话/设备lease跨UID：优先明确会话输出树专用共享组，producer在创建和原子替换时主动设置组/模式，mailbox/锁同合同。管理记录、输入、凭据及cgroup不加入共享范围；旧job封存继续挡历史访问。任意未适配producer不能承诺实时原件访问。worker失联/重启后的域及输出恢复另有验证义务。

部署选择暂缓：实际Linux节点、管理员可信路径及升级方式、node/job身份、cgroup委派位置和sudo的FD保留策略都必须显式配置；源码阶段先在隔离CI选临时值。没有需要用户立即完成的操作，不阻断源码开发。


02A 已发布 harness [97a8946](https://github.com/HGamey/harness-ops/commit/97a894675b3804648b660d8bbfe66c7e3025426e)。`install_worker.py` 仅安装新的可信副本并打印未激活的 sudo 策略；固定入口在导入模块前核安装权限，只接收 job ID 和 FD 3。普通节点通过独立 child 传递原 runner 锁，不在多线程服务内改 FD。入口核 job、node 身份、锁文件及未启动状态；运行时再次要求 contained，避免 mutable launch 记录切换到 root 通用执行路径。已结束作业不能从此入口重放。

SafeRoot 的显式特权发布模式保持管理 JSON 属于 node；原子替换前设置 owner 并同步。目标输出运行中归 job，域空后先封存 job 根，再通过固定目录描述符将普通文件和目录交还 node；链接、特殊文件及外来 owner 拒绝。交还失败保留空域与 recovery.required，当前尚无自动恢复此状态的功能。普通输出/管理路径默认模式保持原行为，公共 contained 准入仍关闭。

本机 4 项 runtime 回归一次通过（1.700 秒）、13 项 artifact IO 回归一次通过（0.560 秒）。隔离 Ubuntu CI 的一条真实 nonroot→固定入口→worker 链验证：启动者退出后持锁；node 实时读取日志、写 force cancel、读取最终 0600 原件；job UID 不能再读封存后的原件和管理记录；错误 FD、非 contained 和终态重放均在执行前拒绝。17 个已知 PID 全部退出，临时账户、域、安装目录与 sudoers 片段全部清理。原 01B 内核链仍通过，不把它当作新增生产部署证据。

首次 CI 617a0f5/run39886934844 的测试安装位置选在 `/opt`，该 runner 祖先目录不满足可信权限，安装器按设计拒绝，未启动作业。保留原件及清理结果，只改 fixture 为根下新建的独占临时目录，未放宽产品校验或修改公共目录权限。最终 code-only120197554622（run39887050496）、protocol3.11/3.14 120197554157/120197554223 与 cgroup-kernel120197553890（run39887050372）全部成功后，同一 SHA 直推 main 并核远端。原始证据在本机 developer_transition_20261022/qa/job-privileged-*。

02A 只完成固定入口与有限作业的结果交还。02B 的跨 UID 实时 PF 会话、失联/重启恢复、普通节点能力开放与实际部署均未完成；没有手机访问、真实 lease 修改或数据迁移。


## JOB-CONTAINMENT-02B1：一条跨UID的PF实时控制链（下一批合同，未实现）

先完成固定入口→job UID的PF→同一持久lease域→scoped receipt→node UID查询/投递gray-control动作/读原件→正常停止封存这一条链。继续使用既有PF publisher、owner锁、动作ID和恢复schema；普通公共contained仍不启用。实际节点、手机、后台和历史数据不变。

已核实的阻断：当前worker清洁环境只有PF_JOB_ID/IN/OUT，缺少与ID成对要求的PF_JOB_LEASE_DIR；dispatcher又在启动前创建out/lease-receipts，与空out要求冲突。HOME=out还会使PF默认state根按job变化。node的LeaseState只继承服务环境调用PF lease status，没有现成的共享固定配置入口。PF的queue_owner稳定锁、gray-control内部记录/owner/publisher锁/请求目录都显式使用0600/0700；仅调整最外层目录或umask无法让实时publisher工作。

创建和配置职责：process_group保留现有receipt准备行为；contained保持out全空，由可信worker在验证身份与空目录后，经pinned out FD创建唯一lease-receipts目录，再注入PF_JOB_LEASE_DIR。持久PHONEFARM_STATE_DIR、DEVLOCK_DIR、PHONEFARM_LEASE_DIR由管理员配置；固定安装配置给node reader和worker提供相同的白名单PF环境，不接受请求自选根。实际位置继续由PF resolver及lease status.storage确认，Python不另造路径推导。不可读旧私有状态明确拒绝，不在线递归chmod已有状态。

权限只对声明的PF会话输出和持久lease访问显式启用；新文件与原子替换文件均在发布前主动设置组/模式，默认独立CLI保持原行为。node helper必须能读取session.json/eval-session.json，并打开现有owner/publisher锁与请求目录；node_session继续调用同一PF helper。管理JSON、输入、凭据、安装代码和cgroup不加入输出共享范围。结束仍先封存job根，后交还输出；复用job身份不能访问历史。

源码入口：harness的install_worker/worker_entry/job_cgroup/job_dispatch/node_lease/node_session；PF的小型显式权限策略及gray_control、devlease、queue_owner、confirmation_transport、lease_command和必要目录创建点/tools/devlock。不改方向调度、会话身份或结果schema，不新增账本或独立publisher。

验收采用一条隔离Linux实际PF与固定入口链，传输用fake：node读取活lease和scoped receipt，既有session helper完成capture、同ID查询、stop，读取实际PNG及回执，结束后job身份不能读旧输出；相关默认权限回归确认不启用共享模式仍按旧行为。此前01B/02A通过不替代这条PF实链。全局battery恢复记录、任意producer实时输出、失联/重启域及输出恢复均另有验收义务，不能因正常停止链通过而宣称完成。
