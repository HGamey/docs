# refbench 输出合同的宿主已知答案

METHOD-04 的 REFBENCH-OUTPUT-01 子项修复两个可达问题：生产输出用固定512字节缓冲拼接 postfx，较长 shader 路径会截断整个JSON；实际 compute pass 使用 load_op=NONE，但 schema 不允许该既有值。

生产 `output_json` 改为调用无 Android/Vulkan 依赖的共用序列化函数；动态字符串逐字段转义，不截断 shader/pass 名称。schema 接纳 compute 的既有 NONE 含义。没有新增 wire 字段、改变Vulkan渲染或从JSON成功推断设备资格。

四个宿主测试直接调用生产序列化函数，核四scene已声明参数、SR on/off与失败、quality_only、长字符串/控制字符/Unicode及相关schema字段。另跑已有native-pair文件桥和CLI严格失败/成功两项。首次全部通过，共3.183秒；两个小C++ fixture编译，没有NDK、Cargo缓存、APK或真机操作。QA仅18.7KB，位于任务 `qa/resume-local-20261026/refbench-known-answer/`。

这证明“给定宿主声明事实后输出正确”，并不证明实际Vulkan pass、渲染像素、native LR/HR画质、Android ABI、设备lease、帧时间或能耗。完整METHOD-04保持开放。已发布 refbench `c04ec4ba78d7f04c18b2c211b4e99233d258f087`，先通过[code-only 40285600650](https://github.com/HGamey/refbench/actions/runs/40285600650)，同SHA直接推main。
