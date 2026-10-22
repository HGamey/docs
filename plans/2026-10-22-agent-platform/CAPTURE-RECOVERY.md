# capture 执行与收尾范围

2026-10-23，PRODUCER-RECOVERY-02实现前合同。仅宿主/模拟验证，不恢复原神或Doom真机实验。

## 当前问题

capture通过旧Genshin插件修改常亮、唤醒/启动、注入连续HID并采图。准备或落盘失败会跳过结尾shutdown；正常shutdown也忽略退出码且没有回读。它会改变游戏进度，固定强停/息屏不是恢复原游戏状态。

本批恢复scope仅为本次输入通道、常亮原设置、请求的退出策略。明确排除角色/视角、对白/任务/拾取、账号进度、存档和原前台页面。shutdown策略是停止目标应用并使屏幕休眠；no-shutdown保留游戏/屏幕运行，但仍释放本次输入和恢复临时设置。

## 执行所有权

持强lease的父进程创建独立持久session，记录serial、boot、原常亮设置、退出政策、唯一HID名称和输出目录；在第一处写入前hold。输出用独立子目录，避免再次运行覆盖旧frame编号却追加旧manifest。

复用lease_command监督同exe的capture worker整个PGID。先记PID/PGID再放行；worker核对父lease身份，以父存活管道和独立deadline有界执行。HID持续连接、截图和短ADB均属于该worker组；不把它们逐个塞进只有一个child槽位的lease。父进程恢复步骤在worker组结束后顺序执行。

Genshin插件只抽取其所需的小设备接口，旧Device提供默认实现，capture提供有界、留原件的实现；不改全局Device后端。截图保存二进制PNG，不能经过UTF-8转换。手柄使用唯一session名称，记录注册是否明确可见；未使用手柄时收尾不得懒创建手柄。原插件的mode/auto_choice决策缺口单独入todo，本批不改游戏策略。

短写入前持久pending，明确完成后才清除；超时、未确认退出、HID注册未知、worker遗留组保持屏障，不能只用本地ADB退出或后续设置回读覆盖不确定性。active session还必须有worker在停止所有设备操作、销毁已有输入实例后写出的身份绑定completion记录；watchdog/崩溃或无法落盘时缺少此记录便保留屏障，避免一次uncertainty保存失败后被旧session误放行。父死/异常退出仍可留下本地证据；不把宿主watchdog称作远端已恢复。

## 恢复与结果

采集循环的成功/失败统一交给父进程收尾；插件不自行用固定stayon false覆盖原值。父进程核对boot和session，确认所属HID输入设备缺席（已登记的名称及明确读回），恢复原常亮值并读回。默认shutdown还需确认目标应用已停止、屏幕已休眠；no-shutdown单独记录保留政策，不声称恢复了游戏或屏幕。

无法核对时保留有原因的required证据和lease。只有以上声明范围全部确认才写verified证据并调用complete_recovery，CLI再报告reported_complete。正常frame目标未达到可以同时是采集失败、恢复完成。--recover仅允许同serial/context的结束owner接管；未知写入/注册仍阻断，不能自动猜测。

## 验证

只做与改动相关的既有单元回归、scoped worker父死/超时，以及少量实际CLI/fake ADB链：准备后输出失败仍恢复原常亮；恢复失败阻断新任务、明确续接；输入创建/退出证据和未知状态保留。node沿现有scope/origin/evidence接口显示；协议未变则不重复整套网页布局和node测试。

这些测试不证明真实Android HID卸载、系统设置、息屏、游戏停止或性能，需要后续指定设备验收。部署与代码发布分开。

## CLI与产物

`capture --serial S --out P --frames N --timeout-s 3600 --json`在P下新建唯一capture-session，帧和manifest位于该session的frames目录；JSON的out/manifest/session/report_path给出实际路径，不能继续猜P/manifest.jsonl。默认3600秒，上限86400秒，截止覆盖worker中的游戏准备与采集；父侧快照与恢复命令各有独立短超时，不能把此参数称为整个CLI的总耗时上限。

`capture --serial S --recover SESSION --json`只接续已结束owner的同一context，沿用保存的退出政策。恢复不继续采帧；未知写入、未确认HID注册、缺worker完成记录保持required，独立核验路径另列RECOVERY-UX-01。

每次报告与恢复证据独立保存；失败后续接不改旧原件。原证据verified与已向lease发布reported_complete分开。evidence-origin缺省device，模拟必须显式synthetic。PNG保留原始二进制；worker短ADB的stdout/stderr/命令及退出记录留worker-commands，父命令留commands/journal，worker日志留worker-output。

## 2026-10-23宿主验证

已签名macOS构建及27项相关Rust单测通过；11次实际CLI、64次fakeADB/HID访问检查通过。覆盖准备后manifest失败仍收尾、恢复读回不符/新任务零ADB/同context续接、known HID/no-shutdown恢复原未设置值、unknown HID保留pending、deadline缺completion、父SIGKILL后EOF结束worker组、PNG逐字节保留。成功恢复证据原件仍verified，CLI及job收据reported_complete，旧失败原件不变。fixture PID已清理。

检查记录在本机developer_transition_20261022/qa/capture-unit-results.json、capture-recovery-host-validation.json和两个实际CLI目录024521/024652。两次runner校准负结果（外层gate无JSON、Darwin teardown暂时EPERM）保留；未改观察判据为误认退出。真实Android HID卸载、设置、游戏停止与息屏仍待指定设备验收，本批不部署。
