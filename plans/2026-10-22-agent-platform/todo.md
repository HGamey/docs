# 开发待办

2026-10-26本批交付后，用户要求暂停自动推进，并同步rhodes1-lan供远端接续。整体项目未完成；后续仍按下方未完成项推进，DEPLOY保留实际验收边界。

更新：2026-10-26。**勾选仅表示条目注明的范围已完成，不能外推整个平台已可用。** P0是第一批必要基础/已知缺陷；P1是第一条完整流程；P2/P3是后续增强或研究候选。网页与CLI/API对同一条目共同验收。

## 先看这里：已交付、正在做、后续工作

当前发布组合：**phonefarm 7aa39c5 / harness-ops be8279c / game_opt_loop 4bbc7c3**，均已推进stable；运行节点部署状态另列。精确SHA见[stable-current](stable-current.json)。

| 状态 | 内容 |
|---|---|
| 已交付 | 节点/设备目录、游戏分发、持久队列、结果索引、研究状态、异步人类反馈；网页与CLI/API共用平台状态。 |
| 最近已交付 | 游戏/远控/看屏按设备并行，宿主计算槽位展示，远程日志与诊断，十条优化路线模板，缺项后的实验建议，作业优先级。 |
| 本批已交付 | JOB-EXPORT-CACHE-GC-01：单任务导出缓存查看/清理，Web/CLI/API同级，必需CI通过，已main/stable。 |
| 当前安排 | 暂停自动开发；将代码、计划、私有QA与交接同步rhodes1-lan，远端按现有TODO接续。源码同步不代表运行服务升级。 |
| 后续开发 | 多点/持续输入与视频、更完整的平台能力与重连、剩余producer恢复、资源保留/配额、数据集与可选GOL方法工具。 |
| 单列待验 | 生产部署、真机和真实多节点流程；见文末“部署验收”。这些未验项目不把已交付功能改回未完成。 |
| 暂存 | Doom新增性能/功耗实验、后台选址与原始数据迁移。 |

阅读约定：下方**未勾选父项可能已有已完成子项**，父项描述列出剩余范围；具体交付以子项和提交记录为准。条目包含父子关系，不把勾选比例当作总体工程完成度。各批简要变化见[BATCHES](BATCHES.md)。

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

- [x] BASE-01 对照上游与本地分叉，列可回收提交/补丁、依赖、测试；保护phonefarm与GOL未提交修改。验收：每项有去向，不一把推历史分支。
  - [去向表](BASE-RECLAMATION.md)：PF五旧提交、三工作树补丁、GOL两tracked补丁/一个研究helper、harness三提交七文件、docs27文件逐项闭合。PF a03a6f9/GOL761226c回收真实遗漏，两个通用helper明确研究保留；原树未动，无整支覆盖。
- [x] BASE-02 回收job唯一锁名、raw adb/fetch_apk锁路由、fleet队列解析等既有增强；验证与现有强lease/bridge兼容。 旧增强及强/嵌套/实例bridge宿主链已交付，见LEASE-IDENTITY；逐设备去全局串行仍归JOB-03，未部署旧运行节点。
  - 唯一 job owner 与 raw 命令锁路由已发布 `harness-ops 2b3c741`；只读 fleet 查询/队列显示也已发布 `947a4b9`；后续LEASE-IDENTITY/STRONG-LEASE-IDENTITY已验证强lease集成，逐设备去global串行仍待JOB-03。
- [x] BASE-03 harness 12eda9a已回收并统一APK SHA缓存、partial验收/隔离、显式serial、取消隐式全权限/音量修改。旧fetch_apk现在要求size/serial/SHA并共用节点缓存；legacy混合锁/设备恢复仍另列，未声称真机安装验证。
- [ ] BASE-04 固定三仓源码、运行二进制、协议版本关系；源码/实际程序身份已由BASE-RUNTIME-01交付，剩余为真实跨仓业务协议的需求、实报与具体不匹配解释，复用现能力准入及Web/CLI。 2026-10-26 rhodes1 实报（qa/base04-live-trio-20261026.md）：发现部署产物构建出处 null（HGAMEY_BUILD_* 未设置），带声明重建 7aa39c5/4bbc7c3 并重新部署后，运行节点实报 PF gray-control-gateway@1 compatible、GOL program-start@1、dirty=false，宿主范围无不匹配；设备业务协议实报仍待 DEPLOY-04。
  - [x] BASE-BUSINESS-COMPAT-01 按[BUSINESS-COMPATIBILITY](BUSINESS-COMPATIBILITY.md)已发布PF316a3e0/harness69b9b43：唯一配置程序缓存、灰层新建门禁、明确差异和Web/CLI；147协议/actualHTTPCLI/PF runtime-info/390/六CI全首过。旧原ID/停止/恢复不挡，声明不授予owner动作。
  - [x] BASE-RUNTIME-01 按[RUNTIME-IDENTITY](RUNTIME-IDENTITY.md)区分启动源码观察、配置程序缓存和job实际入口自报；Web/CLI同级，历史缺失保持未知，不用当前版本回填。PF e51b09f/GOLaedf95d/harness0e1b7eb已发布；125协议、两真实程序/三job链/390网页及七CI通过，完整范围见合同。
- [ ] BASE-05 首批外置安装参数/配置/数据/导出缓存及旧jobs只读来源已由下列四子项交付；真实旧节点接续/迁移、全目录保留与清理仍未验，保持原始证据，按DEPLOY-01/03、FLEET-06、DIST-09继续。
  - [x] BASE-LAUNCH-01 按[NODE-LAUNCH](NODE-LAUNCH.md)已发布d0d9eb9：外置配置、冻结参数、更新后同根重启；149协议首过与五CI成功，旧数据不搬、固定worker不自动重装。
  - [x] BASE-CONFIG-01 按[NODE-CONFIG-PATH](NODE-CONFIG-PATH.md)已发布70b51ef：上传凭据目录与HG_CONFIG_DIR一致，固定profile过滤环境的真实worker链/默认/失败保输入通过；150协议首过与五CI成功，无真实凭据变更。
  - [x] BASE-HISTORY-01 按[JOB-HISTORY](JOB-HISTORY.md)已发布92716a0，旧_jobs显式只读来源及Web/CLI同级；152协议首过、唯一390实际HTTP与五CI通过。来源revision隔离，不写旧根、不推断PID；首次QA下载事件计数错误与补证保留。
  - [x] BASE-CACHE-01 按[EXPORT-CACHE-ROOT](EXPORT-CACHE-ROOT.md)已发布d5dfa3f，可选外置目录导出缓存；153协议首轮两旧断言修后3定向/唯一390/五CI均通过，原失败保留。不搬资产partial/唯一原件/含凭据状态，累计保留/配额仍归DIST-09/JOB-09。
- [x] BASE-06 按[共享身份与状态](SHARED-ID-STATE.md)归并已有node/device/research/direction/job/result/协助身份、revision/CAS/幂等与状态证据。2026-10-26只读复核已有并发/重启验收；实验目前由冻结direction revision中的计划定位，无独立experiment实体/跨库绑定。设备物理epoch仍归FLEET-04/CONTROL-07，实际部署另验。
- [ ] BASE-07 统一结构化错误、退出码、可重试性、缺能力/缺权限/离线/忙/恢复阻断；CLI文本与JSON语义一致。 旧devlock前置JSON拒绝已由BASE-GATE-JSON-01修复；其余producer与跨入口错误一致性仍待收口，不逐producer另造格式。
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
  - [x] REMOTE-LOGCAT-01 复用现有远程owner读取有限条Android logcat，Web/CLI同级，明确输出截断；不新开设备会话、不清日志。PFceaa0cd/harnessfa3159e已main/stable；最小相关检查及必需CI通过，另修CI揭露的停止回执权限归还竞态。
- [x] TEST-01 为harness-ops建立协议/故障回归：重复请求、同label、断网、重启、取消、缺原件、坏缓存、路径边界、auth。 指定故障已进入协议及隔离宿主回归，见JOB-QUEUE/EXECUTION、LEASE-IDENTITY、GAME-DISTRIBUTION、JOB-INPUTS/RESULT-INDEX；真实断电/设备和长集成时序另列。
- [x] TEST-02 直接push流程的CI覆盖当前节点协议回归（Python3.11/3.14）及页面脚本检查；构建成功与真机验收分开记录，新模块继续补必要覆盖。
- [x] TEST-03 [显式宿主回归入口与取证](TEST-HOST-LANES.md)已发布3494bf2：fast/driver/单integration入口、fixture admission、失败原件与单调耗时；3-thread定向3/3及runner冒烟过，必需CI同SHA main。旧失败具体底层延迟因原件曾删除无法追溯，不宣称根因已完全证明；生产阈值和默认覆盖不变。
- [x] MERGE-01 修复旧 PR 带入的 gray eval 循环资格阻断；未知 control 可采证但不能形成结论（`8dba439`）。
- [x] MERGE-02 同步普通/动态灰层构建输入与 GOL 消费端，补 npuprobe.cpp；NDK实编、旧工件与跨仓合同验证通过，已发布 phonefarm `6cd1ac5`、GOL `1a8601e`。
- [x] MERGE-03 按 [GRAY-MODE-ADMISSION](GRAY-MODE-ADMISSION.md) 完成已知copyprobe/upop CLI/公共mount/旧shell入口准入，155506f检查通过直推main。默认unsupported、自定义SO capability_unverified；构建/离线分析保留，未开放注入。自定义工件未来执行能力合同另定。
- [x] DOC-01 同步各仓AGENTS/README/组织入口的外部Agent定位与直接commit/push规则，保留代码仓文档边界。
  - [DOC-ENTRY-ALIGNMENT](DOC-ENTRY-ALIGNMENT.md)：组织、四活跃代码仓与docs入口已发布；历史/归档材料保留原范围，不全面重写旧报告。各仓实际必需检查与main核对完成。

## P1：多机管理

FLEET-01/02的人工登记、稳定身份与位置查询已发布，见[FLEET-DIRECTORY](FLEET-DIRECTORY.md)和[DEVICE-IDENTITY](DEVICE-IDENTITY.md)。设备物理身份/epoch能力核验仍归FLEET-04，跨节点执行及真机验收归JOB-10；失败恢复子链已由SESSION-RECOVERY-CHAIN交付，不再重复实现。

- [x] FLEET-01 稳定node/device身份、显式登记/撤销/更新；域名、执行host和数据位置独立。FLEET-DIRECTORY与DEVICE-IDENTITY已覆盖人工声明登记、alias/CAS/历史和Web/CLI；撤销不取消远端任务，登记不证明物理同一性或实机能力。生产登记/设备核验仍独立。
  - [x] FLEET-DEVICE-IDENTITY-01 按[DEVICE-IDENTITY](DEVICE-IDENTITY.md)交付PF权威device_id/alias登记、版本CAS与Web/CLI/API；PF1ce93b8/harnessae3c7e0已发布，126协议/3Rust/实际PF链/390网页与六项CI通过。JOB03必需前置；仍全域执行，不宣称物理核验。
  - [x] JOB-DEVICE-DOMAIN-01 按[LEASE-DOMAIN](LEASE-DOMAIN.md)已发布PF2dbed4c/GOL3160e41/harness7998fa7；127协议、22PF+1GOL、实际签名PF、390显示及七CI通过，Linux新域/旧恢复185/317替身传输。首CI稀疏检出构建失败修复留档；仍global，现有QA恢复屏障不迁移。
  - [x] JOB-DEVICE-LANES-01 按[DEVICE-LANES](DEVICE-LANES.md)已发布PF dd27045/harness af93a5c：显式灰层绑定、设备ID锁、局部恢复与WebCLI同级；实际宿主A/B并行及alias/global/unknown排他。21PF回归、130协议首轮129通过后3项窄修通过、实际601替身传输/622PID全退、390网页和六项required CI均已验；未部署/真机。
  - [x] FLEET-TARGET-01 [合同与验收](FLEET-TARGET.md)：harness548ca96五项actual CI成功后同SHA直推main。CLI/Web/API统一原node绑定，取消隐式后台/凭据fallback，legacy fleet不串token，部署restart同目标单次发送；110协议、实际HTTP/CLI与DOM/390浏览器通过。旧API无pin兼容边界明确，旧未绑定请求不自动续接。未部署/真机；legacy migrate缺SSH绑定暂unsupported，迁移仍留FUTURE-01。
- [x] FLEET-02 网页节点/设备列表、CLI list/status/details共用API；显示运行版本、最近见到、当前任务/队列/lease/恢复态。 共享目录展示已交付，见FLEET-DIRECTORY与FLEET-TARGET；运行二进制身份完善仍归BASE-04，展示不是跨节点执行或生产部署。
  - `/dash/fleet`与`pf-fleet --hub`已共用`/nodes`持久目录；显示所选远端身份、存储、设备/队列/lease/恢复快照及新鲜度。旧无hub模式兼容保留，跨节点任务代理与真实多机部署尚未完成。
- [x] FLEET-03 状态缓存、TTL、新鲜度与连接恢复；设备缺席不误报整个节点离线。 最近尝试/成功、TTL、失败保留快照、设备null/空列表和身份重连已交付两隔离节点验证；见FLEET-DIRECTORY。
  - [x] FLEET-DIRECTORY-01 harness 1cbc7b1（功能ff18e96）已直推main：共享目录登记/更新/停用、旧配置显式导入、只写凭据/身份绑定、离线保留、明确设备采样；固定PF profile环境已同步。76本地协议与两节点隔离HTTP、DOM和390px浏览器通过，最终5项actual CI全部success。首次Py3.11旧restart测试竞态保留，修为等待已保存running，定向1项通过。完整设备实例身份/跨节点执行和生产验收仍分开。
- [ ] FLEET-04 能力按设备实例/epoch记录，区分代码支持、离线验收、实机验收及条件；Android root/nonroot、Harmony、iOS分别呈现。
- [ ] FLEET-05 宿主doctor与单次Android远控诊断已交付；剩余：将设备诊断汇入统一doctor，并补各平台授权/权限问题的可执行修复建议。 2026-10-26 harness 22de4eb：doctor 新增 devices.lease（缓存快照区分 clear/held/blocked/unknown 及对应建议）与分平台授权修复项（Android USB 调试授权/Harmony HDC/只读登记入口）；不执行程序不探测设备。真机侧诊断汇入与实测授权核验仍待设备工作流。
  - [x] REMOTE-DIAGNOSTICS-01 [单次Android诊断](REMOTE-DIAGNOSTICS.md)已随PF7aa39c5/harness33b7aa8发布main/stable，复用现owner和有界回执，Web/CLI同级。最小检查及必需CI通过，不推断root或场景资格。
  - [x] FLEET-DOCTOR-01 [宿主诊断合同](NODE-DOCTOR.md)：API/CLI/网页同等入口，缓存身份、工具、签名、磁盘与权限；不探测设备。f78554a已发布，158协议首轮一个清理错误修后定向过、五CI成功；实际HTTP/CLI及合成UI通过，实际后端浏览器接线因工具不可用未验，父项设备诊断仍开放。
- [ ] FLEET-06 自检、受管更新/回滚、维护网页及运行期准入已交付；剩余：跨参与者更新协调与生产验证。
  - [x] FLEET-OVERLAY-01 文件级 overlay 原子部署：harness 5ef8b79经code-only+node-tests CI后同SHA直推main。管理员工具 node/deploy_overlay.py：release 全量快照+harness-ops 符号链接游标单步 os.replace 切换（运行服务只见完整旧版或完整新版）；activate 强制 py_compile 冒烟，--check-only 非变异；首次采用把既有目录整体转为 baseline release 保证回退有源；rollback 默认沿 receipts previous；保留最近 5 个 release。不动 jobs/token/其他组件，不重启服务。3 项 subprocess 回归；rhodes1 实机已采用 overlay 布局（baseline+当前 release），重启后同 node_id/jobs 根/历史保留，browser DOM 全链复验 problems=[]。跨参与者协调与生产验证仍开放。 实机回滚演练发现 adoption 布局缺一层 harness-ops/ 致 rollback 404：e0772a8 修为标准 release 布局（逐条目移入），实机双向演练（current→baseline→服务正常→恢复 current，同 node_id/jobs 根/历史）通过；回滚链断言补至测试。 实机又暴露两处协调缺口并修复：d6d7041 doctor 增 deploy.overlay 游标健康检查（ok/悬空error/旧布局warning）；b17f351 修安装根推断（release 子路径向上找 releases+deploy-state）；3274f2d 修 node_launch resolve() 把服务入口钉死旧 release 的缺陷——此后服务普通重启即跟随游标（launcher 自身代码更新仍需整重启一次）。~/node 现运行 112909 release（=main），doctor ok16/warning2/error0。 跨参与者协调第一批：5b101b5 activate 新增 --jobs-root 准入（同 .intake.lock 非阻塞取锁 + 只读扫描 claimed/preparing，409 拒绝 / --force 覆盖，准入事实入 receipt）；实机验证——GOL serve 运行中拒绝、取消后 running:0 干净切换。
  - [x] FLEET-UPDATE-01 [宿主安全更新合同](NODE-UPDATE.md)：固定安装路径、先stage/sign、原子文件替换、持久维护屏障与明确回滚。f78554a已发布，真实隔离worker发布/阻断/回滚链、失败取证及五CI通过；父项生产/跨参与者更新仍开放。
  - [x] FLEET-UPDATE-WEB-01 按[NODE-UPDATES-WEB](NODE-UPDATES-WEB.md)补专用网页更新/回滚与只读维护状态入口，复用原作业/请求ID和掌舵权限，不自动重启；实现随f5a7ea4发布，五CI与实际隔离HTTP/CLI通过；浏览器布局、生产/手机仍未验。
- [x] FLEET-07 宿主程序/CPU/GPU/IO声明槽位与手机快照分开，Web/CLI补齐容量展示；harness309d842已main/stable。声明槽位不证明物理GPU或手机在线。
  - [x] FLEET-HOST-RESOURCES-01 修复显式device_access=none的PF/GOL宿主作业仍默认申请legacy-device；新请求不占设备资源，旧request完整内容匹配时找回原row且不改历史。Web/CLI沿同一服务默认；显式resources、写设备与恢复继承保持原语义。f5a7ea4已发布；实际隔离HTTP/CLI、最终五CI通过，详见[JOB-HOST-RESOURCES](JOB-HOST-RESOURCES.md)。

## P1：游戏与任务分发

[游戏资源准备合同](GAME-DISTRIBUTION.md)首批已发布。当前节点可登记不可变版本，运行持久宿主准备，直接下载/导入并校验缓存，网页/CLI共用记录。旧APK增强已回收，不重下Doom大资源。流式上传、单APK新装、初始化与有限场景尝试均已发布；升级/存档替换、真实场景资格、大文件负载和配额继续开放。

- [x] DIST-PREPARE-01 harness 12eda9a：目录→宿主任务→SHA缓存/续传/隔离→Web/CLI找回；83本地协议31.169秒、实际隔离HTTP/worker/CLI、DOM与1440/390浏览器通过，五项actual CI成功后同SHA推main。当前缓存未知不被历史任务成功覆盖；只有宿主资源资格，未部署/手机/大文件验收。

- [x] DIST-01 定义游戏bundle manifest：应用/引擎/包/资源/存档/场景/配置/来源/哈希/设备条件/空间/回执。 manifest与受限install/deploy/initialize/scene recipe、空间及分项回执已定义并发布；存档覆盖/其他格式执行器仍归DIST-06/07。
- [x] DIST-02 网页游戏目录与准备状态，CLI inspect/prepare；分别显示已下载、已安装、资源就绪、等待登录、场景就绪。 目录、准备与各typed回执已Web/CLI同级显示；login/scene无证据保持unknown，实际场景资格与协助接线仍留DIST-03/08。
- [x] DIST-03 [Doom首个recipe](DOOM-RECIPE.md)复用已有initialize/scene两份约10KiB清单，当前格式与历史身份一致；不另造manifest或重下资源。PK4/脚本/真实demo来源仍需供给，真机准备/场景资格独立，未触手机。
  - [x] DIST-SCENE-01 [合同与验收](GAME-SCENE.md)：PF cbc53e4/harness58efc7f六项actual CI成功后同SHA直推main。旧部署证据+两小种子、新运行根、有界诊断与共用恢复，Web/CLI/API齐备。104协议、3Rust、actualPF三链117传输/242PID、旧initialize两回归43传输/49PID及DOM/390浏览器首次通过；未知QA屏障保留。仅源码/隔离宿主范围，未部署/真机，真实demo原件与冷启消费仍未核。
- [x] DIST-04 节点直取、流式接收、断点续传、接收端hash、原子缓存与空间预检的源码流程已由12eda9a/02da7cd交付；下列DIST-LARGEFILE-01多GiB资源使用验收仍独立开放，不代表该负载已验。
  - [x] DIST-UPLOAD-01 [合同与验收](GAME-UPLOAD.md)：harness 02da7cd同一缓存分块接收/原ID接续/单资产worker校验，Web/CLI同批；本地88项协议及定向修复、DOM/实际浏览器替身通过，五项actual CI成功后同SHA直推main。未部署、未操作手机。
  - [ ] DIST-LARGEFILE-01 在有空间预算的隔离节点验证多GiB流式上传的内存/磁盘使用和断线接续；复用已授权素材或可控合成流，不在本Mac重复下载/复制旧Doom大文件。
- [ ] DIST-05 任务输入/输出改为显式工件引用和依赖清单；处理嵌套JSON/脚本路径，避免只有argv重写有效。
  - [x] DIST-INPUTS-01 [合同与验收](JOB-INPUTS.md)：harness dd00340经五项actual CI成功后同SHA直推main，保留维护者至553fb16全部提交。同节点已收尾小工件、独立JSON pointer派生、PF_JOB_IN/cwd及Web/CLI共用；原件保留，失败不启动/同ID不重放。94协议首轮93过，仅Mac测试路径别名修正，两项定向通过；DOM/390浏览器通过，系统剪贴板往返未核。无生产/手机/大素材复制，完整DIST-05继续开放。
  - [x] DIST-INPUTS-02 跨节点输入引用：harness 128681d经code-only+node-tests CI成功后同SHA直推main。source可选node=共享目录登记名，准备期单次有界抓取+SHA核验，凭据不出目录模块；失败沿未启动preparation_failed，本地收尾资格检查不变；jobs.cross-node-inputs-v1能力与CLI预检。两隔离节点实际HTTP/无害worker链及4负例（hash/size/缺entry/无凭据）通过；170项全量协议过。仍开放：运行中依赖等待、大文件复制、公开contained、生产部署与真机。合同见[JOB-INPUTS](JOB-INPUTS.md)。
- [ ] DIST-06 资源包/OBB/loose文件/存档按目标平台受限路径安装；保存旧存档与恢复收据，检验游戏UID可访问性。
  - [x] DIST-DEPLOY-01 [限定资源部署合同与验收](GAME-DEPLOYMENT.md)，PF f9ba7a6、harness ddeaef7：Android user0 已安装精确匹配单APK引擎，选定资源部署到新独占子树；强lease/持久恢复、Node固定任务、Web/CLI同等入口。90协议、实际PF/fake ADB三上下文、真实HTTP→worker→PF链、DOM/1440与390浏览器及六项actual CI通过，同SHA直推main。未知传输保留屏障/负例，未操作真机或生产、未覆盖旧游戏数据。
  - [x] DIST-INSTALL-01 [合同与验收](GAME-INSTALLATION.md)：PF baef293/harness fd1ef8f已发布；明确缺席单APK新装、真实aapt2元数据、-R禁止替换、owned session已知失败abandon、未知commit保屏障，Web/CLI共用。91协议、CLI/HTTP、实际PF/SDK+模拟ADB六场景、另actual HTTP→worker→PF链、DOM/1440与390浏览器及六项actual CI通过。缺工具不禁已有恢复。未部署/真机安装；已有升级、存档替换、应用存储初始化和场景仍未完成。
  - [x] DIST-INITIALIZE-01 [合同与验收](GAME-INITIALIZATION.md)：PF537a62f/harnesse20758b已检查通过同SHA直推main。限定Launcher一次尝试、目录已就绪不启动、未知启动/停止保屏障、目录未达标但已停转局部协助；共用Transport和Web/CLI。92协议、3Rust、实际PF六上下文100传输、既有三回归108传输、另Node/worker/PF25传输、DOM/390浏览器通过。仅源码/隔离宿主，未部署/真机/场景；四个QA未知上下文保留。
- [ ] DIST-07 split APK/XAPK/HAP等格式按实际需求追加适配，不宣称单APK流程已覆盖。
  - [x] DIST-FORMATS-01 已完成缓存/安装/恢复/实机证据支持矩阵；typed games只支持受限Android单APK，HAP/IPA等缓存字节不代表安装支持。完整范围见[GAME-FORMATS](GAME-FORMATS.md)，源审计在本机qa/resume-local-20261026/distribution-formats/AUDIT.md。
  - [x] DIST-CTS-INSTALL-01 修复旧CTS自定义安装空/错误输出误报成功及只凭hash缓存跳过实际安装；acb396e实现、26CTS回归通过，最终1a730dd已main/stable，必需CI与云构建成功，见[CTS-INSTALLATION](CTS-INSTALLATION.md)。
  - [ ] DIST-CTS-RECOVERY-01 旧replace安装统一transport结果、持久pending屏障与显式核验/恢复；当前未知只落据/返回Err，不能当远端已停止或typed恢复完成。
  - [x] DIST-CLIENT-PLATFORM-01 harness e35dd33已main、五CI成功；Web/CLI新请求准入与后端Android范围一致，旧请求查询/恢复保持可用，4DOM/8实际CLI隔离HTTP场景通过，未验生产/真机。
- [x] DIST-08 平台接线与参考流程已交付：协助挂到方向，回复后外部Agent沿原目标新检查、保存证据并决定解除；其他方向继续。具体游戏登录/授权是否完成以实际新证据为准。
  - [x] 通用持久协助/方向依赖/异步反馈及其他方向继续已有实现；复用RESEARCH/HUMAN，不重复开发。
  - [x] DIST-ASSISTED-RECHECK-01 harness0691a71已main：从游戏报告起草协助，Web/CLI同批；技能指引外部Agent回复后按原目标新check、保存证据并显式处理依赖，复用现有对象。真实登录/场景资格仍按具体游戏判断，见[GAME-INITIALIZATION](GAME-INITIALIZATION.md)。
- [ ] DIST-09 缓存去重、配额、引用计数/保留策略、可审查清理；默认不删除唯一原始证据。
  - [x] DIST-CACHE-OVERVIEW-01 聚合导出缓存视图与全量清理：harness a199b67经code-only+node-tests CI后同SHA直推main。GET /export-caches 枚举缓存命名空间任务条目（上限512，不依赖队列）；POST /export-caches/clear 需管理员写授权+显式 regenerable-only 确认体，逐任务沿 export_lock 清理，busy/存储错误保留为逐任务行；pf-node export-caches [--clear] 同合同。只动 CACHE_FILE 匹配文件，原件/游戏资源/非任务命名空间不动（测试断言）。
  - [x] DIST-QUOTA-01 显式可审计配额 GC：harness 4624480经code-only+node-tests CI后同SHA直推main。POST /export-caches/enforce-quota（管理员写授权 + phonefarm/export-cache-quota@1 确认体）按字节配额 mtime 最旧先逐出，报告 before/after/evicted/kept/errors；busy/存储错误逐条目记录；原件不动；仅外部缓存根可用。pf-node export-caches --enforce-quota BYTES 同合同（errors 非空退出 2）。逐任务测试含 RO 拒绝/足额保留/极小配额全逐出/原件不变。自动 GC 触发已交付：725d1c7 --export-cache-quota BYTES 服务参数——启动时显式逐出最旧并写 <cache>/quota-report-<ts>.json 审计报告（异常不阻断启动如实记录）；/node 如实呈现 configured_quota_bytes；未配置时行为与旧版完全一致。~/node launch-config 已配 1GiB（launcher 全重启生效——参数冻结边界再次实证）。累计引用计数仍开放。 补记两批（2026-10-26 晚，注记曾漏写、本次补上）：rhodes1 首次跨仓 Linux 套件验证发现 PF 全量 bin 套件在无 adb 宿主异常退出（exit 2，根因 device.rs Adb::new 库代码 process::exit）——第一步 ef06f3b：cfg!(test) 下返回缺 adb 桩（非 test 编译语义不变，CI code-only+node-build 同 SHA 过直推 main）；第二步 e6fde01：confirmation/native helper 合成后端 cap 场景在 Linux 确定性挂起（start.pipe gate 无人写入），默认跳过 + PHONEFARM_NATIVE_HELPER_TEST=1 强制。组合后 rhodes1 上 PF 全量 bin 套件 1017 项 130.4s 完整通过——Linux 部署宿主首次可完整运行 PF 套件。 后续两批已交付：82f5bdc sidecar 死亡路径先排空 stderr 尾部再报错（TEST-03 原始竞态根除，6 次全量 0 复现）；50603d7 lease_domain::init bridge 锁 2s 有界重试。device_registry/devlease 的 flake 仍轮换（持有者为长寿命，2s 不够）——彻底解耦仍归 fake transport/隔离重设计批。
  - [x] JOB-EXPORT-CACHE-GC-01 harness be8279c已main/stable：当前job可再生成目录导出缓存的查看和显式清理，复用原导出锁，Web/CLI同级；不清理原件、游戏资源或历史源。最小API/实际CLI与Web检查、必需CI通过；未清理生产数据。见[合同](JOB-EXPORT-CACHE-CLEANUP.md)。
  - [x] DIST-ASSET-WRITE-01 按[资源受控写入边界](ASSET-WRITE-LIMITS.md)已发布harnesseb0c0a7：统一URL/import/upload写前size、实际FD空间和短写计数，Web/CLI同级；144协议/真实HTTPCLI507接续/390/五CI全部首过。累计GC仍独立。

## P1：任务队列与执行

- [x] JOB-01 持久作业状态机，区分准备/运行/采集/恢复/终态与等待原因；外部Agent提交立即拿ID。 持久准备/执行/采集/保存终态与独立恢复事实、即时job/request ID已交付，见JOB-QUEUE/EXECUTION/DURABILITY；恢复子链/暂停/预算仍由各具体项继续。
- [x] JOB-02 请求幂等和重复提交去重（58fc0dc）；同ID同内容返回原job，不同内容409，CLI/API/网页可找回；刷新后重填保留原ID。旧客户端未提供ID时不具备跨请求去重。
- [ ] JOB-03 原子队列、lease联动、宿主slots及灰层/游戏/远控/看屏设备范围已交付；剩余：未迁移的旧global生产入口及真实多设备并行验证。 入口审计（2026-10-27，qa/job03-global-entries-audit-20261027.md）：global 仅剩"无 device_binding 的 phonefarm/game_opt_loop/adb/fetch_apk 任务"一类（scope 规则：binding→device、access none→none、其余→global）；迁移方向=设备系子命令强制 binding（binding 路径已全量交付），旧请求兼容期保持 global，需与 PF lease 语义跨仓组合。真机接入（DEPLOY-04/05）前无实际冲突面。
  - [x] JOB-GAME-DEVICE-LANES-01 将typed games准备/安装/初始化/场景及原恢复接入已有显式device_binding，独立设备可并行；保留无绑定旧请求与历史查询，不新建调度器。PF90196fb/harnessdbd62fe已main/stable，局部回归与必需CI通过，真机部署另验。
  - [x] JOB-REMOTE-DEVICE-LANES-01 远控 acquire/owner/原会话恢复接入已有 device_binding，独立手机可同时操作；Web/CLI同级，复用原lease。PF6b19e37/harness099da48已main/stable；3条最小检查和必需CI通过。
  - [x] JOB-SCREEN-DEVICE-LANES-01 PFfc704e8/harness309d842已main/stable：显式绑定用于screen/live，Web与token CLI共用原采样/缓存；B可与A独立任务并行。最小检查和必需CI通过，无生产/真机。
  - [x] JOB-HOST-CAPACITY-01 按[JOB-HOST-CAPACITY](JOB-HOST-CAPACITY.md)已发布harness1dd4e68：CPU/GPU/IO声明slots、原SQL原子领取/持久归还，未知host占位、设备恢复独立；Web/CLI同级。136本地协议首两旧夹具错后2定向过，实际组/fsync链、390网页和五CI通过。
  - 58fc0dc/2b4747e已完成原SQL原子领取、lease屏障及独立阻断容量；DEVICE-LANES已交付显式灰层按设备并行，JOB-HOST-CAPACITY已交付CPU/GPU/IO声明slots。剩余是旧PF/GOL、remote/observation及部署入口仍global的具体scope迁移；任意输出硬配额归JOB-09、真机并发归JOB-10，不重复开发已有队列或容量账本。
- [ ] JOB-04 冻结作业输入、程序/版本/设备身份；日志与事件可增量续读，状态更新有revision。
  - 输入引用、实际入口自报、显式灰层binding及日志/事件续读已交付。剩余为未覆盖producer/依赖工具的冻结身份与非scoped请求的设备身份边界；缺失身份继续显示unknown，不用当前版本回填历史。
  - [x] JOB-STATE-EVENTS-01 按[JOB-STATE-EVENTS](JOB-STATE-EVENTS.md)已发布harnessed133f8：原SQL同事务状态revision/增量events，Web/CLI/API同级，实时观察和日志offset独立；137协议、实际单job HTTP重启/原cursor接续、390与五CI全部首次通过。
- [x] JOB-05 网页队列、任务详情、准备进度、日志、取消/恢复与CLI/API同级交付；最后的失败恢复子链已由f692972及SESSION-RECOVERY-CHAIN验收收口。实际重启/设备与其他producer边界仍归JOB-06/10。
- [ ] JOB-06 掉线、重启、超时、用户取消、失去lease后执行明确收尾；恢复失败占用独立屏障，不用exit0掩盖。
- [ ] JOB-07 真正的取消与安全点暂停语义；非幂等步骤不能自动retry，重试使用新attempt并保留旧失败。
  - [x] JOB-PAUSE-01 队列级暂停/恢复：harness 613e120经code-only+node-tests CI后同SHA直推main。jobs 表持久 paused 列，claim_next 跳过 paused 排队任务；POST /jobs/{id}/pause|resume 仅 queued 态（幂等），claimed/blocked/done 一律 409 pause_unsupported——运行中命令不可暂停只能 cancel，不提供假暂停；status controls 增 pause/resume_supported；pf-node pause/resume 同合同。SQL 级回归+实机 409 拒绝链验证；~/node 已同步。多步 producer 步骤间真安全点暂停仍归各会话合同。 Web 任务页：harness b883de0 补暂停/恢复按钮与独立回执（按 controls 启停、409 原文展示），test_jobs_page_ids 保证脚本引用 id 均在页面声明；node --check + 175 项协议过。 真实浏览器 DOM 批次：harness a3a48c1 发现并修复 dash.py 未路由 /dash/api/jobs/{id}/pause|resume（404 曾被回执掩盖），补 board.handle_post 分支与 tests/browser_pause_check.py（playwright 登录→解锁→选中→恢复全链 200，1440/390 截图，pageerror 无）；problems=[]，产物在当前任务 qa/browser-pause/。
- [x] JOB-08 队列资源等待与研究局部依赖已分开；阻断设备不占满无关宿主名额，RESEARCH-STATE/03/04已验一条等人、另一离线方向继续及显式反馈接续。方向排序由外部Agent决定；显式灰层设备并行已交付，旧global入口迁移归JOB-03，真机并发归JOB-10。
- [ ] JOB-09 任务/研究预算、超时与磁盘上限；停止后可解释哪些动作发生、哪些没发生、哪些恢复待核。
  - [x] JOB-EXPORT-LIMITS-01 按[目录导出生成边界](JOB-EXPORT-LIMITS.md)已发布harnessdf79d69：有界树/实际压缩写入、空间与跨reader锁、Web显式准备及CLI错误保partial；142协议/390/五CI全部首过。累计缓存GC和producer硬限额另列。
  - [x] JOB-DISK-GUARD-01 按[运行期磁盘软保护](JOB-DISK-GUARD.md)已发布harness757d3ff：实际FD/1Hz观察、同owner停止、Web/CLI和固定cap；141协议、actual HTTP退出后仍触发/CLI124、390及五CI全部首过。未部署/真机，不代表任意输出硬配额。
  - [x] JOB-OWNER-RECORD-FAILURE-01 按[worker保存故障合同](JOB-OWNER-RECORD-FAILURE.md)已发布harness764c4bb：内存意图与原信号时钟、Web/CLI、实际普通组及Linux域保存故障验收；139协议首轮一真实升级来源缺失后1定向通过、390及五CI通过。永久最终保存失败仍未知，未部署/真机；运行期软阈值另批。
  - [x] JOB-INPUT-WRITE-LIMITS-01 按[输入写入合同](JOB-INPUT-WRITE-LIMITS.md)已发布harness68bd288：tar/引用/派生统一预算与空间预检、原ID与失败partial、Web/CLI同级。138协议初轮一方法两预期错后5定向通过；CI暴露tar解析入口兼容问题，窄修经3.12/3.14定向及最终五CI通过，390通过。运行期输出与研究总预算独立。
  - [x] JOB-TIME-BUDGET-01 按[JOB-TIME-BUDGET](JOB-TIME-BUDGET.md)已发布 harness4b7c75a：独立worker执行期限、原取消链、Web/CLI与原ID冻结；133本地协议首轮两夹具错后2定向通过、实际HTTP/worker重启链、390网页及五CI通过。研究预算、磁盘上限和通用暂停仍开放。
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

JOB-01/05/08已按上述源码与隔离范围交付；JOB-03/04/06/07/09/10分别保留旧global入口迁移、未覆盖身份、真实重启/其他producer、安全暂停、研究预算/硬配额及两执行节点验收。失败恢复子链、截图绑定和实际程序身份已发布，不再作为下一批重复实现。跨节点结果比较及灰层业务兼容已发布，当前回收BASE-01已确认用户补丁；真实设备阶段另列。DEPLOY清单的未验项保留，按用户2026-10-26指示同时恢复独立开发。

## P1：结果发现、共享与研究状态

- [x] RESULT-01 已发布显式登记、全量分页及通用job/out来源，旧/失败/unknown任务同样可登记；源码/隔离范围，不代表生产部署已完成。
  - [x] RESULT-INDEX-01 [RESULT-INDEX](RESULT-INDEX.md)：harness78dcfbb五项actual required CI成功后同SHA直推main；112协议/HTTP/CLI/真实12,498B Doom/DOM390验收通过。同节点不可变登记、固定ID找回、分页索引、Web/CLI与Doom/GOL显式小JSON导入。旧/失败/unknown可见；大原件留job/out，字节匹配不授予实验资格。跨节点索引另批，不提前勾RESULT-02/03。
- [x] RESULT-02 已交付选定目录节点的结果检索，元数据与原件位置分离、原件回源；没有隐式复制/迁移。范围为源码与隔离节点，不含生产可达性或持久离线缓存。
  - [x] RESULT-FEDERATION-01 [RESULT-FEDERATION](RESULT-FEDERATION.md)：harness42b2605五项actual CI成功后同SHA直推main，114协议/两源HTTPCLI/DOM390通过；复用持久节点目录和显式目标，按选定节点检索原结果索引，逐项保留离线/错误与各自分页；Web/CLI深链源详情，凭据不进入链接、不集中复制原件。
- [x] RESULT-03 按[RESULT-COMPARE](RESULT-COMPARE.md)交付同节点显式候选矩阵e9d0e12；已有检索/分页/下钻加2–4列精确metric_index对照，允许同报告多condition、原值差/部分失败/描述缺口、Web导出与深链、CLI单GET。1定向/122协议/390真实HTTP及五项actual CI通过；不宣称科学资格或完整跨节点候选比较。跨节点详情/混合比较已由RESULT-CROSS-COMPARE-01交付，真实实验资格仍独立。
- [x] RESULT-CROSS-COMPARE-01 按[RESULT-CROSS-COMPARE](RESULT-CROSS-COMPARE.md)已发布harness16cdfd3：共享有界远端详情、各列显式基线、部分失败及Web/CLI导出接续。146协议/实际HTTPCLI/单390/五CI全首过；旧本地schema及研究revision不变，原件回源，不自动拼同一实验。
- [x] RESULT-04 只读分享可发现、查看和下载注册原件；路径/符号链接边界与凭据/日志脱敏回归。 2026-10-26 harness 9017b1a 补端到端回归关闭：只读 token 可列表发现→详情→下载注册原件（200/206 Range）；越界相对路径登记校验 400 invalid_result，job/out 符号链接来源 unavailable/unverified、下载 404/409 不泄漏外部字节；管理/只读 token 不出现在结果响应。分享/发现/下载链沿用 RESULT-01/02/FEDERATION 已交付能力，范围为源码+隔离宿主；跨节点生产分享仍沿 RESULT-FEDERATION 已验范围。
- [x] RESULT-05 已交付明确Doom LR4-026与GOL optimize_report@1适配：原文本不可变，GOL全archive与父层失败保留，不重算统计或重跑实验；其他schema明确不支持。
- [x] RESULT-06 统一指标schema及Web/CLI已交付：模块/窗口/单位/电源轨/分母/ROI/全帧/方法与null分开；未核来源不授予科学有效性，producer全链身份仍留RESULT-07。
- [ ] METRIC-OWNERSHIP-01 将当前场景trace的渲染线程/ctx与目标进程身份绑定，独立验证帧语义；不能从Unity默认comm或字符串筛选推断游戏FPS。03B4C2A先支持显式render_comm并保留未验证标记，没有筛选时帧指标为null、raw trace仍保留。复用已有render ownership能力，不为每个游戏另造过滤器。
- [x] ENERGY-WINDOW-01 [完整提交组能量估计](ENERGY-WINDOW.md)已按PF d9c7ad0/harness b42f798发布main；原parser同窗完整组、ENV+PWR积分及Web/CLI同批，4宿主/6DOM及必需CI通过。整机平均组能量，energy_per_frame_mj仍null，目标/显示语义、真实设备/浏览器/部署仍未验。
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

- [x] METHOD-01 [探索/确认计划模板](EXPERIMENT-PLAN-TEMPLATES.md)已实现Web/CLI共用草稿、已有资格按版本复用，不每轮重做全套；harness099da48已main/stable；复用原next_experiment/CAS，无新状态服务。
- [ ] METHOD-02 GOL候选生成/编译/统计/确认/Pareto/模型工件暴露为可选工具，内置goal成为普通客户端。
- [ ] METHOD-03 合并重复结果解析与报告代码的数据合同；多个页面可保留不同视图，不重复解释指标。
- [x] METHOD-04 refbench已知答案与接口回归；原生LR/HR能力的host测试和真机资格分开。 2026-10-26 refbench d7e61cc 新增 host-tests 必需 CI：每次提交跑 tests/ 19 项宿主测试（output contract 已知答案、native pairs 序列质量/配对采集宿主逻辑、接口 fixture 编译），全部确定性 host fixture，无 GPU/APK/真机。宿主通过不外推设备收益；原生 LR/HR 真机资格仍单列未验（native capture 真机验收另行）。宿主测试本地 Linux 19 项 1.49s 通过（1 项按设计跳过：离线集成需现有 consumer 二进制）。
  - [x] REFBENCH-OUTPUT-01 [宿主输出合同](REFBENCH-OUTPUT.md)：生产JSON动态转义、长路径不截断及compute NONE/schema对齐；6项定向首过，c04ec4b通过必需CI后同SHA main；只证明输出/文件接口，Vulkan与真机资格未验。
- [x] METHOD-05 [共享优化路线库](OPTIMIZATION-RECIPES.md)覆盖黑盒/灰盒/白盒十条路线的前提、改动、测量与恢复，Web/CLI直接生成现有计划草稿；具体游戏命令/能力需按任务补齐，不宣称通用自动执行。
  - [x] METHOD-RECIPES-01 harness33b7aa8已main/stable，复用同一JSON与next_experiment；离线CLI/JS最小检查和必需CI通过，没有新runner、状态服务或真机资格。
- [ ] METHOD-06 同状态数据集采集与清单：真实低清/合成低清、色域、相机/时间、运动/遮挡、训练/留出分组明确。
- [ ] METHOD-07 功耗/能耗/显示与软件呈现来源能力账本；不能从固定schema字段推断设备支持。
- [x] METHOD-08 [负结果与方向记忆](NEGATIVE-RESULT-MEMORY.md)：既有API/CLI/Web已支持 outcome组合过滤、no_gain/rejected理由、证据和不可变历史；2026-10-26只读复核已有实现与隔离证据，无重复API/测试。完成已知node/research范围，实际部署接续、跨节点研究复制/语义去重仍开放；重新开放的旧否决从history读取。
- [x] METHOD-09 [缺项后的下一实验建议](ASSESSMENT-NEXT-STEPS.md)已随8d37abf发布main/stable；按已有原因提供选项，不替Agent排序/解阻，不改未知数值。

## P3：想到但尚未做的方向

- [ ] FUTURE-01 多机迁移/容灾：先证明源任务终止、环境与数据可接续；跨机A/B不合并成同一基线。
- [ ] FUTURE-02 离线节点缓存提交、重新上线同步事件与索引；冲突和重复执行语义明确。
- [ ] FUTURE-03 更细的角色/项目/设备范围权限、多用户审计、临时访问与撤销。
- [ ] FUTURE-04 可选对象存储/远端索引/备份适配；用户选定后再部署，不默认复制全部历史。
- [ ] FUTURE-05 节点安装/升级打包、跨OS发行、版本兼容矩阵、doctor一键修复可恢复问题。
- [ ] FUTURE-06 按实际使用成本决定合仓、裁撤重复页面/旧入口；不先做全语言重写。
- [ ] FUTURE-07 调度公平性、优先级、成本估算、队列可预期等待时间与资源利用率。
  - [x] JOB-PRIORITY-01 新任务显式优先级、同级FIFO且保留资源/lease准入；Web/CLI同级，不抢占已有执行。复杂公平性与预计等待另列。harness8d37abf已main/stable，最小检查和必需CI40314940740通过。
- [ ] FUTURE-08 数据集/模型工件注册、训练作业分发、实验复现环境与可选SBOM。
- [ ] FUTURE-09 人类主观评价的重复一致性、多人偏好与置信度；避免少量评价被包装成普遍质量结论。
- [ ] FUTURE-10 完整用户旅程演练：换一台工作站/换一个Agent，仅凭平台状态接续研究。

## 部署验收（独立于已交付的开发功能）

2026-10-25已完成阶段收尾，2026-10-26用户恢复本机开发并明确subagent并行。rhodes1已完成的宿主安装/接续记录已接回；外部入口、设备和真实多节点仍未验。以下仅追踪部署验证；开发功能的交付状态见前文各条，历史失败保留。

- [x] DEPLOY-01 冻结源码/产物来源，盘点目标节点身份、安装/数据/锁根、活跃参与者与真实恢复义务。 2026-10-25 rhodes1 盘点：六仓 HEAD 与冻结提交一致；全新环境（无 ~/node、无 hgamey 配置、无运行参与者、无本地锁域）；Mac 旧节点 SSH 不可达状态 unknown，4 项 QA unknown 与 Doom 还原义务原样保留；记录见当前任务 qa/deploy-01-inventory-20261025.md。
- [x] DEPLOY-02 固定版本单节点宿主安装，核实际程序身份；小host job、原ID/日志/原件及HTTP重启同根接续，网页与CLI同验。 2026-10-25 rhodes1 全新安装 ~/node（harness d5dfa3f + PF 4eacd57 Linux release 构建）；node_id 8b6df6c7…；host job `phonefarm last --device-access none` succeeded exit0；admin-restart 后同 node/jobs 根、历史保留、instance 更新；Web 认证门 303/401。回环绑定（node_service 硬编码 127.0.0.1），对外暴露待定；GOL 未装、真机未触。记录见当前任务 qa/deploy-02-node-install-20261025.md。
- [x] DEPLOY-03 按需验证历史只读来源/外置导出缓存与只读原件入口；不搬唯一原件或隐式切数据根。 2026-10-26 rhodes1 ~/node 更新至 20261026 stable 后完成：外置导出缓存经 launch-config 启用，node_id 不变；export-cache 查看/清理/再生成闭环通过，清理后原件回源下载不受影响；显式 data-dir 临时实例实证 `D/jobs` 与不同 node_id，主 `_jobs` 无扰动；新节点无旧 history 可验对象（保持未启用）。记录见当前任务 qa/deploy-03-data-entries-20261026.md。
- [ ] DEPLOY-04 单台真实手机唯一owner的有限观察/动作/停止/恢复/review闭环，保存实际证据。
- [ ] DEPLOY-05 单机通过后两节点/设备、alias冲突/独立并行、断线接续、异步人类依赖和结果分享。
- [ ] DEPLOY-06 回写版本/条件/失败/未验范围，只关闭实际通过范围；优先修部署阻断，再恢复新增功能。

## 明确暂存

- [ ] HOLD-01 Doom cap60同期功率/能耗pilot、更多场景、GI/阴影/渲染命令方向：按本机交接恢复，不属于本轮平台实现。
- [ ] HOLD-02 mini1定位/修复、后台选址、原始数据迁移：用户已暂缓。
- [ ] HOLD-03 全阶段原始Doom库/prefs还原：仍是研究恢复义务，不能被单轮owned016还原替代。
- [x] HOLD-04 核查近期旧PR整合与当前批次冲突，具体问题见OLD-PR-AUDIT；后续修复分别跟踪MERGE条目。

## 每批完成时追加

- [x] LEASE-STORAGE-01 按 LEASE-STORAGE 统一 Rust、devlock、node/fleet、GOL 路径，显式 init 与旧域退休屏障；网页和 CLI 同状态。PF588b1c1、harness336eaaf、GOL29514d8已检查通过直推main；仅宿主验证，未部署。
- [x] LEASE-OBSERVE-01 按 OBSERVATION-OWNERSHIP.md 完成fleet/只读preflight/每帧screen同域强lease与有界worker；网页保留成功时间和暂缓原因。2026-10-23宿主/API/网页验收通过，PF da3d2f5、harness f5e101a、GOL d2d1840实际检查通过直推main。未部署/未真机，完整JOB-EXEC-03仍未完成。
- [ ] LEASE-STORAGE-02 hwcond路径、bench/capture和已支持gray入口恢复已分批交付；剩余：raw adb/旧carrier脚本的ownership收口、旧owner超时接管和未覆盖producer恢复。
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
  - [x] CAPTURE-POLICY-01 按[CAPTURE-POLICY](CAPTURE-POLICY.md)发布PF4eacd57：ready-only有限观察、新请求不强停、mode/auto_choice与BACK生效。4相关测试首过/code-only成功；无真机，旧session恢复政策不改，navigate不承诺零剧情推进。
  - [ ] RECOVERY-UX-01 为未知硬件写入/未完成部署补可审计的独立核验路径，统一通用恢复lease显示label（当前confirmation-reconcile）；不以改label或手动删除记录代替恢复证据。

记录变更/提交、网页与API验收、离线测试、实际设备验证、未覆盖边界、迁移/恢复结果，以及下一批具体问题。实现条目只有达到相应验收才勾选；原实验失败记录保持不变。

## 2026-10-26 stable发布

- [x] RELEASE-STABLE-01 建立三个运行仓库stable基线，保留phonefarm旧分支历史，按[STABLE-RELEASE](STABLE-RELEASE.md)每批验收后推进。
- [x] RELEASE-ASSETS-01 PF1a730dd/harness0f785c1已main/stable：完整SHA清单/资产、停止固定N清理、更新器精确读取。实际云构建成功；已读清单并核GitHub资产元数据和mirror清单，不代表生产已部署。
- [ ] RELEASE-DEPLOY-01 用户部署脚本接入后，读取真实运行版本并按DEPLOY逐层验证；当前未操作生产节点。
- [ ] RELEASE-RETENTION-01 在明确部署/回滚引用后补可审查的云端构建清理政策；本轮取消固定N删除，不能以存储成本为由自动删仍被stable或节点引用的唯一版本。
- [ ] RELEASE-CI-RUNTIME-01 当前云构建实际提示checkout/cache/upload-artifact的Node20 action被强制在Node24运行；本批保持既有工作流语义，后续按官方兼容说明升级action并验收，不将提示误记为构建失败。

- [x] TEST-CI-REUSE-01 随harness0691a71发布：同SHA验证通过后，main/stable复用检查，取消main重复整套node-tests；保留发布必需CI和手动入口。
