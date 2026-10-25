# JOB-TIME-BUDGET-01：作业执行期限

2026-10-25，源码与隔离宿主/CI范围已交付（harness `4b7c75a`），未部署或运行真机。属于 JOB-09 的执行期限子项；不代表研究总预算、磁盘上限、通用暂停或生产部署已完成。

## 声明与准入

普通 job 的可选 `time_budget` 为封闭对象：`{"wall_seconds": N, "timeout_action": "graceful" | "escalate"}`。两字段都必需，N 为 1–604800 的整数（布尔和小数非法）。省略保持旧请求 canonical 不变，显式 null 非法。能力 `jobs.time-budget-v1`。字段冻结进原 spec/指纹/launch；同 ID 不得改预算。device_access=recovery 暂不接受；内部准备和 typed recovery 不继承原预算。

CLI `--max-runtime-seconds N [--timeout-action graceful|escalate]`：显式 N 且省略 action 时使用 graceful；只有 action 无 N 拒绝。提交前检查能力，网页同级。未决请求保存原预算对象或明确省略；重载不得按新表单替换。producer 自带秒数不自动映射通用期限。

## 计时与停止

独立 worker 从成功 Popen 后（contained 从 GO 门释放后）开始 monotonic 计时；上传、排队、输入准备不计入。HTTP/CLI 断开或重启不重置 worker 的期限；worker 丢失只沿现有 lost/blocked，不重放命令、不另造替代计时器。contained 复用同一机制，但公共 contained 能力继续不可用。

worker 仅在目标执行仍被观察为活动时检查期限；观察到正常结束优先处理结束。到期先在当前owner内存锁存预算触发与停止意图，再尝试原记录发布；后续764c4bb修复记录I/O失败时丢监督，见[JOB-OWNER-RECORD-FAILURE](JOB-OWNER-RECORD-FAILURE.md)。成功写盘不再是发信号前提。graceful 只请求 SIGINT；escalate 复用现有升级：SIGINT 后 15 秒 SIGTERM，再 15 秒 SIGKILL；contained 的直接命令已退出但域仍有进程时沿原有域回收规则。它是停止请求期限，不保证到该时刻已结束或已恢复。已有人工请求和信号保留，升级只增不减，不重置既有升级时钟。

首次 cancel/force record 可附 `source: execution_deadline`；人工旧记录保持原格式。`read_cancel` 将 force 的来源公开为 `escalation_source`（仅原 force 有 source 时）；预算触发记录始终独立存在，即使首次 cancel 原因是人工。实际信号仍只由现有 owner worker 发出。

## 公开事实

meta 顶层 `time_budget` 保存原声明；`execution.time_budget`（仅显式声明才存在）为：

- `schema: phonefarm/job-time-budget@1`，原 `wall_seconds` / `timeout_action`；
- `status: not_started | running | exceeded | finished`；
- `started_at`、`deadline_at`、`observed_at`、`elapsed_seconds`、`exceeded_at`、`finished_at`（epoch 秒或 monotonic elapsed；未观察为 null）。

仅开始、首次到期、结束时保存观察，不每轮/每次 GET 写时钟。`exceeded` 表示 worker 已触发停止请求，结束后保留 exceeded 并填 finished_at；`finished` 仅表示未触发预算停止即观察到结束，不能据轮询时刻推断精确是否在期限内。worker 丢失保留最后记录，配合 lost/worker_alive 展示，不从墙钟推断已到期/已发信号/已停止。未启动任务仍 not_started。

实际 command returncode/exit、outcome、进程组/域结束范围、设备恢复、队列保留继续沿原合同。CLI terminal_exit 对已可信结束、原 exit=0 且 exceeded_at 有事实返回 124；原非零退出码保留，unknown/进程组未空优先 125。stderr 说明期限触发及原命令码。网页不得仅以 exit0 显示普通成功，不自动 cancel/recover/release，不做前端计时执法。

预算事实放在已有 meta.json，取消 intent 使用已有 cancel.json/cancel.force.json；既有 JOB-DURABILITY 封存集合覆盖，不新增并行日志或调度器。

## 验收范围

一次完整本地协议；实际无害子进程证明独立 worker 在 HTTP 服务退出后仍触发期限、排队不计时、正常结束/未启动/人工升级不混淆，保留首次失败。Linux existing kernel CI 验证 contained 路径。一次 390px 浏览器验证冻结请求、最近 worker 观察、真实退出与恢复分离。无真机操作、部署或实验收益宣称。

## 验证与发布记录

源码提交 `4b7c75a6870ea1b7960ffbdbb0f18931aa95cb1a`。唯一完整本地协议 133 项、70.786 秒，131 通过；两项测试夹具故障原件保留：旧恢复用例读取到刚创建而未写完的 JSON，新用例将 HTTPResponse 的 header 属性误当真实 EOF。改为原子发布旧夹具完成记录、删除新用例无关属性断言，原两项定向 2/2 通过（2.064 秒），不重跑全套。

实际隔离 HTTP/worker 链通过：服务退出后 A 的 3 秒期限仍触发 SIGINT，真实 exit0；无预算 C 独立结束；B 排队超过自身 1 秒仍未计时，服务重启后实际运行 0.2 秒正常结束。初次最终 GET 的 B 仍 finalizing，仅重启原服务完成现有 SQL 终态，不新增作业；最终三任务 done、9 个已知 PID 全退出。

单个390px浏览器首轮4步通过，37 HTTP=36 GET+1合成截断提交；原预算/ID跨重载保留，墙钟超过deadline但worker仍running时不自行判为到期，exit0不覆盖预算触发/恢复required。0设备、0取消/恢复/释放；服务与浏览器退出。

Linux验证复用固定安装/非root launcher退出/脱组后代那一条真实cgroup链，增加5秒预算；原手动force内核链仍保留。五项必需检查 code-only、protocol (3.11)/(3.14)、cgroup-kernel、pf-shared-kernel 全部成功；真实固定worker预算/域回收链通过。同一提交 `4b7c75a6870ea1b7960ffbdbb0f18931aa95cb1a` 已直推 main 并 ls-remote 核对，CI run 40147747815。证据索引为本地 `qa/job-time-budget-{validation,checks,publish,ui-validation}.json`；旧QA恢复义务不清理。
