# Demo 环境搭建（两台 PC 加一台 root 手机）

本手册说明如何在内网里用两台 PC 和一台 root 手机搭起完整的演示环境。演示内容是：人在网页上发起超分加插帧实验，看到帧率和画质结果，并完成盲评。

## 硬件与分工

| 设备 | 角色 | 装哪些模块 |
|---|---|---|
| **PC-A** | 节点：接手机、跑任务、提供网页 | M1、M2、M6（编层要用 NDK） |
| **PC-B** | 操作与 agent：浏览器看网页，命令行发任务，可选运行 AI agent | M4 |
| **root 手机** | 被测设备，USB 接 PC-A | — |

- **PC-A**：macOS 或 Linux。至少 16 GB 内存、50 GB 空闲磁盘，USB 口供电稳定。
- **PC-B**：任意系统，有浏览器和 Python 3 即可。和 PC-A 在同一内网。
- **手机**：可以解锁并 root 的 Android 手机。已验证的是骁龙平台、120 Hz 屏。要装好目标游戏并登录一次。

以后接第二台手机电脑时，按 M3 加入即可，PC-A 不用改。

## 搭建前准备（内网访问不了外网时）

在能上外网的机器上准备好以下内容，再拷进内网：

1. **代码**：`HGamey/hgamey` 和 `HGamey/experiments` 两个仓库，用 `git clone --mirror` 打包；或者推到内网 Git 服务器。
2. **Rust 依赖**：在 `hgamey/phonefarm/src` 和 `hgamey/game_opt_loop` 下各执行一次 `cargo vendor`，并按提示写好 `.cargo/config.toml`。
3. **工具**：Rust 工具链安装包、Python 3.12、Android platform-tools（adb）、Android NDK r28c。
4. **实验代码地址**：如果实验仓库放在内网 Git，M2 的启动配置里要加 `--experiments-url <内网地址>`，节点按提交号从这里取代码。

## 步骤

**第 1 步：手机准备**
- root 手机，打开开发者选项和 USB 调试。
- 用 USB 接到 PC-A，在手机上允许调试。
- 打开游戏，登录，确认能进入游戏世界，然后退出。

**第 2 步：PC-A 安装 M1**（见 [01-device-layer.md](01-device-layer.md)）
- 通过 M1 的验收：`adb devices` 能看到手机，`./phonefarm devices` 能列出它。

**第 3 步：PC-A 先单机验证 M6**（见 [06-selfsr.md](06-selfsr.md)）
- 用"方式一"跑一次 `SETS=7`，可以先设 `WINDOW=30` 缩短时间。
- 确认 `report.md` 里超分加插帧那一臂的上屏帧率约为 117，原生约为 60。偶尔有一臂停在约 90，这是已知的冷启动现象，不算失败。
- 这一步通过，就说明手机、root、层和游戏都没问题。之后网页上出了问题，可以排除这几项。

**第 4 步：PC-A 安装 M2**（见 [02-node-service.md](02-node-service.md)）
- 启动配置里的 `--host` 填 PC-A 的内网地址，这样 PC-B 才能访问。
- 服务命令带上 `--no-tunnel`，内网不需要 M5。
- 服务环境里设好 `DEVLOCK_DIR` 和 `ANDROID_NDK_HOME`。
- 通过 M2 的验收。

**第 5 步：PC-B 接入**（见 [04-agent.md](04-agent.md)）
- PC-B 的浏览器打开 `http://<PC-A地址>:8765/dash/`，能看到手机。
- 在网页上点"接入新 agent"生成邀请码，在 PC-B 上用 `pf-node join` 登记。
- 执行 `./bin/pf-node whoami`，通过验收。
- AI agent 要用大模型接口。内网里没有可用的接口时，跳过 agent，由人在 PC-B 上用 `pf-node` 手工发任务，演示效果一样。

**第 6 步：演示流程**
1. 网页上新建研究，例如"某游戏超分加插帧效果"。
2. 在 PC-B 上发起任务：

   ```sh
   ./bin/pf-node --experiment main --research <研究ID> --device-access write --detach \
     experiment selfsr/repro.sh <串号> 6 SETS=7 WINDOW=60
   ```

3. 网页首页显示手机被这个任务占用。可以进任务页看实时日志。
4. 任务结束后，研究页能看到帧率对比和录屏。
5. 对录屏发起盲评（`pf-node research compare ...`），然后在网页上排序评判。
6. 核查还原记录后，执行 `pf-node reset <任务ID> --note "<核查结果>"` 释放手机。

## 验收清单

- [ ] PC-A：`adb devices` 能看到手机
- [ ] PC-A：单机跑 `SETS=7`，超分加插帧那一臂的上屏帧率约为 117（偶尔约为 90）
- [ ] PC-A：`deploy.py <节点名> --status` 显示的提交，和服务报告的一致
- [ ] PC-B：浏览器打开看板，首页能看到手机
- [ ] PC-B：`pf-node whoami` 正常
- [ ] PC-B 发起的实验跑完，研究页有结果，盲评能提交
- [ ] 实验结束后手机恢复原状，游戏正常运行

## 交付时注意

- token、网页口令、隧道凭据只放在各自机器上，不进仓库、不进文档。
- 搭好后记下 PC-A 部署的提交号（`deploy.py --status`）。以后升级或回滚都以它为准。
