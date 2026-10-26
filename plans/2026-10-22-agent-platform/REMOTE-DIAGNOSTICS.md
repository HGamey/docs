# 单次远控诊断

REMOTE-DIAGNOSTICS-01复用原Android远控owner和动作回执，新增diagnostics，可选package。固定只读命令读取型号/系统/SDK/ABI、当前shell身份、屏幕尺寸密度、前台activity，以及指定包路径/版本/UID；20秒总上限、分区有界文本和原stdout摘要。不调用su，不把shell身份或空输出解释成root/场景资格。

Web复用包名框和原回执显示；CLI为`pf-remote action SERIAL diagnostics --session-id ID --request-id ID [--package PACKAGE]`，也支持原params。旧owner未声明能力则禁用新动作，原请求回执仍可查询。诊断保留原截图身份和原图龄，不重新授予点击资格。没有新增通道、身份数据库或自动修复。

源码已完成，正在提交发布。最小检查：PF既有typed边界一项0.143秒、后端HTTP替身0.543秒、客户端合成链0.031秒。未做本地全套、真机或生产验收。完整跨平台doctor与能力epoch继续按FLEET-05/04推进。
