# M5 公网入口（可选）

## 作用

通过 Cloudflare 隧道，使外网的人员和 agent 可通过一个 HTTPS 地址访问 M2，节点电脑无需公网 IP 和入站端口。

仅在内网使用时无需 M5，令 M2 监听内网地址即可。此外，M3 执行节点本身不依赖 M5。

## 依赖

- 一个已运行的 M2。
- 一个托管于 Cloudflare 的域名，以及具备隧道创建权限的 Cloudflare 账号。
- 节点电脑可访问 Cloudflare。

## 安装

**1. 安装 cloudflared**，存放于 M2 的工作目录：`~/node/bin/cloudflared`。

**2. 创建隧道**，并将一个子域名指向该隧道：

```sh
~/node/bin/cloudflared tunnel login
~/node/bin/cloudflared tunnel create hgamey-node
~/node/bin/cloudflared tunnel route dns hgamey-node <子域名>
```

**3. 编写隧道配置** `~/.cloudflared/hgamey-node.yml`：

```yaml
tunnel: <隧道ID>
credentials-file: /Users/<用户>/.cloudflared/<隧道ID>.json
protocol: http2
ingress:
  - hostname: <子域名>
    service: http://127.0.0.1:8765
  - service: http_status:404
```

**4. 修改 M2 的服务命令**：去掉 `--no-tunnel`，然后重启服务。`run_node.sh` 将同时守护节点服务和隧道。

## 验收

```sh
curl -s -o /dev/null -w '%{http_code}\n' https://<子域名>/dash/      # 返回 200 或 401
```

在浏览器中打开 `https://<子域名>/dash/`，应显示登录页。

## 常见问题

| 现象 | 处理 |
|---|---|
| 浏览器显示 Cloudflare 1033 | 隧道未连接：查看 `~/node/cloudflared.log`；确认该电脑可访问 Cloudflare（部分网络需经代理） |
| 大文件下载中途断开 | 网页会自动续传；如反复断开，应检查网络或代理 |

隧道凭据文件仅存放于节点电脑，不纳入任何仓库。
