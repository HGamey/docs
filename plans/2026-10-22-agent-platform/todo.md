# 开发待办

更新：2026-10-23。**勾选仅表示已完成，不表示计划已经实现。** P0是第一批必要基础/已知缺陷；P1是第一条完整流程；P2/P3是后续增强或研究候选。网页与CLI/API对同一条目共同验收。

## 已完成的准备

- [x] PREP-01 暂存Doom与后台排查；本机写入恢复交接和继续工作提示词，更新Doom todo入口。
- [x] PREP-02 确认本机382个node作业都有终态记录或exitcode；本轮未发真机任务。
- [x] PREP-03 核查组织九个仓库与当前main，区分上游、本地增强、已归档和结项项目。
- [x] PREP-04 完成两轮目的讨论：外部Agent、多方向探索、异步人类协作、网页/API同级、共享研究状态。
- [x] PREP-05 形成项目地图、使用痛点、阶段计划和本待办；记录直接commit/push规则。
- [x] PREP-06 准备独立的当前docs main稀疏工作区，只取README和本轮规划；保留原有交接分支与本机原始实验数据。发布提交与验证回执另行记录。

## P0：代码收口、合同与确定缺陷

- [x] ALIGN-01 对照维护者手册，分开共同原则、产品默认入口差异、未涉及要求与作者自报证据；写入产品准则。
- [ ] ALIGN-02 按准则完成端到端验收，不能以本地单测或设备枚举代替平台可用；持续记录对话修正及每批证据。
- [x] UI-01 以任务页建立可复用的配色、间距与反馈规范，按FRONTEND验收浅深色与三个宽度。
- [ ] UI-02 将同一设计/交互规范推广到节点、游戏、队列和人工协助页面，保留真实空态/错误态。

- [ ] BASE-01 对照上游与本地分叉，列可回收提交/补丁、依赖、测试；保护phonefarm与GOL未提交修改。验收：每项有去向，不一把推历史分支。
- [ ] BASE-02 回收job唯一锁名、raw adb/fetch_apk锁路由、fleet队列解析等既有增强；验证与现有强lease/bridge兼容。
  - 唯一 job owner 与 raw 命令锁路由已发布 `harness-ops 2b3c741`；只读 fleet 查询/队列显示也已发布 `947a4b9`；旧global bridge之外的强lease集成仍待验收。
- [ ] BASE-03 回收APK SHA缓存、partial验收/隔离、显式serial、取消隐式全权限/音量修改等成果。
- [ ] BASE-04 固定三仓源码、运行二进制、协议版本关系；CLI和网页都可查询，客户端遇不兼容时解释具体差异。
- [ ] BASE-05 分开安装代码、配置/凭据、运行数据、缓存；为旧目录提供读取/迁移适配，保持原始证据。
- [ ] BASE-06 统一node/device/study/direction/experiment/job/artifact/human-request的ID和状态草案；测试版本化读写与并发更新。
- [ ] BASE-07 统一结构化错误、退出码、可重试性、缺能力/缺权限/离线/忙/恢复阻断；CLI文本与JSON语义一致。 capture恢复宿主验证再次确认devlock前置拒绝只有stderr、无JSON；统一处理外层gate，不在每个producer各造格式。
- [ ] BUG-01 修复screen前端暂停/关页面仍在后台抓图；观察订阅和性能测量共享资源状态。无设备计数测试+后续真机开销检查。
  - 已发布订阅生命周期修复 `phonefarm 591889e`；本批 LEASE-OBSERVE-01 增加每帧互斥与异常屏障。远程认证接入与真机开销仍未完成，完整条目不提前勾选。
- [x] BUG-02 修复服务重启后job取消失效（225cb5c）；独立worker持有命令进程组和实际子进程对象，flock确认所有权，不凭保存PID发信号。旧任务继续可读，无法确认所有权时明确不支持取消。
- [x] BUG-03 只读状态查询不再创建job；超时节点不能静默消失，返回完整列表和分项错误（harness-ops `947a4b9`，模拟协议与浏览器验收；未部署运行节点）。
- [x] BUG-04 修复原件只读授权缺口，覆盖元数据/日志/单文件/Range；验证访客不能开任务或访问配置凭据。
- [ ] TEST-01 为harness-ops建立协议/故障回归：重复请求、同label、断网、重启、取消、缺原件、坏缓存、路径边界、auth。
- [x] TEST-02 直接push流程的CI覆盖当前节点协议回归（Python3.11/3.14）及页面脚本检查；构建成功与真机验收分开记录，新模块继续补必要覆盖。
- [ ] TEST-03 将长集成中的真实等待与快速回归分开；调查并行 confirmation_driver 的 deadline/heartbeat 偶发失败，不靠反复全套重跑掩盖。
- [x] MERGE-01 修复旧 PR 带入的 gray eval 循环资格阻断；未知 control 可采证但不能形成结论（`8dba439`）。
- [x] MERGE-02 同步普通/动态灰层构建输入与 GOL 消费端，补 npuprobe.cpp；NDK实编、旧工件与跨仓合同验证通过，已发布 phonefarm `6cd1ac5`、GOL `1a8601e`。
- [x] MERGE-03 按 [GRAY-MODE-ADMISSION](GRAY-MODE-ADMISSION.md) 完成已知copyprobe/upop CLI/公共mount/旧shell入口准入，155506f检查通过直推main。默认unsupported、自定义SO capability_unverified；构建/离线分析保留，未开放注入。自定义工件未来执行能力合同另定。
- [ ] DOC-01 同步各仓AGENTS/README/组织入口的外部Agent定位与直接commit/push规则，保留代码仓文档边界。
  - phonefarm、harness-ops、game_opt_loop、refbench 的 AGENTS/直接提交流程已同步；组织总入口与全部旧说明仍待梳理。

## P1：多机管理

- [ ] FLEET-01 稳定node/device身份、显式登记/撤销/更新；域名、执行host和数据位置独立。
- [ ] FLEET-02 网页节点/设备列表、CLI list/status/details共用API；显示运行版本、最近见到、当前任务/队列/lease/恢复态。
  - `/dash/fleet`、`pf-node fleet`和跨节点`pf-fleet status`已共用读取合同；网页目前只展示当前节点，跨节点注册与强lease/恢复态未完成。
- [ ] FLEET-03 状态缓存、TTL、新鲜度与连接恢复；设备缺席不误报整个节点离线。
- [ ] FLEET-04 能力按设备实例/epoch记录，区分代码支持、离线验收、实机验收及条件；Android root/nonroot、Harmony、iOS分别呈现。
- [ ] FLEET-05 一条doctor路径检查连接、工具、签名、版本、磁盘、设备授权和权限，输出可执行修复项；不反复全量onboarding。
- [ ] FLEET-06 节点端自检与安全更新、原子替换、回滚版本；有实验运行时更新语义明确。
- [ ] FLEET-07 CPU编译/分析与手机实验资源分开，能力标签不混成设备存在证明。

## P1：游戏与任务分发

- [ ] DIST-01 定义游戏bundle manifest：应用/引擎/包/资源/存档/场景/配置/来源/哈希/设备条件/空间/回执。
- [ ] DIST-02 网页游戏目录与准备状态，CLI inspect/prepare；分别显示已下载、已安装、资源就绪、等待登录、场景就绪。
- [ ] DIST-03 复用已有Doom样本建立第一个recipe；不重新下载现有大资源，不在规划阶段触碰手机。
- [ ] DIST-04 节点直取文件、流式接收、断点续传、接收端hash、原子完成、内容缓存和空间预检。
- [ ] DIST-05 任务输入/输出改为显式工件引用和依赖清单；处理嵌套JSON/脚本路径，避免只有argv重写有效。
- [ ] DIST-06 资源包/OBB/loose文件/存档按目标平台受限路径安装；保存旧存档与恢复收据，检验游戏UID可访问性。
- [ ] DIST-07 split APK/XAPK/HAP等格式按实际需求追加适配，先列支持矩阵，不宣称单APK流程已覆盖。
- [ ] DIST-08 登录/授权/资源采集的人工任务挂到方向；完成后重新验证游戏状态，其他方向继续。
- [ ] DIST-09 缓存去重、配额、引用计数/保留策略、可审查清理；默认不删除唯一原始证据。

## P1：任务队列与执行

- [ ] JOB-01 持久作业状态机，区分准备/运行/采集/恢复/终态与等待原因；外部Agent提交立即拿ID。
- [x] JOB-02 请求幂等和重复提交去重（58fc0dc）；同ID同内容返回原job，不同内容409，CLI/API/网页可找回；刷新后重填保留原ID。旧客户端未提供ID时不具备跨请求去重。
- [ ] JOB-03 资源队列原子领取、设备lease联动、主机资源预算；多设备并发不被无关全局锁意外串行。
  - 58fc0dc已完成SQLite原子领取、声明资源排队及宿主并发上限；本批2b4747e接入宿主lease屏障、恢复任务和独立阻断容量；原子设备所有权与旧global锁解除串行仍待做。
- [ ] JOB-04 冻结作业输入、程序/版本/设备身份；日志与事件可增量续读，状态更新有revision。
- [ ] JOB-05 网页队列、任务详情、准备进度、日志、取消/恢复操作；CLI/API同等功能。
- [ ] JOB-06 掉线、重启、超时、用户取消、失去lease后执行明确收尾；恢复失败占用独立屏障，不用exit0掩盖。
- [ ] JOB-07 真正的取消与安全点暂停语义；非幂等步骤不能自动retry，重试使用新attempt并保留旧失败。
- [ ] JOB-08 一条方向等人/缺设备时，其他可行任务继续；队列等待与Agent研究优先级分开。
- [ ] JOB-09 任务/研究预算、超时与磁盘上限；停止后可解释哪些动作发生、哪些没发生、哪些恢复待核。
- [ ] JOB-10 完成两节点故障注入验收，然后单列实际手机的准备、执行、还原验收。

- [x] JOB-EXEC-01 [独立 worker 与重启后取消](JOB-EXECUTION.md)，网页/CLI同一回执；真实 HTTP 服务进程重启与无害子进程验证通过，225cb5c已发布，未部署。
- [x] JOB-EXEC-02 [提交幂等、持久队列与原子领取](JOB-QUEUE.md)，58fc0dc已发布；44项本机测试、Linux双版本CI、网页提交/找回/撤销/释放验收通过，未部署。
- [ ] JOB-EXEC-03 lease/恢复屏障与执行收据：worker丢失或恢复未知时限制后续设备写入，明确脱离进程组后代与主机重启的处理边界。
  - 第一批[宿主屏障/恢复通道](JOB-RECOVERY.md)已实现：phonefarm cafdbba、harness-ops 2b4747e。47项协议、跨仓真实宿主锁/无害进程和浏览器验收通过；恢复worker丢失也保留资源，祖先链恢复不放行同级任务。未部署/未真机，完整条目继续保持未勾选。
  - 第二批job身份与scoped收据已实现：phonefarm dc35335、harness-ops 11b50d9。节点启动时传入显式身份，强lease与恢复接管输出有来源/范围的收据，CLI/网页按需读取；不自动解除整个job屏障。验证和发布进度见BATCHES。
  - LEASE-STORAGE-01 已统一持久lease域并提供显式迁移合同；LEASE-OBSERVE-01 补采样窗口所有权；LEASE-STORAGE-02A 已修hwcond恢复记录（e06e1ff）。bench/capture已补producer typed恢复证据及实际lease hold（2f5f11d/bd67d97）；下一条gray-ab/eval；真实主机重启/脱离进程组后代仍待验收，不能把手动放行当还原证据。
  - producer覆盖后续：bench/capture已完成各自声明范围，gray-ab若干off结果未检查，eval摘层仍用字符串检查；逐条按已知范围保存typed证据和恢复屏障，再调用complete_recovery。当前均不得从released/not_reported推断恢复，不让node猜实验产物格式。
  - 旧devlock心跳遗留管道问题已修复b986fcf：zsh内建等待，明确停止并回收自建心跳；5项隔离回归通过，取消保留锁，不把信号退出当还原。

JOB-EXEC-01/02只覆盖 JOB-01/03/04/05/06/07 的基础部分，不提前勾选这些完整条目。下一批围绕 JOB-EXEC-03/JOB-03/06；新想法先记录，不切走当前工作。

## P1：结果发现、共享与研究状态

- [ ] RESULT-01 任意合法job显式注册artifact/result，不再按命令名和近期200条目录猜测。
- [ ] RESULT-02 元数据索引与原件存储位置分离；数据留原节点也可跨节点检索，复制/迁移是显式动作。
- [ ] RESULT-03 网页按游戏/设备/研究/方向检索、比较与下钻，CLI/API分页/游标查询同一索引。
- [ ] RESULT-04 只读分享可发现、查看和下载注册原件；路径/符号链接边界与凭据/日志脱敏回归。
- [ ] RESULT-05 旧Doom自定义结果和GOL报告的兼容读取适配，原文件不可变；不靠重新运行实验补索引。
- [ ] RESULT-06 指标schema明确模块/窗口/单位/电源轨/分母/ROI/全帧/实测或估算；缺失不填0。
- [ ] RESULT-07 source/runtime/game/device/scene/method/reader身份贯通；一个数字能定位原始证据和计算方法。
- [ ] RESULT-08 失败、无收益、未执行、未知和恢复异常一等展示；不能只展正结果。
- [ ] RESEARCH-01 目标与方向持久状态、假设、优先级理由、前提、下一实验、结果和覆盖矩阵。
- [ ] RESEARCH-02 多方向依赖图与乐观并发更新；不同Agent工具可接续，不导入其私有聊天历史。
- [ ] RESEARCH-03 参考prompt/skill + 稳定工具：消费反馈、选择可行方向、提交、回收、更新证据和优先级。
- [ ] RESEARCH-04 事件游标/watch、客户端通知适配和无人可做时的可恢复空闲；不无意义轮询。
- [ ] RESEARCH-05 目标配置支持性能/画质/功耗权衡、原生基线和模块ms收益，保留Pareto结果而非全局固定一票否决。

## P1：异步人类协作与基础接管

- [ ] HUMAN-01 指导、数据/现场协助、主观评价三类持久请求；说明所需内容与依赖方向。
- [ ] HUMAN-02 网页收件箱/回答/历史/状态，CLI/API同读写；回答有ID、对象版本和工件身份。
- [ ] HUMAN-03 只阻断明确依赖的工作；反馈超时保持未知，不自动同意，不把所有方向置为等待。
- [ ] HUMAN-04 图片/视频A/B主观评价，记录偏好、差异程度与评论；支持盲化/随机左右和过期反馈识别。
- [ ] HUMAN-05 人的建议进入事件流，外部Agent记录采纳/暂缓理由；不静默覆盖预注册的旧实验方法。
- [ ] CTRL-01 将现有screen与基本点击/滑动/键输入接入node统一认证/API，网页设备页直接使用。
- [ ] CTRL-02 人工接管/归还与Agent共用lease；有测量时先安全停止/还原并标记该轮状态，其他设备继续。
- [ ] CTRL-03 截图带设备、方向、尺寸、时间和frame标识；动作绑定画面/坐标变换，避免点在陈旧截图。
- [ ] CTRL-04 明确操作回执：已发出、传输失败、后端拒绝、可观测确认；退出码不冒充游戏已响应。
- [ ] CTRL-05 无人观看停采、断线释放触点/按键、测量时观察干扰控制；网页可见当前控制者与模式。

## P2：更完整的远程手机能力

- [ ] CONTROL-01 通用pointer down/move/up、多指、长按、组合手势、持续摇杆/按键与时间轴动作；逐平台验收。
- [ ] CONTROL-02 Unicode/剪贴板输入、软键盘/旋转/前后台切换的能力探测与可靠反馈。
- [ ] CONTROL-03 低延迟视频流、录像与必要音频，按带宽/CPU预算调节；测量开销独立记录。
- [ ] CONTROL-04 观察、操作、测量三种会话模式；接管冲突与失联恢复可复现。
- [ ] CONTROL-05 应用生命周期、文件、日志、抓帧、进程/性能诊断提供一致的远程原语与产物。
- [ ] CONTROL-06 无root/Harmony/iOS显式降级；受系统安全、应用、物理交互限制的步骤生成局部协助请求。
- [ ] CONTROL-07 设备重新连接/重启/授权过期后的能力刷新与epoch切换，不沿用过期身份。

## P2：优化方法与测量易用性

- [ ] METHOD-01 探索/确认两套可复用模板；已验证前提按版本复用，必要时失效，不每轮重做全套资格。
- [ ] METHOD-02 GOL候选生成/编译/统计/确认/Pareto/模型工件暴露为可选工具，内置goal成为普通客户端。
- [ ] METHOD-03 合并重复结果解析与报告代码的数据合同；多个页面可保留不同视图，不重复解释指标。
- [ ] METHOD-04 refbench已知答案与接口回归；原生LR/HR能力的host测试和真机资格分开。
- [ ] METHOD-05 黑盒/灰盒/白盒优化recipe库：分辨率、LOD、阴影、AA、合批/缓存、资源布局、CPU提交、NPU/GPU分工、帧节奏、系统旋钮。
- [ ] METHOD-06 同状态数据集采集与清单：真实低清/合成低清、色域、相机/时间、运动/遮挡、训练/留出分组明确。
- [ ] METHOD-07 功耗/能耗/显示与软件呈现来源能力账本；不能从固定schema字段推断设备支持。
- [ ] METHOD-08 负结果查询与已否决方向记忆，避免换Agent后重复同一失败路线。
- [ ] METHOD-09 无法比较/测不出时自动建议可做的下一种实验，而不是把未知填0或要求人立即出现。

## P3：想到但尚未做的方向

- [ ] FUTURE-01 多机迁移/容灾：先证明源任务终止、环境与数据可接续；跨机A/B不合并成同一基线。
- [ ] FUTURE-02 离线节点缓存提交、重新上线同步事件与索引；冲突和重复执行语义明确。
- [ ] FUTURE-03 更细的角色/项目/设备范围权限、多用户审计、临时访问与撤销。
- [ ] FUTURE-04 可选对象存储/远端索引/备份适配；用户选定后再部署，不默认复制全部历史。
- [ ] FUTURE-05 节点安装/升级打包、跨OS发行、版本兼容矩阵、doctor一键修复可恢复问题。
- [ ] FUTURE-06 按实际使用成本决定合仓、裁撤重复页面/旧入口；不先做全语言重写。
- [ ] FUTURE-07 调度公平性、优先级、成本估算、队列可预期等待时间与资源利用率。
- [ ] FUTURE-08 数据集/模型工件注册、训练作业分发、实验复现环境与可选SBOM。
- [ ] FUTURE-09 人类主观评价的重复一致性、多人偏好与置信度；避免少量评价被包装成普遍质量结论。
- [ ] FUTURE-10 完整用户旅程演练：换一台工作站/换一个Agent，仅凭平台状态接续研究。

## 明确暂存

- [ ] HOLD-01 Doom cap60同期功率/能耗pilot、更多场景、GI/阴影/渲染命令方向：按本机交接恢复，不属于本轮平台实现。
- [ ] HOLD-02 mini1定位/修复、后台选址、原始数据迁移：用户已暂缓。
- [ ] HOLD-03 全阶段原始Doom库/prefs还原：仍是研究恢复义务，不能被单轮owned016还原替代。
- [x] HOLD-04 核查近期旧PR整合与当前批次冲突，具体问题见OLD-PR-AUDIT；后续修复分别跟踪MERGE条目。

## 每批完成时追加

- [x] LEASE-STORAGE-01 按 LEASE-STORAGE 统一 Rust、devlock、node/fleet、GOL 路径，显式 init 与旧域退休屏障；网页和 CLI 同状态。PF588b1c1、harness336eaaf、GOL29514d8已检查通过直推main；仅宿主验证，未部署。
- [x] LEASE-OBSERVE-01 按 OBSERVATION-OWNERSHIP.md 完成fleet/只读preflight/每帧screen同域强lease与有界worker；网页保留成功时间和暂缓原因。2026-10-23宿主/API/网页验收通过，PF da3d2f5、harness f5e101a、GOL d2d1840实际检查通过直推main。未部署/未真机，完整JOB-EXEC-03仍未完成。
- [ ] LEASE-STORAGE-02 后续：统一 hwcond 恢复状态目录，取消失败时临时回退；清点 raw adb/旧 carrier 脚本是否经过同一 ownership，普通旧 owner 超时接管与完整恢复义务分别处理。
  - [ ] LEASE-IDENTITY-01 为普通外层 devlock 队列记录实例身份，并让 nested 强 lease 显式引用它；只读状态准确关联同一 owner。当前 active gray control 的强 lease 和 bridge 可证实，但普通外层缺 marker，保留 unknown 诊断；不得用相同 label 推断归属。03B4C1 仅修正“已知恢复义务优先显示 blocked”，未放宽准入或 clear。
  - [x] LEASE-STORAGE-02A 按 [HWCOND-RECOVERY](HWCOND-RECOVERY.md) 统一持久路径、拒绝覆盖遗留快照、明确损坏/读取/恢复错误；修复bench恢复失败仍继续。28hwcond/3storage单测、6实际CLI/fake ADB通过；e06e1ff实际code-only成功并直推main。仅宿主验证，未部署；producer全局恢复收据另批处理。
  - [x] PRODUCER-RECOVERY-01 按 [BENCH-RECOVERY](BENCH-RECOVERY.md) 完成bench实际hold、本地命令门控、远端负载收尾、独立恢复证据及同context续接；2f5f11d检查通过后直推main。43相关单测、3 shell宿主fixture、9实际CLI/fakeADB通过，仅宿主验证。硬件写入未知和未完成部署保留屏障，不能自动complete；不是整个job或真机恢复验收。
  - [x] PRODUCER-RECOVERY-02 按 [CAPTURE-RECOVERY](CAPTURE-RECOVERY.md) 完成capture实际hold、worker组门控、完成记录、输入/原stayon/退出政策读回及同context恢复；bd67d97实际code-only成功后直推main。27相关unit、11实际CLI/fakeADB、64传输核验通过；未部署/未真机，不声称游戏状态恢复。
  - [ ] PRODUCER-RECOVERY-03 下一批gray-ab/eval：先冻结挂层、全局属性、文件、采集/输入、应用进程的恢复范围，移除字符串冒充完成及忽略off错误；复用已有lease/worker/收据，分清共享摘层合同与整job恢复。
    - [x] PRODUCER-RECOVERY-03A 按 [GRAY-RETIREMENT](GRAY-RETIREMENT.md) 修shared off typed限定目标停用及gray-ab/eval/frameaudit错误传播，a38afa9实际code-only成功；24相关unit、3条适用CLI/fakeADB证据通过。未部署，不声明整job恢复。
    - [ ] PRODUCER-RECOVERY-03B 完整gray producer session：首次写入hold、属性/安装范围、受控采样/输入进程、ftrace实例及停充回读，全部义务完成后才complete_recovery。
      - [x] PRODUCER-RECOVERY-03B1 按 [GRAY-CAPTURE-SESSION](GRAY-CAPTURE-SESSION.md) 完成frame-audit capture强lease、原属性/安装/历史文件恢复及recover；f966c53实际code-only成功直推main。17相关unit、9有效CLI/896 fake传输通过，仅宿主验证；父03B不完成。
      - [x] PRODUCER-RECOVERY-03B2 将storeop-check切换到同一受控会话，独立入口先接；随后eval借用父lease，只写子范围报告，不提前清除父屏障。保持既有画质比较和判据，合同见 [GRAY-STOREOP-SESSION](GRAY-STOREOP-SESSION.md)。
        - [x] PRODUCER-RECOVERY-03B2A 独立storeop-check复用受控Session和PNG证据；7a3d7d1检查成功直推main。20相关unit、4有效CLI/564 fake传输通过，无部署；eval legacy调用明确保留待迁。
        - [x] PRODUCER-RECOVERY-03B2B 按 [GRAY-EVAL-LAUNCH-SESSION](GRAY-EVAL-LAUNCH-SESSION.md) 合并完整gray_eval父会话的workload.launch路线（资格/3基线/ABBA/recover）；实际父lease/context、子范围只写verified，全部真实资源恢复后才父complete。9e1bb0a经实际code-only成功直推main，13唯一unit/3唯一worker/5有效CLI通过。gray路径不隐式新增停充/锁频；script任意exec/gamepad另批。
      - [x] PRODUCER-RECOVERY-03B2C 按 [GRAY-EVAL-SCRIPT-SESSION](GRAY-EVAL-SCRIPT-SESSION.md) 完成有限输入、3-child所有权、资格/3baseline/ABBA及同context恢复。默认12秒拖动保留，非法脚本提前拒绝，无自然输入收据不清屏障。2fb0b16实际code-only成功后直推main；10相关unit、3唯一worker和4有效CLI/1868门控fake传输通过，仅宿主验证。
      - [x] PRODUCER-RECOVERY-03B3 按 [GRAY-AB-LOADOP-SESSION](GRAY-AB-LOADOP-SESSION.md) 完成loadop完整会话：一次停充跨两臂、充电原值/记录绑定、3child采样、同context恢复；保留ctrl/knob顺序和passdump=0。73c0a8d检查成功直推main；8unit、1worker、3CLI/512门控fake传输通过，仅宿主验证。power协议明确40秒sampler，不能混旧t10短点列。
      - [ ] PRODUCER-RECOVERY-03B4 按实际native可达分支收齐灰层入口：未支持的none/copyprobe编排提前拒绝并保留历史disabled证据；继续清理无调用旧链，以及可执行gray-layer/旧脚本的实际ownership，不随生命周期迁移扩大注入功能。
        - [x] PRODUCER-RECOVERY-03B4A 按 [GRAY-MODE-ADMISSION](GRAY-MODE-ADMISSION.md) 提前拒绝未支持copyprobe/upop，覆盖gray-ab/gray-layer公共mount与3个旧shell入口；155506f检查成功直推main。6unit、3实际CLI零transport/锁调用、7shell fixture通过；没有部署/真机。保留历史/离线工具，loadop主体不变。
        - [x] PRODUCER-RECOVERY-03B4B 清理无调用的 storeop_check_legacy→reach_scene/launch_to_world_layered 及私有 raw 抓帧/摘层链；保留 whitebox 的 screencap_raw_bin 和像素分析。557158a 检查成功直推 main，3项既有相关测试通过。后续持久会话合同见 [GRAY-CONTROL-SESSION](GRAY-CONTROL-SESSION.md)，仅设计已完成。
        - [x] PRODUCER-RECOVERY-03B4C1 按 GRAY-CONTROL-SESSION 实现持续 owner、断线接回、有序请求/控制 epoch、看屏和有限输入、灰层证据、停止与同 context 恢复。PF 4ca316d、harness 2b8066d 检查成功直推 main；21 Rust/10 Python 相关用例、JS及网页通过，两条实际 PF + fake ADB + HTTP 链435次门控传输。保留初次失败原件，无部署/真机；整体 job 仍沿 fresh clear + 核查说明释放。
        - [ ] PRODUCER-RECOVERY-03B4C2 在同一 manual owner 内接入性能采样并暂停观察干扰，迁移仍可执行的旧 gray-layer/reference/quality 路径。保留旧实验语义和原件；完成前不关闭03B4，不将局部恢复收据自动当整个 job 完成。
          - [ ] PRODUCER-RECOVERY-03B4C2A 按 [GRAY-CONTROL-SAMPLING](GRAY-CONTROL-SAMPLING.md) 接入当前场景 sample：1–120秒、复用三child和纯sleep plan、每轮原件、完整资源父恢复、Web/API/CLI同批。不自动重启/触控/停充/改风扇频率，尚无真实设备性能结论。
          - [ ] PRODUCER-RECOVERY-03B4C2B 在同一采样资源合同内接显式有限输入、需停充时复用绑定context的ChargeSuspend；明确实际窗口与功耗/能耗资格。
          - [ ] PRODUCER-RECOVERY-03B4C2C 将quality begin/finish改为跨人工等待的同session客户端，不重启finish；保留原图与旧半分辨率预览差别。
          - [ ] PRODUCER-RECOVERY-03B4C2D 迁移reference、test_loadop四臂refbench和旧gray-layer挂层入口；保留无层基线、字符串extras、自报/自然退出。旧keep-prop不启动语义与C1不同，不直接别名；保留typed off旧遗留、status诊断和离线分析。
  - [ ] CAPTURE-POLICY-01 旧Genshin step未消费mode/auto_choice，ready-only未命中会巡航；明确参数行为并修复，不把navigate声称为不推进剧情。当前恢复批不改策略，也不运行真机。
  - [ ] RECOVERY-UX-01 为未知硬件写入/未完成部署补可审计的独立核验路径，统一通用恢复lease显示label（当前confirmation-reconcile）；不以改label或手动删除记录代替恢复证据。

记录变更/提交、网页与API验收、离线测试、实际设备验证、未覆盖边界、迁移/恢复结果，以及下一批具体问题。实现条目只有达到相应验收才勾选；原实验失败记录保持不变。
