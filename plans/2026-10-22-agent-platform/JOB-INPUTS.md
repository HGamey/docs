# DIST-INPUTS-01：同节点的小工件输入绑定

服务、CLI、网页同批。复用既有 Jobs._prepare、持久队列和请求幂等；不新增独立调度器。大APK/PK4留现有缓存/设备部署，首批每份输入<=8MiB、总计<=32MiB，最多32份。原实验原件保留。

## 请求合同

2026-10-25，源码与隔离宿主/Web验证完成；最终合入维护者main后的CI及发布回执见下。

spec可选 inputs：schema=phonefarm/job-inputs@1、node_id、files、bindings、cwd。node_id必须等于当前目标节点；cwd只支持default/input。省略inputs时完全保留旧spec/fingerprint。

files各项：name（受限标识）、target（任务in内相对新路径）、source={job_id,path,size,sha256}。来源只允许同节点输出；先检查源任务已完成宿主收尾和无活动worker，原进程组已确认absent或contained已交还输出。不能从raw finished或PID不活推断结束。设备恢复未完成的负结果可读取，但不宣称设备恢复。

跨节点来源（harness 2026-10-26，DIST-INPUTS-02）：source可含可选node=共享目录登记名（1-64小写字母/数字/_/-，local保留）；声明时仍必须提供size/sha256。准备期由节点用该登记项的凭据对远端`GET /jobs/{id}/out?path=`做一次有界抓取（上限为声明size+1），逐字节核SHA；凭据只在目录模块内部使用，不进回执/日志/清单回显。远端不可达/凭据被拒/任务不存在/大小或SHA不匹配沿同一未启动preparation_failed终态，不重试不等待。能力`jobs.cross-node-inputs-v1`在节点配有共享目录时声明；CLI在清单含node来源而节点缺该能力时拒绝提交。不取消本地同节点收尾资格检查；远端任务状态不做本地推断，只按抓取与哈希结果判定。仍不支持运行中依赖等待、大文件复制与公开contained。

绑定各项：source（files中的name，必须是JSON原件）、target（独立新输出路径）、values=[{pointer,input}]。显式JSON pointer必须命中已有字符串字段，input引用已声明files name；写入该任务内输入的绝对路径。保留源JSON字节不改，resolved JSON另存，回执说明派生映射，绝不沿用源SHA来声称派生内容相同。不得猜测替换脚本文字。脚本用PF_JOB_IN或cwd=input引用任务内相对路径。

所有target互斥、无父子冲突、不覆盖tar上传或现有路径。固定小清单可完整预验证；源文件用SafeRoot/openat单FD流式读取，边复制边hash一次，比较size/SHA、前后stat；成功后持久化in与输入回执再queue.ready。缺失/变化/不匹配沿已有未启动preparation_failed终态，不偷偷等待或重放；显式新request_id可在修好后再次准备，同ID仍找回原终态。

## 共享入口

Node能力jobs.inputs-v1。GET /jobs/{id}/input-reference?path=...只在显式请求时按上述收尾条件计算单个有界文件引用，返回schema/node_id/source；列表不全量hash。

pf-node --inputs MANIFEST.json在普通命令前使用，提交前核cap与node_id、冻结输入文件；只打包直接本机argv文件的旧行为保留。任务详情/API提供声明及准备回执，前端增加输入JSON折叠框和产物“复制输入引用”。未知提交时保持原ID、inputs、argv冻结。旧节点不支持时不提交新能力。

PF_JOB_IN由节点worker注入，用户不能通过请求覆盖。cwd=input仅改变执行工作目录，不改变程序固定路径。需检查固定worker/contained环境接合：公开contained本来不可用，不扩大权限或能力。

## 验证范围

一次完整Node协议；新增聚焦一条实际HTTP/CLI→输入来源→绑定→无害worker链，包含源原件未改、嵌套JSON路径正确、同ID不重复；必要拒绝合并覆盖未知收尾、缺失/不匹配、路径冲突。网页DOM+一次窄浏览器失联找回/引用复制；不触手机、不下载/复制旧大数据。若有失败只修对应链，保原件。

## 第一版固定细节

inputs严格字段：schema/node_id/files/bindings/cwd全部显式；schema为phonefarm/job-inputs@1，cwd为default或input。files 1–32项，name为1–64位字母数字._-，唯一；每项严格name/target/source，source严格job_id/path/size/sha256。bindings可空，最多16项；每项严格source/target/values，source引用一个files.name，values 1–128项，严格pointer/input，input引用files.name。pointer遵循JSON pointer转义，命中已有字符串；同一绑定内不得重复同一pointer。整体清单序列化不超过32KiB。路径必须无空段、点段、父级、控制字符和绝对路径。

GET input-reference返回{schema:phonefarm/job-input-reference@1,node_id,source:{job_id,path,size,sha256}}，不触发任务、不扫描其他文件。引用生成与实际准备都核宿主完成资格，设备恢复状态不据此更改。GET /jobs/{id}/inputs返回schema=phonefarm/job-input-status@1、node_id、job_id、declared（完整请求或null）、preparation（持久回执或明确缺省状态）。回执存job根inputs.json，状态pending/ready/failed；从未准备或取消使用not_prepared，旧任务没有声明使用not_declared。ready仅表示输入冻结完成，不代表任务已运行。详情按需读取，不在每次列表对所有输入hash。

Node将清单保留于规范化spec；省略inputs完全保持旧请求。在准备阶段tar解包后、构造命令和sync_tree之前物化输入；全部落点不覆盖既有文件。绑定源JSON字节保留，派生文件记录source/target/values及生成大小，不冒用源SHA。hash只用于声明源内容和复制接收端验证，不为派生对象再重复hash。

CLI input-reference JOB_ID PATH显式读取单个引用；全局--inputs FILE先读取并校验小清单，核jobs.inputs-v1/node_id后才打包或提交；pf-node inputs JOB_ID读同一详情API。网页沿原任务页提交冻结inputs，未知响应保持原ID；产物新增按需“复制输入清单”操作，用当前输入引用生成可直接粘贴的最小完整清单。来源不可用时显示原因，不用当前普通路径假冒稳定引用。

复用已有PF_JOB_IN：普通worker和固定特权worker均已从本job/in注入，不新增同义环境变量。cwd=input采用该in目录；默认程序根不变。当前公共contained仍不可用，本批不扩大准入；明确核查固定特权worker的环境构造，避免新字段静默丢失。普通输入能力不宣称支持运行中的源、未完成依赖等待、跨节点引用或大素材自动复制。这些需求保留完整DIST-05待办，首批先交付有用的明确路径。

## 实施与验收记录

服务、CLI、网页已共用以上合同。原件+派生总计32MiB；复制/派生不逐文件重复fsync，由现有Jobs._prepare一次sync_tree在queue.ready前持久化。输入回执ready说明已物化输入，不替代作业准备与执行状态。

本地完整94项协议首轮93过（39.611秒），唯一失败是新测试将macOS逻辑/var路径与实际/private/var工作目录混同；只修cwd期待为resolve，不改PF_JOB_IN或派生路径。两项定向输入测试2/2通过（2.115秒），保留首轮原日志。实际HTTP/CLI/无害worker覆盖来源引用、嵌套JSON、原件保留、同ID执行一次、未知宿主/缺件/错误hash/上传路径冲突不启动；无设备。

一轮DOM和一次390px真实浏览器替身通过：提交/单引用/原ID找回/inputs读取各1次，页面scrollWidth390。unknown响应冻结ID+body，失败显示原因和未启动；仅按需hash。页面clipboard写入Promise成功，但自动化系统剪贴板读为空，未声称真实剪贴板往返验证；可见JSON及手动复制路径可用，DOM覆盖拒绝fallback。owned tab15关闭、视口复位，fixture PID46911退出、50584端口关闭。

本机原件：qa/job-inputs-validation.json、job-inputs-protocol-1.log、job-inputs-protocol-targeted-1.log、job-inputs-ui-validation.json及其DOM/浏览器/截图索引。原始证据保留本机，文档不假称已上传。首次合入e6b9bc4后的bf2bd1b五项必需CI全部success；发布前发现维护者又到553fb16，门禁阻止旧SHA推送，已保留新提交并重验，最终SHA/发布以下一段为准。

未覆盖跨节点、运行中依赖等待、大文件复制、公开contained、生产部署和真机；完整DIST-05不因本子项完成而勾选。

最终harness dd003401af2bdff6611e6d4baabef3350e75a150已获code-only、protocol 3.11/3.14、pf-shared-kernel、cgroup-kernel五项actual check-runs success；同SHA直推main并ls-remote核实。未部署。
