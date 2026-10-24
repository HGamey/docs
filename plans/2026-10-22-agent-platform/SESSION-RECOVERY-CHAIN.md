# JOB-SESSION-RECOVERY-02：失败恢复任务的原会话接续

2026-10-25，下一批合同。仅源码、隔离宿主和既有 Linux CI；无生产或真实设备操作。只读依据：本机 qa/session-recovery-child-next.md。继承 [JOB-RECOVERY](JOB-RECOVERY.md) 的祖先资源借用与 [JOB-CONTAINMENT](JOB-CONTAINMENT.md) 的宿主完成边界。

## 身份与范围

灰层原作业 A 的 session 始终在 A/out；R1 恢复失败后，R2 的 recovery_for 必须为阻断叶子 R1，实际 PF_RECOVERY_OUT 仍为 A/out。两种身份分别保存和核对，不复制 descriptor、不将 A 改成 R1。新 lease receipt 属于 R2，真实 recovery_of 由 PF 的 marker 决定，不能从队列父强填。A/R1 原失败、日志和屏障保留，成功恢复不自动 release。

复用现有 queue、intake 锁、request ID、worker 和 PF，不新增调度器/lease。游戏部署、安装、初始化、场景已有的恢复来源解析保持原行为。不得借此开放新控制会话、任意路径或设备串行号输入。

普通 worker 只报告原进程组退出，灰层恢复命令的 recovery.status 仍为 not_reported。当前 finalize 会将其直接置 done，这也需要修复：与 game-deploy 恢复相同，灰层 session-recover 的 not_reported 保留 queue blocked，真实 exit/outcome 不变。不为本批新建 producer 汇总器，也不把 scoped PF receipt 升格成整个作业的恢复完成。R2 即使报告限定恢复成功也先保 blocked；计划读原 session closed 后显示审收据与显式 release，而非允许继续恢复。释放顺序 R2→R1→A，沿现 fresh lease clear/子任务检查。

## 读与写接口

新增 capability `jobs.session-recovery-chain-v1`，保留 `jobs.session-recovery-v1`。

`GET /jobs/{id}/session/recovery` 及 dash 镜像只返回节点派生计划：schema_version=1、node_id、job_id（查询入口）、supported、eligible、reason、source_job_id、session_id、serial、session_path（原输出相对路径）、target_job_id、attempts。attempts 为该原来源下按父子排列的恢复记录，每项 job_id/recovery_for/request_id/queue_state；原 A 不伪装恢复 attempt。valid 链但当前不可接续仍返回计划和理由；不通过新 job 或设备命令获得计划，只调用既有宿主 session-status。

从 A 或已知恢复 attempt 查询得到同一叶子。节点追溯实际 queue spec 和 descriptor：源为 phonefarm 灰层会话；每个恢复 attempt 必须匹配原来源路径、serial、process_group、device_access=recovery、逐级 recovery_for 和继承 resources。仅同路径的任意手工命令不被当成相同规范请求。兼容既有一轮 typed 命令，旧 spec 不补默认字段、不改 canonical 指纹。

`POST /jobs/{leaf}/session/recover {}` 继续用 X-Request-ID。原 request ID 先按冻结 spec 返回原任务，即便 session 已关闭；同 ID 改入口/内容冲突。新 ID 在现有 intake admit 内重新核唯一叶子、宿主完成与存储终态、原 session phase=recovery_required/live_owner=false。旧祖先新请求返回 409 `session_recovery_leaf` 和明确 target_job_id；不代客户端换入口。不可接续返回 409 `session_recovery_blocked` 和理由。请求体仍为空，不接 source 路径。

多子项/循环/错误绑定保留并解释，不按时间挑最新。只有 SQL cancelled_before_launch 可忽略；preparing/queued/claimed 子项阻断新增。done 子项要先审收据与原显式 release；不因 exit0 推断设备恢复。storage_blocked 先 retry-finalize；宿主身份/范围不明先 execution reconcile。原会话 descriptor 缺失不造恢复。

dispatcher 对匹配灰层规范恢复的冻结 spec，从同一来源解析取得 PF_RECOVERY_OUT；解析失败在启动前记录失败，不能丢线程或改成不带来源继续启动。queue 的 recovery_for 和祖先借用不变。其他 producer 保持现有来源逻辑。

## Web 与 CLI

网页明确显示原 session 来源、当前阻断叶子、每次恢复尝试和不可接续理由；新恢复尝试无需先解除失败恢复任务屏障。新建时读取计划并冻结 endpoint target_job_id，持久化原 request ID 与目标 node/leaf；未知回执仍查原 ID，不随刷新计划改投。旧未决项只能按其原 selected 入口接续，不猜新叶子。返回任务 recovery_for 必须匹配冻结 endpoint。

CLI 新增 `session-recovery JOB` 只读同计划；`session-recover JOB --request-id ID` 保持原语法，不自动换目标。旧祖先409能看见具体叶子和查询命令。CLI/API/网页共享同一资格，不把页面按钮作为唯一准入。

## 验收

一条 HTTP/CLI 和真实无害 worker 链：A blocked → R1 blocked → 计划指 R1 → R2 借祖先资源继续；普通同资源仍等待、无关宿主任务继续，原 ID 回查不新增，旧祖先新 ID 拒绝，两个并发新 ID 至多一个入队。核 session 来源不变、每次失败和收据保留，成功不自动 release。复用既有测试夹具，仅新增缺失链，不重写队列单测。

既有 Linux pf-shared-kernel 的 integration_pf_recovery.py 延长首恢复再次失败→第二恢复成功，核 actual PF_RECOVERY_OUT、私有原 out 与共享新 out、真实 receipt/recovery_of、旧 UID 权限与收尾。当前 Mac 不重复容器/设备实验；实际 Linux 检查由同 SHA CI 给证据。按 AGENTS 协议整轮一次，之后只修失败链。网页一次390流程展示叶子和原ID找回；不重做所有控制动作。

真实设备、公共 contained 启用、跨 boot、历史未知 QA 屏障仍单列。本项不解除它们，也不以源码修复声称已恢复手机。
