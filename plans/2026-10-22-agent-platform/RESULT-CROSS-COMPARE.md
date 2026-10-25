# RESULT-CROSS-COMPARE-01：跨节点详情与显式基线对照

2026-10-25，实施合同。接续既有 RESULT-FEDERATION 与 RESULT-COMPARE，完成现todo同名条目。只读源码/隔离宿主/CI，不部署、不迁移原件、不运行游戏。

## 入口与身份

新增 results.cross-compare-v1。GET /remote-result?target=<JSON>&result_id=RID 与 /result-cross-compare?spec=<JSON> 及 dash 镜像均要求hub显式pin和现只读权限。target恰为目录的{name,node_id,revision}，拒任意URL/token/path，不自动refresh或改绑。本地详情保留/results/RID。

新spec schema=phonefarm/result-cross-compare@1，≤16KiB。顶层node_id为hub；targets为1–4个互异已绑定远端目标，不能以hub身份冒充远端。columns为2–4项{id,label,node_id,result_id,baseline_column_id}，每列node_id必须为hub或明确target，至少一列远端，不能有未使用target。rows为1–16项，沿旧{id,label,metric_indexes}，context沿旧合同。ID/标签/指标索引限制复用旧验证。baseline_column_id必须显式为已选列ID或null；自列可表示自己的基线。没有默认跨机合并或按指标名join。

读取和缓存按(node_id,result_id)去重；同报告多condition列仍允许。每机基线可独立指定，如A_base/A_candidate、B_base/B_candidate分别成对；作者也可显式选跨节点基线，此时标明cross_node=true，仍只称登记值差。null保原值、差值未知，reason=baseline_not_selected。每个cell返回baseline_column_id；基线不可查只影响依赖它的差值。旧本地schema/首列基线/RESEARCH criteria不变，不自动迁移旧研究revision。

## 有界详情

共用现NodeResultSearch服务实例的四个reader槽和8秒总deadline；search/detail/cross不各建新池或队列。一源一次/node，随后只GET去重的/results/RID，全带源pin、禁redirect、每响应沿2MiB上限。源必须声明results.index-v1和node.target-v1，无须升级cross能力。网络在目录锁外，源批次完成与装配返回前核冻结revision；变化就丢该源结果，不改投新URL。迟到线程仍持槽且不得发送过期凭据，不修改已返回响应/目录。

详情核schema/node_id/result_id/manifest.result_id、登记sequence/时间与必要数组/对象形状；manifest复用现内部验证，拒非有限/非法指标。保原manifest/provenance/source_observations/metric_evidence/current事实，当前读取不是同时刻快照。远端字节只是源声明，不升级为hub实测。所有导航由已核目录URL+源pin+rid/源名重建，不信任远端导航URL；原件仅回源站自己的详情/下载入口，不代理内容或转发凭据。

remote-result响应{schema_version:1,node_id:<hub>,source:{name,node_id,url,revision},result_id,attempted_at,read_at,status,error,result}。成功status=ok；失败result=null，沿已有目录错误分类，另有not_found。源声明结果API后，详情404仅在有限JSON错误code=result_not_found时归not_found；普通404仍unsupported，不回显远端错误正文。部分失败不伪装为空结果。

## 比较与客户端

装配每列source/read_at/登记身份后复用原_cell/_descriptors/_delta，避免第二套算术。沿已知unit/scope与有限数值要求、正基准百分比规则；不补FPS、功耗、显著性或资格。响应basis=registered_values、comparability=not_established，列status available/unavailable、error和source_status独立，partial只表示不可查列；字段/方法/设备/窗口/ROI差异照列显示。最终JSON沿客户端6MiB边界，过大明确413，提示减少列/指标，不截断或自动重查。

CLI results show RID --node NAME --source-node-id ID --revision N走remote-result；必须给全冻结目标，不另查目录选新版。results compare FILE仅见明确新schema才走新路由，旧路径不变；单次业务GET，完整partial JSON及exit2。错误不自动重试、注册或下载。

网页在原远端搜索组提供查看/加入跨节点对照，详情经hub有限读取，精确metric索引及每列基线由人明确选择。保各源身份/目录版本/登记时间、未知/错误和源详情链接。比较导出/深链保存完整spec；刷新不能从当前目录重写冻结版本。失败保选择，迟到响应按请求版本隔离；窄屏不横溢。旧同节点研究判据入口不接受新cross spec。

## 验收

复用两源小结果HTTP夹具：同名rid、四列独立基线、同报告多condition、缺轴和部分离线，核(node_id,rid)去重、一次/node、原指标差值/源链接/manifest不改；CLI show与compare仅单业务GET。另一合并链针对详情的404/身份/配置在途变化/redirect/共享槽，沿旧Event夹具，不重复分页和算术矩阵。单390选择两源→显式基线/指标→部分失败保列→回源链接/导出深链接续，无原件代理或设备。唯一一轮协议，失败只窄跑。

## 已发布与验收记录

2026-10-25，harness `16cdfd3101dc3ed23733c803ce69360406f7a963` 五项actual CI成功后同SHA直推main并ls-remote核对。唯一本地协议146项/83.842秒首过；两条新增纵向链覆盖实际HTTP/CLI的四列独立基线、null/显式跨机基线、部分离线、原ID去重，以及404/身份/redirect/在途revision变化和共享四槽。原同节点及研究判据回归通过。

唯一390px合成HTTP网页链首过，业务GET为检索1、详情2、比较2；四列选择、B离线保留、1293字节spec原生导出及同tab深链重载保持原revision。总29GET含3次QA收据，0POST/跨域/原件/设备；0页面错误、宽度390无溢出。root目视partial截图。fixture26928/browser27007退出，61253端口关闭。网页是合成HTTP验收，协议测试才是实际Node/CLI，不冒充真机或生产。

本机证据：`qa/result-cross-compare-{validation,checks,publish,ui-validation}.json`、协议原日志及UI原回包/截图。没有新增执行/设备调用，没有迁移数据或清理旧QA恢复屏障。旧研究revision没有升级为新cross schema。
