# 开发待办

更新：2026-10-25。**勾选仅表示条目注明的范围已完成，不能外推整个平台已可用。** P0是第一批必要基础/已知缺陷；P1是第一条完整流程；P2/P3是后续增强或研究候选。网页与CLI/API对同一条目共同验收。

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
- [x] UI-02 将同一设计/交互规范推广到节点、游戏、队列和人工协助页面，保留真实空态/错误态。 节点、游戏、队列、结果与研究协助已沿同一规范，保留空态/错误/未决；各批真实隔离浏览器证据见相应合同，研究页390px已验。

- [ ] BASE-01 对照上游与本地分叉，列可回收提交/补丁、依赖、测试；保护phonefarm与GOL未提交修改。验收：每项有去向，不一把推历史分支。
- [x] BASE-02 回收job唯一锁名、raw adb/fetch_apk锁路由、fleet队列解析等既有增强；验证与现有强lease/bridge兼容。 旧增强及强/嵌套/实例bridge宿主链已交付，见LEASE-IDENTITY；逐设备去全局串行仍归JOB-03，未部署旧运行节点。
  - 唯一 job owner 与 raw 命令锁路由已发布 `harness-ops 2b3c741`；只读 fleet 查询/队列显示也已发布 `947a4b9`；后续LEASE-IDENTITY/STRONG-LEASE-IDENTITY已验证强lease集成，逐设备去global串行仍待JOB-03。
- [x] BASE-03 harness 12eda9a已回收并统一APK SHA缓存、partial验收/隔离、显式serial、取消隐式全权限/音量修改。旧fetch_apk现在要求size/serial/SHA并共用节点缓存；legacy混合锁/设备恢复仍另列，未声称真机安装验证。
- [ ] BASE-04 固定三仓源码、运行二进制、协议版本关系；CLI和网页都可查询，客户端遇不兼容时解释具体差异。
  - [x] BASE-RUNTIME-01 按[RUNTIME-IDENTITY](RUNTIME-IDENTITY.md)区分启动源码观察、配置程序缓存和job实际入口自报；Web/CLI同级，历史缺失保持未知，不用当前版本回填。PF e51b09f/GOLaedf95d/harness0e1b7eb已发布；125协议、两真实程序/三job链/390网页及七CI通过，完整范围见合同。
- [ ] BASE-05 分开安装代码、配置/凭据、运行数据、缓存；为旧目录提供读取/迁移适配，保持原始证据。
- [ ] BASE-06 统一node/device/study/direction/experiment/job/artifact/human-request的ID和状态草案；测试版本化读写与并发更新。
- [ ] BASE-07 统一结构化错误、退出码、可重试性、缺能力/缺权限/离线/忙/恢复阻断；CLI文本与JSON语义一致。 capture恢复宿主验证再次确认devlock前置拒绝只有stderr、无JSON；统一处理外层gate，不在每个producer各造格式。
  - [x] BASE-GATE-JSON-01 按[GATE-ERRORS](GATE-ERRORS.md)补六类外层拒绝 JSON、稳定 code 和本次 command_started:false；不把放行后失败或恢复未知包装成未开始。PF c37df6b已发布，9单元/8调用宿主链/code-only首过；无设备，父BASE-07仍开放。
- [ ] BUG-01 修复screen前端暂停/关页面仍在后台抓图；观察订阅和性能测量共享资源状态。无设备计数测试+后续真机开销检查。
  - 已发布订阅生命周期修复 `phonefarm 591889e`；本批 LEASE-OBSERVE-01 增加每帧互斥与异常屏障。远程认证接入已由REMOTE-OWNERSHIP完成；真机观察开销仍未核，完整条目保持开放。
- [x] BUG-02 修复服务重启后job取消失效（225cb5c）；独立worker持有命令进程组和实际子进程对象，flock确认所有权，不凭保存PID发信号。旧任务继续可读，无法确认所有权时明确不支持取消。
- [x] BUG-03 只读状态查询不再创建job；超时节点不能静默消失，返回完整列表和分项错误（harness-ops `947a4b9`，模拟协议与浏览器验收；未部署运行节点）。
- [x] BUG-04 修复原件只读授权缺口，覆盖元数据/日志/单文件/Range；验证访客不能开任务或访问配置凭据。
- [x] REMOTE-OWNERSHIP-01 PF1ce29a1/harness58b21dd已检查成功直推main：live每帧复用强observation，busy/unknown保留旧时间，去除raw截图/别名/电源/视频旁路；无人观看停新采，Web原seq/图龄与状态分开。实际PF/HTTP/CLI及DOM/390px隔离浏览器通过；未部署/真机，高帧率编码器留REMOTE-VIDEO-01。
- [x] REMOTE-OWNERSHIP-02 同批PF持久owner+guarded Transport、串行动作/持久屏障、固定ID与只读找回；release等待已started并取消未开始队列，unknown不重放。Web/API/pf-remote共用事实；启动退休保unknown不清lease。7个唯一Rust关键用例、103协议、实际正常/异常PF链与6项actual CI通过；首次两失败及两处QA恢复义务保留。限定close不声称游戏还原，见[合同与验收](REMOTE-OWNERSHIP-REPAIR.md)。
- [ ] REMOTE-VIDEO-01 在逐帧观察稳定后恢复可监管的高帧率观看；编码器需本地/远端身份、有限停止和未知恢复，不能用新设备请求绕过测量租约。
- [ ] REMOTE-ACTIONS-01 为shell、安装/卸载、push/pull/logcat、HDC分别补实际owner/有界输出/设备副作用与恢复合同；本批旧raw入口明确unsupported，保留功能需求，不用Android验收外推Harmony。
- [x] TEST-01 为harness-ops建立协议/故障回归：重复请求、同label、断网、重启、取消、缺原件、坏缓存、路径边界、auth。 指定故障已进入协议及隔离宿主回归，见JOB-QUEUE/EXECUTION、LEASE-IDENTITY、GAME-DISTRIBUTION、JOB-INPUTS/RESULT-INDEX；真实断电/设备和长集成时序另列。
- [x] TEST-02 直接push流程的CI覆盖当前节点协议回归（Python3.11/3.14）及页面脚本检查；构建成功与真机验收分开记录，新模块继续补必要覆盖。
- [ ] TEST-03 将长集成中的真实等待与快速回归分开；调查并行 confirmation_driver 的 deadline/heartbeat 偶发失败，不靠反复全套重跑掩盖。
- [x] MERGE-01 修复旧 PR 带入的 gray eval 循环资格阻断；未知 control 可采证但不能形成结论（`8dba439`）。
- [x] MERGE-02 同步普通/动态灰层构建输入与 GOL 消费端，补 npuprobe.cpp；NDK实编、旧工件与跨仓合同验证通过，已发布 phonefarm `6cd1ac5`、GOL `1a8601e`。
- [x] MERGE-03 按 [GRAY-MODE-ADMISSION](GRAY-MODE-ADMISSION.md) 完成已知copyprobe/upop CLI/公共mount/旧shell入口准入，155506f检查通过直推main。默认unsupported、自定义SO capability_unverified；构建/离线分析保留，未开放注入。自定义工件未来执行能力合同另定。
- [ ] DOC-01 同步各仓AGENTS/README/组织入口的外部Agent定位与直接commit/push规则，保留代码仓文档边界。
  - phonefarm、harness-ops、game_opt_loop、refbench 的 AGENTS/直接提交流程已同步；组织总入口与全部旧说明仍待梳理。

## P1：多机管理

FLEET-01/02的共享目录子项已发布，见[FLEET-DIRECTORY](FLEET-DIRECTORY.md)。完整设备身份/能力和跨节点执行仍开放；下一批按主线推进游戏分发，不重做已有队列/恢复链。JOB-SESSION-RECOVERY-02保留具体已知待办。

- [ ] FLEET-01 稳定node/device身份、显式登记/撤销/更新；域名、执行host和数据位置独立。
  - [x] FLEET-DEVICE-IDENTITY-01 按[DEVICE-IDENTITY](DEVICE-IDENTITY.md)交付PF权威device_id/alias登记、版本CAS与Web/CLI/API；PF1ce93b8/harnessae3c7e0已发布，126协议/3Rust/实际PF链/390网页与六项CI通过。JOB03必需前置；仍全域执行，不宣称物理核验。
  - [x] JOB-DEVICE-DOMAIN-01 按[LEASE-DOMAIN](LEASE-DOMAIN.md)已发布PF2dbed4c/GOL3160e41/harness7998fa7；127协议、22PF+1GOL、实际签名PF、390显示及七CI通过，Linux新域/旧恢复185/317替身传输。首CI稀疏检出构建失败修复留档；仍global，现有QA恢复屏障不迁移。
  - [x] JOB-DEVICE-LANES-01 按[DEVICE-LANES](DEVICE-LANES.md)已发布PF dd27045/harness af93a5c：显式灰层绑定、设备ID锁、局部恢复与WebCLI同级；实际宿主A/B并行及alias/global/unknown排他。21PF回归、130协议首轮129通过后3项窄修通过、实际601替身传输/622PID全退、390网页和六项required CI均已验；未部署/真机。
  - [x] FLEET-TARGET-01 [合同与验收](FLEET-TARGET.md)：harness548ca96五项actual CI成功后同SHA直推main。CLI/Web/API统一原node绑定，取消隐式后台/凭据fallback，legacy fleet不串token，部署restart同目标单次发送；110协议、实际HTTP/CLI与DOM/390浏览器通过。旧API无pin兼容边界明确，旧未绑定请求不自动续接。未部署/真机；legacy migrate缺SSH绑定暂unsupported，迁移仍留FUTURE-01。
- [x] FLEET-02 网页节点/设备列表、CLI list/status/details共用API；显示运行版本、最近见到、当前任务/队列/lease/恢复态。 共享目录展示已交付，见FLEET-DIRECTORY与FLEET-TARGET；运行二进制身份完善仍归BASE-04，展示不是跨节点执行或生产部署。
  - `/dash/fleet`与`pf-fleet --hub`已共用`/nodes`持久目录；显示所选远端身份、存储、设备/队列/lease/恢复快照及新鲜度。旧无hub模式兼容保留，跨节点任务代理与真实多机部署尚未完成。
- [x] FLEET-03 状态缓存、TTL、新鲜度与连接恢复；设备缺席不误报整个节点离线。 最近尝试/成功、TTL、失败保留快照、设备null/空列表和身份重连已交付两隔离节点验证；见FLEET-DIRECTORY。
  - [x] FLEET-DIRECTORY-01 harness 1cbc7b1（功能ff18e96）已直推main：共享目录登记/更新/停用、旧配置显式导入、只写凭据/身份绑定、离线保留、明确设备采样；固定PF profile环境已同步。76本地协议与两节点隔离HTTP、DOM和390px浏览器通过，最终5项actual CI全部success。首次Py3.11旧restart测试竞态保留，修为等待已保存running，定向1项通过。完整设备实例身份/跨节点执行和生产验收仍分开。
- [ ] FLEET-04 能力按设备实例/epoch记录，区分代码支持、离线验收、实机验收及条件；Android root/nonroot、Harmony、iOS分别呈现。
- [ ] FLEET-05 一条doctor路径检查连接、工具、签名、版本、磁盘、设备授权和权限，输出可执行修复项；不反复全量onboarding。
- [ ] FLEET-06 节点端自检与安全更新、原子替换、回滚版本；有实验运行时更新语义明确。
- [ ] FLEET-07 CPU编译/分析与手机实验资源分开，能力标签不混成设备存在证明。

## P1：游戏与任务分发

[游戏资源准备合同](GAME-DISTRIBUTION.md)首批已发布。当前节点可登记不可变版本，运行持久宿主准备，直接下载/导入并校验缓存，网页/CLI共用记录。旧APK增强已回收，不重下Doom大资源。流式上传、单APK新装、初始化与有限场景尝试均已发布；升级/存档替换、真实场景资格、大文件负载和配额继续开放。

- [x] DIST-PREPARE-01 harness 12eda9a：目录→宿主任务→SHA缓存/续传/隔离→Web/CLI找回；83本地协议31.169秒、实际隔离HTTP/worker/CLI、DOM与1440/390浏览器通过，五项actual CI成功后同SHA推main。当前缓存未知不被历史任务成功覆盖；只有宿主资源资格，未部署/手机/大文件验收。

- [x] DIST-01 定义游戏bundle manifest：应用/引擎/包/资源/存档/场景/配置/来源/哈希/设备条件/空间/回执。 manifest与受限install/deploy/initialize/scene recipe、空间及分项回执已定义并发布；存档覆盖/其他格式执行器仍归DIST-06/07。
- [x] DIST-02 网页游戏目录与准备状态，CLI inspect/prepare；分别显示已下载、已安装、资源就绪、等待登录、场景就绪。 目录、准备与各typed回执已Web/CLI同级显示；login/scene无证据保持unknown，实际场景资格与协助接线仍留DIST-03/08。
- [ ] DIST-03 复用已有Doom样本建立第一个recipe；不重新下载现有大资源，不在规划阶段触碰手机。
  - [x] DIST-SCENE-01 [合同与验收](GAME-SCENE.md)：PF cbc53e4/harness58efc7f六项actual CI成功后同SHA直推main。旧部署证据+两小种子、新运行根、有界诊断与共用恢复，Web/CLI/API齐备。104协议、3Rust、actualPF三链117传输/242PID、旧initialize两回归43传输/49PID及DOM/390浏览器首次通过；未知QA屏障保留。仅源码/隔离宿主范围，未部署/真机，真实demo原件与冷启消费仍未核。
- [x] DIST-04 节点直取、流式接收、断点续传、接收端hash、原子缓存与空间预检的源码流程已由12eda9a/02da7cd交付；下列DIST-LARGEFILE-01多GiB资源使用验收仍独立开放，不代表该负载已验。
  - [x] DIST-UPLOAD-01 [合同与验收](GAME-UPLOAD.md)：harness 02da7cd同一缓存分块接收/原ID接续/单资产worker校验，Web/CLI同批；本地88项协议及定向修复、DOM/实际浏览器替身通过，五项actual CI成功后同SHA直推main。未部署、未操作手机。
  - [ ] DIST-LARGEFILE-01 在有空间预算的隔离节点验证多GiB流式上传的内存/磁盘使用和断线接续；复用已授权素材或可控合成流，不在本Mac重复下载/复制旧Doom大文件。
- [ ] DIST-05 任务输入/输出改为显式工件引用和依赖清单；处理嵌套JSON/脚本路径，避免只有argv重写有效。
  - [x] DIST-INPUTS-01 [合同与验收](JOB-INPUTS.md)：harness dd00340经五项actual CI成功后同SHA直推main，保留维护者至553fb16全部提交。同节点已收尾小工件、独立JSON pointer派生、PF_JOB_IN/cwd及Web/CLI共用；原件保留，失败不启动/同ID不重放。94协议首轮93过，仅Mac测试路径别名修正，两项定向通过；DOM/390浏览器通过，系统剪贴板往返未核。无生产/手机/大素材复制，完整DIST-05继续开放。
- [ ] DIST-06 资源包/OBB/loose文件/存档按目标平台受限路径安装；保存旧存档与恢复收据，检验游戏UID可访问性。
  - [x] DIST-DEPLOY-01 [限定资源部署合同与验收](GAME-DEPLOYMENT.md)，PF f9ba7a6、harness ddeaef7：Android user0 已安装精确匹配单APK引擎，选定资源部署到新独占子树；强lease/持久恢复、Node固定任务、Web/CLI同等入口。90协议、实际PF/fake ADB三上下文、真实HTTP→worker→PF链、DOM/1440与390浏览器及六项actual CI通过，同SHA直推main。未知传输保留屏障/负例，未操作真机或生产、未覆盖旧游戏数据。
  - [x] DIST-INSTALL-01 [合同与验收](GAME-INSTALLATION.md)：PF baef293/harness fd1ef8f已发布；明确缺席单APK新装、真实aapt2元数据、-R禁止替换、owned session已知失败abandon、未知commit保屏障，Web/CLI共用。91协议、CLI/HTTP、实际PF/SDK+模拟ADB六场景、另actual HTTP→worker→PF链、DOM/1440与390浏览器及六项actual CI通过。缺工具不禁已有恢复。未部署/真机安装；已有升级、存档替换、应用存储初始化和场景仍未完成。
  - [x] DIST-INITIALIZE-01 [合同与验收](GAME-INITIALIZATION.md)：PF537a62f/harnesse20758b已检查通过同SHA直推main。限定Launcher一次尝试、目录已就绪不启动、未知启动/停止保屏障、目录未达标但已停转局部协助；共用Transport和Web/CLI。92协议、3Rust、实际PF六上下文100传输、既有三回归108传输、另Node/worker/PF25传输、DOM/390浏览器通过。仅源码/隔离宿主，未部署/真机/场景；四个QA未知上下文保留。
- [ ] DIST-07 split APK/XAPK/HAP等格式按实际需求追加适配，先列支持矩阵，不宣称单APK流程已覆盖。
- [ ] DIST-08 登录/授权/资源采集的人工任务挂到方向；完成后重新验证游戏状态，其他方向继续。
- [ ] DIST-09 缓存去重、配额、引用计数/保留策略、可审查清理；默认不删除唯一原始证据。

## P1：任务队列与执行

- [x] JOB-01 持久作业状态机，区分准备/运行/采集/恢复/终态与等待原因；外部Agent提交立即拿ID。 持久准备/执行/采集/保存终态与独立恢复事实、即时job/request ID已交付，见JOB-QUEUE/EXECUTION/DURABILITY；恢复子链/暂停/预算仍由各具体项继续。
- [x] JOB-02 请求幂等和重复提交去重（58fc0dc）；同ID同内容返回原job，不同内容409，CLI/API/网页可找回；刷新后重填保留原ID。旧客户端未提供ID时不具备跨请求去重。
- [ ] JOB-03 资源队列原子领取、设备lease联动、主机资源预算；多设备并发不被无关全局锁意外串行。
  - 58fc0dc已完成SQLite原子领取、声明资源排队及宿主并发上限；本批2b4747e接入宿主lease屏障、恢复任务和独立阻断容量；DEVICE-LANES已补显式灰层的原子设备所有权/并发及局部恢复；全producer、主机CPU/GPU/磁盘预算与真机仍按后续项推进。
- [ ] JOB-04 冻结作业输入、程序/版本/设备身份；日志与事件可增量续读，状态更新有revision。
- [x] JOB-05 网页队列、任务详情、准备进度、日志、取消/恢复与CLI/API同级交付；最后的失败恢复子链已由f692972及SESSION-RECOVERY-CHAIN验收收口。实际重启/设备与其他producer边界仍归JOB-06/10。
- [ ] JOB-06 掉线、重启、超时、用户取消、失去lease后执行明确收尾；恢复失败占用独立屏障，不用exit0掩盖。
- [ ] JOB-07 真正的取消与安全点暂停语义；非幂等步骤不能自动retry，重试使用新attempt并保留旧失败。
- [x] JOB-08 队列资源等待与研究局部依赖已分开；阻断设备不占满无关宿主名额，RESEARCH-STATE/03/04已验一条等人、另一离线方向继续及显式反馈接续。方向排序由外部Agent决定；多设备并行锁域仍归JOB-03。
- [ ] JOB-09 任务/研究预算、超时与磁盘上限；停止后可解释哪些动作发生、哪些没发生、哪些恢复待核。
- [ ] JOB-10 完成两节点故障注入验收，然后单列实际手机的准备、执行、还原验收。

- [x] JOB-EXEC-01 [独立 worker 与重启后取消](JOB-EXECUTION.md)，网页/CLI同一回执；真实 HTTP 服务进程重启与无害子进程验证通过，225cb5c已发布，未部署。
- [x] JOB-EXEC-02 [提交幂等、持久队列与原子领取](JOB-QUEUE.md)，58fc0dc已发布；44项本机测试、Linux双版本CI、网页提交/找回/撤销/释放验收通过，未部署。
- [ ] JOB-EXEC-03 lease/恢复屏障与执行收据：worker丢失或恢复未知时限制后续设备写入，明确脱离进程组后代与主机重启的处理边界。
  - 第一批[宿主屏障/恢复通道](JOB-RECOVERY.md)已实现：phonefarm cafdbba、harness-ops 2b4747e。47项协议、跨仓真实宿主锁/无害进程和浏览器验收通过；恢复worker丢失也保留资源，祖先链恢复不放行同级任务。未部署/未真机，完整条目继续保持未勾选。
  - 第二批job身份与scoped收据已实现：phonefarm dc35335、harness-ops 11b50d9。节点启动时传入显式身份，强lease与恢复接管输出有来源/范围的收据，CLI/网页按需读取；不自动解除整个job屏障。验证和发布进度见BATCHES。
  - LEASE-STORAGE-01 已统一持久lease域并提供显式迁移合同；LEASE-OBSERVE-01 补采样窗口所有权；LEASE-STORAGE-02A 已修hwcond恢复记录（e06e1ff）。bench/capture已补producer typed恢复证据及实际lease hold（2f5f11d/bd67d97）；gray-ab/eval后续已由03A/03B系列迁移；脱组后代已有限定Linux内核保证，真实主机重启及未适配producer仍待验收，不能把手动放行当还原证据。
  - producer覆盖现状：bench/capture及gray既定03A/03B4范围已保存typed证据/屏障，见GRAY-CONTROL-SESSION；其余未适配producer与真实恢复仍开放。不得从released/not_reported推断恢复，不让node猜实验产物格式。
  - 旧devlock心跳遗留管道问题已修复b986fcf：zsh内建等待，明确停止并回收自建心跳；5项隔离回归通过，取消保留锁，不把信号退出当还原。

- [x] JOB-FINISH-01 按[JOB-GROUP-COMPLETION](JOB-GROUP-COMPLETION.md)交付原进程组结束证据与资源保留，Web/CLI/维护页同事实；harness eb3e69d实际必需检查成功直推main。5既有测试、新增1条宿主双模式回归与定向DOM通过，8PID退出；只覆盖原进程组，无设备/部署。
- [x] JOB-DURABILITY-01 按[JOB-DURABILITY](JOB-DURABILITY.md)交付worker释放锁、关键记录同步后再SQL提交；失败保资源让出名额，retry-finalize不重放，取消/未执行事实由SQL保留。harness 8381c19实际必需CI成功后直推main；61项本地协议与定向Web通过。只关闭源码/隔离宿主范围，真实断电、producer数据与完整JOB-EXEC-03仍开放。
- [x] LEASE-IDENTITY-02 按[STRONG-LEASE-IDENTITY](STRONG-LEASE-IDENTITY.md)完成强lease宿主boot/start身份与Web事实展示；PF201719b/harness66a532e必需CI通过后直推main。11项lease、1项queue、3项命令测试和22条隔离宿主CLI通过；修复实际短命zombie边界并保留首次失败。仅完成源码/宿主范围，真实重启、设备恢复和完整JOB-EXEC-03仍开放。
- [ ] JOB-CONTAINMENT-01 按[JOB-CONTAINMENT](JOB-CONTAINMENT.md)明确setsid脱组后代的内核执行保证与跨OS边界；第一批策略/准入/回执/WebCLI已发布fb4dc56，63项本地协议、3组宿主与DOM通过。普通节点contained仍不可用；隔离Linux内核worker另完成01B，固定入口、跨UID与失联域回收已由02A/B1–3完成；public contained、真实跨boot及未适配producer仍开放，完整条目不勾。

- [x] JOB-CONTAINMENT-01B 独立特权worker的Linux cgroup v2后端已发布487e5fb，实际cgroup-kernel及必需CI全部成功后直推main。4本地worker回归与一条内核链通过，4PID全退；首次fixture类型错误与原件保留。普通节点contained继续不可用，只关闭隔离后端范围，合同见JOB-CONTAINMENT。
- [ ] JOB-CONTAINMENT-02 固定特权worker启动边界、跨UID实时工件/PF控制、worker丢失及主机重启后的域恢复；不得给HTTP服务或整个解释器提权代替此设计。
  - [x] JOB-CONTAINMENT-02A 固定root-owned入口/配置、runner锁FD交接、node-owned管理记录与完成后输出交还已发布97a8946。17本地相关回归及一条真实nonroot内核链通过，17PID全退；首次fixture祖先权限错误留原件。四项实际CI全成功后同SHA直推main，公共策略暂不启用、无部署。
  - [ ] JOB-CONTAINMENT-02B PF实时会话/设备lease跨UID权限，producer主动维护共享组及原子发布模式；失联/重启域恢复独立验证。
    - [x] JOB-CONTAINMENT-02B1 先按JOB-CONTAINMENT合同接通固定PF环境/持久锁域、worker创建scoped receipt与现有gray-control publisher跨UID访问；实际Linux PF+fake transport一条正常停止链验证，默认私有权限保持。全局battery恢复/任意producer/失联与重启恢复仍独立待验。 已发布PF b247ea2/harness914f711，实际Linux跨UID链通过，四次失败原件保留；public contained仍false。
    - [x] JOB-CONTAINMENT-02B2 共享供电恢复记录与node私有封存会话来源分开；Web/CLI/API typed异步恢复同上下文，先验原worker已域空/交还，不重新开放历史输出。PF f98f619/harness b1e4fab已通过实际CI后直推main；1轮battery失败→恢复链、幂等与原件保留通过，首轮替身失败保留。失联/跨boot及失败恢复子链接续仍分别待验。
    - [x] JOB-CONTAINMENT-02B3 源码/隔离Linux范围完成并发布harness c61dae9：固定入口按job ID异步核查失联域/交还输出、持久身份复用门禁，Web/CLI同合同。真实失联→普通核查→force回收→旧输出拒读通过，22PID全退；跨boot仅判定模型，物理重启/生产部署/设备恢复仍不在本项完成范围。
    - [x] JOB-SESSION-RECOVERY-02 按[SESSION-RECOVERY-CHAIN](SESSION-RECOVERY-CHAIN.md)接续灰层恢复：恢复作业自身失败并保留资源时，typed入口与网页可沿明确恢复子链继续接续原session；不能要求先清除未完成的设备屏障，也不能让新请求作为同级任务永远排在失败恢复任务后。保留每次attempt，复用已有recovery_for祖先借用规则，不重写失败历史。已发布f692972，4定向/119协议、Linux真实PF与390替身原ID接续通过；五项actual CI同SHA main，生产/真机/公共contained仍单列。

JOB-01/05/08已按上述源码与隔离范围交付；JOB-03/04/06/07/09/10仍分别缺并行资源域、身份贯通、真实重启/其他producer、暂停、预算或两执行节点验收。失败恢复子链已发布，不再作为未完成重复实现。M4的RESULT-03/RESEARCH-05已发布，BASE-07外层子项已发布，当前CTRL-03截图绑定，随后BASE-04实际程序身份；真实设备阶段另列。

## P1：结果发现、共享与研究状态

- [x] RESULT-01 已发布显式登记、全量分页及通用job/out来源，旧/失败/unknown任务同样可登记；源码/隔离范围，不代表生产部署已完成。
  - [x] RESULT-INDEX-01 [RESULT-INDEX](RESULT-INDEX.md)：harness78dcfbb五项actual required CI成功后同SHA直推main；112协议/HTTP/CLI/真实12,498B Doom/DOM390验收通过。同节点不可变登记、固定ID找回、分页索引、Web/CLI与Doom/GOL显式小JSON导入。旧/失败/unknown可见；大原件留job/out，字节匹配不授予实验资格。跨节点索引另批，不提前勾RESULT-02/03。
- [x] RESULT-02 已交付选定目录节点的结果检索，元数据与原件位置分离、原件回源；没有隐式复制/迁移。范围为源码与隔离节点，不含生产可达性或持久离线缓存。
  - [x] RESULT-FEDERATION-01 [RESULT-FEDERATION](RESULT-FEDERATION.md)：harness42b2605五项actual CI成功后同SHA直推main，114协议/两源HTTPCLI/DOM390通过；复用持久节点目录和显式目标，按选定节点检索原结果索引，逐项保留离线/错误与各自分页；Web/CLI深链源详情，凭据不进入链接、不集中复制原件。
- [x] RESULT-03 按[RESULT-COMPARE](RESULT-COMPARE.md)交付同节点显式候选矩阵e9d0e12；已有检索/分页/下钻加2–4列精确metric_index对照，允许同报告多condition、原值差/部分失败/描述缺口、Web导出与深链、CLI单GET。1定向/122协议/390真实HTTP及五项actual CI通过；不宣称科学资格或完整跨节点候选比较。跨节点详情/混合比较待RESULT-CROSS-COMPARE-01。
- [ ] RESULT-CROSS-COMPARE-01 复用已有远端索引和显式node身份，提供有界远端详情与候选比较；各节点基线/来源独立，不从同名指标自动拼同一实验。
- [ ] RESULT-04 只读分享可发现、查看和下载注册原件；路径/符号链接边界与凭据/日志脱敏回归。
- [x] RESULT-05 已交付明确Doom LR4-026与GOL optimize_report@1适配：原文本不可变，GOL全archive与父层失败保留，不重算统计或重跑实验；其他schema明确不支持。
- [x] RESULT-06 统一指标schema及Web/CLI已交付：模块/窗口/单位/电源轨/分母/ROI/全帧/方法与null分开；未核来源不授予科学有效性，producer全链身份仍留RESULT-07。
- [ ] METRIC-OWNERSHIP-01 将当前场景trace的渲染线程/ctx与目标进程身份绑定，独立验证帧语义；不能从Unity默认comm或字符串筛选推断游戏FPS。03B4C2A先支持显式render_comm并保留未验证标记，没有筛选时帧指标为null、raw trace仍保留。复用已有render ownership能力，不为每个游戏另造过滤器。
- [ ] ENERGY-WINDOW-01 复用trace parser导出完整GPU-submit分组对应的实际时间窗，对相同窗口电池积分后报告分组能耗估计；目标/显示语义未核时不能命名游戏每帧能耗。C2B2仅交capture窗口电池能量，energy_per_frame_mj保持null。
- [ ] RESULT-07 source/runtime/game/device/scene/method/reader身份贯通；一个数字能定位原始证据和计算方法。
- [x] RESULT-08 全部报告结论可筛选并同等展示，当前job恢复事实单列；旧/失败/unknown及GOL非kept记录验收通过。只覆盖索引展示，不替producer判定结果。
- [x] RESEARCH-01 目标与方向持久状态、假设、优先级理由、前提、下一实验、结果和覆盖矩阵。 fb9ecf1已交付单节点目标/方向、结果引用与覆盖声明；预算/优先级由外部Agent使用，不是硬限额或调度器。
  - [x] RESEARCH-STATE-01 [合同与验收](RESEARCH-STATE.md)：harness fb9ecf1五项actual required CI成功后同SHA main；116协议54.941s、两HTTP/CLI链1.881s首过。390实际流程分3context接续，5次JS/6次HTTP，浏览器重传仍1回执；两QA偏差及未实点分支如实保留。四类对象/CAS/历史/局部依赖/文本协助/事件与Web/CLI已交付，不新建调度器。
- [x] RESEARCH-02 多方向依赖图与乐观并发更新；不同Agent工具可接续，不导入其私有聊天历史。 fb9ecf1独立revision/CAS、同研究局部依赖核环与历史已交付；原operation及重启接续已验，无私有聊天导入。
- [x] RESEARCH-03 按[RESEARCH-HARNESS](RESEARCH-HARNESS.md)交付参考prompt/skill与入口纠正：消费反馈、选择可行方向、提交、回收、更新证据和优先级；复用已有工具，不内置Agent。已发布harness de01858/PF af96ae6，3技能校验及一次实际离线PF/HTTPCLI接续通过；首次夹具准备失败保留，唯一实际分析无设备调用，required CI同SHA main。
- [x] RESEARCH-04 按[RESEARCH-WATCH](RESEARCH-WATCH.md)交付研究元数据有界等待、绑定游标、Web显式监听/浏览位置及CLI JSON通知适配；无人可做时保存独立已处理位置，不无意义轮询。job/result事件与停止运行器的外部唤醒不由此流代替。已发布df7c4ec，2定向/121协议/390真实NodeResearch与五项actual CI同SHA main；界面隐藏为模拟，容量/源码边界见合同。
- [x] RESEARCH-05 按[RESEARCH-CRITERIA](RESEARCH-CRITERIA.md)交付版本化目标、预声明候选、原生基线/模块ms规则和条件性点值Pareto；复用RESULT-03，不以质量或未采功耗固定一票否决，不改原报告结论。已发布d4bc398，1定向/123协议、溢出修复后2受影响回归、390实际NodeResearch及五项CI通过；精确范围见合同。

## P1：异步人类协作与基础接管

- [x] HUMAN-01 指导、数据/现场协助、主观评价三类持久请求；说明所需内容与依赖方向。 fb9ecf1三类持久请求与反向依赖投影已交付；本项基础文本，图片视频量表仍HUMAN-04。
- [x] HUMAN-02 网页收件箱/回答/历史/状态，CLI/API同读写；回答有ID、对象版本和工件身份。 fb9ecf1研究页收件箱/回答/历史/状态与CLI/API同级，原版本及注册证据绑定保留；actor为自报标签而非实名身份。
- [x] HUMAN-03 只阻断明确依赖的工作；反馈超时保持未知，不自动同意，不把所有方向置为等待。 fb9ecf1已验A等人/B独立，回复、关闭或到期都不自动解阻；显式resolution与处理理由独立保存。
- [x] HUMAN-04 图片/视频A/B主观评价：HUMAN-EVALUATION-01已发布f7e1ee1，固定原件/随机一次、偏好/差异/评论和原上下文迟答；盲化仅界面隐藏标签，不宣称保密双盲，多人统计另列FUTURE-09。
  - [x] HUMAN-EVALUATION-01 已发布f7e1ee1，固定原件/排列、受限显式媒体、结构化反馈、界面隐藏标签及旧canonical兼容。118协议/2新HTTPCLI/实际390媒体与五项CI通过；首轮QA断言保留，仅续视频反馈，详见[合同与边界](HUMAN-EVALUATION.md)。
- [x] HUMAN-05 人的建议进入事件流，外部Agent记录采纳/暂缓理由；不静默覆盖预注册的旧实验方法。 fb9ecf1不可变反馈/事件与adopt/defer/reject理由已交付；新计划保留原research_revision，不改旧问题与旧方法。
- [x] CTRL-01 将现有screen与基本点击/滑动/键输入接入node统一认证/API，网页设备页直接使用。 Android首批有限screen/live/tap/swipe/text/key/start/stop已统一operator认证与PF owner，见REMOTE-OWNERSHIP-REPAIR；高帧率/任意raw/HDC另列。
- [ ] CTRL-02 人工接管/归还与Agent共用lease；有测量时先安全停止/还原并标记该轮状态，其他设备继续。
- [x] CTRL-03 按[CONTROL-FRAME](CONTROL-FRAME.md)补普通owner截图设备/方向/尺寸/采集时间/frame、原坐标绑定、单次消费与执行前几何校验；灰层已有能力复用，不重做接管。 PF a746d73/harness574c94b已发布，截图绑定/原坐标/单次消费/原ID回执及Web/CLI已验，见[CONTROL-FRAME](CONTROL-FRAME.md)；仅源码/隔离宿主范围，真机仍单列。
- [x] CTRL-04 明确操作回执：已发出、传输失败、后端拒绝、可观测确认；退出码不冒充游戏已响应。 有限动作的accepted/started/completed/failed/unknown/not_executed和原ID找回已交付；completed只证明声明范围的命令/采集完成，应用响应未知不冒充确认。
- [ ] CTRL-05 无人观看停采、断线释放触点/按键、测量时观察干扰控制；网页可见当前控制者与模式。

- [x] CTRL-06 已发布harness114d867，按GRAY-REFERENCE-CLIENT合同补网页reference流程与CLI整组体验对齐：显式passdump/几何/供电期望、同组label与epoch、重复/partial原ID接续；共享平台回执，不做浏览器私有ledger。整组门禁与旧回执重连已通过定向JS/390浅1440深UI替身，未部署；页面隐藏/离开停止后续提交，不是持久工作流worker。

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
  - [x] LEASE-IDENTITY-01 按[LEASE-IDENTITY](LEASE-IDENTITY.md)完成PF/devlock/GOL实例与宿主进程身份、nested显式引用和Web事实显示；8dce700/30a0921/cc47f0e必需检查成功后直推main。5PF单测、3脚本、GOL Guard和18宿主命令/4身份拒绝通过；无真实设备或部署。旧/损坏记录不自动删除，完整JOB-EXEC-03保持未完成。
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
      - [x] PRODUCER-RECOVERY-03B4 源码与既定宿主/Web范围已交付（未部署/真机）；按实际native可达分支收齐灰层入口：未支持的none/copyprobe编排提前拒绝并保留历史disabled证据；继续清理无调用旧链，以及可执行gray-layer/旧脚本的实际ownership，不随生命周期迁移扩大注入功能。
        - [x] PRODUCER-RECOVERY-03B4A 按 [GRAY-MODE-ADMISSION](GRAY-MODE-ADMISSION.md) 提前拒绝未支持copyprobe/upop，覆盖gray-ab/gray-layer公共mount与3个旧shell入口；155506f检查成功直推main。6unit、3实际CLI零transport/锁调用、7shell fixture通过；没有部署/真机。保留历史/离线工具，loadop主体不变。
        - [x] PRODUCER-RECOVERY-03B4B 清理无调用的 storeop_check_legacy→reach_scene/launch_to_world_layered 及私有 raw 抓帧/摘层链；保留 whitebox 的 screencap_raw_bin 和像素分析。557158a 检查成功直推 main，3项既有相关测试通过。后续持久会话合同见 [GRAY-CONTROL-SESSION](GRAY-CONTROL-SESSION.md)，仅设计已完成。
        - [x] PRODUCER-RECOVERY-03B4C1 按 GRAY-CONTROL-SESSION 实现持续 owner、断线接回、有序请求/控制 epoch、看屏和有限输入、灰层证据、停止与同 context 恢复。PF 4ca316d、harness 2b8066d 检查成功直推 main；21 Rust/10 Python 相关用例、JS及网页通过，两条实际 PF + fake ADB + HTTP 链435次门控传输。保留初次失败原件，无部署/真机；整体 job 仍沿 fresh clear + 核查说明释放。
        - [x] PRODUCER-RECOVERY-03B4C2 源码与既定宿主/Web范围已交付（未部署/真机）；在同一 manual owner 内接入性能采样并暂停观察干扰，迁移仍可执行的旧 gray-layer/reference/quality 路径。保留旧实验语义和原件；完成前不关闭03B4，不将局部恢复收据自动当整个 job 完成。
          - [x] PRODUCER-RECOVERY-03B4C2A 按 [GRAY-CONTROL-SAMPLING](GRAY-CONTROL-SAMPLING.md) 接入当前场景sample，1–120秒、每轮sleep plan/三child、完整父恢复、Web/API/CLI及重连缓存结果。PF618e456、harness cd62842经实际检查成功直推main；4 Rust、1受影响worker、1 CLI、JS/浏览器及两条606传输的宿主HTTP链通过。显式render_comm，无筛选帧指标null；无部署/真机/性能收益结论。
          - [x] PRODUCER-RECOVERY-03B4C2B 有限输入和固定供电政策已按B1/B2交付；charge绑定/恢复、同期trace能量资格已验。每帧能耗独立留ENERGY-WINDOW-01；C2C/D旧客户端迁移也已交付。
            - [x] PRODUCER-RECOVERY-03B4C2B1 显式input+frame_id、逐round计划、部署后真实图/方向/图龄及完整预算复核；sample capability2与父@2隔离旧owner，兼容C1/C2A。PF65de801、harness e0818b4检查成功直推main；5Rust、1CLI、JS/网页及两链493次fake传输通过。缺natural负例按预期保QA隔离lease，全部已知PID退出。无部署/真机。
            - [x] PRODUCER-RECOVERY-03B4C2B2 按 [GRAY-CONTROL-POWER](GRAY-CONTROL-POWER.md) 交付创建时observe/battery、绑定ChargeSuspend/恢复与同期积分估计。PF86edb56、harness d4d8428实际检查成功直推；6Rust/2Python、定向JS/网页、3会话+1短CLI共830次fake传输通过，缺natural保1个QA隔离lease。无真机/部署，每帧能耗仍待完整分组窗。
          - [x] PRODUCER-RECOVERY-03B4C2C 已按C1/C2交付，将quality begin/finish改为跨人工等待的同session客户端，不重启finish；保留原图与旧半分辨率预览差别。
            - [x] PRODUCER-RECOVERY-03B4C2C1 源码、宿主替身与Web/CLI验证后已发布，按 [GRAY-CONTROL-QUALITY](GRAY-CONTROL-QUALITY.md) 接owner内begin/checkpoint/finish共享manifest、完整原图/半尺寸预览、Web/CLI与跨epoch续接；不自动断言同状态。
            - [x] PRODUCER-RECOVERY-03B4C2C2 已发布并通过客户端验收，迁移旧genshin_quality为既有session客户端，自动导航仅用显式尺寸/方向profile；人工等待后新图输入，无raw adb/off/restart。C2C1完成不代表旧入口已改。
          - [x] PRODUCER-RECOVERY-03B4C2D 源码与既定宿主/Web范围已交付（未部署/真机）；迁移reference、test_loadop四臂refbench和旧gray-layer挂层入口；保留无层基线、字符串extras、自报/自然退出。旧keep-prop不启动语义与C1不同，不直接别名；保留typed off旧遗留、status诊断和离线分析。
            - [x] PRODUCER-RECOVERY-03B4C2D1 已发布bf56025并通过6case纯客户端验收，按 [GRAY-REFERENCE-CLIENT](GRAY-REFERENCE-CLIENT.md) 迁移REF_ARM=1 passdump参考客户端，复用既有会话与共享CLI组件；显式几何/供电政策/有限输入，不声称无层或同状态。
            - [x] PRODUCER-RECOVERY-03B4C2D2 PF4282d09与harness53bb414已检查成功直推main，三链宿主及网页模板定向检查通过；按[GRAY-REFBENCH-SESSION](GRAY-REFBENCH-SESSION.md)将test_loadop四臂迁入单Lease+现有Session，真实无层A/B后再安装C/D，保留字符串extras、八原件、自报与自然退出。
            - [x] PRODUCER-RECOVERY-03B4C2D3 PF f7b4012已检查成功直推main，4相关单测与11零设备拦截用例各一次通过；按[GRAY-LEGACY-MOUNT-RETIREMENT](GRAY-LEGACY-MOUNT-RETIREMENT.md)退役旧六挂层模式并移除裸写私链；keep-prop仅安装不启动不能直接alias。保留typed off/status/离线分析，shell off也改typed转发。
  - [ ] CAPTURE-POLICY-01 旧Genshin step未消费mode/auto_choice，ready-only未命中会巡航；明确参数行为并修复，不把navigate声称为不推进剧情。当前恢复批不改策略，也不运行真机。
  - [ ] RECOVERY-UX-01 为未知硬件写入/未完成部署补可审计的独立核验路径，统一通用恢复lease显示label（当前confirmation-reconcile）；不以改label或手动删除记录代替恢复证据。

记录变更/提交、网页与API验收、离线测试、实际设备验证、未覆盖边界、迁移/恢复结果，以及下一批具体问题。实现条目只有达到相应验收才勾选；原实验失败记录保持不变。
