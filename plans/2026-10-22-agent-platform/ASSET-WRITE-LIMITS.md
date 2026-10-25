# DIST-ASSET-WRITE-01：资源缓存受控写入边界

2026-10-25，已发布harness eb0c0a715a1dfd9a2756f8e1b5bd37efb2a1fbab。承接 DIST-09 与目录导出边界；累计缓存配额、引用保留与清理仍单列。本批不改变 manifest/job canonical，不扫描旧资源，不重复 hash，不部署或操作设备。

URL 下载、本节点工件导入、旧 import_file 适配及分块上传共用小型 writer。保留原 SHA flock、上传 owner/seal、partial 和文件/目录 fsync、增量 SHA、完整 size/hash/identity 验证与 receipt 发布。普通观测仍只读已保存事实；旧缓存/报告不回填。

每次写前核本次资产声明 size；URL/import 最多读 min(CHUNK, remaining+1)，超出的一个字节只用于判断，超长块不写入，不把前 N 字节当整个超长响应成功。导入保留前后源身份检查。writer 在实际 partial FD 上逐次检查 f_bavail，要求此次待写块加64MiB余量，短写按 os.write 返回值推进，错误后不再写。空间检查不是预留、物理块配额或全节点累计限制；磁盘同步失败仍不能报告可信续传 offset。

新 size/space 拒绝不截断已收前缀；沿原 partial 同步保存。唯一既有兼容例外是 HTTP200 忽略 Range 时从零重来：在验证 Content-Encoding、已提供且明确超长的 Content-Length 和原完整空间预检之后才 truncate。不能宣称这个既有重来分支保留旧前缀；若开始重传后失败，保留本次收到的前缀。

公开 games.write-limits-v1 与 /node.execution.asset_write_limits：scope=game_asset_cache_transfer，size_bound=manifest.size，min_free_bytes=67108864，per_write_space_check=true。新准备报告保存 write_limits；写入故障保存 write_error（code/details）及原 error 文本。已写字节是本次实际写入，prefix_bytes 是当前传输已接受前缀，不等于 fsync 已完成。上传 API 沿原 JSON 错误增加有限 details/scope；旧错误不增空字段。明确 asset_size_exceeded(413)、asset_space_insufficient(507)、asset_write_space_failed(507)、asset_storage_error(503)，不泄露 CDN 重定向凭据。

网页与 CLI 使用同一报告/错误：显示政策、失败原因与实际前缀，查询原上传 offset 后才能显式继续；新增空间错误不自动重传。旧节点政策未知，不推断新版限制。沿现游戏准备和上传页面，不新建工作流。

验收复用小 HTTP 资源/导入与上传夹具：超长 HTTP、读取期间增长的源、短写、传输期间空间下降及 ENOSPC；核未越过 size、不发布 verified、源和已有缓存保留、原上传 ID/offset 可查询。保留现合法续传/缓存命中不重复hash验证。只新增必要合并方法，唯一一轮本地协议，失败窄跑；单390网页链只查新政策/失败与原ID接续，不重做上传大矩阵。


## 发布证据

唯一本地144协议80.925s全部首次通过。两新增合并方法覆盖真实短写、8192字节旧prefix在超长HTTP200回包前拒绝时保留、无长度响应越界只读不写、导入源增长保12字节prefix、实际接受3字节后低空间/ENOSPC保原inode及10字节prefix；同upload ID查询同步offset后显式续传/验证。原HTTP/CLI链补507：仅一次PUT后查询原ID/prefix并exit2，下一显式调用完成。

单390浏览器+synthetic HTTP四步首次通过，含reload：28GET/2POST/2PUT，单upload ID；12字节首传507接受4字节，查询不写，显式8字节续传。0pageerror，预期404/507 console保留，宽390；fixture24952/browser24984退出、60102关闭，root目视failed截图。仅ID/target/requires_query跨reload保存，完整失败回执在原页内存和QA，未声称持久上传历史。CLI数字字段response_bytes最后静态纠正，不重跑本地全套；最终同提交五项actual CI全部成功，同SHA main/lsremote核对。qa/asset-write-limits-{validation,checks,publish,ui-validation}.json留本机；无部署/真机/迁移，所有旧QA未知恢复义务保留。
