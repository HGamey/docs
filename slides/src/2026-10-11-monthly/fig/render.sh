#!/bin/zsh
# 把 fig/*.html 渲染成 2 倍分辨率 PNG（无头 Chrome）。尺寸取页面 body 的 width/height。
# 用法: ./render.sh [名字...]   缺省全部
cd ${0:A:h}
CH="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
mkdir -p out
for f in ${@:-*.html}; do
  f=${f%.html}.html
  w=$(grep -oE 'body \{[^}]*width: *[0-9]+px' $f | grep -oE '[0-9]+px' | head -1 | tr -d px)
  h=$(grep -oE 'body \{[^}]*height: *[0-9]+px' $f | grep -oE 'height: *[0-9]+px' | grep -oE '[0-9]+' | head -1)
  "$CH" --headless=new --disable-gpu --hide-scrollbars --force-device-scale-factor=2 \
    --window-size=${w},${h} --virtual-time-budget=3000 --screenshot=out/${f%.html}.png "file://$PWD/$f" 2>/dev/null
  echo "out/${f%.html}.png ${w}x${h}"
done
