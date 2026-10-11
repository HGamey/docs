# Harness 运行入口与验收边界

此页针对本轮 draft 分支；main 尚未合入这些接口。具体 PR、提交和测试结果见
[实现进度](2026-10-11-implementation-progress.md)。指定并记录实际 GOL/PF
可执行文件的 SHA 与源码提交；不能把旧二进制当作最新源码的验收。

## 1. 从候选开始的离线入口

准备一份符合 candidate schema 的 JSON，然后调用：

```sh
game_opt_loop candidate-prepare run --candidate candidate.json --phonefarm-bin /absolute/path/to/phonefarm --out prepared-new
game_opt_loop candidate-prepare inspect --session prepared-new
```

`prepared-new` 必须不存在。驱动冻结候选、两端可执行文件与预算，实际依次运行
GOL compile/verify 和 PF adapt-storage/verify-adaptation，并保留原始输出、退出
状态和工件身份。没有重试或固定 shader 回退；中断的 InFlight 保留 UNKNOWN。

| 结果 | 可以说明什么 | 下一步 |
|---|---|---|
| PREPARED_OFFLINE | 四个编译/适配阶段完成，冻结字节及本地重放一致 | 单独验收 native 构建、挂点、GPU 等价和真机实验 |
| BLOCKED / rejected / failed | 必需阶段未完成，原始拒绝或错误保留 | 依据具体 receipts 修正输入或补能力；不以摘要改成通过 |
| UNKNOWN | 中断或进程/工件状态无法确认 | 保留会话与原始证据，先核对拥有的进程和副作用 |

`offline_compilation_adapter_complete` 只描述四阶段；请求 native build 后若该
阶段 BLOCKED/NOT_RUN，`preparation_chain_complete=false`。当前准备入口不接受
已存在 SDK 的 native 构建。显式缺失 SDK 的检查使用会话内私有空 SDK，保留
实际 PF 的 missing-tool 拒绝，不能称作 Android 构建成功。

候选准备与独立确认都是独立执行入口。campaign 将算法候选接入实际游戏评测、
连续画质和恢复的自动路径仍需贯通；执行这些命令不能称为任意商业游戏的
自动寻优闭环已经成立。

PF 已有显式 `--process-supervision inherited-group` 接口。外层须独占新进程组，
在成功退出后确认组消失再重读验证；工件自身始终不能证明外层停止。该接口的
controlled fake NDK 验证不替代实际 NDK 或 GPU 验收。
PF 必须是该新进程组的 leader，且显式提供 `--ndk`；在普通共享 shell 进程组中
直接加这个 flag 会被拒绝。

## 2. 探索结束后的独立确认入口

确认不复用探索结果来证明候选。PF 先冻结规则、候选、负载和至少三轮基线；GOL
注册轮数与顺序；PF 才能收集每臂独立原始数据。接口顺序为：

```sh
phonefarm confirmation-driver prepare --request request.json --out session-new
game_opt_loop confirm freeze --plan session-new/plan.json --out frozen.json
phonefarm confirmation-driver collect --session session-new --frozen frozen.json
game_opt_loop confirm judge --frozen frozen.json --samples session-new/samples.json --out confirmation-report.json
```

以上是接口形状。真实 transport 缺 `--execute` 时只校验，不产生设备操作，
prepare 也不会生成供下一条命令使用的 session/plan，因此这组命令不能作为
可直接连续运行的真实设备示例。显式
fake transport 可以执行模拟整链路，但来源始终 synthetic，GOL 结论 BLOCKED。
fake prepare/collect 顶层为 `SIMULATED_BLOCKED`；exit 0 只表示机械执行完成。
单轮观测和 GOL judge 仍是 exit 1 / BLOCKED，不能据 shell 成功码宣布实验通过。
不可将 fake request 改个标签用于真机。驱动范围是明确 serial 的 Android
root/sysparam/KGSL/电池；HDC、未知 root 或能力明确拒绝。

真实执行的 request 必须先绑定实际设备、前台游戏、确定性负载、时长、候选及
统计假设。初次接回一加按 [真机验收顺序](oneplus-acceptance.md) 进行，只读资格
通过后再安排持租约的控制验证。prepare 与 collect 之间有注册间隙；驱动会
复核 boot、原始快照和冻结字节，不能据此推断场景从未变化。

失败或中断后不能重跑同一 heldout ID。恢复前须确认拥有的远端 worker、子组和
未完成 launch intent；本机 ADB 被杀不代表远端空闲。显式 reconcile 只恢复
原设置、核验停止与完整快照，并永久关闭旧会话；不会将旧样本改成有效观测。

复现采集协议的离线机制，进入所选 PF 源码的 `src/` 目录运行：

```sh
PHONEFARM_DRIVER_GOL_BIN=/absolute/path/to/game_opt_loop cargo test --test confirmation_driver_cli complete_actual_driver_all_control_categories_retains_synthetic_and_cannot_replay
```

测试生成新 fake transport 会话，调用实际 GOL freeze/judge，并逐臂调用实际
观测 CLI 对照报告。结果保持 synthetic/BLOCKED，不调用设备或模型。已归档的
会话是证据：测试可能故意改坏原始候选/负载以检验冻结，不能直接重跑旧会话。

## 3. 查看结果

离线展示指定两个仓库根，并关闭设备 fleet 轮询：

```sh
game_opt_loop serve --root /absolute/path/to/game_opt_loop --phonefarm-root /absolute/path/to/phonefarm --port 8794 --no-fleet
```

打开 `http://127.0.0.1:8794`。用于展示的 runs/tasks 必须位于这些配置根目录内；
页面不会沿任意外部路径寻找缺失工件。

只读面板的评测页保留完整 receiver、A/B、资格与来源；Cases 页汇总确认报告和
模型阶段。只扫描配置根目录内有界文件，展示原始 JSON 字节、SHA 与关联错误。
模型阶段 completed、保存的 PASS、hash 匹配均是对应范围的声明，不授予实验
资格。负结果、缺项、UNKNOWN、恢复缺口和未核验成本应与通过结果一同展示。

## 4. 当前交付门槛

| 阶段 | 离线已验证的机制 | 仍需实际证据 |
|---|---|---|
| 候选准备 | 实际编译重放、有限 SPIR-V 适配与接收者独立校验 | Android 构建、Vulkan 验证、GPU 输出等价 |
| 灰层 | host 下录制代/逐 dispatch 资源保留与退役 | 一加目标/source/layout/compute 合同、命中与 GPU 完成 |
| 采样与确认 | fake transport 驱动协议、原始重建、冻结与 judge | 真实温度/放电/控制/恢复、游戏场景重复性、完整轮数 |
| 连续画质 | 配对账本、空间/色偏/相邻残差及坏帧门禁 | 真实同步 LR/HR/候选序列及来源，插帧中间真值 |
| 模型路线 | 五阶段工件与专家/预算/恢复协议 | 真实训练、量化、Android 推理与计量 |
| 案例 | 完整原始证据只读展示 | 经独立确认的真实游戏收益与限制 |

本轮设备操作、真实训练与模型实验 API 调用均为 0。系统 epic 保持开放；离线
测试数量、root 成功、工件 hash 或页面存在均不能替代上述实际证据。
