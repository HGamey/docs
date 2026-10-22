# 近期旧 PR 合并专项审计

日期：2026-10-22。范围：HGamey 组织 2026-10-15 起的合并记录，重点审阅 10-22 整合 10-11/12 草稿的变更。以下合并时间均为 UTC（日本时间加 9 小时）。

结论：确实有旧草稿集中进入 main，且找到两项具体代码问题。不能把“旧”本身当作缺陷，也没有依据整体回退。应保留已合入功能，单独修复灰档执行前置条件和动态层构建输入，再明确当前不可用的 copyprobe/upop 能力。

本次仅查询 GitHub API、读取源文件和 Git 对象；没有操作手机、访问运行后台、运行/新增测试、编译、修改代码工作树或提交。只新增本报告。另一个任务正在运行的全量测试不作为本审计已经复现的证据。

## 合并了什么

| 整合入口 | 合并时间 | 旧草稿与实际规模 |
|---|---|---|
| [phonefarm #63](https://github.com/HGamey/phonefarm/pull/63) | 2026-10-22 09:34:39 | #17、18、19、20、21、22、23、24、25、26、27、33、34、36、39 及三个集成分支；289 个文件，+57,153/-836 行 |
| [game_opt_loop #86](https://github.com/HGamey/game_opt_loop/pull/86) | 2026-10-22 08:24:14 | #26–35、37、38、48、62、63 及 blackbox 集成分支；398 个文件，+43,209/-218 行 |
| [refbench #6](https://github.com/HGamey/refbench/pull/6) | 2026-10-22 07:45:32 | #2、#3 的受控 LR/HR 采集与完成标记；17 个文件，+1,801/-13 行 |
| [docs #5](https://github.com/HGamey/docs/pull/5) | 2026-10-22 07:40:20 | 10-11 创建的系统交接与历史进度 |
| [docs #20](https://github.com/HGamey/docs/pull/20) | 2026-10-22 09:33:52 | 三个代码仓旧草稿的规格文档迁移 |

有些旧 PR 被 GitHub 自动标为 merged，有些仅被 closed 并由整合 PR 带入。比如 phonefarm #27/#33/#34/#36/#39、GOL #62/#63、refbench #2/#3 的 `merged_at` 是空值，不能只搜索 `is:merged` 就漏掉它们，也不能把它们的关闭时间称为单独合并时间。

harness-ops 此期间最近合并的是 #28（代码仓文档规则）与 #29（分支规则），没有上述类型的旧业务代码整合。组织搜索未发现 sr_loop、knobs、.github 的同批旧代码合并。

审计固定版本：phonefarm `0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff`；game_opt_loop `d090d27ceb62c8dc15331e5121d1cf0ffd048ad0`；refbench `b53c7856825b2a0de9358677d2836ecd04ad73e6`；harness-ops `07b0aa7beee4f256834b752fa00273a5e6868560`。这些 SHA 用于定位审阅版本，不是另加的运行验收要求。

## 已确认问题

### P1：所有 gray eval 都在候选采集前被必然阻断

来源：[phonefarm #63](https://github.com/HGamey/phonefarm/pull/63) 合入旧 [#18](https://github.com/HGamey/phonefarm/pull/18)。旧 #18 自身已有此逻辑，不是本轮开发引入。对比 #63 的第一父提交 `3a9e671`，此前没有这个候选前门禁。

证据链：

- [`src/eval.rs:2303–2309`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/eval.rs#L2303) 在冷机/基线后，明确将 `facts.device.control_verified=None`，然后切为 `Stage::Experiment` 并要求 `experiment_allowed`。
- [`src/qualification.rs:416–420`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/qualification.rs#L416) 对所有非 Probe 阶段检查 control。None 得到 unknown。
- [同文件 :502、:525](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/qualification.rs#L502) 要求所有检查 pass 才允许 experiment。因此无论其他条件如何，gray 分支都会在进入 A/B 循环前返回。
- control 的实际命中证据要到 [`eval.rs:2528`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/eval.rs#L2528) 才写入，而这里已不可达。

影响：通过 GOL/phonefarm `eval kind=gray` 运行 loadop/storeop 等候选，可能先花时间完成前置工作与三轮基线，随后固定拒绝所有候选。这不是偶发温度失败或当前手机不支持。

修复建议：把“允许采集候选证据”的前置条件与“证明改动实际生效、允许形成结论”的后置条件分开；或者在前置阶段复用真正执行过的挂层探测证据。不能直接把 None 改成 true，也不要一并取消恢复和原始结果记录。修复后仅补一个覆盖该实际执行分支的验证即可，无需扩展大量独立门禁测试。

### P2：动态层 builder 没有接入 main 新增的 NPU 翻译单元

来源：[phonefarm #63](https://github.com/HGamey/phonefarm/pull/63) 把旧 #20/#21/#24 的动态工件 builder 与更晚的 main 层源码组合。它是新旧接口整合遗漏。

证据链：

- 合并前 main 已有 `npuprobe_on_present`，合并后的 [`vk_layer_refknobs.cpp:2712、2726`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/knobs/gray/layer/vk_layer_refknobs.cpp#L2712) 仍声明并无条件调用它；定义在独立的 `npuprobe.cpp`。
- 普通构建入口 [`build_layer.sh:41–45`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/knobs/gray/build/build_layer.sh#L41) 正确编译两个 .cpp 并链接 `-ldl`。
- 动态入口 [`grayartifact.rs:744–753`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/grayartifact.rs#L744) 的输入复制没有 `npuprobe.cpp`；[:819–829](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/grayartifact.rs#L819) 只编译 `vk_layer_refknobs.cpp`。此函数不可能从该构建列出的翻译单元中获得定义。
- #63 集成修正仅给宿主测试补了该源文件：[`native_artifact_host.rs:65–68`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/tests/native_artifact_host.rs#L65)，生产 builder 没有同步。
- GOL 的旧 [`candidate_prepare_native.rs:5–10、97–119`](https://github.com/HGamey/game_opt_loop/blob/d090d27ceb62c8dc15331e5121d1cf0ffd048ad0/src/candidate_prepare_native.rs#L97) 也只认识 legacy4/ownership5 输入集合。单边增加 PF 构建输入后还需同步这条可选路径，否则新的输入/工件会被它拒绝。

影响：动态 `gray-artifact build` 缺少必需定义，视链接器配置会链接失败，或生成带未解析项目符号、无法在普通游戏进程加载的 .so。当前它本来标记 `deployable=false`，所以不把它夸大成已经发生的真机故障。源码遗漏已确认；未重新跑 NDK 编译或 Android 加载。

修复建议：让普通层构建和动态 builder 共用实际输入/链接清单；显式选择是否带 NPU 模块，并保证调用与定义一致。同步 GOL 消费端的可选接口。只做一次实际动态构建及产物链接/加载依赖检查，不用靠增加摘要字段或重复 host fixture 证明兼容性。

## 明确行为变化，不能当作仍可使用的能力

[旧 #23](https://github.com/HGamey/phonefarm/pull/23) 的设计明确停用了原有启发式 copyprobe/upop 注入，这一点不能误称为意外合并冲突。但进入 main 后会影响继续做旧实验的人：

- [`dispatch_ownership.h:491–498`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/knobs/gray/layer/dispatch_ownership.h#L491) 的非 host-fixture 构建无条件拒绝注入；Android 又明确禁止 host-fixture 宏。因此当前生产层 copyprobe/upop 不会执行目标算子。
- [`graylayer.rs:328–339`](https://github.com/HGamey/phonefarm/blob/0cd6571394f2421f2e01f5cb18c9fa5c246ea9ff/src/graylayer.rs#L328) 仍暴露两个模式并以写属性/挂层完成为命令成功。属性成功不能代表算子运行。

建议在能力发现和这两个命令入口明确返回 unsupported/未接入原因，避免花完整实验窗口测到 A 对 A。若要恢复，需完成真实生产来源/布局/目标合同；不要为保留旧菜单而恢复有资源生命周期问题的旧注入，也不要启用测试宏。loadop/storeop、只读 frameaudit、独立 npuprobe 不因这个函数而被证明失效，不能扩大影响范围。

## 未找到回归证据的部分及边界

- 没有发现 #63 覆盖 main 后来的 `devlock.rs`、whitebox、fleet、platform、device、`npuprobe.cpp` 或普通 `build_layer.sh`：上述文件相对 #63 第一父提交无内容差异。两项问题也因此不能概括为“整合把新代码全覆盖了”。
- GOL #86 的 campaign 是显式选项；与 `--screen`/`--extra-device` 的限制在 `cfg.campaign` 分支内部（`optimize/mod.rs:286–316`）。这是新增模式限制，没有证据表明普通 optimize 被强制变成同步等人的流程。当前平台可继续以外部 Agent 为主，不必把 campaign 设为必经入口。
- refbench #6 的配对捕获默认 off，on 时标明 `quality_only`/`performance_eligible=false`。本次抽查没有确认普通渲染路径回归；没有重做 GPU 真机验收，不能据此保证整个新捕获链可用。
- docs #5/20 保留 10-11/12 的历史记录，其中“当前均为 draft 未合并”如今过时，但文件开头带历史更新日期。应在入口加历史快照/已整合说明并指向 10-22 当前计划；不要覆写旧实验失败与证据边界。没有发现这些文档改变运行代码。
- 当前开发批次基于已包含 #63 的 phonefarm main；harness-ops 通用任务网页/API 没有被这批旧 PR 覆盖。修复以上问题应另列小批次，避免混进首批任务发现/查看功能。

## 验证证据应如何理解

GitHub #63 合并提交上 `phonefarm`、`gray-layer`、`release`、`code-only` 均成功；但当前 workflow 的 phonefarm 步骤是 `cargo build --release`，gray-layer 调的是普通 `build_layer.sh`。这些检查不会触发 gray eval 的候选前分支，也没有跑动态 gray-artifact builder。CI 绿与上述缺陷可以同时存在。

GOL #86、refbench #6 的合并提交只有 `code-only` check；PR 描述另记了本地测试，二者应分开。GOL 描述也承认 `tools/live_dashboard/test_frontend.cjs` 引用不存在的 `renderToday`，未在本次修复；这是一项已有验证缺口，本审计未据此推断运行网页必坏。

本审计不是 10 万新增行的完整认证；重点覆盖合并边界、常用 eval 前置路径、原生层构建依赖、可选 campaign 和当前批次冲突。没有依据撤销整批合并，也没有依据直接更新实验机器并声称全部兼容。
