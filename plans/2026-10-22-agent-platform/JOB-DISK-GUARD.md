# JOB-DISK-GUARD-01：运行期磁盘软保护

2026-10-25，源码与隔离宿主/CI已交付（harness757d3ff），未部署/真机。前置为JOB-OWNER-RECORD-FAILURE；归JOB-09。只约束可观察的日志长度与文件系统余量，不是任意producer输出硬配额。

## 请求与兼容

可选闭合对象 `disk_guard={max_log_bytes,min_free_bytes,stop_action}`，两个字节阈值均为1..9007199254740991的整数，布尔/小数非法；action为graceful或escalate。整体省略保持旧canonical/指纹不变，显式null拒绝。能力jobs.disk-guard-v1。声明冻结进原spec/meta/launch；原ID重试不改阈值、不重执行。恢复任务拒绝显式guard，typed recovery和内部准备不继承来源guard。

CLI `--disk-guard FILE` 读取至多4KiB JSON；Web在现预算区域提供同三项。缺能力提交前拒绝，未决请求保存显式值/明确省略/旧未记录三态，旧ID不补新字段。普通源码worker具备能力；固定worker须新安装capabilities声明jobs.disk-guard-v1，向旧prefix提交显式guard在launch写入前拒绝。功能放现job_budget.py，不扩固定MODULES、不热改旧prefix；公共contained仍不可用。

## 采样与停止

原worker保留实际producer日志FD的非继承duplicate及job/out目录FD，按st_dev合并文件系统。日志只用同inode的fstat().st_size逻辑长度；空间只用f_bavail*f_frsize。Popen/contained gate启动前检查一次；日志>=上限、任一文件系统可用量<下限或读取失败时拒绝放行，仍沿原未启动/输出交还义务收尾。

运行中原循环最多每秒一次采样，普通先poll实际child、contained先观察域；已可信结束不补迟到停止。contained leader已退而域仍populated时继续观察。不开线程/代理/目录扫描，不截断或删除日志、产物。空间为共享文件系统余量，不能归因当前job；不保证per-UID quota、别的mount/输出目录或脱组后代，也不承诺系统调用硬时延。

首次触发通过同一OwnerStop请求声明动作：log_limit对应disk_log_limit，disk_space_low对应disk_space_low，observation_failed对应disk_observation_failed。已知人工请求不丢失，升级只增不减，沿原信号计时；graceful仍仅SIGINT。阈值/信号过程中可继续写入，enforcement固定observed_stop_request。记录保存失败沿前置内存监督合同，不依赖写盘成功才停止。

## 记录与终态

execution.disk_guard为schema phonefarm/job-disk-guard@1，包含原三项声明、enforcement、status(not_started/running/triggered/rejected/finished)、started_at、observed_at、finished_at、triggered_at、trigger_reason(null/log_limit/disk_space_low/observation_failed)、observation。

observation初始null，已采样为{status:known|unknown,log_bytes:int|null,filesystems:[{device:int,roles:[job|out|log],available_bytes:int|null}],error:null|{code,message}}。未知不填零；已读数值可保留但整体unknown。公开时刻为epoch，节流和信号计时用monotonic。开始、首次触发、结束才主动保存；正常采样在内存，其他既有保存可能顺带带出最近值，GET只读。不能拿最近保存值冒充实时磁盘状态。

启动前拒绝为rejected，started_at/triggered_at保持null、trigger_reason说明拒绝原因；执行中首次触发为triggered并保留原时刻/原因；结束填finished_at且保留triggered/rejected，不将触发抹成成功。最终采样不再触发停止。实际退出、宿主完成、设备恢复和容量资格仍独立。disk_observation_failed自主请求时outcome interrupted，沿未知125优先；已知日志/空间阈值触发且可信exit0的CLI返回124，真实非零退出码保留。启动前未执行没有假退出码。既有owner_record_error仍保持原规则。

状态事件只投影disk_guard_status/trigger_reason/triggered_at三项，不复制每秒字节观察。旧任务不回填，旧SQL历史不补事件。

## 验收

一次完整本地协议，聚焦闭合声明、原ID和旧字段省略；一条实际小日志worker/HTTP服务退出链，保原日志和退出证据；少量受控statvfs/观测失败注入，不实际填盘。复用既有固定worker真实Linux链覆盖guard观察与新prefix能力，不额外新增长工作负载。390px Web核声明冻结、最近保存观察/unknown、阈值触发与真实退出分离。上一批记录I/O故障链已覆盖的部分不重做。无真机/生产部署/历史迁移；JOB-09任意输出硬限额继续开放。

## 验证与发布

源码757d3ff：唯一完整本地协议141项/79.393秒全部首过。新合并runtime用例以4个worker核日志路径替名后仍按producer同inode触发、预检低空间零child/无假exitcode、实际启动后EIO导致真实exit0但interrupted、自然结束最后空间观察低而不补迟到停止；7个已知PID全退出，无fixture强杀。API合并用例核prebody拒绝、旧canonical省略、原ID冻结、launch传递、可选event投影及固定cap正负。

唯一actual HTTP/CLI链首过：真实--disk-guard提交，HTTP先退出，独立worker约1秒后log_limit、仅INT、真实child exit0、group absent；4234字节原log同inode完整，consumer一次。重启同data，原ID查回、CLI wait124；2服务+worker/child全退出。qa/job-disk-guard-host-20261025T105801185429/summary.json保原始HTTP、meta/launch/log及CLI stdout/stderr。

单390context四项首过，35GET+1合成POST503，原ID/guard跨reload冻结、known/unknown/rejected/旧缺字段分开，pageerror0，仅预期503 console诊断。fixture20280/54720和browser20355退出，root目视unknown截图。Linux原固定worker5秒期限链加入guard大阈值已知观察/安装cap/最终无触发断言；不替换原期限动作，五项actual CI全部成功，同SHA main/lsremote核对。qa/job-disk-guard-{validation,checks,publish,ui-validation}.json为索引。MODULES不扩、旧prefix不热改，旧QA unknown义务保留。
