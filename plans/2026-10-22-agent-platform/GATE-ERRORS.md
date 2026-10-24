# BASE-GATE-JSON-01：外层准入拒绝

2026-10-25，实施合同。仅补 phonefarm devlock gate 的可读结果，不改锁所有权、排队、producer 格式或 Node 恢复判定。

## 输出合同

所有 gate 在子命令 dispatch 前确定拒绝的路径，stdout 输出一行 JSON：

```json
{"schema":"phonefarm/devlock-error@1","stage":"admission","code":"devlock_wait_expired","error":"具体诊断","command":"capture","command_started":false,"exit_code":2}
```

保留原 stderr 诊断与退出码2。明确兼容变化：非 JSON 调用在这一类失败时也新增机器行；不复制各命令的 --json 解析，不新增旗子。command 只保存顶层子命令名，不回显整个 argv/env。成功、producer 自己的输出和已开始后的失败保持原状。

| code | 触发 |
|---|---|
| invalid_lock_arguments | strip_lock_args 拒绝；不表示 producer 的全部参数已检查 |
| lease_storage_unavailable | gate/acquire 的 require_ready 失败 |
| devlock_unavailable | 找不到脚本，或 acquire 未正常返回0/1 |
| inherited_instance_unverified | 已有 DEVLOCK_INSTANCE_ID 的 renew 未返回0，不创建新实例 |
| devlock_wait_expired | acquire 明确返回1且本次等待预算不足下一轮 |
| devlock_owner_invalid | acquire 返回0但 owner JSON/schema/label/holder_pid/instance 无效 |

从确定分支传递 code，不解析错误字符串，不添加笼统 retryable:true。

command_started:false 只表示本次 gate 未放行子命令。外层脚本可能写过队列元数据，其他 job/旧 attempt/父会话状态未知；不能推断无设备副作用、未获得锁、已经释放或恢复完成。坏 owner 不自动清理。spawn/wait、heartbeat/release 是放行后的路径，不补 admission JSON。进程崩溃、无回包仍未知。

## 客户端与验证

已静读 GOL evaluate/optimize/enter_scene、PF MCP/experiment/parallel、Node eval/session 等实际消费者：它们以退出码或专属 schema 判断，不以 stdout 非空当成功。未承诺仓外私有脚本兼容。Node 合并日志和网页原文本展示已能呈现这一行，日志整体不是单一 JSON 文档；不新增推断卡片或自动重试。

一条现有隔离宿主夹具扩展验证六类拒绝、capture 前置拒绝、普通非JSON调用，以及成功/已启动失败不被误标；运行既有 devlock 单元回归。重用现有构建缓存，签名后运行；不跑全套长集成或真机。BASE-07 的所有 producer 错误统一仍未完成，本批只收口明确的外层缺口。
