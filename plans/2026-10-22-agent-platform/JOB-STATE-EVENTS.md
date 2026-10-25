# JOB-STATE-EVENTS-01 — 作业提交状态与增量事件

2026-10-25。已发布 harness `ed133f816ce2a01f0125c690437024f73fdca283`；属于JOB04，事件表示Node已持久提交/观察的状态，不是无损worker执行历史。

## 范围与持久化

能力 `jobs.state-events-v1`。原job队列SQLite新增jobs.status_revision（旧0）、status_projection（旧null）、status_committed_at（旧null）；单行持久stream_id和job_events表。事件字段：sequence全流递增、event_id随机历史锚、job_id、revision、kind、at、committed_status；unique(job_id,revision)。kind为admitted/queue_transition/worker_observation/baseline。已存在行第一次发生现有写入或dispatcher观察时用baseline；历史done可保持0，不扫描历史文件补造阶段。

唯一发布函数在原SQL事务重读当前行、直接比较小对象；变化时一起更新projection、revision+1、at并insert事件。时间不参与相等比较，无对象hash。reserve/ready/claim/cancel_pending/record_unstarted/set_state/release_capacity共用；重复无变化不增长，rollback不留下事件。新reserve初始admitted，其后queue_transition；旧0行首次baseline。

committed_status是闭合平面对象：queue_state、storage_kind、storage_stage、host_capacity_state，以及phase、outcome、exit、lost、recovery_status、cancel_mode、cancel_status、cancel_signals（已记录signal名称列表）、time_budget_status、time_budget_exceeded_at、owned_scope、owned_status、reconciliation_state。后13项来自最后提交worker观察；尚无观察为null（signals空数组）。SQL明确未启动事实可给phase=finished/outcome/exit=null/lost=false/recovery_status=not_needed。storage只记录kind/stage，不复制异常正文。owned取contained cgroup或原进程组已报告scope/status。缺值不按当前配置回填。

worker仍写现有meta/cancel/exitcode，不写SQL/新JSONL，不改固定安装权限。原dispatcher在intake锁内从现记录观察运行中的claimed；终态只走现finalize，runner flock与关键记录fsync完成后在同一SQL事务提交终态、容量事实和projection/event。若候选已terminal但尚未取得runner或完成保存，不预发普通成功事件。storage故障发布queue/storage事实，保原执行观察；retry可补提交。cancel/reconcile写入口安排现dispatcher；reconcile callback原finalize提交。无新后台监控。

同服务/不同服务SQL写者在事务中重读最新projection；观察读取与提交在既有intake串行路径，不能拿锁前旧候选覆盖新提交。claim本身不等于程序启动。HTTP离线时worker可能完成多个阶段，重启只补现观察；短任务可从claimed直达终态。事件不是每个signal的独立送达。原事故/lost与设备恢复义务保持。

## 状态读取与API

job/request找回/列表/队列项增加 `status_revision`、`status_revision_scope:"committed_job_state"`、`status_committed_at`、`committed_status`（projection）。SQL旧行0/null；未入SQL旧目录null/null，读取不补行/事件。当前worker_alive、lease、waiting、controls、release资格与完整旧响应仍是实时观察，不受revision覆盖；不能因revision相同跳过刷新或核权限。

去掉legacy _execution_status的两处GET写meta，保同形只读推导，不把GET变迁移/发事件入口。日志保原byte offset，列表cursor、event cursor和日志offset独立；本批不扩日志替换/截断保证。

`GET /job-events?job_id=ID&cursor=C&limit=100`，job_id可省略看全节点，limit1..200。首次无cursor从流首开始；支持after=0或显式非负位置，与cursor互斥。要求显式node pin，read-only/API/Web同权限，不读工件/运行lease/创建job。

返回schema_version=1,node_id,stream_id,job_id,items,head,next_after,next_cursor,has_more。item同事件字段。opaque cursor复用encode/decode，绑定 `{v:1,node_id,stream_id,job_id,after,event_id}`；短SQL读事务核scope/stream/head/历史锚，scope/stream/history改变409并保原cursor，不自动归零。空页不推进，head为全流现有上界不是消费确认；过滤job时锚必须是该job事件。after无cursor也核存在位置/范围。未知job_id返回404。旧legacy无SQL记录不支持事件，不伪造历史。

GET只读不写已读、revision或事件。无自动裁剪/ack/重放。首批即时有界分页，不引入longpoll或跨数据库全局流。

## 客户端与验证

CLI `pf-node job-events [--job ID] [--cursor C|--after N] [--limit N]`，单次GET/完整JSON/attempts1，使用当前显式Target、不自动分页/归零；不借follow的Ctrl-C取消语义。旧wait/follow/124/125保持。Web详情显示Node已提交revision/时间/字段范围与折叠事件区，每次一页/最多200 DOM；浏览cursor按目标node+job隔离，刷新可接续；切target不串用。相同revision仍绘制当前状态，事件不自动改控制资格/草稿。

一轮本地协议，失败定向修。原SQL并发测试加revision/event一致性及无变化不增；一个HTTP/CLI集中case验证pin/read-only/页游标重启、旧记录及GET只读；复用actual host重启链验证worker在服务离线结束后只补提交终态，日志offset独立。既有durability链加fsync阻断/重试事件断言。单390接续/旧记录/未知状态，既有LinuxCI仍运行。不操作设备/生产/PF/GOL，不关父JOB04其他身份输入要求。


## 已完成验证

137项本地协议首次全过，73.051s；同一SQL准入并发只发一条事件，事件insert故障回滚状态/revision，重复无变化不增版。原durability链核worker仍持锁/保存失败时不预发普通成功，retry后状态、释放、revision/event一致；所有已知PID退出。legacy两种只读推导不改原metadata，旧SQL0/无SQL null不补造历史。新HTTP/CLI集中case验证显式pin、只读权限、scope/stream/history锚和单页读取。

实际单job宿主链首次通过：HTTP退出前v5，worker离线完成后SQL仍v5；同根重启提交v6唯一终态，原cursor接续，日志按原字节offset拼接，命令只执行一次。4已知PID全部退出，0device。原件在 `qa/job-state-events-host-20261025T101131527425/summary.json`。

单390px浏览器4步首过，41HTTP全GET、其中事件4次，0POST/device；同revision动态unknown更新、截断后保原cursor、重载显式续读、job隔离与旧null/0均通过。服务13619/49238和browser13672已退出。最终[CI40149727456](https://github.com/HGamey/harness-ops/actions/runs/40149727456)两Python、真实cgroup及PF shared/recovery和code-only五项actual required全成功；同SHA main和ls-remote已核。未部署或真机，父JOB04其余完整输入/身份范围继续开放。

本地索引 `tasks/developer_transition_20261022/qa/job-state-events-{validation,checks,publish,ui-validation}.json`。
