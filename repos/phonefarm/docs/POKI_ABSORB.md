# phonefarm 吸收 Poki：功能映射、iPhone 支持与实施计划

2026-11-25 · 分支 `HGamey/hgamey:poki/approval-skills`

Poki（https://github.com/JosssphZhou/poki ，AGPL-3.0-only）是开源的 iPhone agent。center 是网络服务，AGPL 第 13 条会把对外服务的修改版也纳入开源义务，所以本方案**只借产品行为和公开的设计思路，所有实现都是重写**，不拷代码，不拷提示词原文。阅读用的克隆只放在本地话题目录，不进 hgamey 仓库。

## 1. Poki 是什么

Poki 有两半：

- **手机端 app（当前主线，0.5 Beta）**：agent 循环跑在 iPhone 上，用户自带模型账号。
  - 读屏：主路径是进程内 XCTest 会话读 AX 快照；iOS 27 另有 `remoteAXService` 直读（不出现「自动化运行中」横幅）；读不到树时用 Vision OCR 补。
  - 输入：XCTest 合成指针事件，并做拟人化（按压时长、滑动曲线、打字节奏）。
  - 连自己的开发者服务：用一个回环 VPN（`10.7.0.1` 回到本机），走 RemotePairing / CDTunnel / RSD，再挂个性化 DDI（iOS 26.4+ 是 Cryptex）。
  - 审批卡片显示在锁屏和灵动岛；技能和记忆库存在手机上；入口有 Siri、快捷指令和操作按钮；有内置浏览器、语音对话（Realtime）、分屏；手机端自签续期（免费账号 7 天）。
- **宿主端（已冻结，只修安全问题）**：
  - Poki MCP（Node）：60 多个工具，`look`/`do` 两件套，批处理，租约和输入锁，宿主审批对话框。
  - Rust 原生桥（基于 idevice）：HID 触摸和键盘、AX、AppService、录屏。
  - Poki Agent：学习路线零模型回放、独立复核、单步时限、iMessage/Telegram 通道。
  - Poki Setup：安装、配对、续签。

下表逐项列出 Poki 的功能和 phonefarm 的对应做法。

## 2. 映射表

状态说明：✅ phonefarm 已有；🟢 本 PR 已做；🟡 能做，已排期；🔁 不适用，给出替代方案；⛔ 不做。

| Poki 功能 | phonefarm 现状 | 状态 | 做法与说明 | 工作量 |
|---|---|---|---|---|
| 读屏（AX 树 + 截图 + OCR 兜底） | `els_full` + 截图 + `--perceive ocr`，安卓/鸿蒙/iOS(WDA) 都有 | ✅ | — | — |
| `look`/`do` 回执（`sent`/`not_sent`/`unknown`） | `exec` 不返回结果；以画面差判定 | 🟡 P1 | Backend 动作返回三态；`unknown` 的动作不重放；批内遇 `unknown` 即停 | 2 天 |
| 画面变化判定 | 小图差 + 背景噪声 + 仲裁（比 Poki 的灰度网格更细） | ✅ | — | — |
| 多步计划（一次调用多个动作） | `plan_max`，逐步核验，遇挫作废 | ✅ | — | — |
| 元素编号跨读稳定 | 用文字点名 + 框内吸附 | ✅（不同做法） | 点名比编号更稳，保持 | — |
| **敏感动作审批** | 无 | 🟢 | `approval.rs`：按观察到的事实分类。付款、删除、钱包必问；发送、通话、密码、账号变更按配置 ask/auto/deny；同应用同目标 45 s 内沿用一次允许；超时按拒绝；被拒的动作交还模型改走别的路 | 已完成 |
| 审批 UI（锁屏 / 灵动岛） | — | 🔁 P1 | center 网页任务页加审批卡（截图 + 理由 + 允许/拒绝），配合 Telegram 推送；API 已在本 PR | 网页 1 天，TG 1 天 |
| 接管（handoff，「你来」） | 无 | 🟡 P1 | 动作 `handoff{kind,text}`：暂停写动作，center/TG 通知人，人点「完成」后继续；输验证码、密码、付款时用 | 1.5 天 |
| **技能（保存的流程）** | `script` 回放坐标；`tree` 熟路（页面图） | 🟢 | `skill.rs`：成功局编译成命名技能。按文字定位并带落点提示；记录起点路标和到达文字；输入文字变成 `{{参数}}`；回放零模型，每步过审批，对不上就停，交还模型从当前画面续跑；连败 2 次标为过期 | 已完成 |
| 学习路线自动匹配 | — | 🟢 | `run --skill auto` 按目标模板匹配并取参数；`run --save-skill` 局成功后自动收技能 | 已完成 |
| 只收验证通过的路线 | done 复核 hook（`achieved`） | 🟢 | 编译时默认要求 `achieved=true` | — |
| 结果读取（字段 / 列表零模型） | `--assert` 验收词 | 🟡 P2 | 技能加 `read` 段：按学到的唯一选择器读字段 | 2 天 |
| 独立复核 | done 复核 hook | ✅ | — | — |
| 单步时限、中途纠偏、停止 | 预算 / 步数 / 看门狗；center 有 pause/cancel | ✅ / 🟡 | 中途纠偏：center「人工留言」注入 `note`（P2） | 1 天 |
| 记忆库（关于你、各应用记忆、习惯） | `lessons.jsonl`（任务级 + 全局） | ✅（近似） | 加「锁定」和变更记录（P3） | 1 天 |
| 租约、输入锁、并行读 | devlock / devlease / 设备登记 | ✅（更完整） | — | — |
| Telegram / iMessage 通道 | center 网页 + pf-node | 🔁 P1 | Telegram bot：发任务、收审批、收结果；用白名单。iMessage 不做（农场不需要） | 2 天 |
| 内置浏览器网页任务 | 无 | 🔁 P3 | 农场直接操作手机自带浏览器；WebView 页已有专门判定 | — |
| 语音（Talk to Poki） | — | ⛔ | 农场无人值守，不需要 | — |
| Siri / 快捷指令 / 操作按钮 / 灵动岛 | — | ⛔ | 手机端产品入口；农场的入口是 center 和 TG | — |
| 分屏（Duo / iPad） | — | ⛔ | — | — |
| 屏幕历史（Backtape） | 每局截图 + log.jsonl 已全量留证 | ✅（近似） | — | — |
| 自动解锁（存 PIN） | 安卓/鸿蒙用 `input`；iOS 无 | 🟡 P2 | iOS：PIN 只放节点私有配置，经 stdin 交给解锁步骤，不进模型上下文 | 1 天 |
| 录屏 | 截图序列；安卓有 `screenrecord` | ✅ / 🟡 | iOS 录屏走原生桥（P3） | — |
| 外部 agent 接入（MCP relay） | `phonefarm serve`（MCP）+ center | ✅ | — | — |
| 手机端自签、回环 VPN、DDI 个性化 | — | 🔁 | 农场由宿主 Mac 管：USB 常连，见 §3 | — |
| 宿主审批对话框（安装、打开 URL、剪贴板） | center 角色权限 + 设备登记 | ✅（不同做法） | — | — |
| 拟人化输入 | 无 | 🟡 P3 | 只对反自动化敏感的应用开 | 1 天 |
| Unicode 输入（剪贴板粘贴，先存后还） | 安卓 `adb input text` 输不了中文（实测） | 🟡 P1 | 安卓：装一个只做「设剪贴板 / 粘贴」的小助手（或 IME），先读用户剪贴板，粘贴后还原，输完核对框里的文字；鸿蒙：`uitest` 已能输入，待核实 | 1.5 天 |
| 长按、双击、拖动、捏合 | `run` 只有 tap/swipe | 🟡 P1 | 动作表加 long_press、double_tap、drag；安卓用 `input swipe x y x y 600` 等，iOS 走 WDA / HID | 1 天 |

## 3. iPhone 支持

### 3.1 Poki 怎么装、怎么驱动

- **装 app**：用户 Apple 账号（免费可用）登录 GSA 拿开发证书和 7 天描述文件，重签 IPA，经 installation_proxy 装上；打开开发者模式；配对；挂 DDI。iOS 18–26 还要打开 UI Automation，每次开机输一次自动化口令。
- **驱动**（宿主侧，与农场相关）：
  - iOS 27+：CoreDevice displayservice 的 HID 会话发触摸和键盘，`remoteAXService` 读 AX 树。**不需要在手机上装任何 app**，只要开发者模式、配对和 DDI。Poki 实测 iOS 18.6.2 和 26.6 上触摸发不进去。
  - iOS 18–26：XCTest/WDA runner（要装一个签名的 runner app）。
- **免费 Apple 账号的限制**（Poki 文档和安装脚本里的数字，未在我们的账号上实测）：描述文件 7 天过期；每周最多 10 个 App ID；同时最多 3 个侧载 app；只有一张开发证书（和 Xcode、AltStore 等会互相顶掉）；不能用 Network Extension。付费账号（99 美元/年）是 1 年有效、设备上限 100。

### 3.2 我们的方案

phonefarm 已有 `ios.rs`（WDA，serial `ios:<udid>`，能看能动，没有遥测）。分两步：

| | A. WDA（现成） | B. 原生桥（新写 `ios_native.rs`） |
|---|---|---|
| 适用 iOS | 17–26（也能用于 27） | 27+ |
| 手机上装什么 | WDA runner（要签名） | 不装 app |
| 7 天重签 | 需要（免费账号）；节点上跑续签守护，剩 2 天时续 | 不需要 |
| 宿主依赖 | Xcode 构建并签名 WDA（一次）、iproxy | idevice（Rust crate，MIT）、DDI 文件 |
| 工作量 | 续签守护和安装脚本 2 天 | 读树、触摸、键盘、启动 app、截图共 6–8 天 |

建议：先用 A 把 iPhone 接进农场跑通（审批和技能对 iOS 照样生效，因为都建立在 `Backend` 之上），同时做 B。B 做完后，iOS 27 设备不用再管签名，这是农场规模化的关键。

### 3.3 硬件

- **iPhone**：现在一台都没有，需要 Yao 提供。建议一台 iOS 26.4 以上、能升到 27 的机型（iPhone 12 及以后），专门做测试机，不登录个人 Apple ID 的 iCloud。如果只有 iOS 18–26 的机器，就只能走 A。
- **宿主**：iPhone 用 USB 插一台 Mac。
  - macmini3 现在的问题：磁盘只剩 2.6 GB，没有 Xcode，农场手机已经迁到 office-ubuntu。
  - 走 A：WDA 在本机 MacBook 上用 Xcode 构建签名一次，产物拷到节点，用 `ios-deploy` 或 idevice 安装。节点只要 libimobiledevice 和 iproxy，不需要 Xcode。
  - 走 B：Linux 也行（usbmuxd + idevice），可以直接插 office-ubuntu，和安卓机放在一起。
- 都需要一个 Apple 账号做签名（A）或配对（B 只需要信任这台电脑）。建议单独注册一个免费账号，不用 Yao 的主账号。

### 3.4 和租约体系统一

- serial 沿用 `ios:<udid>`，`platform::open` 按前缀选后端（A：`ios:`；B：`ios-native:`，或在 `ios:` 内部按 iOS 版本自动选）。
- 设备登记（`device_registry`）给 udid 起别名，devlock / devlease 不需要改：租约锁的是设备 id，和平台无关。
- `fleet` 和 `devices` 已经列出 `idevice_id -l` 看到的设备。要补的是 `fleet` 里 iOS 的健康项：WDA 是否在线、签名剩几天、DDI 是否挂载。
- 能力声明：iOS 后端的 `layers()` 照实写「只能看和动」，没有遥测、shell、sysfs；center 不会把性能类任务派给它。

## 4. 实施优先级

| 优先级 | 内容 | 状态 / 工作量 |
|---|---|---|
| P0 | 审批（phonefarm 侧）+ 技能库 + center 审批 API | 🟢 本 PR |
| P0 | 真机实测一个日常任务（RedMagic）：生成技能 → 零模型回放 → 触发审批 | 本 PR 内完成 |
| P1 | center 网页审批卡 + Telegram 推送 / 回复审批 | 2 天 |
| P1 | 动作三态回执 + handoff | 3.5 天 |
| P1 | iPhone 路线 A：WDA 签名、安装、续签守护、fleet 健康项（需要硬件） | 2–3 天 |
| P2 | iPhone 路线 B：idevice 原生后端（iOS 27） | 6–8 天 |
| P2 | 技能 `read` 段、中途纠偏、iOS 自动解锁 | 4 天 |
| P3 | 记忆锁定和变更记录、拟人化输入、iOS 录屏 | 3 天 |

## 5. 真机实测（2026-11-25，REDMAGIC `91253241019A`，office-ubuntu，devlock 租约下，模型 kimi-k2.7-code）

| 局 | 内容 | 结果 |
|---|---|---|
| 录制 | 「打开便签，新建一条便签，正文为「pftest bread jam」，保存后回到列表」+ `--save-skill note-add2` | 达成；48.9 s，9 次模型调用；自动存成 4 步技能，目标模板 `…「{{text_1}}」…` |
| 回放 | 同一句话换成「pftest coffee tea」+ `--skill auto` | 达成；30.5 s，**0 次模型调用**；uiautomator 独立核对，便签确实存在 |
| 审批·拒绝 | 「删除正文为「pftest coffee tea」的便签」，模拟审批人拒绝 | 「删除」两次被拦、两次都被拒，动作没有下发；模型报告「需人工批准，卡在此处」，复核 `achieved=false`，便签保留 |
| 审批·允许 | 「把三条 pftest 便签全部删除」，模拟审批人允许 | 达成；110 s；问了 2 次，其余删除点击在 45 s 有效期内沿用允许 |
| 回放·画面不对 | 上一局把手机留在编辑页，再回放技能 | 第 1 步判「起点画面不符」，什么都没发就停下（`step_precondition_failed`） |
| 修复后复测 | 回放「pftest final」，再在允许下删除 | 回放达成，30.6 s，0 次模型调用，终态词通过；删除一次问人，确认框交给模型，71 s 达成；SoC 33 °C |

模拟审批人写的是 `<id>.decision.json`，和 center API 写的文件相同（API 本身有单元测试）。

实测中发现并已修复：

- **gate 误拒（已有缺陷）**：安全窗口（便签编辑页）截图全黑，「空白背景」像素检查拒掉了树里真实存在的输入框，局卡死进看门狗。现在点名已吸附到元素时跳过像素检查。
- **技能输入步盲打**：`type` 步原来不记起点路标。现在除 launch 外每步都记。
- **删除确认框的「确定」漏审**：确认框文案带删除词（「要删除此记事吗?」）时，点确认按删除处理；同应用 45 s 内已批过删除就不重复问。
- **弹窗规则抢答（已有缺陷）**：UniversalEngine 会在模型主动打开的删除确认框上点「取消」，一局里连点 5 次，删除始终完成不了。现在上一步刚放行了敏感动作的话，下一帧不交给弹窗规则。
- **技能终态词没参数化**：换参数回放时，终态还在找旧文字。现在终态词里的输入文字也换成 `{{参数}}`。

记下但未修：

- 安卓不能输入中文（见映射表 P1）。
- 审批被拒后模型偶尔会原样再请求一次，会再问一次人。P2：同一局里被拒过的同目标动作直接拒，不再打扰人。

## 6. 本 PR 的接口

- `phonefarm.toml`：

```toml
[approval]
send = "ask"         # ask | auto | deny（发送、通话）
password = "ask"
account = "ask"
critical = "ask"     # 付款、删除、钱包：ask | deny（auto 无效）
timeout_s = 300
grant_s = 45
wallet_pkgs = []
messaging_pkgs = []
```

  也可以用环境变量 `PF_APPROVAL_SEND` 等覆盖。

- 落盘契约：`$PF_JOB_OUT/approvals/<id>.json`（schema `phonefarm/approval@1`）、`<id>.jpg`、`<id>.decision.json`（`{"decision":"allow|deny","by","note"}`，先写入的为准）。
- center API：
  - `GET /jobs/<jid>/approvals[?pending=1]`
  - `GET /jobs/<jid>/approvals/<ap>/screenshot`
  - `POST /jobs/<jid>/approvals/<ap>`，请求体 `{"decision":"allow"}`（只读 token 拒绝）
- CLI：
  - `phonefarm skill list | show | save --task T --run R --name N | match "<目标>" | run N [--set k=v] [--fallback --task T]`
  - `phonefarm run ... --skill auto|N --save-skill N`
