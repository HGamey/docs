# 节点更新的网页与状态入口

FLEET-UPDATE-WEB-01 复用 [安全更新](NODE-UPDATE.md) 与既有作业队列。网页 `/dash/updates` 与 CLI `pf-node update-status [--json]` 读取同一状态；写入仍为已有 `/dash/api/jobs` 或 `pf-node --device-access none update_node ...`。本批不增加可执行任意 shell 的新入口，不自动重启服务。

只读 API 为 `GET /node-updates`、`GET /dash/api/node-updates`，必须明确 node pin；能力名 `node.updates-v1`。响应 `schema=hgamey/node-updates@1`，含 node_id、observed_at、scope=host-only、layout（phonefarm/game_opt_loop/harness_ops/state_dir）、updater、maintenance、limitations。

updater 包括 available、reason、executable、identity_state、observed_at、contract=hgamey/node-update@1。只有配置入口是当前服务目录中的安全 updater、模块路径相符，且文件仍与启动观察一致才可用；旧/自定义入口或变更后身份不明不由 bins 中出现 update_node 自动推断支持。读取不执行程序、不刷新身份、不查看手机、不申请 lease、不唤醒截图。

maintenance 包括 state=clear/blocked/unknown、run_id或null、reason、observed_at、all_participants_verified=false。clear仅表示没有读取到已记录的安装维护义务，不证明其他参与者都停止。未知或损坏状态不给虚构run ID；网页禁写。blocked只允许对记录的原run进行精确回滚。状态不轮询，用户显式刷新。

网页提交单组件和明确ref，先展示固定参数，经过掌舵权限、目标pin、安全更新能力及updater可用性检查。请求体固定 host-only；提交前把原request ID、目标和完整body写入独立pending槽（无凭据）。未知响应、5xx、冲突或坏JSON不自动重发，不另开新请求。刷新后可按原ID查询，只有返回任务的request_id、bin、argv吻合才解除pending并链接任务页。只读用户可以查看/查回，但不能提交。详情/日志继续使用现有jobs页面，不创建另一套作业状态。

CLI状态查询仅一次，不重试；JSON保持API报告，读取成功exit0（maintenance阻断不伪装成传输失败），协议/目标/连接错误exit1。实际写作业的成功、回滚、进程退出和服务重启仍是不同事实。生产部署、真实断电及设备影响继续另验。

本项实施中，验收与提交完成后回写；不能以页面存在或HTTP成功代替已更新的证明。
