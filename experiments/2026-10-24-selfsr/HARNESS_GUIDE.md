# 测量 harness 指南：自研超分 + 插帧的结果是怎么产生的

本文说明 STEP1–STEP6 里每一个数字的来源：代码在哪，一次 A/B 怎么跑，每个指标从哪里读，怎么统计，跑完怎么还原，以及踩过的坑。
所有代码都在 phonefarm 仓库的 `feat/selfsr` 分支。下文路径都相对于该仓库根目录。

## 1. 总体流程

```
selfsr_ab.py run --a <臂A> --b <臂B> --rounds 6
  │  开跑前: 检查设备上没有层残留; 记下所有要改的系统设置的原值 (orig.json / panel_orig.json)
  │  按 ABBA 顺序排 12 臂: A1 B1 B2 A2 A3 B3 …
  ├─ 每一臂
  │    等电池降温 → 拿设备锁 → 设好本臂的系统设置与层属性 → 冷启动游戏、挂层 → 自动进大世界
  │    → 进世界后再设刷新率和系统开关 → 静置 settle 秒 → 测量窗口 (并行采 5 路数据) → 停游戏、摘层、核对
  ├─ 收尾 (含 Ctrl-C): 所有设置写回原值并回读核对
  └─ 统计: 每对 B−A 配对差 → 精确符号翻转检验 + bootstrap 95% 区间 → report.md / report.json
```

## 2. 组件

| 组件 | 位置 | 作用 |
|---|---|---|
| Vulkan 层 `libVkLayer_refknobs.so` | `knobs/gray/layer/`，`knobs/gray/build/build_layer.sh` 用 NDK r28c 编 | 在游戏 `vkQueuePresentKHR` 前做超分（`selfsr.cpp`）和插帧（`interp*.cpp`），插帧经子图层送显（`interp_sc.cpp`）。所有行为由 `debug.knobs.*` 系统属性控制，默认全关，等于透传 |
| `phonefarm` | `src/`（Rust），`cargo build --release` | 设备锁和租约（`tools/devlock`、`phonefarm lease`）；挂层会话 `gray-layer session`；自动进游戏 `onboard`；解析 GPU 提交 `parse-trace` |
| A/B 编排 | `loop_v1/selfsr/selfsr_ab.py` | 本文的主角，见第 4–7 节 |
| GPU 提交采集 | `loop_v1/tools/ftrace_capture.sh` | 设备端 root 脚本。在私有 ftrace 实例里采 `adreno_cmdbatch_*` 等事件，采完删掉实例，不动全局 tracing |
| 一键复现 | `loop_v1/selfsr/repro.sh`（root）、`repro_noroot.sh`（非 root） | 把多组 A/B 串起来，出总报告和对比截图 |

## 3. 层是怎么挂进游戏的

**root 设备**（`gray-layer session`）

1. 会话先拿设备租约。
2. 把 `.so` 拷进游戏的 native 库目录（`/data/app/…/lib/arm64`）。
3. 设 `debug.vulkan.layers=VK_LAYER_refknobs`，然后冷启动游戏。
4. 游戏建 VkInstance 时，Android 的 Vulkan loader 会加载这个层。
5. 会话结束时删 `.so`、清属性。

每臂结束后，`selfsr_ab.py` 会再独立核对一次：`.so` 不在、属性为空。核对不通过，整轮立刻中止。

**非 root 设备**（`noroot_smoke.sh`）

1. 目标 App 必须是可调试的。我们用自编的 refbench，以及重打包成可调试的第三方游戏。
2. 设 `settings global enable_gpu_debug_layers=1`、`gpu_debug_app=<包名>`、`gpu_debug_layers=VK_LAYER_refknobs`。
3. 用 `run-as <包名> cp` 把 `.so` 放进 App 私有目录。
4. 结束时删回这三项 settings，并删掉 `.so`。

层属性（`debug.*`）用 shell 身份写。有的 root 方案里，su 所在的 SELinux 域会被 init 拒绝写 `debug.*`，而 shell 域在所有设备上都能写。

## 4. 臂（`--a` / `--b`）

| 臂 | 含义 |
|---|---|
| `native` | 不挂层，游戏原样运行 |
| `layer` / `layer:unlimit=1` | 只挂透传层；`unlimit=1` 另外解开应用进程内的限帧器（见第 8 节） |
| `hwc:<F>` | Android Game Mode 降分辨率 F，放大交给系统合成器 |
| `sr:<F>:<宽>x<高>[:<锐化>]` | Game Mode 降到 F 倍，层内超分到 <宽>x<高> |
| `ip:<合成方式>[:<投帧率>[:键=值,…]]` | 层内插帧。合成方式 `bm` = 块匹配运动补偿，`dup` = 复制上一帧，`real` = 不插帧的对照 |
| `srip:<F>:<宽>x<高>:<锐化>:<合成方式>[:<投帧率>[:键=值,…]]` | 超分和插帧同时开 |
| `gfrc:<三位>`、`<挂层臂>+gfrc:<三位>` | 厂商自带的超分 / 插帧（对标用），或与自研层同开 |

插帧臂常用的键（写进 `debug.knobs.interp_<键>`，或由脚本自己处理）：

| 键 | 作用 |
|---|---|
| `backend=sc` | 插帧经子图层送显（见第 8 节），正式结果都用它 |
| `lead_ns=25000000` | 第一张合成帧定在「现在 + 25 ms」上屏，给 GPU 留时间 |
| `stamp=0` | 不在画面上打调试标记 |
| `panel=120` | 进世界后把屏幕切到 120 Hz，回读 SF 刷新周期确认，切不到这一臂就作废 |
| `unlimit=1` | 解开进程内限帧器 |
| `sfwait=off` | 进世界后关掉定制 SF 的「等游戏帧」特性（见第 8 节），收尾写回开 |
| `sfovr=<Hz>` | 用 SF 后门给游戏 uid 下帧率覆盖（排查用，正式结果不需要） |
| `sc_grid`、`sc_kick_ns`、`sc_real_wait`、`sc_gap_ns`、`sc_pacer`… | 子图层节奏的排查开关，默认全关 |

## 5. 一臂的完整步骤

1. **降温**：`--cool-max 40` 时，先停游戏、熄屏，直到电池温度 ≤ 40 °C（最多等 15 分钟）。过热时系统会压帧率、弹高温窗口。整轮用 `--fan` 把机身风扇开到最高，收尾写回原值。
2. **加锁并设置**：不挂层的臂整臂在 `tools/devlock` 下跑。挂层的臂由 `gray-layer session` 持有设备租约。设置内容包括 Game Mode 干预、`debug.knobs.*` 和厂商功能开关。
3. **冷启动并进世界**：`phonefarm onboard --enter-only` 用截图和 OCR 识别界面，点「点击进入」等按钮进到大世界。它认不出的游戏，用 `--enter-script 's:<秒>;t:<x>,<y>;…'` 固定点击序列。不登录、不改游戏内任何设置。
4. **进世界之后再设**：刷新率、`sfwait`、`sfovr`。原因是有的系统在游戏启动时会把刷新率压回 60 Hz，启动前设会被覆盖。
5. **静置**：`--settle` 秒（默认 20）。
6. **测量窗口**：`--window` 秒（正式结果用 60），5 路数据并行采集，见第 6 节。
7. **收尾**：停会话、摘层、`am force-stop`，核对层已不在。

## 6. 每个指标从哪里来

| 报告里的指标 | 来源 | 说明 |
|---|---|---|
| 游戏图层上屏帧率 `display_fps` | `dumpsys SurfaceFlinger --timestats`，窗口开始时清零，结束时 dump | 游戏 SurfaceView 图层的 `totalFrames ÷ displayOnTime` |
| 子图层上屏帧率 `display_fps_sc`、间隔 p95 | `dumpsys SurfaceFlinger --latency selfsr_interp#N` 的实际上屏列，每秒清一次、取 6 段 | timestats 不统计子图层，只能这样量。latency 缓冲只存 128 帧，所以按 1 秒分段 |
| 实际看到的帧率 `seen_fps` | 有子图层时取子图层，否则取游戏图层 | 子图层是不透明的，盖在游戏图层上 |
| 游戏渲染帧率、渲染 p95 帧时、每帧 GPU 忙碌 | `ftrace_capture.sh` 采 GPU 提交/退役事件，`phonefarm parse-trace` 只统计游戏进程自己的线程 | 衡量游戏自己画了多少帧，与上屏数无关 |
| 插帧给真实帧加的延迟 `sc_added_delay_p50_ms` | 层内记录：真实帧从「游戏调 present」到「排定的上屏时刻」的间隔 | 这是下限：提前量 + 合成帧占的那一格；实际上屏只会更晚 |
| 窗口内真实帧数 / 合成帧数 | 层的 marker（会话 `marker` 动作）和 `knobs_interp_frames.csv` | 用于确认层确实加载、确实在插帧 / 超分。没加载或没超分的臂作废 |
| 锐度、HUD 区锐度 | 窗口结束时截图，拉普拉斯方差；HUD 区是右上角那排固定图标 | 整屏锐度受场景影响大，HUD 区在各臂之间内容相同 |
| SSIM | 与本轮第一张原生截图比 | 冷启动后时间和云层会变，只作参考 |
| 送显缓冲尺寸 | `dumpsys SurfaceFlinger` 的 HWC 层表，游戏图层的 Source Crop | 用来确认降分辨率或超分确实生效 |

## 7. 统计和作废规则

- **ABBA 配对**：第 i 对的两臂紧挨着跑，奇偶对的先后顺序互换，以抵消发热和时间漂移。
- **作废**：出现以下任一情况，这一臂作废，它所在的那一对整对不进统计，并写进报告的「作废臂」：
  - 有报错；
  - 测量时游戏不在前台；
  - 挂层臂的层没加载或没摘干净；
  - 屏幕没切到要求的刷新率（窗口开始前和结束后各回读一次；系统可能在窗口中途把刷新率压回去）；
  - 超分臂没在超分。
- **检验**：对每个指标算 B−A 配对差，做精确符号翻转检验（6 对时可能的最小 p 是 2/64 ≈ 0.031），再用 bootstrap 给出差值的 95% 区间。
- **输出**：
  - `report.md` 是对比表；
  - `report.json` 是同样的数据；
  - `arms.jsonl` 每臂一行，含全部原始字段；
  - `traces/` 下有 timestats、子图层 latency、ftrace、SF 摘要；
  - `shots/` 下是每臂的截图；
  - `onboard/` 下是进门过程的截图和 OCR 记录；
  - `restore.json` 是收尾核对的结果。

## 8. 插帧要越过的三道系统关卡

插帧的目标是在 60 帧的游戏上做到上屏约 120 帧。在实际设备上，算法本身之外还有三道关卡。

1. **进程内限帧器**：部分系统定制的 `libgui` 在应用进程内限制每秒送显次数。层多送的合成帧也算在里面，结果只是把一半真实帧换成了合成帧，总数不变。
   - 层的 `unlimit=1` 在进程内找到这个限帧器的配置，把它解开。
   - 插帧本身改走 **子图层**：在游戏窗口下挂一个 `ASurfaceControl` 子图层，真实帧拷一份、合成帧画进层自己的 AHardwareBuffer，用 `ASurfaceTransaction` 直接交给 SurfaceFlinger，不经过被限帧的缓冲队列。
2. **按 vsync 排上屏时刻**：
   - 同一游戏帧的合成帧和真实帧如果在同一个 vsync 就绪，SF 只显示后一张。所以第一张定在「现在 + 提前量」，后面每张间隔至少一个刷新周期，每张都带 `desiredPresentTime`。
   - 子图层还要做旋转：游戏按 `preTransform` 预旋转时，子图层设相同的反向变换。
3. **定制 SF 的「等游戏帧」特性**：
   - 现象：有的定制 SurfaceFlinger 在合成时，如果游戏图层的缓冲 GPU 还没画完，会每 1 ms 轮询一次，最多等 7–8 ms。这一帧因此合成晚了，下一个 vsync 因为 backpressure 整帧跳过。那一格排好的子图层缓冲，下次和后一张一起就绪，前一张被顶掉。最终上屏卡在约 89 帧。
   - 定位方法：在测量窗口内另开 `atrace -t 3 gfx view`，看 SF 主线程的 `commit` → `fence unsignaled` → 一串 `wait` → 下一次 `PrevFramePending` / `onCommitNotComposited`，以及子图层的 `releasePendingBuffer`。
   - 处理：该 SF 自带运行时开关 `dumpsys SurfaceFlinger --fpsControl_waitframe_off`，不改系统文件。`sfwait=off` 在进世界后关掉它，收尾写回开。关掉后子图层上屏约 118–120 帧。
   - 排查中试过、但**无效**的做法（STEP4 有完整表格）：
     - 用 SF 后门（`service call SurfaceFlinger 1039`）把 uid 帧率覆盖改成 120；
     - 定时给 SF 发空事务叫醒它；
     - 把上屏时刻对齐 vsync 网格；
     - 真实帧交出前在 CPU 上等 GPU 做完（游戏渲染掉到 22 帧）。

## 9. 复现命令

```sh
# 编层 (NDK r28c, 也可用 ANDROID_NDK_HOME 指定)
bash knobs/gray/build/build_layer.sh

# root 设备: 原生 vs 自研超分 + 插帧, 6 对
python3 -I loop_v1/selfsr/selfsr_ab.py run --serial <serial> --out <目录> --a native \
  --b 'srip:1:<屏宽>x<屏高>:1.0:bm:120:stamp=0,panel=120,backend=sc,lead_ns=25000000,unlimit=1,sfwait=off' \
  --rounds 6 --fan --cool-max 40 --window 60

# 只看插帧本身: 透传层 (解限帧) vs 插帧
python3 -I loop_v1/selfsr/selfsr_ab.py run --serial <serial> --out <目录> --a 'layer:unlimit=1' --b '<同上>' --rounds 6 --fan --cool-max 40 --window 60

# root 设备: 多组串跑, 出总报告
sh loop_v1/selfsr/repro.sh <serial> [配对数] [输出目录] [包名]

# 非 root 设备: refbench 钉 60 帧 + 可调试第三方游戏, 出 report.md 和 *_cmp.png
sh loop_v1/selfsr/repro_noroot.sh <serial> [轮数] [输出目录]
```

前提：

- root 路径：游戏已装好并登录过（onboard 能进大世界）。
- 非 root 路径：已装可调试的 refbench（`REFBENCH_DEBUGGABLE=1` 编），可选装可调试的游戏包。

## 10. 踩过的坑

- **收尾可能在另一个进程**：每臂的设置和收尾经 `tools/devlock run` 在子进程里执行，主进程里记的状态传不过去。凡是「进世界后才设」的东西，收尾都按设备上的实际状态清理，不依赖进程内变量：
  - `sfovr`：读 SF 里现有的后门条目，逐条删除；
  - `sfwait`：在有这个开关的系统上一律写回开。
- **不要并行跑两轮同一台设备**：后一轮开跑时会把前一轮改过、还没还原的值记成「原值」。前一轮完全退出后再开下一轮。
- **不要删还在恢复中的输出目录**：挂层会话失败时，租约会留下「需恢复」屏障，恢复要读会话目录里的记录。删了目录就只能手工清租约（备份租约目录，并在 `phonefarm-recovery.jsonl` 里记一条原因）。
- **锁队列是全局的**：一台设备的残留请求会挡住所有设备。被杀掉的进程留下的队首请求，10 分钟后才会被跳过。
- **HWC 层表的列数因系统而异**：有的系统多一列 `[*]`。Source Crop 要按「4 个带小数点的数」识别，不能按固定列号取。
- **热**：同一设备连续跑，电池温度会逼近系统的降帧阈值，屏幕会被压回 60 Hz（这一臂会因 `panel` 回读失败而作废）。降温阈值设低一点（40 °C），配合风扇。
- **算延迟用的时钟**：层用 `CLOCK_MONOTONIC`，`/proc/uptime` 包含休眠时间，两者对不齐。窗口统计以层 CSV 里最后一条上屏时刻为终点往前取。
