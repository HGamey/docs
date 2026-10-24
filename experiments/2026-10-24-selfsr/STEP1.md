# 自研超分+插帧 · 第 1 步：原神怎么出图（只读）

设备 RedMagic NX809J（Adreno 840，Android 16，RedMagicOS 11.0.23），`com.miHoYo.Yuanshen`，2026-10-24 01:54–01:58。
游戏设置未改（游戏内 60 帧），G-FRC+ 关（`game_gfrc_mode` 本游戏位 000），插着充电。
复现：`loop_v1/selfsr/probe_present.sh <serial> <out>`（游戏在前台时跑，只读）；基线用 `loop_v1/gfrc/gfrc_ab.py --on 000`（两臂都是原生）。
原始数据在 `step1/`。脚本在 phonefarm `loop_v1/selfsr/probe_present.sh`。

## 出图链路

```
Unity(IL2CPP) 主线程 UnityMain
  └─ Vulkan (vulkan.adreno.so)
       场景 pass 2140×968 ……… 后处理 …… 最后一个 pass 2141×969：全屏放大 draw + 约 50 笔 UI draw（同一个 pass）
       └─ vkQueuePresentKHR → swapchain 2141×969 R8G8B8A8_UNORM
  └─ SurfaceView(BLAST) 缓冲 2141×969
       └─ HWC DEVICE 合成：ROT_90，放大到屏幕 2688×1216（×1.256），面板 60 Hz
```

| 项 | 值 | 来源 |
|---|---|---|
| 图形接口 | Vulkan（进程映射 `vulkan.adreno.so`；GLES 库也在，但实际渲染走 Vulkan，层能拦到全部 render pass） | `gfx_libs.txt`；`knobs/evidence/02,06,07` |
| 送显分辨率 | 2141×969（swapchain = SurfaceView 缓冲） | `sf_hwc_layers.txt` Source Crop |
| 屏幕原生 | 2688×1216，支持 60/90/120/144 Hz，原神前台时 60 Hz | `display.txt` |
| 屏幕前最后一次放大 | HWC 硬件做，×1.256，双线性级 | `sf_hwc_layers.txt` Comp Type=DEVICE |
| 游戏内部放大 | 最后一个 pass 的第一笔 draw 采样场景结果放大到 swapchain，UI 在同 pass 后画 | `knobs/evidence/07_sr_feasibility` |
| Android Game Mode | standard/custom 可用，当前无任何干预 | `game_modes.txt` |
| 调试层开关 | 全为空，`ro.debuggable=0` | `debug_layer_settings.txt` |

## 原生基线（6 臂，同局镜头不动，每臂 10 s）

| 指标 | 均值 | 最小 | 最大 |
|---|---|---|---|
| 上屏帧率（SF timestats） | 59.86 fps | 59.79 | 59.89 |
| 游戏渲染帧率（GPU 提交） | 60.00 fps | 59.97 | 60.08 |
| 游戏渲染 p95 帧时 | 18.29 ms | 17.17 | 18.98 |
| 锐度（拉普拉斯方差，整屏） | 88.5 | 79.1 | 103.9 |
| SF 丢帧 / 10 s | 0.17 | 0 | 1 |

空对照（原生对原生）3 对全部不显著（p ≥ 0.5），说明测量链在两臂相同时不会假报差异；
同时给出噪声量级：p95 帧时 ±0.8 ms、锐度 ±15%。后面各步的效应要明显大于这个才算数。
截图：`step1/native_off1.jpg`。

## 对后面几步的含义

1. **插得进去**（第 2 步）：Vulkan 路线成立，`VK_LAYER_refknobs` 以前已在原神大世界里只读挂载、反作弊未拦（`knobs/evidence/06`）。
   最新 main 的正式入口是 `phonefarm gray-layer session`；旧的 bind-mount 脚本已退役。
2. **超分有两个落点**（第 3 步）：
   - 不改游戏设置降渲染分辨率：Android Game Mode 降分辨率已在真机生效过（缓冲宽 2141→1344，`_tmp/npu/N7`）。
   - 输出端：HWC 现在用简单放大把 2141×969 拉到 2688×1216；我们的超分直接输出到 2688×1216 就能比原生更清楚。
3. **插帧要先过刷新率这关**（第 4 步）：原神前台面板压在 60 Hz，60→120 必须让面板跑 120 Hz（G-FRC+ 开插帧时就切到了 120 Hz）。
4. 层里不能开线程（原神会杀层线程），全局 `debug.vulkan.layers` 会打崩 `cn.nubia.gameassist`，挂上后要立刻收回。
