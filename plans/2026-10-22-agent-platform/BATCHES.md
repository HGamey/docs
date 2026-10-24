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

## 2026-10-23：bench 范围明确的执行和恢复已发布

phonefarm main=`2f5f11d1636841212fa78daf51ba7040b9fd392a`，实际code-only check119967822754 completed/success后直接推送并核对远端。[CI](https://github.com/HGamey/phonefarm/actions/runs/39819820066)。合同见 [BENCH-RECOVERY](BENCH-RECOVERY.md)。没有PR、强推或绕过检查。

bench首次设备写前持久记录session并hold实际强lease；本地命令记录PID/PGID后放行。远端复用worker所有权账本，管理benchmark与sampler，进程组确认结束后才恢复频率，最终关闭session并只读复核。实验成败和恢复结果分开记录；恢复失败保留屏障。`bench --recover`只接管同serial/context已结束owner，成功新增证据，旧失败原件不变。已有job回执接口继续展示scope/origin/evidence，不从bench退出码推断完整job恢复。

rootless/no-lock兼容；root可用时受控worker和负载采用root UID，报告明确记录，不能混用旧shell UID性能基线。默认只删除runner/model，保留关闭账本。所有未完成部署、未知硬件写入以及未知worker锁保持屏障；当前这类状态可能需独立核验，未实现自动恢复。硬件写前持久pending，只有明确命令成功才清除，重启recover也不能用新回读抹掉可能还在进行的旧写。

最终通过43相关Rust单测、3远端shell宿主fixture、9已签名实际CLI/fake ADB用例，共93次传输均核对PID/PGID/serial/pending，设备写均已有hold。覆盖rootless正常运行、旧别名无panic/零ADB、恢复失败阻断新任务、错误serial/context零ADB、同context成功恢复保留旧失败证据，以及15秒真实本地命令超时后无后续设备调用、recover仍required/零ADB。shell测试使用真实宿主进程组，/proc身份和kill错误为fixture；CLI远端响应均模拟，不声称Android、断电或性能验收。

负结果保留：QA首次输出目录与复制session重名，在CLI调用前失败，修正后通过；短审修正旧状态错误分支RefCell panic、无root探测错误处理和硬件写超时后误complete漏洞。最后一项另增超时回归，未盲目重复全套测试。证据在本机qa/bench-recovery-*、bench-worker-host.log；无node协议/页面布局改动，不重跑无关网页布局。

PRODUCER-RECOVERY-01已勾选，整体LEASE-STORAGE-02/JOB-EXEC-03/M3仍未完成。下一具体批PRODUCER-RECOVERY-02处理capture shutdown与恢复回读，随后gray-ab/eval；未知写入/部署核验和恢复label统一另入todo。未部署、操作手机、初始化真实锁域或迁移原数据；Doom继续暂存。


## 2026-10-23 capture worker与声明范围恢复（PRODUCER-RECOVERY-02）

问题：capture准备/落盘失败跳过shutdown，正常shutdown也覆盖原stayon并未读回；持续HID和截图传输没有同一受控所有权。合同先写CAPTURE-RECOVERY。现以父强lease监督same-exe worker PGID，启动门控后执行，父死EOF/独立deadline有界。session保存原设置、boot、输入身份和退出政策；每次写/HID创建前durable pending。只在worker全部设备操作与本地清理结束后写completion，active恢复必须具备该身份绑定记录；避免worker uncertainty落盘失败被旧manifest误放行。

父进程确认owned HID缺席、原stayon读回和请求的退出政策；no-shutdown保留应用/屏幕但仍关闭输入并恢复临时设置。game_state_restored明确false，不能声称角色、任务、存档或原前台已还原。采集结果和恢复结果独立，缺completion/unknown/pending/读回失败保持屏障；同context recover保留旧失败。PNG不经UTF-8转换，独立session避免旧frame/manifest混合。插件只加小QuestDevice接口，旧Device行为保留，mode/auto_choice/ready-only策略另批。

phonefarm bd67d97已推main并ls-remote核对；实际code-only check119988773686 completed/success（run39825838744）后推同SHA，无PR/强推。27项相关unit通过（初次genshin过滤为0，已纠正并执行3项plugin_tests）；11次实际CLI/fakeADB、64次传输门控通过，含父死EOF、deadline、恢复失败/续接、HID未知、原始PNG。qa/capture-recovery-host-validation.json记录两次fixture校准负结果和清理证明。没有运行真实游戏、操作手机、部署、迁移锁根或原数据；未改node合同/网页布局，不重复无关UI测试。BASE-07补记外层devlock拒绝无JSON；RECOVERY-UX-01/CAPTURE-POLICY-01保留未完成。

下一条仍为gray-ab/eval各自的producer恢复证据；整体LEASE-STORAGE-02/JOB-EXEC-03/M3尚未完成。用户明确未清空todo/plan前继续，整体goal保持active，单批完成不结束开发。


## 2026-10-23 指定目标灰层停用（PRODUCER-RECOVERY-03A）

合同见 [GRAY-RETIREMENT](GRAY-RETIREMENT.md)。共享off返回带serial、命令原始输出、范围的typed证据；读与校验先于修改，目标/state/当前安装路径或全局层冲突时停止。精确停止目标并读回，关闭并读回当前层12个属性（含旧清单遗漏的npuprobe），只删登记SO和已核验后的state。其它包同名SO、共享临时SO、历史marker和无快照的legacy settings保留；权限/传输错误不能当文件缺席。

gray-ab、frameaudit和eval保存停用证据，失败非零或ABORT并停止后续设备写。gray-ab不再在臂失败后继续并返回全部完成。eval明确layer_deactivated与整体restored未知，禁止用摘层文本生成PASS/KEEP。审查修复了首次snapshot先于部署脚本的顺序问题。完整sampler/输入/ftrace/停充/原属性恢复仍归03B，本批不调用complete_recovery。

phonefarm提交a38afa961f4fe2e87d0baa9eea21025d860acab0，实际code-only check119999626232 completed/success（[run39828942704](https://github.com/HGamey/phonefarm/actions/runs/39828942704)）。提交前吸收并发fd0bb36的fleet/keepalive鸿蒙状态修改并重新构建签名，不覆盖同事修改。24项相关Rust unit通过；实际CLI初轮3条65次fake ADB，npuprobe修复后仅重跑受影响2条68次，复用未变前置失败案例，最终适用证据共3案例69次。两个预期权限失败均保留state/原始输出，不进入下一臂；所有fixture PID结束。证据qa/gray-retirement-host-validation.json及030605/030830两目录。无部署、手机操作、锁根迁移或原始数据搬动；没有网页合同变化，未重复布局测试。

03A限定停用已完成；03B完整producer会话尚未实现，整体目标保持active并继续。


## 2026-10-23 有限灰层抓帧会话（PRODUCER-RECOVERY-03B1）

按 [GRAY-CAPTURE-SESSION](GRAY-CAPTURE-SESSION.md) 将实际 frame-audit capture 接到强 Lease 和串行有界传输。首次写前持久 hold 与 pending，保存 boot、目标安装目录和12个精确原属性；拒绝未知已有安装。只接管停止目标后确认的文件，旧 marker/audit 原目录备份、恢复，暂存目录由 shell 创建确保 adb push 可写。所有点击、触发、截图经同一会话，PNG取原始字节。

采集与恢复分别记录；本地采集/最终报告写失败不丢已知恢复结果。恢复先核 boot、安装及 STATE 归属，再停目标、清理自有安装/临时文件、恢复历史和原属性并读回。frame-audit recover 仅接同serial/context已退出owner，未知命令或写入保持屏障；报告原件 verified，租约完成后返回 reported_complete。scope限于原属性、自有文件和目标停止，game_state_restored=false；无设备写的准备失败用 no_device_mutations。

签名构建后17项相关Rust unit通过（2 session、5 graylayer、3 lease_command、7 devlock）；初始过滤误用graylayer::tests导致0项，纠正后只执行该组并保留日志。9项有效CLI语义验收通过，共896次fake ADB传输，均核PID/PGID/serial和写前hold/pending。覆盖正常、坏PNG但完成恢复、恢复不符后阻断新任务、错误serial、同context恢复且保留旧失败、未知写及后续恢复拒绝、最终metadata落盘失败保留恢复报告。实际共11次CLI：runner误给recover添加--lock-wait造成2次参数拒绝，未计有效身份/恢复验收；修参数后从已有session接续，前面通过案例未重跑。原始两轮qa/gray-capture-host-cli-20261023T033457和033750、unit-results/host-validation保留。

源码 f966c53aad66de5754ff3dbce0acfdfe4165898f 经实际code-only check120015070859 completed/success（run39833339122）后同SHA推main，并ls-remote核对。没有部署、手机操作、真实锁根/原数据迁移，不声称真实设备恢复或性能验证。网页合同未变，结果仍通过既有作业产物与恢复收据消费；未重复无关布局测试。03B父项、LEASE-STORAGE-02/JOB-EXEC-03/M3及整体goal继续进行。下一具体子项是storeop-check的同一会话控制，再处理eval父租约借用；子范围完成不能清父任务屏障。


## 2026-10-23 独立storeop检查会话（PRODUCER-RECOVERY-03B2A）

合同为 [GRAY-STOREOP-SESSION](GRAY-STOREOP-SESSION.md)。独立frame-audit storeop-check复用已有GraySession，rule/live/on在开启全局层和启动前验证、设置并读回，现场开关读回成功才继续。每段三张无损PNG落盘，使用已有image库在资源上限内解码RGBA；沿用pixel_diff、UI排除区域及三类比较，坏图/缺帧/不同尺寸明确失败。JSON保留原指标字段并提供result、全部帧路径、session、collection和recovery；本地失败不跳过安全收尾。

20项相关Rust unit通过（4 session、15 frameaudit、1 eval livecheck）；初次eval测试模块路径误用导致0项，已纠正为path_tests并保留日志。签名实际CLI四条一次通过：正常214次transport/9张PNG/6组零差值；PNG签名合法但截断IDAT触发解码失败，187次/1张且恢复完成；on=1非零116次/3张，pending保留且后续零transport；坏rule47次只读/零写，scope=no_device_mutations。共564次门控传输、124写前pending核验，所有fixture PID退出，无服务。qa/gray-storeop-host-cli-20261023T034828/result.json、host-validation、unit-results均保留；未重跑03B1九条。

源码7a3d7d1ba704d8689e6b6642e4880e93a5d48458经实际code-only check120021145525 completed/success（run39835080780）后同SHA直推main，并ls-remote核对。没有部署、真实设备、数据迁移或性能验证。没有网页协议变化，沿现有产物/恢复收据显示；不重复布局测试。eval仍显式调用storeop_check_legacy，不能宣称整eval已经受本session保护。

下一03B2B与完整gray_eval父会话合并，先交付调用storeop的workload.launch分支及recover，避免发布借用子步骤后父层永久required的半成品。源码核实storeop前仅能力只读/本地准备；实际后续义务是层/原属性/历史输出、fan、目标/启动marker、sampler、专属ftrace实例和专属工具目录。gray_eval当前没有停充或锁频，不能凭共享helper名称扩大恢复声明或隐式新增这些操作。script分支允许任意exec/gamepad及额外工具上传，另列未迁移范围。整体goal保持active。


## 2026-10-23 Gray 自驱动 eval 父会话（PRODUCER-RECOVERY-03B2B）

按 [GRAY-EVAL-LAUNCH-SESSION](GRAY-EVAL-LAUNCH-SESSION.md) 让同一个强Lease覆盖静态storeop检查、运动自检、三基线、交替候选/对照和最终恢复。子collector不创建/结束租约；新v2会话有明确completion_owner，frame-audit recover拒接父身份。属性、安装与历史文件只备份一次，各代产物归档；无层基线和probe两臂会重置image-check观测开销。

父会话登记真实改变的fan、自有工具/采样进程/trace实例；worker有两个受控child、独立采集退出码、当前静止验证与关闭账本。trace只使用独占实例和boot时钟，删除前确认自有进程停止；不动全局trace、不隐式停充/锁频。已知部分上传尚未启动可收尾；未知写/pending保屏障。eval recover只续接相同context，原失败报告不改写，game_state_restored=false。远端关闭账本、脚本和轮次原件按报告政策保留，不声称无残留。

恢复和采集结果独立。父在Experiment阶段保留restored未知，只有已核静止且无pending的专用admission可以继续采集；Conclusion仍需完整恢复。审查修正两个判定缺陷：普通renderpass数量不能补成改写命中，最终统计必须使用冻结required_metrics，避免render_cadence仅凭功耗改善而PASS。原阈值、图像差值和统计实现复用。

13项唯一相关Rust unit通过，实际14执行（审查后只重跑命中扩展及新增冻结目标回归）；3项唯一worker宿主fixture通过，新增idle模式后仅重跑受影响1项。5次实际=5次有效CLI/fakeADB、2048门控传输，529次写前base pending与41次父pending核验。完整三基线+六对ABBA/15张静态PNG完成后注入fan恢复差异，正确保屏障；独立recover拒父0传输；同context恢复成功109传输且旧原件不变；synthetic来源只到三基线、不进候选，仍完成恢复；首次id只读失败0写正常no_device_mutations。

QA一处断言误去stderr读取本来在JSON stdout的拒绝说明，原负记录保留，离线核对后只续跑剩余3调用，无重跑。首条CLI已运行的签名映像与最终增量只差命中兜底/最终统计/help，该例实际begins=2且恢复失败未进入最终统计，适用性已单独记录；其余最终签名binary。所有2048模拟进程退出、隔离lease清空，无服务。证据qa/gray-eval-host-validation.json、041728/042432原件、unit结果及worker日志。

源码9e1bb0a81c93f02d9532435d4585972c7e0bdd61已直推main并ls-remote核对。原check120037546850步骤成功但终态卡住；仅重跑code-only检查一次，实际check120042514678 completed/success（run39839817705），推送前再次确认同SHA与远端并发状态，未重跑实验验证。没有部署、真机验证、锁根迁移或原始数据搬动。网页沿既有job产物与恢复收据合同，没有另建协议；未重复无关布局测试。03B脚本路线/gray-ab及整体goal继续，不能把该子批当全目标完成。


## 2026-10-23 有限触控脚本 gray eval 会话（PRODUCER-RECOVERY-03B2C）

按 [GRAY-EVAL-SCRIPT-SESSION](GRAY-EVAL-SCRIPT-SESSION.md) 统一launch/script父循环，移除旧未受控eval脚本通路。有限sleep/tap/swipe计划在写设备前验证；精确兼容旧input swipe以保留默认三次12秒拖动。输入、trace、sampler各有登记PGID、设备时钟deadline与实际退出/采样窗口。坐标用实际PNG边界；自然输入成功必须覆盖trace，输入中断不以进程消失代替触控恢复。旧launch collection字段和负结果保留，最终质量/冻结指标判据不变。

phonefarm 2fb0b164f431b7ea37339896a768360f0fab6f63 经实际code-only check120059560881 completed/success（[run39846075471](https://github.com/HGamey/phonefarm/actions/runs/39846075471)）后同SHA直推main并核对远端。10相关Rust单测通过；3唯一worker行为共5执行（真实模板替换重跑1项，POSIX awk表达式失败修复重跑1项，旧两child合同通过）。所有失败原件保留。4实际=4有效CLI一次通过，共1868传输、410base写前pending/41父pending：16采样及18比较PNG后已知fan恢复失败；同context恢复成功、原失败不变；输入中断缺自然收据保屏障；非法exec零transport。全部模拟PID结束，仅预期中断fixture屏障保留。

最后launch字段兼容增量不影响四条script CLI；最终构建签名并追加既有live-check单测，没有重跑整链。原始回执qa/gray-script-host-validation.json、gray-script-host-cli-20261023T045448/result.json及unit/worker/source-review均在本机。没有部署、手机操作、锁根迁移、原始数据搬动或真实性能结论。网页继续消费既有job产物/恢复收据，没有新页面合同，不重复布局测试。

本项完成，03B/03/JOB-EXEC-03/M3和整体goal仍active。下一03B3 gray-ab loadop：显式整轮停充、owned sampler、按原ctrl/knob顺序采集、两臂passdump=0；不把旧t≈10后短功耗点列当同步trace窗，也不套eval资格/ABBA。03B4其余路径仍待。


## 2026-10-23 Gray-ab loadop 采集和停充恢复（PRODUCER-RECOVERY-03B3）

按 [GRAY-AB-LOADOP-SESSION](GRAY-AB-LOADOP-SESSION.md) 迁移loadop完整命令。保留ctrl/knob顺序、默认有限拖动、原场景门禁和两臂passdump=0；复用三child输入/trace/sampler与原始证据，不套eval资格/基线/ABBA。新协议在首臂前停充、跨场景准备保持，明确记录40秒sampler全窗；保留PerfSnapshot关键字段和null原因，旧env charge_suspended不作新owner证明。没有新增fan、锁频或温控写。

停充记录在首次写前绑定serial/token/context/原节点值，候选缺席与权限/传输未知分开；新session不能接管外来record。loadop收尾先只读核当前groups，再恢复充电并读回，随后输入自然完成/关闭worker/trace/层和属性。输入未自然完成不能清整个屏障；已核实的charge部分恢复仍在required报告保留。gray-ab recover只接本owner，eval/frame-audit不能代替。已知缺失round证据保留为采集错误；报告持久化失败独立返回2而不掩盖已知恢复结果。

源码73c0a8d5765351b1ed6d9c7dd70f2b6eb18e8005经实际code-only check120070209770 completed/success（[run39849109461](https://github.com/HGamey/phonefarm/actions/runs/39849109461)）后同SHA直推main、ls-remote核对。构建签名及8相关Rust unit、1受影响worker测试通过；3实际=3有效CLI一次通过512门控传输、99base pending/16父pending/3charge binding：完整ctrl1→knob1末尾known charge mismatch；同context恢复成功、原失败/collection/绑定原件不变；foreign record零传输拒绝。全部模拟PID结束，无lease遗留，外来record原样保留。

回执qa/gray-ab-loadop-host-validation.json、052339/result.json、unit/worker/source-review均在本机。已放电observed_no_mutation仅源码审读；输入缺natural但先恢复charge以worker核验+源码顺序审读覆盖，未宣称这两个分支已做完整CLI。无生产部署、手机操作、锁根/数据迁移或真实性能结论；网页沿现有job产物/恢复收据，无新UI合同，不重复布局测试。

03B3勾选，03B/03/JOB-EXEC-03/M3与整体goal仍active。下一03B4A先把默认native不可执行的copyprobe/upop从宿主和已知shell入口提前拒绝，避免先运行none基线、摘层和触控；自定义SO能力未核验，不凭ABI/manifest直接放行。保留工件构建/离线分析与旧disabled证据，不恢复注入功能。


## 2026-10-23 未支持灰层模式提前准入（PRODUCER-RECOVERY-03B4A / MERGE-03）

按 [GRAY-MODE-ADMISSION](GRAY-MODE-ADMISSION.md)，copyprobe/upop 在排队、选择设备、摘层或基线之前返回结构化unsupported；默认缺执行合同，自定义SO能力未核验。公共mount同样保护，删除gray-ab不能执行的none/copyprobe私有采集链；enable_layer/upop_ab/quality的已知旧入口同批覆盖。离线工件/历史证据与reference-only路径保留。两个灰命令的参数消费与gate一致，控制字作为serial/path值不会误触help/免锁，真正help与recover保持原路线。

phonefarm155506f3407bfbe7a78bece9bdbbd88fe7f9d3f6经actual code-only check120077413135 completed/success（[run39851167210](https://github.com/HGamey/phonefarm/actions/runs/39851167210)）后同SHA直推main并核远端。构建签名、6相关unit一次通过；3实际CLI一次通过且零设备/锁调用/实验输出/恢复记录；7shell fixture一次通过（5拒绝零命令，2reference/passdump仅在首条假ADB截停），4shell语法检查。全部原件与回执在本机qa/gray-mode-admission-*；未重跑已过loadop/eval链，未部署/手机操作/数据迁移，无真实性能结论。

03B4A与限定MERGE-03勾选，03B4/03/JOB-EXEC-03/M3仍未完成，整体goal active。下一具体03B4B先清理已经无调用的旧storeop/reach_scene/launch链，保留whitebox仍用raw截图；随后完善可执行gray-layer与旧参考脚本的持久会话和恢复。用户质疑关goal，本轮get_goal核实active，未调用update_goal；单批完成不代表总目标完成。


## 2026-10-23 灰层无调用旧链清理与持续控制合同（PRODUCER-RECOVERY-03B4B）

删除已被 owned collectors 替代的 storeop_check_legacy 及其私有场景/原始 ADB/抓帧链，共移除217行旧代码；保留实际 loadop 场景、whitebox raw 截图和独立 PNG/像素分析。可达命令与恢复协议未改变，不重跑已验 CLI。构建并签名，既有 raw header、pixel diff、storeop comparison 三项各执行一次通过，无新增测试。

源码557158acfa4350a70a3442c95ef5517b8bc2062e经实际code-only check120080767875 completed/success（[run39852120062](https://github.com/HGamey/phonefarm/actions/runs/39852120062)）后同SHA直推main并核远端。本机qa/gray-legacy-cleanup-*保留构建、测试、调用审查与发布记录。无部署、设备操作、锁根/原始数据迁移。

[GRAY-CONTROL-SESSION](GRAY-CONTROL-SESSION.md) 冻结下一批：一个持久作业持有真实lease，网页与Agent通过同一有序mailbox连接其owner；接管转移会话输入权，不转移lease。首批交付看屏/有限输入/灰层采集和停止恢复；性能采样与旧脚本迁移另列03B4C2。现阶段该能力尚未实现。03B4B勾选，03B4/03/JOB-EXEC-03/M3及整体goal继续active。

## 2026-10-23 可持续远程灰层会话（PRODUCER-RECOVERY-03B4C1）

按 [GRAY-CONTROL-SESSION](GRAY-CONTROL-SESSION.md) 交付实际 PF producer、node API、pf-node 和任务页。单一存活 owner 保持强 lease，宿主 publisher 分配有序请求，started/result 各自持久化；HTTP 超时后按原 ID 找回，同 ID 幂等，不自动重放输入。控制 epoch/人工 TTL 与 lease 分开，stop 在动作边界优先处理并保留未执行回执。全尺寸有界 PNG 解码、最新图片/年龄/实际方向核对后才输入；copyprobe/upop 仍提前拒绝。

复用 GraySession 的12属性、安装/历史文件及目标停止恢复；公开 session ID 独立于内部 token。devlock 父进程在 SIGINT 后等待子进程恢复，避免外层锁提前遗留。已知读回失败保留原恢复报告，同 context recover 可续接；通用 node job exit0 仍为 not_reported，局部恢复成功不能自动释放任意 job。只读 lease 汇总先呈现已知恢复 blocked，再保留普通外层未知诊断；外层实例关联另列 LEASE-IDENTITY-01，准入/clear 未放宽。

21项相关 Rust 测试各一次通过。harness 10项唯一 Python 用例共12次执行（ID约束变化、并发 rebase 后各只重跑受影响CLI一项），JS状态检查和浏览器390/768/1440浅深色验收通过。网页 actor 重载丢失已修复；原始图、accepted/完成、未决ID、接管/续期/交回、有限输入、关闭后资源保留均核验。fixture已停、tab已关闭。

两条实际签名 PF + node HTTP + fake ADB 链通过435次门控传输：正常链198次，包含一次真实HTTP响应超时、原ID找回/幂等、只读401零发布、重连零设备动作、旧epoch拒绝、输入/trigger/audit/stop；另一资源名的作业仍被device lease挡住并取消未执行。取消链237次，node SIGINT→已知属性读回失败exit2→原context恢复成功，旧失败原字节保留。各核查201/241个已知PID均退出、无lease遗留。normal使用最终build-3；cancel使用build-2，最后增量只改只读汇总，未重复已过取消链。

首次normal fixture把全局状态错误断言为held，在unknown外层诊断处停止；该次182次传输及正常恢复原件保留，不计通过链。全部尝试617次。初次构建引用未引入的ctrlc依赖也保留日志，改用已有libc sigaction后成功。qa/gray-control-host-summary.json、harness-validation、ui-browser-validation、unit-results、source-review、publish及逐次目录均留本机；没有真实性能或Android恢复结论。

PF [4ca316d](https://github.com/HGamey/phonefarm/commit/4ca316d27cc6a4965453e7d778257505ba33bb59) 经实际code-only120098287846成功；harness [2b8066d](https://github.com/HGamey/harness-ops/commit/2b8066df295d91b5b40ae641951bfb9ca0760dc2) 经code-only120098396581及protocol 3.11/3.14的120098396412/120098396849全部completed/success后，同SHA直推main并核远端。吸收同事27923bd的默认mbp1地址修改，无生产连接/部署/数据迁移。

03B4C1完成，03B4C2继续：在同一owner内复用采样worker及父恢复合同，避免调用会自动停止目标的旧eval整轮入口；之后迁移旧gray-layer/reference/quality可执行入口。03B4/03/JOB-EXEC-03/M3和整体goal仍active，没有调用update_goal。

## 2026-10-23 持续会话的当前场景采样（PRODUCER-RECOVERY-03B4C2A）

按 [GRAY-CONTROL-SAMPLING](GRAY-CONTROL-SAMPLING.md) 新增sample动作，1至120秒，沿原controller/epoch与有序mailbox。每轮sleep plan、独立派生脚本和现有三child guard/ready/uptime收据共用同一owner；保持运行场景，不自动重启/停止、触控或调整充电/风扇/频率。measuring期间观察和输入排队，stop在有界采样安全边界收尾；健康的采集失败允许继续，未知资源状态保屏障。

control-only base @3明确绑定资源父，旧binary拒绝，缺父记录不回落成旧局部完成。恢复先核worker、trace与自然完成，再恢复base，最后用完整父报告complete；原C1 @2恢复合同保留。每round使用自身plan，父manifest读取上限8MiB，动作/descriptor仍64KiB。远端关闭账本/脚本/原件按现有政策保留，不声称设备无任何文件残留。

纯解析复用looptrace/env_stats和原窗口绑定，无eval编排或目标停止。render_comm可选且显式，未选线程只保存raw/env、帧指标null；已选也只称GPU submit proxy，目标进程/ctx与显示帧语义未验证。功率要求真实battery全窗放电及trace覆盖，不接受旧bypass文件为证明；每帧能耗明确null。collection失败与恢复状态独立，原始错误、计划、脚本和采样证据可追。目标线程归属另列METRIC-OWNERSHIP-01，不混用旧Unity假设。

Web/API/CLI沿同一action合同；网页新增秒数/可选线程、measuring反馈、失败及原件，重连读取latest_sample缓存，已选旧回执不被状态刷新覆盖。4项相关Rust回归各一次、1项既有三child worker扩展一次（8.665秒）、1项CLI及定向JS通过。浏览器390浅/1440深/768浅验证新增状态和布局；仅文案与缓存接续增量重跑对应JS，未重复浏览器/CLI。fixture已停止、tab关闭。

最终签名build-2用于两条实际PF+nodeHTTP+fakeADB链，共606次门控传输：normal319次，同owner两轮3/5秒、无comm/null与显式comm/proxy、Charging功率null/能耗null、采样内capture排队、同ID不重采、原件下载和完整stop；parent-recovery287次，已知trace恢复读回失败时base尚未开始、exit2保留hold，随后同context恢复成功，旧报告原字节与引用保留。各核322/291个已知PID无存活、无lease marker。旧C1 closed状态读取一次零ADB；恢复兼容为单测覆盖，不宣称旧C1完整链重跑。

首次fixture未解析fake-bin/adb软链接，首条只读命令退出，0条logged fake transport且未挂层/采样；no-device-mutations恢复原件保留。仅修fixture路径与启动失败等待，之后未完成的两链各执行一次。首次源码构建成功，去掉重复原件链接后重建签名，未重复已过实验。qa/gray-control-sampling-host-summary.json、unit-results、worker原件目录、source-review、publish与gray-control-sample-ui-validation.json留本机。无部署/真机/锁根或原数据迁移，无真实性能结论。

PF [618e456](https://github.com/HGamey/phonefarm/commit/618e456b910e3513d431ad237b204b60b5f72a8b) 经实际code-only120107718390成功；harness [cd62842](https://github.com/HGamey/harness-ops/commit/cd62842ec307163fe70a34e6bf1f5956f09d5a0d) 经code-only120107749700、protocol3.11/3.14的120107749875/120107749897全部completed/success。同SHA直推main并核远端，没有PR。

只勾C2A。下一C2B1先接显式有限输入及动作版本能力，防新helper将输入计划投给旧idle owner后被忽略；C2B2再接显式充电绑定与同期能耗。quality、reference、四臂refbench和旧挂层入口迁移继续待办。C2/03B4/03/JOB-EXEC-03/M3及整个goal保持active，未调用update_goal。


## 2026-10-23 当前场景的显式有限输入（PRODUCER-RECOVERY-03B4C2B1）

按 [GRAY-CONTROL-SAMPLING](GRAY-CONTROL-SAMPLING.md) 增加sample的显式input+frame_id，默认仍idle。复用有限Plan与三child执行体，每round保存input_kind、规范化计划和生成脚本；声明时长须覆盖seconds+10、最多600秒，不自动补输入。预算按完整计划核对。cached图先核坐标，资源部署和上传完成后、worker放行前再取真实PNG/rotation核图龄、尺寸、方向与预算。输入来源图和检查原件保存，initial_frame.path沿会话图片合同相对job out；run.artifacts相对run目录。自然完成只证明有限输入结束，不声称游戏状态已还原。

sample capability=2与resource parent@2绑定；新publisher对旧idle-only owner明确拒绝input，旧binary对新父绑定拒绝。新版保留C1无父及C2A @1严格idle的恢复语义，不能把旧parent升级成新计划。缺自然完成记录使action unknown并保持完整恢复义务；预放行几何拒绝没有worker permit，原件仍可读、健康会话可继续。

网页默认无输入，提供JSON步骤、示例、绑定帧信息和明确重绑；新图或过期禁止提交，不自动采图或缩放。结果显示输入类型、声明时长（非实测耗时）、参考图。CLI帮助与原序列化用例同批更新。

构建签名、5相关Rust既有用例各一次通过；定向JS、1CLI及浏览器单流程通过，390浅/1440深无溢出、console空，fixture与tab已关闭。worker执行体未改，沿用已有验证，不重跑。实际PF+nodeHTTP+fakeADB两链一次通过493次门控传输：normal336，5种非法请求及cached帧/坐标拒绝无transport，上传后方向变化拒绝零worker，finite13秒→idle15秒、逐轮计划、幂等、同owner及完整stop；negative157，缺自然收据时即使模拟组静止也不提前恢复base，exit2/recovery_required。

normal核339个已知PID全部退出且无lease；negative核160个已知PID全部退出，仅QA隔离目录保留1个recovery_required租约（无pending/childPID/PGID）与模拟远端原件，没有伪造收据或强制clear。旧C2A closed状态只读一次零ADB，新旧恢复边界由相关unit覆盖。qa/gray-control-input-host-summary.json、065811955825/result.json、unit-results、ui-validation、source-review、publish留本机。无部署/真机/原数据迁移，无性能结论。

PF [65de801](https://github.com/HGamey/phonefarm/commit/65de801d25a1c1bb083a0d13dcf1c901db0bc290)经actual code-only120113860496（run39861735718）成功；harness [e0818b4](https://github.com/HGamey/harness-ops/commit/e0818b41bb3fb7a9d9d203d58c349348e14b2ec0)经code-only120113919879（run39861754546）及protocol3.11/3.14 120113920234/120113920426（run39861754523）全部completed/success后同SHA直推main并核远端。无PR。

只勾C2B1，下一C2B2显式供电条件与同期指标资格。C2B/C2/03B4/03/JOB-EXEC-03/M3及整体goal保持active，未调用update_goal。


## 2026-10-23 持续会话的固定供电与同期能量（PRODUCER-RECOVERY-03B4C2B2）

按 [GRAY-CONTROL-POWER](GRAY-CONTROL-POWER.md) 实现创建时observe/battery固定政策，默认只观察。battery首次采样准备一次、跨轮保持，复用已有ChargeSuspend的原记录/serial/token/context绑定与回读恢复；已放电走observed_no_mutation零控制写。新父@3兼容旧@1/@2恢复，sample能力仍2且接受@2/@3绑定。状态查询只投影宿主持久白名单，不为读状态调用ADB。

每轮power.json与回执保留前后观测。采后供电失败保留原性能数据，只剥离功耗资格；末次证据保存失败由返回快照传入解析，不能被较早成功文件掩盖。全sampler功率沿原口径，新增trace_energy_j/trace_power_w_mean采用同uptime边界的电池V×I线性插值/梯形积分，声明estimated；缺放电或窗口夹持返回null。energy_per_frame仍null，完整分组窗/目标身份另列todo。

Web模板只填现有argv，默认observe、显式battery，不自动POST或覆盖未决请求。固定policy、充电控制修改、前后实际放电、恢复与功率资格分开显示；CLI/API同合同。6个相关Rust各一次通过；node ownership/CLI既有用例各一次、定向JS和390浅/1440深单浏览器流程通过。后续flat initial显示修复仅增量跑相关JS。worker没改不重跑。一次错误Rust selector跑0，纠正模块名后只实际跑1次；原日志保留。

实际PF+nodeHTTP+fakeADB三条会话及foreign短CLI共830次门控传输：跨finite3秒/idle5秒一次停充，恢复读回失败后同context成功382；初始放电零控制写260；缺natural在recover中先核组、归还charge，再严格拒绝完整恢复188；foreign记录在lease/transport前拒绝0。固定4W合成数据得到12J/20J仅证计算链，不是手机能耗。841个已知PID全部退出；缺natural保留1个新的QA隔离required租约及原件，旧B1负例未动。

第三链最后夹具直接索引省略child_pid造成KeyError，原失败result/run-1保留，语义动作已经执行；只对已保存原件补离线断言，再跑未执行foreign短CLI，无重跑会话。qa/gray-control-power-host-summary.json、negative-offline-check、unit-results、ui-validation、source-review、publish索引留本机。无部署/真机/数据迁移或性能收益结论。

PF [86edb56](https://github.com/HGamey/phonefarm/commit/86edb56f2e9e445f16b574a0e00988078793de4c)经actual code-only120122515647（run39864307969）成功；harness [d4d8428](https://github.com/HGamey/harness-ops/commit/d4d8428422cf9273e51c084fd47a0d90fcfef8f5)经code-only120122573729（run39864325073）与protocol3.11/3.14 120122572790/120122573023（run39864324691）全部completed/success后同SHA直推main并ls-remote核对，无PR。

勾B2与限定B，继续C2C画质跨人工等待同owner，后续C2D旧reference/refbench/挂层迁移。C2/03B4/03/JOB-EXEC-03/M3及整体goal仍active，未调用update_goal。


## 2026-10-23 同owner的共享画质检查点（PRODUCER-RECOVERY-03B4C2C1）

按 [GRAY-CONTROL-QUALITY](GRAY-CONTROL-QUALITY.md) 增加quality能力1，begin/checkpoint/finish沿既有mailbox/current actor/epoch。manifest记录三成功阶段与最近尝试，attempt原件按request ID分开；新ID可重采失败阶段，同ID不重放。完整PNG与RGB偶数坐标stride2半尺寸预览独立保存，finish还需native layer_loaded/pkg/readonly_stats marker。缺marker保图、阶段不前进；采集完成不代表同场景/同时辰/画质通过或设备恢复。没有新增HTTP、owner、lease或worker协议。

Web/CLI同回执与manifest，重连恢复共享状态，未决attempt不冒充正在运行；quality错误不会混进性能sample结果。网页swipe范围对齐既有10000ms API。构建签名、3相关Rust各一次、1既有CLI、定向JS与390浅/1440深浏览器通过；未决attempt文案只增量跑相关JS。原界面截图与日志留本机。源码审核发现真实marker计数在readonly_stats内，已在构建前修正源码和fixture，未拿替身顶层格式冒充真机。

actualPF+nodeHTTP+fakeADB唯一会话一次通过199传输：4张截图，14工件逐字节HTTP下载，4张32×18原图的16×9预览逐像素核验。人工接管/归还和CLI重连、旧epoch/旧图零transport拒绝、同ID成功或失败均不重采、缺marker后显式新ID完成均通过。仅最初启动一次，无quality导航/重启/off/采样。stop exit0，202已知PID全部退出，本会话无lease遗留；B1/B2隔离负租约未动。qa/gray-control-quality-host-summary.json、host-cli-20261023T074748109188/result.json、unit-results、ui-validation、source-review、publish记录完整。首次发给空闲QA agent的普通消息未唤醒，发现后用followup启动；没有实际链重复执行。无部署、真机、数据迁移或性能结论。

PF [45737cc](https://github.com/HGamey/phonefarm/commit/45737cc89b780dd5e2d3192623c8a6092a759bea)经code-only120134106935（run39867743861）；harness [c62e062](https://github.com/HGamey/harness-ops/commit/c62e062515763f8dc8a2d2efcad1265622751ab4)经code-only120133380672（run39867530203）、protocol3.11/3.14 120133380614/120133380885（run39867530213）全部completed/success后同SHA直推main并ls-remote核远端。

只勾C2C1。C2C2已按docs f1db6b4开始迁移旧genshin_quality客户端，不混入这批提交；C2C/C2D/03B4/03/JOB-EXEC-03/M3与整体goal保持active，未调用update_goal。


## 2026-10-23 旧画质导航接入既有会话（PRODUCER-RECOVERY-03B4C2C2）

旧genshin_quality.sh改为薄入口与stdlib Python客户端；显式既有本机out或node URL+job，没有隐式生产地址、serial或新job/owner。mode只核现有层，调用者负责已站定大世界；原off/force-stop/安装/清marker/点门及passes/frame启发式移除。显式package/尺寸/rotation/坐标profile，每个输入前freshcapture，人工等待后finish重新取图；调用内固定actor/epoch，跨调用可读新epoch，中途改变即停止。原2500ms时钟拖动和等待顺序保留，图片使用owner的完整PNG与明确派生半尺寸引用，不伪造raw或覆盖历史shot.png。

确定式步骤ID与共享回执防重复：完整调用只返回旧结果（owner退出后也可读），partial不重走导航，丢响应仅查原ID后停止。新label沿既有目录元数据查冲突，失败不当空；finish限制当前latest begun label，历史/partial由Web/generic动作显式接续。pf-node session-status --json新增可选结构化错误，只有原服务端not_found+404才作缺请求，泛404/5xx/网络/坏响应分别保留，不改默认人类输出或action行为。真实owner先写completed回执后写ready，客户端对相同live身份executing短暂只读重试；不等待measuring/恢复或采用新控制权。输出scene_comparison=not_evaluated，不制造未核实的恢复/画质布尔结论。

Python/bash/help检查通过；1既有CLI用例扩展并仅跑1次通过，回执注明无独立raw log、未为补证据重跑。纯客户端fixture一次通过5case/9次真实薄脚本调用：正常local begin→remote finish保留真实等待、6输入各freshcapture、一次completed/ready间隙、完整重复；epoch中断+partial重复；丢响应同ID查询；泛404零POST；profile不符仅capture零输入。总18动作POST/71读取，98已知client/fake-tool PID全退。没有运行PF、ADB、owner、网络或lease；未重复C1像素/供电/恢复链，旧B1/B2负租约未动。qa/gray-quality-client-{host-summary,cli-validation,source-review,publish}.json与host-20261023T075952944642原件保全部stdout/stderr/requests。无部署、真机或数据迁移。

PF [14d9c30](https://github.com/HGamey/phonefarm/commit/14d9c304feb63a75a06ad0f8eaf8675da16826d3)经code-only120138570930（run39869058863）；harness [8d44d09](https://github.com/HGamey/harness-ops/commit/8d44d0960c926bb9a0b2ffd3003167a45c7a1420)经code-only120135626045（run39868193086）与protocol3.11/3.14 120135625631/120135625744（run39868192952）全部completed/success后，同SHA直推main并核远端。没有PR。

勾C2C2与限定C2C，继续C2D旧reference/test_loadop/挂层入口的实际ownership迁移。C2/03B4/03/JOB-EXEC-03/M3与整体goal仍active，未调用update_goal。最新接续入口已压缩，历史INIT-PROMPT完整留本机单独文件，HANDOFF/conversation继续保留批次记录。


## 2026-10-23 参考采集连接已有会话（PRODUCER-RECOVERY-03B4C2D1）

REF_ARM=1从raw ADB/重启/独立perf改为显式既有passdump owner中的capture→有限sample→marker。要求原图几何profile与供电期望，不推断世界场景或内部极高画质；不自动stop。默认保留40秒输入（3×12秒swipe）和30秒trace，WL/CAPDUR提供默认值、CLI优先；LEAD固定6秒，其他值在查询前拒绝。新供电窗口沿现有sample，不伪造旧power.json或summary。完整重复读取三旧回执，partial/丢响应只查原ID、不重采。quality/reference抽取共享CLI传输和调用检查，游戏导航仍在quality。

AST/bash/help静态检查与唯一纯客户端fixture通过：5reference+1受影响quality回归，共9调用、12动作/64只读，85已记录PID全退出。覆盖旧默认时序、完整/partial重复、sample丢响应、epoch/geometry/policy拒绝；无PF/ADB/网络/owner/lease、Cargo构建或旧核心链重跑。qa/gray-reference-client-{host-summary,source-review,publish}.json及host-20261023T081747665442/result.json留本机原件；没有失败/重跑，B1/B2负租约未动。

PF [bf56025](https://github.com/HGamey/phonefarm/commit/bf560254e1fcc9e0b46f7aa0dd1f82013c8191de)经actual code-only120145441975（run39871097174）completed/success后，同SHA直推main并ls-remote核对。harness本批未修改/未重复检查。只勾D1；D2四臂refbench与D3旧挂层仍待。网页原语已可手动完成，但整组幂等/epoch/profile/power断言缺口CTRL-06独立实施，不能提前称完全等价。总goal仍active，未调用update_goal。


## 2026-10-23 网页参考组与CLI共享回执（CTRL-06）

任务详情新增折叠参考采集：显式profile/固定供电期待/有限input，只有三固定gr-label回执均明确not_found才执行capture/sample/marker。整个调用冻结session/actor/epoch，逐步检查预算/几何/最新图；重复完整组从旧回执读参数，owner关闭后仍可查询。partial/断线不自动续跑；POST不确定仅查原ID一次后停止。页面隐藏/离开/切换任务停止后续提交，已发送动作由owner收尾；整组不自动stop，无浏览器持久reference ledger，无新HTTP协议。

定向JS一次通过；单次390浅/1440深浏览器流程无溢出/console错误，最终3POST完整采集，closed重复与partial零POST。首次旧UI fixture capture回执缺result.image，被正确阻断，保留1capture校准日志，仅修新fixture再继续，产品源码未改、JS未重跑。qa/gray-reference-ui-{validation,source-review,publish}.json、http-log、校准日志和3截图留本机；两个fixture服务及tab已清理。没有PF/ADB/设备/部署或真实指标验证。

harness [114d867](https://github.com/HGamey/harness-ops/commit/114d867b708f920b234dc8e6500fa9355d0d3cd9)经actual code-only120149890095(run39872448995)、protocol3.11/3.14 120149889819/120149889370(run39872449017)全部completed/success后同SHA直推main并ls-remote核对。只勾CTRL-06；D2四臂宿主验证仍在进行，D3待，整体goal active未update_goal。


## 2026-10-23 四臂 refbench 与网页任务模板（PRODUCER-RECOVERY-03B4C2D2）

四臂改为单 Lease + 既有 Session：A/B 核验未安装层，C/D 才安装并在首次启动传完整字符串 extras。原件、自报完成、自然退出、改写预期、恢复分别记录；D 的创建期 effective 可非空，但实际 begins 应为零。超时后的已知清理不冒充自然退出，恢复命令只接续原 context、不重跑。网页模板只填现有任务参数，经原队列提交；未确认请求不能被模板覆盖。没有新增 owner/worker 或指标推断。

签名构建与6相关单测通过；唯一 actualPF + fakeADB 三链/四调用通过：正常四臂446传输，首臂超时193，恢复读回失败434，同context恢复71，共1144；1151已知PID退出，未留本批lease。B1/B2有意缺natural的隔离负租约保持。模板仅一次定向DOM/fetch验证范围、参数边界、只填不提交、未决请求保护，不重复控制会话整链。无失败/重跑。原件见本机qa/gray-refbench-{host-summary,unit-results,source-review,publish}.json、host-cli-20261023T083058472862/result.json及gray-refbench-ui-{validation,publish}.json。

refbench稀疏checkout另用git show/git grep HEAD核对当前producer/contract：新增可选sr_pipeline画质采集默认off，不改变bw_pingpong合同；不等于安装APK已核验。详见GRAY-REFBENCH-SESSION。无部署、真机、迁移或FPS/功耗收益结论。

PF [4282d09](https://github.com/HGamey/phonefarm/commit/4282d09637ecacec8a89f5faed4563a891991257)经code-only120151184048（run39872841344）；harness [53bb414](https://github.com/HGamey/harness-ops/commit/53bb414ac6cf0824bcc84f21145156121d717c4c)经code-only120153821146（run39873642064）、protocol3.11/3.14 120153821323/120153821157（run39873642041）全部completed/success后，同SHA直推main并核对远端。仅勾D2，D3待发布，整体goal继续active。


## 2026-10-23 旧裸挂层入口退役（PRODUCER-RECOVERY-03B4C2D3）

旧 probe/loadop/passdump/frameaudit/frameaudit-ts/target 在 gate/lease/ADB 前返回结构化 retired（exit2），指向显式持续会话。移除无调用 mount_mode、安装/启动/setprops 私链；共享 control/refbench 准入、adb_stdout、层路径与规则不变，typed status/off 和历史原件保留。shell不再raw ADB或全盘删SO/settings，只拒绝或带显式serial转typed CLI。旧keep-prop仅安装不启动语义不能静默变为新session启动。

签名main/unit构建、4相关单测与唯一11项host拦截一次通过：8 actualPF（6 retired/2 unsupported）、3 shell（1 retired/2 fakePF透传）；真实gate/ADB=0、无lease，11已知PID均退出。没有失败或重跑D2/旧控制链。qa/gray-legacy-mount-{host-summary,unit-results,source-review,publish}.json及host-cli-20261023T084417524768原始结果留本机。源码删除并非真机能力退役验收，未部署。

PF [f7b4012](https://github.com/HGamey/phonefarm/commit/f7b40122e7f5a67507e4cb84355c70128da1e24b)经actual code-only120154703260（run39873906724）completed/success后同SHA直推main并ls-remote核对，无PR。下一项LEASE-IDENTITY-01：普通外层label仍被当身份，需实例绑定并与nested强lease关联；不能只修unknown显示。整体goal保持active。


## 2026-10-23 LEASE-IDENTITY-01：同名不再授予所有权

普通外层队列由PF单一Rust helper记录实例、实际holder启动/宿主启动身份；PF/GOL/run继承并验证同一实例，nested强lease持久引用。父holder退出阻止新借用，已有强owner仍可完成恢复。显式queue退役要求无nested且bridge可独占，只出元数据审计；旧/坏/不完整记录继续阻断。Web通过真实job_id关联任务，展示outer/device/unknown/recovery事实，不再按label认定正在测量。

PF [8dce700](https://github.com/HGamey/phonefarm/commit/8dce7003725cefdd5cc45e5940b7084db7f90f1b)：code-only120162030846（run39876120980）。GOL [30a0921](https://github.com/HGamey/game_opt_loop/commit/30a0921e29fccbd96b14dd6f1b066a77687003c0)：code-only120162365155（run39876224244），rebase保留同事98fc250。harness [cc47f0e](https://github.com/HGamey/harness-ops/commit/cc47f0e3ce309679c2d7de41b17601d22f6c7406)：code-only120162029055（run39876120487）、protocol3.11/3.14 120162029767/120162029593（run39876120481）。均completed/success后同SHA推main并核远端，无PR。

5PF定向单测、3旧脚本用例一次通过；GOL Guard首次有效lib用例及rebase后同一受影响用例通过，保留错误target/0测试日志。三宿主组一次通过18顶层命令/4身份拒绝，1严格host adapter、0真实设备调用，41已知PID全退出、本轮marker清空；不重跑旧灰层实验链。网页一次静态浏览器流程与定向脚本通过，原图/原始日志留qa/lease-identity-*。Linux实现未由本机这批实跑，旧B1/B2负租约保留。

LEASE-IDENTITY仅关闭自身源码/宿主范围。JOB-EXEC-03、恢复UX、真实重启/脱组后代及端到端验收仍开放；goal经get_goal核实active，未调用update_goal。下一聚焦node作业终态：主命令退出后同组后代仍存活时不能释放调度资源；先冻结窄合同，不把它宣称为任意进程树或整机断电保证。


## 2026-10-23 JOB-FINISH-01：命令退出不提前放行同组资源

现有worker回收直接子进程后，最多0.5秒只读核查原进程组；组仍在/未知时保留真实退出码，发布interrupted/required，既有调度继续保资源。host-none与恢复任务均适用，无关资源继续；不凭历史PGID补发信号。Web详情呈现范围/时间/事实，维护页不再用exit0写“完成”，CLI exit0+未确认组结束返回125。缺字段旧记录明确未提供；setsid脱组及设备恢复不由absent证明。

harness eb3e69dcf407b86b9b08308cd8efc469d85f3794已通过code-only120165766193（run39877260415）与protocol3.11/3.14 120165766110/120165766509（run39877260474）后同SHA推main核远端。5既有worker/CLI测试、1实际宿主回归（host/recovery双路径、8PID全退）、任务详情与维护页定向验证通过；未重跑浏览器或灰层整链，无真机/部署。qa/job-group-*完整索引与负预检记录留本机。

下一JOB-DURABILITY-01：不能只在rename后补fsync；必须避免dispatcher在未持久化终态上先done。先审现有SQLite与worker/meta提交顺序并冻结最小合同。JOB-EXEC-03/主机重启/脱组containment/端到端与整体goal仍开放。


## 2026-10-23 JOB-DURABILITY-01：结果保存确认后才释放资源

把terminal候选与SQL提交分开：worker仍持flock或记录未完成同步时公开finalizing；同步失败转storage_blocked，SQL持久reason并保留声明资源，释放宿主执行名额。显式retry-finalize仅重同步原候选；保存成功后仍按设备恢复义务保留资源，普通release无法绕过存储失败。准备失败/取消有SQL未执行事实，缺目录/meta也能被网页/API列表发现；输入目录和人工放行说明各在对应SQL转换前同步。Web、CLI与API使用同一能力与回包。

harness 8381c19aa86816f20021f00e6457b2d344cfea75通过code-only120173034747（run39879451850）、protocol3.11/3.14 120173033919/120173033777（run39879451702）后同SHA直推main核远端，无新上游冲突。61项本地协议17.514秒全过；3项SafeRoot/2条host故障链及定向DOM通过，原命令计数仍1、无关作业继续、缺meta权限路径可见，原始日志/预期fsync异常留qa/job-durability-*。未部署、真机、真实断电或Doom重跑，不证明任意producer输出持久化。

只关闭JOB-DURABILITY-01声明范围。整体goal经核实继续active，未调用update_goal；用户质疑关goal已记入本机conversation。下一按todo预审LEASE-IDENTITY-02：强lease与恢复上下文复用已实现宿主boot/start身份，避免PID复用推断；queue身份本身不重做。JOB-EXEC-03/containment/端到端与其他待办仍开放。


## 2026-10-23 LEASE-IDENTITY-02：强lease区分宿主进程实例

PF [201719b](https://github.com/HGamey/phonefarm/commit/201719b97321862f3872a29b79bbb9ee24d396e4)与harness [66a532e](https://github.com/HGamey/harness-ops/commit/66a532e5500b2d58aabf14f1303cd3a3354b4ee3)复用已有boot/start读取并扩展强lease、恢复接管及worker核验；同boot换PID实例仍须原组消失，不同boot只排除旧本地进程，pending/设备恢复义务不变。Web呈现七种宿主身份事实，旧/缺/坏记录不伪装成已验证身份。完整合同见[STRONG-LEASE-IDENTITY](STRONG-LEASE-IDENTITY.md)。

11项lease中短命zombie一项首次失败，依据Apple接口参数修复后单项通过，原日志保留；另1项queue和3项命令监督通过。3组实际宿主CLI一次通过22命令、23PID全退、0设备/传输调用；保留pending/required/partial负例。一次DOM验证显示与门禁，不重复整套浏览器或灰层实验。PF code-only120179340708与harness code-only120179343980、protocol3.11/3.14 120179343947/120179344082全success后同SHA推main，无PR/部署/真机/物理重启。

用户再次询问是否关闭goal，get_goal确认active，未调用update_goal。只勾本批子项；JOB-CONTAINMENT-01下一只读收束，完整JOB-EXEC-03/两节点/真实设备和其余计划保持开放。


## 2026-10-23 JOB-CONTAINMENT-01A：显式请求执行范围

harness [fb4dc56](https://github.com/HGamey/harness-ops/commit/fb4dc56375252f175732619c11d959a3f1dfbb52)交付execution_policy准入、/node实际策略能力、worker执行范围及Web/CLI入口。contained未实现时在上传body/输入准备/child启动前拒绝，不静默降级；缺省请求不改旧幂等指纹，未决请求沿原body重试。请求与实际后端分开，旧记录缺证据不回填。只完成协议与有限范围声明，完整containment/JOB-EXEC-03不关闭。

63项协议一次通过17.856秒，DOM一次通过；三组宿主11HTTP/2job/6PID全退，零设备。首次fixture误挡只读lease快照，已保原result/log并按原ID续接，未重放已创建job；只新增未执行的explicit job，实际观察setsid后代仍活时原组已空，回执为unverified。最终SHA（功能5468e0f加末尾空行修正）code-only120184129840、protocol3.11/3.14 120184130255/120184129669全success后直推main；无部署/PR/真实cgroup。qa/job-execution-policy-*保存证据与负结果。

下一只读收束Linux后端的实际权限模型与隔离内核验收：不能把同UID可迁出域当防逃逸保证，也不凭本机Darwin或fixture宣称Linux已支持。goal继续active；本轮未调用update_goal。


## 2026-10-23 JOB-CONTAINMENT-01B：实际内核管理脱组后代

harness [487e5fb](https://github.com/HGamey/harness-ops/commit/487e5fb0076e8a78a7fd4aa34523768a8238264e)增加管理员显式配置的独立Linux特权worker后端。门控入域后才exec，非root身份/NNP/清洁环境；leader退出仍持域及取消能力，域空后封存目录再完成。源码审查修复同UID后续访问旧输出，以及启动门未确认退出却误报not_needed两处问题。普通HTTP node不提权、contained仍不可用。

本机4项worker回归一次通过1.834秒。第一次Linux CI因fixture把tuple和list比较失败，原始日志显示目标权限和入域事实，但不计通过；仅修fixture规范化和早失败PID记录。最终Ubuntu24.04/kernel6.17内核链通过输入/输出、权限与迁出拒绝、setsid后代、leader退出保留、force域空和旧数据封存。4PID全退，账户/域清理完成；qa/job-cgroup-*留本机两次原始证据。code-only120190073130、protocol3.11/3.14 120190072430/120190072703和cgroup-kernel120190072687全success后同SHA推main核远端。

下一JOB-CONTAINMENT-02只读收束生产固定特权启动、跨UID工件/PF控制与worker失联恢复；未部署、未操作手机或真实lease，Doom/迁移继续暂存。总goal仍active，未调用update_goal；只勾01B，不关闭完整containment和JOB-EXEC-03。


## 2026-10-23 JOB-CONTAINMENT-02A：普通节点启动固定特权worker并取回结果

harness [97a8946](https://github.com/HGamey/harness-ops/commit/97a894675b3804648b660d8bbfe66c7e3025426e)交付管理员独立安装、固定 job ID/锁 FD 入口、node-owned 管理 JSON 与域空封存后的输出交还。源码审计补上旧 job 重放拒绝和 runtime 再次强制 contained；公开节点能力仍关闭。具体边界见 [JOB-CONTAINMENT](JOB-CONTAINMENT.md)。

4 项 runtime/13 项 artifact IO 本地回归各跑一次通过。首次 CI 因测试 `/opt` 祖先可被普通用户修改，安装器提前拒绝；保存失败原件，只调整 fixture 专属路径。最终真实非 root 启动链验证锁存续、日志/取消、原件归还、历史访问拒绝和三类入口拒绝，17PID全退，临时账户/域/安装/策略清理。四项 actual checks：code-only120197554622、protocol3.11/3.14 120197554157/120197554223、cgroup-kernel120197553890全success后同SHA直推main核远端。qa/job-privileged-*保存两次原件和验证/发布索引。

只勾02A，完整02与JOB-EXEC-03保持开放。下一只读收束PF实时会话跨UID访问，不把有限作业结果交还冒充运行中producer通用可读；失联恢复另验。本轮未部署、访问手机、改真实lease或迁移数据。goal核实active，未关闭或暂停。


## 2026-10-23 JOB-CONTAINMENT-02B1：跨账号实时PF控制与结果交还

PF [b247ea2](https://github.com/HGamey/phonefarm/commit/b247ea253387f6bcce14acb1bce7f32a37f85f93)、harness [914f711](https://github.com/HGamey/harness-ops/commit/914f711629d757d85622ff5284ba42dd4875a900)已完成固定PF环境、同持久lease根、producer主动共享发布、node实时publisher及封存后历史读取。显式profile也适配普通process_group；旧私有queued输出在启动前拒绝，默认无profile不变。真实Linux链发现并修复目录API误用和越过输出边界同步祖先两处问题，前两次fixture/编译配置错误也保留原件。

3项PF、4项受影响node回归及1条普通profile作业回归通过，PF构建签名通过。最终隔离Linux使用真实PF和synthetic transport，capture/同ID查询/stop/历史查询通过，1次截图958字节，活lease到最终clear；194已知PID全退、账户/域/安装/策略清理。actual code-only、protocol3.11/3.14、cgroup-kernel、pf-shared-kernel全部success后同SHA直推main，未部署或碰手机。详见[JOB-CONTAINMENT](JOB-CONTAINMENT.md)及本机qa/pf-shared-*。

只勾02B1；完整02和goal保持开放。下一02B2先处理停充记录与封存来源恢复，02B3单列失联/重启域与输出交还；public contained仍不可用。用户再次询问关闭goal，实际get_goal核实active，未调用update_goal，未请求关闭或暂停。


## 2026-10-23 JOB-CONTAINMENT-02B3：失联执行域异步核查与输出交还

harness [c61dae9](https://github.com/HGamey/harness-ops/commit/c61dae9d4d27f6c570fb9b243b8f97a16513761c)交付固定reconcile入口、持久boot/domain/handoff与UID复用门禁、Web/CLI异步入口及共用的会话恢复/队列释放资格。普通核查不杀进程，强制操作单独显式确认；不重放、不修改原失联/失败及设备恢复义务，旧安装不误报新能力。

本地69协议首轮68过；新跨boot fixture因macOS路径别名提前拒绝，修正fixture后2项相关回归通过，失败原件保留。定向DOM/CLI和双版本CI通过。真实Linux单链验证worker失联留下setsid后代、普通核查和同UID启动阻断、force域空及输出私有交还、下一作业拒读历史；3job中2执行1拒绝，22PID全退，临时账户/域/安装/策略清理。全部actual required checks及两个kernel任务success后同SHA推main核远端；无部署、真机、物理重启或迁移，public contained仍false。详见[合同与证据](JOB-CONTAINMENT.md)。

只勾02B3的源码/隔离宿主子项。完整02、JOB-EXEC-03、失败恢复子链接续等仍开放。下一回到既定多机管理FLEET-01/02，之后游戏分发与结果索引，避免宿主边界扩展吞掉业务主线。

## 2026-10-24 FLEET-DIRECTORY-01：网页与CLI共享多机目录

harness [1cbc7b1](https://github.com/HGamey/harness-ops/commit/1cbc7b1e43a6a942322609f1a1a618accf24f7e9)已直推main。网页和`pf-fleet --hub`共用持久登记、私有只写凭据、稳定node_id绑定、revision更新、启停和离线快照。目录刷新只读远端宿主缓存，设备采样单独显式请求；远端任务链接指向实际快照来源。修复FleetState遗漏固定PF环境的问题。

76项本地协议25.107秒一次通过，包含两个隔离HTTP节点；定向DOM、390px和桌面浏览器通过。首轮CI只有旧restart fixture出现ready日志先于running落盘的竞态，失败原件保留，仅修测试等待已发布状态，定向1项通过。最终5项actual checks全部success后推同SHA。详见[FLEET-DIRECTORY](FLEET-DIRECTORY.md)。无部署/手机/迁移，旧Doom数据留本机。

只勾目录子项，完整FLEET与总体goal仍开放。get_goal核实active，未调用update_goal；本批不关闭或暂停目标。下一批回收BASE-03 APK缓存成果并冻结游戏bundle/准备/分发合同，Web与CLI共同交付。


## 2026-10-24 DIST-PREPARE-01：游戏版本目录与宿主资源准备

harness [12eda9a](https://github.com/HGamey/harness-ops/commit/12eda9ae41c72cfcd5432d363cda4f64c40a99a9)已直推main并核远端。不可变版本清单、小型冻结输入和既有持久队列连接为同一流程；公开URL/已结束job产物流式进入SHA缓存，支持续传、校验、损坏隔离、空间预检和原子收据。Web与pf-node games共用目录、准备、原请求找回与报告；当前缓存资格和历史任务阶段分开，设备四状态仍未知。旧fetch_apk共用缓存并移除默认serial、隐式全权限与音量修改；Doom样本只复用5项历史元数据。

83项本地协议31.169秒一次通过，包括实际隔离HTTP/worker/CLI/cache；七项新增聚焦断传/坏缓存/不可变/权限/幂等/重定向。定向DOM和1440暗/390浅实际浏览器替身通过，一次POST 503后同ID找回，无二次提交；第一次DOM仅cwd错误未执行case，原件保留。code-only、protocol双版本和两个kernel检查actual全部success后推同SHA。无部署、真机或大文件下载；UI服务已退，tab关闭、viewport复位。

审查修正：历史failed/verified进度原本可能掩盖当前缓存unknown，改为独立observation/progress。只关闭BASE-03与DIST-PREPARE-01宿主子项，完整DIST继续开放。下一DIST-UPLOAD-01本机大文件流式接收；另记录旧CLI隐式生产目标的迁移缺口FLEET-TARGET-01，不在本批切换后台或搬原件。goal仍active，未调用update_goal。证据与合同见[GAME-DISTRIBUTION](GAME-DISTRIBUTION.md)。


## 2026-10-25 DIST-INSTALL-01

PF `baef293` / harness `fd1ef8f`，六项actual CI全部success后同SHA直推main。限定新装、持久PackageInstaller归属/未知commit屏障、真实aapt2、固定Node队列与Web/CLI已交付；缺元数据工具仍可接续旧恢复。91本地协议35.446秒、真实CLI/HTTP1.469秒、真实PF/SDK+模拟ADB六场景、另Node/worker/PF链及DOM/1440与390浏览器通过。未签名807B fixture只验证元数据，模拟commit不证明真实验签；原未知负例与两次DOM断言失误保留。详细边界/证据见[GAME-INSTALLATION](GAME-INSTALLATION.md)。只勾安装子项，DIST-INITIALIZE-01及完整M2继续，goal保持active。


## 2026-10-25 DIST-INITIALIZE-01

PF `537a62f` / harness `e20758b`，六项actual CI成功后同SHA直推main。新增显式Launcher初始化、目录已就绪不启动、有限停止义务与未知屏障；提取共用Transport，Web/CLI同批。92本地协议36.652秒、3相关Rust测试、实际PF六上下文100传输、旧三关键回归108传输、另HTTP/worker/PF25传输及DOM/390浏览器通过。首次Node fixture相对binary错误留原件，改为绝对路径后通过；无产品修复。四个synthetic未知context保留，全部已知PID退出。详见[GAME-INITIALIZATION](GAME-INITIALIZATION.md)。仅完成本子项，未部署/真机/场景；下一DIST-INPUTS-01，完整todo/plan和goal仍active。

## 2026-10-25 DIST-INPUTS-01

harness dd003401af2bdff6611e6d4baabef3350e75a150已检查成功后同SHA直推main；期间保留维护者e6b9bc4和553fb16新远程功能，发布门发现远端变化后重验最终SHA，没有force或PR。合同JOB-INPUTS；原件、负例和验证索引qa/job-inputs-validation.json留本机。94本地协议首轮93过，唯一新fixture Mac cwd别名错误定向修复，两项2.115秒通过；一轮DOM/390浏览器通过，真实系统剪贴板回读未核。无设备/生产/大素材复制。

仅本子项完成，整体goal active。新合入remote/live审计确认测量占用仍raw截图、长动作release提前放锁两项P1，先修后续回同M2；Doom冷启/复位不能用历史已运行控制台playDemo结果冒充已验。
