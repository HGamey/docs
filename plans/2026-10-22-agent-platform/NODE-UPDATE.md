# 节点代码更新与回滚

FLEET-UPDATE-01 是 FLEET-06 的有限宿主实现。发布源码/产物与运行服务重启分开；不更换固定特权 worker，不迁移实验数据，不自动安装设备工具，不声称整个多仓事务或所有远端参与者已协调。

服务向固定 `update_node` worker 传入绝对安装布局 `HG_UPDATE_LAYOUT`（phonefarm/game_opt_loop/harness_ops/state_dir）及实际 `HG_UPDATE_JOBS_ROOT`。harness 组件平铺到 updater 所在目录；回滚状态按三份规范化安装路径固定，不跟随 jobs/data-dir 变化。手工入口 `--install-root` 使用同一状态推导，需要明确 `--jobs-root`；`--offline` 仅是管理员声明停止参与者，回执不把它写成已验证。managed worker 禁止覆盖这些路径或选择 offline。

git 获取、编译/预构建校验、Mach-O 签名及验证均先在 staging 完成；失败不得替换运行文件。PF 源改用 HGamey 组织。仅发布代码/明确产物，拒绝或排除任务、运行证据、作业、lease、凭据和 Git 目录。保留目的地非发布文件；不会同步删除。更新每个文件前保存旧内容、前后身份及回执，再原子替换该文件。新目录及关键记录同步父目录；这是文件级发布，不是多组件原子切换。

实际发布及回滚持有现有 `.intake.lock`，只读队列检查其他 claimed/blocked/旧执行记录；只允许本次 claimed update_node 自身，queued 可以等候但不能在锁内启动。额外的持久 maintenance marker 在第一次替换前保存；中断后仍阻止普通任务，网页/CLI队列给出原 run 与原因。只有固定 updater 对该 run 的明确 rollback 可进入，其他原作业的取消/状态/恢复查询仍沿原接口处理。发布成功或所有旧文件恢复且终态已保存后才清除 marker。无法识别的状态/回执保守阻断，不能靠删除记录解除。

2026-10-26后续CI暴露一个真实时序问题：启动/观察updater的dispatcher短暂持有intake，原非阻塞检查可能把自己的启动过程误判为更新失败。修复为发布锁准入最多等待5秒，先取得锁再核队列和发布；超时仍明确失败，安装级`.update.lock`继续立即拒绝并行更新。等待不重放任务、不重启服务、不放宽claimed/blocked门禁。定向真实worker夹具让父dispatcher持锁到子进程实际报告等待，确认期间文件未替换，再放锁完成发布/回滚；原CI失败日志保留。

显式 `--rollback RUN_ID` 先核目标仍等于该更新写入或原先内容，拒绝覆盖后来的人为修改。自动还原失败或执行中断保留 `rollback_required` 和原件，之后只能接续原更新恢复。回执 `hgamey/node-update@1` 区分 preparing/staged/publishing/succeeded/restored/failed/rollback_required；stdout 为摘要及回执位置，构建日志在 stderr。无来源声明的上传不能沿用旧构建身份。

作用域仅为共享这份安装/维护状态、经过此 Node 调度器的入口。独立旧服务、另一套安装、直接运行命令和设备内进程不在该保证里。更新前仍须确认实际参与者。真实掉电、生产更新、旧 Mac 节点、rhodes1 外部访问及真机回归另验。实际提交与证据见下文。


## 本批验收与入口

已发布 harness `f78554a`，主体9fb738b；五项actual CI与NODE-DOCTOR所列相同，先验证后同SHA推main。真实隔离队列→独立worker→指定代码目录发布→维护marker阻止普通任务→原run rollback→等待任务恢复通过。更换data-dir保持同安装维护状态，原实验文件未变。Git临时仓、签名拒绝、他项claimed阻断、更新/回滚双故障、后续更新拒绝及原run接续已验；初次fixture别名、Path类型及zsh管线错误均保留原日志并修正，未重跑本地整套。Linux CI明确安装zsh/rsync，避免更新用例被环境缺失跳过。

CLI `pf-node-deploy [--restart] [组件@ref...]` 现在明确走 device-access none；它不要求手机已授权，但实际发布仍检查队列及持久维护屏障。也可 `pf-node --device-access none update_node --git phonefarm@REF`；原run回滚使用 `pf-node --device-access none update_node --rollback RUN_ID`。更新完成不自动重新启动，只有显式--restart才使用原admin-restart接口；未知响应不重发。

Web队列已展示更新未完成/原run或维护状态未知，原有 `/dash/api/jobs` 可提交固定更新作业。专用网页更新/回滚操作和只读维护状态页仍列FLEET-UPDATE-WEB-01，不能说整个网页维护流程已交付。QA：`qa/resume-local-20261026/node-update-agent-summary.json`、`node-protocol-first.*`、`node-protocol-targeted.*`、`deploy-helper-targeted.*`、`update-waiting-ui.*`。原件留本机，未操作真实部署服务或设备。
