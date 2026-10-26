# 结果不完整时的下一步

METHOD-09在已有固定revision的研究assessment中附加suggestions，不改变原比较、判据或判断结果。根据已有缺值、单位/范围不匹配和窗口/图像条件缺口，给出可做选项与原因，包括补全原件、复用或重采可比基线、先做可测模块、保存同状态画面，以及需要人时的局部异步协助。

每条建议含kind、title、reason、next_experiment计划文本和affected（column_id/axis_id/reason）。CLI assess原样返回JSON，Web展示并可复制建议。没有已知方向时不猜方向ID，不自动创建作业或修改方向。优先级和是否采用仍由外部Agent决定；无建议不代表全部科学前提已满足。源码ae24e6f随harness8d37abf发布main/stable，必需CI通过。唯一相关HTTP/CLI0.766秒与Web渲染0.093秒通过；没有真实设备或生产操作。
