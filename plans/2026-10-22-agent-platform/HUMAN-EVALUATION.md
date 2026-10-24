# HUMAN-EVALUATION-01：固定原件的图片/视频 A/B 评价

2026-10-25，已发布 harness f7e1ee1704421794e9f2fc0f877f556e9631dbf6；五项实际必需CI成功后同提交直推main并核远端。属于 HUMAN-04；复用 RESEARCH-STATE 的四类对象、原操作、版本与结果来源。只做源码、隔离小媒体和浏览器，不部署、不操作手机、不复制旧实验媒体。只读审查依据在本机 qa/human-evaluation-next.md。

## 一条流程

创建固定两份原件和 A/B 次序的 evaluation 请求 → 人显式加载、查看和回答 → 不可变反馈绑定原问题版本 → 外部 Agent 决定采用/暂缓与下一实验。回复、负面评价、到期或关闭均不自动解阻、授权设备操作或判定实验无效。旧自由文本评价保持原含义。

首批两张 PNG/JPEG 或两份 MP4/WebM 完整原件；A 为左侧/窄屏上方，B 为右侧/下方。没有裁片、转码、逐帧同步、自动配准或统计汇总。完整视频由人独立手动播放，解码成功不等于看完整段。

## 原对象可选扩展

新增 capability `research.evaluation-v1`。字段缺席时不添加默认 null 键，保持旧 data/operation canonical 与原ID重放兼容。更新旧请求不得悄悄补字段。新字段有值时严格验证；不接 evaluation:null。

request 仅 type=evaluation 可增加：

```json
{"evaluation":{"schema":"phonefarm/ab-evaluation@1","media_kind":"image","slots":{"A":1,"B":0}}}
```

media_kind=image|video。slots 恰好 A/B 两键，各为 evidence_refs 的有效整数索引，且不同；所选 ref 为同节点已登记 job_output result_source，登记 observation available、有 stamp 与 size，并在大小限内。允许两项最终引用同件原件作为控制对照；保存事实，不据此伪造两种候选。

创建仅核不可变登记元数据，不打开/hash媒体；因此只称“绑定已保存”，不称“可显示”。external/imported_json/unverified 不能充当槽位，其余说明引用沿原合同保留。当前字节缺失/变化在显式加载时失败，不改变旧问题。

**创建端固定排列**：Web 在创建时提供一次随机 A/B 与明确固定排列选择；随机后写入 data.slots，发送、刷新、原ID接续不得重新随机。CLI/API 接收明确 slots，外部客户端可以先随机再提交同一固定 data。平台不许诺跨参与者平衡随机分组，也不在 GET 时随机。请求状态更新不得修改内容或排列。

feedback 可增加：

```json
{"evaluation":{"schema":"phonefarm/ab-feedback@1","preference":"A","difference":"small","reviewed":{"A":true,"B":true},"client_display":{"A":"decoded","B":"decoded"},"identity_revealed":false}}
```

此结构仅回答带 AB 的原 request_revision；普通文字答复仍可省略。原 feedback.text 为非空评论，不另造评论槽。

- preference=A|B|no_preference|unable_to_judge|null。difference=none|small|moderate|large|unknown|null；差异是可见差异程度，不是质量分数或胜率。null 不折算为0、无偏好或无差异。
- reviewed 恰好 A/B，值 true/false/null，是提交者声明。A/B/no_preference 要求两边 true；unable_to_judge/null 仅接 difference unknown/null，保留未看过/看不清原因。
- client_display 恰好 A/B，decoded/failed/not_loaded/unknown；是客户端报告，CLI可用unknown，不冒充浏览器观测。图像 decode 成功、视频 loadeddata 且 videoWidth/videoHeight 有效才可报告 decoded；loadedmetadata/音频播放不算画面。
- identity_revealed 为 bool|null，表示本客户端显示区是否主动展开素材身份；不是服务端证明其从未知道身份。
- Web偏好判断要求两边本地显示成功和人主动勾选已查看；失败仍可提交无法判断/一般文字反馈。API不能证明真实观看或自然人身份。
- 服务端根据原 request_revision 的固定 slots 添加提交时 evaluation_binding（schema/media_kind/slots/两份原引用及注册绑定），不接受答复另换素材。既有提交时 request state/revision/context_changed 原样保留。

## 显式媒体读取

只读路由：
`GET /research-state/request/{id}/media/{A|B}?revision=N`
及 `/dash/api` 镜像。revision 与 expected node pin 必须明确。沿现 token/查看密码，不新增会话、权限或媒体复制库。页面 URL 只有同源 cookie+expected_node_id，不带 token 或任意宿主路径。

复用 NodeResults.source 的 SafeRoot/FD/登记 stamp/describe_file 与原 Range/ETag。先在同一FD上检查实际大小、最多64KiB头结构及媒体类别，再调用原哈希/流式发送；只由内部允许的 MIME 改为 inline，旧下载入口保持 attachment。没有客户端任意 MIME 参数。列表、详情和创建不调用媒体source或隐式读大文件。

上限：图片每份16MiB、单边8192、总像素16,777,216；视频每份128MiB。按 fstat 在 hash 前拒绝超限；视频只限字节，不因此叫“短片已核”。页面可见限制。首次显式读取沿旧 describe_file 完整SHA一次，后续同stamp复用缓存；重启/缓存失效可能重算，不能称零I/O。没有全文件前端Blob/base64或服务端重复复制。

| 格式 | 服务器有限识别 | 剩余 |
|---|---|---|
| PNG | signature/IHDR、宽高/像素限制 | 浏览器完整解码 |
| JPEG | SOI/有界标记/SOF宽高；64KiB内未找到拒绝 | 浏览器完整解码 |
| MP4 | 有界合法ftyp及常见视频容器brand；不接任意ISO BMFF | 实际codec/视频轨/尺寸由浏览器报告 |
| WebM | EBML header与DocType=webm | 实际codec/视频帧由浏览器报告 |

HTML/SVG/JSON、未知/无法识别格式拒绝，不按后缀回退。容器识别不证明完整有效或有视频轨；浏览器解码失败显示原原因，可回答无法判断。源变化/缺失不删旧反馈，不自动替换为同路径新文件。

## 界面与接续

研究页增加AB创建与结构化回答入口，独立评价页/面板可用普通研究API加载固定版本。只显示中性 A/B、原尺寸/时长/缩放/读取状态；素材身份入口默认折叠并明确“会揭示标签”。原问题标题/说明也可能带标签，创建时提示使用中性措辞。

这是界面隐藏标签，不是抵抗主动读API的保密盲化；现只读权限仍可查原result/request上下文。报告保留identity_revealed声明及这一范围，不能称双盲或身份未知已证明。

原件由显式按钮加载；video preload=none、无autoplay、默认静音和独立原生controls。响应式布局不交换A/B。隐藏、切换请求/离页时暂停并移除媒体src；再次显示不自动继续加载。图片contain缩放，显示原尺寸，不拉伸伪装同状态。素材变化/关闭问题/上下文版本差异各自显示，不合并成虚假的“评价有效”真值。

Web/CLI同读写request/feedback；CLI create 带evaluation时先核新增能力。show request 返回固定版本的同源媒体路径/范围说明（不自动下载），CLI显式 `research media REQUEST A|B --revision N --output FILE`，使用同一target认证，只读一次整件流式下载；同遍核长度/SHA，完整后原子发布且不覆盖已有文件。失败仅处理本次partial，不自动重试或创建反馈。丢回执只读原operation；修正答复新ID+supersedes，不重开问题或改依赖。

## 最少验收与范围

两个小协议纵向足够：旧请求/反馈canonical与原operation升级兼容，固定slots/CAS/迟答原绑定；实际PNG/JPEG/小视频头与Range、权限pin、源变化/不支持/超限仅所需分支。沿既有SafeRoot和媒体合同，不复制整个路径/auth矩阵。

一次390px实际浏览器流程：固定AB创建→显式加载两图→主观答复→CLI读原反馈；同流程展示小视频可显示帧和可见失败/无法判断。不重做研究页全部并发和断线矩阵。复用本机已安装工具生成极小synthetic媒体，无外部下载和设备操作。按AGENTS新协议整轮一次；失败只补受影响链，保留原日志。

后续参与者真实身份/更细权限、保密盲化、跨人随机化/一致性统计、裁片/ROI/同步播放另列；这些不算本批完成，也不要求同步等人。


## 实现接口补充

request.derived 的新增 evaluation 保存 schema/media_kind/slots，并给出 media.A/B（token API相对路径，含revision与expected_node_id）、limits(max_bytes/max_dimension/max_pixels)与blinding=presentation_only。网页沿同源dash API使用这些路径。详情仍只读元数据，不打开媒体。

feedback.derived.evaluation_binding 保存原请求的schema/media_kind/slots及references.A/B、evidence_bindings.A/B。原data中的评价声明、服务端固定原件绑定、客户端显示报告与提交时上下文各自保留。旧对象不补任何evaluation键。

媒体GET只允许revision/expected_node_id查询，Range用HTTP头，不新增meta/offset接口。明确错误：未绑定目标400 target_required；非AB问题409 research_evaluation_required；登记源不符合可读job_output元数据409 research_media_unbound；大小413 media_too_large、图像尺寸413 media_dimensions_exceeded、头格式415 media_unsupported；原件变化/缺失沿结果来源原错误。浏览器解码仍独立核。


## 当前验收记录

两条新 HTTP/CLI 纵向首次通过（1.241秒）；完整118项节点协议首次通过（55.825秒），无重复整轮。旧canonical回执、原请求绑定、变化/不支持/大小限制与实际CLI不覆盖下载已核。网页390链已完成：1个图像请求、2个主观答复共3次研究POST均201；PNG/JPEG与160×96、1.2秒MP4实际解码，失败视频415后提交无法判断。源身份展开后同页面重读仍记录true，一次现有CLI show核对原v1绑定。合成媒体共13,145B，仅用于平台验收，不作为Doom画质证据。首次QA的Playwright option.isDisabled断言不适配原生option；按真实DOM disabled属性确认后只续视频反馈，原失败保留，没有重发图片/请求写入。2个browser contexts均pageerror=0且390px无横向溢出，fixture PID75305和53126端口关闭。页面隐藏卸载仅静态审查，不宣称浏览器注入验证。本机索引 qa/human-evaluation-validation.json，原日志 human-evaluation-targeted-1.log 与 human-evaluation-protocol-1.log。


同页素材身份揭示按 node/request/revision 记录在内存；重读不回退为未揭示，跨新页面/浏览器不追踪。它仍只是客户端声明，不是保密盲化证明。
