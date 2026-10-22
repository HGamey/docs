# 持续会话的显式供电条件

状态：2026-10-23，03B4C2B2实施合同，尚未实现/验收。承接 [GRAY-CONTROL-SAMPLING](GRAY-CONTROL-SAMPLING.md)。本批只源码和宿主替身，不部署、不操作手机。

## 固定政策与原值所有权

新会话CLI增加 `--power-policy observe|battery`，默认observe。政策在会话创建时固定，不能对已运行会话切换，也不从现存停充文件自动推断政策。网页在已有任务提交表单提供小型灰层会话模板及同一selector，只填入可审阅argv，最终提交仍走同一任务/幂等流程；有未决提交时不覆盖其body。旧PF不认识参数时明确拒绝，网页不凭模板存在声称旧版本已支持。

observe沿现有只采集行为，不取得/修改充电控制。battery在首次sample工具部署及round脚本准备后、几何/预算最终核验前，复用ChargeSuspend准备一次，之后跨round保持。创建前先做宿主侧原记录准入；foreign/损坏/来源未知记录拒绝，不接管或删除。持lease后仍复核。已实测放电走observed_no_mutation，不写控制节点。原值、控制节点、实验值及独立record原件先绑定相同serial/token/session，再允许写入；沿现有候选失败回滚和回读协议。

新资源父用 phonefarm/gray-control-resources@3，由base@3明确绑定。新程序兼容旧父@1严格idle和@2有限输入，二者仍无供电条件化，不能升级政策；旧程序拒绝新绑定。sample动作能力仍2（有限输入语法不变），新版descriptor验证必须接受@2/@3的finite能力，不能误拒绝旧B1会话。

每轮battery采样前核绑定记录和实际放电，采样后再观察。before失败不放行worker；after失败保留已有trace/env/输入原件及独立失败原因，不抹掉可核的帧/模块测量。充电是否改过、实际是否放电和功耗数据是否有资格是三个不同字段。旧全局bypass文件不得作为本会话电池功耗证明。风扇、频率本批不改。

## 恢复与失败

确证全部登记组消失后恢复本会话绑定charge并回读，再走严格自然完成/远端关闭验证、trace删除、灰层/12属性/目标停止、父报告与complete。恢复供电不等于整个实验恢复完成。

当前base的sticky stop仍保留：采集后健康检查非零时，当次finish可能在到达charge恢复前就停止。此时公开required/charge仍保留，使用原context recover重新核组并恢复绑定charge；缺自然完成时后续严格检查仍保持required。不得为归还供电清除sticky/pending、不伪造自然完成，也不以进程已消失自动complete。已恢复的charge与失败报告分别留证。

## 公共状态与数据

会话descriptor新增 `power`：policy、stage、control_modified_by_session、initial及last_observation。只发布白名单观测字段（battery status/current_ua/capacity_pct、on_battery、observed_at等），不发布lease token、内部record路径或完整binding。旧descriptor字段缺失显示未知。stage沿现有not_started/observed_no_mutation/recorded/suspended/restoring/restored；observe为unmodified。sample回执和run.json新增power对象，含policy、before、after、error；每轮power.json保存同一观测，失败也保留。恢复后的descriptor从持久父状态刷新。

原whole-sampler功率保持原口径。新增同期trace窗口 `trace_energy_j`、`trace_power_w_mean`：复用sample_window与capture uptime元数据，只在完整采样、每个样本电池放电、合法功率范围、sampler成功且有效点夹住窗口两端时计算。统一用同一电池V×I数据序列，窗口边界线性插值后梯形积分；明确这是采样积分估计，不外推缺失端点。记录rail、方法、clock=uptime、起止时间和estimated=true。policy=battery另需本轮before/after实际放电与条件化无错误。缺资格返回null及原因，不填0，不用charge policy替代实测资格。

`energy_per_frame_mj`本批仍null：既有parser的首末submit/group间隔与capture窗口不完全相同，目标进程/显示帧语义也未核。后续复用parser导出完整分组时间窗再对同窗积分，不用全窗功率/FPS或capture能量/n_frames拼接。将该剩余项明确列入todo，不能以本批电池能量完成代表游戏每帧能耗完成。

网页同时展示固定政策、实际准备/恢复状态、修改与观测、同期估计指标及不可用原因；CLI帮助包含同一完整新建命令。不新增可并发改充电的另一owner或每轮开关。

## 验证范围

相关既有owner/parser/充电绑定回归及一个有裁剪边界的积分数值案例；不重跑未改worker。实际PF+fakeADB/HTTP验证跨round一次停充/恢复、已放电无写、失败同context恢复，以及缺natural时可独立归还charge但整个会话仍required。可以沿前批runner分少量有区别的会话，不重跑旧游戏实验。所有失败报告、隔离屏障和原件保留，不能为测试通过清掉恢复义务。
