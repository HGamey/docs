#!/bin/bash
# 构建 WASM 并组装可部署目录 dist/（静态文件 + Vercel 密码中间件 + 头部配置）。
# 依赖：cargo-makepad、fonttools（FONTTOOLS_PY=装了 fonttools 的 python，默认 python3）
# 用法：bash build.sh            只构建
#       bash build.sh serve      构建后在 http://127.0.0.1:8123 本地预览（带 COOP/COEP 头，无密码）
# 部署：cd dist && vercel deploy --prod   （项目需设置环境变量 SITE_PASSWORD）
set -euo pipefail
cd "$(dirname "$0")"
cargo makepad wasm build -p hgamey-deck --release
OUT=target/makepad-wasm-app/release/hgamey-deck
rm -rf dist && mkdir -p dist
cp -R "$OUT"/. dist/
sed -i '' -e 's#<title>hgamey-deck</title>#<title>HGamey 进展</title>#' -e "s#<html>#<html lang='zh-CN'>#" dist/index.html
# 系统是暗色时补上 ?theme=dark（WASM 从 URL 读初始主题；页面里按 T 或底栏按钮切换）
python3 -I - dist/index.html <<'PY'
import sys
p = sys.argv[1]; s = open(p, encoding="utf-8").read()
js = "<script>try{var u=new URL(location.href);if(!u.searchParams.has('theme')&&matchMedia('(prefers-color-scheme: dark)').matches){u.searchParams.set('theme','dark');history.replaceState(null,'',u)}}catch(e){}</script>\n        "
assert "<script type='module'>" in s, "index.html 结构变了"
open(p, "w", encoding="utf-8").write(s.replace("<script type='module'>", js + "<script type='module'>", 1))
PY
# makepad 的 JS 把共享内存初始值写死为 64 页（4 MB）；本 deck 嵌入了数据和裁剪图，静态段超过 4 MB，调大到 256 页。
sed -i '' 's/initial:64,maximum:16384,shared:true/initial:256,maximum:16384,shared:true/' dist/makepad_wasm_bridge/wasm_bridge.js
grep -q 'initial:256,maximum:16384' dist/makepad_wasm_bridge/wasm_bridge.js || { echo '共享内存补丁没打上（makepad JS 变了？）'; exit 1; }
# 中文字体只保留用到的字（18 MB → 约 1 MB）
"${FONTTOOLS_PY:-python3}" -I scripts/subset_fonts.py dist
cp deploy/middleware.js deploy/vercel.json dist/
printf '{\n  "name": "hgamey-deck",\n  "private": true\n}\n' > dist/package.json
echo "dist/ 就绪：$(du -sh dist | cut -f1)"
if [ "${1:-}" = "serve" ]; then
  python3 -I scripts/serve.py dist 8123
fi
