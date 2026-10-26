# 游戏分发格式与证据范围

2026-10-26按实际源码和已有验收记录整理。登记/缓存、安装、运行与恢复分别报告；此矩阵不以文件扩展名或某个设备后端存在推断支持。


“可登记/缓存”均指清单中的普通字节资产，经 size/SHA、来源与路径校验；不表示识别了包结构、授权可用、设备支持或游戏可运行。所有真机栏只评价本轮开发合同/既有 QA，不能用其他历史手工安装或 Doom 运行记录替代。

| 格式 / 用途 | 登记、缓存 | 正式安装/部署执行 | 恢复语义 | 既有真机证据 |
| --- | --- | --- | --- | --- |
| Android 单个、自包含 base APK | 支持通用资产；installation recipe 选唯一正大小资产并绑定 APK SHA/package/versionCode。文件名和 role 不构成 APK 识别。 | 有实现：aapt2 badging+xmltree 核身份、minSDK/ABI；Android user 0；包须明确缺席（含保留包）；PackageInstaller session + `-R` 禁替换，提交后核实际单 base APK/版本/SHA。不是升级或重装。 | 已知提交前失败只 abandon 本活跃 owner 明确拥有的 session，清本次 stage；commit 未知保留义务，不猜归属、不自动重放/卸载；确认提交的应用有意保留。 | 未验实际手机安装/签名；已有真实 SDK/PF/HTTP/worker + synthetic Android。 |
| split APK 集合（base + config/feature） | 每个文件可独立登记/缓存；这不建立完整集合/依赖/签名一致性。 | 不支持集合选择或多文件 PackageInstaller。PF metadata 拒 split、uses-split、isSplitRequired 等声明；资源部署与初始化也要求已安装包只有一条 base.apk。单独可安装、自包含 base 被当单 APK 处理，不代表支持其可选 split 集合。 | 没有 split 集合事务/恢复。旧单包恢复不能外推。 | 无该 typed 链真机证据。 |
| APKS / XAPK / AAB 等容器 | 可作为一个不透明文件或用户已提供的多项资产缓存。 | 无内置解包、集合选择、设备 ABI/density/language 选包、OBB 关联或容器安装器。把容器选为 installation 资产只会进入单 APK metadata 门禁，不会自动展开。 | 缓存中断可接续；没有容器部署/安装恢复。 | 无。 |
| OBB | 可以登记/缓存完整 OBB 字节。 | 没有 `/sdcard/Android/obb/<package>` 的 typed 安装。把 OBB 选成普通资源，只能写入应用 files 下本次新独占子树；这不等于游戏要求的 OBB 已到位。 | 仅新子树资源事务可结算；无原 OBB 替换/备份/恢复。 | 无标准 OBB 路径真机资格。 |
| loose 文件 / PK4 等资源 | 支持逐文件资产。deployment recipe 选 1–64 项、限定相对路径；不自动解归档。 | 有实现：精确匹配已安装单 APK，要求既有外部 files 父目录可访问，写新 `pf-game-<token>` 子树；接收端 size/SHA 校验后发布。应用 UID 读取与真实引擎读取分别记录。 | 已知失败可核同 boot、marker、限定路径后回滚本次新树；未知传输保留义务；成功目录有意保留。不会删除原游戏树。 | 真实 PF + fake ADB 验证；未验真实手机存储限制、UID/SELinux 或游戏消费。 |
| 存档 / prefs / 私有数据 | 可作为字节资产登记/缓存，也可复制进普通新资源树。 | 无替换原存档、导入数据库/prefs、私有目录修改、UID 修正或快照还原。复制到新树不代表游戏已加载存档。 | 不支持旧存档备份/覆盖/恢复；保留原树是现资源路线的边界。 | 无该功能真机证据。 |
| Doom render-demo 小种子 | 支持两项显式缓存资产；合计 ≤1 MiB；固定 v73 adapter/package/version。 | 有界诊断：引用原成功资源部署报告+session，新运行树放 `.demo` 与兼容脚本，固定 renderer/路径/启动参数，观察后停止。不是任意 savegame/游戏场景适配。 | 复用 initialize 的启动/停止义务；新运行证据与应用配置有意保留，不声称旧应用数据恢复。 | synthetic transport；原真实 demo 宿主文件仍未定位，冷启读取/同状态/性能资格未验。 |
| HAP / Harmony / OpenHarmony | 可登记/缓存不透明 HAP 资产和描述性平台信息。 | games typed API 拒 `hdc:` 目标。没有 HAP metadata、安装身份、模块选择、签名或 typed 恢复。PF 的 HDC 看/动/文件传输和 CTS 显式命令入口，不等于商业 Harmony/HAP 通用安装支持。 | 无正式 HAP 安装恢复；不外推 Android 的 root/权限或恢复能力。 | 本任务没有该 typed 分发链真机证据；nova16 无 root 的用户约束保持。 |
| IPA / iOS 包 | 可登记/缓存不透明字节资产。 | games typed API 拒 `ios:`；iOS 后端只是已有 WDA 的看/动/应用启停，没有 IPA 安装、签名配置、push/pull 或 WDA 自安装。 | 无 IPA 安装/应用数据恢复。 | 无该 typed 分发链证据。 |


## 已发现的入口差异

正式games后端已在新请求拒绝非Android目标；客户端原先仍允许提交而收到400。harness e35dd33使Web/CLI提前使用同一serial规则，旧request/事务读取和基于原回执的恢复保持可用。四个整脚本DOM场景、实际pf-node加隔离HTTP八个场景通过，真实浏览器/设备仍未验；五项必需CI通过，e35dd33已main并包含在本批stable 0f785c1中。

旧CTS的独立自定义安装存在丢退出状态、空输出误报成功及仅凭传输hash跳过安装的实际问题，已在PF1a730dd main/stable修复，见[CTS-INSTALLATION](CTS-INSTALLATION.md)。不能借typed games的SDK/PackageInstaller证据为其背书。Doom现有单APK/PK4路径不要求先扩充全部容器格式；后续适配按明确输入需求推进。
