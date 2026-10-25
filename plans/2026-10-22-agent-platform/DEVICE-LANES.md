# JOB-DEVICE-LANES-01：已登记设备的灰层会话并发

2026-10-25冻结；已发布PF dd270457d5038847fa8544d8046b9f00a1e694ad、harness af93a5c8b8b8db91b96f3f898fb9935149cc739d；六项actual required CI通过，同SHA main与远端已核对。仅源码、隔离宿主/CI，无生产升级或真实手机操作。复用DEVICE-IDENTITY、LEASE-DOMAIN、现持久job队列/Session/恢复链，不建第二调度或设备owner。

## 绑定与入口

PF纯宿主 `lease resolve --serial S --json` 在ready layout2与有效enabled精确alias返回闭合、最多4KiB的 `phonefarm/device-binding@1`：schema、domain_id、registry_revision、device_id、serial。Node `GET /node/devices/resolve?serial=S`（dash同源）返回{node_id,binding}。能力devices.lanes-v1表示Node接口存在；实际PF resolve/status仍须核验。未知/disabled/旧布局明确拒绝，不降global或选择另alias。

Node spec可选device_binding；只允许固定phonefarm的gray-layer session、唯一显式匹配serial，不允许exe替换、raw argv注入binding或device_access=none。原spec无字段保留原canonical。新增请求在原intake/admit内核当前PF绑定；同ID先原spec/指纹找回，不因registry后来变化阻断旧查询。worker只对创建session将原绑定序列化成 `--device-binding JSON`；PF取锁后再次核domain/device-root/current revision/ID/alias/enabled，不更新请求或自动重新resolve。

PF仅经灰层parser确认的显式绑定session跳outer queue。继承DEVLOCK_INSTANCE_ID与device作用域冲突拒绝。其余PF/GOL/remote/observation/部署入口继续global。CLI `devices-resolve SERIAL` 输出可直接保存，提交 `--device-binding FILE` 核envelope.node_id和当前pin后只放binding进spec；不接受歧义FILE-or-JSON。Web复用现任务页选择并冻结相同对象，旧任务不回填当前登记。

## 一个根中的实际lease

根ROOT、domain_id、bridge/owner-mutex及配置device FD根不变。永久ROOT/v2/devices目录是旧v2拒绝闸；当前2dbed4c的closed-layout读取会持续拒绝，即使所有lane已结束。首次显式绑定acquire在短准入锁内create并同步，失败也不删除顶层namespace。不另activation journal、域ID或每job环境。原fence继续挡更旧PF。

marker为devices/<SHA256(device_id)>.d；稳定FD在device_lease_dir/by-device-id/<key>.lock，永不unlink。ID大小写不会在宿主大小写不敏感路径碰撞；同ID的USB/WiFi alias同锁。人工错误把一台物理机登记成两个ID无法自动识别，保operator_declared/physical_identity_verified:false。registry@1原件execution_scope=global保持历史字节，当前能力/执行范围另读lease协议，不篡改原件。

设备准入顺序：device_id FD EX→原bridge SH→root owner短锁→try global qmutex；验证global两槽/FIFO为空、目标lane空、其余记录均可归属、原绑定仍有效，然后持久marker。qmutex/owner仅持到发布；不持到设备操作结束。global仍bridge EX，并核全部持久lane记录；outer create、registry CAS、global恢复也同核。FD释放不解除持久未知。登记CAS任何lane残留均busy，未知记录unavailable。

现script在轮到label后、typed create前先qappend；成功才qdrop。typed create对bridge WouldBlock/已识别lane占用返回1，坏结构/IO/无法归属返回2，不按错误文本猜。PF/GOL原循环只重试1；新lane让已有global FIFO优先。timeout/异常的队头按原显式queue管理保留，不把TTL当设备恢复。

## 恢复与状态

scoped marker format2、scope=device、device_binding；global format1保持。集中校验marker格式/身份，guarded transport仍用同一Lease绝对marker/token。Session原资源文件、公开descriptor和job-lease@1收据条件式带binding/lease_scope；不与恢复了哪些资源的recovery.scope混用。

有效binding的A active/abandoned/pending/recovery_required/本地进程未知只挡A与global；B可继续。坏binding/schema/path/缺marker/无法归属/不完整枚举阻全域，读不修复。完整marker无session且无pending/recovery可用原recover-abandoned加精确binding，沿独立idle assertion与宿主身份核验；缺身份/未绑定pending仍需人工核验，不自动清除。

恢复沿原Session及marker绑定选择device_id锁与SH bridge，原context/token/serial/domain/child/group都核；不按新alias重新resolve，不接受恢复请求覆盖设备。忽略global FIFO以避免排队global与A恢复互等，但global owner/全域unknown仍挡。恢复祖先借用/leaf接续、原失败与显式review release都保持；A完成不宣称全场clear。

PF `lease status --json`保留aggregate状态/entries，并一次返回完整 `device_scopes`：schema=phonefarm/device-scopes@1、domain_id、registry_revision、global:{status,error}、queue_waiting:boolean、unknown:boolean、devices:[{device_id,status,error}]。devices包含全部当前登记ID（含disabled），唯一且完整；global只表示实际global owner/义务，不因B持SH而held。FIFO读取未知/无法归属记录提升unknown。lane active核自己的FD及owner实例/heartbeat，不拿他人bridge冒充。

Node只消费同一缓存snapshot，不每job另起PF轮询；一个纯门禁函数用于tick/launch/waiting/release/recovery。新启动需同domain/revision、global与目标clear且FIFO空；原scope release/recovery需同domain/id可识别，忽略无关登记revision、FIFO与其他B，仍保宿主/持久化资格。旧global沿aggregateclear。SQL同一claim事务额外判断声明scope：global与任意device冲突，同domain/id冲突，host-only互不挡；recovery仅借原祖先。显式device资源强制保存，用户自由resources不能删除scope冲突。

## 网页与证据

现jobs模板/普通提交共用绑定选择，默认省略global；选择只读登记/resolve不采手机。新request冻结完整body/原pin，pending保存小型原binding事实，未知回包先查原ID；旧未决正文不自动补新绑定。详情/队列显示声明与实际scope，等待/可review资格直接来自后端，恢复仍空body与固定source/leaf。无另页面/队列。

源冻结后最小验证：少量PF核心单元；一条实际host/fake-transport握手链证A/B重叠、A2 alias排他/global FIFO/原scope恢复，旧v2在空devices namespace拒绝；一条实际NodeHTTP/worker链证SQLclaim/同ID/局部阻断及review；一次Node完整协议（失败后仅受影响）；一轮390选择/未知找回/局部等待/恢复显示。复用Linux现共享链的实际构建，不重做图像/游戏/登记/原控制矩阵。首次失败与全部QA原unknown义务保留。

当前分工：root PF devlease/gray/session/receipt整合与最终验证发布；remote agent registry/domain/outer script；audit agent Node scope/queue/session/API；frontend agent现jobs/fleet及pf-node。最终实现和验收完成前不勾本条，不关闭JOB03/goal。


## 验证记录（2026-10-25）

PF 21项相关单元首次通过（13 lease、4 registry、4 domain）；后续窄审修复了未发布marker但FD仍持有的空闲误报，以及恢复入口忽略坏FIFO/qmutex的边界，最终实际链分别覆盖。签名debug程序通过实际Node HTTP/worker执行A/B并行、A别名排他、global FIFO、A恢复时B继续、原范围review release与旧v2拒绝；601次假ADB调用均核实际child PID/PGID与持久marker，622个记录PID全退，最终两设备与global均clear。原首脚本http名称冲突发生在提交前；第二次脚本收据URL写错发生在并发/恢复之后，保留原件并只续剩余尾段，没有重走A实验。这里全部为宿主替身，不是Android实测。

Node完整协议仅跑一轮：130项中129通过，旧全域恢复等待原因被新scope冲突误投影成resource；只修等待分类，旧断言不改，旧失败项与2项新scope测试再次全部通过（2.364秒）。实际链继续消费最终投影。Web一次390 context通过原ID/绑定跨刷新、A阻断/B可review、恢复继承；39GET+1合成POST，零release/recover/device调用，服务与浏览器已退出。

本地证据索引为qa/job-device-lanes-validation.json、job-device-lanes-ui-validation.json；首失败、原始状态和旧QA未知恢复义务都保留。本批debug构建未传显式构建revision/dirty声明，runtime-info如实为null；验证二进制不冒充最终发布SHA。release程序未替换。Linux既有shared与recovery链另由required CI验收，不等同逐设备真机或生产升级。

发布证据：qa/job-device-lanes-checks.json 与 job-device-lanes-publish.json。Linux protocol 3.11/3.14、cgroup-kernel、实际PF shared/recovery及两仓code-only全部成功；现有GOL3160e41无需改动，读取配置PF给出的queue目录并继续使用全域执行。
