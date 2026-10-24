# 第 0 步「能拿锁」、第 2 步「一键关掉并还原」的核对记录

2026-10-24 12:03–12:11，RedMagic NX809J（91253241019A），原始输出 `checks/step0_2.txt`。

## 第 0 步：测试工具能正常拿锁

`tools/devlock run check-lock@91253241019A -- …`：拿到锁，锁内命令执行，退出码 0。
锁内调用 `phonefarm fleet` 时，它正确认出设备被占用（`device domain occupied … devlock.d`），只返回 deferred，没有去碰设备。
在这之前，第 2–5 步的全部 A/B 实验（每臂一次 devlock 或 gray-layer 会话租约，共 100 多臂）都在锁下跑完，没有残留锁。
看板 `/dash/live` 在测量臂持锁期间显示「测量中」，停掉了视频编码器（实测，见 harness-ops `node/dash_live.py`）。

## 第 2 步：一键关掉并还原

| 时刻 | 状态 | 层 .so 在游戏 lib 目录 | `debug.vulkan.layers` |
|---|---|---|---|
| 12:09:55 | `gray-layer session` 就绪（层已挂） | 1 | `VK_LAYER_refknobs_readonly` |
| 12:11:23 | 会话结束（到时） | 0 | 空 |
| 12:11:25 | `phonefarm gray-layer off com.miHoYo.Yuanshen --serial <S>` | — | — |
| 12:11:29 | 关掉后 | 0 | 空；`gray-layer status`：未施加 |

- 一键关掉的命令是 **`phonefarm gray-layer off <包名> --serial <S>`**，返回 `status: verified`（回读核对过）。
- 会话还在跑时执行 `off`：它会等会话交还租约，然后摘层并核对（12:04 那次，`status: verified`）。
- 正常结束或被打断时，会话自己也会摘层（即使带了 `--keep-prop`）。
- 每次 A/B 实验收尾都会写 `restore.json` 回读核对：Game Mode、G-FRC+ 位、`debug.knobs.*`、刷新率、风扇、屏幕超时都恢复原值，层不在。
