# 一加 Ace 5 真机验收顺序

设备历史：PKG110 / OnePlus Ace 5 / Snapdragon 8 Gen 3 / Adreno750v2；原厂 ColorOS Android 16；之前 Magisk 30.7 root 验证成功。以上是 2026-10-06 记录，接回后重新读实际状态。不要用红魔 NX809J 的控制点或灰层挂点作为一加已验证事实。

## 1. 只读资格与基线（0 API Token）

- 指定目标 serial；确认在线、实际型号/版本、root、SELinux、前台包与游戏版本。
- 使用当前源码构建并签名的 phonefarm；保存两仓 commit、CLI 版本、工件 hash。
- 读可用采样与控制点，区分节点存在/可写/回读成功/持续生效；记录充电状态与轨道。
- 原神若未安装或登录，只做到设备/载体资格；不把登录页采样当大世界负载。
- 确认游戏内画质、帧率、地点/时间/视角、温度和风扇；触控负载自检运动，记录每游戏 profile。
- 原设置基线重复采样并按冻结规则判定；不稳定或缺必需指标先补证据，不启动寻优。

### 游戏负载的温度准入

2026-10-12 真机读数确认：不能把 CPU/GPU 结温、电池温度与机身传感器混为一个最大值，也不能要求正常游戏负载始终低于 40°C。`loaded_steady_v1` 的 producer/receiver 已在未合并的 PF #27 实现并完成离线 gate；三次真机配置探测仍均在写入前拒绝，尚未获得控制或性能资格。旧 `legacy_cold_v1` 记录保持原规则。调用方只选版本化 preset，不能在看到候选结果后修改数值。

| 域 | 新规则的来源 | 本机已读配置对应的上限 |
|---|---|---|
| CPU | 最低有效 passive trip 与 HAL severe 阈值的较小值，减 5°C | HAL 95°C → 90°C；更低的实际 trip 会进一步降低上限 |
| GPU | 同上，减 5°C | passive/HAL 95°C → 90°C |
| 机身传感器 | 同上，减 3°C | `skin-msm-therm` passive/HAL 60°C → 57°C |
| 电池 | 独立接收端上限；`battery/temp` 的十分之一摄氏度单位单独转换 | 45°C |

这些是本次测试规则，不是普适的物理安全温度声明。先读取并冻结实际 sensor path/type/unit、trip、HAL 原始记录与映射；缺失或未知拒绝。无 passive 合同的 shell 传感器保留诊断记录，`socd` 等不同语义字段不参与结温最大值。HAL 状态必须为 0–2 且没有 override，原厂热管理不修改。

进入测试还需至少 30 秒、至少 6 次完整读数，间隔不超过 6 秒、各传感器范围不超过 3°C。从该窗口冻结均值 ±2°C 的比较范围，运行中复核同一范围和绝对上限。温度漂移会降低可比性；一个低温点不能替代稳定窗口。新模块先用于控制能力验证；完整确认还需实际整窗 ENV/观测与恢复，probe 结果本身不授予性能资格。`796ea31` 的 uptime deadline 修复通过 671 项本地测试；第三次实际探测已满足间隔要求，但 CPU zone26 的 7.7°C 跨度仍触发拒绝，没有实际参数写入。该失败不重标为成功。

## 2. 受控能力验证（0 API Token）

- 持有完整实验设备租约；冻结目标、预算、指标来源、冷热门槛、时长/轮数与停止规则。
- 动态灰层先完成 Android/Vulkan 构建和 GPU 生命周期验收：当前 draft 工件 `deployable=false`。phonefarm #23 已交付录制代和逐 dispatch 不可改写资源组的 host 验证，但没有真实 GPU 证据；生产拒绝没有 source/layout/target/compute 合同的旧启发式路线。每次替换的 descriptor 与依赖图像资源须保留到合法录制退役且全部提交完成。
- 先只读挂层验证目标 Vulkan 管线，再做 copy 等价验证；记录命中而非只看进程存活。
- 系统控制先应用一个已资格化控制点、回读/持续观测、恢复核验；不能持续锁定就如实降级。
- 停充只有实际电池放电与窗口证据成立才使功耗进入比较；失败保留功耗 null。
- 获取符合 manifest 的真实配对连续帧；未知对齐不能获真游戏画质资格。
- refbench 的 native LR/HR 采集单独运行；其额外 HR 绘制和读回改变负载，`quality_only` 结果不参与普通帧时或功耗比较。
- 恢复前必须证明本次拥有的远端 worker 与子进程组已停止；本机 ADB 超时不证明远端停止。身份缺口、未完成 launch intent 或恢复核验失败时保留 state 和排查证据，阻止后续实验；不宣称 REVERTED。

## 3. 整链路候选与确认

1. 先用确定性本地候选走 campaign → 静态检查 → 动态挂点 → 交替 A/B → 连续质量 → 资格/统计 → 恢复 → 档案。全程 0 API Token。
2. 专家模式验证 pending 不测、接受后恢复继续、拒绝方向不占设备；专家接受不替代指标裁决。
3. 有效负结果也入档；只有达冻结收益或非劣效界的候选进入待确认集。
4. 预注册独立轮数、顺序和收益/非劣效界，冻结至少三轮原始基线准入记录；采集每臂唯一 trace/ENV 与 cold/root/控制/恢复/退出 receipts，再交给单轮观测重建及独立确认 judge。旧 run_once 目录缺上下文时不能补填成有效观测。
5. 再执行有显式预算的模型探索/训练路线；记录 provider、次数、成本、回退与候选来源。

## 4. 最小证据包

| 项目 | 必须有的实际证据 |
|---|---|
| 被测对象 | 设备/系统/游戏版本/设置、地点/时间/视角与负载 hash |
| 候选 | 方向、来源、工件 hash、专家或自主决定、接口/格式适用性 |
| 部署 | 目标挂点、有效命中、非目标 pass/UI 控制、起止配置 |
| 时序 | 来源明确；KGSL 代理与 display/present 分开，未知不补零 |
| 功耗 | 轨道、停充回读、采样窗口放电状态；不可比原因 |
| 画质 | 对齐/时间/来源 manifest、逐帧指标、异常帧、检测范围 |
| 裁决 | 原始 A/B、规则/目标/阈值 hash、CI/检验、缺失指标资格 |
| 恢复 | 写前/写后快照、每个改动恢复回读、失败/残留状态 |
| 确认 | 与探索分开的输入/轮次、复测结果和结论边界 |

验收从只读阶段开始；root、编译成功或历史案例都不足以越过部署与实测门槛。

## 2026-10-12 早期 E06 会话历史终态

现有`confirmation-driver prepare`确实执行到写前资格：7点/31.71s/最大跨度2°C/HAL0暖机通过，随后完整工具安装和冻结快照建立；首次`thermal_snapshot`退出82。原始失败值缺失，原因UNKNOWN，候选档5与三基线均NOT_RUN。不能把此前绝对cap失败或旧boot跨度失败移作该次原因。

独立`reconcile`实际完成，冻结配置快照与initial逐字节相同、归属组消失、session永久撤销；后续普通租约exit0。该恢复不重新验证旧样本，不使基线0/3变为通过。下一次必须使用新会话。

安装阶段的43次调用/94.49s与写前温度留证缺口分别由[组织#22](https://github.com/HGamey/.github/issues/22)、[#21](https://github.com/HGamey/.github/issues/21)跟踪。新源码需完整回归与签名，保持原温度参考和全部资格门禁；再执行同一现有prepare、真实基线与冻结单参数对照。宿主加载或发布速度的改善不能直接称为游戏性能改善。

当时发布的[公开面板](https://phonefarm.osaka.ananthe.party)展示真实终态与限制；390px检查来自本机Chrome模拟移动视口。报告与面板均不授予root控制、提交代理或显示/功耗优化资格。


## 2026-10-12 入场限制记录：518 / f889

上述 E06 恢复仅属于原会话，不能外推到新会话。整合 source `518f082` 已完成 722 项本地 gate；新 loaded/queued prepare 实际运行 121.491s，安装 5 件 helper 后 reader 路径 transport 超时，主工具包安装、候选控制、三基线与 A/B 均未到达。独立 reconcile 实际 15.328s / exit85，当前恢复状态仍 UNKNOWN，持久屏障保留。

[f889 只读诊断 draft #36](https://github.com/HGamey/phonefarm/pull/36) 已完成 730 项本地 gate，使用已签名且绑定来源的工具实际诊断一次：23.969s / host exit0，远端 20s deadline / exit124，raw 1892B。reader.stderr 为 `render_reader: missing terminal request`，reader exit81；两个登记 PID 的端点未找到，但完整进程组扫描和结束标记缺失。不能据此宣布远端空闲、恢复完成或准入新实验。具体上游 producer 失败条件尚未确定。

下一步先完善并验收针对原会话的拥有权恢复协议，核验原 helper 与全部相关进程组；只在持久屏障由真实恢复收据解除后创建新的普通实验租约。保留原阈值与时限，不盲重试 prepare、不手动删除 marker。当前三轮有效基线仍 0/3、优化收益未知。

原件：`oneplus_live_20261012/livephase/endfield-phonefarm-loaded-queued-prepare-518f082/review.json` 与 `livephase/endfield-helper-readonly-diagnostic-f889981/review.json`；对应 SHA-256 `01553f4daeab0dc93568ebdbdaaa74e2439097adf8ffdb170add1c07abd623d2` 和 `df686b22da91805f8bf92b345d525ec4f6e240178307681b60a1f223afc01fdb`。

### 最新检查点：3de 单次退役仍 UNKNOWN

`3de3159` 经 738 项 gate、default Clippy exit0、独立审查和新签名工具
`80657a…` 接受后，只在原会话执行了一次死进程限定退役。27.5469s 返回
UNKNOWN：receiver code0，但 stdout/stderr 均为0B，缺 native 终态及完整表。
不能由包装进程 exit0 推断关闭、删除或组消失。持久屏障保留，未重试，
新普通租约、商业终末地 API 检查和 GPU buffer 捕获均 NOT_RUN。

原件 `livephase/endfield-native-helper-retirement-3de3159/review.json`，SHA-256
`6b4a965f6d59d8f1aab4f275a75094bf54cf143c51cb36c001d113b0e4e7ef2f`。
网站 v6 将此负结果与原记录一同导入，56 个现场阶段并非56个合格实验。

媒体实验的入口是商业游戏实际渲染 API、layer 加载、现有资源读回与参考资格。
截图不能充当低清算法输入，原厂放大输出不能充当独立高清真值；自有游戏的
采集成功不授予终末地能力。质量采集与性能对照各自保存范围，先取得可用的
同帧资源后才开始模型迭代和 PSNR/运行成本验证。
