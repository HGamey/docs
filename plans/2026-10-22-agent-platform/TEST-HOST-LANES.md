# 宿主回归入口与真实等待

TEST-03 已发布 phonefarm `3494bf2a97b6cbb1169362fc7565a42a3421025b`，先通过 [code-only 40282885216](https://github.com/HGamey/phonefarm/actions/runs/40282885216)，再同 SHA 推 main。没有修改生产超时、心跳或 lease 阈值。

`python3 tools/test_host.py fast --out <新目录>` 只运行 binary 单测。`driver` 运行 confirmation_driver_cli，默认一个测试线程；`integration --target <名称>` 显式选择一个集成目标。可用 `--filter <名称> --exact` 定向，`--plan` 先显示命令；`--cargo` 指定工具路径。每次输出独立回执、构建与选择/运行日志，零匹配失败；没有自动重试。Cargo 默认覆盖未改，没有 ignore 长测试。

confirmation_driver 单 test binary 内只允许一个持有真实计时/进程 fixture，fixture 内原有竞争仍执行。记录 admission 等待与各 CLI 单调耗时；panic 时保留原件，避免旧测试无证据可追。

历史 45 项并行运行中 43 过、两个 deadline/heartbeat 失败；当时至少八项同时超过 60 秒，单项复验曾通过。旧失败 fixture 已删，无法事后证明具体 CPU/I/O 根因。本批解决回归并发失控和证据丢失，不宣称证明了所有偶发延迟原因，也不取消宿主过载时的生产失败规则。

本机冷构建 26.06 秒。两个历史失败与现有真实竞争测试，3 libtest 线程单次 3/3 通过，共 42.82 秒；fixture 无重叠。runner 实际 CLI 冒烟 1/1 通过；零匹配预期失败。第一条 shell PATH 缺 Cargo 的失败日志原样保留，随后显式用已安装 rustup 路径，不重复安装工具链。只有一个共享 target，约 1.10 GB。QA：任务目录 `qa/resume-local-20261026/test-timing/`；无真机操作或生产 lease 修改。
