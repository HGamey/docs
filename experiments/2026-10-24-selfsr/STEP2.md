# 自研超分+插帧 · 第 2 步：透传 Vulkan 层挂进原神

设备 RedMagic NX809J，`com.miHoYo.Yuanshen` 7.1.0，2026-10-24 02:41–03:01。游戏内设置未改。
层：phonefarm `knobs/gray` 的 `libVkLayer_refknobs.so`（commit cec2807，冻结副本），probe 模式（只计数、不改任何东西）。
挂/摘：`phonefarm gray-layer session --mode probe`（会话持设备租约、启动游戏、结束摘层）。
测量：与第 1 步同一套（SurfaceFlinger timestats + ftrace GPU 提交 + 截图）。
复现：`python3 -I loop_v1/selfsr/selfsr_ab.py run --serial S --out DIR --a native --b layer --rounds 6`。

## 结果：没有可测出的差别

每臂冷启动进大世界、静置 20 s 后采 10 s。6 对 ABBA，B4 那一臂 onboard 没进到大世界（测量前作废，层照常摘除），有效 5 对。

| 指标 | 不挂层 | 挂层 | 差 | 95% 区间 | p |
|---|---|---|---|---|---|
| 上屏帧率 | 29.70 | 29.68 | −0.07% | [−0.08, +0.04] fps | 0.75 |
| 游戏渲染帧率 | 29.59 | 29.47 | −0.41% | [−0.47, +0.32] fps | 0.63 |
| p95 帧时 | 36.3 ms | 37.0 ms | +1.7% | [−2.0, +2.7] ms | 0.75 |
| 锐度 | 93.8 | 86.5 | −7.7% | [−18, +3.6] | 0.31 |
| SF 丢帧 | 0 | 0 | 0 | | 1.0 |

- 层确实在工作：每个挂层臂的采样窗口内层自己数到 300–600 帧（marker `readonly_stats.frames`）。
- 每个挂层臂结束后核对：游戏 lib 目录里没有层 .so、`debug.vulkan.layers` 为空；收尾 Game Mode 仍为 standard。

## 口径与限制

- **5 对的最小可能 p 是 0.0625**，所以「不显著」本身说明不了多少；能说的是差值区间：上屏帧率在 ±0.1 fps 内，p95 帧时在 ±2.7 ms 内。
- 本批测量时手机过热（CPU0 ≈ 99.6 °C，充电中），原神被压在 **30 帧档**（第 1 步时为 60 帧档）。两臂交替，热的影响落在两边。
  每帧 GPU 忙约 29 ms / 33 ms 间隔，这个状态下原神是 GPU 受限的。
- 锐度与 SSIM 跨冷启动受云、时辰影响大（同为原生的两臂之间 SSIM 只有 0.65），只作参考。
- 会话自带的 `sample` 动作在本机 ftrace 子进程起不来（worker exit 76），会话的点击功能识别不出屏幕方向（`SurfaceOrientation` 读不到），
  所以测量与导航不用会话的；有层臂用 `onboard --no-cold --no-devlock` 在会话租约内导航。
