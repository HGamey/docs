# DIST-SCENE-01：Doom render demo 有界启动尝试

2026-10-25，已发布PF cbc53e4 / harness 58efc7f；限源码与隔离宿主验收，未部署。衔接 [GAME-INITIALIZATION](GAME-INITIALIZATION.md) 与 [JOB-INPUTS](JOB-INPUTS.md)。本批只做源码、隔离宿主与网页，不访问真机或旧大素材。

## 目的与范围

外部 Agent/网页选择已登记游戏版本、当前 Android serial、一次已完成的资源部署和小种子，提交可查回的有界场景尝试。复用现有 game-deploy job、强 Lease、initialize 的身份/启动/停止/恢复生命周期；不新增队列或第二个大型 Session。

首批精确适配 idTech4A++ v73（package com.karin.idTech4Amm、versionCode 11073，APK SHA由recipe冻结），固定 Q3EMain 的 game=doom3 与程序构造的 command。旧静态 demo 在同 PID17766 内的重复回放只作历史事实；不宣称冷启复现、游戏模拟状态相同或性能实验资格。

## 冻结的接口

Manifest 新增可选 **scene_attempt**；原描述性 scene 字段含义不变。

```json
{
  "schema": "phonefarm/game-scene-recipe@1",
  "adapter": "doom3-v73-render-demo-v1",
  "package": "com.karin.idTech4Amm",
  "version_code": 11073,
  "apk_sha256": "64 lowercase hex",
  "resource_prefix": "doom3",
  "demo_name": "shot_pf_state_001",
  "demo_asset_name": "shot_pf_state_001.demo",
  "compat_asset_name": "weapon_base.script",
  "renderer": {"opengl": "GLES3.0", "multithread": true},
  "wait_seconds": 10
}
```

resource_prefix为空字符串表示原部署根；否则为受限相对目录，无dot/parent/shell字符。其下必须有冻结部署清单中的 base/*.pk4。demo_name只允许1–64 ASCII字母数字下划线短横线，不含扩展名。两个不同asset均来自同manifest缓存，各正size、合计<=1MiB；程序固定放 base/demos/<name>.demo 与 base/script/weapon_base.script。renderer.opengl限 GLES2/GLES3.0，multithread必须boolean；等待1–30秒。无任意console文本/Intent extras/环境参数。

PF请求 schema为 phonefarm/game-scene-request@1，沿 game-deploy --request FILE --serial S --out DIR --json。含与旧请求相同的 node_id/game_id/version/serial/mode、recipe、两项assets，另 deployment={"report_path":"source-deployment.json","state_path":"source-session.json"}。相对证据路径只相对request父目录解析；解析后固定绝对路径，原件只读，各<=128KiB。PF能力 doom_render_scene_v1=true；report schema phonefarm/game-scene@1，产物仍 deployment.json/session_path=game-deploy。session新schema phonefarm/game-scene-session@1；旧initialize @1继续可解码恢复，旧字段不隐式改义。

Node新增 games.scene-v1、POST /games/ID/versions/VERSION/scene（Web同路由经/dash/api），body为：
```json
{"node_id":"NODE","serial":"S","mode":"check","deployment":{
  "report":{"job_id":"12hex","path":"deployment.json","size":1,"sha256":"64hex"},
  "state":{"job_id":"same12hex","path":"game-deploy/session.json","size":1,"sha256":"64hex"}
}}
```
两个source为同节点既有input-reference的source对象，必须同job/上述固定路径。Node构造原有 job-inputs@1 files 两项、空bindings、cwd=input；manifest用两个固定本次输入文件名，不做字符串猜测。复制/校验/宿主已收尾资格复用原输入准备，失败不启动。种子复用内容缓存，不搬APK/PK4。源部署允许同game的旧版本，但node/serial/package/versionCode/APK一致，原report+state都须确认committed且无未结恢复，PF核原独占marker/清单；不同游戏/设备/包拒绝。

CLI：pf-node games scene ID VERSION --serial S --deployment-inputs FILE [--check] [--request-id ID]。FILE正是body.deployment两项source对象；可用已有input-reference生成。网页可输入原部署job ID，通过现有两个input-reference只读端点获取并冻结后显式提交；不得因丢回复生成新ID重放。重复提交/恢复/历史沿现有契约；恢复只接续原Session欠下的停止，不重拷种子/不重放launch。恢复准备不依赖原外部素材仍可获取。

## 设备生命周期与事实

1. 先核本机小种子/两份部署证据的结构与绑定，再取得一次强Lease。核user0、精确单APK、独占UID、boot、原stopped=true且未观察到UID进程，禁止通过隐式force-stop制造前提。
2. 原committed树只读，复用/提取既有inventory与内容核验。每次准入内容核验一次并记录大小；不在列表/状态查询重复hash。原state含资产size/SHA，report单独不足；不调用旧recover充当核验。
3. apply先持久hold/create intent，创建 files/pf-scene-<attempt-token> 新sibling运行根和owner marker，guarded Transport放入两小种子；接收端size/SHA与原子发布。运行根有意保留为证据，不删除任何旧存档/原资源/历史prefs/库。
4. 固定command指定原资源native basepath及新savepath，固定demo播放与本次日志；不自动加map/loadGame/exec来掩盖失败。一次am start -W明确完成后，有界收集本次运行根日志、必要的一张图和可得的新实例/有效配置事实，随后复用原有限停止及PM/UID/boot回读。不通过另一个看屏owner旁路采集。
5. Java dataDir、GL/多线程/surface和native库可受Launcher配置影响。**不要求先root或读取整份私有prefs**；缺配置观测可进行明确的有界诊断apply，结果保持unknown/needs_assistance。只读得到的已知不兼容自定义库/注入配置须明确不属于该adapter，不能当builtin-v73已验证。精确APK不证明实际native库；本次命令路径不是应用写入沙箱。平台传输只写新root，应用自身副作用有意保留、不声称旧树字节绝不变；如声称内容未变须有对应核验。
6. 未启动且已知传输完成的部分新root可有意保留并结算限定scope。已知场景失败仍完成stop后needs_assistance；未知push/launch/stop保持sticky recovery_required。原owner/child/transport不足不能自动清Lease。停止只证明该scope，不证明所有target后代或旧游戏状态还原。

报告保持 installed/resources_deployed/application_access/login/scene_ready/recovery 各对象，另 input、launcher_configuration、runtime_seed、launch、scene_observation、stop。报告 phase可checked/observed/needs_assistance/recovery_required；check只读、不开新root/不启动。scene_ready不能由am start/日志关键词独自变verified；render_demo_loaded须本次输出、路径/输入身份对应，same_displayed_frame、same_game_simulation_state、performance_eligible保持unknown/未主张。截图不得标同状态真值。

## 实现与最少验证

PF由同一个initialize State/Session分流新typed场景，旧@1恢复兼容；scene helper只管场景profile、只读部署证据、新root/seed与观察，Guarded Transport共用。Node沿既有games/deployments、job-inputs与恢复叶子；Web/CLI同合同。

定向宿主链：Rust核错/缺seed与跨设备部署先拒；实际PF/闭合synthetic ADB核来源错误零传输、正常prepare/launch/observe/stop、缺日志但停止完成、未知launch不重放且保屏障。未另造全部push/stop故障矩阵；它们共用持久pending/Transport机制，不把本批验收外推全故障覆盖。共享initialize只回归旧已就绪短路与已有unknown恢复，协议整轮按仓库要求一次。实际浏览器验证显式source引用、同ID找回和分项事实，不将替身当真机。

验收：3个定向Rust、104项本地协议（42.040秒）、一条HTTP/无害worker/CLI冻结输入与恢复链（2.530秒）、定向DOM与一次390px隔离浏览器通过。Web两份引用读取、1次scene POST丢回包后按原ID查询、四阶段无横向溢出且无页面错误；浏览器和fixture已退出。PF release 87秒并签名核验；实际PF三条闭合替身链首次全过，117次ADB传输、242个已知PID全部退出。旧initialize正常/已就绪短路与unknown-launch恢复两上下文43次传输、49个已知PID全部退出。来源错误与未知恢复都为零设备传输；缺日志保留截图、停止完成后needs_assistance。两个未知启动QA屏障仍保留，不因进程退出删除。没有产品修补或测试重跑。

六项actual required checks在同一提交上成功后直接推main并核远端：PF cbc53e480cfbfbbf93e0b15cc5269ffcd9c50625、harness 58efc7f5f582d6bf0c29265f32bfdbe61d6fff9a。证据留本机qa/game-scene-{validation,publish,ui-validation}.json；实际PF原件在game-scene-integration-20261025T051028004292，旧共享回归在game-initialization-integration-20261025T051027195763与051034612403。只关闭本源码/宿主子项，真实Doom场景资格与完整M2仍开放。

真实demo宿主原件未定位，仅有历史63,166B与SHA；compat 2,662B，两种子合计65,828B。旧8,924,016B存档不放入本批输入。真实冷启playDemo/新root引擎读写/独立attempt同画面留后续真机验收；本批不触碰手机补证据。细节来源和未决见本机qa/doom-scene-contract-next.md。
