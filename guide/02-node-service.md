# M2 节点服务与网页

## 作用

在接手机的电脑上运行一个 HTTP 服务，提供：

- **任务队列**：按设备排队执行实验，每台手机同一时刻只有一个任务在写。
- **研究库**：研究、方向、留言、盲评、交付（SQLite）。
- **网页看板** `/dash/`：人查看进度、给出指导、做盲评。
- **HTTP API**：给命令行 `pf-node`（M4）和其他系统调用。

## 依赖

- M1 已安装，并通过验收。M1 克隆的仓库下面也用来部署。
- Python 3.12 及以上。macOS 自带的 `/usr/bin/python3` 版本太低，用 `brew install python@3.12` 装一个；下面的 `python3` 都指它。
- 一个空闲端口，默认 8765。
- 部署时能访问 GitHub。访问不了时见第 6 步的 `--no-fetch`。

## 安装

以下用 `~/node` 作安装根目录。换成别的目录时，下面所有路径一起改，包括启动配置里的 `--phonefarm`、`--game-opt-loop` 和服务命令里的 `--workdir`。这三项不写时默认指向 `~/node`。

**1. 管理 token。** 生成一个至少 32 个字符的随机串，只给本人读：

```sh
mkdir -p ~/node/.state && python3 -c 'import secrets; print(secrets.token_hex(32))' > ~/node/.state/node.token
chmod 600 ~/node/.state/node.token
```

**2. 网页口令。** `~/.config/hgamey/dash.pass` 是查看口令，`steer.pass` 是写操作口令，各写一行，每个至少 12 个字符；短于 12 个字符时网页当作没有设置。

```sh
mkdir -p ~/.config/hgamey && chmod 700 ~/.config/hgamey
for f in dash steer; do python3 -c 'import secrets; print(secrets.token_urlsafe(18))' > ~/.config/hgamey/$f.pass; done
chmod 600 ~/.config/hgamey/*.pass
```

口令目录可以用环境变量 `HG_CONFIG_DIR` 换到别处，例如同一台电脑上已有节点、不想共用口令时。换了目录，第 5 步的服务环境里也要设同样的 `HG_CONFIG_DIR`。

**3. 启动配置** `~/node/node-launch.json`：

```json
{"schema": "hgamey/node-launch@1",
 "node_token_file": "/Users/<用户>/node/.state/node.token",
 "service_args": ["--port", "8765", "--host", "127.0.0.1", "--data-dir", "/Users/<用户>/node/data",
                  "--phonefarm", "/Users/<用户>/node/phonefarm", "--game-opt-loop", "/Users/<用户>/node/game_opt_loop"]}
```

- 路径都要写绝对路径。
- 要让局域网里的其他电脑访问，把 `--host` 改成本机内网地址或 `0.0.0.0`。
- 其他参数见 `python3 harness-ops/node/node_service.py --help`。

**4. 部署配置。** 写一个 `hgamey/deploy-node@1` 文件，只写路径和服务命令，不写凭据。字段说明见 hgamey 仓库 README 的"部署"一节。放在哪里有两种做法：

- 长期节点：在仓库里新增 `deploy/<节点名>.json` 并提交，可以照 `deploy/` 下现有的 macOS（launchd）或 Linux（systemd）配置改。部署用 `python3 deploy.py <节点名>`。
- 只在本机用、不进仓库：放在安装根，例如 `~/node/deploy-node.json`。部署用 `python3 deploy.py --config ~/node/deploy-node.json`，可以加 `"name"` 字段作为部署回执里的节点名。

macOS 用 LaunchAgent（见第 5 步）时的例子，`<uid>` 用 `id -u` 查：

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

- `jobs_roots` 是启动配置里 `--data-dir` 下的 `jobs`。
- `path_prepend` 写 `cargo` 所在的目录：Homebrew 装的是 `/opt/homebrew/bin`，rustup 装的是 `~/.cargo/bin`。
- 要构建灰档层时，`gray_layer` 改成 `true`，并加 `"build_env": {"ANDROID_NDK_HOME": "<NDK 目录>"}`。

**5. 注册系统服务。** 服务要执行的命令是：

```sh
~/node/harness-ops/node/run_node.sh --config ~/node/node-launch.json --workdir ~/node --no-tunnel
```

- 要公网访问时，去掉 `--no-tunnel`，再按 M5 配置隧道。
- `--workdir` 是日志目录和工作目录，不写时是 `~/node`。安装根不是 `~/node` 时一定要写，否则日志会写进 `~/node`。
- 服务环境里要设好 M1 的 `DEVLOCK_DIR`；改过口令目录时设 `HG_CONFIG_DIR`；需要编译灰档层时，还要设 `ANDROID_NDK_HOME`。
- `run_node.sh` 会守护节点服务，服务退出后自动拉起。它会把 `/opt/homebrew/bin` 加到 PATH 前面。
- `~/node/harness-ops` 要到第 6 步第一次部署后才有。服务仍要先注册：`deploy.py` 发布文件前会执行部署配置里的 `stop`，服务没注册时这一步报错，部署中止。注册后到首次部署前，服务会反复启动失败，日志里是找不到 `run_node.sh`，属正常。

Linux 用 systemd 用户服务；要开机即起、不必登录，再执行一次 `sudo loginctl enable-linger <用户>`。

macOS 有两种做法：

| 做法 | 要不要 sudo | 什么时候启动 |
|---|---|---|
| LaunchAgent，放在 `~/Library/LaunchAgents` | 不要 | 注册后立即启动。电脑重启后不会自动起来，要再执行一次 `launchctl bootstrap` |
| LaunchDaemon，放在 `/Library/LaunchDaemons`，写 `UserName` 为运行用户 | 要 | 开机即起，不用登录。部署配置里的 `stop`、`start` 要写成 `sudo -n /bin/launchctl ...`，并给这两条命令配免密 sudo |

LaunchAgent 的例子 `~/Library/LaunchAgents/com.hgamey.node.plist`：

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

用 `user/<uid>`，不用 `gui/<uid>`：只用 ssh 管理、没人登录桌面的电脑上，`gui/<uid>` 会报 `125: Domain does not support specified action`。`LimitLoadToSessionType` 设为 `Background`，是为了有人登录桌面时不再多起一份。

**6. 构建并发布。** 在 M1 克隆的仓库根目录执行（用部署配置文件时，把 `<节点名>` 换成 `--config <文件>`）：

```sh
python3 deploy.py <节点名>                # 构建 origin/main 并发布、重启、核对版本
python3 deploy.py <节点名> --status       # 查看已安装的提交和服务实际报告的版本
```

- 首次部署要编译 phonefarm 和 game_opt_loop，在 Apple Silicon 的 Mac 上约 6 分钟。
- `deploy.py` 先执行 `git fetch`。节点访问不了 GitHub 时，在能访问时先 `git fetch`，部署时加 `--no-fetch`。
- 回执里 `"status": "succeeded"`，且 `after` 里 service、phonefarm、game_opt_loop 都报告同一个提交，才算部署成功。

回滚就是部署上一个提交：`python3 deploy.py <节点名> --ref <提交>`。

## 验收

```sh
curl -s http://127.0.0.1:8765/node -H "Authorization: Bearer $(cat ~/node/.state/node.token)"
                                          # 返回 node_id 和 service.revision（即部署的提交）
python3 deploy.py <节点名> --status        # 已安装的提交与服务报告的一致
curl -s -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/dash/
                                          # 303：转到登录页 /dash/login
```

浏览器打开 `http://<本机地址>:8765/dash/`，输入查看口令后：

- 首页能看到已连接的手机。没接手机时为空。
- 新建一个研究任务并保存，刷新后仍在。写操作要先输入写操作口令。

## 停止与卸载

```sh
launchctl bootout user/$(id -u)/com.hgamey.node      # macOS LaunchAgent：停止，重启电脑前不会再起
launchctl kickstart -k user/$(id -u)/com.hgamey.node  # 重启服务
```

卸载时先停止，再把 plist 从 `~/Library/LaunchAgents` 挪走。安装根里的数据目录和 token 按需备份后再处理。

## 常见问题

| 现象 | 处理 |
|---|---|
| 网页打不开 | 看服务是否在跑：`--workdir` 下的 `node_service.log` 和 `run_node.log`；`--host` 是否只监听了 127.0.0.1 |
| 登录页提示没有设置查看密码 | 口令短于 12 个字符，或服务环境里的 `HG_CONFIG_DIR` 和放口令的目录不一致 |
| 首页手机为空 | 先回到 M1 的验收；确认服务环境里的 `DEVLOCK_DIR` 与 M1 用的一致 |
| 部署停在 `git fetch` 并报连不上 GitHub | 网络恢复后先 `git fetch`，部署加 `--no-fetch` |
| 部署在 `launchctl bootout` 报 `No such process` | 服务还没注册，先做第 5 步 |
| 部署后出错 | 部署上一个提交 |
| 任务卡住 | `pf-node reset <任务ID> --note <说明>`（见 M4） |

不要手工覆盖安装目录，也不要删除 `releases/`、`deploy-state/`、数据目录或 token 文件。
