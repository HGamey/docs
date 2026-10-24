# Remote/live 所有权接合修复

2026-10-25。新上游553fb16与既有强lease/观察合同有已确认冲突，静态审计原件在本机qa/upstream-remote-live-audit-20261025.md。源码已实现并提交PF1ce29a1/harness58b21dd，六项actual required CI成功后同SHA直推main并核远端；尚未部署/未真机。

保留operator认证、设备开放策略、HTTP/MCP/控制台和实时页面。改动目标是所有设备传输共用已有实际所有权，而非仅改提示或降低busy采集频率。

## 观察路线

每一张新画面经PF已有observation::run，复用强lease、全局bridge、排队优先、有界worker与未知屏障。screen capture单帧CLI及只读capability供node固定PF路径/profile调用；不启动额外长驻弱锁服务。已存在的screen本地页面继续共用同一capture helper。

Node live首批使用逐帧采样替代未受监管screenrecord/ffmpeg，最多按现有screen节奏采样；busy/blocked/unknown/deferred只保留旧图。不再裸查询ro.serialno、尺寸、熄屏；枚举/选择使用已有fleet缓存，不用标签推断能并发。高帧率视频重新接入前须补可确认终止的远端编码器所有权，本批不把低频截图宣传为15fps视频。

HTTP继续提供帧seq/实际采集时间；保活重传不能变成新帧。状态单独报告采集许可/原因，最后画面与当前状态区分。隐藏页面终止本订阅，其他观看者独立；无人观看不开始新采集，已开始的有界worker继续受owner监管。

## 远程动作路线

PF remote-control owner持实际serial+全局bridge强lease，复用既有Transport的完成尾行、日志和bounded worker；Node只保留operator认证、开放策略和宿主mailbox gateway。未确认owner前不查询尺寸/属性；标签和ignore_global_lock不能绕过所有权。正常应用/UI改变保留，close范围仅remote_control_commands_settled，不声称游戏还原。Node重启扫描持久ref，同一动作固定request_id，未知只查询，不自动重放。

首批Android screenshot/tap/swipe/text/key/start_app/stop_app，Web旧app_start/app_stop映射到PF名。任意shell/安装/卸载/推拉/日志/HDC明确unsupported，补齐各自所有权/恢复后再开放；高帧率video同理。没有隐藏raw fallback。

HTTP保留operator入口及Web CSRF；acquire需request_id，响应直接descriptor+acquisition_id；GET session用session_id或原acquisition_id只读找回。所有动作带session_id/request_id；GET devices/S/requests/ID?session_id=SID查询，image=1只取已完成图。accepted/started或closing用202，不能解释为成功或已交回。首批CLI为pf-remote，显式--url和--token-file，同ID查询和结构化结果，无自动设备动作重试。

## 与主任务冻结的首批接口（2026-10-25，覆盖上文可选措辞）

主任务采纳 PF owner + guarded Transport；本节作为实现对接合同。新增 `start_app`/`stop_app`，保留应用改变的 close scope 不变。root负责 screen/live/node gateway，PF owner与前端分别由subagent实施。

### CLI

```text
phonefarm remote-control owner --serial S --out DIR --ttl-seconds 300 --max-seconds 3600 --json
phonefarm remote-control action --session DIR --request - --json
phonefarm remote-control status --session DIR [--request-id ID] --json
phonefarm remote-control recover --session DIR --serial S --confirm-control-stopped --note TEXT --json
```

owner 的 stdin 为 node lifeline（父进程始终保持 pipe open，不承载动作），stdout 首行是 ready descriptor；node断连/EOF不是正常 close。action/status为宿主 mailbox helper，不调用ADB。owner串行执行Mailbox。

Descriptor：

```json
{"schema":"phonefarm/remote-control@1","session_id":"ID","serial":"S","phase":"ready","live_owner":true,"capabilities":["screenshot","tap","swipe","text","key","start_app","stop_app"],"deadline":"RFC3339","ttl_seconds":300,"last_request_id":null,"recovery":{"required":false,"scope":"remote_control_commands_settled","game_state_restored":false}}
```

`phase`：starting/ready/closing/closed/recovery_required。status必须按已有host boot/start identity返回live_owner；失败未知不能报false。

Request：

```json
{"schema":"phonefarm/remote-control-action@1","session_id":"ID","request_id":"client-stable-id","action":"tap","params":{"x":100,"y":200}}
```

params冻结为 screenshot `{max_side?: int}`；tap `{x,y}`；swipe `{x1,y1,x2,y2,ms}`；text `{text}`；key `{key}`；start_app `{package,activity?:string}`；stop_app `{package}`。另有纯控制动作 renew/release，params为空。输入数值与文本沿用现有harness约束并在PF再次验证；不接受shell命令。Activity省略时只运行固定resolve-activity+am start组合；已知返回码失败是failed而非unknown。每个动作固定超时，swipe最大10秒；客户端不能传任意timeout扩大边界。

Receipt：

```json
{"schema":"phonefarm/remote-control-action-result@1","session_id":"ID","request_id":"client-stable-id","sequence":1,"status":"completed","result":{"exit":0},"error":null}
```

status为accepted/started/completed/failed/unknown/not_executed；查询无该ID返回明确not_found。截图result额外含relative_path、mime、width、height、bytes、captured_at，gateway只读这个session里的regular file。没有result的started在owner死亡后为unknown；已落result优先。重复request id同内容返回原记录，异内容conflict；规范化请求不加实现hash。action helper可立即accepted，node短等status或202返回request_id，任何HTTP超时不得重放输入。

renew更新owner本身的heartbeat；release冻结后续新设备请求并有序关闭，回复accepted/closing，只有PF闭合回执+lease释放后status才closed。TTL/最长时长/授权撤回为有序close原因；EOF/node死强制recovery_required，即使当前无设备动作，也留下可核的异常关闭记录。已接受release在状态查询立即显示closing；发布闸门内原子领取/标记started，release优先于尚未领取的请求，后者not_executed。

正常close保存immutable close receipt→确认无pending/unknown且全部started有结果→complete_recovery(`remote_control_commands_settled`, `device_command_completion`, evidence)→退出。刻意保留用户应用/UI改变，不能自作主张force-stop应用。start_app只确认启动命令已结束，不声称游戏进程已结束。所有未知分支sticky保留，后续动作只能查状态/人工恢复。

### 可实际操作的恢复边界

普通恢复：local/admin运行上述recover命令。note必须非空，明确记录独立核验原会话device transport已结束；这不要求关闭用户故意启动的游戏。内部acquire_recovery核原context/serial、旧owner和记录PG已结束；恢复报告绑定原token/session/last_request，`origin=operator_assertion`、`device_verified=false`、`game_state_restored=false`。保存并fsync报告后complete_recovery。未提供确认或note、原owner仍活、PG未知、context不匹配均拒绝。此入口不暴露给普通operator的remote HTTP/MCP release。

现有unbound pending启动窗口有专门缺口：`acquire_recovery`拒绝command_pending；要避免永久无可用恢复，另加**仅管理员显式**的同一入口选项 `--participants-stopped`，并在命令help明确要求独立停止/核验该原会话所有可能本地launcher/worker/transport和设备命令。它仍须验证原owner身份已结束、marker/context/serial匹配，先保存完整原marker与operator note，再通过一个远程合同专用的Lease恢复方法转移并退休pending。不能复用一般acquire或修改普通acquire_recovery的保守默认；不能简单remove目录。若无法提供这项全参与者核验则保持blocked，且错误中给出该明确管理员入口。

这是一项显式扩展：现有PF没有该pending管理员恢复API。其证明来源是人工独立核验，不是PID消失或TTL；收据必须保留此事实。实现时应给原marker/schema/host身份不匹配、缺confirm、缺participants-stopped、缺note、旧owner仍活添加拒绝测试。普通已绑定child的恢复继续使用现有acquire_recovery，不走新例外。若本批不接受此例外，必须明示启动pending需要现有系统级人工reconciliation，不能宣称所有crash恢复均已交付。


## 验证计划

冻结后仅运行受影响关键链：真PF锁+封闭fake transport下busy/unknown零新设备调用；正常单帧与旧帧重传；长action中release/expiry不交出owner；unknown/node死亡后持久屏障仍在；既有Node协议和required CI。网页一轮定向DOM/浏览器覆盖时间/暂停/未决/释放。无真机、生产部署或旧Doom素材操作。

## 已发现并修正中的启动窗口

已知busy/readiness拒绝仅在实际未取得lease、未调用设备的分支持久startup not_started证据；Node核原owner已退出后可闭合该acquisition。不是所有缺文件都算未启动。

另有acquire成功至session/hold落盘前的崩溃窗：普通abandoned恢复之后，Node原ref仍可能未知。本批补PF launch.json（在acquire前绑定context/serial/原host进程身份）、status及现有local recover中的startup retirement。原owner已结束、无动作/transport执行记录、管理员独立核验且同域fresh明确clear才保存人工退休收据；该分支不清任何lease，original_outcome保持unknown。缺launch旧记录不自动猜或补身份。Web/CLI区分人工闭合未知启动与明确not_started；不能把两者都显示未执行。定向单测及真实PF/admin/Node入口的建模启动上下文已验收；没有对该极窄时序窗做定时故障注入。

## 已发布验证范围

Node协议103项40.173秒通过；PF共7个唯一关键测试通过（包含新增启动退休；release原测试随两次实际故障窄修重验）。实际PF/HTTP/CLI封闭fakeADB首轮发现release后status仍ready，第二轮发现尚未started的tap在release前执行，两轮失败原件保留。修复后r3签名release正常链通过：强域别名互斥、观察暂缓零传输、原请求只读取图、同ID不重放、交回期间持锁、排队动作not_executed、交回后单帧恢复；6个已知transport PID全退。独立negative链已通过：远端尾行缺失保留unknown及恢复屏障、明确admin恢复；owner SIGKILL下guarded worker退出但结果仍unknown，原ID可查且新动作拒绝。两处故意保留的恢复屏障继续留存。无真机。

CLI已受理后响应正文断线仍保原ID/unknown且仅一次POST；无首帧断开订阅在一次有限等待后退出，之后解除忙态零新增采集，定向实际HTTP验证通过。DOM与脚本syntax通过；CUA工具两次超时原件保留。随后使用已有Playwright1.60+匹配headless Chromium1223，独立390px context实测控制流程通过：acquire/screenshot/tap/release各一次写，原ID接续、未知不重放、closing不冒充closed、状态错误禁操作，7状态无横向溢出，pageerror为0。5张截图和HTTP收据留qa；browser/context关闭、fixture56538退出、56717端口关闭。live seq/年龄/隐藏仍由DOM覆盖，不冒充实际OS隐藏测量。

证据索引：本机qa/remote-ownership-validation.json、remote-ownership-ui-validation.json、remote-ownership-publish.json。当前仅Android有限动作和逐帧观看；高帧率视频、复杂动作/HDC另列todo。
