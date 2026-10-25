# JOB-EXPORT-LIMITS-01：目录下载缓存生成边界

2026-10-25，已发布 harness df79d694f3e711688c48b9aca502161fa203a46d。归JOB-09，承接JOB-DISK-GUARD，约束Node自己执行的目录压缩写入；不改变job spec，不关闭累计缓存配额/任意producer输出硬限额。

## 范围与政策

现GET /jobs/J/out?path=DIR先生成.outcache，再处理meta/Range/分块；任务结束后的生成不受worker guard约束。沿用此接口、原FD相对路径/普通文件目录约束/源变化检查。单个原文件直接下载保持原行为。

公开jobs.export-limits-v1和/node.execution.export_limits，固定scope=directory_export_cache_generation；source_bytes=512MiB、compressed_bytes=512MiB、entries=4096（含导出根和所有目录/文件）、max_depth=64、min_free_bytes=64MiB。源载荷为每条普通文件路径的fstat逻辑长度，稀疏洞与重复inode路径都计入。缓存实际压缩写入计gzip头尾、tar/PAX和padding；不是物理占块或全节点累计配额。

先对选定子树做有界签名/预检，流式枚举并在保存名字前全局计条目，随后只排序已获准名单，不先os.listdir物化无限目录。归档第二遍也执行同一预算，沿原源变化核查；不新增内容hash或全节点扫描。超过目录预算时明确提示选择较小子目录/原文件。旧cache不批量删除；仅能在本次有界核查内确认命中的cache复用，不能承诺所有超新目录预算的历史cache仍可经目录接口访问。

## 实际写入与并发

每个job使用固定稳定的非阻塞flock，至多一个目录cache builder；缓存命中可直接读，取得锁后再核缓存，不新增后台队列。不同job/已打开旧cache/不同已完成子目录缓存和崩溃残留仍可累计，不声称全节点空间预留。锁文件保留稳定身份，不每次unlink。

唯一新tmp的gzip底层writer在每次实际写入前核本次compressed_bytes剩余额与实际job目录FD的f_bavail*f_frsize，要求本块加64MiB余量。短写按os.write真实返回值计数；一次预算/空间失败后锁存失败，gzip close不得继续补尾。普通syscall I/O故障归明确export错误。空间检查不是预留，其他进程可并发消耗空间。

生成预算/空间失败只清本次派生tmp，不截断源文件、不替换旧cache。完整归档/flush及签名核查后才沿原replace和描述/元数据发布；后续hash/元数据发布故障可能发生在完整新cache已替换之后，不伪称双文件事务或成功响应。清理tmp失败明确报告，不能静默当已清理。保留既有内容SHA/ETag读取语义，不为本批反复计算同一内容hash。

## API、CLI与Web

公共JSON错误：413 export_budget_exceeded(dimension/limit/observed)，507 export_space_insufficient或export_write_space_failed，503 export_storage_unavailable，409 export_busy。原ArtifactChanged仍409且不冒充新预算错误；新增错误携带有限诊断和scope，不返回不完整下载成功。

CLI目录元数据/下载遇这些export错误直接显示原因，保已有本地partial，不自动重新压缩、换路径或当旧节点二进制fallback。网络重连/原源变化语义不能把export_busy当源变更去反复删part重启。Web在现产物区提供显式“准备当前目录下载”，单次meta请求后展示大小/链接与政策；失败原地显示原因，无自动重试，不把大tar读入JS内存。原文件下载/校验保持。缺能力的旧节点显示未提供政策，不猜新限额。

## 验收

复用既有artifact_io协议：小目录→meta/Range/缓存命中，低测试限额覆盖载荷/条目/压缩输出与低空间/ENOSPC，核原件/旧cache不受损、tmp边界与无假成功；两个Jobs reader共享同job锁拒第二builder。只加必要合并用例，不扩归档格式矩阵、不下载大素材/扫描旧研究树。CLI新错误不重试，390px页面单次准备/失败/较小目录或原文件入口。每批一次完整协议，失败仅窄跑；源码/隔离宿主/CI，不部署/真机。

## 已验证范围

唯一一轮本地142协议79.306s全首过。复用实际只读HTTP/CLI归档链验证meta/Range/cache、跨Jobs reader稳定非阻塞锁、CLI元数据/分块busy不重试并保五字节partial；一新增合并方法覆盖真实短写、条目/载荷/压缩预算、低空间和写入一字节后的ENOSPC。源、旧cache/描述与本次tmp边界均过。

单390实际浏览器+synthetic HTTP首过，16GET/0POST：0隐式meta、根目录一次413、小目录一次meta及150B native download。无pageerror/横溢；预期413 console诊断保留。fixture22994和browser23372已退、端口57516关闭，root目视prepared截图。五项actual required CI success后同SHA直推main并ls-remote核对。索引在本机qa/job-export-limits-{validation,checks,publish,ui-validation}.json；无部署/真机/迁移，不清理旧QA恢复义务。
