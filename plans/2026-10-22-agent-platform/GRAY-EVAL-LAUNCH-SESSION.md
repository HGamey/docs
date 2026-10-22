# Gray eval 自驱动负载父会话

2026-10-23，PRODUCER-RECOVERY-03B2B 实施前合同。覆盖现有kind=gray且workload.launch可解析的路线：storeop静态画面检查、资格自检、三基线、ABBA、收尾和recover。旧脚本路线继续明确未迁移，不因该批获得整任务恢复资格。没有部署或真机验收。

## 一个会话贯穿父流程

run_eval把实际Lease移交父GraySession，所有设备命令沿同一串行lease_command/pending通道。storeop抽成只采集collector，接收已有Session，不创建第二个lease或会话，不完成恢复。独立storeop入口仍自行prepare/install/finish。父目录保存原请求、阶段、轮次、资源原值/登记和证据引用。

新Session版本必须有completion_owner=standalone或gray_eval_launch；旧v1仅兼容独立会话。frame-audit recover/finish在任何设备操作前拒绝父身份。eval recover --session DIR --serial S只接同context且旧owner结束的父会话，不重跑实验。未知/pending写仍需独立核验，不凭新读回自动清除。

原属性和历史文件只备份一次、SO只安装一次。首次prepare把安全单文件名的launch marker纳入目标files目录的历史清单；不继续用旧start_with_extras无记录删除。每轮restart先停目标并读回、归档并清本轮产物、重设完整模式再启动；本轮archive指针在成功清理后更新，旧归档和最初备份保留。无层资格/基线使用全部旋钮关闭且global layer为空；带层ctrl/candidate使用probe模式，必须关闭image-check留下的frameaudit/live/on等观测开销。全局层仅保持至当前目标证明加载后再narrow。

## 实际恢复义务

仅治理真实改变的资源：层/12原属性/安装和历史文件、目标及launch marker、fan_enable、远端采样进程、专属ftrace实例、专属工具目录。gray_eval没有停充/锁频，本批不隐式新增，也不声称这些控制已恢复。无fan节点须从成功父目录读取确认；有节点先保存原值，修改后逐项读回，结束恢复实际改过的原值。

工具放独占phonefarm-confirmation-gray目录，复用现有worker/bootstrap的PID/start/PGID账本与permit/closed协议。增加gray_sample有限模式，两个已登记子进程分别执行trace workload和只读sample_env，分别记录退出码并等待结束；不能用本地ADB退出宣称远端停止。采集失败但进程生命周期已明确结束，可以返回失败数据并安全收尾；无法核验进程身份/未知启动保持屏障。

ftrace只用本session创建的独占实例，拒绝已有实例和无instances能力，不退全局buffer。保存身份、明确创建/写入阶段，boot时钟与/proc/uptime对齐；保留原trace元数据/overrun头供既有解析，采样窗口与环境窗口仍需验证。结束先确认受控进程结束，再停trace并移除自有实例、恢复fan；不覆盖其他trace或共享脚本。保留关闭的远端ownership账本供核验，临时工具保留政策与路径写进报告，不声称设备目录零残留。

## 结果和父完成

所有采集、marker、PNG/trace/env和失败原件可追到父目录。沿用现有qualification、live_check_verdict、trace/env解析和ABBA统计；不改收益或画质阈值。恢复事实独立：base.restore_for_eval只产生局部verified/required；父核验所有登记资源并持久报告后才complete_eval。只有完整父收据允许facts.restored=true。本地结果落盘失败仍执行已知安全收尾并保留已知恢复结果；game_state_restored=false。

在父会话仍持有恢复义务时，Experiment admission 可将 control/restored 的 unknown 作为待采集/待收尾事实，不能假填 restored=true；仅新父会话专用入口采用此规则，调用前先读回已登记进程静止、trace关闭且父pending为空。已知恢复失败仍阻断；Conclusion 一律需要完整恢复。既有独立资格入口、性能/画质/统计阈值不变。

## 调用与产物

`eval --request REQUEST --serial S --out DIR [--evidence-origin device|synthetic]` 自动将 gray + workload.launch 路由到父会话，原脚本路线保持原边界。可选 `payload.layer_so` 指定本地构建层，缺省使用原层路径；storeop_dont_care 必须同时通过信封白名单和原 scope 解析。自驱动路线拒绝 loadop、onboard 输入，seconds 限1..86400。无synthetic结论豁免；来源会进入资格证据。

输出在唯一gray-TOKEN子目录，含eval-session.json原请求/阶段/资源、eval-collection.json基线/两臂/部分PNG、各轮原trace/env/退出码/marker和eval-report.json。恢复通过 `eval recover --session DIR/gray-TOKEN --serial S --json`；原失败报告不改写。正常或REJECT为0、ABORT且已恢复为1、恢复/报告错误为2；恢复入口完成为0，否则2。

已知未启动的部分工具部署可关闭登记后恢复；未知写/pending仍拒绝自动清除。父资源恢复写先核boot；独立frame-audit recover不接父身份。首次只读失败且没有设备修改，可以产生no_device_mutations局部报告并正常结束，不制造假恢复屏障。

## 验收与接口分工

先冻结实际调用的最小接口，再并行实施：GraySession的父身份/restart/marker与延迟完成；专属trace采样worker；父资源会话和eval编排/恢复入口。不要先发布无调用骨架或永远required的父壳。验证只覆盖改变：既有关键纯解析/判据；远端worker宿主fixture的两个子进程退出与失败；少量实际CLI/fakeADB覆盖完整launch路线、资格早退仍收尾、恢复失败/续接以及独立recover拒绝父身份。复用已完成的gray/lease负例，不再反复全套测试。模拟不能替代真机性能、断电或逃离进程组验收。
