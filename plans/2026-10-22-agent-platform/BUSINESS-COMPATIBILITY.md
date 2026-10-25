# BASE-BUSINESS-COMPAT-01：灰层业务协议兼容

2026-10-25，实施合同。接续runtime身份，解决Node接受新灰层owner，但配置PF helper无法控制/找回它的具体错配。仅源码、隔离宿主和CI；不部署或操作手机。

## 唯一支持声明与缓存

PF现runtime-info的supported_protocols追加 `phonefarm/gray-control-gateway@1`，表示当前session创建、gray-control@1 descriptor、action@1/result@1及typed recovery这一既有网关实现。实际session的parent版本、allowed_actions、sample=2/quality=1仍由原typed helper逐项核验；声明不替代owner能力、设备资格或恢复证据。GOL不消费此网关，因此不冒称支持。

Node要求该精确条目，不按Git SHA或包版本猜兼容。原配置程序观察移为Jobs持有的唯一缓存，启动观察一次，显式runtime refresh更新；NodeRead只投影，不另存副本/定时探针。GET仍只做廉价stat，不执行PF。jobs独立实例与HTTP服务共用同一规则。

execution.programs.phonefarm增加compatibility.gray_control：status=compatible/incompatible/unknown，required数组、missing数组（unknown时null）、reason。reported直接使用原identity.supported_protocols，不复制。仅observed且stat仍匹配计算compatible；旧无gateway声明为incompatible并给缺项，文件替换/未观察保持unknown。node.protocol-compat-v1声明该解释接口；原jobs.session-control-v1仍只表示Node路由存在。运行中owner实际自报与当前配置helper是两份事实，历史不回填。

## 新建会话准入

仅bin=phonefarm、argv前两项gray-layer/session的新请求使用这条规则：必须使用配置PF，禁止该入口exe替换；其他producer的exe功能不变。HTTP新ID在读上传正文前检查，原intake admit在创建/prepare前复核，dispatch在写launch记录前再核缓存/stat。没有自动runtime刷新/重新选程序/重试。旧request优先找回原canonical，不增字段或改指纹。

错误为409 program_protocol_unsupported（required/missing）、503 program_identity_unavailable（unknown/stale与显式刷新建议）、400 program_override_unsupported（新会话exe）。错误携program、compatibility与command_started:false；这是本次新producer未启动事实，不抹旧请求历史。排队后不满足则finish_unstarted failed，保原job及具体错误，不重新prepare或运行。

配置路径换成另一路径时，即便新观察匹配，也不启动旧prepared命令；比较冻结prepared可执行与当前配置解析路径，明确未启动。stat到exec仍可能被管理员替换，本合同不声称不可变可执行锁；实际job自报继续记录真实入口。

旧session status、原ID receipt、stop和typed recovery不应用新建门禁，保留原格式/owner验证与错误/恢复屏障。新声明缺失不等于旧恢复一定不支持。不开raw fallback、不自动部署或寻找旧binary。

## 网页与CLI

现runtime区域展示状态、所需/缺失声明和来源时间；stale说明需显式刷新。网页新gray session提交和pf-node在既有node读取后解释该兼容状态，复用旧ID查回顺序，不新建影子cache或额外探针。旧节点缺node.protocol-compat-v1时，新客户端创建灰层session明确不支持；普通命令和旧会话读取/恢复保持。

## 最小验收

复用runtime与session夹具，合并一条真实HTTP/CLI链：缺声明零prepare拒绝、显式刷新匹配后接收、路径替换stale拒新建但原ID找回、排队后更换被未启动收口；新exe拒绝，旧receipt/stop不被门禁挡。现PF runtime-info实际无设备命令只核新声明；不重跑测量/恢复矩阵。一个390现runtime面板流程显示缺项/刷新匹配/stale与新建失败保输入。每批唯一完整协议，失败仅受影响窄跑。

BASE-04的其余跨仓业务关系仍按具体消费者追加，不凭本项宣称所有协议或设备已兼容。
