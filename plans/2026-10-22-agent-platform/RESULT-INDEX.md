# RESULT-INDEX-01：显式结果索引与小报告导入

2026-10-25，已发布harness 78dcfbba61b67137ac1d373173f6663030de376b；五项actual required CI成功后同SHA直推main并核远端。依据本机result-index-next与result-index-ui-next审计。主线从任务执行进入结果发现；只做源码/隔离宿主，不部署、不迁移本机实验或重新采集。

## 范围与事实

NodeRead已有全量任务分页；缺口是旧看板只从最近200任务猜目录并最多展示15项。新增独立的已登记结果页与API，不继续扩大扫描。结果登记不创建job、不占设备、不使用job_inputs的宿主完成准入；旧worker、失败、unknown和大原件均可登记。原件仍在原job/out，显式导入仅保存一份至多1MiB的UTF-8 JSON。

首批能力`results.index-v1`，稳定身份为(node_id,result_id)。只保证同节点；跨节点索引下一独立子项，复用现有目录，不建新的中心原件库。登记/导入是元数据操作，不授予测量有效、收益、设备恢复或同状态资格。研究/方向先作为显式标签，随后研究对象合同接入。

## 不可变登记

`POST /results`接`phonefarm/result@1`，`result_id`为客户端预先生成的1–64位`[A-Za-z0-9._-]`标识。原ID相同规范内容返回原记录；不同内容409。修正使用新ID与`supersedes`，不覆盖旧数字。来源当前变化不重新解释原记录。首批字段：

| 字段 | 合同 |
|---|---|
| schema, result_id, title | schema固定；title非空、至多240字符 |
| job_id | 本节点已有12位job ID或null；旧源自报job ID只能放external_reference，不关联当前同名job |
| labels | game_id、game_version、device、research_id、direction_id；字符串或null，缺省null |
| origin | producer、producer_version字符串或null；evidence_origin为device/synthetic/unknown；observed_at有限时间戳或null。均为登记者声明 |
| outcome | gain/no_gain/regression/failed/not_run/unknown，是报告结论，不能由exit0或kept推成gain |
| source_outcome | 原报告状态文字或null，保留原语义 |
| sources | 至多64个命名来源，见下；原件清单与指标不混为一体 |
| metrics | 至多256项，value为有限数字或null；name、condition、unit、scope、method、window、denominator、power_rail、image_domain、region、source、unavailable_reason；缺失的描述字段为null/unknown，不填0 |
| claims, limitations | 文字数组，登记者声明与实际来源检查分开显示 |
| supersedes | 同节点旧结果ID或null；不修改被替代记录 |

指标`scope`为module/game_loop/display/gpu_submit_proxy/power/image/unknown；`method`为measured/derived/estimated/reported/unknown。`condition`明确baseline/candidate或原报告候选标签；window为保留原意的文字或null。region为full_frame/roi/unknown或null；source为`{name,pointer}`或null，pointer使用JSON Pointer。单位和窗口未知时不进行默认换算或计算排名；null展示未知及原因。CPU与GPU模块不可相加，ROI不可冒充全帧，GPU提交代理不可冒充显示FPS。

来源为`{name,kind:"job_output",job_id,path}`或`{name,kind:"external_reference",locator,declared_size?,declared_sha256?}`。path只限现有job/out内受限相对路径，复用SafeRoot。外部locator仅历史声明，不能变成服务器任意文件读取接口。adapter内部另可生成`{name,kind:"imported_json",import_id}`，客户端普通登记不得伪造导入原件。

登记只对显式来源做有限观测。普通大文件只stat，不为索引扫描/hash全部数据；若指标指定JSON pointer，显式小JSON单份<=1MiB/总<=4MiB，使用同一FD读取/解析/hash一次。存在且可读时pointer和声明value必须相符，否则拒绝该登记；缺失/不可读来源仍可作为unverified登记，不能标matched。不要求原件是成功作业。注册时内容核对只表示声明数值与当次源字节一致，不表示科学结论已验证。

详情分开返回不可变manifest、登记时source_observations与metric_evidence，以及当前job/source事实。原输出可能继续改变；受限原件入口比较登记时stamp，变化/缺失返回明确错误，不悄悄将新字节作为旧结果下载。历史源缺失仍保留结果。导入原字节与元数据同一SQLite事务保存，无双写文件发布窗口。

## API与存储

`_results/index.sqlite3`单独保存不可变登记、检索列和小导入原字节；复用SQLite FULL事务风格，不使用执行队列。无大文件复制、自动清理或后台导入。

- `GET /results?job_id=&game_id=&game_version=&device=&research_id=&direction_id=&outcome=&limit=&cursor=`：只读索引，不扫描原件。limit默认20、上限100；递增sequence倒序keyset，cursor绑定node_id、筛选和首屏上界。
- `GET /results/{id}`：完整登记+当前事实。结果非存在404；坏源作为分项状态，不让整个结果消失。
- `GET /results/{id}/sources/{name}`：明确选择的已绑定原件；job源复用受限FD/Range下载，external不可下载，import源读取原BLOB。下载不触发设备动作。
- `POST /result-imports`：`{schema:"phonefarm/result-import@1",import_id,adapter,filename,content,labels}`。import_id至多48字符；content为原UTF-8文本，解码后至多1MiB，HTTP JSON至多6MiB；filename仅显示，不作宿主路径。原文本不重新格式化；只在这一层计算一次内容SHA。
- `GET /result-imports/{id}`：固定ID找回导入状态和result_ids；`GET /result-imports/{id}/source`读取原字节。相同ID/同内容返回原记录，冲突409；导入与全部派生登记一个事务完成，不产生部分成功或执行任务。

结果详情响应`{schema_version:1,node_id,result_id,sequence,registered_at,manifest,provenance,source_observations,metric_evidence,current}`。provenance注明registration/import及import_id；current含job与sources状态。列表返回`{schema_version:1,node_id,items,next_cursor}`，item为result_id/title/job_id/labels/outcome/registered_at/provenance/metric_count/source_count，不传全报告。导入回执为`{schema_version:1,node_id,import_id,adapter,filename,size,sha256,registered_at,result_ids,source_url}`。所有URL不含凭据，Web补原target。

`source_observations`为数组，每项name/kind/status(available/unavailable/external)/size/sha256/reason；内部stamp不对外冒充资格。`metric_evidence`数组按指标index对应status(matched_source/unverified)、source_name/pointer/reason。`current`为`{job:null|{job_id,status,record,reason},sources:[{name,status,reason,url}]}`；source当前状态额外可为changed，job record沿已有job详情。只有详情做stat，不在列表扫描/hash来源。原件URL为token API根路径，网页转换为`/dash/api`后沿HGTarget授权与pin。

读取沿现有只读token/查看密码；新增写API要求admin/steer和显式目标pin，缺pin400。`/dash/api/results`、`/dash/api/result-imports`同服务；不用第二套网页目录。新能力缺席时客户端解释不支持。新登记/导入与同内容幂等返回均201；不同值409/result_value_mismatch，无效/不存在pointer400/result_pointer_invalid，原件变化409/result_source_changed。

## 适配与客户端

`doom-lr4-026-v1`只接`hgamey/lr4-026-original-native-owner-result@1`，生成import_id同名结果，三模块baseline/candidate/reduction与全帧/ROI分别保原pointer。显示FPS、功耗和每帧能耗的原null必须保留；限定两独立启动、6秒、FreeDVFS与GPU elapsed语义。外部旧job/path/SHA只是历史声明。

`gol-optimize-v1`只接`game_opt_loop/optimize_report@1`：父结果import_id保总报告与layers失败错误，再为每项archive生成`<import_id>.<序号>`，不能只保kept。A/B只从明确verdict per_metric字段映射；Entry未保存的source/运行版本/证据来源保持unknown。不把layers.eval_errors伪造成archive entry。保全部扩展证据原字节和pointer，不凭顶层免责声明补资格。两adapter均不重算统计。

CLI `pf-node results list [filters]`、`show ID`、`register FILE`、`import FILE --adapter NAME --import-id ID [labels]`、`import-status ID`，共享Target、NoRedirect，写入只一次；未知回执提示GET原ID。register文件内result_id固定，import无显式ID时在读取/提交前生成并打印。CLI JSON与网页共用响应，不输出凭据。

网页`/dash/results`提供筛选分页、登记JSON、小文件导入、详情原件；jobs详情加按job_id跳转入口，旧自动发现标明范围。未决登记/导入保存操作种类+原target+原ID，不保存File/完整报告，不丢响应后自动换ID。简单并列对照只展示原数值/条件，合同不全时说明不可自动比较，不造统计图或排行榜。

## 最少验收与后续

一条隔离HTTP/CLI无害任务链覆盖不可变去重、并发冲突、两页筛选、failed/unknown/旧任务可见、源pointer与原件/只读权限；一条小导入链使用指定Doom12.5KiB原JSON和代表性GOL fixture，核raw保真、全archive+layers错误与null。只在新下载入口测试其新增边界，复用已有SafeRoot回归。Web一次DOM+390px登记/丢响应找回/导入/未知/原件链。冻结后协议一轮、失败定向修；不重跑Doom实验或科学分析。

本子项完成后只关闭声明的源码/隔离范围；RESULT-02跨节点检索、研究与人类异步状态、所有producer身份贯通和真实部署仍按todo推进。

## 本批验证

112项协议首轮通过（49.507秒）；两条新HTTP/CLI纵向通过（1.589秒），包括一个无害worker、旧失败任务、不可变冲突、并发、分页、只读权限、原件Range及变更拒绝。唯一真实Doom小报告12,498B导入0.592秒通过：原文bytes完全一致，9项模块值/4项画质/3项null全部匹配，没有产生job或worker，也没有重算统计。GOL fixture保所有archive与父层错误。

Web七项定向DOM与390px浏览器通过；登记丢响应后原ID GET找回、不重发，再一次导入和带pin原文下载。390px无溢出、无page error。首次DOM替身hidden布尔转换失配保留，只修QA stub并续跑剩余分支，产品未因此改动。fixture PID67464和端口63424、browser/context均已关闭。证据qa/result-index-{validation,ui-validation,actual-doom-validation}.json留本机。没有部署、手机操作、历史大数据迁移或新性能结论；所有旧unknown恢复义务保留。
