# 有归属的游戏资源部署

2026-10-24，DIST-DEPLOY-01开发合同。接续宿主缓存和流式上传。PF负责设备事务，harness负责冻结输入、现有Jobs排队与Web/CLI；本批源码和隔离宿主验收，不部署生产、不操作手机或读取旧Doom大文件。

## 本批边界

第一条可执行路线是Android、已安装且精确匹配的单APK引擎，资源写到应用外部files目录下新建的独占子树。不得用install-r、uninstall、pm clear或修改prefs绕过前提。新装/升级、多包、旧存档覆盖及场景启动各有后续条目；不将其默认视为完成。

只复用Doom历史小清单中的package/versionCode/APK SHA和资源逻辑路径。旧清单不可变，新示例使用新version。可部署资源只需recipe选中的资产在当前节点verified；无关APK/save缺源不阻断。资源落盘校验、UID读取检查和真实应用/场景读取是不同事实。

## 清单与冻结输入

游戏manifest可新增可选`deployment`，严格验证：

```
{"schema":"phonefarm/game-resources-recipe@1",
 "package":"com.example.game","version_code":1,"apk_sha256":"64位小写SHA",
 "files":[{"asset_name":"data.pk4","path":"base/data.pk4"}]}
```

package为合法Android包名；version_code为正整数；files非空≤64，资源名必须在manifest中唯一存在。path只能由1–128位字母/数字/点/下划线/横线组成的非空相对分量，不能`.`、`..`、绝对路径、重复或父子文件冲突，保留`.phonefarm-*`名称。没有任意shell/绝对远端路径/权限修改字段。

节点生成≤64KiB `phonefarm/game-deploy-request@1`：`{schema,node_id,game_id,version,serial,mode,recipe,assets}`，mode为check/apply；assets仅包含选中`{name,path,sha256,size}`，path为节点已验证缓存对象的绝对宿主路径。输入只含小JSON，不复制游戏字节到job/in。PF检查普通源文件和size；设备接收端一次SHA校验决定是否可提交，不重复全量本机hash。cache receipt在提交时核验；排队期间丢失/改变由producer报告，不能因历史准备成功强行部署。

## PF设备事务

`phonefarm game-deploy capabilities --json`纯宿主返回schema `phonefarm/game-deploy-capabilities@1`及`resources_v1:true`。`phonefarm game-deploy --request FILE --serial S --out DIR --json`执行check或apply；`phonefarm game-deploy recover --session DIR --serial S --out RECOVERY_OUT --json`仅接续原会话，新输出不覆盖原会话。帮助/能力/恢复不经过普通devlock等待；普通执行沿旧外层队列和强Lease，不造另一锁协议。

先验Android serial（hdc/iOS拒绝）、当前Android user必须0、boot ID、`pm path`的单base APK及实际SHA、dumpsys versionCode和UID、既有`/sdcard/Android/data/<package>/files`父目录。失败在第一写前结束；不主动提权，也不声称已排除管理员预先开启root adbd。不推断root或非root均可访问。check模式只读取，输出实际检查回执。

apply在out/game-deploy持久化上下文；远端根固定派生为上述父目录下`pf-game-<原lease token>`，只接受新目录。不覆盖已有目录/旧文件；先hold_for_recovery，再保存write intent，才经lease_command发有界命令。新根有独占marker；每项资源在新根内创建所需子目录、push临时文件，检查实际size/SHA，再发布到recipe相对路径。当前token不成为可任意传入的删除路径。

仅在原boot、同根marker和限定路径均核实后，恢复可删除本事务新建根；不改原应用、旧存档或其他目录。已知命令失败可回滚并读回根不存在；未知传输/待定写入保留required，不用本地进程消失证明设备已恢复。成功commit有意保留部署目录，确认临时文件不存在，出限定`owned_resource_transaction_settled`收据后complete_recovery；不能写原游戏已恢复。跨boot/陌生目录/坏上下文拒绝自动清理。恢复不重放apply。

UID读取优先尝试实际run-as；不可用返回unknown与原因，不自动root、授权存储或修改owner。即使UID检查通过，也不当作目标应用SELinux域/引擎读取或scene_ready已验证。文件内容和目录都已核验时resources_deployed可以verified，application_access和scene独立保持未知。

报告固定`out/deployment.json`，schema `phonefarm/game-deployment@1`，包含node_id/game_id/version/serial/mode、phase、observed_at、installed、resources_deployed、application_access、login、scene_ready、checks、error、session_path、recovery。各事实为`{status,...}`；phase为checked/preflight_failed/applying/committed/recovery_required/restored/failed。session_path为`game-deploy`或null。恢复在恢复job/out写新报告并引用原上下文，保留旧job原件；成功和失败历史不覆盖。producer接收源session与恢复out分开。

报告的有限事务结果为`recovery.status=verified|required`；`reported_complete`只出现在独立强lease收据中。恢复报告mode保持原check/apply，Node外层mode为recover。installed事实保留原核验时间，恢复不冒充重新核对APK。未知传输不会因为宿主进程已退而自动清理。

## Node / Web / CLI

能力`games.deploy-v1`仅在配置PF确实返回上述能力时提供，不能只按Python源码宣称runtime支持。`POST /games/{id}/versions/{version}/deploy` body `{node_id,serial,mode}`，必需原X-Request-ID，JSON≤64KiB。node_id须匹配；固定PF argv、device_access=write、资源`device:<serial>`与原game键；按既有Jobs去重和查询，不在HTTP里跑ADB。check也按设备互斥排队。

GET版本增加deployment_support和deployments，列相同游戏版本的历史job、冻结target与mode、原报告/原件链接；普通查询不采样设备，不把历史observed_at伪装为实时状态。安装、资源、登录、场景不放进跨设备的全局true。错误保持结构化且说明缺能力/缺缓存/无配方等可修正原因。

`POST /jobs/{id}/deployment/recover {}`使用新request ID，服务从原冻结输入和report派生serial/context；复用recovery_for和现有宿主完成资格，禁止网页/客户端拼路径。恢复失败仍保留每个attempt，接续选择实际阻断资源的恢复子链。成功后的队列释放沿已有release流程和真实lease clear，不从单个scope自动提升为任意job恢复。

恢复资格与登记在原intake锁内完成；同ID先查原任务，避免不同ID同时造同级恢复。GET给出`recovery.target_job_id`，调用者向明确叶子提交。已done恢复不会被忘记而重开原失败上下文；取消且未启动的子任务不妨碍继续。若恢复尚未接管就失败，只在`context_moved:false`且报告source_session匹配冻结argv时继续原上下文；不能把未知迁移状态当未迁移。所有game-deploy执行后的not_reported队列结果（含check和recover）都保留显式核查，未自动提升整个job恢复。

CLI `games deploy ID VERSION --serial S [--check] [--request-id ID]`，`games deployment JOB_ID`，`games deployment-recover JOB_ID --request-id ID`。提交仅一次，不确定先查原request；不自动换ID重写。Web在版本页提供本节点目标选择（设备列表只显式请求）、检查/部署、原请求找回、历史报告/恢复入口。未选设备只显示一条说明；选后紧凑展示四项事实，证据和局限可展开。

## 验收与仍开放的工作

复用现有协议；新增一条actual PF+fake ADB新目录提交/原文件不变链、一条已知写后失败→同context恢复/未知保屏障链。harness真实HTTP/worker集成覆盖冻结输入与重复request，Web/CLI共用同字段。只做必要定向测试，之后一次全node协议与实际CI；PF构建后签名。多GiB压力、真实Android/鸿蒙能力、APK新装升级、存档备份覆盖、启动/复位场景均不能以本批宿主替身替代。

## 2026-10-25 发布与实测边界

PF `f9ba7a6831098873a883ace1dfe06bc33765c624`、harness `ddeaef795f376ade9e4eeb3ae5fdeaf2fe1e0e49`已各在actual required CI成功后同SHA直推main并核远端。PF先rebase另一维护者到`2c8d335`的542个祖先提交，无冲突；构建和验证都基于整合后源码。harness kernel CI固定使用本批PF提交；没有开PR、生产部署或真机操作。

- Node协议一次90项38.988秒通过；窄修后同一HTTP链2.822秒通过，涵盖缓存丢失后同ID找回、异目标冲突、恢复未接管时原context继续、阻断叶子和成功历史不重开。第一次暴露新recovery被普通队列当done的失败、一次3秒能力探针替身超时均保留；仅新部署producer加强review，probe对齐5秒有界超时。
- 实际签名PF二进制两项纯测试通过；fake ADB三上下文验证commit仅新增目录、SHA失败仅回滚新目录、partial push未知保留屏障。复制context拒绝、正确源恢复转入新attempt、旧源随后拒绝均有原件。未知负例的保留lease是故意留下的义务，不能当临时垃圾清掉。
- 另跑真实HTTP→持久队列→独立worker→实际PF→fake ADB：资源verified、scene未知、限定lease收据released/reported_complete，job仍blocked；检查原件后显式queue-release才done。同ID返回原job，旧文件不变。
- Web定向DOM一次通过；实际1440/390无横溢，2次deploy POST（check/apply各1）、1次recover、1次显式替身核查释放、1次设备列表请求。apply模拟503后刷新仍用原ID找回，未重发。修复恢复成功后缓存资格陈旧的问题，仅补验该分支；tab/fixture已关闭。

本地证据位于开发交接目录`qa/game-deployment-{validation,publish}.json`、`game-deployment-http-*.log`、`game-deployment-protocol-1.log`、两套`game-deployment-integration-*/`和`game-deployment-ui-*`。actual CI：PF [code-only](https://github.com/HGamey/phonefarm/actions/runs/40096597506/job/120880051346)；harness [protocol/kernel](https://github.com/HGamey/harness-ops/actions/runs/40096924568)、[code-only](https://github.com/HGamey/harness-ops/actions/runs/40096924566)。只勾DIST-DEPLOY-01子项；完整M2、安装/升级/存档/场景和整体goal仍未完成。
