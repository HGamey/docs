# JOB-07 安全点暂停设计

2026-10-27。设计笔记：producer 级安全点暂停（gray-control session 多阶段操作间的受控暂停/恢复）。

## 已覆盖

- 队列级暂停/恢复：JOB-PAUSE-01（harness 613e120/fecf550）。
- 取消链：SIGINT→TERM→KILL（225cb5c）；超时预算（4b7c75a）；磁盘软保护（757d3ff）。
- 非幂等不重放：同 ID 不重放（58fc0dc）；输入准备失败不重试（dd00340）。

## 未覆盖：producer 级安全点暂停

gray-control session 多阶段操作中，用户/Agent 请求暂停——当前只能 cancel（杀进程）。

### 设计

1. **pause_requested**：session state 新增布尔；producer 在每个 safe point（阶段边界）检查。
2. **safe point**：阶段边界（deploy→install→initialize→scene→capture→recover）。单命令 job 无 safe point。
3. **暂停行为**：不杀进程、不释放 lease；当前阶段完成后不再开始下一阶段。
4. **恢复**：清除标志，继续下一阶段。
5. **接口**：`session-pause <id>` / `session-resume <id>`；dash session 页按钮。

### 子批

| 子批 | 内容 | 范围 |
|---|---|---|
| SAFE-PAUSE-01 | session state + safe point 检查 + pause/resume 路由 | PF Rust + harness |
| SAFE-PAUSE-02 | Web session 页暂停/恢复按钮 + 深色/390 截图 | harness JS |
| SAFE-PAUSE-03 | 真机 gray-control session 暂停/恢复实测 | 设备依赖 |

### 前置

JOB-PAUSE-01（已完成）；灰层 session 生命周期合同（GRAY-CONTROL-SESSION）；SAFE-PAUSE-03 需设备。
