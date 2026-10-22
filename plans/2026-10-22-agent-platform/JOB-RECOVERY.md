# 设备屏障与恢复任务

2026-10-22，JOB-EXEC-03第一批宿主联动已实现，phonefarm cafdbba、harness-ops 2b4747e；完整条目仍在实施。接续JOB-QUEUE，不作为真机通过证明。

## 本批结构

phonefarm增加`lease status --json`宿主只读入口。不连接设备、不创建状态目录、不续期或清理锁。返回schema_version=1、status=clear/held/blocked/unknown、bridge_held、entries与error；entries描述普通/嵌套强lease的serial、label、state、reason与recovery_context，原token/完整marker不外露。损坏、未来格式、缺失身份不能报clear。clear只说明没有观察到宿主屏障，不证明设备已还原。

旧tools/devlock不得覆盖/偷取强marker，不能因owner丢失或心跳过期跳过强屏障。普通外层owner释放保留嵌套屏障，原生reconcile仍可继续。活动嵌套lease下续普通外层心跳仅在已知marker身份、当前bridge锁及相关进程关系下允许，不清恢复字段。

node的`GET /leases`与`/dash/api/leases`只读消费该合同，短缓存避免重复进程；未知/不兼容明确返回unknown。`/queue`包含同一lease状态。写设备任务在状态非clear时保留queued，说明device_lease等待；真正命令入口仍负责持有现有锁，读取快照不是原子取得设备。

## 提交与恢复

spec新增device_access=none/write/recovery。phonefarm/GOL/adb/fetch_apk未声明时保守按write，其余none；none仅用于真实宿主计算/解析，不能将其用于绕开设备所有权。job记录执行范围，旧行按既有bin保守解释。

write任务结束但recovery未报告时保留调度资源，不从exit0推断恢复。blocked只保留声明资源，不再占宿主执行名额，因此无关宿主/设备方向可继续。未知强屏障对旧全局bridge覆盖的设备写入仍保守阻断，不能宣称已实现独立设备并行。

recovery任务须提供recovery_for=一个blocked job ID，继承该任务资源。允许借用目标及其blocked祖先链的资源保留，与同级恢复任务和其他活动任务仍互斥；不跳过phonefarm自身的恢复资格。恢复任务使用普通命令/日志/产物/取消语义，由人或外部Agent填写明确的恢复/诊断命令，平台不猜测还原动作。其结束不自动解除父任务屏障。已确认结束的恢复命令记done，原父任务仍保留；恢复worker丢失或报告required时，这个恢复任务也blocked，不能让另一个同父任务越过。可明确指定这个失败的恢复任务作为下一恢复目标，沿祖先链核查；失败/取消原记录保持不变。

管理员（包括已授权Agent）释放父任务时必须提供note；存在preparing/queued/claimed/blocked恢复子任务时拒绝。write或recovery任务还要求最新lease观察为clear，旧队列放行不能清除真实强屏障。这个说明是调度放行记录，不标成设备验证成功。网页与CLI均展示/提交同一字段。

## 验证与后续

本批验证通过：6项devlease单测、4项实际CLI回归、3项旧桥接隔离测试；47项node协议回归（13.376秒），覆盖升级幂等、lease变化让出调度、恢复worker丢失、祖先链和放行权限。跨仓实际debug binary、真实文件锁与HTTP服务的6项宿主检查通过：write等待而host任务真实运行；释放lease后write无害命令执行exit0，not_reported仍blocked。网页实走准备/提交恢复、原父保留、note不能绕屏障、clear后显式放行；三宽度浅深色无水平溢出。仅临时目录和无害宿主进程，设备传输调用为0，未部署。

CLI新增`pf-node leases`、`--device-access`、`--recovery-for`，显式新参数要求jobs.recovery-v1；旧请求未指定device_access时保持原规范化形状，升级后重试不会因新增默认字段冲突。恢复资源从父任务继承；CLI拒绝额外--resource，API采用父资源。

接续重点：将job身份绑定到强lease记录和可审核的设备执行/恢复收据；统一持久化lease根及重启语义。目前默认DEVLOCK_DIR位于临时目录，不能承诺断电/重启后屏障仍在；只迁移node路径会和旧参与者分叉，必须整体安排。强marker label仍可能是操作名，不能据此猜job ID。

完整设备lease托管、通用设备恢复收据、宿主断电/脱离进程组后代、两节点及实际手机验收仍要继续，不能仅凭本批观察/队列联动勾选完整JOB-EXEC-03或M3。
