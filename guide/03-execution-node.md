# M3 执行节点（第二台及以后的电脑）

## 作用

让另一台电脑上的手机也由同一个中心（M2）调度。人只打开中心的网页，agent 只连中心。执行节点主动连接中心，不需要入站端口、SSH 或公网隧道，放在 NAT 后也可以。

执行节点不另建研究库或网页。

## 依赖

- 一个已运行的 M2，作为中心。
- 新电脑已装好 M1 的依赖：Rust、git、Python 3、adb 或 hdc。
- 新电脑能用 HTTPS 访问中心地址。

## 安装

**1. 在中心生成机器邀请码**（用持有管理 token 的 `pf-node`，见 M4）：

```sh
pf-node nodes invite --note '第二台电脑'
```

**2. 在新电脑上加入：**

```sh
git clone https://github.com/HGamey/hgamey.git && cd hgamey
python3 deploy.py join --center https://<中心地址> --invite <邀请码> --name <节点名> --root "$HOME/node"
```

- `--root` 必须是不存在或为空的目录。
- join 会自动完成这些事：生成本机身份、私有 token 和配置；构建并发布；注册系统服务并启动。Linux 用当前用户的 systemd，macOS 用登录用户的 launchd。
- 同一台电脑上已有 M1 在用时，先把 `DEVLOCK_DIR` 设成它正在用的目录，再执行 join。

**3. 以后升级：**

```sh
python3 deploy.py --config "$HOME/node/deploy-node.json"          # 升级到 origin/main
python3 deploy.py --config "$HOME/node/deploy-node.json" --status
```

## 验收

在中心执行：

```sh
pf-node nodes list                         # 能看到新节点，状态在线
pf-node nodes show <节点名> --devices       # 能看到新电脑上的手机
```

中心网页首页也能看到这台电脑的手机。

## 常见问题

| 现象 | 处理 |
|---|---|
| join 中途失败（构建出错） | 不要重新 join，直接执行上面的升级命令继续；邀请码不会再消耗 |
| 节点显示离线 | 看新电脑能否访问中心地址；电脑重启后服务会自动起来 |
| 要换名字 | 由管理员执行 `pf-node nodes update <原名称> --revision <当前revision> --rename <新名称>` |
