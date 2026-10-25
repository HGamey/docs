# 目录导出缓存的独立位置

2026-10-25，BASE-CACHE-01实施合同，尚未验收。只分离当前job目录导出的新派生文件，省略新参数保持原布局。

## 配置与实际位置

管理员可在node_service或launcher service_args指定 `--export-cache-root /absolute/directory`。语法错误和与当前jobs/历史来源根相互包含的配置启动前拒绝。根不可创建/打开时保留不可用状态；不退回原job目录。原jobs、数据根、node_id、队列、worker固定配置和旧请求ID不改变。

外置根通过SafeRoot固定；命名空间采用已固定jobs根FD的dev/ino，再接jid。不对内容增加标识hash，不新建注册库；描述另保存源根/dev/ino/jid/rel小binding，命中同时核现有树签名/size/SHA。换源inode或根路径可导致重建，不宣称透明迁移。归档、描述和本次tmp在同一目的文件系统创建/替换；不跨盘rename原件。外置模式不读旧.outcache作fallback，不扫描、复制、迁移或删除旧cache。

省略参数仍使用jobs/J中的原.outcache；已有文件继续按原资格检查。新模式保留jobs/J/.out-export.lock的稳定inode，旧/新reader共用builder互斥，不能声称当前job GET零写。单文件下载不依赖导出缓存；历史来源仍只读单文件，目录导出不开放。

## 存储错误与预算

沿用目录导出有界遍历/压缩字节/空间储备/短写计账及错误合同。外置模式的空间检查必须使用实际目的目录FD，不用源job盘空间推断目的盘。只清理本次未发布tmp；已发布归档后描述保存失败仍明确记录，不冒充双文件事务。目的根不可用返回503 export_storage_unavailable（启动观察的空间失败按507），不写原job cache。单文件、旧ID、停止/恢复不因此阻断。

`GET /node` 的 storage.export_cache 提供 scope=directory_export、mode=legacy_job_directory|external、configured_root、effective_root、availability=available|unavailable、reason、legacy_entries_migrated=false。available仅表示启动成功打开目的根，不是当前可写空间或永久资格。能力为jobs.export-cache-root-v1；网页节点与作业页显示同一事实，pf-node node原样给JSON。实际请求错误仍沿原导出反馈。

## 原件、接续与清理边界

- job in/out/log、meta/prepared/exit/恢复/lease及SQL均为原件或执行状态，不能当缓存删除。稳定.out-export.lock是协调记录，不随导出清理。
- `_games/cache/blobs`可能是上传/导入后唯一内容；partial是已接受前缀，uploads/receipts绑定原ID与offset，quarantine保留失败原件，locks协调传输。名字叫cache不等于可丢，不改其路径及冻结job参数。
- `_dash/screens`的帧不能保证重现；`_dash/live`未知观察可能关联恢复义务。此批不移/删，不以再次采手机代替宿主重建。
- `_remote`、结果/研究、节点目录登记是持久状态；目录登记可能含远端token。外置配置/凭据按NODE-LAUNCH与NODE-CONFIG-PATH，不能进入导出缓存。
- 内存摘要/元数据缓存、updater构建工作区与HTTP导出文件分别处理。历史目录不创建缓存。

即使是导出件，源已丢时旧cache也可能成为最后副本，不提供无条件清空按钮。此批无后台GC/累计配额；未来显式清理须先核源与引用并沿相同稳定锁，归DIST-09/JOB-09。更换配置需重启HTTP/launcher；不停止独立worker或搬实验原件。

## 最小验收

新增一个实际HTTP/CLI外置缓存链：同ID不同源隔离、重开服务命中、旧cache和源不改、单文件/目录下载、目的FD空间预算与不可用不fallback。复用现artifact测试的故障规则，不重跑旧设备矩阵。一次390网页仅验位置与错误反馈。按规则一次完整协议；如失败只定向修复。无手机、生产、真实大素材复制。
