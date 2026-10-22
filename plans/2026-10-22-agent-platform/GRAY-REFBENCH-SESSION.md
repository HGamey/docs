# 四臂 refbench 的统一会话

2026-10-23，PRODUCER-RECOVERY-03B4C2D2实施合同。仅源码/宿主替身，尚未实现/部署/真机验收。依据旧test_loadop.sh、当前gray_session/native layer，以及原研究checkout refbench/app/src/main.cpp与contract（不视为当前安装APK证明）。

## 实验与接口

新增phonefarm gray-layer refbench-loadop --serial S --out DIR [--frames 1800] [--intensity 8] [--timeout-seconds 150] [--so FILE] [--evidence-origin device|synthetic] [--json]。frames范围60..100000，intensity1..64，单臂timeout1..600秒，均在设备动作前核验。恢复为gray-layer refbench-recover --session DIR --serial S --json。旧test_loadop.sh只转发，保留位置out、FRAMES/INTENSITY/显式REFBENCH_SERIAL默认，取消硬编码设备与隐式证据输出目录；CLI参数优先。

固定包io.github.hgamey.refbench，固定顺序A_noloayer_off（保留旧拼写）、B_nolayer_on、C_rewrite_off、D_rewrite_on。一个现有Lease+Session覆盖四臂，无sampling Parent、trace/worker/停充/锁频/风扇。A/B每次确认SO与STATE不存在且global layer为空，C才首次安装，立即带完整--es字符串extras启动，不多启动默认负载；C/D loadop=1、passdump=0。所有臂scene=bw_pingpong、run_id=原label、frames/intensity/knob.loadop均用--es，回核应用实际自报参数。

## 所有权与恢复

Session增加专属completion owner与固定inventory，在现有6个layer/frameaudit输出外加入external refbench_out.json和refbench_started。复用prepare、首次hold、原值/历史文件备份、按轮归档、命令日志及typed restore。拆出既有install_mode的激活/取得输出、安装、启动私有步骤，避免另造恢复框架。未知写入/transport/身份/恢复读回不确定保留required；新recover仅接续原context恢复，不重跑四臂。

A/B已active但未安装同样有输出/目标义务，按原ownership字段恢复。C/D当前有效layer marker成立即清全局属性，退出后才能换臂。恢复范围是本会话安装/属性/历史文件/目标停止；超时后的已知force-stop和完整读回可完成恢复，但不能把它改写成自然退出。实验预期不符或应用失败与恢复失败分别记录。

## 原件与事实

正常保留八个旧*.refbench.json和*.layer.json，A/B层文件无输出时保留空文件并在报告明确confirmed_absent，不能伪造JSON。另保每臂.started原件和集合报告；失败保已取得原件，未执行臂不捏造。layer原件使用既有可用输出路径并记录来源，错误/缺席不冒充无层基线。

分四层：collection（原件与缺失）、natural_exit（本臂started内容匹配且cleanup前目标已消失）、clean_completion（run_id/scene/params/effective_knobs/frames_submitted/clean_exit自报）、rewrite_expectation（已知答案）。frames_submitted按真实producer每present成功后的计数，不按GPU submit换算。C的LOAD→DONT_CARE必须有实际begins>0；D允许创建期effective非空，但有效begins总和应为0（旧“effective必须空”注释错误）。所有解析结论指向原件；不把这些自报称FPS/功耗收益。

集合报告及独立recovery报告随原job/out可发现，网页通用任务提交/原件页与CLI可运行/访问同一内容；参考整组Web体验由并行CTRL-06补。退出0要求完整且通过上述资格并恢复完成；失败/预期不符为1，恢复required或调用错误为2，结果保留各层事实，不以退出码遮盖。

## 最小验证与分工

platform独占gray_session.rs的专属owner/固定inventory/小型私有抽取/API；root负责新gray_refbench.rs、main/graylayer/devlock路由、薄shell、AGENTS、Git/Cargo/docs。project唯一actualPF+fakeADB QA owner，源码冻结及签名构建后一次跑正常四臂、超时可恢复、恢复读回失败再同context恢复三条短链，保失败原件；不跑已有control/供电/quality整链。相关纯解析/owner小测试各一次，受抽取影响选现有对应测试，不做全矩阵。无真实设备/部署/数据迁移。

## 网页任务模板（同批补齐）

任务提交区增加四臂refbench模板：serial必填，frames/intensity/timeout沿CLI范围和默认值，可选SO路径。填写模板只生成bin=phonefarm、argv=gray-layer refbench-loadop ... --out @out --json，仍由现有提交按钮/请求幂等/任务队列执行，失败原件沿任务out页；不直接触发本地设备、不提供浏览器恢复捷径。未确认任务提交时模板不得覆盖。显示A/B真无层、C/D改写，采集自报而非性能，避免误当游戏采样模板。小型HTML/JS改动和定向交互检查，不重跑CTRL-06整链或node协议本地全套；root拥有AGENTS/docs，frontend独占jobs.*与CLI帮助示例。
