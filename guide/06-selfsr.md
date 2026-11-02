# M6 自研超分与插帧

## 作用

在不修改游戏本身的前提下，在 root 手机上将一个 Vulkan 层（灰档层）注入游戏进程，实现以下三项功能：

- **超分**：游戏以低分辨率渲染（如 0.56 倍），由该层将画面放大至屏幕分辨率。放大采用 Catmull-Rom 插值，再经 RCAS 锐化。
- **插帧**：在两帧真实帧之间合成中间帧，将上屏帧率从约 60 提升至约 117。
- **A/B 测量**：在同一台手机上交替进行加载层与不加载层的冷启动，比较帧率、画质、温度和功耗，开、关两组之间严格配对。

已有结论见 HGamey/docs 的 [experiments/2026-10-24-selfsr](../experiments/2026-10-24-selfsr/)，测量方法见同目录下的 [HARNESS_GUIDE.md](../experiments/2026-10-24-selfsr/HARNESS_GUIDE.md)。

## 依赖

- M1 已安装，`DEVLOCK_DIR` 已设置。
- **一台已 root 的 Android 手机**，已安装并登录过游戏，启动后可进入大世界。
- Android NDK（r28c 已验证），用于编译层，通过 `ANDROID_NDK_HOME` 指定。
- 实验脚本仓库 [HGamey/experiments](https://github.com/HGamey/experiments)。

## 安装与运行

**方式一：本机直接运行（仅用 M1）**

```sh
git clone https://github.com/HGamey/experiments.git && cd experiments
export PF_ROOT=<hgamey 克隆目录>/phonefarm
export ANDROID_NDK_HOME=<NDK 目录>
sh selfsr/repro.sh <串号> 6 SETS=7 WINDOW=60      # 第 7 组：超分加插帧 vs 原生，6 对，每臂 60 秒
```

- 层由脚本现场编译。
- 每组结束时（包括按 Ctrl-C 中断），脚本均会恢复系统设置，并回读核对。
- 产物位于输出目录，包括 `report.md`、对比图和录屏。

**方式二：经平台运行（M1 + M2 + M4）**

```sh
pf-node --experiment main --research <研究ID> --device-access write --detach \
  experiment selfsr/repro.sh <串号> 6 SETS=7 WINDOW=60
```

- 结果显示在研究页，也可对产物发起盲评。
- 任务结束后处于 blocked 状态。核查产物中的还原记录后，执行 `pf-node reset <任务ID> --note "<核查结果>"` 释放手机。

`repro.sh` 文件头列出了全部分组（1 至 12）及可调参数（`SR_SCALE`、`IP_FPS`、`WINDOW`、`POWER` 等）。参数统一以 `KEY=VALUE` 形式附于命令之后。

## 验收

- 第 7 组运行完成后，`report.md` 中超分加插帧一臂的上屏帧率应明显高于原生，约为 117 对 60。
- 运行结束后手机恢复原状：游戏可正常启动，开发者选项中无残留的调试图层设置。

## 常见问题

| 现象 | 处理 |
|---|---|
| 编译层失败 | 检查 `ANDROID_NDK_HOME` |
| 无法进入游戏世界 | 先在手机上手动启动游戏并登录一次 |
| 上屏帧率停留在约 90 而非约 117 | 已知问题：由冷启动时随机出现的系统合成节律导致，详见 STEP6 附录。每臂冷启动时会重新判定 |
| 脚本中途异常退出，手机状态异常 | 手动还原：移除层、清除 `debug.knobs.*` 属性、恢复充电、将刷新率恢复为原值、关闭机身风扇；然后用 `./tools/devlock status` 确认无残留的锁。收尾步骤见 HARNESS_GUIDE 第 5 节 |
