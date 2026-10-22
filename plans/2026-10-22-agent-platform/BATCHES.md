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

## GitHub流程与协作记录

用户明确授权管理员取消强制PR。phonefarm、harness-ops、game_opt_loop、refbench规则保留禁止强推、禁止删除及code-only必需检查；本机保存修改前规则到transition/qa。开发提交先进入verify分支，通过后直接推main；不创建PR、不绕过失败检查。

产品决定持续写PRODUCT-CHARTER/DISCUSSION/todo；可获得的用户消息与重要回应写本机conversation.jsonl，明确原话/摘要，不把回顾伪称完整历史导出，不保存聊天凭据到仓库。

下一批聚焦系统：游戏分发合同、持久队列与可恢复进程身份、跨节点注册/索引、研究状态和异步人工请求。前端由具体subagent实现，主Agent负责系统合同与整合。
