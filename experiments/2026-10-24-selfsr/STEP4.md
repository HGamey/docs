# 自研超分+插帧 · 第 4 步：插帧（进行中，被红魔的进程内限帧器挡住）

设备 RedMagic NX809J（RedMagicOS 11.0.23，Android 16），`com.miHoYo.Yuanshen` 7.1.0，2026-10-24。游戏内设置未改。
层：phonefarm `knobs/gray/layer/interp*.cpp`。每次游戏 `vkQueuePresentKHR`，层在真实帧之前多送 m 张合成帧
（`dup` 复制上一帧 / `bm` 块匹配运动补偿，`interp_mult` 张，t = k/(m+1)）。

## 已做成的

- 合成帧确实上屏：层用 `VK_GOOGLE_display_timing` 回读每张的实际上屏时刻，真实帧与合成帧交替出现。
- 块匹配插帧（I5 路线 B 的 4 个着色器）接进层并在真机上跑通；同一帧插多张时复用运动场。
- 屏幕高刷：红魔 `settings system refresh_rate_mode` 1/2/3/4 = 60/90/120/144 Hz，原神进世界**之后**设才生效
  （启动前设会被游戏助手压回 60 Hz；AOSP `min/peak_refresh_rate` 无效）。`panel_probe.txt`。
- 延迟口径：真实帧「游戏调 present → 实际上屏」，插帧臂与 `interp_mode=real` 对照臂之差（60 Hz、1 张插帧时 +10–19 ms，节奏未调好时测得）。

## 卡住的地方：每进程每秒约 30 次送显

层回读、测量窗口内（屏幕 144 Hz，周期 6.92 ms）：

| 臂 | 每秒上屏 | 真实帧 | 合成帧 | 上屏间隔 | 游戏渲染帧率 |
|---|---|---|---|---|---|
| 不插帧 `ip:real` | 29.9 | 29.9 | 0 | 34.6 ms（5 个周期） | 29.9 |
| 插 1 张 `ip:dup` | 29.9 | 15.0 | 14.9 | 34.6 ms | 14.9 |
| 0.45 倍超分、不插帧 | — | — | — | — | 29.8（GPU 有余量也不回 60） |
| 0.45 倍超分 + 插 1 张 | 30.0 | 15.0 | 15.0 | 34.6 ms | 14.7 |

总送显数被钉在约 30/s：插帧只把一半真实帧换成合成帧。与 GPU 余量无关（0.45 倍降分辨率后游戏仍 30 帧）。
第 1 步手机凉时原神 60 帧，过热后 30 帧——是外部按温度定的目标帧率。

逐个排除：
- SurfaceFlinger 没限：原神图层 `requestedFrameRate` = 144 Hz（层投的票生效），uid 帧率覆盖 = 144 Hz（`*_sf.txt`）。
- 高通 perf HAL 帧节奏 `vendor.perf.framepacing.enable=0`：无变化。
- 原神没传 `VkPresentTimesInfoGOOGLE`（不是 Swappy 的显示时刻反馈问题）。
- `libvknubia.so`（被 `libvulkan.so` 加载进游戏进程）：只统计帧率、经 socket 报「掉帧等级」给提频，不限帧。
- `libzteifasperfctl.so`：把送显事件经 `/proc/perfmgr/perf_ioctl` 报给内核提频，不限帧。

找到的限帧器：红魔改过的 **`libgui.so` 里的 `FpsControl` / IFAS**（`setLimitedFps to %d`、`Desired Fps`、`Ceiling`/`Floor`、
`IScreenRefreshRate::beginFixFps`）。它在应用进程内、缓冲队列这一级，参数由系统服务经 binder 下发；没找到可直接改的属性或文件。
本机 `log.tag=S`，logcat 可读 0 字节，看不到它的日志。
G-FRC+ 能到 117 帧，是厂商在合成侧插帧、并同时抬高这个上限。

## 下一步的路线（待定）

1. 绕开应用缓冲队列：层用 NDK `ASurfaceControl` 在游戏窗口上挂一个子图层，合成帧经 `ASurfaceTransaction` 直接交给 SurfaceFlinger，
   不走被限帧的 `queueBuffer`。工作量大，但不依赖厂商。
2. 逆向 `libgui.so` 的 `FpsControl` 下发路径，找系统服务里的开关（只对红魔有效）。
3. 在没有这套限帧器的 root 机上验证插帧本身（例如农场里能 root 的非红魔机）。
