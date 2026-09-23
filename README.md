# HGamey 文档资料

演示文稿、架构图等不属于代码的文档，统一放在这里。项目现状以 [game_opt_loop/STATUS.md](https://github.com/HGamey/game_opt_loop/blob/main/STATUS.md) 为准。

## 目录

| 路径 | 内容 |
|---|---|
| `slides/2026-09-24-HGamey-Harness.pptx` | 2026-09-24 项目介绍（harness 视角，含鸿蒙 nova 16 实测） |
| `diagrams/01-harness-overview` | harness 全貌 |
| `diagrams/02-repos` | 三个仓库的调用关系 |
| `diagrams/03-optimize-flow` | 一次优化任务的流程 |
| `diagrams/04-layers-matrix` | 三层优化 × 能不能做 |
| `diagrams/05-measure-judge` | 测量与判定 |
| `diagrams/06-devices-queue` | 设备与排队 |
| `diagrams/07-architecture` | 系统架构 |

每张图都有 `.drawio`（可编辑）、`.png`、`.svg` 三个文件。

## 约定

- 新版演示文稿用「日期-主题」命名放进 `slides/`，旧版保留，不覆盖。
- 修改图时改 `.drawio`，再导出同名 `.png` 和 `.svg`。
