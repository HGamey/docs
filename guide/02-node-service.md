# M2 节点服务与网页

## 作用

在接手机的电脑上运行一个 HTTP 服务，提供：

- **任务队列**：按设备排队执行实验，每台手机同一时刻只有一个任务在写。
- **研究库**：研究、方向、留言、盲评、交付（SQLite）。
- **网页看板** `/dash/`：人查看进度、给出指导、做盲评。
- **HTTP API**：给命令行 `pf-node`（M4）和其他系统调用。

## 依赖

- M1 已安装，并通过验收。
- Python 3.12 及以上。
- 一个空闲端口，默认 8765。

## 安装

以下用 `~/node` 作安装根目录。

**1. 管理 token。** 生成一个至少 32 个字符的随机串，只给本人读：

```sh
mkdir -p ~/node/.state && python3 -c 'import secrets; print(secrets.token_hex(32))' > ~/node/.state/node.token
chmod 600 ~/node/.state/node.token
```

**2. 网页口令。** `~/.config/hgamey/dash.pass` 是查看口令，`steer.pass` 是写操作口令，各写一行。

**3. 启动配置** `~/node/node-launch.json`：

```json
{"schema": "hgamey/node-launch@1",
 "node_token_file": "/Users/<用户>/node/.state/node.token",
 "service_args": ["--port", "8765", "--host", "127.0.0.1", "--data-dir", "/Users/<用户>/node/data"]}
```

- 要让局域网里的其他电脑访问，把 `--host` 改成本机内网地址或 `0.0.0.0`。
- 其他参数见 `python3 harness-ops/node/node_service.py --help`。

**4. 部署配置。** 在仓库里新增 `deploy/<节点名>.json`，可以照 `deploy/` 下现有的 macOS（launchd）或 Linux（systemd）配置改。这个文件只写路径和服务命令，不写凭据。字段说明见 hgamey 仓库 README 的"部署"一节。

**5. 注册系统服务。** 服务要执行的命令是：

```sh
~/node/harness-ops/node/run_node.sh --config ~/node/node-launch.json --no-tunnel
```

- 要公网访问时，去掉 `--no-tunnel`，再按 M5 配置隧道。
- Linux 用 systemd 用户服务，macOS 用 LaunchDaemon。
- 服务环境里要设好 M1 的 `DEVLOCK_DIR`；需要编译灰档层时，还要设 `ANDROID_NDK_HOME`。
- `run_node.sh` 会守护节点服务，服务退出后自动拉起。

**6. 构建并发布。** 在仓库根目录执行：

```sh
python3 deploy.py <节点名>                # 构建 origin/main 并发布、重启、核对版本
python3 deploy.py <节点名> --status       # 查看已安装的提交和服务实际报告的版本
```

回滚就是部署上一个提交：`python3 deploy.py <节点名> --ref <提交>`。

## 验收

```sh
curl -s http://127.0.0.1:8765/node -H "Authorization: Bearer $(cat ~/node/.state/node.token)"
                                          # 返回 node_id 和 service.revision（即部署的提交）
python3 deploy.py <节点名> --status        # 已安装的提交与服务报告的一致
```

浏览器打开 `http://<本机地址>:8765/dash/`，输入查看口令后：

- 首页能看到已连接的手机。
- 新建一个研究任务并保存，刷新后仍在。

## 常见问题

| 现象 | 处理 |
|---|---|
| 网页打不开 | 看服务是否在跑：`~/node/node_service.log`；`--host` 是否只监听了 127.0.0.1 |
| 首页手机为空 | 先回到 M1 的验收；确认服务环境里的 `DEVLOCK_DIR` 与 M1 用的一致 |
| 部署后出错 | 部署上一个提交 |
| 任务卡住 | `pf-node reset <任务ID> --note <说明>`（见 M4） |

不要手工覆盖安装目录，也不要删除 `releases/`、`deploy-state/`、数据目录或 token 文件。
