# 移动游戏超分辨率与插帧阶段结果：网页幻灯片（Makepad 2.0 / Splash + makepad-d3，WASM）

本幻灯片用于对外交流时介绍超分辨率与插帧的阶段结果。引擎、外壳、三维背景与图表组件与 [`../hgamey-deck`](../hgamey-deck) 相同（整体复制，包名改为 `selfsr-deck`），仅内容文件 `content/deck.json` 不同。`data/` 目录为空，因此「结果浏览」页无内容。

线上地址与访问密码仅记录于私有仓库 `HGamey/internal` 的 `decks/selfsr-deck.md`，不在本仓库公开。

## 修改数字

仅需修改 `content/deck.json`，版式无需改动。各类内容块的写法见 `../hgamey-deck/README.md`。尚无数据的条目写作 `{"t": "pending", "pending": true, "title", "what", "job"}`，取得数据后替换为 `forest`、`table`、`chart3d` 等内容块。每页的数据来源写在 `foot` 字段。

本文件与 `../../src/selfsr-pptx/content.json` 保持一致；修改时应先改后者，再复制至此。

## 构建与部署

工具链、补丁与目录布局与 `../hgamey-deck/README.md` 一致（`agent/tools/makepad` 与 `makepad-d3` 需先应用 `patches/` 下的补丁）。

```bash
FONTTOOLS_PY=/path/to/python-with-fonttools bash build.sh        # 生成 dist/
FONTTOOLS_PY=... bash build.sh serve                              # 本地预览 http://127.0.0.1:8123（无密码）
cd dist && vercel link --yes --project selfsr-deck --scope <团队>   # build.sh 会删除 dist，需重新链接
vercel deploy --prod --yes                                        # 项目已配置环境变量 SITE_PASSWORD
```

`deploy/middleware.js` 与 hgamey-deck 相同，仅 cookie 名称与摘要盐值不同（`selfsr_auth`、`selfsr-deck:`）。
