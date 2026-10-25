# BASE-01：回收 Harmony SmartPerf 供电过滤

2026-10-25，实施合同。旧PF研究树HEAD9d92712的三处用户修改已限定保存为本机qa/base-pf-three-user-changes.patch。README和SPEC中的2026-09-24 nova16冒烟叙述保留为历史作者记录，本轮不把它升级为新验收。当前PF主线已增加FpsScope等帧来源规则，这些必须保留。

## 已确认遗漏与行为

旧src/perfsrc.rs未提交补丁要求BatteryService采样前后均chargingStatus=0、pluggedType=0且nowCurrent<0才接受电池功率；否则power_watt=null并给原因。当前主线尚无此过滤，会仅依据currentNow/voltageNow的数值合理性给功耗，USB充电可落在合理区间。回收这一通用正确性修复，不更改Android、电源轨算法、原始CSV或帧来源。

在线Harmony在采集前后各执行一次现Device.shell的BatteryService只读命令（各8秒预算），两端缺字段/非放电即清除power_watt并保存原因。两端通过也只说明端点供电条件，不能证明整个窗口未切换；SP电流电压均值仍是电池功率估计，不叫完整整机能耗或每帧能耗。保留既有来源/采样数/温度/FPS资格。

离线CSV默认无法推断充电条件：同目录需要battery_before.txt与battery_after.txt原报告，任一缺失/不符合则power_watt=null。这些文件由数据提供者声明属于该采样，不以同目录存在冒充自动窗口认证。网页/CLI原有PerfSnapshot指标与unavailable原因共享此结果，不新增独立UI计算。原始数字/报告文件不改写。

## 验证与处置

沿旧补丁的真实供电文本夹具，最少覆盖USB充电、缺字段、断电负电流，以及离线缺前/后报告和有效端点；在原offline测试保留FpsScope/frame_judge断言。一次相关Rust测试，必要一条小CSV实际CLI链，无手机/hdc/后台/大素材。不跑无关完整harness协议或已有测量恢复矩阵。

旧用户README/规格修改转存docs的历史记录，不能复制到40行代码README或丢弃；旧研究工作树不提交、不reset、不重写。所有源补丁最终写入BASE-01映射表，是否采纳各自有明确去向。
