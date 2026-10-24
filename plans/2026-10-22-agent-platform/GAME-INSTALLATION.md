# 明确缺席的单 APK 新装

2026-10-25，DIST-INSTALL-01开发合同，接续GAME-DEPLOYMENT。同M2分发链；先源码/隔离宿主/CI，不读取旧Doom APK、不下载工具或碰真实手机。

## 范围与结果

仅Android user0、设备上包括保留记录在内明确不存在的包、单base APK新装。已有安装、升级、split/集合、共享UID、APEX、测试APK、授予运行时权限、启动和存档覆盖都不在首批。成功commit是有意保留安装；不自动uninstall、pm clear或恢复成“未安装”。资源目录未必由安装创建，因此resources、application_access、login、scene保持各自未知，资源部署仍走已有独立步骤。

安装必须显式`-R`并核目标`pm help`具备禁止替换语义及create/write/commit/abandon。AOSP android16-release默认允许replacement，省略`-r`不够；见[PackageManagerShellCommand](https://android.googlesource.com/platform/frameworks/base/+/refs/heads/android16-release/services/core/java/com/android/server/pm/PackageManagerShellCommand.java)。具体设备能力不能从AOSP源码或Android版本猜出。

## 输入与入口

manifest新增可选`installation`：`{schema:"phonefarm/game-install-recipe@1",package,version_code,apk_sha256,apk_asset_name}`。严格字段，选定且唯一的asset必须匹配sha，正数size；固定包名/版本来自真实APK元数据核查，不猜文件名。Doom只新增小元数据示例版本，不修改旧版本。

Node冻结`phonefarm/game-install-request@1`：与资源request同样node_id/game_id/version/serial/mode/recipe/assets，mode=check/apply；只有一个选中APK的缓存路径、size、sha。`POST /games/{id}/versions/{version}/install {node_id,serial,mode}`复用固定`game-deploy --request @in/manifest.json --serial S --out @out --json`和device/game资源。GET版本分别给installation_support/installations，既有deployments只列资源事务。GET `/jobs/{id}/deployment`与recover共用，外层新增operation=install|deploy；安装报告schema `phonefarm/game-installation@1`，mode仍原check/apply，恢复外层mode=recover。CLI增加`games install`，报告与恢复继续用deployment/deployment-recover；Web同等入口，不建新队列或恢复引擎。

PF `game-deploy capabilities`保留resources_v1，新增fresh_single_apk_v1表示实现事务能力，metadata_tool.available/error单列当前工具状态；Node新安装须两者具备才宣告games.install-v1，installation_support.recovery_available只依赖事务能力。缺固定工具应显示可修复能力缺口，不自动下载安装SDK；不关闭已有事务的恢复入口，恢复无需重新读APK元数据。

## APK身份与固定工具

调用管理员固定的AAPT2_BIN（可按明确PATH发现）`aapt2 dump badging`以及`dump xmltree --file AndroidManifest.xml`，取package/version/minSDK/native ABI并拒绝split、uses-split、sharedUserId和测试APK。不实现自己的二进制XML解析，不复用CTS猜文件名fallback。输出有界并留原件，检查源file identity未变。PackageInstaller负责实际验签；本批不再次对整个APK做apksigner/hash全量扫描，不把未执行的证书核查写成verified。接收端实际SHA须匹配既有已验证缓存身份，Android安装成功事实与独立证书审计区别记录。

节点固定worker环境可选加入AAPT2_BIN，必须绝对管理员配置路径。普通进程模式按现有工具发现；shared安装仍走root固定配置，不能让HTTP传工具路径/任意命令。APK元数据失败在首个设备写之前结束。

## 同一lease内的事务

沿现有game-deploy队列/强lease/有界命令/持久intent/远端完成行，不另造设备锁。安装request和资源request版本严格区分，旧session继续可读；安装义务是独立持久字段，remote root固定为`/data/local/tmp/pf-install-<原token>`。报告scope为owned_package_install_transaction_settled。

预检元数据、user0、boot、SDK/ABI、pm能力以及包的installed/retained absence。check无设备写。apply：先hold和intent，再创建新marker子树、push APK、receiver size/SHA；create `-R --user 0 --pkg PKG -S SIZE`并立即保存明确返回的session ID；write指定base.apk；提交前再核缺席，持久commit intent，最后commit。只有明确remote completion与确定成功输出才记录commit确认；警告/非零不当作未安装证明。成功后核package/version/单APK实际SHA，再清本任务stage，有限收据结算。

已知commit前失败可在同一活跃owner中abandon自己明确创建的session，再清拥有stage；create成功但ID丢失、未知write/commit、已提交后身份核验失败都保留required。未知commit不重放，不从当前出现同包同SHA推断归属，也不卸载。跨进程恢复先接管原强lease和持久context：已确认commit仅复核安装并收尾stage；未提交session只有可核当前归属才abandon，无法核实时留required说明，不能用staged-only列表冒充普通session。所有原件与attempt保留，沿已交付Node恢复叶子/资格/显式queue-release规则。

## 验收

少量实际签名PF+fake ADB链：正常新装保留；已有/保留包或缺-R在首写前拒绝；commit返回未知保屏障且恢复不重放/卸载。真实AAPT2用新建微型APK读取（使用已安装SDK；无旧大文件/网络下载），不把metadata通过当实际安装或签名通过。Node一次协议、真实HTTP/worker绑定链，Web/CLI同批；仅新失败或改动触发定向补验。


## 2026-10-25 验证与发布进度

PF提交`baef293bb69d10da6ce528421608a43cfd5a4cc5`，harness提交`fd1ef8fceedfc568149224220be7a6f1d1de5615`，均先推verify分支，PF code-only及harness五项actual CI全部success后同SHA直推main；源码与隔离宿主范围已发布，不等同真机可用。

91项节点协议一次通过（35.446秒）；新增安装HTTP链改为真实CLI install/recover后定向通过（1.469秒），覆盖只要求选中APK缓存、跨操作同ID冲突、缺aapt2仍可恢复、旧请求缓存丢失仍查回、不同family报告隔离。安装/资源历史只扫描一次，无法分类的输入仍留原job链接。

Mac签名release、相关请求/session测试、两条metadata parser及一次application required-split修正定向通过。全新807B未签名APK由已装SDK35生成，真实aapt2输出纠正了旧aapt样例格式；不读取旧Doom APK，不追加ZIP解析器。metadata报告scope仅host_metadata_only；普通APK PackageInstaller负责实际格式/验签，没有独立容器或证书审计。

实际PF与真实aapt2、模拟ADB五上下文通过：正常commit35次transport；已有/保留包各6次、缺-R5次且均首写前拒；未知commit18次且模拟设备实际出现包，恢复0次transport，保留原attempt和required。另定向已知install-write失败30次transport，当前owner abandon一次、commit零次，只清stage，包仍缺席并取得限定收据。独立真实HTTP→worker→签名PF→aapt2→模拟ADB链通过，显式review后才queue-release。未签名fixture的模拟安装不证明真机PackageInstaller验签成功。

Web DOM与1440/390浏览器通过：安装回包丢失→切动作→刷新→同ID查回，仅一次install POST、零设备采样；缺工具时新安装禁用，旧事务恢复资格仍可读。两次DOM断言错误（重复链接文案/续接计数）保留，修正测试后从检查点继续，没有重跑已过前半链；产品没有因此改动。浏览器tab13已关闭、viewport复位、fixture38567退出130，61266不监听。

本机证据根`tasks/developer_transition_20261022/qa/`：`game-installation-validation.json`、各协议/PF/HTTP日志、`game-installation-ui-*`和三份integration目录。未知提交负例`game-installation-integration-20261025T010006812174/unknown-commit/recover-unknown/out/game-deploy`的lease刻意保留，禁止按测试清理。没有生产部署、真实手机操作、旧原件迁移或全平台完成声明。安装不保证外部files父目录，启动/初始化/场景与人工协助继续按todo推进。
