# 完整GPU-submit组窗的电池能量估计

ENERGY-WINDOW-01沿现有control sample原件增加三个指标：`gpu_submit_group_window_energy_j`、`gpu_submit_group_window_power_w_mean`、`battery_energy_per_gpu_submit_group_mj`。第三项为同窗整机电池能量乘1000除完整组数；不使用旧FPS分母。GPU提交组来自既有解析器的cadence，既不证明GPU工作已经完成，也不证明游戏/显示帧和目标进程归属。因此`energy_per_frame_mj`仍为null，ownership/display仍为false。

## 计算与资格

解析器只运行一次，同步保留每个原始组的两个真实提交端点。选择完全位于capture内的连续完整组，裁去头尾整组，保留原始分组相位；不足一组的尾间隔另记。旧summary和golden字节不变。

新估计要求唯一明确boot trace clock、所选submit CPU均有零overrun观察、唯一一致的capture uptime元数据，以及原有全采样窗的电池放电资格。v1使用ENV，v2使用按原顺序的ENV+PWR；时间必须有限且严格递增。样本必须实际夹住窗口两端，边界线性插值后梯形积分；覆盖检查允许的tail slack不授予能量外推许可。缺资格返回null与具体原因，原件和其它合格指标独立保留。

同时修复旧capture积分只读ENV、覆盖却看ENV+PWR的断层：4W序列中capture为100.5–103.5秒，ENV只至102秒而PWR延续103.6秒时，正确积分12J，旧逻辑可能只得6J。旧run和回执不自动重写，重分析必须保存其方法/版本。

`metric_provenance.submit_group_energy`记录available/reason、estimated、battery rail、整机scope、方法、uptime起止、组数、每组submit数、裁边数、不完整尾间隔、完整序列点数、render_comm、分组/筛选声明、clock/loss观察及三项数值。序列点数包含窗外夹逼点，5Hz电池采样不支持逐组能量分布或P95。

网页在原sample详情增加折叠区，严格核数值与资格一致；旧/缺失/矛盾记录不采用残留数字。CLI保留原JSON透传，帮助说明新字段与限制。不改采集动作、充电、owner、输入或恢复流程。

## 验收

phonefarm提交 `d9c7ad0`，harness提交 `b42f798`；同SHA必需CI通过后已发布main：PF code-only40295933822，harness code-only40295936090/node-tests40295936071全部成功；PF node-build40296587424成功；三仓已按本批配套SHA快进stable并读回核实。四项针对性宿主测试首轮通过：分组裁边/相位、原parser golden、v2同序列积分/拒外推、实际control summarizer。后者合成窗106.25–108.75秒、4组，断言9.5J、3.8W、2375mJ/组；无筛选、错clock、loss或充电时新值为null。构建23.981秒、签名成功、测试0.157秒。既有测试清理保留，因此无单独留存run.json，不声称已有真实数据报告。

六项定向DOM fixture场景及语法检查通过，包含旧记录、不可用、零值、矛盾资格和不同筛选声明。没有真实浏览器布局、HTTP生产数据、设备采集、FPS/功耗收益或恢复验收。证据在本机 `qa/resume-local-20261026/energy-window/`。METRIC-OWNERSHIP-01和实际部署继续开放。
