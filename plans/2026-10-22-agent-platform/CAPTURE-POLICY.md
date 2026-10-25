# CAPTURE-POLICY-01：有限观察与单步输入策略

2026-10-25，实施合同。修复既有capture/QuestConfig参数未生效的缺陷；不运行游戏，不重做onboarding或旧恢复矩阵。

## 行为

新capture --ready-only请求仅有限观察已有画面：跳过ensure_game_ready，不唤醒/启动/进门，不调用step或BACK。
默认目标1；显式--frames非1拒绝。最多8次截图，命中横屏探索态与两个HUD即保存原PNG、成功；未命中返回
ok=false/code1、frames=0/steps=0、stop_reason=not_ready及实际观察次数，不进入巡航。新请求shutdown=false。
普通capture默认200帧、auto、原退出政策不变。两路径共用原worker完成/父finish，不早exit绕过恢复。

共用QuestMode精确解析auto/navigate/dialogue/interact；capture保持原公开auto/navigate/dialogue，quest保留interact。
无效模式/quest缺mode值在构建设备之前拒绝。模式约束step；普通自动prepare仍可能启动/输入，像素误判或走位
触发剧情也无法被该策略排除，不宣称navigate整调用绝不推进剧情。

| 模式 | 单步允许 |
|---|---|
| auto | 原策略；auto_choice=false不得确认Choice，也不能用capture BACK绕过 |
| navigate | 探索摇杆/巡航与攀爬脱困；无对白/分支/标题/加载A，无明确或盲X |
| dialogue | DialogueText A，以及auto_choice=true的Choice A；不走位、不交互 |
| interact | 只在明确InteractionPrompt发送X；不走位、不选对白 |

capture的BACK仅auto且同一状态允许step时保留。QuestConfig.auto_choice沿现公开字段生效，不另造CLI开关。
通用GenshinPlugin intercept是另一个入口，本批不冒称它也使用QuestConfig。

## 恢复和证据

Worker.open只建主机状态，HID惰性创建；ready-only采集阶段不创建它。整条调用仍持原lease且父finish写回/核验
原stayon，所以不称零设备写或无恢复义务。新观察请求不force-stop/sleep；旧session恢复继续遵守已保存shutdown，
不回填新默认。game_state_restored=false、pending/unknown与worker完成证据保持原规则。

单轮相关参数与实际动作分派测试：有限观察用原capture流程+原PNG读取回调，fake QuestDevice的任何写入直接失败；
命中/未就绪均零step，覆盖默认200造成的旧fallthrough。插件一个合并动作记录测试核各mode和BACK规则。
不另造设备框架，不重跑完整harness或真机矩阵。帮助/skill同步，Web通用job仍使用相同参数与报告，无独立算法。

## 交付（2026-10-25）

PF 4eacd579cf34e8e4afb9dcc0e165e9921a42a868 经code-only成功后同SHA直推main并核对。
唯一一次签名测试进程运行4项受影响测试，全部首过（测试5.17秒，进程5.65秒）；
实际capture流程使用拒绝任何写入的QuestDevice与小合成原PNG，验证8次未就绪/一次命中均零step/BACK、
正确退出和原字节保存。一个动作记录测试验证mode分派、navigate无盲X、auto_choice及BACK策略。
参数覆盖普通默认、新ready-only默认/显式帧数和非法mode。没有设备、实际quest、生产或旧恢复矩阵重复。
Worker/session恢复代码未改，旧session仍按原记录执行；本轮不新增真机分类/剧情或恢复保证。
原始日志与发布收据见本机qa/capture-policy-validation.json及capture-policy-*。
