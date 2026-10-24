# FLEET-TARGET-01：显式连接目标与原节点绑定

2026-10-25，harness 548ca96已发布，五项actual required CI成功后同SHA直推main并核对远端。依据本机qa/fleet-target-next.md确认的隐式生产URL、凭据回退、legacy fleet环境串用及网页远端目录作用域问题。仅源码/隔离宿主，不选择或部署生产后台，不迁移原件。

## 单一事实源

复用`/node`的持久node_id、host、storage、service与capabilities。连接URL、执行宿主和数据根分别显示；同node_id合法搬目录时显示新的事实，不凭路径或hostname重新定义节点身份。共享目录已绑定的远端ID继续复用，不另造注册表/队列/认证代理。

服务能力新增`node.target-v1`。客户端发送`X-Expected-Node-ID: <32 lowercase hex>`；供链接、图片与事件流使用的等价查询参数为`expected_node_id`。两者同时出现必须一致，重复/非法值400 `invalid_target`，与当前持久ID不符409 `target_mismatch`。在读取上传body、创建job、触发观察或执行动作之前拒绝，并关闭未消费body的连接。原请求内容/幂等fingerprint不加入这个连接约束。

新受支持的CLI/Web对写入要求目标已绑定且节点支持本能力，对原ID查询也携带同一绑定。原始旧API无header请求暂保兼容，不能声称它有目标绑定保证；这是旧调用方的明确能力边界，不是让新客户端默认降级的开关。新客户端遇旧节点禁止写，仍能明确只读查看。`/node`本身允许无绑定发现；operator通过同样只读且鉴权的`/remote/node`取得节点事实，不借用管理token。

## CLI

`pf-node`显式`--url`或PF_NODE_URL；删除无参生产地址，帮助先于凭据解析。凭据仅显式token环境或指定文件，不搜索祖先/用户目录。显式CLI token-file优先于环境；未指定CLI文件时两种环境凭据同时存在视为冲突，文件缺失/不可读直接失败，不回退。错误/日志不得输出token。拒绝自动HTTP重定向，尤其不能把认证或写请求送到新地址。

`--node-id`或PF_NODE_ID为预期目标。直接写入前必须提供预期ID，先用`node`只读取得ID不需要人参与；连接后核实际ID与能力，在打包/上传/写入前输出node_id、host和data_root。所有后续请求和续查命令保留原URL/ID；一次未知写入不换目标、不自动选下一台。显式URL与凭据的旧读命令仍可用，不恢复隐式后台。

games、fleet目录客户端及pf-remote复用同一目标解析/请求约束。operator可以只读`node`发现；动作必须预期ID，恢复/回执仍原节点。legacy `pf-fleet run`选择后将已经核实的node_id、URL及选定token文件冻结传给pf-node，移除继承的PF_NODE_TOKEN及旧目标ID；登记有node_id时须核一致。未知选机结果不重投另一节点。hub模式的目标是hub本身，远端凭据不读回客户端；本批不增加hub代理执行。

显式fleet设备采样同样要求目标能力；不能把GET当成无副作用。legacy `migrate` 的SSH/rsync与恢复启动没有节点绑定合同，本入口返回unsupported且不执行迁移；保留旧实现供未来FUTURE-01迁移合同接续，不宣称本批完成跨机恢复。旧数据迁移继续按用户决定暂存。

`pf-node-deploy`也移除祖先凭据查找与raw curl默认地址。更新成功后调用共用目标的`pf-node admin-restart`，仅发一次重启请求；未知回执保留原URL/node_id供只读核查，不自动再重启。wrapper使用显式环境目标，未改变实际更新器的原子替换/回滚边界；那些能力仍由FLEET-06/FUTURE-05跟进。

## 网页

提供共用轻量目标部件，默认可见当前入口、node_id、执行host和data_root摘要，路径可展开。首次直接进入当前origin时用成功`/node`绑定本页目标；目录跳转携带明确expected_node_id，后端不符时留在可读诊断状态，不写、不自动改绑。

同一origin会话内保留绑定，导航和刷新不悄悄替换；显式目录链接可选择新的预期目标，但旧未决请求始终保留自己的node_id/入口。新未决项保存目标与原request_id；旧只有ID项标为目标未记录，不自动查回或重投到当前节点。身份未知/变化时保留原记录与续查信息，禁用依赖写入，不创建新ID。所有已知JSON请求、原件/图片链接、控制/live观察订阅使用同一约束。

fleet远端选择仅查看目录；本站导航明确指当前后台，提供携带远端ID的“在所选节点打开任务/游戏/控制”入口，不冒充全站跨节点代理。链接中不得含凭据；目标节点使用自己的登录方式。错误、加载和失联状态保持可见，不增加逐次确认弹窗。

## 本批最少验收

两个隔离HTTP节点验证旧预期ID在新节点零写入、上传拒绝不读body、查询不把相同request_id当同一个跨节点请求；无header兼容边界独立记录。CLI验证缺URL/指定文件坏不回退、fleet环境不串凭据、重定向不发送第二写、help不需凭据。网页一条目录跳转/身份更换阻断/原未决ID保留链及窄DOM覆盖。协议全套冻结后一次，失败只修对应链；不重复Doom/远程设备实验。

本地110项完整协议48.551秒通过。实际两节点HTTP拒绝在body/100 Continue/观察前，登录与operator身份链通过；4项新CLI关键合同、24项旧CLI定向适配、fleet采样能力补验和无害部署wrapper/未知重启链均通过。24项初轮中1项旧断言期望raw 409正文，改为新受限诊断后只补该项；首次日志保留。

Web全部静态JS语法、7项共享目标DOM检查、390px两入口实际浏览器通过：1次提交丢失回包、后端身份变化拒绝、原ID找回和带pin原件下载，没有重投。首轮原件链接因fixture用type代替kind未出现，只修夹具补该分支，未重跑提交链。浏览器/context和两个fixture PID/端口均退出；没有手机或生产调用。完整证据索引留本机qa/fleet-target-validation.json与ui-validation.json。

完整多机执行/能力epoch/部署仍在原todo，不因本子项完成提前关闭。真实设备观察成本、完整更新器与跨节点迁移不在上述宿主验证中。
