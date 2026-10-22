# HGamey 文档资料

演示文稿、架构图等不属于代码的文档，统一放在这里。项目现状以 [STATUS.md](STATUS.md) 为准（每周更新）。实验的请求和报告放在 `experiments/`。

## 规则：代码仓库只放代码

HGamey 的代码仓库（game_opt_loop、phonefarm、refbench、harness-ops）只放代码。说明按下表写到对应位置：

| 内容 | 写在哪 |
|---|---|
| 这次改了什么、为什么改 | commit message 正文 |
| 一个功能或修复的说明、验证方法与限制 | commit message 正文；需要长期保留的详细说明放本仓库并链接 |
| 长期有效的文档：设计、规格、架构图、进展汇报、实验报告 | 本仓库 |

代码仓库里允许保留的非代码文件：

- 根目录 `README.md`，不超过 40 行：是什么、怎么构建、文档在哪
- 给 AI 编程助手读的配置：`AGENTS.md`、`CLAUDE.md`、`skills/`、`.cursor/`
- 许可证：`LICENSE`、`THIRD_PARTY_NOTICES.md`
- 程序或测试要读取的数据：`tests/`、`fixtures/`、`testdata/`、`evidence/`，以及 phonefarm 的 `tasks/`、`loop_v1/runs*/`

每个代码仓库都有 CI 检查 `code-only`，新增或修改上述例外之外的 `.md`、图片、PPT、PDF、drawio 文件会受到检查约束。

2026-10-22 起本轮开发采用直接 commit、push 的协作方式，不新开 PR。每批只提交自己的改动，记录验证与限制，推送前核对远端并保留同事的并发修改；不 force push。已有 PR 和历史报告继续作为历史证据，不自动批量合并或关闭。

各代码仓库原有的文档已连同修改历史迁到 `repos/<仓库名>/`，路径与原仓库一致。

## 目录

| 路径 | 内容 |
|---|---|
| `plans/2026-10-22-agent-platform/` | 外部 Agent 远程真机优化平台：已确认目的、项目地图、多机/分发/队列/结果计划、网页与CLI/API共同验收及完整待办 |
| `plans/2026-10-11-system-handoff.md` | 会议需求、系统交接审计、跨仓迭代顺序与验收对应表 |
| `plans/2026-10-11-implementation-progress.md` | 本轮 draft PR、集成验证范围、剩余端到端接口与真机门槛 |
| `plans/harness-runbook.md` | 候选准备、独立确认、只读页面的实际入口与结果含义 |
| `plans/oneplus-acceptance.md` | 一加 Ace 5 接回后的资格检查、受控验证与整链路证据要求 |
| `slides/2026-09-24-HGamey-Harness.pptx` | 2026-09-24 项目介绍（harness 视角，含鸿蒙 nova 16 实测） |
| `slides/2026-10-11-HGamey-近一个月进展.pptx` | **进展 v01**（2026-10-11）：09-11 → 10-11 近一个月进展，以 BH3GEI 的工作为主线 |
| `slides/HGamey-进展-v02-2026-10-11-手机节点上线与作战室.pptx` | **进展 v02**（2026-10-11）：手机节点上线、作战室、当天合并的 32 个改动；总蓝图、节点架构、harness 全貌、提前停、画质对比图与真机截图 |
| `slides/HGamey-进展-v03-2026-10-11-原神一帧拆解.pptx` | **进展 v03**（2026-10-11）：阶段 1 完成——原神一帧 33 步拆解、白写回只有 1.3%、为什么省不下来、阶段 2 开关清单与决定（原神只做机制验证，收益转向 Megacity 和第二个商业游戏）；作战室截图放大、蓝图更新 |
| `slides/HGamey-进展-v04-2026-10-11-阶段2完成与目标模式.pptx` | **进展 v04**（2026-10-11）：阶段 2 完成——3 个「去掉多余写回」开关原神真机命中、白写回 3→0、画面差在噪声内；目标模式（定目标 → 自主执行 → 人掌舵 → 停止和报告）；功耗旁路口径、云端编译；浅色作战室截图 |
| `slides/HGamey-进展-v05-2026-10-11-Megacity拆帧与主战场.pptx` | **进展 v05**（2026-10-11）：换游戏拆帧——Megacity 白写回 7.73 MB/帧（原神 3 倍）、4 倍抗锯齿缓冲来回搬运约 186 MB/帧，阶段 3 主战场定为 Megacity；两游戏一帧对比图、Megacity 一帧瀑布图、开关清单、云端出方案成本、作战室改版预览、节点失联教训 |
| `diagrams/01-harness-overview` | harness 全貌 |
| `diagrams/02-repos` | 三个仓库的调用关系 |
| `diagrams/03-optimize-flow` | 一次优化任务的流程 |
| `diagrams/04-layers-matrix` | 三层优化 × 能不能做 |
| `diagrams/05-measure-judge` | 测量与判定 |
| `diagrams/06-devices-queue` | 设备与排队 |
| `diagrams/07-architecture` | 系统架构 |

每张图都有 `.drawio`（可编辑）、`.png`、`.svg` 三个文件。

| `repos/game_opt_loop/` | 从 game_opt_loop 迁来的文档：完整 README、STATUS、DESIGN、GOAL、接入说明 |
| `repos/phonefarm/` | 从 phonefarm 迁来的文档：完整 README、设计与各 SPEC、拆帧报告、各子目录说明 |
| `repos/refbench/` | 从 refbench 迁来的文档：完整 README、DESIGN |

## 约定

- 进展汇报按版本出：每个阶段性成果出一版完整的新 deck，旧版不覆盖，命名 `slides/HGamey-进展-v<两位版本号>-<日期>-<这版的成果>.pptx`，并在上表逐版登记。生成脚本只有一份，在 `slides/src/progress/`：每版的文字数字存 `content-vNN.json`，`python3 -I collect.py content-vNN.json --day <日期>` 刷新 PR 数据，`node build.js content-vNN.json` 出文件；图用 `fig/*.html`（引用 `fig/tokens.css`）经 `fig/render.sh` 渲染成 PNG，截图放 `img/`。
- 其他演示文稿有更新时直接覆盖 `slides/` 里的同名文件，旧版靠 git 历史查看；换主题的新文稿用「日期-主题」命名。
- 修改图时改 `.drawio`，再导出同名 `.png` 和 `.svg`。
