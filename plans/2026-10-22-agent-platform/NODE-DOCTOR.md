# 节点环境诊断

FLEET-DOCTOR-01：同一套宿主诊断供网页 `/dash/doctor`、查看权限 API `GET /dash/api/doctor`、只读 token API `GET /doctor` 和 `pf-node doctor [--json]` 使用。API 必须带明确 node pin，能力名 `node.doctor-v1`。没有自动安装、修复、运行任务、设备枚举或 lease 探测。

响应 `schema=hgamey/node-doctor@1`，包含 `node_id`、Unix 秒 `checked_at`、`scope=host-only`、各状态数量 `summary`、`checks` 和 `limitations`。每项有稳定 `id`、`category`（connection/runtime/tools/storage/permissions/devices）、`status`（ok/warning/error/unknown/not_applicable）、`title`、`message`、`observed_at`、`evidence`、`remediation`。修复项是文字及可选人工执行命令，不执行这些建议。HTTP 200 表示诊断读取成功，不代表所有检查通过。

读取服务启动身份、缓存的程序身份及文件指纹；不触发 runtime refresh。检查配置工具能否在宿主找到、数据根和 jobs/导出缓存的访问权限及可用空间。macOS 仅对 Mach-O 文件进行有界 codesign 验证。缺工具、过期程序身份、错误文件格式各自说明，不能因此把可访问节点说成离线。签名与文件状态仍是时间点观察，不保证随后启动的内容。设备授权、root、真实功能资格明确未知，另由持有 owner/lease 的设备工作流检查。

CLI 单次请求，无隐式重试。`--json` 输出相同报告；退出 0 表示没有发现已知 warning/error，2 表示报告有 warning/error，1 表示连接/目标/协议错误。unknown 不被伪造为通过。网页初次读取和人工重新诊断，不轮询；失败保留上次报告并标明旧记录。

这只关闭 FLEET-05 的宿主子项，设备诊断及真实跨主机部署仍开放。源码验收见下文。


## 本批验收

已发布 harness `f78554a79d6c48c4949f308f191d659d5103690e`（主体9fb738b）。本地158协议首轮157过，新worker链产品断言通过但测试清理句柄写错；修正后仅该用例定向通过。远端 Python3.11/3.14 两套协议及页面语法、cgroup-kernel、pf-shared-kernel、code-only 五项全部通过：[节点检查](https://github.com/HGamey/harness-ops/actions/runs/40284822539)、[代码检查](https://github.com/HGamey/harness-ops/actions/runs/40284822523)。

实际只读 token、dashboard cookie、node pin、页面/资源/API接线通过；monitor request、MCP、runtime refresh 调用均0，last_view=0、wake=false。CLI实际HTTP链及错误计数不一致拒绝通过。合成HTTP下1440/390浏览器布局、loading/重试/401/错目标已验；实际后端的浏览器连接因IAB不可用、Chrome超时未完成，不把两者合并为已验。QA在任务 `qa/resume-local-20261026/doctor-ui`、`doctor-actual-ui`、`doctor-validation.json`。

部署后用 `pf-node --url URL --node-id ID doctor --json`，或打开该节点 `/dash/doctor`。工具缺失只影响相应检查；设备授权/root/真机资格仍未知。服务的 adb 配置已改为 ADB_BIN/PATH，移除硬编码 Homebrew 路径。
