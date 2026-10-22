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
