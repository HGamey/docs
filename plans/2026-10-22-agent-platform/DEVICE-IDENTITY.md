# FLEET-DEVICE-IDENTITY-01：稳定设备登记（JOB-03 前置）

2026-10-25 冻结；源码与隔离验收已通过，发布状态见文末。源码/隔离宿主/CI，不接触真实手机或运行节点。设备并发仍不可用；下一批在此基础上升级单一锁域、队列和恢复屏障。

## 本批行为

PF 在已配置的唯一 lease 根持久管理 device registry。Node 不另存一份，不把目录节点、远程控制 policy.aliases 或自动探测结果当作物理同机证据。稳定身份为 (node_id, device_id)；登记明确是 operator_declared，physical_identity_verified=false。连接别名为 PF 使用的原始 serial 字符串（例如 USB serial、host:port、hdc:serial），精确匹配，不做大小写/前缀/地址猜测。不同别名可以显式属于一个设备；一个别名不能同时属于两个登记，包括已停用登记。

本批只管理登记信息。enabled=false 表示停用该登记，不取消已有任务、不撤销 raw 工具权限，也不宣称设备离线。所有现有作业 canonical、设备选择、执行策略和 lease 排他保持原合同；无登记仍是旧全域执行。旧 job 不回填当前登记。不得因两个不同 serial 就启用并发。

## PF 接口与保存

- `phonefarm lease devices --json`：纯宿主读，不创建目录、不执行 adb/hdc。缺记录是 revision=0 的空登记；损坏、不可读或存储未初始化明确错误，不能返回空成功。
- `phonefarm lease devices-set --request FILE|- --json`：单条创建/更新，输入最多64KiB，闭合字段 `{expected_revision, device:{device_id,label,aliases,enabled}}`。所有字段必填；expected_revision 为0至2^53-1整数，device_id 为1–64位字母数字开头并只含字母数字下划线短横线，label 为1–128个无控制字符，aliases 为1–32个不重复、1–256字节无空白/控制字符的serial字符串，enabled为布尔值。最多128个设备，整个当前记录最多64KiB。
- 返回 `schema: phonefarm/device-registry@1, revision, devices, updated_at, identity_basis: operator_declared, physical_identity_verified:false, execution_scope:global`。devices 各项包含上述四字段，按 device_id 排序。设备 ID 不删除；通过 enabled 停用，更新同 ID 保留身份。移除别名和显式重分配只修改新版本，历史不改；后续作业必须冻结并重新核对原绑定，不能解释旧记录为新绑定。
- 写入必须 expected_revision CAS 匹配；持原稳定 bridge 排他FD后取 queue-owner 短锁，再尝试创建原 qmutex（存在立即拒绝、不等不偷）；在短事务内确认原两owner槽均不存在及外层队列空，再保存并仅释放自己创建的 qmutex。不持锁等待设备，不清理未知/恢复记录。读取不授予执行权限。写入返回新的登记；同内容且匹配revision可返回原版本。revision冲突/忙不能自动重试。
- PF 保留已发布版本原件，原子发布当前记录；版本历史与current一致性由同一模块负责，不增加内容hash。中断/读取未知必须报错，不覆盖旧版本。当前 `device-registry.json` 本身即最新已发布原件；CAS更新先将旧current原字节create-once归档至 `device-registry-history/<旧revision>.json`（已有须字节一致），fsync后原子替换current。归档后中断仍读旧current，重复归档无害；最新版本直到下次更新才需历史副本，不引入双对象的新版本提交窗口。
- 失败单行JSON `schema:phonefarm/device-registry-error@1, code:invalid_request|conflict|busy|unavailable, error`，退出2。CLI解析/存储失败均如此，不将stderr当成功JSON。

## Node、CLI、网页

- GET `/node/devices` 和 `/dash/api/node/devices`：read权限，返回同一PF登记并加node_id。
- POST 同路径：admin/steer+CSRF、目标node pin、最多64KiB，转交同一devices-set请求，单次有界调用；不产生job。400 invalid_request，409 conflict/busy，503 unavailable。Node不在Python重新实现CAS/alias唯一性，也不改队列。
- 使用配置PF程序、cwd和pf_env；调用不继承PF_RUNTIME_REPORT或PF_JOB_*，有界时间/输出；未知旧PF协议明确不可用。capability `devices.registry-v1` 只声明Node API，不代表配置PF/物理核验/并发能力；GET失败保持错误。
- `pf-node devices` / `pf-node devices-set FILE`（`-`可stdin）：读/显式写同级；写要求目标pin，单POST，不自动重试；响应丢失提示先GET查原状态，保留输入revision。
- 现fleet页面加本节点设备登记区域：显示revision、稳定ID、别名、启停和声明边界，支持创建/编辑；只读无编辑，加载/错误/保存中/冲突有反馈，冲突保留表单并让用户显式重载。保存断线标未确认，不自动重放。远端目录快照不允许误写本节点，原节点入口保留pin。CLI/API与Web同优先。

## 集中验收

先源码冻结，执行一次必要协议整轮；失败只修受影响链。PF复用缓存debug构建并签名，只跑新模块必要纯宿主用例。一个实际PF+隔离HTTP/CLI链覆盖A两个别名+B、重复别名/CAS拒绝、旧版本保留、停用/再启用、持强lease时登记写拒绝且旧记录不变、旧PF不可用；不调用手机工具。网页一次390px覆盖加载、保存反馈、冲突/未确认保留和只读/远端边界。保留第一失败，不重跑旧producer、游戏或完整UI矩阵。

JOB-03并发、旧域fence与物理核验仍待下一批；此项通过不能勾整个FLEET-01/JOB-03。

## 本批验证与范围

126 项 Node 协议首次全过（66.468 秒），一条新 HTTP/CLI 定向首次通过（1.365 秒）；3 项 PF 纯宿主测试首次通过（0.13 秒）。最初构建包装脚本把多个cargo测试目标当成单一目标，编译成功后筛选失败、未执行测试；修正选择器直接复用已构建二进制，未重编或执行无关测试。PF普通debug产物来自同次构建并重新签名。

实际签名 PF / Node HTTP / CLI 链首次通过（1.520 秒）：A两别名+B、占用别名/CAS拒绝、旧版本原字节、停用/再启用、真实强lease期间busy、CLI单次保存/读取及旧PF不可用。18个直接宿主子进程均收尾，lease wrapper的额外子进程自然完成并核组空、最终lease clear；零job/设备调用。已存在QA恢复义务保持原样，没有新恢复屏障。

390px网页主链3次POST覆盖成功、CAS409、提交已保存但回包截断后显式GET找回；只读和远端边界通过、无横溢或pageerror。首轮发现HTML pattern连字符在Chromium v模式无效，修复后第二context只验证输入约束（12GET/0POST），不重跑已发生写入。共44HTTP=41GET+3POST；首轮console/原件保留，预期409和截断与真实pattern缺陷分开。测试浏览器、服务、端口均关闭。

本机QA索引为 `tasks/developer_transition_20261022/qa/device-identity-{validation,ui-validation}.json` 和 `device-identity-host-20261025T083910447545/summary.json`。注册helper若在持qmutex期间被外部强杀，可能留下元数据屏障；不自动清除，需按域空闲核查处理，不能据网络超时推断没写入。当前没有部署/真机/物理身份验证或设备并发收益。

PF `1ce93b8ef9b550aec22f70a79567342620b8a27e`、harness `ae3c7e09baa02f5d5bf0176c51a0ce1e1ac7ef69` 六项 actual required CI 全部成功，同 SHA 直推 main。后续单根布局升级与设备 lane 仍未交付。
