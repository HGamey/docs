# 节点启动配置与重启接续

2026-10-25，BASE-05 的第一批子项 BASE-LAUNCH-01。当前是实施合同，验证与发布后补；不代表整体目录分离或生产部署完成。

## 要修复的路径

原 `run_node.sh` 忽略附加参数，固定无参数启动 `~/node/harness-ops/node_service.py`。服务本身接受的 `--data-dir`、`--worker-prefix`、`--host-capacity` 因而无法通过该入口接续；`/admin/restart` 只退出进程，外层循环再次落到默认根。直接编辑安装脚本又会被 updater 覆盖。独立数据没有消失，但运行实例和节点身份可能换了。

## 启动与更新合同

- `run_node.sh --config /absolute/node-launch.json` 读取小 JSON，闭合字段为 `schema: hgamey/node-launch@1`、`service_args: [字符串...]`、`node_token_file: 绝对路径`。参数原样传给原服务解析器，不执行 shell 字符串；配置只存凭据文件路径。
- 也可以直接传原服务参数；与 `--config` 互斥。两种模式均可 `--no-tunnel` 只启动服务，供已有 ingress/supervisor 使用。无参数继续 legacy 默认目录，默认 token 位于 `HG_CONFIG_DIR` 或 `~/.config/hgamey` 下的 `node.token`。
- 脚本按自身目录定位 launcher/service。解释器可由 `HG_NODE_PYTHON` 显式选择；否则使用含 Homebrew 前缀的 PATH 中 `python3`。工作目录和日志仍为 `~/node`，默认隧道仍为原 cloudflared 安装与配置。
- launcher 启动时有界读取并验证配置/token，沿服务原 argparse 检查参数。错误直接退出，不退回默认根，不输出 token 正文。当前 launcher 生命周期内冻结参数和 token；修改配置或凭据需显式重启 launcher。
- `/admin/restart` 仍只重启 HTTP 服务，launcher 用同一 argv/解释器加载更新后的同目录代码。服务参数错误 exit 2 停止重试；其他退出按原间隔重启。关闭 launcher 只有限终止自己持有的服务/隧道进程，不把独立作业或设备恢复义务当作已完成。
- updater 仍只支持原 `~/node` 安装布局；本批不推广任意安装根。外置配置不在 rsync 覆盖范围。固定 worker 的 jobs_root 校验保持，不能因为改 data-dir 自动重装或改权。

## 数据与可观察性

服务启动日志显示 entry、phonefarm、data_root、jobs_root、node_id；Web/CLI 继续复用现有 `/node` 的 runtime/storage 身份。同一数据根接续原节点 ID，首次选择新的数据根是新的实例，不能冒充迁移成功。

省略 `--data-dir` 仍用 PF 根的 `_jobs`；显式使用则是 `D/jobs`，即使 D 等于原 PF 根也不同。本批不搬旧数据、不复制 ID、不联合扫描历史目录。旧 `_jobs` schema 可在原根读取，但完整服务会初始化队列，目录导出会写缓存，故不称严格只读历史挂载。

## 验收与剩余范围

以隔离 HOME/配置/安装副本做真实服务启动、版本更新和 HTTP restart，检查带空格参数、实例变化但 node_id/数据根/旧小任务不变；变更外置配置不影响已运行 launcher。另检查错误拒绝、直接参数和 legacy 默认。所有子进程有句柄并收尾，不连接设备、不读取生产凭据。按仓库规则运行本批唯一完整协议测试及必需 CI。

BASE-05 继续开放：独立缓存策略、上传密码与 HG_CONFIG_DIR 一致性、严格只读历史来源及显式迁移尚未交付；data_root 内目录登记含远端 token，不能将整个目录当无凭据缓存分享。
