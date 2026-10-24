# 本机资源流式上传

2026-10-24，DIST-UPLOAD-01 已发布harness `02da7cd`，五项实际CI成功后同提交直推main并核远端。接续 [GAME-DISTRIBUTION](GAME-DISTRIBUTION.md)，数据仍写选定节点同一 SHA 缓存；网页和 CLI 同等，不部署或读取旧 Doom 大文件。

## HTTP 与身份

能力 `games.upload-v1`。`POST /uploads` JSON `{upload_id,game_id,version,asset_name}`，upload_id 为客户端先生成并保存的 UUID 或 1–64 位小写字母/数字/横线/下划线，首位字母或数字。同 ID 同绑定幂等，异绑定 409。先保存绑定再获取内容保留；中途失败公开unclaimed/null，用户显式原ID重试可补登记，已取消则不会重开。asset_name 放 JSON，期望 size/SHA 和资源声明都来自不可变版本；不会修改 manifest 的 source。不上传本机路径，不要求客户端再全量 hash。

- `GET /uploads/{id}` 查原会话；不存在 404 `{code:"upload_not_found",error:...}`。
- `PUT /uploads/{id}`，`Content-Type: application/octet-stream`、必需 Content-Length、Upload-Offset；每块 1 B–8 MiB。偏移在内容锁内核对，不匹配 409，禁止覆盖。服务器最多每次读 256 KiB，异常也同步已收前缀及父目录，之后 GET 给出真实持久 offset。请求体读取限时30秒，不接受无界 chunked transfer。
- `POST /uploads/{id}/finalize {}`，收齐后封存 partial 身份，再以固定 request ID `upload-verify:<id>` 提交/找回现有校验 job，返回会话。关联任务还核固定spec和原清单输入指纹，不按请求ID盲目采纳普通job。校验仅针对选中资源，不自动下载清单其他文件，也不回退 URL。
- `POST /uploads/{id}/cancel {}` 取消缓存写入保留、保留 partial。正在持锁写入或校验则409；先停当前传输或沿原 job 取消，随后可再次取消会话。旧排队校验遇取消/不同 owner 必须拒绝，不能验证新上传的字节。新尝试显式用新 ID，旧会话/报告不删。

JSON 请求≤64 KiB；写入需 admin token 或网页 steer+X-HG，读取沿用只读访问。`/dash/api/uploads` 同合同。写失败且body未消耗时关闭HTTP连接，不让残余字节成为下一请求。

会话记录：`{schema_version:1,upload_id,node_id,game_id,version,asset_name,sha256,size,offset,phase,error,chunk_max_bytes:8388608,created_at,observation,job}`。job 为 null 或原 job 状态加 job_url/report_url。phase 为 unclaimed/receiving/received/sealed/verifying/verified/failed/cancelled/superseded/busy/unknown。offset 为持久前缀字节；写锁忙且无法确认时为 null，不能当0继续上传。封存后的size已持久，可在校验期间展示。observation始终是当前缓存事实；过去取消/失败、当前缓存和校验job结果分开。

创建成功或续传前必须核回包的 node_id、game/version/asset、size/SHA。字节收齐≠校验通过，job accepted/exit0也不能代替当前cache receipt。网络不确定时只查原ID，不自动换ID创建；显式重试可以复用同ID。重新选择同大小文件并不证明内容相同，最终由服务器SHA判定。

## 共享缓存与队列

上传的字节直接进入原 `partial/<sha>.part`。新增小型SHA写入保留记录，记录upload_id/size/open|sealed|cancelled|verified|failed及封存文件身份，不复制数据。所有下载/导入/上传/校验入口共用SHA flock；open/sealed保留期间普通准备不得改写，但仍继续其他独立资源。暂停保留原会话；显式取消解除该内容的保留，不占手机lease或全平台队列。

cache API：`upload_open(asset,id)`、`upload_status(asset,id)`、`upload_chunk(asset,id,offset,length,stream)`、`upload_seal(asset,id)`、`upload_cancel(asset,id)`、`verify_upload(asset,id,on_progress=None)`。HTTP写操作非阻塞取内容锁，忙时结构化409；查询不做hash。`AssetUploadError(status,code,message)`为公共安全错误。status返回 `state,offset,error`，可带busy/cache_hit；state对应open/sealed/cancelled/verified/failed/superseded/busy/unclaimed/unknown。GET不能返回尚未同步的写入偏移。查询成功非阻塞拿锁后，对既有前缀及父目录做fsync后才返回offset；不创建文件、不改字节、不hash，失败返回unknown/null。

封存前不全量hash；既有bundle_prepare producer新增 `--verify-upload ID --asset-index N`，输入仍为完整不可变manifest。报告只列选中资源，并加 `scope:{kind:"uploaded_asset",asset_index,asset_name,upload_id}`；原game资源键及队列关联保留。专用verify在同锁内核对相同upload ID与seal身份，hash一次，复用现有原子发布；坏内容隔离并报告failed。已验证的相同内容直接复用，不重复全量hash。

HTTP服务只接收有界块、记录会话和提交小型任务；不增加第二后台执行器。服务重启后原会话、已收前缀和原校验job仍可查。封存后提交回包丢失可反复finalize找回同一job；不得重开为可写。

## Web / CLI

在游戏资源行增加选择文件、上传并校验、暂停、查询/续传、显式取消/新尝试入口。文件只在浏览器内存持有，sessionStorage仅保存node/游戏/版本/资源/上传ID；离开页面停止发送，刷新后重选文件与GET原ID接续。先确认服务端offset，再更新确认进度；活动传输最多一条，默认1 MiB块，最大8 MiB。

`pf-node games upload ID VERSION ASSET FILE [--upload-id ID]`，新尝试先输出upload ID，再登记，按服务端offset流式读取，同一FD每块核fstat身份。stdout只输出最终原会话JSON，进度stderr。`games upload-status ID`、`games upload-finalize ID`、`games upload-cancel ID`独立可用。CLI写结果未知只GET原ID并退出；用户带原--upload-id显式接续，不自动二次写。所有请求禁止重定向凭据。

## 验收与后续

复用83项已有协议，新增聚焦两条缓存边界（断流/偏移/取消续接、封存竞争/校验或隔离）、一条真实HTTP+worker+CLI接续链，以及必要客户端丢回包分支；不再镜像全缓存测试。Web做一条选择文件→暂停/续传→后台校验的替身流程并检查窄屏。旧大文件、生产及真机不在本批验收内；累计配额/保留清理、跨节点自动分发、设备部署和场景仍属后续。

## 2026-10-24 实现与证据

实现 `02da7cd`：NodeUploads保存原绑定，复用AssetCache的SHA保留/锁/partial与现有Jobs worker；HTTP块限制8MiB、每读256KiB、30秒绝对读取期限。GET在锁内同步已有前缀后报告offset，上传与封存不再全量hash。校验报告只列选定资产与scope。普通下载不改写正在上传的SHA，独立资源仍可处理。网页和CLI用同一个合同；网页新增折叠上传入口，CLI持有同一FD，每块核文件身份。

本地88项协议35.170秒一次通过；静态审查后对未完成登记的同ID补登、无关request任务关联、取消未认领绑定三处窄修，HTTP定向链1.822秒/新增取消分支1.865秒、CLI两项1.178秒通过。真实隔离HTTP短传123457字节→服务/Jobs重建→CLI接续约2MiB合成文件→独立worker→同一原job的结果可查；额外资源保持缺失，设备状态未知。验证从未读取旧Doom大文件。

DOM一次有效执行通过。浏览器同一上传ID完成断线、暂停、刷新后重选及接续；最终创建1次、finalize1次，最大块1MiB，1440和390像素无横向溢出。首次替身3秒暂停窗口在工具调用间结束，已保存负例；只将替身窗口放宽到10秒、续接原ID和65536字节，未重跑DOM或改产品代码。此UI替身丢弃内容，不代表实际hash/worker验证；后者由独立HTTP链验证。fixture退出、端口不监听，标签关闭且视口恢复。

本机原件：`qa/game-upload-validation.json`、`game-upload-node-protocol.log`、`game-upload-http-{followup,final}.log`、`game-upload-client-followup.log`、`game-upload-ui-{browser-validation,http-receipt,dom}.json`及两张截图。最终CI与发布信息另写`qa/game-upload-publish.json`。大文件实际压力、总配额/保留清理、跨节点传送、设备安装/部署/场景仍开放；本批不部署或操作手机。

发布核验：`02da7cd657e2f6873080c7e057807ffe37292b37`在验证分支的code-only、protocol Python3.11/3.14、cgroup-kernel、pf-shared-kernel实际check-runs均success，然后直接推main并通过ls-remote确认同SHA；未新建PR。[协议与内核CI](https://github.com/HGamey/harness-ops/actions/runs/40093553966)、[代码仓规则CI](https://github.com/HGamey/harness-ops/actions/runs/40093553885)。总目标保持active，只完成DIST-UPLOAD-01子项。
