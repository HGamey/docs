# 超分与插帧阶段结果：网页幻灯片（Makepad 2.0 / Splash + makepad-d3，WASM）

2026-11-02 与东南大学赵普教授交流用。引擎、外壳、三维背景与图表与 [`../hgamey-deck`](../hgamey-deck) 相同（整份复制，包名改为 `sr-zhao-deck`），只换了 `content/deck.json`；`data/` 为空，所以「结果浏览」没有内容。

线上地址和访问密码只记在私有仓库 `HGamey/internal` 的 `decks/sr-zhao-deck.md`，不写在这里。

## 改数字（不碰排版）

只改 `content/deck.json`。块的写法见 `../hgamey-deck/README.md`；尚无数据的条目写成 `{"t": "pending", "pending": true, "title", "what", "job"}`，有数据后换成 `forest` / `table` / `chart3d` 等块。数字来源写在每页 `foot`。

## 构建与部署

工具链、补丁与目录布局同 `../hgamey-deck/README.md`（`agent/tools/makepad`、`makepad-d3` 打好 `patches/` 下的补丁）。

```bash
FONTTOOLS_PY=/path/to/python-with-fonttools bash build.sh        # 产出 dist/
FONTTOOLS_PY=... bash build.sh serve                              # 本地预览 http://127.0.0.1:8123（无密码）
cd dist && vercel link --yes --project sr-zhao-deck --scope <团队>  # build.sh 会删掉 dist，重新链接
vercel deploy --prod --yes                                        # 项目已设环境变量 SITE_PASSWORD
```

`deploy/middleware.js` 与 hgamey-deck 相同，cookie 名和盐不同（`sr_zhao_auth`、`sr-zhao-deck:`）。
