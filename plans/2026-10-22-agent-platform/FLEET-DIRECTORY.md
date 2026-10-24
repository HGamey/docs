# 多机目录第一批

2026-10-24，实施合同。解决现有 `pf-fleet` 仅有本机配置、网页仅见当前节点的问题。每个 node 可以保存目录，Web 与 CLI 使用同一个 API；本批不指定生产中心、不搬数据、不代理跨节点作业。

## 身份与保存

登记名 `name` 是不可变的 1–64 位小写字母、数字、下划线或连字符，`local` 保留。URL 是连接入口，首次成功读取 `/node` 后绑定持久 `node_id`；后续换 URL 或凭据仍须匹配该 ID，不自动换绑。拒绝重复登记同一个 node_id 或本节点 ID。设备暂以 `(node_id, transport, serial)` 标识连接，物理设备跨宿主身份和 boot/epoch 留给 FLEET-04。

私有 JSON 保存登记、只写凭据、最后成功快照和最近尝试结果，使用文件锁与原子落盘，目录 0700、文件 0600。网络请求在锁外，回写核配置 revision，迟到响应不能覆盖修改或停用。停用保留历史，仅停止目录探测，不取消远程任务。所有公开响应只含 credential_present，不含 token 或凭据文件路径。

## 共用协议

`GET /nodes` 返回 `{schema_version:1, now, local:<NodeRead.identity>, items:[entry]}`。
`GET /nodes/{name}` 返回一个 entry。`?refresh=1` 排队刷新已到期的缓存后立即返回；不等待网络。单条查询可用 `?devices=1` 显式刷新设备，也立即返回。普通 GET 完全只读缓存。最多四个远端请求并行，每条总截止 8 秒；自动刷新最短 30 秒，设备显式刷新不受 30 秒限制但不重复已有请求。

entry 包含 `name,url,tags,note,ssh,goals_dir,enabled,revision,node_id,credential_present,created_at,updated_at,connection,snapshot,stale,refreshing`。ssh/goals_dir 仅保留旧配置元数据，本批不执行。

connection 是 `{status,reachable,attempted_at,last_seen_at,error}`。status 为 unverified、ready、unavailable、unauthorized、incompatible、identity_mismatch、disabled 或 no_credentials。ready 只说明节点协议可达，不代表有手机、空闲或已恢复。

snapshot 为 null 或 `{at,url,identity,fleet,queue}`，只在三段协议和 node_id 全部匹配后整份替换。失败保留原快照并标陈旧，最近尝试和最近成功时间分开。重新查询缓存不重置采样年龄；设备 null 不转成空列表，未知 lease 不当空闲。缓存超过 30 秒、连接失败或停用均标陈旧。

远端依次只读 `/node`、`/fleet?cached=1`、`/queue`；先确认 `fleet.cached-v1` 能力，旧节点明确 incompatible，不能回退到隐式设备采样。显式 devices=1 改用 `/fleet?wait=5`，继续遵守远端唯一 FleetState/lease。HTTP 禁止重定向，错误只记录固定分类，不返回远端响应或请求凭据。

`POST /nodes` 创建 `{name,url,tags?,note?,ssh?,goals_dir?,token?}`，返回 201 entry。`POST /nodes/{name}` 更新 `{revision,...可变字段}`，返回 200 entry；enabled 可停用/启用，revision 冲突或重名返回 409。token 省略保留、null 清除、非空字符串设置；空字符串拒绝。创建及更新仅保存配置，客户端可显式排队刷新。无硬删除和强制换绑入口。

Bearer 只读凭据可查询/刷新；管理凭据可登记。`/dash/api/nodes` 完全同一合同，沿用查看权限与 steer 写权限，不另造鉴权。新增能力 `fleet.directory-v1`、`fleet.cached-v1`。

## 网页与 CLI

`/dash/fleet` 提供当前节点与远端目录、登记/更新/停用、所选节点设备/版本/宿主/存储/队列/lease/恢复。远端 job 链接指向远端 URL；不能落到本地同名 job。切换节点与异步响应隔离。页面可见时每 10 秒读取目录并请求到期刷新，隐藏后停止；远端设备采样由明确按钮触发。凭据输入不回显、不入 URL 或 localStorage；写入不自动重试，超时先重新查询。

`pf-fleet --hub URL` 或 `PF_FLEET_HUB` 进入共用目录模式：nodes、status、show、register、update、disable、enable、import。查询支持 JSON；status/show 的 --refresh 刷目录，show --devices 显式采样。凭据由 PF_NODE_TOKEN(_FILE) 提供 hub 认证，远端凭据以 --token-file 输入；import 显式读取旧 nodes.json 并逐条登记，已有名字报冲突，不覆盖、不删旧文件。未指定 hub 时保留旧 run/migrate/status 行为；hub 模式 run/migrate 明确未支持，不静默回落。

## 验收与剩余范围

使用两个隔离 HTTP 节点验证：登记后两端共用事实、重启保留、只读不能写、离线保留、身份变化拒绝、在途修改不回写、无隐式设备采样。必要协议测试一次，页面宽窄布局和关键交互检查。修复 FleetState 未继承固定 PF profile 环境这一已发现的连接缺口。

本批覆盖 FLEET-01/02/03 的目录部分；跨节点作业分发、设备实例能力、生产部署与真机验收仍开放，不把整项勾完。
