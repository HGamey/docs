## Native LR / HR 质量采集（尚未 Android / GPU 验收）

[设计与边界](docs/SPEC_NATIVE_PAIRED_CAPTURE.md)；数据缺口 [#15](https://github.com/HGamey/.github/issues/15)。
新增 `capture.native_pairs=on`，在相同 `frame_index` / `phase` / `iters` 下分别
直接渲染 1280×720 native LR 和独立 1920×1080 native HR；`imgHi` 始终只是候选
compute 输出。内容是程序化白盒 ALU 场景，不能证明原神或其他真实游戏优化。

采集额外加入 HR draw、readback、同步、写盘，输出标明 `quality_only` /
`performance_eligible=false`。质量采集与正常性能/功耗实验必须分开运行。
规则必须在采集前冻结，采集记录实际时钟，导入不替换时间或阈值。

```bash
# 需先用正常 Android 工具链构建，并在设备端放置既有 candidate.spv 和冻结规则。
# 此命令是待真机验收用法，本 PR 未执行设备操作。
adb shell am start -n io.github.hgamey.refbench/android.app.NativeActivity \
  --es scene sr_pipeline --es run_id native_quality_001 --es frames 60 --es intensity 6 \
  --es knob.postfx on --es postfx.shader /data/local/tmp/postfx/candidate.spv \
  --es capture.native_pairs on --es capture.start 0 --es capture.count 4 --es capture.stride 1 \
  --es capture.rules_path /data/local/tmp/postfx/frozen_rules.json \
  --es capture.rules_sha256 "$RULES_SHA256"
# 进程结束后，pull 整个 files/capture_native_quality_001 目录；不要只拉 manifest。
# 新完成合同必须含 completion.json + run-scoped refbench_out.json，并验证绑定；
# 旧无 marker 捕获不能回填或当新版完整采集。参见 docs/SPEC_CAPTURE_COMPLETION.md。

python3 tools/native_pairs.py validate --manifest capture/manifest.json \
  --rules frozen_rules.json --candidate-shader candidate.spv
python3 tools/native_pairs.py import --manifest capture/manifest.json \
  --rules frozen_rules.json --candidate-shader candidate.spv --out imported
phonefarm sequence-quality --reference imported/reference.json \
  --candidate imported/candidate.json --rules imported/frozen_rules.json \
  --out quality_report.json
```

桥保留原始 RGBA、raw/capture manifest/candidate SHA、帧号/相位/原生 role，
共同执行 linear UNORM8→sRGB8 转换。phonefarm 只比较 native HR 与候选；native LR
PNG 另存为模型输入。当前来源枚举没有 controlled whitebox，桥保守输出 `unknown`：
有限指标可为 PASS，但总体 UNKNOWN、真实游戏资格和无鬼影证明均为 false。
合成夹具可得到明确限定的指标 PASS；这不是实际 GPU 或真实游戏验收。

```bash
# 标准库 Python + 本机 C++；PHONEFARM_SEQUENCE_BINARY 可选，指向已有消费方。
PHONEFARM_SEQUENCE_BINARY=/path/to/phonefarm python3 -m unittest discover -s tests -v
python3 tests/host_syntax.py --vulkan-include /path/to/existing/include
```

本轮只验证 host SHA/严格合同/PNG/现有消费方接口与语法。Android/JNI stub 和 SPIR-V
占位不能证明 Android ABI、有效 shader、Vulkan 生命周期或 GPU 输出。没有下载 NDK、
构建 APK 或操作设备。FrameGen 中间时刻真值仍未实现。
