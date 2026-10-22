# 观察采样的所有权（LEASE-OBSERVE-01）

状态：2026-10-23 宿主隔离验收通过，发布进度见 BATCHES；不部署或接设备。

## 问题与范围

fleet、只读 preflight、screen 原来先查 clear 再调用 transport，检查结束后可能与测量同时运行。screen 的 Android 截图还有无界等待。共享状态查询不是授权；必须在枚举、尺寸查询、shell 和截图整个窗口持有同一设备域。

本批复用强 Lease、全局 bridge 和 devlock.d，不新增第二套锁。观察使用保留的域身份 `@observation`，不能当作实际设备 serial。它不借用 DEVLOCK_LABEL，不加入等待队列；存储未就绪、主/嵌套记录、qmutex 或已有等待者均暂缓。原子 mkdir 决定与普通 devlock 的先后，旧脚本不得接管 owner 尚未发布的目录。当前全局域仍串行；分设备并发另列待办。

## 执行窗口

父进程取得非阻塞强 Lease，先记 command_pending，再启动独立进程组中的内部采样 worker。worker 在 stdin 启动屏障后核对 marker、token 和自己的 PGID，才执行设备访问。父进程记录 child 身份之后才放行。该内部入口不接受任意命令。

fleet 整轮最多 30 秒，只读 preflight 最多 45 秒，screen 每帧最多 20 秒。父进程等待真实 child，并确认进程组已消失才清除 child 记录和释放。超时、信号、transport timeout、等待失败或残存后代会结束本地进程组并保留 marker，阻止新测量；本地终止不能证明设备端已停止。父进程异常退出同样留下可审核 marker。worker 独立监听父进程存活管道，并设置自己的硬截止；父进程死亡或停顿不能留下无限运行的采样组。不会自动偷取或自动确认恢复。现有明确的人工 idle 确认恢复通路可用于核查后的观察记录。

没有订阅时 screen 不启动 worker；每帧释放后才休眠，页面暂停只停止后续帧。已经开始的一帧有界收尾。共享服务与页面读取缓存不占有设备。HDC 失败不得把上次截图冒充新帧。

## 结果合同

- fleet 成功：原 `phonefarm/fleet@1`，增加 `observation: sampled`，devices 是数组；exit 0 至少一台，exit 1 成功但没有设备。
- 获取所有权失败：同 schema，`observation: deferred`、reason 字符串、`devices: null`，exit 3，无设备探测。
- 执行失败：`observation: error`、reason、`devices: null`，exit 3，不能冒充新鲜空列表。
- consumer 兼容旧成功格式，但必须有正确 schema、devices 数组及成功退出码。deferred/error 保留最后成功快照与时间；尝试时间单独更新以节流。无历史数据保持未知。网页同时显示旧数据年龄与暂缓/错误原因。

## 验证与限制

最小验证覆盖：采样期间实际 legacy/strong 竞争者不得启动；等待队列优先；同 label 不能借锁；无设备 exit 1 清空缓存；deferred/error 不刷新成功时间；超时或父进程终止后 marker 阻断；无观看者不采样；窄屏显示旧快照和原因。

本批不保证任意脱离进程组后代、已有 adb/hdc 常驻 daemon 或远端命令因宿主信号退出。这些不确定性保留在恢复屏障与后续 transport 工作中。未升级的旧 devlock 仍有空 owner 接管竞态，部署必须同时升级参与者。不会以模拟协议测试宣称真机性能验证完成。
