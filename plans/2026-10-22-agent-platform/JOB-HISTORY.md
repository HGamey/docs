# 旧作业的显式只读来源

2026-10-25，BASE-HISTORY-01 实施合同，尚未验收或发布。当前作业与历史来源分开寻址，不迁移旧数据。

## 管理员配置与身份

服务新增 `--history-roots /absolute/history.json`，可放进已交付的 launcher service_args。JSON 为 `{"schema":"hgamey/job-history@1","sources":[{"source_id":"old-local","revision":1,"label":"原本机作业","jobs_root":"/absolute/old/_jobs"}]}`。配置最大64KiB、最多16个来源；ID唯一、revision为正整数、root绝对路径。省略保持空列表。语法错误启动前拒绝；某个根不可读只使该来源 unavailable，不隐藏它或切到另一根。

管理员为改指向的来源递增revision；这是配置声明，不是旧数据不可变证明。进程内用SafeRoot固定旧根，所有路径组件复用现有禁止链接/特殊文件的规则。禁止把当前jobs根再次登记为历史。节点重启从配置读取，更新配置需重启launcher/服务；不会写历史根身份文件或新注册库。

新能力为 `jobs.history-read-v1`。`GET /job-sources` 返回schema `hgamey/job-sources@1`、node_id、items；每项含source_id/revision/label/jobs_root/read_only/availability/reason。后续所有历史job读请求必须带 `source_revision=N`，不符返回409 `history_source_changed`，客户端保留原选择并要求显式重选。来源+revision+node+job共同组成引用；列表cursor也绑定来源、revision和查询。

## 读取接口

共享 token 与 `/dash/api` 同名路由，只读token可用；沿原node pin。命名空间是 `/job-sources/{source_id}/jobs`，再接 `/{job_id}`、`/{job_id}/artifacts`、`/{job_id}/log`、`/{job_id}/out`。所有修改/取消/恢复请求拒绝，不转发到当前jobs。当前`/jobs`及旧cursor完全保持。

- 列表接受query/limit/cursor，不将当前实时state筛选用于历史记录。列表给小投影：id、label、bin、started、recorded_state、recorded_phase、recorded_exit、observation=`historical_readonly`。元信息缺失/非法列warning，不伪装零任务。详情在相同投影外附独立record原meta，不通过当前PID、worker锁或恢复逻辑改写它；meta缺exit时可单独读取旧exitcode作“记录中的退出码”，缺失仍null，不猜进程状态。
- 列表/详情/工件响应包含node_id/source_id/source_revision。工件列表复用现安全目录读取与分页；日志沿offset/limit有界读。
- out只支持显式单文件path，复用现Range/etag/SHA下载合同和内存摘要缓存。目录导出明确不支持，不在旧根创建锁、归档、缓存或partial。客户端不降级到当前jobs或整目录下载。
- 来源不存在404、不可读503、revision变化409、请求不合法400；错误含稳定code。读到的原文件仍可能由外部程序修改，现etag/stamp边界继续适用，不宣称存储快照。

## Web 与 CLI

现jobs页增加来源选择、历史只读提示及来源revision深链；切换时清分页、选中项、日志offset和工件状态。历史详情用独立只读分支，展示原记录及日志/文件，不触发session、取消、恢复、release、input-reference、状态events或目录导出。当前来源界面保持。选定来源失败时保留错误和原选择，不能自动退回当前任务。

CLI在现jobs/status/log/artifacts/out只读命令增加 `--source SOURCE --source-revision N`；另有 `job-sources`发现入口。历史out必须指定单文件path，默认本地目的目录包含来源/revision/job，partial身份包含node/source/revision/job。不扩展wait/kill/恢复或写命令；缺revision不能自动替用户选择新值。

## 验收与未覆盖

隔离临时旧_jobs设为只读，真实HTTP和CLI读取两个同名job来源及原件；验证node/source/revision/cursor边界、坏meta/不可读来源、符号链接拒绝、修改入口拒绝、无SQL/身份/导出缓存写入。一次390宽网页链检查来源切换、原记录与单文件下载、错误保留及没有写请求。按规则本批唯一完整协议测试，失败仅定向修复；不读生产旧任务，不连接手机。

此批仅旧作业目录适配，不导入当前结果索引，不恢复旧运行任务，不生成迁移成功收据，不解决累计缓存配额或原始实验迁移。BASE-05父项仍开放。
