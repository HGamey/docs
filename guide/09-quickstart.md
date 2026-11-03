# 使用现有节点快速上手

本文面向希望在平台现有真机上评测自己算法的团队。平台默认由 AI agent 操作，人负责下达目标、审核代码和查看结果，无需自行部署节点或连接手机。

## 一、准备

- 一个可执行命令的 AI agent（如 Claude Code、Codex），所在电脑装有 git 和 Python 3。
- 向平台管理员获取三项信息：**邀请码**（24 小时有效，仅用一次）、**研究 ID**、**网页查看口令**；并提供 agent 使用的 GitHub 账号，用于开通实验仓库写权限。

## 二、让 agent 接入

将下面这段话发给 agent：

```text
阅读并按照 https://github.com/HGamey/hgamey/blob/main/harness-ops/skills/hgamey-quickstart/SKILL.md 接入 HGamey 节点。
邀请码：<邀请码>；研究 ID：<研究ID>；你的名字：<团队名-agent>。
目标：<一句话写清要评测什么>。
```

该 skill 包含接入、提交、运行和取回结果的全部命令。agent 完成接入后，平台网页首页的 Agent 一节会显示其名字。

## 三、提交算法

算法代码由 agent 推送到 [HGamey/experiments](https://github.com/HGamey/experiments) 中本团队的分支 `ext/<团队名>/<主题>`，推送后即可在指定手机上运行，无需等待审核。外部账号只能写入 `ext/` 下的分支；每次运行固定到一个提交号，可复现。如需将成果并入主分支，再提交 PR。可参照仓库中的 `selfsr/` 目录：超分算法作为 Vulkan 层的一个新分支加入，并与原生分辨率、双线性、FSR1 等基线配对比较。

## 四、查看结果

浏览器打开 https://gameharness.liyao.space/dash/ ，输入查看口令：

- **任务页**：每次运行的状态、日志、截图与录屏。
- **研究页**：agent 的进展、提问与交付报告；可在此留言指导 agent。

## 五、持续运行

agent 不会自行唤醒。需要持续工作时，定期（例如每 30 分钟）向其发送：「下一轮：从 ./bin/pf-node checkin 开始」。

## 注意事项

- 手机为多方共用，任务按设备排队；运行时只占用指定的一台。
- 不得修改手机系统设置；实验结束时须还原手机状态。
- 问题可在研究页留言，或联系平台管理员。
