#!/usr/bin/env python3
"""把 dist/ 里的中文字体（霞鹜文楷，各 18 MB）裁成只含本 deck 用到的字。
用到的字 = content/deck.json + data/*.json + src/*.rs + 私有目录 *.json 里出现的全部字符 + ASCII + 常用标点。
用法：subset_fonts.py <dist 目录>   需要 fonttools。"""
import glob, os, sys
dist = sys.argv[1]
here = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
chars = set(chr(c) for c in range(0x20, 0x7f))
chars |= set("，。、：；？！「」『』（）《》〈〉【】—…·×±≈≤≥→←↑↓％‰°−–’‘“”·")
priv = os.environ.get("HGAMEY_DECK_PRIVATE") or os.path.join(here, "../../../../internal/decks/hgamey-deck/private")
for pat in [os.path.join(here, "content/*.json"), os.path.join(here, "data/*.json"), os.path.join(here, "src/*.rs"), os.path.join(priv, "*.json")]:
    for p in glob.glob(pat):
        chars |= set(open(p, encoding="utf-8").read())
chars = {c for c in chars if c.isprintable()}
try:
    from fontTools.ttLib import TTFont, newTable
    from fontTools import subset
except ImportError:
    sys.exit("需要 fonttools：pip install fonttools，然后用装了它的 python 运行（build.sh 里设 FONTTOOLS_PY）")
import struct

def only_format12_cmap(font):
    """霞鹜文楷的 format 4 cmap 子表 fontTools 读不了；只保留 format 12（覆盖全部字符）。"""
    raw = font.reader["cmap"]
    n = struct.unpack(">H", raw[2:4])[0]
    for i in range(n):
        _, _, off = struct.unpack(">HHI", raw[4 + 8 * i:12 + 8 * i])
        if struct.unpack(">H", raw[off:off + 2])[0] == 12:
            ln = struct.unpack(">I", raw[off + 4:off + 8])[0]
            sub = raw[off:off + ln]
            new = struct.pack(">HH", 0, 2) + struct.pack(">HHI", 0, 4, 20) + struct.pack(">HHI", 3, 10, 20) + sub
            t = newTable("cmap"); t.decompile(new, font); font["cmap"] = t
            return
    raise SystemExit("没有 format 12 cmap")

text = "".join(sorted(chars))
for name in ["LXGWWenKaiRegular.ttf", "LXGWWenKaiBold.ttf"]:
    f = os.path.join(dist, "makepad_widgets/resources", name)
    font = TTFont(f, lazy=True)
    only_format12_cmap(font)
    opts = subset.Options(); opts.layout_features = ["*"]; opts.hinting = False; opts.notdef_outline = True
    sub = subset.Subsetter(opts); sub.populate(text=text); sub.subset(font)
    font.save(f + ".sub"); os.replace(f + ".sub", f)
    print(name, os.path.getsize(f) // 1024, "KB")
print("字符数", len(chars))
