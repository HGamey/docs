# M6 自研超分与插帧

## 作用

不改游戏本身，在 root 手机上给游戏挂一个 Vulkan 层（灰档层），完成三件事：

- **超分**：游戏以低分辨率渲染（如 0.56 倍），层把画面放大到屏幕分辨率。放大用 Catmull-Rom 插值，再用 RCAS 锐化。
- **插帧**：在两帧真实帧之间合成中间帧，把上屏帧率从约 60 提到约 117。
- **A/B 测量**：同一台手机上，开层与不开层交替冷启动，比较帧率、画质、温度和功耗，开关组之间精确配对。

已有结论见 HGamey/docs 的 [experiments/2026-10-24-selfsr](../experiments/2026-10-24-selfsr/)，测量方法见同目录的 [HARNESS_GUIDE.md](../experiments/2026-10-24-selfsr/HARNESS_GUIDE.md)。

## 依赖

- M1 已安装，`DEVLOCK_DIR` 已设好。
- **一台已 root 的 Android 手机**，游戏已安装并登录过，打开后能进入大世界。
- Android NDK（r28c 已验证），用来编译层。用 `ANDROID_NDK_HOME` 指定。
- 实验脚本仓库 [HGamey/experiments](https://github.com/HGamey/experiments)。

## 安装与运行

**方式一：本机直接跑（只用 M1）**

```sh
git clone https://github.com/HGamey/experiments.git && cd experiments
export PF_ROOT=<hgamey 克隆目录>/phonefarm
export ANDROID_NDK_HOME=<NDK 目录>
sh selfsr/repro.sh <串号> 6 SETS=7 WINDOW=60      # 第 7 组：超分加插帧 vs 原生，6 对，每臂 60 秒
```

- 层由脚本现场编译。
- 每组结束（包括按 Ctrl-C 中断）时，脚本都会把系统设置写回，并回读核对。
- 产物在输出目录里，包括 `report.md`、对比图和录屏。

**方式二：经平台运行（M1 + M2 + M4）**

```sh
pf-node --experiment main --research <研究ID> --device-access write --detach \
  experiment selfsr/repro.sh <串号> 6 SETS=7 WINDOW=60
```

- 结果出现在研究页，也可以对产物发起盲评。
- 任务结束后会停在 blocked。核查产物里的还原记录后，执行 `pf-node reset <任务ID> --note "<核查结果>"` 释放手机。

`repro.sh` 文件头列出了全部分组（1 到 12）和可调参数（`SR_SCALE`、`IP_FPS`、`WINDOW`、`POWER` 等）。参数一律写成 `KEY=VALUE` 跟在命令后面。

## 验收

- 第 7 组跑完后，`report.md` 里超分加插帧那一臂的上屏帧率明显高于原生，约 117 对 60。
- 跑完后手机恢复原状：游戏正常打开，开发者选项里没有残留的调试图层设置。

## 常见问题

| 现象 | 处理 |
|---|---|
| 编译层失败 | 检查 `ANDROID_NDK_HOME` |
| 进不了游戏世界 | 先在手机上手动打开游戏并登录一次 |
| 上屏帧率卡在约 90 而不是约 117 | 已知问题：冷启动时随机出现的系统合成节律，详见 STEP6 附录。每臂冷启动时会重新判定 |
| 脚本中途异常退出，手机状态不对 | 手动还原：摘掉层、清掉 `debug.knobs.*` 属性、恢复充电、把刷新率设回原值、关掉机身风扇；然后用 `./tools/devlock status` 确认没有残留的锁。收尾步骤见 HARNESS_GUIDE 第 5 节 |
