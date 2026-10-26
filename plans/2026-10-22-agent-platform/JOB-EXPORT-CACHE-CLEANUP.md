# 当前任务的导出缓存清理

JOB-EXPORT-CACHE-GC-01只管理当前节点job生成的目录导出缓存。查看该job已发布archive及metadata的数量、字节和存放模式；查询不创建namespace目录，不扫描原件或全盘，不计算内容hash。

清理为显式operator操作，固定node/job，沿原export lock非阻塞执行；构建占锁时返回忙，不等待或重试。仅删除明确命名的.outcache缓存及对应元数据，不删除out/in、结果、游戏资源、任务记录或锁。已打开的下载可继续读原FD，新下载按原生成流程。缺失是0项，不伪称删除原始实验数据。

CLI与Web同批，使用既有认证和目标绑定；readonly只能查看。外置根不可用保持原错误，不转去旧路径清理。harness `be8279c` 已发布main/stable；累计资源缓存与自动保留政策仍属DIST-09。

验收：唯一API及实际CLI链0.698秒，Web实际handler的小型检查通过；code-only 40317136127、node-tests 40317136207全部成功。未运行本地全套，未操作生产或手机。返回逻辑文件字节，不能将其直接当作磁盘实际释放量；部分删除失败如实保留remaining/errors，下次导出仍可重新生成。
