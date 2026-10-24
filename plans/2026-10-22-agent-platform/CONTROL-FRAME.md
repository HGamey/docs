# CTRL-03：截图绑定的普通远控输入

2026-10-25，实施合同。补普通 remote-control owner 的截图/坐标关系；复用既有 lease、mailbox、原请求回执和恢复，不另造接管器。灰层 session 的现有 frame 合同不重写。仅源码与隔离宿主验收，不连接手机。

## 身份与坐标

新 PF owner 在持久 state 写 `input_contract:"frame-bound-v1"`，public descriptor 原样公开，并提供 `input_frame_id`（当前可用于输入的截图 ID 或 null）与 `frame_max_age_seconds:30`。capabilities 也声明该输入合同；旧会话缺字段保持不支持，不由新 helper 的版本补造。旧截图/回执可读，其他已支持动作与关闭/恢复仍可用。

成功 screenshot 的 result 保留原字段并增加 `frame_id`（该截图 request_id）、`session_id`、`serial`、`rotation`（Android SurfaceOrientation 0..3，未知为 null）。`source_width/source_height/original_size` 是真实截图像素空间，`width/height` 是返回 PNG 尺寸。`captured_at` 为开始实际截图采集前的宿主时间，年龄保守包含采集/处理耗时，不因重复读取或下载刷新。采集前后方向读取必须一致才能声明 rotation；未知方向仍可观看。

tap/swipe 的 params 必须携 `frame_id`，其他字段不变；坐标始终为原始截图像素，不接受 CSS/DPR/客户端猜测的旋转。限定本 node/serial/session、当前 input_frame_id、最新成功截图、0..30秒年龄、已知方向和图内坐标（0≤x<source_width，0≤y<source_height）。未来时间也拒绝。同尺寸与方向不证明游戏内容静止。

## owner 执行与回执

同 request ID 同内容优先返回原回执，不因当前图后来失效而重放动作。新请求在串行 claim、写 started 前核当前图；不满足则持久化 `status:not_executed`、`error_code` 和原 request_id，继续处理其他请求。缺少 frame_id 等语法错误沿既有 invalid_request 返回，不新增请求。

开始截图或任何输入/应用动作前将 input_frame_id 持久置 null，保留旧图供查看。renew 和只读查询不改变资格。新 screenshot 成功且方向已知后设为该 frame_id；tap/swipe/text/key/start_app/stop_app 无论成功与否都不重新授权旧图。未知状态继续原恢复屏障。

已开始的 tap/swipe 在同一个 lease 内再次采集验证图与方向，校验原图尺寸/方向和时间，随后才发出 input。复用有界 Transport；验证图与读回日志留在原会话。已知几何不符/方向不能确定/采集后过期返回 `status:failed`、`result.input_issued:false` 与具体 code，不把它说成未开始整个动作；会话可继续，下一次输入须显式新图。传输/输出完成未知仍 `unknown` 并保恢复屏障，不借这一拒绝路径消除未知。发出的输入回执带 `input_issued:true` 和 source frame_id；completed 仍只表示命令完成，不证明应用响应。

稳定输入错误码：`input_contract_unavailable`、`frame_unavailable`、`frame_stale`、`frame_orientation_unknown`、`coordinate_out_of_bounds`、`frame_geometry_changed`。执行中已知方向无效使用 frame_orientation_unknown；无法确认传输沿原 unknown。验证预算包含新增有界读回，不挤占 owner 的收尾预算。

## Web、API 与 CLI

Node 只透传 PF 图片/回执事实，tap/swipe 在旧 owner 缺新合同前明确拒绝；不按网页缓存授予权限。`pf-remote action` 增加易用的 `--frame-id`，也允许 params 显式提供，冲突拒绝，不自动取最新图或重试。HTTP 与原请求查询路径不变。

网页沿现 control 页：显示 frame、原图/传输尺寸、rotation、采集时间/年龄及不能输入的具体原因；等待 PNG 解码并核自然尺寸后才启用。pointer down/up 冻结同图、同会话与映射，换图/过期/越界拖动取消，提交一次 frame_id+原图坐标。旧图继续看，历史图片重读不自行授权；资格必须来自当前 descriptor.input_frame_id，动作后需显式截图。共享 live 观察图继续只读。网络不确定保留原请求及来源 frame，只查原 ID；不换 ID、不自动新图重放。

## 最小验证

PF 只运行受影响 remote_control 单元与一条实际 PF/有界假传输链：缩小图对应原坐标、单次消费、新图/旋转或过期拒绝、已知不符不注入、原 ID 找回，未知沿原屏障。复用现已有方向解析与锁测试，不跑旧全部 producer。

Node 一条实际 HTTP/CLI 链验证新字段、旧 owner 不支持与原回执保留，再按仓规完整协议一次。网页先冻结计划，单390流程验证缩放映射、失效保旧图、丢回包只查询。生产/真机、游戏内容变化判别、HDC/iOS与高帧率控制不由本批宣称。
