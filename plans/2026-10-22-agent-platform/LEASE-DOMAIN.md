# JOB-DEVICE-DOMAIN-01：同一根目录的版本化锁布局

2026-10-25 冻结；已发布PF2dbed4c / GOL3160e41 / harness7998fa7，七项actual required CI成功后同SHA main。仅源码、隔离宿主与CI，所有执行仍global。此批为下一步设备lane打底，不放宽任何Node调度、远控或恢复资格，不迁移当前运行节点/QA未知恢复记录。

## 不变量与布局

`DEVLOCK_DIR`、`storage.devlock_dir` 始终是原锁域根ROOT；原 `ROOT/phonefarm-bridge.lock`、`ROOT/queue-owner.lock` 及设备FD锁目录保留，绝不unlink替换稳定inode。注册表及其历史留ROOT。新global队列与两strong marker槽统一位于 `ROOT/v2/global`；queue_dir只供状态读取和内部FIFO，不能注入为另一执行域环境。

未登记布局的旧域继续layout1全域语义。显式 `phonefarm lease domain-init --participants-stopped --json` 将空闲域升级；新布局版本2与domain_id持久绑定原ROOT及配置device_lease_dir。执行仍global/exclusive bridge，旧上下文在原v1根继续恢复，不能搬到空v2绕过义务。

ROOT/domain.json为闭合、受限、持久的 `phonefarm/lease-domain@2` 记录；原ROOT/devlock.d改为有界JSON常规文件fence，schema为 `phonefarm/lease-domain-fence@2`，精确绑定同domain_id和ROOT。文件由已同步临时文件hard_link create-once发布并同步ROOT，不能覆盖原owner目录/任何文件。旧PF/脚本见该路径存在且不是旧owner结构会拒绝。更早TTL偷锁或私设每serial根的脚本不在支持范围，必须停用；fence不是防任意绕锁命令的权限隔离。

## 升级与中断

管理员确认停止所有参与者；仅重启HTTP服务不够。升级持原bridge排他和owner短锁，不创建可能遗留的旧qmutex；核原两槽不存在、原qmutex不存在、原queue有界为空。现有unknown/pending/recovery、未知v2目录或记录均拒绝，不移动/清理。

先原子保存domain记录（复用同ID重入），创建并同步合法v2/global，再原子create-once发布fence。匹配record+fence+目录才ready。record已存在但缺/不匹配fence时，新工具报migration_required/unavailable，绝不回退v1；显式同根domain-init可补齐确切属于该记录的缺失步骤。未知文件/extra/不同device-root保留并拒绝；不增加pending journal或猜测成功。初始化成功后再次调用只核同一身份，不新建域、不要求当前活跃v2 owner为空。

每次v2解析还核原root queue为空、无原qmutex/nested槽，避免迁移前已通过旧check的脚本后来在旧目录活动。发布fence使用no-replace：竞态创建的旧owner保留并阻断，不覆盖。旧日志/seen/空queue留原地作为历史，当前队列从新目录开始，不复制实验数据。

## PF内核与脚本

一个 `lease_domain::Paths` 解析器供Storage、queue_owner、devlease及registry使用；读操作不创建。helper每次实际mutation在原owner短锁内重新解析。strong lease取得原bridge后重新解析queue位置，防止等待期间升级导致旧路径缓存。acquire、observe、recovery、confirmed idle、status及队列空闲检查均用同一实际槽，Transport绑定的marker绝对路径来自本次owner。

`__queue-owner check --root ROOT` 新协议 `phonefarm/devlock-check@2`，返回 layout_version、domain_id、queue_dir 和现有状态。新版tools/devlock无论显式/默认ROOT都检查协议，FIFO路径来自queue_dir；创建owner目录移入typed create，在owner mutex+bridge核空后创建。create带内部 `--queue-dir EXPECTED`，只作与最新解析结果的相等校验，不能作为执行root；不匹配拒绝且不建槽。防止script基于旧FIFO决策、helper却写新布局。成功owner/renew/release仍沿原instance@1，不因label借锁；旧script/helper混搭明确拒绝，不回退旧TTL逻辑。

Storage envelope仍schema_version=1，加 `layout_version:1|2, domain_id:null|string, queue_dir:absolute|null`；devlock_dir保留原ROOT，device_lease_dir保留原配置绑定。布局未知/未就绪不提供可执行资格。lease status继续schema_version1与全域clear/held/blocked/unknown；匹配fence是布局事实，不能列为当前owner，坏fence仍unknown。PF fleet只读锁信息用queue_dir；观察worker环境仍传原ROOT。

设备注册CAS同时改用queue_dir检查实际两槽/FIFO/qmutex，仍持原bridge和owner mutex。不得因旧槽变fence让全部登记永久busy，或忽略真实新owner后允许修改。

## Node / GOL / Web消费

Node保留storage_root原义，新增只读queue_dir解析；旧PF未提供layout字段按明确legacy合同读root，新布局缺路径必须unknown。node_fleet读真实队列；remote Occupancy的两槽同目录，按当前PF快照重新解析而不永久缓存迁移前根，错误保持unknown阻断。调度/queue-release仍要求原PF全域clear；worker环境不改变，历史job canonical不变。

GOL执行Guard继续原脚本，serve默认看板目录改为PF queue_dir；显式lock-root只作为历史读取覆盖，不传给PF执行。Node网页同时标明锁域根/当前队列目录/布局，不把queue_dir叫新的设备域；GOL现看板仅有lock_root事实，本批把它明确标为只读队列目录，不猜未提供的域根或扩大Paths结构。Web/CLI可见相同PF事实，不新增另一队列。必要skill说明随实际行为更新。

## 验收与不包含范围

先全部源冻结。最小PF纯宿主测试覆盖空域升级、同ID重入、record无fence的显式补齐、未知/活owner/旧队列/设备根不匹配拒绝，保持原inode与原件。一个实际隔离CLI/脚本链验证新global outer+strong同域互斥、状态/自然收尾、旧PF/script遇fence/协议拒绝；不调用手机工具。一次Node完整协议（新变更），定向覆盖fleet/remote读取真实queue_dir及未知布局不放行；GOL只验受影响resolver/看板。网页只核新增路径事实，不重跑旧注册流程。已有Linux共享内核链继续实际验证，必要调整fixture读取实际marker但保PID/PGID/serial门控断言。

不声明设备lane、多机并发、生产升级、真实重启、游戏恢复或新的权限保证。下一批JOB-DEVICE-LANES-01按注册绑定冻结目标、局部恢复和队列冲突后再启用并发。


## 本批验证记录

PF 22项受影响单元、GOL 1项resolver与binary typecheck首过；Node127协议首次全过66.758s（process66.971s），不重复本地整套。签名PF/typed脚本隔离实际链12事件、13直接进程全部退出，包含v1→v2、嵌套strong、持有时同ID初始化、global排他、登记busy/clear及旧script/PF拒绝，最终leaseclear、原bridge inode/fence保留。新负例不产生恢复义务，旧QAunknown全部保留。

Web390显示33GET/0POST/零设备，初次长标签重叠已局部修正并留截图；GOL skill仅改队列路径含义，现通用validator对旧名game_opt_loop的underscore报错，未改兼容名称。源码提交PF2dbed4c/GOL3160e41已过code-only，harness167a358首轮两版本协议及cgroup成功；pf-shared在Cargo排除栈/文件枚举失败，未到fixture。7998fa7补稀疏检出的根.gitignore，让同CI验证这一最小假设，不清cache或重新取大型资源。后续结果以qa/lease-domain-{validation,checks,publish}.json及最终main提交为准。

最终7998fa7七项所需检查全部成功，同SHA三仓main已核。补根.gitignore后Cargo构建成功，Linux升级域链185次替身传输、旧布局恢复链317次替身传输通过；稳定inode/fence核验通过。首失败原件保留，不重跑本地127协议。执行仍global，设备lane继续下一项，不关闭父项/goal。
