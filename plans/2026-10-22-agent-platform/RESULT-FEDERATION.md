# RESULT-FEDERATION-01：选定节点的结果检索

2026-10-25，已发布harness42b2605ca3b4e34ec3d0865e4a208bd123042800；五项actual required CI成功后同SHA直推main并核远端。只做源码与隔离宿主；不部署、不操作手机或迁移原件。承接已发布的 RESULT-INDEX-01。复用持久节点目录和原结果索引，不另建中心结果数据库。

## 查询与身份

新增能力 `results.federation-v1`；`GET /result-search` 与 `/dash/api/result-search` 同实现，沿只读 token/查看密码，必须带当前 hub 的显式 expected node pin。

`targets` 是 JSON 数组，显式选 1–4 项 `{name,node_id,revision}`。name 为已有目录登记名，node_id 为其已绑定的 32 位身份，revision 为选定时的正整数版本。同名或同 node_id 重复拒绝。未绑定的节点先在目录核对，本查询不自动首次绑定或调用 refresh。客户端从只读 `/nodes` 获取选择；不接受 URL/凭据/任意路径。

其余过滤/limit 与 `/results` 相同；limit 默认20、上限100，按每节点计。cursor 仅允许一个 target，保持源节点不透明游标，不重算首屏上界；target/revision/过滤不变才能续页，错误不自动回第一页。各节点独立分页，没有跨节点时钟排序或全局游标。本节点仍用原 `/results`，不列入远端 targets。

每项在目录锁内复制配置，网络在锁外；核 expected node_id/revision/enabled/credential，再只请求源 `/node` 和 `/results`，两者都带源 node pin，响应也核源身份。源必须声明 results.index-v1 和 node.target-v1。禁止重定向、设备/fleet/queue/job扫描及原件下载。返回前再次核配置版本，配置变化时丢弃迟到页，不改绑重试。

读取共用现有2MiB响应限制与8秒总截止；整个服务至多4个远端读取线程，无排队。槽忙立即给busy分项。DNS未返回时槽继续由该线程占用，截止后的线程不得发凭据，迟到结果不改变已返回响应或持久目录。没有后台刷新、结果副本或离线数据库。

## 响应

`{schema_version:1,node_id:<hub>,filters,limit_per_node,partial,nodes:[...]}`。每项：

```
{source:{name,node_id,url,revision},status,attempted_at,read_at,error,
 requested_cursor,items,next_cursor}
```

status 为 ok/unavailable/unauthorized/unsupported/identity_mismatch/configuration_changed/disabled/no_credentials/unknown_node/busy/invalid_cursor。失败 items 与 next_cursor 为 null；成功空页才是 `[]`。error 为 `{code,message}`固定诊断或null，不回显源报错正文/凭据。read_at 只表示索引读取时间。partial 表示本次有失败分项，不表示页面已穷尽。请求语法/身份非法整体400/409，选定节点失败总体仍200并保留每一项。

items 沿源摘要字段，每项增加由本地登记的实际 source.url/node_id/result_id 构造的 detail_url：`<url>/dash/results?expected_node_id=<id>#result=<rid>`。URL不含token；不信任远端传来的导航地址。源自己的登录与浏览器可达性仍需要满足，hub可检索不等于用户已登录源站。原件继续在源详情通过原stamp校验下载。

## 客户端

CLI `pf-node results search --node NAME [--node OTHER] [同结果过滤]`；首次查询只读目录冻结原name/node_id/revision。续页仅一个node，显式 `--source-node-id ID --revision N --cursor CURSOR`，不得重新读目录后自动换目标。hub仍用既有 --url/--node-id/凭据解析，凭据不转交CLI或链接。输出同一分组JSON，partial返回非零但保完整结果；无写入/自动重试。

Web在结果页增加明确节点选择和分组查询；未绑定项禁选，已绑定但上次离线可显式重查。各组保留自己的页、原时间/游标/筛选，单组失败不能抹掉其他成功。过滤/选择变化后不混用旧页；下一页失败保原cursor。迟到响应按请求版本隔离。页面重载/隐藏不自动继续远查。本地登记/导入和pending目标完全保持当前hub，远端只读链接开源站，不跨域fetch或转发凭据。

## 最少验证

两条隔离协议链：两个源同名result分别保身份/分页/源链接，新增结果不越原highwater，源离线不吞分项；在途配置变化/源身份/重定向拒绝，核只读权限和路径只含node/results。全协议一轮，失败仅定向。Web定向DOM与一次390px选两节点→部分失败→独立分页→源链接；不重跑Doom导入、设备或恢复。结果复制/迁移、持久离线缓存、研究对象、生产网络/登录验收继续按todo。

## 本批验证

114项协议52.856秒、两条隔离HTTP/CLI链3.279秒首轮通过。真实索引分页覆盖两源同名结果、首屏上界、partial与CLI exit2、在途revision、源身份/307拒绝和只读权限；只GET node/results，无原件、job或设备，6个fixture server及reader slots收尾。Web定向DOM与390px链通过2次远查、独立下一页、1次带pin导航，无POST/设备API/跨域源API。首次DOM微任务替身与浏览器末尾把jobs.css误当jobs API的断言保留；只续余下DOM/从既有HTTP记录核后置断言，没有重放浏览器。原失败finally未保存page_errors，未声称该计数为零。浏览器/context、PID69371、端口65113/65112已关闭。

源码/隔离范围，未部署/手机/原件迁移。qa/result-federation-{validation,publish,ui-validation}.json留本机，所有旧unknown恢复义务保留；下一RESEARCH-STATE-01。
