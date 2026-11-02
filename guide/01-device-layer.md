# M1 设备层（phonefarm）

## 作用

在一台电脑上连接手机，并提供：

- **设备连接**：Android 用 adb，HarmonyOS 用 hdc。
- **设备锁**：同一台手机同一时刻只有一个写者。锁是内核文件锁，持有进程退出时自动释放。
- **采集与评测**：帧率、温度、截图、录屏，以及灰档 Vulkan 层（M6 用到）。

M1 不含网页、任务队列，也不提供远程访问，这些由 M2 提供。

## 依赖

- macOS 或 Linux。
- Rust stable 工具链（`cargo`）、git。
- zsh：`tools/devlock` 是 zsh 脚本。macOS 自带，Linux 要另装。
- `adb`（Android 手机）或 `hdc`（HarmonyOS 手机）。
- 需要灰档层时，还要 Android NDK（r28c 已验证）。
- 手机打开 USB 调试，用 USB 直连电脑。

macOS 上可以用 Homebrew 安装，不需要 sudo：

```sh
brew install rust
brew install --cask android-platform-tools      # 提供 adb
```

Homebrew 的命令在 `/opt/homebrew/bin`。用 ssh 远程执行命令时，这个目录常常不在 PATH 里，先执行 `export PATH=/opt/homebrew/bin:$PATH`。

## 安装

```sh
git clone https://github.com/HGamey/hgamey.git
cd hgamey/phonefarm
cd src && cargo build --release && cp target/release/phonefarm .. && cd ..
codesign --force --sign - ./phonefarm      # 仅 Apple Silicon 需要，否则启动时会卡住
```

在 Apple Silicon 的 Mac 上，首次编译约 2 分钟。

设备锁目录用环境变量 `DEVLOCK_DIR` 指定。同一台电脑上的所有使用者（包括 M2 节点服务和手工脚本）必须用同一个目录，并且这个目录要能在重启后保留：

```sh
export DEVLOCK_DIR="$HOME/.local/state/phonefarm/locks"
mkdir -p "$DEVLOCK_DIR"
```

把这一行也写进 shell 配置（如 `~/.zshenv`），以及 M2 的服务配置。

同一台电脑上已有在用的节点时，用它正在用的锁目录，不要另建；否则同一台手机会有两个写者。只是试装、不接手机时，可以用新安装根下的独立目录。

## 验收

```sh
adb devices                       # 或 hdc list targets：能看到手机
./phonefarm devices               # 列出能连上的设备
./tools/devlock status            # 显示设备锁；没有任务时为空
./tools/devlock run test@<串号> -- adb -s <串号> shell getprop ro.product.model
                                  # 拿锁执行一条命令，退出后锁自动释放
```

`./tools/devlock status` 输出的 `storage.devlock_dir` 应是上面设的目录。暂时没有手机时，前两条只打印空列表；最后一条可以先换成 `./tools/devlock run test@demo -- echo ok` 验证锁本身。

四条都正常，M1 即安装完成。

## 常见问题

| 现象 | 处理 |
|---|---|
| `phonefarm` 启动后没有反应（Apple Silicon） | 重新执行上面的 `codesign` |
| `adb devices` 显示 unauthorized | 在手机上点"允许 USB 调试" |
| `hdc list targets` 很慢或卡住 | `hdc kill -r` 重启 hdc 服务 |
| 提示锁被占用，但没有任务在跑 | 先用 `./tools/devlock status` 看持有者；持有进程确实已不在时，把对应的锁目录挪走，不要直接删除 |

完整用法见 HGamey/docs 的 [repos/phonefarm](https://github.com/HGamey/docs/tree/main/repos/phonefarm)。
