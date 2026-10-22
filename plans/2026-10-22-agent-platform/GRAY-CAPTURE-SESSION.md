# 灰层抓帧会话

2026-10-23，PRODUCER-RECOVERY-03B1实施前规格。先把完整会话接到现有有限操作frame-audit capture；storeop-check/eval共享调用的lease借用，以及gray-ab的后台采样/ftrace/停充仍分批处理。不得新建无调用的恢复骨架。

## 真实入口与范围

capture显式取得强Lease并移交GraySession，所有设备读写走现有串行lease_command，第一次写前durable hold。新out是独立session子目录，stdout/metadata明确给路径。新增frame-audit recover --session DIR --serial S，只接续同serial/context、旧owner已结束的会话；不从普通抢锁绕过恢复标记。analyze保持离线；storeop-check本批不迁，以免在eval已有lease内重复抢锁或提前释放。

会话保存boot、包与当前安装目录、12个原属性值、登记的安装/备份范围、每次写入意图。目标包校验、foreign全局层、已有layer state/安装SO、可读目录与本地SO校验先于首次写，已有未知安装拒绝覆盖。原属性必须在旧off前快照，新入口不调用旧off。截图保存原始字节；JSON直接有界读取，不复用fa_shot/fa_pull共享文件；SO staging用session独占目录。

停止目标并确认包及子进程缺席后，保存旧marker/audit文件；文件在原目录以session专属名字改名备份，不能删除历史证据或把读取失败当不存在。新采集材料归档在session目录；收尾只删除/恢复本session明确登记的文件。安装、属性设置、启动、触控点击、FA_GO触发均进入同一持久写入记录，失败传播而非继续下一帧。当前有限抓帧不用新增远端后台worker。

## 恢复与证据

完整收尾先核对boot/目标安装路径与无未完成写，再停止目标并确认缺席，保存本轮证据，移除本session安装SO/临时文件，恢复备份和12属性原值并逐项核验。全局层原值必须为空。只完成该会话的“原属性/自有安装和输出文件/目标关闭”范围，game_state_restored=false，不声称存档、角色、原前台或其它任务恢复。

超时、启动/等待不确定、未完成写pending或读回无法核验保留屏障，停止后续设备操作；不能凭后来读回正常推断旧写已结束。recover可重试已知安全范围，未知旧写需独立核验，当前不自动清除。采集成败与恢复成败分别持久记录，旧失败原件不覆盖；即使本地结果落盘失败，也执行可证明安全的收尾并报告恢复记录。

## 验收

仅必要现有lease/devlock回归和少量实际CLI/fakeADB：正常抓帧含旧属性/历史文件恢复、采集失败仍收尾、恢复读回失败阻断后续写、同context恢复保留旧失败、未知写与错误serial拒绝。PNG字节与命令门控直接核验。scope不包含sampler/ftrace/停充，因此不能据此完成03B父项。无部署、真机或原数据迁移。
