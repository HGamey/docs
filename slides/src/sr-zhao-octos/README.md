# 超分与插帧阶段结果：PPTX 生成源

`slides/2026-11-02-超分与插帧阶段结果.pptx` 的来源。内容与网页版 `slides/web/sr-zhao-deck` 同一份（`content.json` = 其 `content/deck.json`），改数字先改这里再复制过去。

做法按 octos-org/octos `crates/octos-agent/resource-skills/pptx` 技能的「无字底图 + 原生文字」路径：

1. `python3 -I layout.py art`：按内容排版，写出每页无字底图的图形 `art/*.json`（卡片、线、点、三维柱、方块背景）。
2. 起静态服务后 `python3 render_art.py`：浏览器里的 three.js（`art.html`）渲染 3840×2160 底图 `art/*.png`，并回传三维柱的投影坐标 `art/anchors.json`。
3. `python3 -I layout.py spec`：写出技能 `native_deck.py` 的页面数据 `deck.json`（289 个原生文字框，坐标与底图同一 1920×1080 版面）。
4. `python <技能>/scripts/native_deck.py build --spec deck.json --out build` → `build/editable.pptx`；`check_pptx_compat.py` 检查。

配色取自 hgamey-deck（`src/gen.rs`）。字体为微软雅黑；没有雅黑的预览环境用 `DECK_FONT=<本机字体>` 另出一份看排版。图形是位图底图，文字全部可编辑。
