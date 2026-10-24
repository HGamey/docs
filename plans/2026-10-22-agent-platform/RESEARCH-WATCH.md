# RESEARCH-04：有界研究事件等待与接续

2026-10-25，已发布harness df7c4ecece85ebb3b0b364b2a2a11b45ee05ca7b。仅源码、隔离宿主与 CI。复用 [RESEARCH-STATE](RESEARCH-STATE.md) 的已提交事件及 [RESEARCH-HARNESS](RESEARCH-HARNESS.md) 的客户端交接，不新增 Agent、共享消费锁、任务调度器、消息代理或模型调用。

## 流与游标

原 `GET /research-events` 和 dash 镜像增加 `wait_seconds`（整数0..15，默认0）与可携带 `cursor`。capability 为 `research.watch-v1`。旧 numeric after/limit/research_id 仍支持；cursor 与 after 不能同时传，显式 research_id 必须匹配 cursor，省略时沿 cursor 的范围。wait>0 或 cursor 需显式 node pin，不能把凭据当目标绑定。

在原研究 SQLite 增加 metadata 表，stream_id 初次生成并持久化，服务重启不变。opaque cursor 编码 v/node_id/stream_id/research_id/after/operation_id；operation_id 是位置上原事件的身份，after=0 时 null。它不是授权凭证。读时核流、范围和事件锚；流不符、after 超出 head 或原位置对应不同 operation 明确拒绝，不归零、不跳过历史。失败保留客户端原游标。不能宣称可识别所有任意外部改库；正常研究写入口不改历史。

响应沿用 schema_version/node_id/items/next_after/has_more，添加 stream_id/research_id/head/next_cursor/wait_result（events|timeout|immediate）。head 是已提交全流上界，不能作为已处理位置；next_cursor 只到本批返回的最后一项，无项时保留原位置。事件 item 内容不变，不夹带反馈全文或秘密。

有匹配项立即返回；空页才有界等待。用进程内 Condition 在成功提交后通知，读与等之间避免丢唤醒，醒来或截止时重新读数据库。等待不持有 SQLite 事务、不循环查询 SQL。只给等待路径16个名额，满时429/research_watch_busy及retry_after_seconds，正常读取/写入不占等待名额。断连未及时感知的等待也在15秒截止。重启中断连接不是空页，恢复后从原已保存位置继续。

## CLI 和网页

CLI `research events --wait-seconds 15 --cursor CURSOR` 单次 GET，沿现30秒网络预算；返回同一 JSON，空等成功exit0，离线/拒绝非0，SIGINT停止。cursor/wait非零核新cap；不自动重发/分页/写文件/推进业务确认。JSON batch是供外部运行器消费与通知的适配入口，原next_cursor只有处理并保存相应决定后才能成为该消费者的checkpoint。不同消费者独立保存，不设全局已读。

网页复用事件区：立即续读、显式开始监听/停止、当前范围和最近读取状态。监听最长5分钟，每次15秒，单flight；收到新版本只展示链接和提示，不改表单/草稿/对象选择/依赖。收到has_more继续受限分页，展示最近最多200项并说明截断；旧事件仍可从位置重读。

目标/研究范围变化、页面隐藏/pagehide、错误或总预算结束都取消等待并使迟到响应失效，回到可见不自动恢复。旧节点仍可立即读取，监听不可用。429给可重试时间而不自动重试。正常空等可在监听预算内再发下一次有界请求，禁止短间隔空轮询。

本地存储仅保存节点URL/ID、范围及已展示cursor，无凭据/正文/媒体；标签明确“浏览位置”，不冒充Agent已处理。按目标/范围隔离，恢复先核目标及流；流变更提供显式重新读取入口，不自动覆盖原位置。存储不可用保内存并说明。支持复制可携带位置JSON用于同一目标的其他客户端，并明确事件处理确认由消费方决定。

## 空闲边界

现流只覆盖研究元数据提交，不覆盖job终态、恢复/释放、result登记或设备重连。这些原因导致的空闲仍需原status/result/设备能力入口；保留各自待查ID。没有可行方向时外部Agent可依原合同记录idle/reason和接续块；有可行方向则继续。浏览器提示和CLI JSON均不会启动模型、采纳反馈或修改优先级；停止的运行器不会被平台自动唤醒。

后续 JOB-04/结果事件若统一，需要各自持久提交边界与命名空间，不能拼接独立数据库的sequence。通知送达确认、跨节点消费另列，不以本批watch宣称已有。

## 最小验收

一条真实HTTP/CLI纵向：空等截止→等待时并发提交及时返回→同库重启原cursor续读→原过滤/错误流/原事件锚不匹配被拒且保游标。核只发一GET、提交仅产生一原事件、等待无设备/队列任务；必要竞态/容量边界并入同一模块，不重写原CAS/媒体测试。协议整轮一次，失败仅续受影响项。

一条390真实浏览器：显式监听→新事件可见且草稿保留→隐藏取消→显式原位置恢复→网络错误不推进→重载携带原浏览位置。仅isolated fixture，先冻结计划和资产，不重复全部研究UI验收。公开页面/CLI/API同级交付，源码与隔离结果不冒充生产部署。

## 发布证据

2新HTTP/CLI定向首过2.798s，121整轮协议首过61.723s；CLI/parser/skill与JS语法通过。五项actual required CI成功后同SHA main并核ls-remote。

390实际浏览器+真实NodeResearch/HTTP首轮通过：7事件GET，2次fixture元数据写（另setup种子1），总HTTP38，最多1个浏览器事件在途；未保存草稿保持、隐藏后原cursor显式接续、截断正文错误不推进、重载仅恢复浏览位置。pageerror0、scrollWidth390；root已看截图。隐藏为headless内document.hidden/visibilitychange模拟，不是OS后台行为。5分钟预算、200DOM、4页补读和旧cap只静态核验。PID81725/57936和browser已关闭。

本机证据qa/research-watch-{validation,publish,checks,ui-validation}.json、CLI和syntax记录。无设备/生产，研究流不替代job/result事件或已停止运行器的唤醒。下一[RESULT-03](RESULT-COMPARE.md)先交显式候选矩阵，RESEARCH-05随后复用。
