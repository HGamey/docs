# 既有会话中的参考臂客户端

状态：2026-10-23，PRODUCER-RECOVERY-03B4C2D1源码与客户端替身验收后已发布（phonefarm bf56025）。只源码与宿主客户端替身；不部署/操作手机。当前reference实际是genshin_upop_ab.sh的REF_ARM=1→ref/passdump，不存在gray-layer reference子命令。D2四臂test_loadop与D3旧挂层入口另列。

## 范围与共享组件

REF_ARM非1仍在任何实验动作前拒绝不支持的upop。REF_ARM=1改为薄客户端，必须显式现有node URL+job或本机session out、actor、label和profile；必须是passdump owner。调用者负责已处于世界场景、游戏渲染精度已设极高；客户端不能从marker计数、包名或截图尺寸推断这些，也不自动登录/点门/重启/改游戏设置。

从genshin_quality.py抽取小型gray_control_client.py，共享locator、结构化status/error、单次POST、原ID回执查询、固定session/actor/epoch及有限ready等待。clock profile/导航留在quality客户端，不能让reference继承原神时钟坐标。保持C2C2用法、原请求ID和原输出；没有新HTTP、owner、资源父或恢复框架。

reference profile严格为schema=phonefarm/gray-reference-profile@1、package、width、height、rotation，均使用原图几何。--input指定有限输入JSON，默认既有workload_spin_touch_v1.json；旧默认是sleep3秒+3次12秒swipe+sleep1秒=40秒。--seconds默认30，范围沿sample的1..120，计划必须覆盖seconds+10，不能自动填充或缩短旧12秒拖动。兼容旧WL/CAPDUR作为input/seconds默认值，显式CLI优先；旧LEAD未设或6允许，其他值在任何查询/动作前明确拒绝，因为当前owner trace lead固定6，不能静默忽略。输入语义由已有owner有限Plan解析，不运行任意exec。--render-comm可选且显式；未提供时帧指标未知，不能补Unity默认。--expect-power-policy observe|battery必填，只断言当前固定policy，不改变会话供电政策。

## 操作与幂等

动作顺序fresh capture→sample(seconds,input,frame_id[,render_comm])→marker。每次新调用冻结初始actor/epoch，动作前重核；原图尺寸/rotation精确核profile，已有owner在输入部署后再核新鲜度和方向。所有请求用label派生的确定式ID，首capture与末marker共享回执决定是否执行：完整则只读取三份旧结果（owner结束后亦可读）；存在partial/unknown则停止，不重新导航或采样。POST不确定只查原ID，不能因网络/泛404换ID。采样失败保留现有原件和恢复义务，不补marker假装完成。

参考客户端不停止已有owner。完成后可用既有Web stop或session-action显式结束，沿Parent恢复供电、输入、trace和base；不新增wrapper stop参数或第二套关闭状态机。输出明确本客户端未请求stop，不能以脚本exit0当整个job已恢复。当前控制会话仍由既有预算和owner负责。

## 证据与旧协议差别

参考全分辨率PNG、sample轮计划/raw/指标、marker与三份回执全部保存在原job/out。客户端只返回这些已有结果/路径与label、session/job/参数，不下载全out，不制造旧shot_idle.raw、power.json或summary.json。现有网页capture/sample/marker/回执可手动完成同样动作，不新增仅CLI可调用设备原语。网页已有供电政策和显式stop，但尚无reference整组label幂等、固定epoch、profile/expect-policy自动断言；不能把原语齐备写成整组体验等价。对应缺口列CTRL-06。

trace lead仍6秒、默认输入40秒/trace30秒，保留旧有限输入时序。供电改用现有创建时固定policy和同期trace窗口；旧独立perf延迟10秒采功率不是同一协议，不能把新指标重命名为旧数据。新输出注明GPU submit proxy、缺render_comm为未知、每帧能耗仍未核实；场景/内部渲染精度/同状态资格未由客户端验证。

## 最小验收与后续

共享模块抽取后对quality旧客户端跑相关回归，reference用同一JSON-only CLI fixture验证完整三动作/旧默认40秒输入和30秒采样、完整/partial重复、epoch/geometry/policy拒绝、丢响应仅原ID查询；检查没有原始ADB/off/restart/perf/script/new job/stop。CLI参数/帮助/静态检查即可，不重跑C1/B1/B2 owner、像素、供电恢复或真实设备链。source/API原格式必须与真实实现核对，替身不能自创字段。

D2需一个真实Lease+既有Session覆盖A无层/off、B无层/on、C改写/off、D改写/on；保留A_noloayer_off历史文件名和八份原件、--es字符串extras、passdump=0、自报与自然退出。A/B不允许通过安装层后关闭改写冒充无层。当前restart_mode要求已安装，不能直接拿来跑A/B。D3等两实际脚本链迁移后再收口旧mount/launch链；旧--keep-prop不启动语义不能别名当前始终启动的control，typed off历史恢复/status/离线分析仍保留。

## CTRL-06 网页整组参考流程（源码与UI替身已验，harness114d867已发布）

任务详情增加折叠“参考臂采集”。显式填写label、profile的package/原图width/height/rotation、期望observe/battery、seconds、有限input JSON和可选render_comm；旧40秒计划只有用户点击才填入，不自动缩放。查询本组只读gr-LABEL-00/01/02；三份完整回执即展示旧结果，owner关闭仍可查询。任一存在但不完整只展示原ID及partial/unknown，不自动接续；只有三个明确not_found+HTTP404才允许新执行。GET必须保留结构化错误，泛404/断网不能当未开始。

采集按capture→sample→marker，调用冻结session/actor/epoch/表单，每步核live、budget、passdump、samplecap2、固定power期望。capture精确profile/30秒新鲜/仍是latest；sample绑定该frame。成功资格和结果参数取自平台回执，与D1一致。每个POST仅一次，失去响应只查询原ID然后停止后续步骤。切换任务/页面隐藏或关闭停止新的后续提交，已发动作由owner收尾；重连后重新查询共享回执，不使用浏览器持久ledger。整组不接管、不stop、不宣称同状态/极高精度/显示FPS/每帧能耗。

结果分capture/sample/marker三行，展示请求ID、原件入口和阶段；局部失败不能藏掉已收原件。原“停止并恢复”仍是独立动作。文件只改jobs.html/css/js；定向JS行为检查及单次390/1440浏览器替身验收，不重复PF/owner/恢复链，不部署。
