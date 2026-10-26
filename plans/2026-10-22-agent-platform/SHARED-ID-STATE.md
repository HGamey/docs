# 共享身份、状态与接续合同

BASE-06于2026-10-26归并已有实现及并发验证。以下表格是当前接续依据，各对象保留自己的ID和revision范围；不要求重命名schema或增加重复数据库。完成基础身份/状态合同，不代表物理设备epoch、跨节点执行或生产接续已验。

| 对象 | 可接续身份 | 状态与并发边界 |
|---|---|---|
| node | 数据根持久node_id；目录名是本地别名，URL、host和service instance单列 | 目录revision CAS与请求pin；身份读取不探设备。见[FLEET-TARGET](FLEET-TARGET.md)、[FLEET-DIRECTORY](FLEET-DIRECTORY.md) |
| device | `(node_id,device_id)`；执行绑定另含domain_id、registry_revision、serial | 登记CAS和历史绑定；旧未绑定任务仍global。行政登记不证明物理身份/当前连接epoch。见[DEVICE-IDENTITY](DEVICE-IDENTITY.md)、[DEVICE-LANES](DEVICE-LANES.md) |
| study/research | `(node_id,kind=research,id,revision)` | operation_id幂等优先于revision CAS；目标、限制和判据保留历史 |
| direction | `(node_id,kind=direction,id,revision)`，所属research固定 | 等待原因、依赖、优先级、证据和结论由外部Agent维护；CAS不是Agent领取锁 |
| experiment | 当前以`(node_id,direction_id,direction_revision)`定位冻结的next_experiment计划，再查其中保存的request_id及job/result引用 | 计划引用研究revision；没有独立experiment_id，也没有计划到job的跨库原子绑定。未保存的客户端ID无法凭聊天之外的状态找回 |
| job | `(node_id,job_id)`；request_id是另一个提交幂等键 | 完整规范请求及body相同返回原job，冲突409；队列领取原子化。status_revision只覆盖已提交状态投影，日志offset、事件cursor和实时owner/lease独立 |
| artifact/result | 原件位置为`(node_id,job_id,relative_path)`；注册结果为`(node_id,result_id)` | result不可变，保存登记时来源观察；当前原件变化/缺失单列。路径不是内容身份，只有已有SHA才表示内容校验 |
| human-request/feedback | `(node_id,kind,id,revision)`；反馈引用原request revision | 请求CAS；反馈追加不可变，纠正使用新ID/supersedes。关闭请求或收到答复不会自动解除方向依赖 |

研究/方向/实验计划/协助的具体字段和流程见[RESEARCH-STATE](RESEARCH-STATE.md)、[RESEARCH-HARNESS](RESEARCH-HARNESS.md)。作业见[JOB-QUEUE](JOB-QUEUE.md)、[JOB-STATE-EVENTS](JOB-STATE-EVENTS.md)，原件见[JOB-INPUTS](JOB-INPUTS.md)、[RESULT-INDEX](RESULT-INDEX.md)。不同节点的相同局部ID不代表同一对象；历史来源还必须携带来源修订，不能回落到当前节点的同名job。

direction=verified是作者声明。exit0、宿主进程退出、关键记录持久化、手机恢复完成与实验有收益各有自己的证据。不能相互替代，也不能因某个revision没有变化跳过实时安全资格检查。

既有验收包含研究CAS单赢家/原operation重放/依赖环回滚、重启后历史与迟到反馈；队列同request只建一个job、异内容拒绝和跨进程单owner领取；目录CAS、设备登记历史、状态事件重启接续。2026-10-26只读审计复用了这些原始QA，没有为文档归并重跑测试。未关闭FLEET-04/CONTROL-07的设备epoch能力账本、JOB-10/DEPLOY的实机与部署验收。

机器可读的不可变实验计划→规范作业绑定，可在后续明确需要自动查询该关系时另行增强。当前轻量harness必须在提交前保存原request ID，提交后显式关联job/result，并在断线后先查原ID；不自动猜测哪条作业属于哪项实验。
