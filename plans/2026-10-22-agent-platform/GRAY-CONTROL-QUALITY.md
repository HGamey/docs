# 同一持续会话中的画质采集检查点

状态：2026-10-23，03B4C2C1实施合同，尚未实现。只源码/宿主替身，不部署、不操作手机。C2C2另迁移旧genshin_quality客户端；C2C整体在旧入口迁移前保持待办。

## 一个有限动作与共享原件

已有control owner增加action_capabilities.quality=1与quality动作；没有能力的旧owner在publisher处拒绝。params严格为op、label，op仅begin/checkpoint/finish；label使用有界安全短标识。require_controller使用当前actor/epoch，阶段间允许人工接管与归还，逐阶段记录提交者/epoch，不把旧控制权沿用成权限。请求/回执/序列/幂等沿C1，不新增HTTP、owner、设备锁或通用workflow引擎。

begin在新label记录第一张质量检查图；checkpoint在同label begin后记录中间图；finish在checkpoint后记录最终原图和当时marker。三个阶段严格顺序，每成功阶段不可覆盖；failed阶段可明确用新request ID重新采集，旧失败产物仍留存，同ID不重放。已有label不允许再次begin。阶段无导航、输入、重启、摘层、采样或结束owner副作用。人/外部Agent使用已有capture/tap/swipe导航及等待，不由核心硬编码原神坐标/时钟。

核心状态为begun/checkpointed/collected；操作中记录active_attempt，失败保存last_attempt/error且不前进成功阶段。collected只表示三个阶段所需证据齐全，不代表同场景、同时辰、A/B可比、画质通过或设备恢复。budget到期/stop仍走现有父资源恢复，质量状态和工件留存；新session不能接管旧label的阶段。

manifest固定gray-control/quality/<label>/quality.json，schema phonefarm/gray-quality@1，绑定session/job/serial/package/mode，包含label、phase、各成功阶段的request/actor/epoch/时间和图像/marker引用、最近尝试。各attempt使用独立目录和文件，以request ID区分，原件不覆盖；manifest只保三个成功阶段与最近尝试，避免不断膨胀。descriptor.latest_quality提供同一摘要与manifest相对out路径；历史可由原件列表/manifest访问。Web重连直接读持久状态，不依赖浏览器私有ledger。

沿既有capture真实PNG/旋转核验保存完整原图并更新latest_image，便于后续输入继续使用其frame ID。同时宿主派生独立preview-half.png：取源(2x,2y) RGB、宽高向下取半，明确derived与方法。PNG和marker分别记观测时间，不声称原子同帧。finish若marker缺失/坏值或预览失败，先保原图及独立错误，不能标collected；健康owner仍可继续或显式重采，未知设备状态沿原恢复屏障。

原图、预览、manifest路径相对job out；原始文件不含lease token或充电binding。started/unknown不能自动重放。质量阶段不改变既有恢复资格或generic job释放规则。暂停人工等待不阻断其他可行研究方向，当前owner仍按既有预算占用该设备。

## 网页、CLI与旧入口

Web提供label及begin/checkpoint/finish三个按钮、当前阶段/缺失原因、原图/半尺寸预览/marker/manifest入口、最近请求回执；当前actor/epoch与同ID不重放沿现有控制面。默认不自动导航、不自动判断画质。将网页swipe上限从2000ms对齐现有API的10000ms，支持旧时钟2500ms手势，不改变API范围。pf-node帮助给同一quality请求例；不新增服务端API。

旧genshin_quality.sh目前begin会off/重启/挂层/固定坐标进入时钟，finish不重启但直接adb触控截图。旧PNG取全分辨率raw每2像素抽样，不是原尺寸。C2C2须把它改成既有session客户端，固定坐标自动流程只在显式宽高/方向profile下执行；人工等待后重新capture再输入，禁止沿用begin旧图。启动由既有session托管，finish不能重新install/off/restart。历史raw/PNG保持原义，新原图和preview分开，不制造raw文件或静默替换旧shot.png。C2C1先交付可跨Web/Agent使用的共享采集原语，不能据此宣称旧脚本已迁移。

## 最小验证

扩展现有动作能力/顺序/幂等测试和一个实际像素stride预览用例。actualPF+nodeHTTP+fakeADB单会话验证begin→人工epoch变化/重新连接→checkpoint→finish、旧图不能输入、原件下载、无额外launch/off、同ID不重采、缺marker保原图与显式新请求重采；最后正常stop沿既有恢复。旧owner拒绝quality零设备调用。Web一次窄/宽布局与状态操作验收，CLI序列化相关用例即可；不重跑B2供电/worker/旧Doom实验。
