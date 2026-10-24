# RESULT-03：显式候选指标矩阵

2026-10-25，实施合同。仅源码、隔离宿主与 CI。先交同节点2–4列显式数值对照，随后 RESEARCH-05 复用同一选择/范围加目标判据与条件性点估计 Pareto。原始登记值可以有用，不要求先完成 RESULT-07 才显示算术；字段一致也不能升级为已证明科学可比。

## 输入与接口

新增 `results.compare-v1`。`GET /result-compare?spec=<JSON>` 及 dash 镜像共用只读派生模块；显式目标pin必需，沿现查看权限，不产生job、operation或新数据库。CLI `pf-node results compare FILE` 读取至多16KiB UTF-8 JSON，单次GET，输出同一JSON；不自动选结果/指标、翻页或重试。

spec 的 schema 为 `phonefarm/result-compare@1`，含：

- `node_id`：当前明确节点，与目标pin相同。
- `columns`：2–4项 `{id,label,result_id}`；id唯一，result_id可重复（同一旧报告可能含baseline/candidate多个condition）。第0列是明确选择的基线，不从数值自动挑最好者；label由作者给出，不自动宣称原生路径。
- `rows`：1–16项 `{id,label,metric_indexes}`，id唯一；metric_indexes长度等于列数，每项为0..255整数或null。明确null表示该列未选此轴，不用相似指标名猜配对。
- 可选 `context`：`{scope,assumptions,interventions}`；scope人读文字，后两项文字数组，都是作者声明，不是系统核验事实。用于说明固定比较范围和有意改变的渲染/模型等条件。

不自动补造测量合同或默认阈值。保留可导出的spec，暂不新增comparison写库；之后research revision可冻结同一spec。新比较共享链接需带node_id和spec，不含凭据。跨节点详情代理尚缺，不能只改URL或按同名result混成同一基线；远端search原入口保留。

## 派生事实

复用 NodeResults.get 获取最多4个不同的原结果，去重同result读取；返回每列的明确node/result身份、不可变manifest/registration与当前source/job事实及回源链接。不可查列逐项标记，其他列继续，不因一个404抹掉全表。禁止扫描旧目录/下载大原件/重新分析统计样本；现有结果读的来源核验边界不改变。

每行按原metric_index返回原metric及原metric_evidence。索引越界、未选、结果不可查、value=null分别有原因，不填0。名称、condition、unit、scope、method、window、denominator、power_rail、image_domain、region和source均可下钻。

相对第0列逐项展示描述字段的相同、不同或未记录；双方null仍为未记录。condition、版本/模型等可能是有意干预，显示实际差异而不要求所有字段完全相同。字段匹配只叫descriptor关系，不叫qualified或统计验证。

对显式所选有限数值，在unit非空且相同、scope已知且相同的前提下给出 `candidate - baseline` 的“登记值差”；百分比为 `(candidate-baseline)/baseline*100`，仅baseline>0时定义，非正基线保绝对差、百分比null与原因。溢出为null，不产生Infinity。缺值或unit/scope冲突时不计算相应差，原值仍展示。其他描述缺口、历史source匹配或当前不可查单独显示，不擦掉已登记值。没有FPS换算、CPU/GPU相加、补采功耗、自动gain/PASS或全局排序。

响应明确这些是选定登记值的算术观察、比较资格由作者结合原证据判断。原result的outcome/source_outcome/claims不改写；原job恢复屏障与性能数字分开显示。返回当前来源状态的时间性也不修改原manifest。

## Web

在现 results 页增加比较入口，复用已读列表/详情选择列和精确metric索引，允许同一结果不同condition。提供基线/候选标签、行标题与指标选择、缺失项、比较范围声明和导出spec。只GET派生结果，按钮有等待/失败/重试反馈，不在浏览器重复算差或按名称自动join。

桌面用可读矩阵，窄屏用每指标的列卡片或限定表格滚动，避免整页横向溢出。基线身份、各列来源/缺失、登记值差与限制一起呈现，能下钻原结果/来源。深链刷新核原node/spec；错误保选择，不自动转投节点。原单结果页和远端检索保留。

## 验收

一条实际HTTP/CLI链：2个小结果、3列（同result选两个condition）/若干显式行，含模块ms、画质下降、null功耗、相同name但scope或unit不同、零基线、越界与不可查列。核数值算术/原字段/部分失败/原manifest不变/只读权限/pin、CLI单GET且不自动注册。复用小结果夹具，不导入大报告、不重跑旧真机或媒体矩阵；协议整轮一次，失败只续相应路径。

一条390浏览器链：选择明确基线/候选指标→比较→看缺失与描述差异→下钻原件→导出/深链重载保持身份与选择。先计划后执行，不反复全页遍历。先不声称有科学资格自动验证、统计显著或完整跨节点比较；RESEARCH-05将消费此矩阵做用户配置的条件性点值判据/Pareto。

## 发布证据

2026-10-25发布harness `e9d0e1260c154650cbec7f6bc033c3aa34fac18c`，五项实际必需CI成功后同SHA直推main并核远端。单实际HTTP/CLI首过0.593秒，122协议首过62.057秒；真实NodeResults/HTTP浏览器390px通过，3列3行、同报告两condition，31次HTTP全GET、2次compare、0设备/producer job/写入。106B原件和导出spec保持原样，固定pin深链接续，scrollWidth390、pageerror0。

QA首启动初始化顺序错误与附件下载误作导航均保留；第二browser context只续原件/导出/深链余项，不重播已过表单流程，产品源未改。PID84101、59880端口与两browser context关闭。旧能力/迟到回应/基线重排/容量/20秒超时只静读；本批未重验桌面/暗色主题、没有设备或科学收益验证。证据索引本机 `qa/result-compare-{validation,publish,checks,ui-validation}.json`；原报告与所有unknown恢复义务保留。
