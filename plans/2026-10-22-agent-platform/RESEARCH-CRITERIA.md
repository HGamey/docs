# RESEARCH-05：版本化目标与条件性点值判断

2026-10-25，已发布 harness d4bc398；五项必需 CI 成功后同 SHA 推 main 并核对远端。复用 RESULT-COMPARE 的明确选择、原件与描述关系；外部 Agent 决定解释和采纳。本批源码/隔离宿主/CI，不操作生产或手机。模块 ms 下降可以成为目标；画质可下降、功耗可选，不强制超过双线性。

## 保存目标

research.data 增加可选 `criteria`；未出现时不补键，保旧 operation 规范内容，显式 null 表示该 revision 无数值判据。沿原 CAS、历史、audit 和理由，不新增对象/调度器。新增能力 `research.criteria-v1`。方向的 next_experiment.research_revision 继续冻结当时目标。

非空 criteria 的精确结构：

- `schema: "phonefarm/research-criteria@1"`。
- `comparison`：原 `phonefarm/result-compare@1` spec，最多16KiB。第0列为作者声明的原游戏基线；同节点，允许同报告不同 condition。
- `baseline_role: "original_game"`：作者角色声明，不是平台核验原生渲染路径。机制对照可另列候选。
- `axes`：1–16项 `{id,row_id,label,unit,scope,role,prefer,required,rule}`。id唯一，row_id明确引用comparison.rows且不重复；label最多128字符；unit非空；scope为结果支持的已知scope；role为objective/constraint/observe，至少一个objective；prefer为minimize/maximize；required为布尔。
- `rule`：null或 `{kind,threshold}`。kind为absolute_improvement_min、relative_improvement_percent_min、candidate_min、candidate_max、degradation_max。threshold有限；improvement/degradation阈值非负。observe只能rule=null、required=false；required=true必须有rule。无规则只观察，不暗填阈值。
- `pareto_axes`：0–16个唯一axis id，必须已定义。空数组表示未请求Pareto，不自动选择。精确点值比较，不引入隐式epsilon或总分。

comparison.context 保存范围、假设和有意干预；research.evidence_refs 保存作者依据，research audit 保存作者声明与revision。不据字段相同授予科学资格。

判据必须能在候选结果尚未登记前冻结。保存时对comparison的不同result ID只读登记元数据：已存在者记录sequence/registered_at，尚不存在者记录declared_pending；不要求采样或先运行候选。同一comparison跨revision更新规则时沿用原绑定，不能把后来可见结果回填成当时已见。绑定存在observations并在research派生详情中可见，与原evidence_refs数组独立。已登记绑定若后来身份不符，assessment将该列标不可用；预声明ID可在后续查询解析到其首次不可变登记。缺值/未登记者不阻止其他轴或方向。declared_pending只说明保存时没有绑定到登记，两库读取不是跨库事务；它本身不证明候选登记严格晚于研究提交。

## 只读判断

`GET /research-state/research/ID/assessment?revision=N`（及dash镜像）必须显式node pin和正revision；不默认latest。CLI `pf-node research assess ID --revision N` 单GET、只读权限，核原ID/revision/cap，不写研究状态。无criteria返回409 research_criteria_required。

响应形状：

- `schema_version:1,node_id,research_id,revision,current_revision,audit,criteria,criteria_bindings`。
- `basis:"registered_point_values",comparability:"not_established",comparison:<共享comparison响应>`。
- `candidates`按comparison列顺序（包含基线）返回 `{column_id,axes:[{axis_id,row_id,value,unit,rule,status,reason,observed}],requirements:{status,required_axes,unknown_axes,failed_axes}}`。
- 轴status为satisfied/not_satisfied/unknown/observed。observed为用于阈值的数（无rule时为原值），不能计算为null。单位与scope必须匹配该axis声明；缺列/索引/null/单位范围冲突留unknown和原因。
- improvement按prefer定符号：minimize为baseline−candidate，maximize为candidate−baseline；relative再除正baseline×100。degradation为improvement的负值，允许负数表示改善。candidate_min/max只需候选有效值，其余规则需共享delta有效；不从未定义百分比猜值。计算溢出unknown。
- requirements仅聚合显式required规则：任一失败not_satisfied，否者有未知undetermined，全满足satisfied，无required为not_configured。optional规则不改变该汇总，但逐项展示失败/未知；这不是实验有效性、自动采纳或方向状态。
- `pareto:{status:"computed"|"not_requested",axis_ids,scope:"complete_value_subset",excluded_axis_ids,undetermined_column_ids,columns:[{column_id,status:"non_dominated"|"dominated"|"undetermined"|"not_requested",dominated_by,reasons}]}`。仅所选轴全部有有限同unit/scope值的列进入计算。各轴不差且至少一轴严格更好为点值支配；基线也参与。缺轴列保留undetermined，不能默默删轴或称全局最优。约束阈值与Pareto分别展示，不用一个替代另一个。

原result的outcome/source_outcome/claims与job/recovery不变；登记值差/阈值满足/Pareto均为作者所选范围和假设下的点值观察，不是统计显著、因果或显示FPS结论。当前来源状态可随读取变化，原criteria revision和登记内容不变。

## 网页

研究页提供同级可操作的判据编辑：选择/导入当前比较spec（结果页可链接带pin的研究草稿），以普通表单编辑轴、方向、阈值、required和Pareto轴，保存仍走原研究CAS与原operation；不能只给一个大型JSON文本框。研究详情显式查看所选revision的判断，显示基线、所有逐轴值/规则结果、缺项、有限子集Pareto和共享比较/原件入口。不能在浏览器另写算术或自动改变direction。

复用比较spec与现结果页深链，不再造另一套指标匹配；已有comparison矩阵可在原结果页打开。判据更新创建新revision，旧计划/旧判断仍能按原revision读取。CAS冲突保草稿，读判断不覆盖未提交表单。目标变化和断线保原ID/选择；按钮反馈明确。

## 最小验证

一条实际HTTP/CLI纵向：旧无criteria operation仍可原样找回；保存含预声明候选ID的目标，后来登记后按原revision判断；低ms/低PSNR在作者容忍范围、optional null功耗不抹掉ms，选入缺失power轴则Pareto未确定；新阈值不改旧revision/旧计划；原manifest与job不改写，显式pin/只读/单GET。

一条390网页主链：从比较选择建立轴/容忍值→CAS保存→查看精确revision判断与Pareto→调整质量容忍保存新revision→读旧revision仍保旧规则；保草稿与原件入口。协议整轮一次，失败只续相关分支，不重跑Doom或旧研究/媒体矩阵。

## 实际验证与范围

单条实际 HTTP/CLI 0.608s、123 条完整协议 63.279s 首轮通过。后续静态审查发现共享比较器的大整数差值会在有限性检查抛 OverflowError，改为返回已有 arithmetic_overflow；只续比较与判据两条受影响回归，1.262s 通过，未重复本地整轮。最终提交由五项实际 CI 通过。

390px 实际 NodeResearch 网页首轮通过：106B 合成原件、两份结果登记、研究 v1/v2 两次保存、三次精确 revision 判断（1→2→1），52 个 HTTP 请求（49 GET、3 POST，其中一个解锁）。质量容忍从3改为1，旧版仍保留3；缺功耗显示未知，加入 Pareto 时保持未确定；读旧版不覆盖未提交草稿。页面宽390、详情356，无溢出或页面异常，原件/比较链接保留。浏览器、fixture PID85911/端口61678已关闭；无设备、作业、生产操作。

旧 criteria 缺席/null 的浏览器路径、pending body/迟到响应隔离、缺能力、文件导入及桌面深色仅静态检查；预声明跨登记与旧 operation 的兼容由协议覆盖。点值满足规则不代表实机性能、统计显著或科学资格。原始记录位于本机 qa/research-criteria-*，首轮与后续修复分开保存。
