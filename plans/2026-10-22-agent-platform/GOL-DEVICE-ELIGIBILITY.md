# BASE-01 GOL：设备枚举与 legacy gpu-op 资格修复

2026-10-25，实施合同；先在本机QA冻结再修改源码。范围仅 GOL `src/harness/mod.rs`、`src/main.rs` 内现有测试；旧研究工作树与未跟踪 `src/bin/harmony_touch_probe.rs` 保持原样。后者明确研究专用保留，不产品化。

## 当前合同核对

- PF `src/main.rs:536–569`：ADB stdout 原样输出 `SERIAL STATE`；HDC `list targets` 每条非空/非 `[Empty]` 行加 `hdc:`；iOS `idevice_id -l` 每条加 `ios:`，并明确 USB 枚举不证明 WDA 就绪。本次不实际执行这些命令。
- 共享 `parse_device_list` 同时供 legacy `PhonefarmDriver::probe` 与 `optimize::caps::Prober::devices` 使用。它须保留三族有效枚举，不把 HDC/iOS 从全平台枚举中删除。
- ADB 只接第二列精确 `device`；拒绝 offline/unauthorized/recovery 等状态和无状态普通行。HDC 接裸 `hdc:key` 或第二列大小写不敏感的 `device`；拒绝显式错误状态。iOS 接当前 PF 的裸 `ios:udid` 格式，拒绝额外状态/说明；前缀必须有非空 key/udid。沿用既有 serial 字符约束。
- “枚举中存在”不是测量、root、WDA、租约或载体就绪。保留 `ProbeStatus::Ready`/`can_evaluate()` 的兼容形状，但说明与展示改为枚举事实，避免把它当各子命令资格。

## 具体执行路线与边界

| GOL 路线 | 当前执行入口/依据 | 本次处置 |
|---|---|---|
| 无子命令 legacy shader 演化 | `main.rs:726` 与 `engine/evolution.rs:388` 都构造默认 `PhonefarmDriver`，默认子命令 `gpu-op`；PF `gpuop.rs:28,406–414,1031–1073,1149–1158` 使用 Android headless runner 或 Android refbench、root/设备条件化 | 仅这条主入口使用新的 `ProbeStatus::gpu_op_device_problem(requested_serial)`；HDC/iOS 枚举不授予该 Android 载体资格 |
| `optimize`/goal 等统一 v2 评测 | 独立 `optimize::eval::EvalDriver` 调 `phonefarm eval --request`；`optimize::caps`/plan 依据目标、平台、root、载体与 kind 分层。PF eval 对 v1/shader 转 gpu-op，gray 进受控会话，sysparam/whitebox 各走专用实现，validate-only/qualify-from 可纯离线 | 不把所有 eval 笼统改成 Android-only，不改 EvalDriver 或计划器；共享 parser 的真实状态过滤自然适用 |
| 可选 `PhonefarmDriver::with_subcommand` | 当前生产调用没有覆写默认 gpu-op；仅提供库级入口，不能凭一个枚举判所有未来命令支持 | 不引入通用后端支持表，不对自定义子命令新增猜测门禁 |

显式 `--device` 必须精确出现在这次枚举中且属于 Android；若仅另一台 Android 在线，不能替显式 HDC/iOS/缺席目标满足条件。省略 `--device` 时只判断至少有一个枚举中的 Android；不新增自动选机、改变 PF 的最终 serial 解析或声称多机歧义已解决。

新函数返回 `None` 仅表示该枚举满足 gpu-op 的后端/目标前提；否则给明确原因（目标缺席、目标不是 Android、没有 Android 或枚举不可用）。legacy `device_path_ready` 同时要求该前提和既有子命令支持；控制台/拒绝诊断打印原因，保存的 `HarnessInfo` 增可选 `gpu_op_device_reason`（成功缺席），原字段含义明确为枚举事实/有限前提。保留既有 dry-run、expert 条件和实际 PF 二次检查，不授予新设备操作资格。

## 旧补丁逐块处置

旧 diff 基线 `547462f76a06f5c90b01f60e1ba6b9fc98669fac`，文件保持未提交；这里的行号是旧 diff 的原始 hunk 起点。

| 文件 / 原 hunk | 内容 | 决定 |
|---|---|---|
| harness `106` | `has_gpu_op_device` + describe 换行 | 采用动机，重写为按显式目标的有限 gpu-op 资格，并同时排除 iOS；不搬原“任意非 HDC 即可”的条件 |
| harness `138` | SubcommandStatus match 排版 | 不搬，纯格式 |
| harness `149`、`322` | 删除“PF gpu-op 接收端尚未实现”的过期说明 | 修正当前同处说明，区分枚举、命令、载体和单次实际执行 |
| harness `445`、`454` | stderr iterator 链换行 | 不搬，纯格式 |
| harness `518` | 设备列表状态过滤 | 采用并按当前三族输出补 iOS；不采用会删除全部 iOS 的旧 else 分支 |
| harness `599` | HDC 正向枚举、offline/unauthorized 与 HDC-only 测试 | 合并到三条纯测试，增加 iOS 和显式目标不能借另一台资格 |
| harness `659`、`763`、`845` | 测试表达式换行 | 不搬，纯格式 |
| main `142` | 测量可用性注释 | 采用有限前提说明；不能把有 Android 等同取得有效数字 |
| main `235` | 使用 GPU 后端判据/打印 HDC 提示 | 采用，改为精确目标与 Android/HDC/iOS 原因；保留当前分支已有 expert/dry-run 语义 |
| main `168`、`258`、`287`、`335`、`363`、`449`、`482`、`557` | logger/println/字段/局部表达式排版 | 不搬，纯格式 |

以上覆盖旧两文件 diff 的全部 hunk；不是合并旧 main、旧依赖或整个研究分支。

## 唯一受影响验证计划

三条 `harness::tests::device_*` 纯测试：三族有效枚举；空/错误/离线/未授权列表；gpu-op 后端与显式目标匹配。只编译该 lib test 目标并签名，运行这一过滤组一次，原输出保存 `qa/base-gol-device-test-1.*`。不运行现有会调用真实 PF 的 probe tests，不运行实际主入口、整套测试、设备或生产服务；root另做一次bin编译检查以覆盖main字段变化。

尚未做：真实枚举/评测/部署、optimize 的所有后端执行资格、设备已连接后的权限/载体验证、stat/枚举到实际执行之间的变化保证。它们仍由相应执行端合同处理。

## 交付（2026-10-25）

GOL 761226c64eb535f98ec722750241eb1fe48f2505 经code-only成功后同SHA直推main并核对。
三条纯解析/资格测试单轮全部通过（359项未运行），测试binary已签名；lib编译23.534秒，
实际测试进程0.393秒。root另做一次offline bin cargo check，3.01秒成功，覆盖main报告字段。
没有调用probe/设备或运行主入口，生产binary未更新。旧原件未动、无全套重复测试。
本机原始索引qa/base-gol-device-test-1.json与base-gol-device-publish.json；逐hunk去向和最终diff另存。
