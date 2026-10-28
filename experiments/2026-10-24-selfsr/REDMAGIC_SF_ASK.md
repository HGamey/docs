# 给红魔系统组：SF 的 waitframe 让游戏内子图层插帧丢 1/4 的帧

设备：RedMagic NX809J，RedMagicOS 11.0.23（Android 16），屏幕 120 Hz（`refresh_rate_mode=3`）。游戏：原神 7.1.0（uid 10518）。日期 2026-10-26。

## 更新 (2026-10-28)：我们已在应用侧绕开，不需要改系统也能到 118 帧

不 root、不改任何系统设置（waitframe 保持出厂开启），只改我们自己的 Vulkan 层：

| 冷启动 A/B（原神大世界，120 Hz，每臂 20 s 窗口，有效 4 对） | 原方案 | 子图层起游戏图层的名字 |
|---|---|---|
| 实际看到的画面帧率 | 89.7 | **118.7** |
| SF 丢帧 / 20 s 窗口 | 1204 | **4.5** |
| 游戏渲染帧率 | 59.2 | 59.3 |

做法：子图层名从 `selfsr_interp` 改成 `SurfaceView[<包名>/selfsr_interp`。atrace 显示 waitframe 从此不再在 commit 里轮询游戏缓冲的 fence，整帧跳过从约 30 次/秒降到 0。

我们由此推断（请确认）：

1. waitframe 按图层名（`SurfaceView[<包名>/` 前缀）认「游戏图层」，每个 vsync 等它来一帧：缓冲 fence 没 signal 就每 1 ms 轮询，图层这一帧没来就等满 range factor（出厂 50% vsync，实测每帧固定空等 4.22 ms）。
2. 名字匹配到多个图层时，以我们的子图层为准（它每个 vsync 都来一张、交时已画完），所以不再等原神自己的图层。
3. 原神自己的图层不送新帧时（我们试过把它停掉），系统会判游戏掉帧，把峰值刷新率降到 60 Hz。所以原神的图层要照常送，只是被我们不透明的子图层盖住。

想请红魔确认 / 提供：

- 上面的推断对不对；这种做法会不会和游戏助手、G-FRC+、温控降帧冲突。
- 如果靠图层名认游戏图层，后续版本会不会改；有没有正式的方式（API、白名单、图层元数据）让第三方插帧的子图层被当成游戏的上屏图层。

原始记录与 atrace：`step4/waitframe_name/`，正式结果 `step4/name_auto_ab/`；实验记录见 `STEP4.md`「追加 (2026-10-28)」。

---

以下是 2026-10-26 的原始问题描述。

> 本文 2026-10-26 上午的版本把原因写成了 `isVsyncValid` 按 60 Hz 放行，那是错的（帧率覆盖改成 120 后照样 89 帧）。以下为更正后的版本。

## 现象

我们在原神进程里挂了一个 Vulkan 层。层给游戏窗口挂一个 `ASurfaceControl` 子图层 `selfsr_interp`，每个真实帧之后插一张中间帧。
所有缓冲都带 `desiredPresentTime`。

- 期望上屏 120 帧，实际只有 **89 帧**，上屏间隔有 1/3 是 16.7 ms。
- 关掉 SF 的 waitframe（`dumpsys SurfaceFlinger --fpsControl_waitframe_off`）后，上屏 **119.9 帧**，间隔 p95 8.3 ms。

## 原因

SF atrace（`step4/redmagic_waitframe/atrace_waitframe_excerpt.txt`）：

1. commit 里 `fence unsignaled, frameNum: N`（游戏图层的缓冲，GPU 还没画完），随后是 `wait` 每 1 ms 一次，持续约 7.5 ms。
2. 这一帧因此合成晚了。下一个 vsync 的 commit 里 `PrevFramePending 1`、`PrevFrameMissed 1`，结果是 `onCommitNotComposited`，整帧跳过。
3. 被跳过的 vsync 里排的子图层缓冲，下次和后一张一起就绪，前一张被 `releasePendingBuffer` 顶掉。3 秒内 103 次，约 34 次/秒，正好等于 120 − 89。

已排除的因素（都还是 89 帧）：

- 用后门 1039 把 uid 帧率覆盖设成 120，`isVsyncValid` 全部按 120 判定。
- 每 8.3 ms 给 SF 发一个空事务叫醒它。
- 把 desiredPresentTime 对齐 vsync 网格。

## 请求

满足下面任意一条即可：

1. waitframe 只针对「这一帧唯一要更新的是游戏图层」的情况。同 uid 还有其他已就绪的图层（比如子图层）时不等，或者等待不超过半个 vsync。
2. 给第三方或游戏自带的插帧提供一个按包名或 uid 关闭 waitframe 的接口，替代现在全局的 `dumpsys` 开关。
3. 确认 `--fpsControl_waitframe_off` 对系统其他功能（G-FRC+、游戏助手帧率档）没有副作用，我们就在测试和演示时用它。

## 复现

```
python3 -I loop_v1/selfsr/selfsr_ab.py run --serial <红魔> --out <目录> \
  --a 'srip:1:2688x1216:1.0:bm:120:stamp=0,panel=120,backend=sc,lead_ns=25000000,unlimit=1' \
  --b 'srip:1:2688x1216:1.0:bm:120:stamp=0,panel=120,backend=sc,lead_ns=25000000,unlimit=1,sfwait=off' \
  --rounds 1 --fan --cool-max 40 --window 60
# A 臂约 89 帧，B 臂约 120 帧。窗口内另开: adb shell su -c 'atrace -b 65536 -t 3 gfx view' | grep -E "fence unsignaled|onCommitNotComposited"
```

phonefarm `feat/selfsr`（f6d552e）。
