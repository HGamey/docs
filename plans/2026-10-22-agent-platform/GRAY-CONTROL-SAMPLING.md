# 持续会话中的当前场景采样

状态：2026-10-23，03B4C2A 已实现，源码/宿主worker、实际PF + fake ADB + node HTTP及网页验收通过，发布索引见 [BATCHES](BATCHES.md)。承接 [GRAY-CONTROL-SESSION](GRAY-CONTROL-SESSION.md)。本阶段仅源码和宿主替身，不部署或操作手机。

## 首批结果与范围

Agent 或人连接已有会话后，可采一次当前场景的有限 trace 与环境数据；采完继续看屏、操作或再次采样。采样不重新启动/停止目标，不导航，不发送触控，不隐式调整充电、风扇或频率。03B4C2B1增加显式有限输入，详见末节；03B4C2B2增加创建时固定的 [供电政策与同期能量估计](GRAY-CONTROL-POWER.md)；reference/quality/refbench客户端已分别迁到持续owner或专属四臂Session。

新增 `sample` 动作，params 为 `{"seconds":30}`，整数1至120秒，可选 render_comm 为1至64字节、不含控制字符的显式线程筛选。网页默认30秒，不默认选择线程，API和CLI同范围。动作必须匹配当前 controller/epoch。提交仍立即返回 accepted，结果用原 request_id 查询；同 ID 不重新采样。开始前检查剩余会话预算至少覆盖 seconds+10；生命周期截止后不强杀正在执行的有界采集，命令退出/恢复可能延后，记录实际时间。

进入 measuring 后，同一串行通道不执行截图、触控、trigger 或其他设备动作；网页保留旧图和年龄，提示测量中。stop 可投递，在当前采样安全边界优先恢复；未开始动作保持 not_executed。采集成功、采集失败和资源恢复状态分别记录。

## 复用既有资源与三子进程协议

复用 `gray_eval_session::Parent` 的 owned remote 目录、独占 ftrace instance、worker、boot/进程组登记和原件回收。control 有明确 schema/owner，不冒充 eval。不能调用 `eval::run_owned_gray`：它包含停止目标和实验编排，破坏持续场景。

首批复用已有三子进程采样体：trace、sampler、纯 sleep workload。idle plan 总长 seconds+10，覆盖6秒 trace lead、请求窗口及读回余量；不采用恰好 seconds+6，以免写/读回开销使 trace 超出 workload 窗口。生成的 plan 不含 input。继承既有 ready、独立 guard、设备 uptime 和自然完成收据；不另写一套 PID/超时协议，不修改旧两子进程 gray_sample 的语义。

控制会话允许每次不同秒数，因此 plan 按 round 持久化。每轮脚本由受限 Plan 生成，保存在本地原件及 owner 目录的固定派生路径；路径只由内部 round ID 派生，不接受任意脚本路径或 shell。在首次上传前记录 round 和部署状态，worker 只运行这个固定位置。历史 round 使用自身 plan 校验自然完成，不能用最近一轮 plan 覆盖旧语义。eval/loadop 原固定 plan 保持不变。

可以给已有三子进程主体增加明确 control 模式与 round-kind；其唯一执行差异是受控 workload 的路径。每次请求按 sequence 派生唯一 round，两个不同 request_id 不覆盖目录。下载 trace、原环境采样、各 child exit/time、自然完成及 quiescence 原件。

## 父恢复义务

开始控制会话时创建资源父记录，并在任何资源部署写入之前绑定到 base manifest。新资源会话使用旧 C1 会拒绝的 base schema，防止旧程序忽略新增字段后提前完成。缺失或损坏父记录不能回落为 C1 的局部恢复。旧 C1 manifest 没有该绑定且未部署采样资源时，仍按原限定合同恢复。

整个会话停止/恢复的顺序为：核验 owner/boot → 全部登记进程组静止及各轮自然完成 → 停止并读回 tracing_on → 删除本会话 trace instance → base 恢复层/历史文件/12属性/目标停止 → 持久化完整父恢复报告 → complete_recovery。远端关闭账本和原件沿现有政策保留。普通 finish_control 必须拒绝跳过已绑定的资源父义务。

采样结束时核当前 worker 静止、trace 停止和本轮自然完成后才回 ready；会话的最终资源恢复义务仍保留。采集数据缺失而生命周期已核清时，可保留 failed 结果并继续会话。写入、进程或 trace 状态未知则停止接收动作、保留屏障；不能以退出码或历史 quiescence 文件代替当前验证。独立 session-recover 从同一 manifest 恢复全部范围，旧失败报告不改写。

## 结果与指标

每次 sample 返回不可变结果及原件的相对 out 引用。结果至少记录 collection 状态/错误、requested seconds、actual child/trace/sampler 窗口、source/evidence_origin、目标存活观察、trace parser summary、环境统计和限制。未启动或失败的采集同样留下记录；恢复完成不改写成采集成功。

复用 looptrace/env_stats 和已有证据绑定/窗口校验的纯计算部分，不引入另一套统计。不能隐式套旧 UnityGfxDeviceW 默认：没有 render_comm 时仍保存完整 raw/env，帧相关指标为null并提示未选线程；有显式筛选时记录筛选值和原parser摘要，但目标归属仍未验证。帧率名称明确为 GPU submit proxy，不能显示成显示 FPS；模块/队列耗时保留原有定义。功率仍取真实电池采样及供电状态，充电、缺字段或窗口不可核时为未知/null，不填0；旧旁路记录不能代替当前采样的供电资格。首批不发布每帧能耗：不把全窗平均功率与另一 trace 窗口帧率直接相除冒充同期能耗。不做跨运行收益裁决或自动质量判断。

公开 sample result 使用 path、collection(status/error)、requested_seconds、metrics、metric_provenance、limitations、artifacts(name/path)。所有公开路径相对 job out；run.json 内原件路径相对该 run 目录，并标 artifact_path_base。metrics 提供 gpu_submit_fps、frame_p95_ms、gpu_active_mean_ms、queue_p95_ms、power_w_mean，以及明确null的energy_per_frame_mj。没有采集到的值不用0补齐。

Web 提供秒数、开始采样、measuring/回执/失败、结果和原件入口；API/CLI沿已发布 action 协议，不新增第二个 job/设备 owner。旧节点按 allowed_actions 明确不可用。只读账户可看缓存和已产生结果，不能启动采样。

## 最小验收与后续

首批验证一条正常实际 PF + node HTTP + fake ADB 链：同 owner 两次不同秒数采集，中间抓图/有限输入，测量中观察不执行，结果/原件可查，同 ID 不重复。第二条注入完整父恢复失败并从原 context 继续，证明 base 不能提前 complete；旧 C1 恢复兼容用既有相关测试和一处最小宿主检查。worker只跑受影响模式/窗口回归，不重跑所有旧实验。Web检查新增表单、measuring、失败和结果链接，复用已有布局。

上述两条链已通过，319+287=606次门控fake传输；四项相关Rust回归和一个受影响worker用例通过。旧C1仅做了新版状态读取（零ADB）及恢复合同单测，未重跑旧C1整条恢复链。Web新增缓存接续：GET读取latest_sample，不触发采集；用户已选旧回执不被后续状态刷新覆盖。

03B4C2B1已增加显式有限输入；03B4C2B2继续充电绑定，03B4C2C 迁移 quality，03B4C2D 迁移 reference/refbench/旧挂层入口。旧 keep-prop 的“不启动”与 C1 的“保留属性但已启动”不同，不能直接别名。reference 旧延后功耗窗与新 owned 窗口必须分协议；refbench 保留四臂无层/改写、字符串 extras 和自然退出。上述子项已完成既定源码/宿主验收，03B4收口；整体目标仍active，部署/真机旅程、指标归属和同窗分母继续开放。


## 03B4C2B1：显式有限输入（已实现，宿主替身验收通过）

沿用 sample 动作。无 input 保持原 idle 行为；显式输入为 params.input（既有有限 script 数组或仅含 steps 的对象），同时必须提供 params.frame_id。frame_id 单独出现或 input=null 均拒绝。示例：

```json
{"seconds":3,"frame_id":"capture-1","input":[{"action":"sleep","ms":6000},{"action":"swipe","x":80,"y":100,"to_x":120,"to_y":100,"duration_ms":1000},{"action":"sleep","ms":6000}]}
```

复用有限 Plan 解析器，支持已定义的 sleep/tap/swipe/repeat 和精确旧数值 input swipe；不接受任意 shell、文件路径或隐式导航。seconds 仍1至120。显式计划必须覆盖 seconds+10 秒且不超过600秒；不悄悄补等待、循环或改变计划。该10秒余量沿当前采样合同覆盖6秒trace lead及读回余量。剩余会话预算按 max(计划声明时长, seconds+10秒) 检查；实际执行开销与原件中的真实窗口另记，不能将声明时长当实测耗时。stop继续在有界动作完成后收尾，不中途重放或另开owner。

输入必须引用最新且30秒内的真实截图，方向已知，全部坐标在该图边界内。owner先按缓存截图核坐标/新鲜度；在资源部署与本轮脚本上传完成后、worker放行前再次截取真实PNG并读方向，复核尺寸、方向、新鲜度及剩余预算。这样部署耗时不能绕过30秒图龄限制。保存 input-frame.json、原引用及检查截图作为输入几何证据；只证明开始时几何，不声称序列中场景/方向恒定。检查失败不启动输入或trace；可能已准备的owned工具/round脚本仍归原父会话收尾。各round冻结规范化plan、生成脚本与输入类型，恢复用该round原plan核自然完成，不能换成最近计划或默认idle。自然完成缺失时保持恢复义务。

新owner公开 action_capabilities.sample=2；C2A没有该字段，只支持idle。新版publisher在投递input前同时检查该版本与新资源父绑定，缺能力时返回明确错误且不进入旧owner队列。owner也检查同一合同。新资源父用 phonefarm/gray-control-resources@2，base仍用@3但绑定父schema=@2，因此旧C2A程序会拒绝新会话而不能按idle恢复。新版读旧@1保持严格idle校验，兼容旧C1@2无父与C2A恢复；@1不能用新输入plan升级。

新@2 round保留 input_kind=idle|finite：idle仍精确匹配Plan::idle，finite从保存的规范化steps重新解析并精确核对，二者均按每轮seconds验证。不改三child协议、child guard或自然完成schema；复用已验执行体。恢复收据仍表达完整资源/层/属性/停止目标范围，输入自然完成核验在前。

网页在原采样表单增加明确的“执行有限输入”选项与JSON步骤编辑；默认关闭，旧会话不提供该能力时禁用。开启时展示所绑定帧ID/方向、声明计划时长规则，使用同一action回执，保留失败和原件入口。CLI示例同批更新。网页不自动抓图，不把旧坐标缩放到新方向。

最小验证：扩展既有parser/owner/parent/mailbox相关用例，宿主HTTP正常链验证一次显式计划与后续idle round及幂等；不支持版本/非法计划/过时或变化几何在输入前拒绝。自然完成缺失用一条独立失败链证明仍保留义务，不用伪造收据解除。旧C1/C2A兼容用单测和旧原件只读检查，不重跑全部旧链。若执行体未改，不重复既有worker完整测试。本批不改供电、不部署、不碰真机。
