# 可持续操作的灰层会话

状态：2026-10-23，03B4C1 已实现并通过源码、真实宿主 PF + fake ADB + node HTTP 和网页验收；03B4C2A [当前场景采样](GRAY-CONTROL-SAMPLING.md)也完成上述层级验收。尚未部署或在手机验证。有限输入采样与显式供电政策已接入，见 [供电合同](GRAY-CONTROL-POWER.md)；旧入口迁移仍待。本文记录已交付边界和后续范围，提交与验证索引见 [BATCHES](BATCHES.md)。

## 使用结果与现有缺口

Agent 启动一次挂层会话，拿到 job/session ID。断开客户端后，手机仍由同一存活 owner 管理。人可打开该任务看屏、接管有限输入、抓取灰层证据；Agent 可在控制权交回后继续。关闭会话先停止接收新动作、完成当前有界操作，再验证资源恢复。owner 崩溃则保留原上下文，显式恢复，不重复输入。

旧 gray-layer 的 Lease 随一次命令退出而释放，但目标进程内的层仍存活；narrow 只清全局加载属性，不能卸掉已加载层。旧 off 只证明限定停用，并没有原属性值。这些旧入口仍待迁移。node 现有独立 supervised job 的 HTTP 服务重启不会结束 owner；新增的会话通道连接该 owner。另起 screenshot/input job 会被正确挡住，recovery_for 也不能用于连接正在运行的 owner。现有 screen/fleet 是独立观察者，活动会话的截图须经过本会话的动作序列。

## 一个 owner，一条执行通道

新增 phonefarm `gray-layer session`，作为前台持久进程执行；远程放在现有 node job 中，提交仍立即返回 job ID。直接 CLI 可由调用者正常托管该前台进程，不自行另建守护进程或 RPC 服务。它持有真实 Lease、原值快照、安装文件归属和持久 pending，串行处理有限动作。

新增明确的 `gray_layer_control` completion owner，不伪装成 eval/launch/loadop。复用 GraySession 的 prepare、已知文件/属性恢复、受控 transport、原始 PNG 与 marker/audit 读取。安装属性必须按 mode 显式设置，不能继承现有 capture 的 passdump/frameaudit 默认值。probe/target、passdump、loadop、frameaudit、frameaudit-ts 分别保留当前可执行配置；copyprobe/upop 继续在排队和设备访问前拒绝。

正常连接不 acquire_recovery、不借 label、不暴露 Lease token。持续进程死亡后，只有恢复命令可取得原 context；不以反复退出/保留 barrier/接管来模拟普通交互。空闲会话仍占其设备与当前 job 名额，不能伪标 blocked 来腾名额；多设备解除全局串行仍是 JOB-03 的独立待办。

## 第一批 CLI 与节点入口

第一批服务入口：`gray-layer session --mode MODE --package PKG --serial S --out DIR [--max-seconds N] [--power-policy observe|battery] [--actor-id ID] [--so PATH] [--keep-prop] [--evidence-origin device|synthetic]`。package显式给出，不自动进入原神或任何游戏场景。max-seconds必须为正且有上界；默认900秒、最大3600秒，作为资源预算记录；初始actor_id默认为agent。keep-prop只在活owner内保留，并允许narrow，最后恢复原值。

本机客户端入口：`gray-layer session-status --out DIR [--request-id ID] --json`只读宿主事实或指定动作回执；`gray-layer session-action --out DIR --request REQUEST.json|- --json`只投递并立即回执，`-`从stdin读有界JSON，不碰设备、不排队拿另一把锁；`gray-layer session-recover --session DIR --serial S --json`只恢复原owner范围，不重新挂层或执行动作。

node 复用同一 job/out：`GET /jobs/:id/session`、`POST /jobs/:id/session/actions`、`GET /jobs/:id/session/actions/:request_id`；dashboard 提供相同 alias，pf-node 提供对应 status/action 子命令。GET 只读取宿主状态和既有原件；POST 需现有管理 token/掌舵权限。只读账户能看缓存和结果，不能触发新的设备采集。GOL 链接或读取这套状态，不另起设备 owner。

node 只向该 job 固定控制目录写入请求，不能接受客户端给出的任意主机路径。控制 descriptor 须绑定 schema、job_id（本机直接运行时可为空）、session_id、producer、serial、相对 session 路径、当前状态及支持动作；不信任任意产物里出现的路径或命令。现有 SafeRoot/受限原件接口可复用。只在对应会话 owner 的宿主 flock 仍被持有时投递新动作，PID/旧时间戳仅诊断，不能证明存活。

第一批同时交付 Web、API、CLI 和实际 PF producer；不先发布会制造成功错觉的占位控制按钮。

## 持久动作与输入控制权

控制目录位于这次唯一 out 下，session descriptor 与内部 session manifest 分开。一个目录不得被第二个会话覆盖。请求包含 request_id、session_id、actor_id、预期控制 epoch、action 与参数；使用有界 JSON，不接受 shell、任意文件写、任意子命令或上传可执行动作。

固定 `out/session-control.json` 为公开descriptor，`out/gray-control/` 为控制目录。owner持有其中owner.lock的宿主flock；单一Rust publisher短持publisher.lock，分配递增sequence并原子发布不可变请求。先持久更新next_sequence，后发布请求，发布期间一直持锁；崩溃留下的sequence空洞允许跳过。消费者只读完整原子发布文件，按sequence串行处理。request_id仅允许安全短标识，最多1024个请求、单请求最大64KiB；预算达到后仍可正常stop。

node POST调用受限的本机 `phonefarm gray-layer session-action` helper，out只能是该job已确定目录，request通过受限输入传入；不自行写第二套mailbox。helper在宿主完成投递后立即返回JSON，不等设备执行。这样CLI与HTTP共用同一个publisher与幂等实现；node对helper设短超时，超时后用原request_id查询，不另发输入。producer的started与result各自独立原子持久，已有started或result的ID永不重新执行。

公开descriptor的schema固定为 `phonefarm/gray-control@1`，字段为session_id、job_id、serial、package、mode、phase、accepting、controller、allowed_actions、latest_image、recovery、created_at、updated_at、deadline_at。controller包含actor_id、kind(agent/human)、epoch、expires_at；latest_image为空或包含相对out的path、request_id、captured_at、width、height、rotation。recovery包含status、相对session路径与已知报告引用。存活检查结果由status返回live_owner字段，不靠descriptor中的陈旧布尔值。

请求schema为 `phonefarm/gray-control-action@1`，使用request_id、session_id、actor_id、control_epoch、action、params。tap参数为x/y/frame_id，swipe为x1/y1/x2/y2/duration_ms/frame_id，frame_id引用先前capture请求；trigger/audit为go；take_control为kind/ttl_seconds；其余动作params为空对象。回执schema为 `phonefarm/gray-control-action-result@1`，包含session_id、request_id、sequence、status、requested_at、started_at、finished_at、result、error；status为accepted/started/completed/failed/unknown/not_executed。GET与本机CLI使用同一只读helper生成这些状态，避免节点再实现一套死亡/半完成判断。

同 ID 同请求找回原回执；同 ID 不同请求冲突。先持久 accepted，再由唯一 owner 记 started，之后才允许设备命令；结果独立持久 completed/failed/unknown，附操作时间、错误与相对产物引用。HTTP accepted 不能显示为操作完成。owner 死亡或 started 缺结果禁止自动重放；尚未开始的请求在停止时明确 not_executed。重连读原回执，不重新生成 ID 重试输入。

人工接管转移的是本 owner 内的输入控制权，Lease 不转移。take_control/release_control 在当前动作安全边界生效并递增 epoch；旧 epoch 的未执行输入明确拒绝。人类控制有期限（默认60秒、上限300秒，同一控制者take_control可续期而不改变epoch），关闭页面或失联后到期交回原控制者；期限不是用户反馈或研究方向的完成证明。owner继续运行，其他研究方向不等待。共享授权模型下actor_id用于协作归属，不能宣称已实现未来的细粒度多用户权限。

首批动作：capture（有损/无损不得混称，默认原始PNG）、tap、有限 swipe、marker、audit、frameaudit trigger、narrow、take_control、release_control、stop。坐标以该会话最近实际图片尺寸/方向核验；拒绝过期图片或越界坐标，不根据固定机型猜。tap/swipe 都有明确结束，不提供持久按下或任意 exec。FA trigger/audit 只在相应模式支持，回执记录真实 go 和证据，不把旧文件当新采集。

已实现的输入范围：坐标引用最新 capture，最多30秒；执行前再核实际图片尺寸与方向，无法读取方向时不可输入。API swipe 为1至10000毫秒，网页输入表单为1至2000毫秒。普通请求最多1023个，另保留1个 stop 名额；停止在每个动作边界优先处理，未开始的其余请求保留 not_executed 回执。

状态分开 starting/ready/executing/measuring/restoring/closed/recovery_required，公开允许动作与阻断原因。截图、输入和采样共用同一执行序列。测量时暂缓观战采集和输入，保留缓存图的采集时间/年龄。首批提供显式刷新与抓图，不默认持续保存大量视频/PNG；将来的观看流使用有界缓存，正式证据抓取仍另有不可变引用。

## 停止、取消和恢复

stop 可由已授权操作者请求，不要求原输入控制者在线；停止接收新动作，当前有界操作完成后开始恢复。生命周期到达显式预算也走相同路径。SIGINT 对应正常停止请求；强制取消、SIGKILL、命令未知、写前 pending 或恢复读回失败不得伪造成功，保留恢复屏障与原始失败。

正常恢复至少验证本会话目标停止、owned 安装/暂存清理、历史文件恢复与12项原属性读回；后续采样纳入其自身远端进程/trace/输入/功耗义务后才可扩大 scope。报告不声称游戏进度、场景、账号或原前台恢复。独立 recover 校验同 serial/context/owner 与原进程结束，不自动清除未知远端写。

node job 的终态与 PF session 恢复必须区分。现有通用 worker exit0仍是 not_reported，不能凭一份局部 lease receipt 自动解除任意 job 资源。本会话首批沿用现有 fresh lease clear + 核查说明释放流程，网页在关闭完成后提供相同释放入口，Agent 可用 CLI/API 完成，无需等待人类。自动终态需要额外的受限 job 完成合同和全部动作范围证明，另列后续，不暗改通用调度判断。

## 分批交付和验收

03B4C1：实现上述完整交互会话：持续挂层→断线接回→同 owner 看屏/有限输入/灰层采集→stop→恢复失败后同context recover；node API/pf-node/job详情同批交付。精简验收用真实宿主owner、fake ADB和实际HTTP，覆盖重连不重放、并发输入epoch、只读授权、停止读回失败、恢复、另一作业仍被lease挡住。复用既有 Session/lease 单测，不重复全套性能实验。Web按现有任务页规范验收状态、按钮反馈、窄屏与浅深色。

03B4C2：把性能采样作为 manual owner 的子范围，复用 owned sampler/trace/有限输入，不重新拿锁、不重启未知场景、不提前 complete 父会话；观察/输入在采样窗内明确暂停。然后迁移仍可执行的旧 gray-layer 命令及 reference/quality 脚本，保留原实验行为和证据，完成后才能关闭03B4。第一批不把未接的采样动作列为可用，也不把仍存在的旧执行入口称为已完成 ownership 迁移。

其中C2A已经交付纯sleep的当前场景sample，不产生触控/硬件条件改动；新base @3绑定采样资源父，旧C1 @2仍按旧范围恢复。C2B有限输入/供电已完成源码与宿主验收；C2C/D旧客户端迁移继续未完成，不能把子批勾选当整个C2完成。

本设计以可操作闭环为验收，不以单纯挂层成功、保持锁或提前拒绝旧命令代替。03B4C1 两条实际宿主链通过，覆盖重连/超时找回/同 ID 幂等、只读授权、接管旧 epoch 拒绝、有限输入/灰层采集、停止及已知恢复失败后续接。未产生新性能或 Android 恢复证据。

当前全局 lease 汇总保留普通外层 devlock 缺实例身份的 unknown 诊断；已知 nested 恢复义务优先显示 blocked，bridge 与内部 job/serial/context 身份独立呈现。该问题另列 LEASE-IDENTITY-01，不能借相同 label 或本次控制会话成功放宽 clear/准入。
