# M5 公网入口（可选）

## 作用

通过 Cloudflare 隧道，让外网的人和 agent 用一个 HTTPS 地址访问 M2，节点电脑不需要公网 IP 和入站端口。

只在内网使用时不需要 M5，让 M2 监听内网地址即可。另外，M3 执行节点本身不需要 M5。

## 依赖

- 一个已运行的 M2。
- 一个托管在 Cloudflare 上的域名，以及能创建隧道的 Cloudflare 账号。
- 节点电脑能访问 Cloudflare。

## 安装

**1. 安装 cloudflared**，放到 M2 的工作目录：`~/node/bin/cloudflared`。

**2. 创建隧道**，并把一个子域名指向它：

```sh
~/node/bin/cloudflared tunnel login
~/node/bin/cloudflared tunnel create hgamey-node
~/node/bin/cloudflared tunnel route dns hgamey-node <子域名>
```

**3. 写隧道配置** `~/.cloudflared/hgamey-node.yml`：

```yaml
tunnel: <隧道ID>
credentials-file: /Users/<用户>/.cloudflared/<隧道ID>.json
protocol: http2
ingress:
  - hostname: <子域名>
    service: http://127.0.0.1:8765
  - service: http_status:404
```

**4. 改 M2 的服务命令**：去掉 `--no-tunnel`，然后重启服务。`run_node.sh` 会同时守护节点服务和隧道。

## 验收

```sh
curl -s -o /dev/null -w '%{http_code}\n' https://<子域名>/dash/      # 返回 200 或 401
```

浏览器打开 `https://<子域名>/dash/` 能看到登录页。

## 常见问题

| 现象 | 处理 |
|---|---|
| 浏览器显示 Cloudflare 1033 | 隧道没连上：看 `~/node/cloudflared.log`；确认这台电脑能访问 Cloudflare（有的网络要经代理） |
| 大文件下载中途断开 | 网页会自动续传；反复断开就检查网络或代理 |

隧道凭据文件只放在节点电脑上，不进任何仓库。
