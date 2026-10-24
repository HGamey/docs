# 游戏资源准备合同

2026-10-24，DIST-PREPARE-01 已发布 harness [12eda9a](https://github.com/HGamey/harness-ops/commit/12eda9ae41c72cfcd5432d363cda4f64c40a99a9)。对应 BASE-03、DIST-01/02/04 的宿主准备子项。资源在选定节点的 data_root 留存；本批不部署、不下载旧 Doom 大文件、不操作手机。

## 对象与入口

manifest 为 JSON：`schema_version:1, game_id, version, title, assets`。game_id/version 为 1–64 位小写字母、数字、点、下划线、横线，首位为字母或数字。每一 game_id/version 不可变；重复登记相同内容返回原记录，内容不同回 409，修改必须使用新 version。可选 `application, engine, requirements, scene, provenance, license` 为说明对象，描述包/ABI/系统、来源/许可、场景及缺口；这些元数据不执行命令。

assets 各项为 `{name,role,sha256,size,source}`，可选 `target`（逻辑相对路径，不表示设备安装授权）。name 唯一，SHA-256 64 位小写十六进制、size 为非负字节数。source 为 `{kind:"url",url}`、`{kind:"artifact",job_id,path}` 或 `{kind:"unavailable",reason}`。URL 只接收无凭据、无查询串的公开 HTTP(S) 资源；artifact 只读本节点既有 job/out 普通文件，不接受任意宿主路径。unavailable 可登记历史资源，准备时明确报告缺源；不能把历史 hash 当当前缓存证据。不同资产不能用同 SHA 声明不同大小。

所有 JSON 请求上限 64 KiB。读取使用现有只读 token 或网页查看认证；写入使用管理员 token 或网页 steer，保留 CSRF 合同。

- `GET /games` → `{schema_version:1,node_id,items:[record...]}`。
- `POST /games`，body=manifest → record。
- `GET /games/{game_id}/versions/{version}` → record。
- `POST /games/{game_id}/versions/{version}/prepare`，body=`{}`，必需 `X-Request-ID` → 原有 job 状态对象，含 `id,request_id`。同 ID 同清单去重，不同清单 409。
- `/dash/api/games...` 对应同一服务；`/dash/games` 和 `pf-node games list|show ID VERSION|register FILE|prepare ID VERSION|request REQUEST_ID` 同等能力。

record：`{schema_version:1,node_id,game_id,version,title,created_at,manifest,assets,resource_state,preparations,device_state}`。assets 保留 manifest 字段并附当前缓存 `observation:{state,bytes,total,verified_at,error}` 和最近任务逐项 `progress`（无报告为 null）。历史任务阶段不能覆盖当前缓存未知/变化或错误；下载阶段和字节来自报告，保留报告时间。resource_state 只有 verified / incomplete / unknown；verified 表示本节点全部完成对象有匹配的校验收据及文件身份。device_state 的 installed/resources_deployed/login/scene_ready 均为 unknown。preparations 为持久队列中的本版本准备任务，包含 job 状态及 `report`（缺失为 null）、`job_url,report_url`；报告原件为 `out/preparation.json`。列表不附完整准备历史，使用详情查询；preparation_count保留数量。

## 执行与存储

固定 producer `bundle_prepare` 使用现有 Jobs/SQLite/worker；小型不可变 manifest 快照进入 job/in。`device_access:none`，每版本声明一个 `game:<id>:<version>` 宿主资源。下载不在同步 intake 或 HTTP 线程执行。跨版本同内容由缓存 SHA 文件锁协调；等待可取消，不增加另一个后台队列。

缓存位于 data_root/_games/cache。准备程序接口为 `game_assets.py --manifest FILE --cache-root DIR --jobs-root DIR --out DIR`；`AssetCache(cache_root).observe(asset)` 仅读取收据与 stat，返回上述 observation。准备报告 `schema_version,game_id,version,state,updated_at,assets`；assets 为 `{name,sha256,size,...observation}`，可附缓存命中、传输/续传字节、隔离记录。阶段为 missing/waiting/downloading/importing/verifying/verified/unavailable/failed/cancelled/unknown，时间为 Unix 秒。任务结束与当前文件状态分开显示。

完成对象按 SHA 去重，partial 单独保存；续传验证 Content-Range，不把 200 响应追加到旧 partial。接收端边传输边 hash；续传仅重读已有前缀。验证成功 fsync 后原子发布及保存文件身份收据；GET 不重新全量 hash。文件身份改变即不再 verified。坏完整对象/坏 partial 隔离，保留原因，不自动删除原始证据。每个下载前预检剩余字节和磁盘保留空间；缓存配额与显式清理另属 DIST-09。

取消复用原 job/cancel，保留 partial 和阶段；新尝试用新 request ID，复用内容缓存，不重放旧任务。进度不是安装证据，exit 0 也不能代替校验收据。准备历史直接由持久队列的 producer/资源归属导出，不另写一个可能漏关联的任务索引。

## 本批边界与后续

先支持公开 URL、本节点既有产物和缺源历史登记；本机大文件流式上传、认证下载、设备部署/安装/备份/恢复、场景验证、跨节点自动派发、保留配额仍待后续。BASE-03 同时去除旧 fetch_apk 默认 serial、隐式 `-g`/音量修改，复用同一缓存内核；旧混合入口仍占设备锁，用户应先通过纯宿主准备完成下载。

验收聚焦一条隔离 HTTP→真实 worker→内容缓存→Web/CLI 找回链，另覆盖实际已知故障：断传续传/忽略 Range、坏数据隔离、缓存复用、不可变版本冲突与只读拒写。旧 Doom 清单只导入既有元数据，记录免费 Demo 的许可/可访问源缺口，不推断当前可用状态。完整 DIST 与总体 goal 保持开放。

## 验证记录

本地协议一次83项全部通过，31.169秒；真实隔离HTTP、独立worker、CLI、缓存和历史关联走通，七项新增聚焦上述故障链。原件 `tasks/developer_transition_20261022/qa/game-distribution-node-protocol.log`，摘要 `game-distribution-validation.json`。Doom示例5项资源仅解析/校验历史清单，没有读取或重算大文件。网页定向DOM和实际1440暗色/390浅色浏览器替身通过：一次准备POST模拟503，原ID查询找回同一任务，报告链接可读，无二次POST或横向溢出。首次DOM仅cwd错误，未执行任何case；纠正cwd后一次有效运行通过，原记录保留。五项actual CI（code-only、protocol 3.11/3.14、cgroup-kernel、pf-shared-kernel）全success后，同一SHA推main并核对远端。验证范围不含生产、真机或大文件传输；当前public contained保持原状态。

旧fetch_apk入口保留4位置参数 SOURCE SIZE SERIAL SHA256，但size/serial/SHA不再可省略；节点注入固定cache root。直接CLI需显式cache root；旧basename导入仍读取PF_APK_CACHE_DIR。仅`adb -s SERIAL install -r`，不隐式授予权限或改音量。该入口仍由devlock包裹，并不具备新的typed设备恢复收据。

发布证据：`qa/game-distribution-publish.json`、`game-distribution-ci-checks.json`；网页回执 `game-resources-ui-browser-validation.json` 与 `game-resources-ui-dom.json`。测试tab/viewport已收回，临时HTTP服务已退出。此批不关闭总goal；下一子项DIST-UPLOAD-01补本机大文件流式接收。
