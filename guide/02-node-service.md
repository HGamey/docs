# M2 节点服务与网页

## 作用

在连接手机的电脑上运行一个 HTTP 服务，提供以下功能：

- **任务队列**：按设备排队执行实验，每台手机同一时刻仅有一个任务执行写操作。
- **研究库**：研究、方向、留言、盲评、交付（SQLite）。
- **网页看板** `/dash/`：供人员查看进度、给出指导和进行盲评。
- **HTTP API**：供命令行 `pf-node`（M4）及其他系统调用。

## 依赖

- M1 已安装并通过验收。M1 克隆的仓库同时用于部署。
- Python 3.12 及以上。macOS 自带的 `/usr/bin/python3` 版本过低，需通过 `brew install python@3.12` 另行安装；下文的 `python3` 均指该版本。
- 一个空闲端口，默认 8765。
- 部署时需可访问 GitHub。无法访问时，参见第 6 步的 `--no-fetch`。

## 安装

下文以 `~/node` 作为安装根目录。如改用其他目录，下文所有路径须一并修改，包括启动配置中的 `--phonefarm`、`--game-opt-loop` 及服务命令中的 `--workdir`。这三项未指定时默认指向 `~/node`。

**1. 管理 token。** 生成一个不少于 32 个字符的随机串，权限设为仅本人可读：

```sh
mkdir -p ~/node/.state && python3 -c 'import secrets; print(secrets.token_hex(32))' > ~/node/.state/node.token
chmod 600 ~/node/.state/node.token
```

**2. 网页口令。** `~/.config/hgamey/dash.pass` 为查看口令，`steer.pass` 为写操作口令，各占一行，每个不少于 12 个字符；少于 12 个字符时，网页视为未设置。

```sh
mkdir -p ~/.config/hgamey && chmod 700 ~/.config/hgamey
for f in dash steer; do python3 -c 'import secrets; print(secrets.token_urlsafe(18))' > ~/.config/hgamey/$f.pass; done
chmod 600 ~/.config/hgamey/*.pass
```

口令目录可通过环境变量 `HG_CONFIG_DIR` 指定到其他位置，例如同一台电脑上已有节点且不宜共用口令时。更换目录后，第 5 步的服务环境中也须设置相同的 `HG_CONFIG_DIR`。

**3. 启动配置** `~/node/node-launch.json`：

```json
{"schema": "hgamey/node-launch@1",
 "node_token_file": "/Users/<用户>/node/.state/node.token",
 "service_args": ["--port", "8765", "--host", "127.0.0.1", "--data-dir", "/Users/<用户>/node/data",
                  "--phonefarm", "/Users/<用户>/node/phonefarm", "--game-opt-loop", "/Users/<用户>/node/game_opt_loop"]}
```

- 路径均须使用绝对路径。
- 如需局域网内其他电脑访问，将 `--host` 改为本机内网地址或 `0.0.0.0`。
- 其他参数见 `python3 harness-ops/node/node_service.py --help`。

**4. 部署配置。** 编写一个 `hgamey/deploy-node@1` 文件，仅包含路径和服务命令，不包含凭据。字段说明见 hgamey 仓库 README 的"部署"一节。文件存放位置有两种方式：

- 长期节点：在仓库中新增 `deploy/<节点名>.json` 并提交，可参照 `deploy/` 下现有的 macOS（launchd）或 Linux（systemd）配置修改。部署命令为 `python3 deploy.py <节点名>`。
- 仅本机使用、不纳入仓库：存放于安装根目录，例如 `~/node/deploy-node.json`。部署命令为 `python3 deploy.py --config ~/node/deploy-node.json`，可添加 `"name"` 字段作为部署回执中的节点名。

macOS 使用 LaunchAgent（见第 5 步）时的示例如下，`<uid>` 可通过 `id -u` 查询：

```json
{"schema": "hgamey/deploy-node@1",
 "name": "<节点名>",
 "root": "/Users/<用户>/node",
 "work": "/Users/<用户>/node/.deploy",
 "jobs_roots": ["/Users/<用户>/node/data/jobs"],
 "stop":  [["/bin/launchctl", "bootout", "user/<uid>/com.hgamey.node"]],
 "start": [["/bin/launchctl", "bootstrap", "user/<uid>", "/Users/<用户>/Library/LaunchAgents/com.hgamey.node.plist"]],
 "nodes": [{"url": "http://127.0.0.1:8765", "token_file": "/Users/<用户>/node/.state/node.token"}],
 "gray_layer": false,
 "path_prepend": ["/opt/homebrew/bin"]}
```

- `jobs_roots` 为启动配置中 `--data-dir` 下的 `jobs` 目录。
- `path_prepend` 填写 `cargo` 所在目录：通过 Homebrew 安装的为 `/opt/homebrew/bin`，通过 rustup 安装的为 `~/.cargo/bin`。
- 如需构建灰档层，将 `gray_layer` 改为 `true`，并添加 `"build_env": {"ANDROID_NDK_HOME": "<NDK 目录>"}`。

**5. 注册系统服务。** 服务执行的命令为：

```sh
~/node/harness-ops/node/run_node.sh --config ~/node/node-launch.json --workdir ~/node --no-tunnel
```

- 如需公网访问，去掉 `--no-tunnel`，并按 M5 配置隧道。
- `--workdir` 为日志目录和工作目录，未指定时为 `~/node`。安装根目录不是 `~/node` 时必须指定，否则日志将写入 `~/node`。
- 服务环境中须设置 M1 的 `DEVLOCK_DIR`；修改过口令目录时须设置 `HG_CONFIG_DIR`；需编译灰档层时，还须设置 `ANDROID_NDK_HOME`。
- `run_node.sh` 负责守护节点服务，服务退出后自动重新拉起，并将 `/opt/homebrew/bin` 添加至 PATH 前部。
- `~/node/harness-ops` 在第 6 步首次部署后才会生成，但服务仍须先行注册：`deploy.py` 发布文件前会执行部署配置中的 `stop`，若服务未注册，该步骤报错，部署中止。注册后至首次部署前，服务会反复启动失败，日志显示找不到 `run_node.sh`，属正常现象。

Linux 使用 systemd 用户服务；如需开机自启且无需登录，另执行一次 `sudo loginctl enable-linger <用户>`。

macOS 有两种方式：

| 方式 | 是否需要 sudo | 启动时机 |
|---|---|---|
| LaunchAgent，存放于 `~/Library/LaunchAgents` | 否 | 注册后立即启动。电脑重启后不会自动启动，需再次执行 `launchctl bootstrap` |
| LaunchDaemon，存放于 `/Library/LaunchDaemons`，`UserName` 设为运行用户 | 是 | 开机自启，无需登录。部署配置中的 `stop`、`start` 须写为 `sudo -n /bin/launchctl ...`，并为这两条命令配置免密 sudo |

LaunchAgent 示例 `~/Library/LaunchAgents/com.hgamey.node.plist`：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
 <key>Label</key><string>com.hgamey.node</string>
 <key>ProgramArguments</key><array>
  <string>/bin/sh</string><string>/Users/<用户>/node/harness-ops/node/run_node.sh</string>
  <string>--config</string><string>/Users/<用户>/node/node-launch.json</string>
  <string>--workdir</string><string>/Users/<用户>/node</string><string>--no-tunnel</string>
 </array>
 <key>WorkingDirectory</key><string>/Users/<用户>/node</string>
 <key>EnvironmentVariables</key><dict>
  <key>HOME</key><string>/Users/<用户></string>
  <key>PATH</key><string>/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
  <key>DEVLOCK_DIR</key><string>/Users/<用户>/.local/state/phonefarm/locks</string>
 </dict>
 <key>RunAtLoad</key><true/>
 <key>KeepAlive</key><true/>
 <key>LimitLoadToSessionType</key><string>Background</string>
 <key>StandardOutPath</key><string>/Users/<用户>/node/run_node.log</string>
 <key>StandardErrorPath</key><string>/Users/<用户>/node/run_node.log</string>
</dict></plist>
```

```sh
plutil -lint ~/Library/LaunchAgents/com.hgamey.node.plist
launchctl bootstrap user/$(id -u) ~/Library/LaunchAgents/com.hgamey.node.plist
```

应使用 `user/<uid>`，而非 `gui/<uid>`：在仅通过 ssh 管理、无人登录桌面的电脑上，`gui/<uid>` 会报错 `125: Domain does not support specified action`。将 `LimitLoadToSessionType` 设为 `Background`，是为了避免有人登录桌面时再启动一个实例。

**6. 构建并发布。** 在 M1 克隆的仓库根目录下执行（使用部署配置文件时，将 `<节点名>` 替换为 `--config <文件>`）：

```sh
python3 deploy.py <节点名>                # 构建 origin/main 并发布、重启、核对版本
python3 deploy.py <节点名> --status       # 查看已安装的提交和服务实际报告的版本
```

- 首次部署需编译 phonefarm 和 game_opt_loop，在 Apple Silicon 的 Mac 上约需 6 分钟。
- `deploy.py` 会先执行 `git fetch`。节点无法访问 GitHub 时，应在可访问时预先执行 `git fetch`，部署时添加 `--no-fetch`。
- 仅当回执中为 `"status": "succeeded"`，且 `after` 中 service、phonefarm、game_opt_loop 报告的提交一致时，方可视为部署成功。

回滚即部署上一个提交：`python3 deploy.py <节点名> --ref <提交>`。

## 验收

```sh
curl -s http://127.0.0.1:8765/node -H "Authorization: Bearer $(cat ~/node/.state/node.token)"
                                          # 返回 node_id 和 service.revision（即部署的提交）
python3 deploy.py <节点名> --status        # 已安装的提交与服务报告的一致
curl -s -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/dash/
                                          # 303：转到登录页 /dash/login
```

在浏览器中打开 `http://<本机地址>:8765/dash/`，输入查看口令后：

- 首页可显示已连接的手机；未连接手机时为空。
- 新建一个研究任务并保存，刷新后仍存在。执行写操作前须先输入写操作口令。

## 停止与卸载

```sh
launchctl bootout user/$(id -u)/com.hgamey.node      # macOS LaunchAgent：停止，重启电脑前不会再起
launchctl kickstart -k user/$(id -u)/com.hgamey.node  # 重启服务
```

卸载时先停止服务，再将 plist 从 `~/Library/LaunchAgents` 移出。安装根目录中的数据目录和 token 应按需备份后再行处理。

## 常见问题

| 现象 | 处理 |
|---|---|
| 网页无法打开 | 检查服务是否在运行：查看 `--workdir` 下的 `node_service.log` 和 `run_node.log`；检查 `--host` 是否仅监听 127.0.0.1 |
| 登录页提示未设置查看密码 | 口令少于 12 个字符，或服务环境中的 `HG_CONFIG_DIR` 与口令所在目录不一致 |
| 首页手机列表为空 | 先重新执行 M1 的验收；确认服务环境中的 `DEVLOCK_DIR` 与 M1 所用的一致 |
| 部署停在 `git fetch` 并报无法连接 GitHub | 网络恢复后先执行 `git fetch`，部署时添加 `--no-fetch` |
| 部署在 `launchctl bootout` 处报 `No such process` | 服务尚未注册，须先完成第 5 步 |
| 部署后出错 | 部署上一个提交 |
| 任务卡住 | `pf-node reset <任务ID> --note <说明>`（见 M4） |

不得手工覆盖安装目录，也不得删除 `releases/`、`deploy-state/`、数据目录或 token 文件。
