# HGamey 进展网页幻灯片（Makepad 2.0 / Splash + makepad-d3，WASM）

线上地址和访问密码只记在私有仓库 `HGamey/internal` 的 `decks/hgamey-deck.md`，不写在这里。

## 内容

- 16 页幻灯片 + 「结果浏览」（模型排行榜、画面对比裁剪、真机效应量与 95% 区间）。
- 数据图是三维的（`src/chart3d.rs`，`d3.Chart3D`）：与背景同一套画法（CPU 正交投影、按深度排序、平行四边形面片），
  支持立体柱（类别 × 系列、对数轴、参照面、各列独立归一化）和三维散点（落地垂线、Pareto 前沿）；拖动可小角度旋转，
  进入页面时柱子长出约 0.6 s，之后静止不占帧。规格写在 deck.json 的 `chart3d` 块里。分类矩阵（检出 / 漏检、覆盖）用 `heat` 着色表。
- 背景是三维方块场景：一个小人沿 8 字路线在方块顶面上走，相机平移跟着它；翻页时相机转一个角度。纯装饰，不代表数据。
- 亮 / 暗两种主题：系统为暗色时自动用暗色（index.html 里补 `?theme=dark`），T 键或底栏按钮切换。
  幻灯片正文按色值映射成暗色；结果浏览与总览保持亮色底板。
- 操作：←/→/空格翻页，Home/End，O 总览，E 结果浏览，T 主题，Esc 关闭；触屏左右滑动翻页。
- 页面上不出现画质「通过 / 不通过」判词：画质给三项指标和原生自噪声参照线，性能给效应量、区间、p、对数。
- 公开仓里不写可操作的系统绕过细节；游戏画面裁剪图放在私有 internal 仓（见下）。

## 改数字（不碰排版）

| 文件 | 内容 |
|---|---|
| `content/deck.json` | 每页文字、表格、图表数据。占位条目是 `{"t": "pending", "pending": true, "title", "what", "job"}`；有数据后换成 `bars` / `forest` / `table` 等块即可。 |
| `data/*.json` | 结果浏览用的数据，由 `refresh-data.sh` 从 `hgamey/_tmp` 生成（排行榜、真机判定 JSON、来源 sha256）。 |
| 私有：`internal/decks/hgamey-deck/private/`（`crops.json` + `crops/*.png`） | 「画面对比」的游戏画面裁剪。不进公开的 docs 仓；`refresh-data.sh` 直接写到这里（或环境变量 `HGAMEY_DECK_PRIVATE` 指定的目录），`build.rs` 从这里嵌入。目录不存在时照样能构建，只是画面对比为空。 |

都在编译时嵌入 WASM，改完重新构建。

## 构建

目录布局（Cargo.toml 用相对路径引用，二者与 `hgamey/` 同级放在 `agent/tools/` 下）：

```
agent/tools/makepad      makepad/makepad main @ c6497c7af585870236a4010338ac569ea607f816
agent/tools/makepad-d3   mofa-org/makepad-d3  @ da181cedf4e40875c2c106f169087a843e62e771
```

两处检出要打 `patches/` 下的补丁（`git apply` 即可）：

- `makepad-c6497c7-web-init.patch`
  1. `platform/src/os/web/web.rs`：网页端 `ToWasmInit` 在应用 Startup 创建窗口之前就访问 0 号窗口，直接越界 panic（makepad 自带的 slides 示例同样崩）。改为窗口不存在时只记几何信息。
  2. `widgets/src/widget_async.rs`：Splash 的运行时间预算用 `std::time::Instant`，在 wasm32-unknown-unknown 上 panic；wasm 下不设时间预算。
  3. `widgets/src/lib.rs`：wasm 下主题字体只有拉丁字母，中文全是方块；给 wasm 主题字体加上霞鹜文楷中文成员。
- `makepad-c6497c7-splash-utf8-reeval.patch`（第 5 页「无响应」的根因修复）：Splash 增量求值比较新旧内容时按字节长度切
  `&str`（`platform/script/src/vm.rs`），旧内容长度落在新内容某个中文字符中间就 panic；panic 之后 makepad 的事件处理器
  已被取走，之后每个事件都再 panic 一次（`cx_shared.rs:975`），页面停在最后一帧、键盘鼠标全部失效。改为按字节比较并检查字符边界。
- `makepad-d3-da181ce-chrono-no-wasmbind.patch`：chrono 默认开 `wasmbind`，会引入 wasm-bindgen 导入，makepad 的 JS 加载器不认；关掉默认特性。
- `makepad-d3-da181ce-splash-extra-mods.patch`：`d3.Splash` 在独立的沙箱 VM 里求值正文，只注册了 makepad-d3 自己的模块；
  加一个登记口（`register_splash_mod`），让本 deck 把 `Chart3D` 及其着色器也注册进沙箱。

工具链（2026-10-21 实测可用）：

- cargo-makepad：`cargo install --path tools/cargo_makepad`（从上面的 makepad 检出安装）
- rustc nightly 1.95.0-nightly (9e79395f9 2026-02-10)，target `wasm32-unknown-unknown`（cargo-makepad 用 nightly 编 wasm 多线程）
- Python 3 + fonttools（裁剪中文字体；`python3 -m venv v && v/bin/pip install fonttools`，再设 `FONTTOOLS_PY=v/bin/python`）
- Vercel CLI 50.32.5

```bash
FONTTOOLS_PY=/path/to/python-with-fonttools bash build.sh        # 产出 dist/
FONTTOOLS_PY=... bash build.sh serve                              # 本地预览 http://127.0.0.1:8123（无密码）
```

在非 `docs/` 原始路径的检出（例如 `hgamey/_wt/deck-slides` 工作树）中构建时，`Cargo.toml` 的同级工具目录与 `build.rs` 的私有数据默认路径都会算错，需要显式指定：

```bash
ln -sfn ../tools hgamey/tools     # 使 ../../../../../tools 指向 agent/tools
FONTTOOLS_PY=... HGAMEY_DECK_PRIVATE=/path/to/hgamey/internal/decks/hgamey-deck/private bash build.sh
```

不设 `HGAMEY_DECK_PRIVATE` 也能构建，但 wasm 会小约 1.4 MB，结果浏览的「画面对比」为空。构建后 wasm 约 9.98 MB 可作校验。

部署：`dist/.vercel/project.json` 指向项目 `hgamey-deck`（`build.sh` 会 `rm -rf dist`，工作树里需重新写回），再 `vercel deploy --prod --yes`。缺这个链接时 Vercel 会按目录名新建一个 `dist` 项目。

`build.sh` 在 cargo-makepad 产物之上做了三件事：

1. 把 makepad JS 里写死的共享内存初始值从 64 页（4 MB）调到 256 页：本 deck 嵌了数据和裁剪图，静态段超过 4 MB，否则加载时 LinkError。
2. 中文字体（各 18 MB）按 deck.json / data / src / 私有目录 json 里出现的字符裁成约 410 KB。新增文字后要重新构建，否则新字显示成方块。
3. 复制 `deploy/middleware.js`（与 kdn-deck 相同的密码中间件，cookie 名和盐不同）和 `deploy/vercel.json`（COOP/COEP 头，WASM 多线程需要）。
4. 在 index.html 里加一行脚本：系统为暗色且 URL 没带 theme 时补上 `?theme=dark`。

## 部署

```bash
cd dist
vercel deploy --prod --yes                      # 首次会创建项目 hgamey-deck
vercel env add SITE_PASSWORD production         # 密码见 internal/decks/hgamey-deck.md
vercel deploy --prod --yes                      # 加了环境变量后再部署一次
```

## 已知问题

- Splash 首次求值时字形图集可能还没就绪，部分文字会画成方块；`main.rs` 在每次设置内容后 0.5 s 和 2 s 各重算一次兜底。
- 主题切换时外壳文字用 `Label::set_text_color`；用 `script_apply_eval!` 改已绘制文字的颜色会在 `draw_vars.rs:239` 越界 panic，
  所以底栏做了亮 / 暗两套、按主题切换可见性。
- 原神进程内 NPU 推理只保存了 60 帧的均值与最长值，逐帧数据没有，第 8 页因此不画分布图。
- WASM 约 10 MB，另有 makepad 自带的 emoji 字体 10 MB（主题字体引用了它，没试过删掉会怎样）；首次打开较慢。
