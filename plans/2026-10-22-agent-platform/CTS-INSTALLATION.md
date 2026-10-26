# 旧CTS安装完成与缓存语义

DIST-CTS-INSTALL-01修复旧`test-batch`安装入口丢失transport退出码/stderr、空输出也可能判成功，以及仅凭`.sha256`缓存就跳过安装的问题。实现提交`acb396e`，操作指引`1a730dd`；PF `1a730dd` 已main/stable，code-only与node-build40299712131通过。

默认Android与精确`pm install -r {apk}`模板要求：host metadata工具成功并给出唯一包名；transport正常结束；本次唯一远端shell完成回执rc=0；精确Success；随后当前已安装单APK的实际SHA与本地输入一致。任何缺失或矛盾不判installed。目录扫描复用同一已核metadata包名，不再猜文件名或包名前缀；profile指定却缺失的本地APK明确失败。

已安装字节相同才跳过安装；远端缓存文件须当轮核真实SHA，只能复用传输。旧sidecar不再读写。元数据、shell输出和状态在`<out>/installation/<attempt>/`保留，stdout/stderr每条各至多1MiB；超时或超限失败且保留前缀。既有push_file只提供bool，其后独立接收端SHA检查才允许安装，不能声称原push输出也已被保存。

其他自定义模板和HAP/HDC缺安装身份/完成适配器，在设备写入前明确阻断。现有已安装OH模块/profile（无apk_path）和cts-fetch保持可用。需要这些安装格式时另补明确bundle/module身份、工具回执、状态核验与恢复合同；不把Android规则套到Harmony。

## 验证

唯一共享target两次warm构建42.600/15.898秒，第二次因scanner复用metadata这一实际修正；签名后新聚合回归首过7.054秒，最终26项CTS窄回归首过8.064秒。聚合用独立子进程隔离fake adb/aapt2/pm，真实运行生产代码和host shell，保留非零/空输出/丢回执/缺实际包/遗留缓存等原件。没有全量cargo重跑或真实手机操作。证据索引在本机`qa/resume-local-20261026/distribution-formats/CTS-INSTALL-RESULT.json`。

## 保留的边界

旧CTS仍是replace安装，不是typed games新装事务。本批安装未知只保存证据、返回Err且不自动重试，尚无跨进程持久typed recovery屏障。结束本地transport不证明远端安装已停止、已回滚或lease可安全释放。`recovery_verified=false`保持；后续统一transport结果与恢复屏障另列待办。真实Android工具输出/验签/权限、HAP支持和真机恢复均未验。
