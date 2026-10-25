# BASE-01：旧分叉与工作树改动的去向

2026-10-25。逐项回收，不将旧历史分支整体推回main。旧研究工作树、实验原件和未提交内容均保留。
固定比较PF旧9d92712→当前316a3e0；GOL旧checkout547462f、迁移runtime pin6f713ad与当前3160e41分别记录。
本次是源码/历史映射和有限离线验证，不证明旧部署仍在线。

## 已确认集合

| 来源 | 实际集合 | 去向 |
|---|---|---|
| 旧harness dbb7eb6→d5c9b86 | bfa3fd5、ac16c9d、d5c9b86，恰三提交七文件 | 唯一owner/raw包锁→2b3c741；fleet队列→947a4b9；SHA缓存/partial/serial→12eda9a，后续合同继续演进 |
| PF旧HEAD独有提交 | 与316a3e0共祖818ff45，旧侧仅五提交；早期198不是当前待回收数量 | 三份游戏规格变更按当前prepare/deploy/scene职责映射，旧研究证据保留；两个helper明确研究保留，见下节 |
| PF未提交src/perfsrc.rs | BatteryService前后供电过滤、离线sidecars、相关断言 | 适配当前FpsScope后发布a03a6f9；24相关测试与签名CLI小数据链通过，见HARMONY-SUPPLY |
| PF未提交README与SPEC_PERF_SOURCE | 2026-09-24 nova16有限冒烟自报 | 保留在docs SPEC_PERF_SOURCE第9节，区分历史作者记录与本轮离线验收；不覆盖40行代码README |
| GOL未提交harness/mod.rs与main.rs | 状态过滤与HDC-only资格，另外为格式和旧说明 | 按GOL-DEVICE-ELIGIBILITY适配三平台枚举和显式gpu-op目标；已发布761226c；3纯测试及bin编译检查/code-only通过 |
| GOL未跟踪harmony_touch_probe.rs | 8,528B直接hdc研究helper | 原地研究专用保留，不搬为未遵守当前owner/恢复合同的公共入口 |
| 旧docs4051840 | 27文件 | 26同路径mode/blob OID完全相同；唯一README保留旧四计划入口并演进索引，无须重并旧分支 |

旧harness附属差异也有明确去向：argv/label/raw-exe校验保留；安装不隐式授权/改音量保留；
无SHA/无size旧缓存兼容明确收紧为必须SHA/size/serial，已校验旧basename导入保留。规格文档迁至docs，
不能机械cherry-pick已退役目录和旧默认行为。

PF三次游戏规格提交为dd218bf、2c88964、5560fe1，原文件留研究树；GAME-DISTRIBUTION、
GAME-DEPLOYMENT、GAME-SCENE分别承接SHA与owner、落盘/UID可读、受限回放职责。
同一主题已实现不等于新隔离验收证明旧实机素材可用。3141a32的六个Doom适配/证据文件亦保留原位。

## 审计边界和原始索引

PF可见提交集合已用共同祖先核定；docs浅克隆缺祖先不作为内容丢失证据。
只比较树OID和限定小源码/文本，不读取PPT/图片/大型实验原件，不计算全历史patch-id。
一次git ls-tree -l在promisor仓可能触发隐式补对象，已中断并记录；之后只读树OID且禁传输。
未运行旧研究根git status、reset、rebase或修改其refs。

本机qa/base-history-map.md、base-history-{pf-metadata,docs-metadata,tree-map,small-source}.json、
base-history-docs-readme.diff、base-pf-three-user-changes.patch、base-gol-harness-map.md与
base-gol-device-contract.md记录原始范围/逐hunk处置。它们是限定集合审计，不将未知个人分支
自动宣称已全部合入。两个helper归宿与GOL交付均已核定，本次已确认集合全部有去向，BASE-01收口；未把研究保留项标作产品化。

## 两个helper的明确处置

- 9d92712 的 artifact_transport.py 是有 encoded/raw 双 size/SHA 和显式限额的单member gzip纯解码。
  只在限定搜索范围内找到测试调用，规格中的未来worker不是已实现消费者。当前tar输入、
  目录导出和单文件Range有各自合同，不能为复用强行改变它们。保留原提交/helper/测试/规格，
  不新增生产入口；以后出现真实压缩原件消费者再定点接入。
- 3141a32 的 consumer_window.py 有真实Doom离线差分与反例消费者，用于核对已有软件窗口结果。
  它不生产原采样、不独立证明完整性，合格时仍不声称hardware display FPS。
  保留helper、测试、规格、adapter和原证据链，不重写冻结实验解析器。METRIC-OWNERSHIP-01 /
  RESULT-07继续开放，可复用此算法但必须另有实际adapter、同钟/损失/生命周期证据。

本机qa/base-helper-disposition.md逐行记录调用与三个PF工作树diff的去向；没有第四项独立遗漏。
这些决定不是将未产品化研究标成产品已实现，而是明确该资产的保留位置、用途与后续依赖。
