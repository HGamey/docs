# STEP6 附录：插帧呈现节律双峰的根因与缓解、推荐配置的 30 分钟长稳（2026-10-30/31，研究 `selfsr-genshin-ab-20261028`）

正文与 STEP4 附录之后，平台复现研究继续把第 7 组配置（改名做法 `sc_name=auto`，不关 waitframe）从「6 对 60 s 窗口」推进到「6 对 30 s 同口径正式轮 + 根因轮 + 推荐配置终版 + 30 分钟长稳」。实验代码在 HGamey/experiments `glm/sr-baselines` 分支（baseline_cmp.sh / selfsr_ab.py / cadence_report.py / soak_seen_report.py）。

**先更正 STEP4 附录 §3 的一处表述**：B1 seen=89.5 不是「采样位置假象」——srip-cadence 轮的 sclat 复核（任务 31cc1bb276f9）确认 B1 呈精确 [8,8,17]ms 间隔模式（比例 1.9，notComposited≈30/s），是真症状臂。89.5 fps 是下文双峰的 90 fps 档。

## 1. 双峰现象（25 ms lead_ns 基线，18 臂合计）

上屏帧率双稳：症状臂精确 90 fps（每 4 个 vsync 被 SF 丢 1 个提交，子图层间隔 [8,8,17]ms 周期，`onCommitNotComposited` = 30/s 恰为漏槽数）；干净臂 116.6–118.6 fps（帧帧命中）。症状率约一半（8–9/18），换臂随机、无位置规律。

## 2. 根因（srip-cadence 方向，已定案）

- **SF 侧丢提交，非后端供给不足**：症状/干净臂后端自报 shown 帧数几乎相同（10172 vs 10169），real/synth 亦同；差异全在 SF 接受端。
- **冷启动随机定型、臂内粘滞**：吸引子在进世界冷启动时随机落定，之后整臂稳定（两种都是稳态，窗口末 6 个 1s 段一致）；面板刷新率重设不解锁（B5/B6 各 2 次 reassert 仍症状）。
- 根因指向 ZTE waitframe 的 commit 内 1ms 轮询等游戏图层 fence：`sc_name=auto` 改名只部分绕开，遗留双峰。

### 旋钮矩阵（症状率）

| 配置 | 症状率 | 备注 |
|---|---|---|
| lead_ns=8ms | 4/4（100%） | 附加延迟 16.7ms，越短越糟 |
| lead_ns=25ms（基线） | ~50%（18 臂） | 干净档 116.6–118.6 |
| lead_ns=40ms | 1/4 症状 + 3/4 中间档 111–114 | 无全净，附加延迟 48.6ms |
| 25ms + sc_grid=1 | 60% | 无效 |
| 25ms + sc_winvote=120 | 60% | 无效 |
| **25ms + sfwait=off** | **0/6** | **6/6 全干净 115.94–118.28，notComposited 0–2** |

sc 附加延迟 ≈ lead_ns + 8.5ms（线性）。

## 3. 推荐配置与终版数字

`IP_TUNE=stamp=0,panel=120,backend=sc,lead_ns=25000000,unlimit=1,sc_name=auto,sfwait=off`
（sfwait=off 进世界后 `dumpsys SurfaceFlinger --fpsControl_waitframe_off`，收尾自动写回 on，重启亦恢复。）

终版性能（b0c94110b649，6/6 对全部干净无双峰，全指标 6/6 同向 p=0.03125）：上屏 29.78→117.75（+295.4%）、游戏渲染 +99.5%、GPU 活动时长 −51.1%、帧 p95 −46.0%、锐度 +235.5%（HUD +166.7%）。

功耗（622ff5adb4e7，停充加固版 3/3 对有效）：7.82W（+165.9% vs native 2.94W），电池 30s 窗口 +1.8°C。代价换 3× 上屏帧率与锐度反超。

## 4. 30 分钟长稳（srip-soak-v2 方向，0eb94025a536，valid=true）

停充、风扇最大档、native 对照（panel=120）先跑再 srip（推荐配置），两臂 41°C 左右起步：

- **90fps 双峰全程未复发**：29 个每 60s 抽 2s 子图层 latency 抽样，无 90 档、无 17ms 模式——冷启动定型结论在 30 分钟尺度成立。
- **面板 120Hz 看门狗守住**：30/30 分钟均值 ≥115Hz；43°C 穿越共掉 2s（t=28/31），1s 内重设恢复。
- **热学不过，且机制细化**：srip 电池 42→57°C（native 同轮 39→43°C），后段 55–57°C 高位平台（末 10 分钟均值 56.1°C）。**≥56°C 时插帧子图层呈现节律降到 ~74–75fps（间隔 p95≈25ms），≤55°C 回 116–119**——随温度可逆的高温节流档，与双峰不同（双峰不可逆、与温度无关）；长会话后段约 1/3 时间处于 74fps 档。持续游戏需外部散热方可全程维持 ~118fps。
- **节流档定因（srip-heat-throttle 方向，81ed387c7159）：kgsl 热降频，供给不足型**——GPU 频率 clean 段 967–1200MHz，节流段 539MHz（−49%，切换点 56–57°C 随温度可逆；max_pwrlevel 全程未压，是 DVFS 在热信号下自选低档）；74/118≈0.63 与算力比例量级吻合。SoC 热管理的硬件约束，层配置无绕开空间，唯一缓解=外部散热（<56°C）。
- 采样噪声注记：每 60s seen 抽样中 4 个异常点（fps<1 或 ≈59）全部落在 tap 时刻（每 300s 的静置场景输入），判定为输入扰动弃用。

## 5. 复现（HGamey 平台）

```
# 双峰/节律分析轮（SF_ATRACE 窗口末 3s atrace）
pf-node --experiment 0ceedf8… --research <研究> --device-access write --resource device:91253241019A \
  --detach experiment selfsr/baseline_cmp.sh 91253241019A 'ROUNDS=6' 'WINDOW=30' 'VIDEO=0' 'HOLD_KEEP=1' 'SF_ATRACE=1' 'SETS=srip'
# IP_TUNE 覆写 lead_ns/sc_grid/sc_winvote/sfwait 见 04eb673f0f9d / a097ae0dd8b6 / 805c64ad13c6 / 648c09dd8e78 / b6d5969c4f6e
# 终版性能（推荐配置）：b0c94110b649；功耗：622ff5adb4e7（停充加固版，experiments 16ef83f）
# 30 分钟长稳：experiments b48b406，PHASES=native|panel=120|cool=41|tap=300|reassert=1|hold=1800;srip:1:2688x1216:1.0:bm:120:stamp=0,panel=120,backend=sc,lead_ns=25000000,unlimit=1,sc_name=auto,sfwait=off|cool=41|tap=300|reassert=1|hold=1800|seen=60
# 节律档位判定：python3 selfsr/soak_seen_report.py --set-dir <任务out>（experiments a5a3a19）
```

## 6. 任务索引（HGamey 平台）

| 任务 | 内容 |
|---|---|
| 673fdb6b4bb8 | 性能正式轮（旧配置，双峰数据源之一，18/18 臂） |
| 31cc1bb276f9 | srip-cadence 六对+SF_ATRACE（双峰换臂复现、atrace 三痕迹） |
| 04eb673f0f9d / a097ae0dd8b6 | lead_ns=8ms / 40ms 剂量轮（各 4 对） |
| 805c64ad13c6 / 648c09dd8e78 | sc_grid=1 / sc_winvote=120 轮（均判负） |
| b6d5969c4f6e | sfwait=off+sc_name=auto 验证轮（6/6 全干净） |
| b0c94110b649 | 终版性能（推荐配置，全指标 6/6 同向） |
| 622ff5adb4e7 | 终版功耗（+165.9%，前两轮伪读已定因） |
| 56f5676318a5 | 长稳 r1（无效：plain 臂子进程超时；其 P1_hold.tsv 留档 native 无 panel 时自然降档 464s 不自恢复） |
| 0eb94025a536 | 长稳 r2（有效：双峰不复发 + 56°C 节流档发现） |
| 81ed387c7159 | 节流档定因（GPU 539MHz 热降频实锤，gpu_series.tsv） |
