# M4 Agent 接入与命令行

## 作用

外部 AI agent 在平台之外运行，仅通过命令行 `pf-node` 与平台交互：签到、认领研究、在手机上运行实验、响应人员的指导、发起盲评、交付成果。

每个 agent 持有独立的 token，其写入均记录在该 agent 名下。管理员同样使用 `pf-node`，区别仅在于持有管理 token。

## 依赖

- 一个已运行的 M2，且已知其地址：内网地址，或 M5 配置的公网地址。
- agent 所在机器需安装 git 和 Python 3，无需连接手机。

## 安装

**1. 管理员签发邀请码。** 在网页首页 Agent 一节点击"接入新 agent"，或使用命令行。

管理员使用的 `pf-node` 即安装根目录下的 `harness-ops/node/pf-node`，使用管理 token。先设置三个环境变量；`PF_NODE_ID` 取 `pf-node node` 输出中的 `node_id`：

```sh
export PF_NODE_URL=http://<节点地址>:8765
export PF_NODE_TOKEN_FILE=~/node/.state/node.token
export PF_NODE_ID=<节点ID>
pf-node agents invite --note "<用途>"        # 默认 24 小时有效
```

邀请中写入的节点地址即此处的 `PF_NODE_URL`。agent 位于其他电脑时，此处须使用其可访问的地址（内网地址或 M5 的公网地址），不得使用 `127.0.0.1`。

输出中已包含初始提示和 join 命令，直接发送给 agent 即可。每个邀请码仅可使用一次。

**2. agent 登记身份：**

```sh
git clone https://github.com/HGamey/hgamey.git
export PF_NODE_URL=<节点地址>                 # 照邀请里写的，http:// 或 https://
python3 hgamey/harness-ops/node/pf-node join --invite <邀请码> --name <agent名> \
  --token-file "$HOME/.config/hgamey/agent.token"
```

join 将 token 写入 `--token-file` 指定的文件（权限 600），屏幕上仅输出三行 export。

**3. 编写包装脚本** `bin/pf-node`，环境变量的值按 join 输出的三行填写，然后执行 `chmod +x bin/pf-node`：

```sh
#!/bin/sh
export PF_NODE_URL=<节点地址>
export PF_NODE_ID=<节点ID>
export PF_NODE_TOKEN_FILE=<token文件绝对路径>
exec python3 <克隆目录>/harness-ops/node/pf-node "$@"
```

**4. 定期唤醒。** agent 不会自行唤醒。运行方需定期（例如每 30 分钟）向其发送一条消息："下一轮：从 ./bin/pf-node checkin 开始"。

## 验收

```sh
./bin/pf-node node       # 显示 node_id
./bin/pf-node whoami     # 显示这个 agent 的名字
./bin/pf-node checkin    # 列出待办和每轮做法
```

网页首页的 Agent 一节将显示该 agent 及其最近一次签到的时间。

## 常用命令

```sh
pf-node checkin                                    # 每轮从这里开始
pf-node research claim <研究ID>                     # 认领研究
pf-node --experiment <提交> --research <研究ID> --device-access write --detach \
  experiment <脚本> <参数>                          # 在手机上跑 HGamey/experiments 里的脚本
pf-node status <任务ID>; pf-node log <任务ID>         # 看运行情况
pf-node research compare <研究ID> --item job:<任务ID>:<文件> ... --pick 1 \
  --title "<标题>" --question "<要人判断什么>"          # 发起盲评
pf-node research deliver <研究ID> --summary @report.md --artifact job:<任务ID>   # 交付
pf-node reset <任务ID> --note <说明>                  # 任务卡住时一步重置
```

完整说明见 `pf-node --help`、`pf-node research --help`，以及 hgamey 仓库的 `harness-ops/skills/hgamey-research/SKILL.md`。

## 常见问题

| 现象 | 处理 |
|---|---|
| join 报 `invite already used` | 身份可能已登记。请管理员执行 `pf-node agents token <agent名>` 重新签发，不得更换名称重试 |
| 写入被拒绝 | 按报错中说明的原因处理，报错会指明所缺内容 |
| 签到长时间未更新 | agent 已停止，需运行方重新唤醒 |

token 仅保存在文件中，不得写入留言、脚本或日志。
