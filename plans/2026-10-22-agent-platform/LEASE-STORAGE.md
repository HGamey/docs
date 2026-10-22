# 持久设备锁目录与旧域停用

状态：宿主实现已发布（PF588b1c1、harness336eaaf、GOL29514d8），验证结果见 BATCHES；完整JOB-EXEC-03仍未完成。只涉及宿主锁与参与者路径，不搬实验数据，不部署节点，不证明设备已恢复。

## 路径只有一个解析入口

phonefarm Rust 统一解析：`PHONEFARM_STATE_DIR` 缺省为 `$HOME/.local/state/phonefarm`，devlock 缺省为其 `locks`，设备 lease 缺省为 `locks/devices`。状态目录不可用时明确失败，不退回临时目录。`DEVLOCK_DIR` 显式绝对路径标记为 custom，供现有部署和隔离测试使用；它不自动获得持久性保证。`PHONEFARM_LEASE_DIR` 可显式覆盖设备目录，也必须是绝对路径。`PF_JOB_LEASE_DIR` 是作业产物目录，与这两个目录无关。

宿主只读 `lease status --json` 增加 `storage`：schema_version=1，mode=default/custom，state_dir、devlock_dir、device_lease_dir，readiness=ready/migration_required/unavailable，error=null/说明。读取不创建目录。默认目录未就绪或不可读，外层状态必须 unknown/blocked，不能 clear。

tools/devlock 使用实际 phonefarm 二进制（`PHONEFARM_BIN` 或相邻安装），节点使用已配置二进制，GOL 使用命令所选二进制。消费者读取同一 storage，不各自猜旧路径。无法获得兼容合同就显示不可用并停止设备采样/写入。显式 custom 仍要尊重旧域退休标记，不能绕过它。

GOL `--lock-root` 只供历史锁日志读取；实时 fleet 使用 phonefarm 自身解析的目录。网页显示路径、模式和初始化状态，不提供未经协调的一键迁移。

本批 fleet/只读 preflight 仅在实际宿主 lease 明确 clear 时探测设备；held、blocked、unknown 或存储未就绪均暂缓，旧 label/force 参数不越过此边界。`preflight --fix` 在入口排队后获取并持有实际强 lease，使用它绑定的 serial；不会因为自己已有普通外层 owner 而误拒绝，也不凭 label 略过恢复屏障。节点保留已采快照和采样时间。之前基于普通 owner 的轻量采样不能证明强/嵌套 lease 下也安全，待有明确观察权限合同后再恢复。

初始化或协议错误立即返回，不作为“设备忙”重试两小时；GOL 的锁获取只重试脚本明确的 busy（exit 1），exit 2/启动失败直接报告。

## 显式初始化，不移动整个旧临时目录

操作员停止所有旧参与者后执行 `phonefarm lease init --participants-stopped`。此声明是协调前提，不是自动扫描保证。独立 worker 可跨 node 服务重启存活；仅调用 admin/restart 不代表已停止。run_node 的外层循环也不会替已有进程重新读取环境。

旧域仍为 `/private/tmp/claude-501`。初始化拒绝已有普通/强/嵌套锁、非空 queue、qmutex 或被持有的 bridge。旧恢复必须先在原域核查，不能通过新建空目录略过。持有旧 bridge 期间，发布以下两个绑定同一绝对目标的持久记录：

- 新 `locks/storage.json`：schema=`phonefarm/lease-storage@1`，devlock_dir=目标路径，legacy_devlock_dir=旧域。
- 旧 `devlock.d/phonefarm-lease.json`：schema=`phonefarm/lease-domain-retired@1`，devlock_dir=相同目标。

仅两个匹配记录都成立时默认域 ready。目标冲突或未知旧记录必须拒绝；允许同一目标重复执行修复，但不能删除新目录已有的恢复 marker。旧工具遇退休强标记拒绝写入。该机制不承诺未升级且绕过锁的工具安全。

这里的“旧工具”指已支持强 marker 拒绝语义的兼容版本。部署时所有入口须先升级；更早只看 owner 时间戳的 devlock 可能忽略退休 marker 并超时接管，不在保证范围内。init 不会替换磁盘上的旧二进制或停止已有进程。

不会复制、重命名、删除整个旧根，其中可能有与 phonefarm 无关的临时文件。旧日志/seen/queue/恢复原件留原地，通过初始化记录中的旧域位置追溯；本批不归档大文件。Linux 若旧父目录不存在或不可写，初始化明确 unavailable，不创建受限祖先或暗换旧域；新部署可显式配置可持久的 DEVLOCK_DIR custom。

宿主重启清空旧临时域时，默认状态回到 migration_required；停止参与者后再次显式 init 重建旧域屏障，保留新持久目录中的恢复记录。启动设备服务前须完成该步骤。不声称能阻止未升级旧程序在初始化前自行启动。

## 本批验证与剩余范围

以隔离 HOME/状态根和内部注入旧根验证：只读无副作用，活动旧参与者拒绝，两个域绑定，退休标记阻断，重建旧域时保留新恢复 marker；实际 CLI、devlock、节点、GOL 消费同一路径。全程宿主无害进程，不能把模拟临时域丢失写成真实断电/手机验证。

普通 legacy owner 超过 600 秒仍可能被旧规则接管；全局 bridge 串行、逃离进程组后代、原始 adb 脚本绕开 wrapper、各 producer 的恢复证据、hwcond 状态目录的临时回退、真实宿主重启与双节点设备验收仍在待办。

fleet 的 clear 检查仍是观察门禁，尚未取得覆盖整个采样过程的排他 lease；检查后其他命令取得设备的竞争窗口仍须结合观察订阅/测量互斥继续解决。
