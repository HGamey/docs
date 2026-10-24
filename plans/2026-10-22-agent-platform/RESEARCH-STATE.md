# RESEARCH-STATE-01：独立研究状态与异步协助

2026-10-25，合同已实现，协议通过，已发布 harness fb9ecf10336c7d24d5ff8ada359e233d342bd5ce，五项actual required检查成功后同SHA直推main并核远端。依据产品准则和只读稿 qa/research-state-next.md。只做源码与隔离宿主/浏览器，不部署或操作真实手机。外部 Agent 自行排序和执行；这里保存元数据，不建立新的调度器，也不读取私有聊天历史。

## 对象与版本

能力 `research.state-v1`。四种 kind：`research`、`direction`、`request`、`feedback`。身份 `(node_id,kind,id)`，id 沿结果索引1–64位安全标识；direction ID在本节点此kind内唯一。每次修改产生不可变revision、事件和操作回执；不同方向独立revision。旧 GOL goal/human.md 入口继续作为旧客户端状态，不自动迁移或当新反馈。

元数据存 `_research/state.sqlite3`：objects保存最新指针，revisions保存完整历史，operations保存幂等请求/回执，events保存单调序列，dependencies保存当前方向的依赖投影用于反向查询和核环；同一个FULL事务完成。没有额外后台进程或JSONL双写。无需对每层对象算hash。

每个data至多64KiB UTF-8 JSON，HTTP写体至多128KiB；title240字符，普通说明16384字符，列表至多64项，actor_label128字符。字段之外的内容拒绝，不静默丢弃。可选文字默认null、列表默认[]；state有下列明确默认值。数字不接NaN/Infinity/bool冒充数字。

### research

`title,objective,game,constraints,budgets,device_requirements,state,state_reason,coverage,evidence_refs`。

- title/objective非空。game为null或`{game_id,game_version}`，字符串或null，仅声明，不触发游戏准备。
- constraints/device_requirements为文字列表。budgets为`{name,limit,unit}`列表，limit有限非负，预算只是声明，不冒充执行硬限额/实际消费。
- state默认active，可选active/idle/paused/closed；非active需非空state_reason，其他可null。closed是作者声明，不据此结束设备任务。
- coverage为`{key,disposition,reason}`列表，key安全唯一标识，disposition为planned/in_progress/not_applicable/blocked/rejected/evidence_available，reason文字或null。

### direction

`research_id,title,hypothesis,priority,priority_reason,prerequisites,cost_expectation,expected_benefit,next_experiment,state,state_reason,dependencies,feedback_decisions,evidence_refs`。

- research_id必须存在且后续不可改；title/hypothesis非空。priority为整数或null（声明排序，不由平台选方向）；其他说明文字可null，prerequisites文字列表。
- state默认proposed，可选proposed/preparing/ready/running/waiting_device/waiting_human/waiting_material/awaiting_evaluation/verified/no_gain/rejected/deferred。state_reason可null；waiting_*/awaiting_evaluation/verified/no_gain/rejected/deferred需理由。running/verified均为作者声明，job实际状态另看原任务。
- next_experiment为null或`{title,plan,research_revision}`；title/plan非空，research_revision是该研究已有正整数版本。计划文字不作为argv执行；旧版本保留旧判据。
- dependencies每项`{id,kind,target_id,target_revision,blocking,reason,resolution}`。id在方向内唯一；kind为request/direction，目标必须同研究且指定历史版本存在。blocking是bool，reason非空；resolution默认null，或`{decision,reason,feedback_ids}`，decision=satisfied/waived且reason非空。feedback_ids仅能属于该request目标；direction依赖不接feedback_ids。
- 未解决且blocking的direction边在同一事务核自引用/环；拒绝则不保存任何部分。依赖投影可用递归SQL核环，不扫描全部大对象。目标更新、完成或收到回答不会自动改resolution。
- feedback_decisions每项`{feedback_id,decision,reason}`，同研究反馈，decision=adopt/defer/reject，理由非空。是研究处理记录，不是设备授权；不自动解除依赖。旧上下文可显式处理，保留版本差异提示。

### request

`research_id,type,title,need,why,resume_hint,context,evidence_refs,supersedes,state,close_reason,due_at`。

- type为guidance/assistance/evaluation；title/need/why非空，resume_hint可null。state默认open，可open/closed；closed需close_reason，open时为null。due_at为有限非负秒时间戳或null；到期只提示，不自动关闭、批准或解阻。
- context为`{research_revision,directions:[{id,revision}]}`，全部必须同研究且历史版本存在；可空directions表示研究整体问题。supersedes为同研究同kind旧ID或null，修正问题用新ID。
- 内容/context/素材创建后不可改，更新只能改state/close_reason（可显式重新打开同一问题）。不允许借关闭动作改写问题。
- evaluation至少含一个result_source证据引用；本批自由文本评价。盲化左右/主观量表/片段工具另列HUMAN-04，不冒充已经完成。
- 受影响方向由当前dependencies反查，避免第二份方向清单漂移。未被显式blocking依赖的指导请求不阻断任何方向。

### feedback

`request_id,request_revision,text,evidence_refs,supersedes`，text非空；所属研究从请求取得。创建后不可更新；修正用新ID+supersedes且必须同请求。request_revision必须存在。多个回复独立追加，不提升request revision、不覆盖其他回答。

允许对关闭/旧版本的问题留下迟到答复；保存提交时request state/revision以及context是否已不同的事实。回答不重开问题、不迁移到superseding request、不自动批准/完成方向/解阻。版本不同仅是上下文变化提示，不等于科学评价自动失效。actor_label来自写入envelope，是作者自报；auth_channel只记录admin_token/dashboard_steer，不能证明自然人姓名。

## 证据引用

evidence_refs为有限typed引用，不接受可执行命令：

- `{kind:"job",node_id,job_id}`：同节点核已有任务，旧/失败/unknown允许。
- `{kind:"result",node_id,result_id}`：同节点已有不可变结果。
- `{kind:"result_source",node_id,result_id,source_name}`：同节点已有登记来源；从结果元数据绑定原sequence/registered_at与source observation，不在保存研究时stat/hash大原件。
- `{kind:"external",node_id,locator}`：明确外部声明，node_id可null；仅文字，不访问远端或宿主路径。

对象data保引用本身；revision旁保存服务端evidence_bindings，注明job_reference/result_reference/content_sha256/registration_bound/unverified/external。只有已有SHA称内容身份，stamp绑定不冒充内容hash。来源变化/缺失不抹掉旧反馈；Web链接原结果详情，由原下载入口拒绝变更字节。结果库本批不可删除，写事务前核其不可变引用足够，不造跨库写事务。

## API、并发与操作回执

统一token API和`/dash/api`镜像；读取沿只读token/查看密码。写入沿admin/steer及显式目标pin，绝不启动任务/释放lease。接口：

| 入口 | 合同 |
|---|---|
| GET /research-state/{kind} | 筛选research_id/state/request_id，limit20/max100，cursor；request_id仅feedback，state不适用feedback |
| GET /research-state/{kind}/{id}?revision=N | 当前或指定不可变版本，current_revision单列 |
| GET /research-state/{kind}/{id}/history | 历史分页，原revision/actor/理由/时间保留 |
| POST /research-state/{kind}/{id} | 下述统一写envelope，创建/更新同入口 |
| GET /research-operations/{operation_id} | 原操作回执；404不证明从未发出，不自动重试 |
| GET /research-events?after=0&limit=100&research_id= | 增量事件，limit最多500，返回items/next_after/has_more；无长轮询/后台通知 |

写envelope：`{schema:"phonefarm/research-write@1",operation_id,base_revision,data,actor_label,reason}`。base_revision=null或省略表示创建，正整数表示CAS更新；更新必须非空reason。data不含ID，路由决定对象身份。feedback仅允许创建。operation_id为预先固定安全ID。

事务先查operation_id：同规范路由+请求返回原回执，异内容409/research_operation_conflict；再核CAS，落后409/research_revision_conflict并返回current_revision。验证对象/依赖→保存新版本/依赖投影/event/回执→commit，幂等必须先于CAS。与结果/任务引用核查分开时也先查已存操作，保证旧引用当前不可读不影响原ID找回。

写成功及幂等201，回执`{schema_version:1,node_id,operation_id,object:{kind,id,revision},event_seq,committed_at}`。对象读返回`{schema_version:1,node_id,kind,id,research_id,revision,sequence,created_at,updated_at,data,audit,evidence_bindings,derived,current_revision}`；audit含actor_label/auth_channel/reason/at。derived含未解依赖、请求反馈计数/受影响方向、当前上下文变化或反馈提交时事实，各事实不改旧data。

列表`{schema_version,node_id,items,next_cursor}`，items为kind/id/research_id/revision/title/state/sequence/updated_at及必要未解/答复计数摘要。cursor绑定node/kind/过滤/首屏事件上界；取该上界前每对象最新revision，后续更新不混进旧页。history沿相同keyset和kind/id绑定。事件响应包含node_id/schema_version，sequence单调不靠wall time防漏；事件仅保存事实，不是执行通知。

## Web、CLI与一条工作流程

CLI `pf-node research list KIND [filters]`、`show KIND ID [--revision N]`、`history KIND ID [paging]`、`create KIND ID FILE [--operation-id ID]`、`update KIND ID FILE --revision N [--operation-id ID] --reason TEXT`、`operation ID`、`events [--after N --research-id ID --limit N]`；FILE为data对象，actor-label可选。共享Target/NoRedirect，写前打印原target/object/operation ID，至多读64KiB且只POST一次；未知先查原operation ID，不自动重发、不替用户合并并发草稿。CLI再次写入需要显式相同operation-id及原内容；Web只有查询未找到回执后才显示显式原操作接续，固定原target/对象/base_revision。页面重载不留草稿，重新填写如与已提交内容不同会409，不悄悄新建ID。

网页`/dash/research`提供研究列表/目标编辑、方向列表/局部依赖/处理反馈、三类协助请求/收件箱/答复/关闭、版本历史与原操作找回。基础文字字段用表单；复杂计划/预算/引用可展开JSON。编辑冻结base_revision，409保草稿和原操作ID，显式读新版本/另开编辑才可合并，不能自动覆盖。收到回答与依赖已处理分开显示。未决存kind/id/operation_id/target/base_revision，不保存完整草稿或素材到浏览器存储。页面只发起一次 fetch，但浏览器传输层可在连接中断时重复同一 POST；操作幂等负责去重，不能把一次 UI 动作声称为网络上必然只有一次请求。

验收流程：外部客户端创建研究及A/B两方向→A声明等待请求，B仍独立可更新/可选→网页对固定原问题回答→CLI读原反馈/事件→Agent显式记录采用/暂缓和依赖resolution→旧版本/原操作重启后可查。这个过程不要求内置GOL或自然人同步在线。

## 最少验收与剩余

两个协议纵向：同revision并发只有一个更新成功/原operation幂等先于CAS/服务重启与核环回滚；A等待不阻B、反馈追加/关闭/到期不自动解阻、旧上下文保留、权限和目标。一次实际390px浏览器主要流程覆盖创建/编辑/冲突草稿/请求回答/原ID找回；不新增手写DOM stub矩阵，不重复设备或旧结果导入。按AGENTS整轮协议一次，失败只修对应链。

本批仅单节点共享研究/基础文本协助。后续仍做Agent参考prompt、事件watch/唤醒、明确实验方法/producer身份贯通、HUMAN-04盲化A/B、多用户真实身份/权限、研究跨节点协作以及部署/真机端到端。不把元数据可行性当设备准入或整个研究已完成。

## 本批实现与验证记录

- `node_research.py` 使用短 SQLite 事务保存四类对象；所有页面/CLI读取同一存储。历史 data、audit、evidence_bindings 不覆盖；当前事实另列 derived/current_revision。
- direction.derived 为 unresolved_dependencies/metadata_blocked。request.derived 为 feedback_count/last_feedback_at/affected_directions/context_changed/overdue；affected_directions 项保存 id/revision/dependency_id/blocking/resolved。feedback.derived 固定 request_state_at_submit/request_revision_at_submit/context_changed_at_submit。research.derived 显示当前 direction_counts。
- 事件项为 sequence/research_id/kind/id/revision/operation_id/at；history.items 是完整对象。列表游标固定首屏上界，不因新修改漏掉旧快照对象。
- 两条真实隔离 HTTP/CLI 纵向首轮通过，2 项/1.881 秒；重建服务后复用同一数据根核历史和原操作。116 项完整协议一轮通过，54.941 秒。证据在本机 `qa/research-state-targeted-1.{log,json}` 和 `qa/research-state-protocol-1.{log,json}`，不重复跑已通过链。
- 测试评价素材是明确的 synthetic/external 登记元数据，身份为 unverified，没有看过图像或声称实际主观评价。全部测试未创建执行任务、未调用 PF/GOL/设备，也未部署或移动历史数据。

- Web 实际 390px 流程完成，源码未因 QA 改动。一次工作流分 3 个隔离 context 接续：首次 QA 用 innerText 读折叠内容失败；第二次连接直接断开触发 Chromium 传输层重试，服务正确去重，QA等待未知态超时；第三次仅补响应头后截断及原ID查询，随后完成历史/事件。总计5次JS研究提交、6次HTTP，4个成功独立操作和1个CAS拒绝；两个答复各1 revision/event。全部原始失败保留，没有重跑创建/B修改。冲突后的“另开草稿”按钮未实点，静读核过，不声称浏览器覆盖。
- 390px无横向溢出，3个context均无pageerror；截断正文产生预期网络错误。两个fixture PID71605/72164退出，50671/50919不监听，所有浏览器context关闭。证据索引 `qa/research-state-ui-validation.json`；root已查看请求页截图。真实用户浏览器、设备与历史媒体未操作。
