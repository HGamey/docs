# 显式宿主作业的默认资源

FLEET-HOST-RESOURCES-01 修复提交入口的实际阻断：CLI与网页选择 `device_access=none`、资源留空时都会省略resources字段，服务却仍按phonefarm/GOL等程序名添加 `legacy-device`。即使设备恢复任务已释放宿主CPU容量，纯离线分析仍被这个独占键挡住。

新请求显式无设备访问且未提供resources时，默认资源为空。显式资源仍生效；缺省或显式write继续使用原设备默认；recovery继承原父任务资源。设备binding的原门禁、CPU/GPU/IO容量计账、唯一owner与producer自身lease不变。none是调度声明，不授予设备命令绕过lease的能力。

## 原请求兼容

已接受请求可能保存了旧默认 `resources:["legacy-device"]`。只有原输入为显式none、资源字段省略、属于旧设备程序且无binding/recovery时，服务生成一个内部旧规范候选。在已有intake锁内，先比较新规范请求与原body指纹；若已有记录不匹配，再比较只含旧资源默认的完整候选。匹配时复用原row/spec/指纹/资源/状态，不重新prepare、不执行新准入，也不把旧任务资源改为空。

不同argv、body、host_slots或显式资源仍按原完整内容判409；备选不能创建新记录。历史未区分资源是用户显式给出还是当时补入，因此完全相同的旧规范内容都可按此路径找回。这是原内容兼容，不是历史任务自动迁移。

## 验证范围

一条聚合真实隔离HTTP与pf-node CLI回归在修复前复现错误默认；修复后通过：11次HTTP提交、3次CLI调用只保留8个job，旧行不变、冲突不重准备，新宿主任务可越过保留的设备资源且仍受CPU2容量约束。设备blocked及host已释放是测试设置的前提，未运行真实worker或手机，不宣称完成设备恢复/跨节点调度。

Web/CLI复用同一服务默认，客户端无需新字段。最终提交 `f5a7ea4` 已发布main。160项本地协议首轮159通过；唯一旧restart fixture依赖none隐式设备串行，改为显式声明同一资源后，原超时/取消/重启断言定向通过。原失败保留，未反复全量重跑。最终code-only 40293859867和node-tests 40293859441五项CI全部成功。FLEET-04的能力epoch、JOB-03未覆盖的设备scope、JOB-10与DEPLOY实机验收继续开放。
