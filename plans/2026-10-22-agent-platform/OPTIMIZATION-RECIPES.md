# 优化路线库

METHOD-RECIPES-01使用已有research-plans.json共享十条路线：分辨率/轻量重建、LOD/阴影/着色、AA/后处理、合批/状态复用、资源布局/缓存、CPU更新与并行、CPU/GPU/NPU分工、帧节奏、系统旋钮、灰层附件/带宽。每条写明能力前提、可尝试改动、测量范围与恢复。

`pf-node research recipes [ID]`离线浏览，`pf-node research plan-template exploration --research-revision N --recipe draw-submit`生成已有next_experiment格式；confirmation同样可用。网页方向编辑器可浏览内容并填入同一草稿，保留覆盖前草稿。排序仍由外部Agent决定，选择路线不创建作业或宣称设备具备能力。

具体游戏需要补实际输入、命令、版本、场景和预期收益；这批交付可复用研究计划，不把建议当通用执行脚本。主基线为原游戏，模块ms与允许画质损失可成为限定收益；功耗和每帧能耗按实际来源另报。JS语法与实际离线CLI模板/列表已检查，无设备/训练/大资源操作。源码3e9695f随harness33b7aa8发布main/stable，必需CI通过。
