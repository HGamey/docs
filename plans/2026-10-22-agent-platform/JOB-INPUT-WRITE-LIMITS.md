# 作业输入写入边界

2026-10-25，JOB-INPUT-WRITE-LIMITS-01。源码与隔离验收已发布harness68bd288，未部署。

`/node` 的 `jobs.input-limits-v1` 与 `execution.input_limits` 公开节点固定政策：压缩请求体512 MiB，准备输入文件载荷512 MiB，完整gzip展开流544 MiB，单个PAX/GNU扩展记录1 MiB，tar有效成员数及最终命名空间各4096项，目录深度64，写入空间余量64 MiB。载荷包含tar普通文件、引用原件和真实派生JSON；不是文件系统实际占块、运行期输出或全job配额。

原请求canonical/指纹不加入默认字段。同ID同内容先返回原job，不重新准备或检查新政策；失败原ID也不重放。修正后提交新ID，原件和失败partial保留。

归档使用标准gzip/tar解析，预检完整有界解压流，包括PAX/GNU扩展、padding、尾部及多gzip member；扩展声明大小在标准解析器读取前限1 MiB，避免单次巨大分配。只接受普通文件和目录；拒绝链接、稀疏、特殊文件、绝对/穿越路径、重复成员及file/dir冲突。这明确收紧旧data-filter允许的链接行为。有效路径和typed目标在写入前合并检查；隐式目录计数。第二遍固定原bytes经SafeRoot无覆盖写入，不落额外解压副本或重复hash。目录0700，文件0600或0700（原owner执行位），不恢复其他权限/所有权/时间。

typed输入继续每份8 MiB、原件加派生总32 MiB。小引用一次读取与hash核验后缓存在内存，派生JSON由同一bytes生成；预生成总量通过后才写任何in文件。额外缓存总量不超过32 MiB，不用于大游戏分发。源尺寸/前后身份与SHA均按原合同核查，源不改。

实际in目录FD执行fstatvfs，可用量为f_bavail×f_frsize。存在输入条目时，确定载荷加64 MiB余量须通过；每次载荷写入前再次检查该块加余量。空输入不做空间拒绝，保留诊断/恢复入口。空间检查没有原子预留，不能保证其他进程、inode/配额或元数据写入成功。

独立`input-write-limits.json`（schema `phonefarm/job-input-write-limits@1`）记录pending/ready/failed、limits、declared_bytes、written_bytes、tar_stream_bytes、entries、space、error及时间；status API以`input_preparation`返回，旧job无回填。written_bytes只统计已成功返回的受控write字节；异常系统调用可能留下未计入的partial，绝不当实际占块。ready仍须原sync_tree/queue.ready持久屏障，保存失败沿原durability保留义务。

结构化错误：input_budget_exceeded/413，invalid_input_archive/400，target_conflict/409，input_space_insufficient/507，实际ENOSPC/EDQUOT为input_write_space_failed/507；error中保code/status/message/details。已登记job的准备失败沿原HTTP200返回failed/not_started job与原ID，结构化状态来自回执；回执保存失败明确unknown，不伪造保存成功。HTTP压缩体超限仍在登记前拒绝。Web/CLI读取相同事实，不自动清理/重发。CLI打包字节保持，避免破坏旧请求指纹。

验收只用小真实HTTP/CLI与无害worker：tar+引用+派生总量、同ID政策变化后找回、源不变；小政策阈值和故障注入验证展开/条目/流/空间/写入失败，不填满磁盘、不下载大素材、不操作设备。运行时日志/余量监督、任意producer输出、outcache、游戏缓存清理和研究总预算继续归JOB-09及DIST-09。

## 交付证据

唯一完整本地138协议/73.363秒，137方法通过，helper方法两条纯tar错误码预期在冻结前被载入；修正后3输入+2持久化定向5/5通过3.042秒。保留实际HTTP/CLI、288字节输入/一次执行/原ID政策变化后找回、24字节ENOSPC partial、首次回执发布失败unknown等原始回执，10个已知宿主PID退出。

首CI Python3.11揭示补丁版TarInfo私有reader绕过frombuf检查：本机3.12.14复现后，将限值移至共同_proc_member解码前。3.12/3.14原helper各一次定向通过，最终68bd288五项actual checks全部成功（CI40151184230）后同SHA main。第一失败日志与初SHA43105a6保留，不用重复全套本地测试覆盖负结果。

单390px真实浏览器使用synthetic HTTP：4步首过、28GET、0POST/device；随后新增单metadata限值文案只做静态检查，记录范围不冒充浏览器复测。索引位于本机qa/job-input-write-limits-{validation,checks,publish,ui-validation}.json。无生产部署、实际填盘、大文件或手机实验。
