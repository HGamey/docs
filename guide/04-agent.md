# M4 Agent 接入与命令行

## 作用

外部 AI agent 在平台之外运行，只通过命令行 `pf-node` 和平台交互：签到、认领研究、在手机上跑实验、回应人的指导、发起盲评、交付。

每个 agent 有自己的 token，写入都记在它名下。管理员也用同一个 `pf-node`，只是持有管理 token。

## 依赖

- 一个已运行的 M2，并知道它的地址：内网地址，或 M5 配的公网地址。
- agent 所在的机器装有 git 和 Python 3，不需要接手机。

## 安装

**1. 管理员发邀请码。** 在网页首页 Agent 一节点"接入新 agent"，或用命令行。

管理员的 `pf-node` 就是安装根下的 `harness-ops/node/pf-node`，用管理 token。先设三个环境变量；`PF_NODE_ID` 用 `pf-node node` 输出里的 `node_id`：

```sh
export PF_NODE_URL=http://<节点地址>:8765
export PF_NODE_TOKEN_FILE=~/node/.state/node.token
export PF_NODE_ID=<节点ID>
pf-node agents invite --note "<用途>"        # 默认 24 小时有效
```

邀请里写的节点地址就是这里的 `PF_NODE_URL`。agent 在别的电脑上时，这里要用它能访问的地址（内网地址或 M5 的公网地址），不要用 `127.0.0.1`。

输出里已经包含初始提示和 join 命令，直接发给 agent 即可。每个码只能用一次。

**2. agent 登记身份：**

```sh
git clone https://github.com/HGamey/hgamey.git
export PF_NODE_URL=<节点地址>                 # 照邀请里写的，http:// 或 https://
python3 hgamey/harness-ops/node/pf-node join --invite <邀请码> --name <agent名> \
  --token-file "$HOME/.config/hgamey/agent.token"
```

join 把 token 写进 `--token-file` 指定的文件（权限 600），只在屏幕上打印三行 export。

**3. 写包装脚本** `bin/pf-node`，环境变量的值照 join 打印的三行填，再 `chmod +x bin/pf-node`：

```sh
#!/bin/sh
export PF_NODE_URL=<节点地址>
export PF_NODE_ID=<节点ID>
export PF_NODE_TOKEN_FILE=<token文件绝对路径>
exec python3 <克隆目录>/harness-ops/node/pf-node "$@"
```

**4. 定期唤醒。** agent 不会自己醒来。运行方要定期（例如每 30 分钟）给它发一句"下一轮：从 ./bin/pf-node checkin 开始"。

## 验收

```sh
./bin/pf-node node       # 显示 node_id
./bin/pf-node whoami     # 显示这个 agent 的名字
./bin/pf-node checkin    # 列出待办和每轮做法
```

网页首页的 Agent 一节会出现这个 agent 和它最近一次签到的时间。

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
| join 报 `invite already used` | 身份可能已经登记过。请管理员执行 `pf-node agents token <agent名>` 重新签发，不要换名字重试 |
| 写入被拒绝 | 照报错里的原因处理，报错会说明缺什么 |
| 签到很久没有更新 | agent 已停下，需要运行方重新唤醒 |

token 只放在文件里，不写进留言、脚本或日志。
