# 节点更新的网页与状态入口

FLEET-UPDATE-WEB-01 复用 [安全更新](NODE-UPDATE.md) 与既有作业队列。网页 `/dash/updates` 与 CLI `pf-node update-status [--json]` 读取同一状态；写入仍为已有 `/dash/api/jobs` 或 `pf-node --device-access none update_node ...`。本批不增加可执行任意 shell 的新入口，不自动重启服务。

只读 API 为 `GET /node-updates`、`GET /dash/api/node-updates`，必须明确 node pin；能力名 `node.updates-v1`。响应 `schema=hgamey/node-updates@1`，含 node_id、observed_at、scope=host-only、layout（phonefarm/game_opt_loop/harness_ops/state_dir）、updater、maintenance、limitations。

updater 包括 available、reason、executable、identity_state、observed_at、contract=hgamey/node-update@1。只有配置入口是当前服务目录中的安全 updater、模块路径相符，且文件仍与启动观察一致才可用；旧/自定义入口或变更后身份不明不由 bins 中出现 update_node 自动推断支持。读取不执行程序、不刷新身份、不查看手机、不申请 lease、不唤醒截图。

maintenance 包括 state=clear/blocked/unknown、run_id或null、reason、observed_at、all_participants_verified=false。clear仅表示没有读取到已记录的安装维护义务，不证明其他参与者都停止。未知或损坏状态不给虚构run ID；网页禁写。blocked只允许对记录的原run进行精确回滚。状态不轮询，用户显式刷新。

网页提交单组件和明确ref，先展示固定参数，经过掌舵权限、目标pin、安全更新能力及updater可用性检查。请求体固定 host-only，显式 `resources:[]`；提交前把原request ID、目标和完整body写入独立pending槽（无凭据）。保存/清除共用跨标签页 Web Lock，网络请求不占锁；浏览器缺此能力时禁新提交，查回仍可用且保留不能安全清除的记录。未知响应、5xx、冲突或坏JSON不自动重发，不另开新请求。刷新后可按原ID查询，只有返回任务的request_id、bin、argv、device_access和公开queue.resources吻合才解除pending并链接任务页。只读用户可以查看/查回，但不能提交。详情/日志继续使用现有jobs页面，不创建另一套作业状态。

查回404不自动清除保留记录；需核对原入口、身份和请求。节点公开作业并不包含原spec.exe，本页不声称能核验服务未公开的字段；页面自身固定不传exe，节点原有请求幂等负责保存完整请求身份。脚本更换后updater为stale，页面继续允许读旧任务，但不执行新文件；维护者核对安装并明确重启服务后再检查，不自动刷新入口身份。

CLI状态查询仅一次，不重试；JSON保持API报告，读取成功exit0（maintenance阻断不伪装成传输失败），协议/目标/连接错误exit1。实际写作业的成功、回滚、进程退出和服务重启仍是不同事实。生产部署、真实断电及设备影响继续另验。

## 实施与验收

实现 `4b4a116`、更新入口锁修复 `463a721`，已随最终 `f5a7ea4` 发布main。最终code-only 40293859867与node-tests 40293859441（Python 3.11/3.14、两个Linux kernel job）全部成功。一次本地159项协议99.367秒全部通过；实际隔离HTTP/CLI验证pin/auth、未知维护、文件替换stale及读取不刷新/不唤醒。14组完整页面脚本定向场景通过，覆盖只读/权限/未知响应、原ID查回、错回执/资源声明、存储失败以及两个独立页面交错保存/清除的竞态。

实际静态页/API接线15个GET、0 POST，页面与资源逐字节对应本次工作树；未登录303、API401、缺pin400、错pin409。该HTTP夹具最初两次预期码写错，修正断言后通过，负记录保留。CUA唯一尝试返回浏览器不可用，390/1440视觉验收未完成；不以脚本或HTTP检查代替视觉检查。跨标签页验证使用共享storage/异步锁的两个页面VM，不冒充已在真实浏览器验证Web Locks。

本地证据在恢复任务 `qa/resume-local-20261026/node-updates-*`、`updates-ui-check.*`、`updates-actual-http/`。没有生产部署/手机操作，页面出现或任务受理不等于更新完成。

首次CI 40292238149仅Python 3.14失败，暴露实际父dispatcher仍短暂持有intake锁、子updater立即失败的竞态。发布检查改为单调时钟最多5秒等待原锁，取得后仍核任务资格；install锁保持立即拒绝，不重放作业。定向真实父子进程回归先确认等待期间文件未变，再释放锁后完成。原失败日志保留，最终同SHA各CI通过。
