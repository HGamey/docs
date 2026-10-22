# 指定目标灰层停用合同

2026-10-23，PRODUCER-RECOVERY-03A实施前规格。此批修共享off的正确性与消费者错误处理；完整gray-ab/eval的进程、trace、停充、游戏输入恢复继续03B，不能提前完成整job。

## 原问题

旧off忽略属性写入/删除错误，把任意非零test -e当文件不存在，返回人读字符串；eval仅排除特定文本就声称restored。它跨包find并删除同名SO，删除没有原值快照的4项legacy settings，却不停止已经加载层的进程。gray-ab单臂失败仍继续、最后exit0，frameaudit丢弃off失败。

## 本批行为

off改为接收可选显式包名并返回Result<DeactivationEvidence, DeactivationFailure>。两个分支保留实际命令原始输出及失败原因，不再由消费者解释中文成功文案。范围仅为指定目标的灰层停用：层属性关闭、记录的安装文件退出使用并删除、state清除；明确game_state_restored=false、job_recovery_complete=false，不写complete_recovery或将它当整个任务的恢复收据。

先读并验证state和debug.vulkan.layers，完成计划再写。state存在时必须是单个合法包名与绝对目录，显式目标必须匹配；安装目录须与当前目标nativeLibraryDir/arm64相符。旧安装路径已变化、state损坏、目标冲突、全局属性包含其它层时拒绝自动清理。路径和包名不拼接成未转义shell输入。

不再跨包find删SO，不删无快照的legacy settings，不删历史marker/采集证据，也不猜测共享临时SO属于本次。只删除明确state指向的目标SO和成功清理后的state。缺state时可按显式包名停止目标和关闭本工具属性，但报告清楚未验证未登记安装文件；没有明确目标不能声称任何进程已停止。

停用前先停止指定目标并用成功的进程名读回确认包名及子进程缺席，避免只删文件却让加载的层继续执行。每个属性写入/回读、删除/缺席核验必须成功；ADB/权限/解析失败都不能当不存在。文件存在/缺席以成功读取父目录的列表判定，不能将失败的test/cat等价为空。debug.vulkan.layers只允许空或本层，本层使用的11个debug.knobs属性（含当前构建仍读取的npuprobe）关闭并读回。共享输出明确未恢复原属性配置，仅证实停用状态。

## 消费者

- gray-layer off支持可选目标和JSON输出，错误非零；原普通模式和status保留兼容。
- gray-ab前置off失败即停止，不启动下一臂；单臂失败先执行必要摘层并记录二者结果，不能打印全部完成返回0。任何停用失败后不继续设备写入。完整session恢复仍待03B。
- frameaudit捕获和storeop-check保存typed停用结果，失败变为返回错误；不能采集成功但忽略收尾失败。
- eval gray改读typed结果，保存原始输出；layer_deactivated与整体restored分开。未验证完整恢复时restored保持未知，不能用本批scope产出“整job已恢复”的肯定判断；整个灰档恢复补齐后再确认此事实。

当前旧传输、挂层、run_one/ftrace/停充仍有未完成的ownership与回读工作；这批不声称解决宿主崩溃、远端遗留或恢复屏障，也不部署旧通路。03B先冻结全恢复义务，复用lease_command/confirmation/hwcond，不用新增通用框架替代明确状态。

## 验证

相关现有纯逻辑测试，少量fake ADB验证：指定目标正常停用并保留无关状态；读取/权限/删除失败不可冒充文件缺席；消费者前置停用失败禁止下一臂并不发布job完成。保留raw输出和失败记录。不连接手机、不重跑原神/Doom、不做无关网页布局测试。
