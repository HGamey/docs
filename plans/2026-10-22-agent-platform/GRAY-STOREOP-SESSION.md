# 灰层 storeop 检查会话

2026-10-23，PRODUCER-RECOVERY-03B2A 实施前合同。继03B1后，只迁现有独立 frame-audit storeop-check，eval内调用暂显式保留legacy路径；借用父租约与完整eval恢复另列03B2B，不借子报告完成父任务。

## 独立入口

实际强Lease→已有GraySession，复用原属性/安装/历史文件快照、pending、串行门控、恢复和recover，不新建无调用框架。storeop规则在第一写前验证（0、ds或合法sig:attachment列表），在开启全局层和启动前设置并读回rule/live/on；开关每次读回成功才继续既有等待/截图序列。退出仍恢复所有12原属性。

复用capture_scene及PNG截图。每段三张无损PNG保留为证据并用已有image库解码RGBA，沿用像素差与UI排除区域、noise_1s/off_vs_on/same_state比较；画面不完整、尺寸不同或解码失败须明确失败，不能静默少算某组。原裸截图解析继续服务legacy消费者，不改性能/画质阈值。图片解码有资源上限。截图在当前有界8MiB传输内，超限明确失败，不将截断图算有效。

FA_GO、audit、marker、存活与crash日志读取全部走session；读取失败不能作为零crash/目标存活。保存每帧路径、采集元数据与有限crash日志，错误停止采集，安全收尾与采集结果分开。输出包含session/collection/recovery/report路径；JSON默认兼容旧storeop-check行为，新增显式--json与--evidence-origin。采集失败已恢复exit1，恢复或最终元数据存储失败exit2。恢复scope沿用03B1，不声明游戏状态、sampler/ftrace或停充恢复。

## 临时兼容与下一步

既有eval调用明确命名storeop_check_legacy，仅作为下一批尚未迁移的调用，不声称受本session保护。两条路径共用像素统计纯函数，避免复制判据。03B2B需要实际父Lease和持久父context，子会话完成只写verified；独立recover不得接管borrowed会话。完整eval尚有后续输入/sampler/ftrace/停充，当前子范围不能释放它们。

## 本批验收

保持最小：已有frameaudit相关像素/解析回归；实际CLI/fakeADB正常检查（含原属性/历史恢复、相同RGBA指标）、截图/开关失败停止后续采集且恢复态明确、坏规则首次写前拒绝。共用03B1恢复失败/未知写/身份案例，不重复整套。仅本机模拟，无部署、真机、原始数据移动或性能声明。
