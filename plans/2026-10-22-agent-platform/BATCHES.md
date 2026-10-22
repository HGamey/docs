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
