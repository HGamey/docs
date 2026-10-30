# HGamey 部署

![部署拓扑](diagrams/08-deployment.png)

系统结构见 [07 系统架构](diagrams/07-architecture.png)；新建节点的通用步骤见 [HGamey/hgamey 的 README](https://github.com/HGamey/hgamey#部署)。

## 机器

| 机器 | 作用 | 跑什么 |
|---|---|---|
| **mbp1**（macmini3，Mac mini M4） | 生产节点 | 节点服务（网页看板、研究、任务队列、设备层），Cloudflare 隧道。三台手机都插在这台上 |
| **rhodes1**（Linux） | 测试节点、agent 运行地 | 两个测试节点实例和展示页；所有 agent；team-loop 定时唤醒 agent；meiso-board 给人发留言和待办；到 mbp1 的 SSH 隧道 |
| **研究工作站**（Mac） | selfsr 研究 | 实验、报告、对外报告页；不跑节点服务 |

## 手机

都用 USB 直连 mbp1。

| 手机 | 系统 | 连接 | root |
|---|---|---|---|
| 红魔 11 Pro+（NX809J） | Android 16 | adb | 是 |
| 荣耀 AAP-AN00 | Android | adb | 否 |
| 华为 EMA-AL00U | HarmonyOS | hdc | 否 |

## Agent

都在 rhodes1 上运行。

| agent | 职责 |
|---|---|
| claude-lead | 首席开发者 |
| glm-ops | 部署、运维 |
| glm-researcher | 研究 |
| kimi-researcher | 研究 |
| Claude-Sub | 前端开发 |
| Codex-Sub | 后端开发 |
| kimi-web | 前端功能开发 |
| kimi-simplify | 代码审查 |

前四个注册在平台上，各有自己的 token，在网页首页可见。

## 云服务

| 服务 | 用途 |
|---|---|
| Cloudflare | 对外地址 `https://mbp1.liyao.space` |
| GitHub | HGamey/hgamey（代码）、experiments（实验脚本）、docs（文档） |
| 大模型中转 | 给 agent 提供 GLM、Kimi |
| Vercel | selfsr 对外报告页 |

## 怎么用

- **人**：打开 `https://mbp1.liyao.space/dash/`，口令向负责人要。
- **agent**：设 `PF_NODE_URL` 和自己的 token 文件，每轮从 `pf-node checkin` 开始。
- **部署**：main 的最新提交过了 CI 后，先部署 rhodes1，验收后在 mbp1 上运行 `python3 deploy.py mbp1 --ref <提交>`。回滚就是部署上一个提交。

## 出问题时

| 现象 | 处理 |
|---|---|
| 网页打不开 | 在 mbp1 上看节点服务是否在跑；服务挂了会自动重启 |
| 任务卡住 | `pf-node --admin reset <任务ID> --note <说明>` |
| 手机离线 | 重插 USB，用 `adb devices` / `hdc list targets` 确认 |
| 部署后出错 | 部署上一个提交 |
