# 有界 Launcher 初始化尝试

2026-10-25，DIST-INITIALIZE-01 已发布：PF `537a62f53ad7101de546f443837b52722f7c4ebc`、harness `e20758b94ee4f44d84f3881cf0a88ce62563d4aa`。完成范围为源码/隔离宿主/CI，不含手机、生产部署或旧Doom大文件操作。

## 要补通的路径

新装不保证 `/sdcard/Android/data/<package>/files` 存在，而资源部署明确要求该父目录已存在。提供显式初始化尝试，让应用有机会自行建立存储；不能保证任意游戏首次启动必建目录。目录已就绪时直接报告观察结果，不重复启动。应用首次启动产生的配置、数据库或网络副作用有意保留，不承诺回到首次启动之前，不自动clear、卸载、授予权限或mkdir/chown。

## 公共合同

manifest 可选 `initialization` 严格字段 `{schema:"phonefarm/game-initialize-recipe@1",package,version_code,apk_sha256,wait_seconds}`，等待1–30秒，APK身份同已有配方；不接受任意component/extras/命令。Node冻结 `phonefarm/game-initialize-request@1`，共有node_id/game_id/version/serial/mode/recipe/assets，assets为空，mode=check/apply。入口 `POST /games/{id}/versions/{version}/initialize`、CLI `games initialize`、网页同一准备动作选择器；共用game-deploy执行/报告/recover与原队列，不建新workflow。

PF能力 `launcher_initialization_v1:true`，Node `games.initialize-v1`，版本字段 `initialization_support/initializations`；GET job/deployment外层 operation=initialize，报告schema `phonefarm/game-initialization@1`。原deploy/install指纹保持不变；新session schema `phonefarm/game-initialize-session@1`，目录仍out/game-deploy。报告保留公共installed/resources_deployed/application_access/login/scene_ready/recovery，新增storage_initialization/launch/stop事实。目录就绪不是资源已部署、应用解析成功或场景就绪。

## 设备动作与事实

Android user0，单base APK路径/version/SHA精确匹配。记录boot、UID和原PM user0状态，拒共享UID；UID归属从明确user0包列表核验，不按进程名猜。目录观察与run-as UID目录读取分别报告；run-as不可得只表示缺这项证据。主UID进程快照使用可解析数字UID/PID/NAME，所有原输出保留；它不证明shell完整可见性或独立隔离UID进程已退。

全APK SHA仅在初次预检和恢复重新取得lease时各核一次；同lease中的停止前后只复核boot/版本/UID/原APK路径，保留内容校验的原时间。数字UID进程格式是运行时能力门，无法解析就不启动，不声称已对所有Android版本验证。目录轮询等待范围不含启动命令本身与后续停止，各外部命令仍有独立有界超时。

check只读。apply目录已存在且shell可读写时不启动；否则必须确认原user0 stopped=true且观察中无该UID进程。MAIN/LAUNCHER查询必须得到唯一、目标包匹配组件，不猜activity、不monkey回退。启动前持久化launch_attempted与stop_required，强lease hold后只执行一次有界 `am start -W --user 0`。明确完成且Status ok后，在冻结时间内观察父目录，之后总要停止本次目标。无自动点击登录、授权或下载等待。

停止要求明确完成的 `am force-stop --user 0`、相同boot/包身份及PM stopped=true，保留数字UID进程观察；明确仍见目标UID进程则保持required。有限收据scope `owned_launcher_attempt_force_stopped`，`all_target_processes_gone`固定unknown，不称全设备恢复。未发生启动时也记录没有stop义务的依据。目录未创建但停止已确认，应报告needs_assistance/初始化未达标，不当恢复失败；外部Agent只阻断依赖此方向。

未知启动或停止结果、pending写入、宿主传输未知都保屏障，恢复不重放启动。只有持久状态确认启动完成且欠停止、原boot/精确身份仍成立时，恢复才接续停止；已完成停止可补读目录并结算。原attempt不可变，接管失败与context迁移沿已有恢复叶子规则。无全UID终止能力时只能报告上述限定收据，不能升级含义。

## 共享实现与验证

先将已有资源/安装Session重复的ADB定位、journal、完成行和有界命令抽成小型game_deploy_transport；Session保留各自持久intent/义务/恢复决策，不重写原schema或状态。新初始化复用此层与现有Lease/输出/context机制，避免逐动作复制另一套传输引擎。

验证聚焦：正常出现目录并明确停止；目录不出现但停止成功、局部协助；未知启动/停止保留义务且恢复不重启；原先运行/共享UID/不可解析状态首写前拒。共享传输改动只重验相关资源/安装关键既有行为；Node一次协议与一个HTTP/CLI绑定链、网页一次相应流程，失败或源码变更才定向补验。模拟不宣称真机可用。

## 验收与发布

- 92项Node协议测试36.652秒一次通过；包含实际HTTP/CLI和冷缓存初始化。两个初始化纯测试、既有完成行/包身份测试各一次通过。release构建90秒并重新签名。
- 实际PF+封闭ADB替身六上下文、100次传输：正常建目录并停止；再观察已有目录不启动；未建目录但停止成功；未知启动、未知停止各保留屏障且恢复零传输；原先运行、共享UID首写前拒绝。
- 共享Transport只复验三个既有关键场景、108次传输：接收hash错误回退、未知资源push、未知APK commit，原始证据与恢复屏障保留。
- 另一次实际HTTP→worker→签名PF→替身共25次传输，初始化不要求缓存资产，限定收据和显式queue-release一致。首次仅fixture二进制相对路径错误导致能力缺失，已留日志并改为绝对路径；产品源码未修改。
- DOM与390px浏览器通过：一次initialize POST，动作切换/刷新后按原ID找回；needs_assistance且stop.confirmed与unknown/required分别显示，目标全部进程退出仍unknown。无设备采样；fixture进程/端口/tab已清理，viewport恢复。
- PF code-only与harness五项actual check-runs均在上述精确SHA成功，随后同SHA直推main并核远端；无PR/force/rebase。

本机证据位于开发交接的`qa/game-initialization-validation.json`及其索引日志、原件和截图。四个本批synthetic未知上下文（初始化启动/停止、旧资源push、旧安装commit）故意保留；全部已知宿主PID退出，不能把保留的QA屏障当垃圾清除。

本项解决初始化尝试和结果可见性，不保证任意游戏自动建目录，也不完成Doom Launcher配置或场景复位。下一同M2为DIST-INPUTS-01显式小工件引用与JSON绑定，再接场景配方；完整M2与整体goal继续。

## 从报告请求异步协助

游戏页的 needs_assistance 报告提供“起草协助”入口。在研究页选择现有研究和方向后，草稿带入原任务、游戏版本、设备、缺失事项及复核提示；仍使用原研究请求表单保存，需要局部阻断时另给方向添加依赖。

CLI使用 `pf-node games assistance-draft JOB_ID --research RESEARCH_ID --direction DIRECTION_ID`，带显式节点身份，输出可直接交给 `research create request` 的data。对应只读接口为 `GET /jobs/JOB_ID/deployment/assistance-draft?research_id=...&direction_id=...`，网页使用同一接口的dash代理。草稿不创建任务或修改研究。

人回复后，外部Agent按原节点、版本和设备提交适用的新检查，保存新job证据，再决定是否解除该方向依赖；其他方向继续。初始化检查只覆盖目录等实际可查前提，账号登录和场景是否就绪仍使用具体游戏方法。操作例子已写入hgamey-research技能。实现已随harness `0691a71`进入main；本地只做一条HTTP/CLI小冒烟（0.68秒）和一次网页草稿冒烟。同SHA code-only40302384025与node-tests40302383962通过，已推进stable。
