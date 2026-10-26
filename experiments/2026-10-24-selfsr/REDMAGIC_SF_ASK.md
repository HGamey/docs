# 给红魔系统组：游戏 uid 的子图层插帧被 SurfaceFlinger 按 60 Hz 放行

设备：RedMagic NX809J，RedMagicOS 11.0.23（Android 16），屏幕 120 Hz（`refresh_rate_mode=3`）。游戏：原神 7.1.0（uid 10518）。日期 2026-10-26。

## 现象

我们在原神进程里挂了一个 Vulkan 层。层给游戏窗口挂一个 `ASurfaceControl` 子图层 `selfsr_interp`，每个真实帧之后插一张中间帧。
所有缓冲都带 `desiredPresentTime`，子图层投 120 Hz。

- 期望上屏 120 帧，实际只有 **89 帧**。上屏间隔有 1/3 是 16.7 ms。
- 同一份层代码在荣耀 AAP-AN00（同 SoC）上能**满 120 帧**。refbench 钉 60 帧时是 119.9；Megacity 从 30 帧插到 60.1 帧。

## 已排除

| 怀疑 | 结论 |
|---|---|
| 层没插够帧 | 每个真实帧都插了一张（12599 / 12593） |
| GPU 来不及 | fence 中位数比期望时刻早 10 ms |
| 屏幕不在 120 Hz | 游戏内 frametimeline 64 个显示帧全是 8.3 ms |
| AOSP 帧率覆盖 | `FrameRateOverrides setFrameRate={10518, 120 Hz}`，GameFrameRateOverrides 为空 |
| libgui FpsControl 限帧 | 每帧清 `Limited Frame`（0x60），关总开关（0x20）：都还是 89 |
| 亮度回落 / 提前量 / 间隔 / 定时唤醒 SF | 都还是 88–90 |

## 根因证据

SF atrace（gfx，4 s，游戏内，`step4/redmagic_sf_ifas/atrace_excerpt.txt`）：

```
isVsyncValid uid: 10518 frameRate: 60.00 Hz      ← 1216 次，全部 60 Hz，没有一次 120
not current desiredPresentTime: … expectedPresentTime: …   ← 约每 16.6 ms 一次
[lmt]onIFASRecv 1                                ← 每个游戏帧一次（60/s）
[lmt]onLmtRecv 0 / -1, [lmt]notify:-1
```

frametimeline（`frametimeline_selfsr_summary.txt`）：子图层 85 帧里 22 帧 `Dropped`（26%），scheduled rendering rate 是 120。
120 × (1 − 26%) ≈ 89，和实测吻合。

也就是说，SF 判断原神 uid 的事务能不能在当前 vsync 处理时，用的是 60 Hz。这个 60 来自 ZTE 的 lmt / IFAS 逻辑，不是 AOSP 的帧率覆盖表。
所以同 uid 的子图层事务每隔一个 vsync 才放行一次，前一张被后一张顶掉。

## 请求

满足下面任意一条即可：

1. 一个 uid 下有图层**自己投了更高的帧率**（ExplicitDefault / FixedSource 120）时，`isVsyncValid` 按这个 uid 所有图层的最高投票放行，不只看游戏主图层的 IFAS 帧率。
2. 给第三方 / 游戏自带的插帧提供一个开关（系统属性或 `SurfaceComposerClient::fpsControl` 的参数），让指定 uid 不走 IFAS 的 vsync 限制。
3. 告诉我们 IFAS 的帧率从哪里来、怎么关。我们在游戏侧试过 `FpsControlConfig` 的 0x20 / 0x60，都无效；改 0x80（Ceiling）会让系统把屏幕切到 60 Hz。

放开后，我们会用同一套 A/B 脚本复测（`loop_v1/selfsr/selfsr_ab.py`，6 对，符号翻转检验）。

## 复现

```
python3 -I loop_v1/selfsr/selfsr_ab.py run --serial <红魔> --out <目录> \
  --a 'srip:1:2688x1216:1.0:bm:120:stamp=0,panel=120,backend=sc,lead_ns=25000000,unlimit=1' \
  --b 'srip:1:2688x1216:1.0:bm:120:stamp=0,panel=120,backend=sc,lead_ns=25000000,unlimit=1' \
  --rounds 1 --fan --cool-max 42 --window 60
# 窗口内另开: adb shell su -c 'atrace -b 32768 -t 4 gfx view sched' | grep isVsyncValid
```

phonefarm `feat/selfsr`（a67714a）。
