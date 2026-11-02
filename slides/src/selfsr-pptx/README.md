# 移动游戏超分辨率与插帧阶段结果：PPTX 生成源

本目录为 `slides/2026-11-02-超分与插帧阶段结果.pptx` 的生成源。内容与网页版 `slides/web/selfsr-deck` 共用一份（本目录的 `content.json` 即其 `content/deck.json`）；修改数字时先改本目录，再复制至网页版。

生成方法采用 octos-org/octos `crates/octos-agent/resource-skills/pptx` 技能的「无字底图 + 原生文字」路径：

1. `python3 -I layout.py art`：按内容排版，输出各页无字底图的图形描述 `art/*.json`（卡片、线条、标记点、三维柱与方块背景）。
2. 在本目录启动静态服务后运行 `python3 render_art.py`：由浏览器中的 three.js（`art.html`）渲染 3840×2160 无字底图 `art/*.png`，并输出三维柱的投影坐标 `art/anchors.json`。
3. `python3 -I layout.py spec`：输出技能脚本 `native_deck.py` 所需的页面数据 `deck.json`，文字框坐标与底图共用 1920×1080 版面。
4. `python <技能目录>/scripts/native_deck.py build --spec deck.json --out build` 生成 `build/editable.pptx`，再以 `check_pptx_compat.py` 检查。

配色取自 hgamey-deck（`src/gen.rs`）。默认字体为微软雅黑；预览环境缺少该字体时，可通过 `DECK_FONT=<本机字体>` 另行生成一份用于检查版式。图形为位图底图，文字均可编辑。
