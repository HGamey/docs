# 灰层未支持模式的执行准入

状态：2026-10-23，PRODUCER-RECOVERY-03B4A 已发布源码155506f。实现前冻结合同，以下附宿主验收范围；未部署、未操作手机、未重新开放 native 注入。

## 问题与边界

当前生产 `dispatch_ownership.h::contract_valid` 必定拒绝缺失的 source/layout/target/compute 合同；Android 禁止 host fixture。默认 copyprobe/upop 工件没有可执行路径。自定义 SO 的能力未知，ABI 检查或离线 manifest 校验不构成执行许可。`gray-ab --mode copyprobe` 却会先摘层、跑两个 none 基线，旧 upop 编排同样先跑 passdump 基线；这些副作用必须在入口前避免。

本批拒绝这些未支持的执行请求，保留工件构建、inspect/verify、旧 marker 的 copies/subs/disabled 和 artifact counts/ineligible_reason、shot-diff 等离线分析。既有原件不重写，不凭此声明真机可用。其他 gray/frame-audit/whitebox 的 raw 路径继续在后续 producer/legacy 待办中处理。

## 返回合同

统一 JSON `schema=phonefarm/gray-mode-admission@1`，包含 requested_mode、artifact_source（default/custom）、status=unsupported、reason（默认 `missing_source_layout_target_compute_contract`，自定义 `artifact_capability_unverified`）、execution_started=false、device_commands_started=false、collection_ok=false、recovery_status=not_started、job_recovery_complete=false、game_state_restored=false。退出码 2。拒绝即返回该 JSON，不能写为测量失败、恢复完成或零耗时收益；不创建实验目录或恢复记录。

CLI 在外层设备锁排队/选择设备之前拒绝，公共 `GrayLayer::mount_mode` 也在任何属性写入前拒绝。默认与显式 `--so` 分开归因，不增加绕过选项。入口识别要按参数值消费，不把 `--serial copyprobe` 当模式；gray-ab 重复 --mode 按现有最后一次值处理。help、recover、loadop 和其他模式原语义保留。锁参数不导致拒绝路径进入排队或设备探测；未知/缺值参数仍返回参数错误，不放行设备操作。

## 已知 shell 入口

`enable_layer.sh copyprobe/upop` 在 setprop/mount 之前拒绝；`genshin_upop_ab.sh` 默认 A/B 在 mkdir/off/基线之前拒绝；显式 `REF_ARM=1` 只读参考臂保持既有路径，但不将其称为已迁移的 owned session。`genshin_quality.sh begin upop/copyprobe` 在 mkdir/off/启动应用之前拒绝。passdump 与 finish 本批不改实验语义，旧恢复义务另列。这些 shell 没有自定义 SO 选择入口：enable_layer 的工件目录固定，另两脚本的 OUT 只指定结果目录；不因结果目录改变而归为 custom，也不新增工件绕过选项。

## 最小验证

相关参数选择/公共 mount 的少量单元测试；已签名实际 CLI 在隔离 fake ADB/devlock 下验证默认 copyprobe、自定义 upop、锁选项和参数值混淆，确保零 transport/零锁动作/零实验输出。同样覆盖三个 shell 入口的提前拒绝，必要时用拦截器证明 REF_ARM/reference 未误挡，不运行完整 reference 实验。保留原始 stdout/stderr/退出码和失败，不重复此前完整 loadop/eval 链。不新增性能或 Android 恢复结论。


## 已实现与验证

main 在锁排队之前调用纯参数准入；gray-layer 直接入口和公共 mount 同样拒绝，gray-ab 删除了不能执行的 none/copyprobe 私有采集链。loadop 主体、recover、工件 tooling 与旧离线分析保留。参数按值消费且与 gate 一致：serial/path 等于 --help 或 --no-devlock 不会免锁或被剥走；真正 help 保留，重复 mode 按最后值处理。这次仅修两个灰入口的参数语义，不声称其他命令已统一。

统一构建签名；6项相关Rust单测一次通过。3次实际CLI一次通过：最后mode为copyprobe且带锁参数/无有效state root；缺失custom SO的upop；默认upop不带--json。均exit2、零ADB/HDC/devlock调用、零实验输出或恢复记录，进程均退出。7项shell fixture一次通过，其中5个拒绝场景零设备命令/零输出目录，另外2个仅在reference/passdump的首条假ADB处截停，证明未误挡，并未执行参考实验。4个shell语法检查通过。未重复旧loadop/eval完整链；loadop主体未改，相关功耗合同单测复用。

源码155506f3407bfbe7a78bece9bdbbd88fe7f9d3f6经actual code-only check120077413135 completed/success（run39851167210）后同SHA直推main并核远端。本机qa/gray-mode-admission-*保存原stdout/stderr/退出码与回执，不构成真机恢复或性能结论。网页继续消费既有作业stdout/产物，没有新增页面合同；未部署在线节点。

MERGE-03限定未支持入口准入完成；03B4/03/JOB-EXEC-03/M3尚未完成。后续清理无调用的旧storeop链，并为可执行的gray-layer挂层及旧reference脚本明确持久owner/停止恢复协议。不能以这批零设备动作的拒绝回执代替它们的恢复验收。
