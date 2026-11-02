# M1 设备层（phonefarm）

## 作用

在一台电脑上连接手机，并提供以下功能：

- **设备连接**：Android 使用 adb，HarmonyOS 使用 hdc。
- **设备锁**：同一台手机在同一时刻仅允许一个写入方。锁采用内核文件锁，持有进程退出时自动释放。
- **采集与评测**：帧率、温度、截图、录屏，以及灰档 Vulkan 层（供 M6 使用）。

M1 不包含网页和任务队列，也不提供远程访问，上述功能由 M2 提供。

## 依赖

- macOS 或 Linux。
- Rust stable 工具链（`cargo`）、git。
- zsh：`tools/devlock` 为 zsh 脚本。macOS 已自带，Linux 需另行安装。
- `adb`（Android 手机）或 `hdc`（HarmonyOS 手机）。
- 如需灰档层，另需 Android NDK（r28c 已验证）。
- 手机需开启 USB 调试，并通过 USB 直连电脑。

macOS 上可通过 Homebrew 安装，无需 sudo：

```sh
brew install rust
brew install --cask android-platform-tools      # 提供 adb
```

Homebrew 的命令位于 `/opt/homebrew/bin`。通过 ssh 远程执行命令时，该目录通常不在 PATH 中，需先执行 `export PATH=/opt/homebrew/bin:$PATH`。

## 安装

```sh
git clone https://github.com/HGamey/hgamey.git
cd hgamey/phonefarm
cd src && cargo build --release && cp target/release/phonefarm .. && cd ..
codesign --force --sign - ./phonefarm      # 仅 Apple Silicon 需要，否则启动时会卡住
```

在 Apple Silicon 的 Mac 上，首次编译约需 2 分钟。

设备锁目录由环境变量 `DEVLOCK_DIR` 指定。同一台电脑上的所有使用方（包括 M2 节点服务和手工脚本）须使用同一目录，且该目录须在重启后保留：

```sh
export DEVLOCK_DIR="$HOME/.local/state/phonefarm/locks"
mkdir -p "$DEVLOCK_DIR"
```

上述 export 语句须同时写入 shell 配置（如 `~/.zshenv`）及 M2 的服务配置。

若同一台电脑上已有正在运行的节点，须沿用其当前的锁目录，不得另建，否则同一台手机将出现两个写入方。仅作试装、不连接手机时，可使用新安装根目录下的独立目录。

## 验收

```sh
adb devices                       # 或 hdc list targets：能看到手机
./phonefarm devices               # 列出能连上的设备
./tools/devlock status            # 显示设备锁；没有任务时为空
./tools/devlock run test@<串号> -- adb -s <串号> shell getprop ro.product.model
                                  # 拿锁执行一条命令，退出后锁自动释放
```

`./tools/devlock status` 输出中的 `storage.devlock_dir` 应为上文设置的目录。暂无手机时，前两条命令仅输出空列表；最后一条可先替换为 `./tools/devlock run test@demo -- echo ok`，以验证锁本身。

四条命令均正常执行，即表示 M1 安装完成。

## 常见问题

| 现象 | 处理 |
|---|---|
| `phonefarm` 启动后无响应（Apple Silicon） | 重新执行上文的 `codesign` |
| `adb devices` 显示 unauthorized | 在手机上点击"允许 USB 调试" |
| `hdc list targets` 响应缓慢或卡住 | 执行 `hdc kill -r` 重启 hdc 服务 |
| 提示锁被占用，但无任务运行 | 先用 `./tools/devlock status` 查看持有者；确认持有进程已不存在时，将对应锁目录移走，不得直接删除 |

完整用法见 HGamey/docs 的 [repos/phonefarm](https://github.com/HGamey/docs/tree/main/repos/phonefarm)。
