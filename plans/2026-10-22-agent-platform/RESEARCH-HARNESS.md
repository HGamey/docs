# RESEARCH-03：外部 Agent 参考流程

2026-10-25，已发布harness de018584f6ba0e133850c4dd2302931c0eb124cb、PF af96ae6106805278fb3c0c25bd515da49b897242。仅源码、隔离宿主与 CI，不接生产/真实手机。复用 [RESEARCH-STATE](RESEARCH-STATE.md)、[RESULT-INDEX](RESULT-INDEX.md) 与 [HUMAN-EVALUATION](HUMAN-EVALUATION.md)，不新增协议、内置 Agent、任务调度器或模型调用。

## 交付

harness 的 `skills/hgamey-research/SKILL.md` 是与聊天工具无关的参考入口，按需加载一份命令/接续示例。它接收显式 node URL/ID、凭据来源路径或变量名、research ID、用户授权与运行器边界、可执行 recipe 和已有未决 ID。敏感值不写共享研究、日志或交接。

外部 Agent 读当前研究/全部分页方向和反馈，核原问题版本与证据；明确 adopt/defer/reject 理由，依赖满足/豁免另作记录，回答/关闭/超时不自动解阻。只阻断依赖该项的方向，其余按收益、成本、前提及证据缺口排序。GOL 是可选方法，原生游戏/模块 ms/画质容忍度按具体目标判断，不强制战胜双线性。

选中方向先以当前 revision CAS 保存完整 data 和冻结 next_experiment（title/plan/research_revision），在 plan 中记录固定 job request ID、recipe/输入/输出/判据。随后用现有 detach job 提交，续查原 ID，显式登记小结果和原 job/source，再更新方向证据与优先级。ready/verified 只是元数据声明；入队、退出、宿主完成、保存、设备恢复和科学收益分别判断。失败、无收益和未知均保留，缺值为 null。

并行方向可继续；仅剩运行中/阻断方向或外部回合边界到达时保存可携带接续块。块包含目标/对象版本、已处理事件游标、未决操作/request/job/result/import ID、下一只读查询与局部理由，不包含私有聊天历史或凭据。事件仅元数据，无 job 完成推送或自动唤醒；本批不承诺停掉运行器后继续执行。

纠正现有 hgamey-remote/phonefarm skill 的过时权限、远程原语与 GOL 强路由说明；以实际 help/能力为准，不发布未实现的 shell/install/push/pull 或 TTL 自动解锁承诺。

## 验收与限制

一条真实隔离 HTTP/CLI 链，已有 PF frame-audit analyze 使用极小、明确 synthetic 的合法输入；A 等人、B 离线执行并登记、另一客户端读取原反馈并显式处理。没有真实性能收益结论，不重建 PF、不重复协议全套或媒体验收。技能按 skill-creator quick_validate，另核具体命令与当前 schema；CI 依仓库已有 required 检查。

CAS 是合作约定，不是方向领取锁；direction plan 与 job/result 之间无跨库原子事务。持久研究不能找回从未保存或携带的客户端 ID；没有 ID 时先读历史，不重新猜测提交。未支持的宿主命令不能用 device_access none 绕过。研究状态仍单节点，跨节点 URL 改写不等于迁移。

后续 RESEARCH-04 负责事件 watch/consumer/恢复空闲；RESEARCH-05/RESULT-07 负责判据和生产者身份贯通。预算/优先级不是硬调度或计费。

## 本批验收记录

三个 skill 用 skill-creator quick_validate 通过；默认 Python 首次因缺 PyYAML 未进入校验，随后用已有 bundled Python，不安装依赖。远程 CLI help 与截图元数据位置定点核对；历史五个 docs 链本地未展开，未声称已验证线上可达。

实际隔离 HTTP/CLI 与既有 PF runtime 链使用8,830B synthetic 输入，输出 JSON4,032B/Markdown2,375B。A等待时B完成唯一实际分析，显式登记synthetic/unknown/metrics=[]；第二CLI客户端读原request v1后明确adopt+resolution，关闭/答复不自动解阻。累计56 CLI/111 HTTP、0设备传输，58个已知PID退出；服务关闭，临时凭据移除，未生成设备lease根。

首次夹具误将普通环境放入shared-profile字段，使准备读不到共享GID，HTTP404后留下preparing；正常服务重启把原job15e120ab89ba收口为failed/not_started。原研究/节点/ID和失败原件保留，再CAS冻结新计划，唯一执行jobc5ed605d7d50完成。未从头重建研究，也未重放旧请求。QA反映真实失败，不改成首过。

索引：本机qa/research-harness-validation.json、research-harness-run-1/{summary,continuation-summary,facts,handoff}.json及research-entry-skills-validation.md。两个CLI客户端和合成反馈不是独立Agent行为评测、自然人评价或实机性能验证。本批仅skill，未本地重跑整轮协议/重建PF；发布仍执行仓库required CI。

发布时harness五项actual required CI、PF code-only均成功，同SHA直推main并核ls-remote。PF只改skill，既有cbc53e4可执行程序未重建；不能把新的源码HEAD当二进制来源。下一[RESEARCH-04](RESEARCH-WATCH.md)补有界事件等待与位置接续。
