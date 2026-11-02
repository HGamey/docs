# Demo 环境搭建（两台 PC 加一台 root 手机）

本手册说明如何在内网中使用两台 PC 和一台 root 手机搭建完整的演示环境。演示内容为：人员在网页上发起超分加插帧实验，查看帧率和画质结果，并完成盲评。

## 硬件与分工

| 设备 | 角色 | 安装的模块 |
|---|---|---|
| **PC-A** | 节点：连接手机、执行任务、提供网页 | M1、M2、M6（编译层需 NDK） |
| **PC-B** | 操作与 agent：通过浏览器查看网页，通过命令行下发任务，可选运行 AI agent | M4 |
| **root 手机** | 被测设备，通过 USB 连接 PC-A | — |

- **PC-A**：macOS 或 Linux。内存不低于 16 GB，空闲磁盘不低于 50 GB，USB 口供电稳定。
- **PC-B**：系统不限，具备浏览器和 Python 3 即可，须与 PC-A 位于同一内网。
- **手机**：可解锁并 root 的 Android 手机。已验证的为骁龙平台、120 Hz 屏幕。须预先安装目标游戏并登录一次。

后续增加连接手机的电脑时，按 M3 加入即可，PC-A 无需改动。

## 搭建前准备（内网无法访问外网时）

在可访问外网的机器上准备以下内容，再拷入内网：

1. **代码**：`HGamey/hgamey` 和 `HGamey/experiments` 两个仓库，使用 `git clone --mirror` 打包；或推送至内网 Git 服务器。
2. **Rust 依赖**：在 `hgamey/phonefarm/src` 和 `hgamey/game_opt_loop` 下分别执行一次 `cargo vendor`，并按提示配置 `.cargo/config.toml`。
3. **工具**：Rust 工具链安装包、Python 3.12、Android platform-tools（adb）、Android NDK r28c。
4. **实验代码地址**：若实验仓库存放于内网 Git，M2 的启动配置中须添加 `--experiments-url <内网地址>`，节点将按提交号从该地址获取代码。

## 步骤

**第 1 步：手机准备**
- 对手机进行 root，开启开发者选项和 USB 调试。
- 通过 USB 连接至 PC-A，并在手机上允许调试。
- 启动游戏并登录，确认可进入游戏世界，然后退出。

**第 2 步：PC-A 安装 M1**（见 [01-device-layer.md](01-device-layer.md)）
- 通过 M1 的验收：`adb devices` 可显示手机，`./phonefarm devices` 可列出该设备。

**第 3 步：PC-A 先单机验证 M6**（见 [06-selfsr.md](06-selfsr.md)）
- 按"方式一"运行一次 `SETS=7`，可先设置 `WINDOW=30` 以缩短时长。
- 确认 `report.md` 中超分加插帧一臂的上屏帧率约为 117，原生约为 60。偶有一臂停留在约 90，属已知的冷启动现象，不视为失败。
- 此步通过，即表明手机、root、层和游戏均无问题；后续网页端出现问题时，可排除上述因素。

**第 4 步：PC-A 安装 M2**（见 [02-node-service.md](02-node-service.md)）
- 启动配置中的 `--host` 填写 PC-A 的内网地址，以便 PC-B 访问。
- 服务命令带上 `--no-tunnel`，内网环境无需 M5。
- 服务环境中设置 `DEVLOCK_DIR` 和 `ANDROID_NDK_HOME`。
- 通过 M2 的验收。

**第 5 步：PC-B 接入**（见 [04-agent.md](04-agent.md)）
- 在 PC-B 的浏览器中打开 `http://<PC-A地址>:8765/dash/`，确认可看到手机。
- 在网页上点击"接入新 agent"生成邀请码，在 PC-B 上通过 `pf-node join` 登记。
- 执行 `./bin/pf-node whoami`，通过验收。
- AI agent 需调用大模型接口。内网无可用接口时，可跳过 agent，由人员在 PC-B 上使用 `pf-node` 手工下发任务，演示效果相同。

**第 6 步：演示流程**
1. 在网页上新建研究，例如"某游戏超分加插帧效果"。
2. 在 PC-B 上发起任务：

   ```sh
   ./bin/pf-node --experiment main --research <研究ID> --device-access write --detach \
     experiment selfsr/repro.sh <串号> 6 SETS=7 WINDOW=60
   ```

3. 网页首页显示手机已被该任务占用，可进入任务页查看实时日志。
4. 任务结束后，可在研究页查看帧率对比和录屏。
5. 对录屏发起盲评（`pf-node research compare ...`），然后在网页上进行排序评判。
6. 核查还原记录后，执行 `pf-node reset <任务ID> --note "<核查结果>"` 释放手机。

## 验收清单

- [ ] PC-A：`adb devices` 可显示手机
- [ ] PC-A：单机运行 `SETS=7`，超分加插帧一臂的上屏帧率约为 117（偶尔约为 90）
- [ ] PC-A：`deploy.py <节点名> --status` 显示的提交与服务报告的一致
- [ ] PC-B：浏览器打开看板，首页可看到手机
- [ ] PC-B：`pf-node whoami` 正常
- [ ] PC-B 发起的实验运行完成，研究页有结果，盲评可提交
- [ ] 实验结束后手机恢复原状，游戏运行正常

## 交付注意事项

- token、网页口令、隧道凭据仅存放于各自机器，不纳入仓库或文档。
- 搭建完成后记录 PC-A 部署的提交号（`deploy.py --status`），后续升级或回滚均以此为准。
