# 灰层未支持模式的执行准入

状态：2026-10-23，PRODUCER-RECOVERY-03B4A 实现前冻结。源码与宿主验收限定；不部署、不操作手机、不重新开放 native 注入。

## 问题与边界

当前生产 `dispatch_ownership.h::contract_valid` 必定拒绝缺失的 source/layout/target/compute 合同；Android 禁止 host fixture。默认 copyprobe/upop 工件没有可执行路径。自定义 SO 的能力未知，ABI 检查或离线 manifest 校验不构成执行许可。`gray-ab --mode copyprobe` 却会先摘层、跑两个 none 基线，旧 upop 编排同样先跑 passdump 基线；这些副作用必须在入口前避免。

本批拒绝这些未支持的执行请求，保留工件构建、inspect/verify、旧 marker 的 copies/subs/disabled 和 artifact counts/ineligible_reason、shot-diff 等离线分析。既有原件不重写，不凭此声明真机可用。其他 gray/frame-audit/whitebox 的 raw 路径继续在后续 producer/legacy 待办中处理。

## 返回合同

统一 JSON `schema=phonefarm/gray-mode-admission@1`，包含 requested_mode、artifact_source（default/custom）、status=unsupported、reason（默认 `missing_source_layout_target_compute_contract`，自定义 `artifact_capability_unverified`）、execution_started=false、device_commands_started=false、collection_ok=false、recovery_status=not_started、job_recovery_complete=false、game_state_restored=false。退出码 2。拒绝即返回该 JSON，不能写为测量失败、恢复完成或零耗时收益；不创建实验目录或恢复记录。

CLI 在外层设备锁排队/选择设备之前拒绝，公共 `GrayLayer::mount_mode` 也在任何属性写入前拒绝。默认与显式 `--so` 分开归因，不增加绕过选项。入口识别要按参数值消费，不把 `--serial copyprobe` 当模式；gray-ab 重复 --mode 按现有最后一次值处理。help、recover、loadop 和其他模式原语义保留。锁参数不导致拒绝路径进入排队或设备探测；未知/缺值参数仍返回参数错误，不放行设备操作。

## 已知 shell 入口

`enable_layer.sh copyprobe/upop` 在 setprop/mount 之前拒绝；`genshin_upop_ab.sh` 默认 A/B 在 mkdir/off/基线之前拒绝；显式 `REF_ARM=1` 只读参考臂保持既有路径，但不将其称为已迁移的 owned session。`genshin_quality.sh begin upop/copyprobe` 在 mkdir/off/启动应用之前拒绝。passdump 与 finish 本批不改实验语义，旧恢复义务另列。Shell 的 OUT 自定义路径不能被当作已核验工件；若替代默认构建目录选择工件，归为 custom 未核验。

## 最小验证

相关参数选择/公共 mount 的少量单元测试；已签名实际 CLI 在隔离 fake ADB/devlock 下验证默认 copyprobe、自定义 upop、锁选项和参数值混淆，确保零 transport/零锁动作/零实验输出。同样覆盖三个 shell 入口的提前拒绝，必要时用拦截器证明 REF_ARM/reference 未误挡，不运行完整 reference 实验。保留原始 stdout/stderr/退出码和失败，不重复此前完整 loadop/eval 链。不新增性能或 Android 恢复结论。
