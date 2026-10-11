# HGamey 文档资料

演示文稿、架构图等不属于代码的文档，统一放在这里。项目现状以 [game_opt_loop/STATUS.md](https://github.com/HGamey/game_opt_loop/blob/main/STATUS.md) 为准。

## 目录

| 路径 | 内容 |
|---|---|
| `slides/2026-09-24-HGamey-Harness.pptx` | 2026-09-24 项目介绍（harness 视角，含鸿蒙 nova 16 实测） |
| `slides/2026-10-11-HGamey-近一个月进展.pptx` | **进展 v01**（2026-10-11）：09-11 → 10-11 近一个月进展，以 BH3GEI 的工作为主线 |
| `slides/HGamey-进展-v02-2026-10-11-手机节点上线与作战室.pptx` | **进展 v02**（2026-10-11）：手机节点上线、作战室、当天合并的 32 个改动；总蓝图、节点架构、harness 全貌、提前停、画质对比图与真机截图 |
| `diagrams/01-harness-overview` | harness 全貌 |
| `diagrams/02-repos` | 三个仓库的调用关系 |
| `diagrams/03-optimize-flow` | 一次优化任务的流程 |
| `diagrams/04-layers-matrix` | 三层优化 × 能不能做 |
| `diagrams/05-measure-judge` | 测量与判定 |
| `diagrams/06-devices-queue` | 设备与排队 |
| `diagrams/07-architecture` | 系统架构 |

每张图都有 `.drawio`（可编辑）、`.png`、`.svg` 三个文件。

## 约定

- 进展汇报按版本出：每个阶段性成果出一版完整的新 deck，旧版不覆盖，命名 `slides/HGamey-进展-v<两位版本号>-<日期>-<这版的成果>.pptx`，并在上表逐版登记。生成脚本只有一份，在 `slides/src/progress/`：每版的文字数字存 `content-vNN.json`，`python3 -I collect.py content-vNN.json --day <日期>` 刷新 PR 数据，`node build.js content-vNN.json` 出文件；图用 `fig/*.html`（引用 `fig/tokens.css`）经 `fig/render.sh` 渲染成 PNG，截图放 `img/`。
- 其他演示文稿有更新时直接覆盖 `slides/` 里的同名文件，旧版靠 git 历史查看；换主题的新文稿用「日期-主题」命名。
- 修改图时改 `.drawio`，再导出同名 `.png` 和 `.svg`。
