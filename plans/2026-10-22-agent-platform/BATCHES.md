# 开发批次与验收记录

记录具体交付范围，不把单项修复写成整个平台完成。原始实验数据继续留在本机，运行节点未替换；GitHub main 的现有构建工作流可能发布构建产物，发布产物不等于已部署到手机节点。

## 2026-10-22：通用任务发现、结果读取与节点身份

已推送 harness-ops main：

- [2aec080](https://github.com/HGamey/harness-ops/commit/2aec080)：`/node`、通用 `/jobs` 分页/检索、任务/日志/输出目录与下载的只读合同；`pf-node` 和 `/dash/jobs` 共用。任意命令、旧任务与失败项都可发现，原件留原位置。稳定节点ID、执行宿主、服务版本和数据根独立于入口域名。
- [2b3c741](https://github.com/HGamey/harness-ops/commit/2b3c741)：复用本机既有增强。显示 label 与 job 唯一锁 owner 分离；raw adb/fetch_apk 走现有锁桥；内部 lease helper 不被重复套锁；devlock 管理请求保留跨请求身份且不虚构 job owner。
- [2363c39](https://github.com/HGamey/harness-ops/commit/2363c39)、[94d3d93](https://github.com/HGamey/harness-ops/commit/94d3d93)：验证分支支持直接提交流程，检查全部尚未进入 main 的文件，而不是只看最后一个增量。

任务页按列表与详情工作区组织，节点路径/参数按需展开；明确读取时间、执行失败/未知、按钮忙碌与重试。没有在浏览任务时启动手机采样。下载读取集中在目录 fd/文件 fd 这一层，避免检查路径后重新打开时被链接替换；保留已有 Range 和传输校验。移除了没有必要的逐源码文件哈希和重复身份测试。

验证：本机协议/读取/所有权回归通过；Linux Python 3.11 与3.14 CI通过。JS语法和两项关键异步行为（旧响应不覆盖新选择、刷新失败可重试）通过。浏览器模拟节点实走检索、分页、详情、目录、日志与原件读取，在390/768/1440宽度及浅深色检查；修复390宽度继承公共样式导致操作栏错位。截图保存在本机transition/qa/jobs-*.jpg，页面数据明确为模拟验收。

CI回执：[code-only](https://github.com/HGamey/harness-ops/actions/runs/39775467557)、[node-tests](https://github.com/HGamey/harness-ops/actions/runs/39775455462)。首次新建验证分支曾因before为全零而失败，94d3d93改为相对main检查，保留失败回执而非称首次全绿。

范围：这是通用文件/任务发现，还没有语义结果注册、跨节点索引、持久队列、重启取消、人工协助或接管。旧global devlock桥的串行与忙时行为未改变。没有实际手机或生产节点验收。

## 2026-10-22：看屏生命周期与旧PR资格回归

已推送 phonefarm main：

- [591889e](https://github.com/HGamey/phonefarm/commit/591889e)：每个观看窗口独立订阅，暂停/隐藏/关闭释放；无人观看不构建设备对象、查询尺寸或截图；缓存旧帧不算新帧/FPS。失联订阅5秒过期，已开始的截图可能完成。`--from-start`等尺寸就绪后才消费动作。
- [8dba439](https://github.com/HGamey/phonefarm/commit/8dba439)：修复gray候选采集前先清空control、再要求control通过的循环依赖。只在灰层采集阶段允许尚待采证的control unknown；证据仍unknown，结论仍不合格。明确失败、恢复失败和普通确认路径不放行。
- [9d2ae61](https://github.com/HGamey/phonefarm/commit/9d2ae61)：同步外部Agent产品定位、设备能力边界、精简验证与直接提交规则。

验证：screen的13项Rust测试和实际HTML脚本的6项生命周期检查通过。全套尝试中777个unit通过；后续confirmation_driver 43/45，两项分别遇transport deadline和lease heartbeat stale。二者顺序单独运行均通过（11.10秒、43.21秒）；没有做基线对照，根因未确定，不能写“全套通过”。灰层修复只有一条新增行为回归，44项受影响的既有资格/eval测试通过。

[提交前code-only回执](https://github.com/HGamey/phonefarm/actions/runs/39775597937)。原始本机日志在transition/screen-*.log和gray-*.log。没有真机、节点替换或设备更新。观战与测量协调、统一远程认证/接管仍待做。

## 2026-10-22：动态原生工件补全与旧工件兼容

已推送 phonefarm [6cd1ac5](https://github.com/HGamey/phonefarm/commit/6cd1ac5)，GOL [1a8601e](https://github.com/HGamey/game_opt_loop/commit/1a8601e)。普通层脚本与动态builder共用一份构建输入/参数，包含npuprobe.cpp、-ldl、-z defs，链接阶段拒绝遗漏的项目符号。GOL接受npuprobe6，继续接受legacy4/ownership5；既有schema v1保持不变，不伪称旧工件已经升级。

验证：本机现有NDKr28c完成一次真实动态构建，产物为AArch64 ELF，NPU符号完整，仅依赖系统库；只读验证旧ownership5实际工件20个文件。PF已有工件合同1项、GOL原生单元4项、GOL→PF跨仓合同1项通过（跨仓使用明确标记的synthetic SDK，28.83秒）。没有真机运行、部署或恢复copyprobe/upop注入，产物仍为deployable=false/device_validation=not_run。本机日志native-builder-*。

提交前CI：[phonefarm code-only](https://github.com/HGamey/phonefarm/actions/runs/39776536157)、[GOL code-only](https://github.com/HGamey/game_opt_loop/actions/runs/39776537949)。GOL [3e78e95](https://github.com/HGamey/game_opt_loop/commit/3e78e95)、refbench [da7e30c](https://github.com/HGamey/refbench/commit/da7e30c)同步直接提交与验证分支规则；[refbench code-only](https://github.com/HGamey/refbench/actions/runs/39776538220)通过。

## 2026-10-22：共享设备状态、只读跨节点查询与设备页

已发布 harness-ops main [947a4b9](https://github.com/HGamey/harness-ops/commit/947a4b9)，包含 [6f50f84](https://github.com/HGamey/harness-ops/commit/6f50f84) CLI、[ee18c7c](https://github.com/HGamey/harness-ops/commit/ee18c7c)共享采样器/API、[e40e518](https://github.com/HGamey/harness-ops/commit/e40e518)设备页。

`/fleet`、`/dash/api/fleet`、`pf-node fleet`共用有界读取与同一采样器。`pf-fleet status/run`不再POST查询job；总deadline下每个登记节点都有结果，迟到线程不覆盖超时项。HTTP可达、锁状态、设备采样成功分别呈现。自动选机排除未知/旧快照/采样失败/排队；指定节点仍交给实际锁协议排队，快照不冒充资源预留。旧节点404明确提示升级，无提交查询job的兼容回退。

设备列表未取得为null，成功无设备为[]；失败保留last-good，显示时间和错误。新设备页/API只采元数据，任务/身份/原件浏览不启动手机采样。旧观战页继续既有截图规则；锁未知/心跳超期/刚释放时暂缓。页面每10秒读取缓存，隐藏暂停并取消请求；三宽度/浅深色浏览器检查通过，设备能力声明不冒充实机资格。

整合修复两处问题：旧maint.js不理解unknown锁导致绿色空闲，已按status修正；不兼容嵌套遥测原会使sampler退出，现发布前校验其使用的对象，失败保留上次样本，下一次正常响应可恢复。

验证：27项协议/故障回归通过（含本批6项），最终受影响读取10项复验通过；Linux Python3.11/3.14 CI通过。真实页面JS的隐藏/迟到响应、null/零设备、旧心跳检查，以及旧维护页unknown/stale/free渲染检查通过。浏览器实走unknown、零设备、旧快照/采样错误、队列展开、503失败重试与401暂停；390/768/1440浅深色截图保存在本机qa/fleet-*.png，明确是模拟数据。

提交前回执：[code-only](https://github.com/HGamey/harness-ops/actions/runs/39777998784)、[node-tests](https://github.com/HGamey/harness-ops/actions/runs/39777998749)。本机fleet-node-tests.log、fleet-read-final.log、qa/fleet-browser-validation.json与qa/maint-lock-state-result.json保留。没有运行节点部署或手机操作。

边界：网页只展示本节点；跨节点网页注册表、强lease/恢复屏障、持久排队和游戏分发尚未完成。完整的观察与测量协调仍在CTRL待办，不能将旧空闲检查当作原子互斥。

## 2026-10-22：独立作业进程与重启后取消

已发布 harness-ops main [225cb5c](https://github.com/HGamey/harness-ops/commit/225cb5c)，包含 CLI [03f31be](https://github.com/HGamey/harness-ops/commit/03f31be)、worker [1888c00](https://github.com/HGamey/harness-ops/commit/1888c00)、网页 [7619354](https://github.com/HGamey/harness-ops/commit/7619354)。合同见 [JOB-EXECUTION](JOB-EXECUTION.md)。

HTTP服务先发布作业记录，再启动独立worker；flock随描述符交接，实际命令由worker持有。服务重启后不向保存的PID发信号，取消通过持久请求文件送到worker。默认SIGINT不自动升级；显式force和旧kill接口保留15秒TERM、再15秒KILL。阶段、请求时间、已发信号、真实退出码和恢复证据分别可读；丢失worker为interrupted/恢复待核，不自动重放。旧任务继续可读，不提供无法确认所有权的信号控制。

网页查看权限与掌舵操作分开；API、CLI取消和旧目标页停止共用同一合同。Ctrl-C退出本地跟随且不下载未完成输出，后续status/wait可接续。整合审查发现旧目标确认入口强制读异步PID可能使“任务已提交却页面500”，已改为先保存job ID，并显示真实job锁名；不把提交说成已取得手机。

验证：本机35项节点测试通过，最终目标入口修复后2项受影响整合测试复验通过；真实HTTP服务进程被杀死、重启同一数据根后，可确认原worker仍活并停止无害子进程，得到SIGINT/exit0且recovery仍not_reported。Linux Python3.11/3.14节点CI和JS语法检查通过。网页实走只读/解锁、柔性停止、信号下钻、勾选升级、exit0仍需核查恢复；390/768/1440浅深色布局检查与迟到回执脚本通过。所有设备与任务数据均为本机模拟，未替换运行节点或操作手机。

提交前回执：[code-only](https://github.com/HGamey/harness-ops/actions/runs/39780607910)、[node-tests](https://github.com/HGamey/harness-ops/actions/runs/39780607535)。本机证据：qa/job-supervision-tests.log、job-restart-final.log、job-runtime-tests.log、jobs-cancel-ui-script-result.json、jobs-cancel-browser-validation.json、jobs-cancel-*.png。

边界：这批没有持久排队/提交去重/每设备原子资源领取/恢复屏障，未保证主机断电或脱离进程组后代退出。下一批按todo继续提交幂等和持久队列，不把宿主进程退出包装为设备恢复成功。

## 2026-10-22：服务入口补漏与持久队列

先修复上一批遗漏的signal导入：服务main仍使用signal，单测绕过main未发现入口失败。已发布 [2e0d140](https://github.com/HGamey/harness-ops/commit/2e0d140)，补配置错误路径的真实入口冒烟；[code-only](https://github.com/HGamey/harness-ops/actions/runs/39781284309)、[node-tests](https://github.com/HGamey/harness-ops/actions/runs/39781284307)通过。上批未部署，不能据此声称现网曾受影响。

队列批次已直接推main [58fc0dc](https://github.com/HGamey/harness-ops/commit/58fc0dc)。包含队列核心8c1f78a/966141e、CLI57f554a、网页061f7e0/9298f80/c006daa。合同见 [JOB-QUEUE](JOB-QUEUE.md)。

SQLite唯一请求记录与事务领取保证同ID同内容只对应一个job；同ID不同内容409。输入准备、待执行、已领取、恢复阻断、完成分别持久保存。独立调度器flock防止两个服务重复启动；已有launch标记不重放命令。队列按声明资源和宿主容量排队，无关资源可越过等待队首。准备/等待取消与领取使用同一事务边界，撤销后不执行；worker丢失/恢复required时保留资源，管理员说明释放后仍保留原恢复异常。

网页与CLI/API共同支持提交、队列、按请求ID找回、撤销、资源释放。只读角色可查询，修改需管理/掌舵权限；格式错误400与请求冲突409分开。网络结果未知时不生成新ID；网页刷新仅保留ID，允许重新填写并用原ID提交。调度器异常明确显示，队列记录仍可见。CLI向旧节点发送前先查幂等能力，不默默POST重试。

验证：本机44项节点测试通过（13.163秒）；Linux Python3.11/3.14 CI通过。真实HTTP进程被杀死/重启后，原worker与等待任务保留；4路重复请求都指向同一任务，资源等待任务只执行一次，被撤销任务没有执行标记。实际API验证查询权限、网页幂等/冲突与释放说明，恢复异常未被释放覆盖。两进程SQLite领取、无关资源继续等边界有针对性回归。JS检查覆盖不确定提交、409查询、迟到回执不覆盖当前选择、刷新→404→原ID重填、排队撤销与带说明释放；浏览器实走撤销/释放/503找回/刷新重填同ID，390/768/1440浅深色均无水平溢出。

整合中复现并修复两个竞态：调度线程在释放flock前发布空闲，新请求可能留在无人执行的队列；取消准备任务后，迟到metadata写入可能在重启后永久显示cancelling。失败和修复后触发回执均保留。首次整合测试因旧测试仍预期立即launch及将SQLite文件当任务目录而失败，调整到新执行边界后通过，没有用重复重跑掩盖。

提交前回执：[code-only](https://github.com/HGamey/harness-ops/actions/runs/39785760084)、[node-tests](https://github.com/HGamey/harness-ops/actions/runs/39785760282)。本机transition/qa：queue-integration-first.log、queue-node-final.log、queue-restart-final.log、dispatch-exit-race*.txt、prepare-cancel-restart-race*.txt、jobs-queue-browser-validation.json、jobs-queue-*.png。所有数据来自临时目录、无害宿主子进程或明确标记的网页fixture，fixture和临时网页已关闭。

边界：未部署运行节点、未操作手机、未迁移原始实验数据。本批声明资源不是设备lease；旧global devlock仍可能串行，恢复not_reported仍未接入强屏障；主机断电、脱离进程组后代、恢复诊断任务通道和两节点真机流程仍待做。下一批继续JOB-EXEC-03，不将本批写成完整M3达标。

## 2026-10-22：宿主 lease 屏障与显式恢复任务

已直接推 main：phonefarm [cafdbba](https://github.com/HGamey/phonefarm/commit/cafdbba)（lease只读入口4b330c2、旧桥接保护cafdbba），harness-ops [2b4747e](https://github.com/HGamey/harness-ops/commit/2b4747e)（网页14e901f/2ae2eec、CLI67afcf7、系统整合2b4747e）。合同见[JOB-RECOVERY](JOB-RECOVERY.md)。

phonefarm `lease status --json`观察主/嵌套marker和bridge，返回clear/held/blocked/unknown，不探手机、不建状态、不修改恢复记录。旧devlock acquire不能因owner缺失/过期绕过强marker；普通外层release保留嵌套恢复记录，合法进程树的外层心跳可继续。未知格式或普通旧锁不被误报clear。

node/CLI/网页共同显示实际lease；非clear时设备写任务继续排队，纯宿主工作可运行。blocked只保留资源而不占执行名额。显式恢复任务继承目标资源，可借用目标与blocked祖先链，与兄弟恢复和其他活动任务互斥；恢复任务结束不自动清父屏障。write任务not_reported、任何worker lost或recovery required均保留相应屏障。释放需管理员说明，无未结束/blocked恢复子任务，write/recovery还需fresh leaseclear；原恢复异常不被修改。

本批审计复现：初版把丢失worker的恢复子任务记done，另一同父恢复会获同一资源。已改为保留blocked并开放显式祖先链恢复，复现与回归保留。另修复lease在领取/启动间变化导致重复领取循环，以及新增默认device_access改变旧request指纹的问题。第一次HTTP测试把token提交写成dashboard JSON body导致400，已按已有X-Job合同纠正，失败日志保留。

验证：6项devlease单测、4项实际CLI恢复回归、3项devlock隔离回归；47项节点协议通过（13.376秒），Linux Python3.11/3.14 CI通过。跨仓实际debug binary、真实flock/无害子进程和HTTP的6项检查全部通过，设备传输调用为0：held时write等待，host任务运行；释放后write执行exit0，但not_reported仍blocked。实际网页验证note不能绕屏障、准备恢复仅预填、显式提交、取消与clear后带说明释放；390/768/1440浅深色无水平溢出。JS检查覆盖旧节点能力降级、409保留原因与重新采样。

提交前回执：[phonefarm code-only](https://github.com/HGamey/phonefarm/actions/runs/39789235284)、[harness-ops code-only](https://github.com/HGamey/harness-ops/actions/runs/39789238774)、[node-tests](https://github.com/HGamey/harness-ops/actions/runs/39789238751)。本机qa保留lease-status-*、lease-recovery-node-tests*.log、recovery-child-lost-reservation.txt、lease-node-host-integration.*、jobs-recovery-*截图与浏览器回执；模拟服务54725已停止，临时页面关闭。没有运行节点部署、手机操作或原数据迁移。

边界：宿主观察不授予lease，也不证明设备还原。全局bridge仍串行；默认临时lease根不保证重启持久性。通用恢复收据、job与强lease身份绑定、脱离进程组后代、两节点故障及手机验收未完成，JOB-EXEC-03保持未勾选。下一批继续这些系统问题；同时记录旧devlock心跳sleep继承日志管道的收尾延迟，未在本批扩大修改。

## GitHub流程与协作记录

用户明确授权管理员取消强制PR。phonefarm、harness-ops、game_opt_loop、refbench规则保留禁止强推、禁止删除及code-only必需检查；本机保存修改前规则到transition/qa。开发提交先进入verify分支，通过后直接推main；不创建PR、不绕过失败检查。

产品决定持续写PRODUCT-CHARTER/DISCUSSION/todo；可获得的用户消息与重要回应写本机conversation.jsonl，明确原话/摘要，不把回顾伪称完整历史导出，不保存聊天凭据到仓库。

下一批按todo继续JOB-EXEC-03的强lease身份/恢复收据及重启边界。游戏分发、跨节点注册/索引、研究状态和异步人工请求仍在计划；前端由具体subagent实现，主Agent负责系统合同与整合。


## 2026-10-22：job身份、局部恢复收据与心跳收尾

本批phonefarm [dc35335](https://github.com/HGamey/phonefarm/commit/dc35335)包含[b986fcf](https://github.com/HGamey/phonefarm/commit/b986fcf)心跳修复；harness-ops [11b50d9](https://github.com/HGamey/harness-ops/commit/11b50d9)包含[1d7ad6d](https://github.com/HGamey/harness-ops/commit/1d7ad6d)网页/CLI。发布回执见下段；合同在[JOB-RECOVERY](JOB-RECOVERY.md)。

node在启动worker前补建作业收据目录，传入明确job ID。phonefarm每次取得强lease发布独立公开lease ID及持久快照；全局lease观察带同一关联ID，恢复接管通过recovery_of指回原失败记录。普通退出/释放仅not_reported；完成报告必须带scope、evidence_origin和已存在文件引用。配置回读、会话静止与helper退役范围分别记录，synthetic不变成真机证据。再次hold、未知marker或发布失败保留不确定性；无hold的发布失败也留下ABANDONED屏障，后续进程不能静默取得。

只读API、CLI `pf-node leases [id]`和网页按需读取同一合同。读取限制在作业内固定目录，拒绝链接/身份不匹配；坏记录逐项显示，部分成功不报完整成功。网页提供原件下载、范围/来源/时间及旧能力缺失状态，证据宿主路径仅文本展示，不直接跟随访问。收据是最后发布的快照，崩溃可停留在active；必须结合job与当前全局lease观察，不据此自动清整个job资源屏障。

旧devlock心跳改用zsh内建定时等待，退出时只停止并回收自己创建的心跳；快速非零退出及时关闭日志管道，INT/TERM/HUP取消保留锁记录。没有用广泛杀进程或删除锁来规避恢复义务。

验证：48项节点协议（14.101秒）、9项devlease定向单测串行、2条既有synthetic恢复CLI、5项devlock隔离回归通过。真实签名phonefarm + node worker + 宿主文件锁 + HTTP的4项检查通过，设备传输0：job与全局lease身份一致，原始收据可读，exit0/released仍not_reported且队列blocked。独立宿主CLI验证坏上下文拒绝、发布失败后死owner仍ABANDONED。浏览器验证partial/unknown/空记录/旧能力、synthetic提示和引用展示，390/768/1440浅深色无水平溢出；前端关键JS验证迟到响应不覆盖其他任务。模拟服务56802已停止，临时tab关闭并恢复viewport。

失败记录保留：最初测试路径错误与跨线程Lease缺Sync在实施中修正；一次并行定向组8/9，Drop后bridge_held短暂为true。移除新测试不必要的子进程后串行9/9，未将这写成所有并行时序问题均已解决。此前完整confirmation集成的deadline/heartbeat偶发问题仍列TEST-03，不重复盲跑全套。

本机证据在transition/qa/job-lease-*.log、job-lease-host-smoke.json、job-lease-host-integration.json、jobs-lease-evidence-browser.json及截图。没有生产节点部署、手机操作或原始数据迁移。完整JOB-EXEC-03仍未勾选；下一批继续所有参与者统一持久lease根及主机重启/脱离进程组后代。bench、capture、gray/eval恢复producer覆盖单列待办，不能把它们released/not_reported解释为还原成功。

提交前CI通过：[phonefarm code-only](https://github.com/HGamey/phonefarm/actions/runs/39792389304)、[harness-ops code-only](https://github.com/HGamey/harness-ops/actions/runs/39791953614)、[Linux Python3.11/3.14 node-tests](https://github.com/HGamey/harness-ops/actions/runs/39791953589)。同一候选SHA直接推main，不开PR，不关闭必需检查。


## 2026-10-22：统一持久 lease 存储（本机验收通过，main 发布待 CI）

候选代码已提交并推到各仓 `verify/lease-storage-20261022`：phonefarm `588b1c197827a56d97004673cf2ae462c1802ec3`（含脚本47d96aa）、harness-ops `336eaaf9bcc485f94f99a30c0c6c2756e96e0251`（含网页1227f22）、game_opt_loop `29514d89b3ca3bce72055b7c73797ee8c86e5141`（含29c7863/469b9d4）。截至本记录，尚未发布到 main；后续以同一SHA的实际check-run及远端main为准，无PR。

合同见 LEASE-STORAGE。默认PHONEFARM_STATE_DIR/locks与devices经单一Rust解析器，所有参与者读取status.storage；custom显式兼容，旧协议或未初始化不猜空闲。显式init仅在旧参与者停止后绑定并停用旧域，不搬整个临时目录，不改新恢复marker；普通/强/嵌套锁、queue、qmutex、bridge及未知退休记录拒绝。临时旧域丢失后要求重新init，持久恢复记录保留。网页显示路径/模式/就绪状态，未就绪不可解除设备资源保留。

节点fleet、PF fleet与只读preflight在非clear时停止探测；preflight --fix持有实际强lease，使用其serial。配置错误立即失败，不当成设备忙等两小时；GOL只重试exit1。fleet观察门禁尚不等于覆盖整个采样过程的排他lease，竞争窗口继续留在todo。

本机通过：49项node协议（最终14.060秒）；6项devlock隔离回归；9项唯一Rust定向case（3存储、1fleet门禁、5实际CLI）；GOL4项devlock与1项路径回归、实际CLI构建签名。跨仓已签名实际PF/GOL、真实锁脚本、独立node worker和只读HTTP共4项通过，device transport调用0。默认迁移只在Rust内部注入临时旧根验证，跨仓使用显式custom隔离根，不包装成生产迁移或真机重启。网页390/768/1440浅深、clear+迁移冲突、unavailable/custom-ready、说明不能越过屏障均通过；fixture已关闭。

失败与修正：旧CLI测试精确JSON断言因新增storage字段失败，已保留旧字段断言并核对新合同；review发现并修正preflight --fix被自身outer锁误拒及配置错误长时间重试。GitHub工作流摘要曾显示success但同SHA实际code-only job仍queued，main推送被GH013拒绝；未绕过必需检查，不能按摘要称已发布。PF code-only run39796569493；harness code-only39795913237、node-tests39795913238；GOL最终code-only39796330509。验收日志、截图及这次发布状态在本机qa/lease-storage-*、jobs-storage-*。

无生产部署、真实锁根初始化、手机操作或原始数据迁移。完整JOB-EXEC-03/M3仍未完成。下一步先完成本批CI和同SHA直推main，再推进观察采样 ownership、producer恢复证据、普通旧owner超时/断电/逃离进程组等剩余条目，暂不切回Doom。


## 持久 lease 存储发布进度（接续时以本节为准）

phonefarm `588b1c197827a56d97004673cf2ae462c1802ec3` 与 harness-ops `336eaaf9bcc485f94f99a30c0c6c2756e96e0251` 已经在实际check-run通过后直接推main，并用ls-remote核实。harness的Linux Python3.11/3.14及code-only均成功。没有PR或强推。

GOL `29514d89b3ca3bce72055b7c73797ee8c86e5141` 仍在 `verify/lease-storage-20261022`，main尚未更新。check-run/job `119884317536`（run39796330509）的所有步骤均success，但check-run整体仍in_progress且conclusion=null；workflow摘要提前success不足为发布依据。早先一次main推送被GH013拒绝，未重复绕过。下一步只需等待实际check-run终态，成功后fetch检查并发、同SHA直推main并核实远端；失败则按日志修复，不重做整批测试。不因等待CI切换新开发批次。

本批本机验收与恢复边界见上一节及LEASE-STORAGE。临时测试服务/浏览器均已关闭，源码工作树干净；无部署、手机、真实锁域初始化或数据迁移。qa/lease-storage-publish.json记录两个已发布SHA与GOL待发布。广义开发目标保持active，完整JOB-EXEC-03仍未完成。


## 持久 lease 存储最终发布回执（最新）

三仓同一候选已在实际检查通过后直接推main并核对远端：phonefarm588b1c1、harness-ops336eaaf、game_opt_loop29514d8。GOL最终通过单项check-run119884317536确认completed/success，重试main推送成功；先前workflow摘要与check-runs列表状态不同步的观察/拒绝记录保留，不抹去。没有PR、强推或关闭必需检查。

LEASE-STORAGE-01已勾选。49node、6devlock、9Rust、5GOL及4项实际跨仓宿主检查、网页六布局通过，详见本批原始记录。源码树干净，临时fixture/浏览器关闭。未部署、初始化真实旧锁域、操作手机或迁移原数据。完整JOB-EXEC-03和广义开发goal仍active。

下一批先读PLAN/todo，按LEASE-OBSERVE-01解决采样窗口ownership；不要把clear观察当租约。producer恢复证据、hwcond临时回退、旧owner超时、逃离进程组后代/真正宿主重启/双节点真机仍未完成。暂停中的Doom和后台迁移按原交接保留。

## 2026-10-23：观察采样持有实际租约（本机验收完成，发布检查中）

合同为 [OBSERVATION-OWNERSHIP](OBSERVATION-OWNERSHIP.md)。fleet、只读 preflight 与 screen 每帧共用已有强 lease/bridge/devlock.d，不能借 label；等待队列优先。内部 worker 从枚举、尺寸查询到 shell/截图都在所有权窗口内，父进程先持久记 child 身份才放行。worker 的独立截止与父存活管道确保父进程退出或停顿不会留下无限采样；超时、异常退出、未知后代保持恢复屏障，不自动确认远端空闲。旧 devlock 不再接管 owner 尚未发布的目录。

fleet 明确 sampled/deferred/error；成功零设备仍为 exit 1。Node 与 GOL 保留最后成功设备及其时间，尝试时间独立；网页展示旧数据年龄、暂缓和失败原因。screen 无观看者不采集，每帧释放后才休眠；HDC 使用唯一截图文件并检查 snapshot/recv 状态，失败不下载上次旧图。

宿主验证：50项 node 协议（13.783秒）、7项 devlock、4项实际所有权/异常 CLI、5项已有 lease/recovery CLI、29项 fleet 单测、14项 screen 单测、2项实际 HTTP→worker→fake HDC、10项 GOL consumer。实际已签名 phonefarm→Node 三步验证成功设备→queue暂缓保留原时间→exit1空设备更新。所有 transport 均为 fixture，无真机采样。网页通过 Node 浅深/GOL暗、390/768/1440相关组合与错误/暂缓/无成功/空设备状态。

负结果与修正：短审发现最初仅父进程有 deadline，父被杀后 worker 可能卡住；已加 EOF/独立截止，并将测试改为不解除 fake transport 的 hold，确认整组退出且恢复记录保留。浏览器发现 GOL 390px 锁持有者行撑到518px，局部换行后回到390px；Node 页脚原“占用间隔60秒”的旧说明已改为占用/排队/恢复时暂缓，并复验最终截图。原始证据在本机 qa/observation-*、fleet-observe-*，失败记录不覆盖。

本批没有生产部署、真实锁域初始化、手机操作或原始数据迁移。没有 PR 或强推；候选先推 verify/lease-observe-20261023，实际检查通过后才能将同一SHA直推main。最终发布回执待补。完整 JOB-EXEC-03/M3 保持未完成，下一具体系统批为 hwcond 恢复状态路径及错误处理，再逐条补 producer typed 恢复证据。


观察批发布进度：harness-ops `f5e101a7e8ffb424a93c4a3901e90f9a94c3cab3`、game_opt_loop `d2d18401bc55a6c249a5e0289447d6aa99371e6e` 已通过必需检查直推 main，并以 ls-remote 核对。phonefarm `da3d2f58c89a5268ce77cc66838452e77d45f1ba` 仍在 verify 分支，main 未更新。其重跑 code-only 的所有步骤 success，但 check-run119904493688 仍 in_progress，正常 push 被 GH013 拒绝。未关闭检查或强推。等待实际终态后仅需同SHA推送，不重做整批验证。GitHub workflow/check状态不一致和一次短重跑记录在 qa/observation-ci-state-mismatch.json。


### 观察批最终发布回执（覆盖上段待发布状态）

phonefarm `da3d2f5`、harness-ops `f5e101a`、game_opt_loop `d2d1840` 已全部直接推 main，并用 ls-remote 核对同一SHA。phonefarm check-run119904493688最终completed/success后推送通过；此前GH013拒绝与延迟终态记录保留。harness最终提交的code-only119903474376、Python3.11 119903474296、Python3.14 119903474637均success；GOL重跑check119904487547成功。CI工作流：[PF](https://github.com/HGamey/phonefarm/actions/runs/39801013347)、[harness code-only](https://github.com/HGamey/harness-ops/actions/runs/39801599790)、[node协议](https://github.com/HGamey/harness-ops/actions/runs/39801599650)、[GOL](https://github.com/HGamey/game_opt_loop/actions/runs/39801162942)。

LEASE-OBSERVE-01已勾选；共享合同、本机交接/对话记录/原始回执已更新。最终模拟截图与服务清理回执见qa/fleet-observe-browser-validation.json；GOL最终二进制已构建并签名（首次签名命令误用连字符文件名，纠正为实际game_opt_loop，无未签名执行）。无生产部署、手机操作、真实锁域迁移或原始数据移动。下一批先读PLAN/todo，聚焦LEASE-STORAGE-02中hwcond持久恢复状态与失败处理，随后逐项producer恢复证据。不要重做本批测试，也不切回Doom。


## 2026-10-23：hwcond 持久恢复记录已发布

phonefarm main=`e06e1ff48beabe258a5d991dd8c7137c0b257273`，实际code-only check119919776212 completed/success后直推并核对远端，无PR或强推。[CI](https://github.com/HGamey/phonefarm/actions/runs/39806147255)。合同见 [HWCOND-RECOVERY](HWCOND-RECOVERY.md)。

状态根与lease共享解析，不再临时回退。新恢复文件独占创建、长度封装并同步文件/相关目录，拒绝覆盖未处理记录；旧纯文本严格解析兼容。未知有损serial别名/default/已知临时根记录保留并明确报错，不猜目标设备。恢复读取错误不伪装成不存在；回读匹配原节点与值后才删除记录。停充失败候选须确认回滚，否则停止；恢复失败不再误报已完成。四消费者提前处理状态错误，bench失败停止后续轮次，selftest输出先打开且不再删除未知fan记录。

最终构建签名后通过28项hwcond单测、3项storage单测和6项实际CLI/fake ADB验证。CLI覆盖损坏/不可读记录不写设备、unlock失败exit1、普通bench失败exit2并拒绝部署/测量、相对状态目录错误零transport、成功恢复对照。原始结果在本机qa/hwcond-host-cli-20261023T003403/result.json，单测/编译日志与前次fixture JSON解析失败均保留。本批没有网页协议改变，错误通过现有作业日志和selftest JSON显示，未重跑无关布局测试。

LEASE-STORAGE-02A已勾选；LEASE-STORAGE-02/JOB-EXEC-03/M3仍未完成。无生产部署、真实锁域迁移、手机操作或原始数据移动。停充守卫同实例重试为源码审读，未单独替身执行；不声称真实断电/设备恢复已验证。下一具体批PRODUCER-RECOVERY-01：bench实际lease hold与scoped typed恢复收据，再逐条capture/gray-ab/eval。不要重做本批或切回Doom。
