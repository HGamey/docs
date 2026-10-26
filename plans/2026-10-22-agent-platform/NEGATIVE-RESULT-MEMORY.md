# 负结果查询与已否决方向接续

METHOD-08 的基础能力已存在于结果索引与共享研究状态中。2026-10-26 对当前 harness main `f78554a` 和既有验证进行了只读核对；本批没有再造存储或 API，也没有重跑测试。完成范围是已知节点/研究内的显式查询与持久历史，部署后的实际接续仍待验。

## 接续合同

外部 Agent 先读绑定节点上的研究与方向，遍历分页；可按 `state=no_gain` 或 `state=rejected` 筛方向，再读详情和历史中的理由、证据与适用条件。结果索引可组合 research、direction、game/version、device 与单个 outcome 查询，网页与 CLI/API 使用相同过滤。跨节点结果搜索只访问明确选中的已登记节点，各节点分页和失败独立返回。

`failed`、`not_run`、`unknown` 与 `no_gain`、`regression` 分开保存。执行失败或没有找到注册结果不等于已证明没有收益；旧文件可能尚未登记。方向转为 no_gain/rejected/deferred 必须有理由，更新经 revision CAS，历史记录保留数据、审计理由和证据引用。重新开放方向后，旧否决仍在 history，当前状态筛选不会同时列出其旧状态。

游戏版本、场景、方法或判据改变时，外部 Agent 可以给出新理由和 revision 再评估。平台不自动禁止重验，也不替外部 Agent 选择研究优先级。结果登记与方向更新是独立持久操作；接续时先找回未决 operation/request ID，检查已登记但尚未关联的证据，不把两次提交说成一个事务。

## 现有证据

- `node_results.py` 的 outcome/组合过滤与不可变登记；`result_client.py`、`results.html/js`、`node_result_search.py` 使用同一合同。既有 `test_node_results.py` 实际登记和查询 failed，验证缺失来源、分页、并发登记与冲突；原验收为 RESULT-INDEX。
- `node_research.py` 的状态理由、证据绑定、CAS、不可变 revision/history；`research_client.py` 与网页共用过滤和历史。既有 `test_node_research.py` 经实际 HTTP/CLI 验证并发、服务重启与历史找回；原验收为 RESEARCH-STATE。
- `skills/hgamey-research` 的接续指引已要求复用先前证据、保留负结果，条件变化才重新验证。该 skill 本次作为产品源码审查，没有用于操作线上节点。

既有测试覆盖通用持久化/过滤/历史及显式 failed 结果，没有单独声称每一种负状态均完成浏览器点击或 no_gain/rejected 独立端到端测试。当前枚举和共用读写路径已核对，隔离宿主证据不能替代真实部署。

继续开放：部署后换 Agent 实际接续、跨节点研究状态复制/全局历史、不同命名假设的语义去重、旧原件自动索引。结果联邦查询不代表研究状态已复制。以上分别沿 DEPLOY、FUTURE 与既有导入待办推进。
