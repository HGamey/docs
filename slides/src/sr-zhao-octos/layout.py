#!/usr/bin/env python3
"""layout.py — 由 content.json（与网页版 sr-zhao-deck 的 content/deck.json 同一份内容）生成：
  1) art/<页>.json：无字底图的图形（卡片、线、点、方块背景、三维柱），交给 art.html（three.js）渲染；
  2) deck.json：octos pptx 技能 native_deck.py 的页面数据（原生可编辑文字，坐标与底图同一版面）。

版面 1920×1080 像素 = 13.333×7.5 英寸，1 pt = 2 px。三维柱的标签位置要等底图渲染后由 art.html
回传投影坐标，所以分两步：
    python3 -I layout.py art      → art/*.json
    （render_art.py 渲染 art/*.png 与 art/anchors.json）
    python3 -I layout.py spec     → deck.json
"""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
W, H = 1920, 1080
FONT = os.environ.get("DECK_FONT", "Microsoft YaHei")   # 预览渲染器没有雅黑时用 DECK_FONT 换成本机字体
# hgamey-deck 的配色（src/gen.rs）
INK, INK2, MUTED, LINE, LINE2 = "202020", "383A3C", "85827D", "C4C4C2", "E2E0DA"
PAPER, PANEL, BG = "FFFFFF", "FAF8F4", "F8F8F6"
NAVY, TEAL, TEAL_T, ROSE, AMBER, AMBER_T, BEIGE, BEIGE_T = "33435D", "2E7D74", "E3F0EE", "C76565", "A8752A", "F7EEDD", "8A8174", "F0EEEA"
ROSE_T, NAVY_T, GREY_T = "F5E5E3", "E4E8EF", "EFEEEB"
TINT = {"teal": (TEAL_T, TEAL), "amber": (AMBER_T, AMBER), "rose": (ROSE_T, ROSE), "navy": (NAVY_T, NAVY),
        "grey": (GREY_T, MUTED), "beige": (BEIGE_T, BEIGE), "": (PANEL, INK2)}
TAG = {"实测": (TEAL_T, TEAL), "离线": (NAVY_T, NAVY), "估算": (AMBER_T, AMBER), "外推": (AMBER_T, AMBER)}
STATE = {"done": (PAPER, NAVY, "已完成"), "wip": (TEAL_T, TEAL, "进行中"), "pending": (AMBER_T, AMBER, "待测"), "todo": (BEIGE_T, BEIGE, "未开始")}

STYLES = {
    "chapter": {"font": FONT, "size_pt": 9, "color": TEAL},
    "title": {"font": FONT, "size_pt": 22, "bold": True, "color": INK},
    "subtitle": {"font": FONT, "size_pt": 11, "color": INK2, "line_spacing": 1.15},
    "foot": {"font": FONT, "size_pt": 8, "color": MUTED, "line_spacing": 1.1},
    "pageno": {"font": FONT, "size_pt": 8, "color": MUTED, "align": "right"},
    "h": {"font": FONT, "size_pt": 13, "bold": True, "color": INK},
    "body": {"font": FONT, "size_pt": 10.5, "color": INK2, "line_spacing": 1.2},
    "small": {"font": FONT, "size_pt": 9, "color": MUTED, "line_spacing": 1.15},
    "kv": {"font": FONT, "size_pt": 20, "bold": True, "color": INK},
    "kl": {"font": FONT, "size_pt": 9, "color": INK2, "line_spacing": 1.15},
    "tag": {"font": FONT, "size_pt": 7.5, "bold": True, "color": TEAL, "align": "center", "anchor": "middle"},
    "th": {"font": FONT, "size_pt": 8.5, "bold": True, "color": MUTED},
    "td": {"font": FONT, "size_pt": 9.5, "color": INK2, "line_spacing": 1.1},
    "cell": {"font": FONT, "size_pt": 9.5, "color": INK, "align": "center", "anchor": "middle"},
    "flowt": {"font": FONT, "size_pt": 10.5, "bold": True, "color": INK},
    "flowx": {"font": FONT, "size_pt": 9, "color": INK2, "line_spacing": 1.15},
    "state": {"font": FONT, "size_pt": 7.5, "color": MUTED, "align": "right"},
    "barv": {"font": FONT, "size_pt": 9, "bold": True, "color": INK, "align": "center", "anchor": "bottom"},
    "barc": {"font": FONT, "size_pt": 8.5, "color": INK2, "align": "center"},
    "fval": {"font": FONT, "size_pt": 8.5, "color": INK2, "align": "right", "anchor": "middle"},
    "axis": {"font": FONT, "size_pt": 7.5, "color": MUTED, "align": "center"},
    "num": {"font": FONT, "size_pt": 10, "bold": True, "color": PAPER, "align": "center", "anchor": "middle"},
}


def text_w(s, px):
    """估算一行文字宽度（像素）：中日韩字符按 1 个字号宽，其余按 0.55。"""
    return sum(px if ord(c) > 0x2E80 else px * 0.62 for c in s) * 1.06


def n_lines(s, px, width):
    return sum(max(1, math.ceil(text_w(part, px) / max(width, 1))) for part in s.split("\n"))


def line_h(style):
    st = STYLES[style]
    return st["size_pt"] * 2 * 1.32 * st.get("line_spacing", 1.0)


class Page:
    def __init__(self, sid):
        self.id, self.shapes, self.texts, self.copy, self.bars = sid, [], [], {}, []
        self.n = 0

    def text(self, s, style, box, **kw):
        self.n += 1
        tid = f"t{self.n:03d}"
        self.texts.append({"id": tid, "text": s, "style": style, "box": [round(v, 1) for v in box], **kw})
        self.copy[tid] = s
        return tid

    def rect(self, x, y, w, h, fill, stroke=None, r=8, sw=1.2, shadow=False):
        self.shapes.append({"k": "rect", "x": x, "y": y, "w": w, "h": h, "fill": fill, "stroke": stroke, "sw": sw, "r": r, "shadow": shadow})

    def line(self, x1, y1, x2, y2, color=LINE2, w=1.2, arrow=False):
        self.shapes.append({"k": "line", "x1": x1, "y1": y1, "x2": x2, "y2": y2, "color": color, "w": w, "arrow": arrow})

    def dot(self, x, y, r, color):
        self.shapes.append({"k": "dot", "x": x, "y": y, "r": r, "color": color})

    def para(self, s, style, x, y, w):
        h = n_lines(s, STYLES[style]["size_pt"] * 2, w) * line_h(style) + 4
        self.text(s, style, [x, y, w, h])
        return h

    def tag(self, label, x, y):
        fill, ink = TAG.get(label, (BEIGE_T, BEIGE))
        w = text_w(label, 15) + 22
        self.rect(x - w, y, w, 26, fill, ink, r=5, sw=1)
        self.text(label, "tag", [x - w, y, w, 26], format={"color": ink})
        return w


# ---------- 块 ----------

def blk_h(p, b, x, y, w):
    p.text(b["text"], "h", [x, y, w, 36]); return 44


def blk_p(p, b, x, y, w):
    return p.para(b["text"], "body", x, y, w) + 10


def blk_note(p, b, x, y, w):
    return p.para(b["text"], "small", x, y, w) + 8


def blk_list(p, b, x, y, w, ordered=False):
    y0 = y
    for i, it in enumerate(b["items"]):
        if ordered:
            p.dot(x + 14, y + 16, 14, NAVY)
            p.text(str(i + 1), "num", [x, y + 2, 28, 28])
        else:
            p.dot(x + 6, y + 15, 4, TEAL)
        h = p.para(it, "body", x + (40 if ordered else 22), y, w - (40 if ordered else 22))
        y += max(h, 34) + 10
    return y - y0 + 4


def blk_kpis(p, b, x, y, w):
    items, cols, gap = b["items"], 2, 16
    cw = (w - gap) / cols
    rows = math.ceil(len(items) / cols)
    hs = []
    for it in items:
        hs.append(56 + 12 + n_lines(it["l"], 18, cw - 36) * line_h("kl") + 22)
    ch = max(hs)
    for i, it in enumerate(items):
        cx, cy = x + (i % cols) * (cw + gap), y + (i // cols) * (ch + gap)
        p.rect(cx, cy, cw, ch, PANEL, LINE2, r=8)
        tw = p.tag(it["tag"], cx + cw - 14, cy + 16) if it.get("tag") else 0
        avail = cw - 36 - tw - 8
        kpt = 20 if text_w(it["v"], 40) <= avail else max(14, 20 * avail / text_w(it["v"], 40))
        p.text(it["v"], "kv", [cx + 18, cy + 12, cw - 36 - tw, 54], format={"size_pt": round(kpt, 1)})
        p.para(it["l"], "kl", cx + 18, cy + 72, cw - 36)
    return rows * ch + (rows - 1) * gap + 16


def blk_table(p, b, x, y, w):
    ws = b.get("w") or [1] * len(b["head"])
    tot = sum(ws)
    cw = [w * k / tot for k in ws]
    xs = [x + sum(cw[:i]) for i in range(len(cw))]
    y0 = y
    for i, hd in enumerate(b["head"]):
        p.text(hd, "th", [xs[i] + 6, y, cw[i] - 12, 30])
    y += 34
    p.line(x, y, x + w, y, LINE, 1.4)
    for row in b["rows"]:
        hh = max(n_lines(str(c), 19, cw[i] - 12) for i, c in enumerate(row)) * line_h("td") + 14
        for i, c in enumerate(row):
            p.text(str(c), "td", [xs[i] + 6, y + 7, cw[i] - 12, hh - 10], format={"bold": i == 0, "color": INK if i == 0 else INK2})
        y += hh
        p.line(x, y, x + w, y, LINE2, 1)
    return y - y0 + 12


def blk_flow(p, b, x, y, w):
    items = b["items"]
    y0 = y
    if len(items) <= 4:   # 横排
        gap = 34
        bw = (w - gap * (len(items) - 1)) / len(items)
        th = max(n_lines(it["title"], 21, bw - 28) for it in items) * line_h("flowt")
        bh = max(th + 20 + n_lines(it["text"], 18, bw - 28) * line_h("flowx") for it in items) + 44
        for i, it in enumerate(items):
            fill, ink, lab = STATE.get(it.get("state", "todo"), STATE["todo"])
            bx = x + i * (bw + gap)
            p.rect(bx, y, bw, bh, fill, ink, r=8, sw=1.6)
            p.text(lab, "state", [bx + bw - 90, y + bh - 30, 78, 22], format={"color": ink})
            p.text(it["title"], "flowt", [bx + 14, y + 10, bw - 28, th + 4])
            p.para(it["text"], "flowx", bx + 14, y + 16 + th, bw - 28)
            if i < len(items) - 1:
                p.line(bx + bw + 6, y + bh / 2, bx + bw + gap - 6, y + bh / 2, MUTED, 1.6, arrow=True)
        return bh + 18
    for i, it in enumerate(items):   # 竖排
        fill, ink, lab = STATE.get(it.get("state", "todo"), STATE["todo"])
        bh = 44 + n_lines(it["text"], 18, w - 28) * line_h("flowx") + 14
        p.rect(x, y, w, bh, fill, ink, r=8, sw=1.6)
        p.text(lab, "state", [x + w - 100, y + 10, 88, 22], format={"color": ink})
        p.text(it["title"], "flowt", [x + 14, y + 8, w - 120, 30])
        p.para(it["text"], "flowx", x + 14, y + 42, w - 28)
        y += bh
        if i < len(items) - 1:
            p.line(x + 24, y + 3, x + 24, y + 19, MUTED, 1.6, arrow=True)
            y += 22
    return y - y0 + 14


def blk_heat(p, b, x, y, w):
    y0 = y
    p.text(b["title"], "flowt", [x, y, w - 90, 30])
    if b.get("tag"):
        p.tag(b["tag"], x + w, y)
    y += 40
    lw = 90
    nc = len(b["col_labels"])
    cw = (w - lw - 8 * (nc - 1)) / nc
    for j, cl in enumerate(b["col_labels"]):
        p.text(cl, "th", [x + lw + j * (cw + 8), y, cw, 26], format={"align": "center"})
    y += 30
    for i, row in enumerate(b["rows"]):
        p.text(b["row_labels"][i], "td", [x, y, lw - 8, 40], format={"anchor": "middle"})
        for j, c in enumerate(row):
            fill, ink = TINT.get(c.get("c", ""), TINT[""])
            cx = x + lw + j * (cw + 8)
            p.rect(cx, y, cw, 40, fill, ink, r=6, sw=1)
            p.text(c["t"], "cell", [cx + 6, y, cw - 12, 40], format={"color": ink if c.get("c") not in ("", "grey") else INK2})
        y += 48
    if b.get("note"):
        y += p.para(b["note"], "small", x, y + 2, w) + 4
    return y - y0 + 10


def blk_forest(p, b, x, y, w):
    y0 = y
    p.text(b["title"], "flowt", [x, y, w - 90, 30])
    if b.get("tag"):
        p.tag(b["tag"], x + w, y)
    y += 44
    lab_w, val_w = w * 0.34, w * 0.26
    ax0, ax1 = x + lab_w + 8, x + w - val_w - 8
    lo, hi = b["min"], b["max"]
    fx = lambda v: ax0 + (v - lo) / (hi - lo) * (ax1 - ax0)
    # 轴刻度：min、0、max
    last = None
    for v in (lo, 0, hi):   # 刻度太近时只留前一个
        if last is not None and fx(v) - fx(last) < 70:
            continue
        p.text(f"{v:g}{b.get('unit', '')}", "axis", [fx(v) - 40, y, 80, 22])
        last = v
    y += 26
    top = y
    rows = b["rows"]
    for r in rows:
        lh = n_lines(r["label"], 19, lab_w) * line_h("td")
        vh = n_lines(r["verdict"], 18, lab_w) * line_h("small")
        rh = max(74, lh + vh + 18)
        cy = y + 22
        p.text(r["label"], "td", [x, y, lab_w, lh + 4], format={"bold": True, "color": INK})
        p.text(r["verdict"], "small", [x, y + lh + 4, lab_w, vh + 4])
        p.line(fx(r["lo"]), cy, fx(r["hi"]), cy, TEAL, 3)
        p.line(fx(r["lo"]), cy - 7, fx(r["lo"]), cy + 7, TEAL, 2)
        p.line(fx(r["hi"]), cy - 7, fx(r["hi"]), cy + 7, TEAL, 2)
        p.dot(fx(r["est"]), cy, 7, NAVY)
        p.text(f"{r['est']:g} [{r['lo']:g}, {r['hi']:g}]", "fval", [x + w - val_w, cy - 16, val_w, 32])
        y += rh
    p.line(fx(0), top - 4, fx(0), y - 6, MUTED, 1.2)
    p.line(ax0, top - 4, ax1, top - 4, LINE2, 1)
    return y - y0 + 10


def blk_pending(p, b, x, y, w):
    h = 40 + n_lines(b["what"], 18, w - 130) * line_h("small") + 36 + 20
    p.rect(x, y, w, h, AMBER_T, AMBER, r=8, sw=1.4)
    p.tag("待测", x + 80, y + 14)
    p.text(b["title"], "flowt", [x + 94, y + 12, w - 110, 30])
    yy = y + 52
    p.text("要做什么", "small", [x + 16, yy, 100, 26])
    yy += p.para(b["what"], "small", x + 116, yy, w - 132)
    p.text("产出任务", "small", [x + 16, yy + 4, 100, 26])
    p.text(b["job"], "small", [x + 116, yy + 4, w - 132, 26], format={"color": INK2})
    return h + 14


def blk_chart3d(p, b, x, y, w):
    y0 = y
    p.text(b["title"], "flowt", [x, y, w - 90, 30])
    if b.get("tag"):
        p.tag(b["tag"], x + w, y)
    y += 40
    sp = b["spec"]
    ch = 330
    tints = [TINT.get(c, TINT[""])[1] for c in sp.get("cat_colors", [])]
    bid = f"{p.id}-{b['id']}"
    p.bars.append({"id": bid, "x": x, "y": y, "w": w, "h": ch, "values": sp["values"][0], "colors": tints,
                   "ymin": sp["ymin"], "ymax": sp["ymax"], "yaw": 0.32, "pitch": 0.42,
                   "texts": sp.get("texts", [[str(v) for v in sp["values"][0]]])[0], "cats": sp["cats"]})
    y += ch + 52
    if b.get("note"):
        y += p.para(b["note"], "small", x, y, w)
    return y - y0 + 10


BLOCKS = {"h": blk_h, "p": blk_p, "note": blk_note, "ul": blk_list, "ol": lambda p, b, x, y, w: blk_list(p, b, x, y, w, True),
          "kpis": blk_kpis, "table": blk_table, "flow": blk_flow, "heat": blk_heat, "forest": blk_forest,
          "pending": blk_pending, "chart3d": blk_chart3d}


def column(p, blocks, x, y, w, h):
    p.rect(x, y, w, h, PAPER, LINE2, r=10, sw=1.2, shadow=True)
    cy = y + 24
    for b in blocks:
        cy += BLOCKS[b["t"]](p, b, x + 24, cy, w - 48)
    if cy > y + h:
        print(f"  [{p.id}] 内容超出卡片 {cy - (y + h):.0f}px", file=sys.stderr)


def build_page(i, s, total):
    p = Page(s["id"])
    p.text(s["chapter"], "chapter", [60, 34, 600, 26])
    p.text(s["title"], "title", [60, 60, 1800, 58])
    sub_h = n_lines(s["subtitle"], 22, 1800) * line_h("subtitle")
    p.text(s["subtitle"], "subtitle", [60, 122, 1800, sub_h + 6])
    top = 122 + sub_h + 22
    p.line(60, top - 8, 1860, top - 8, LINE, 1.2)
    bottom = 1000
    if s["layout"] == "split" and s.get("right"):
        column(p, s["left"], 60, top, 890, bottom - top)
        column(p, s["right"], 970, top, 890, bottom - top)
    else:
        column(p, s["left"], 60, top, 1800, bottom - top)
    p.text(s["foot"], "foot", [60, 1012, 1640, 56])
    p.text(f"{i + 1} / {total}", "pageno", [1740, 1012, 120, 26])
    return p


def main():
    stage = sys.argv[1]
    content = json.load(open(os.path.join(HERE, "content.json"), encoding="utf-8"))
    slides = content["slides"]
    pages = [build_page(i, s, len(slides)) for i, s in enumerate(slides)]
    os.makedirs(os.path.join(HERE, "art"), exist_ok=True)
    if stage == "art":
        for i, p in enumerate(pages):
            spec = {"bg": BG, "shapes": p.shapes, "bars3d": p.bars,
                    "voxels": {"rect": [1180, 640, 740, 440], "seed": 11 + i}}
            json.dump(spec, open(os.path.join(HERE, "art", f"{i + 1:02d}-{p.id}.json"), "w"), ensure_ascii=False)
        print(f"art: {len(pages)} 页")
        return
    anchors = json.load(open(os.path.join(HERE, "art", "anchors.json")))
    out = []
    for i, p in enumerate(pages):
        for b in p.bars:
            a = anchors[f"{i + 1:02d}"][b["id"]]
            for k, ((tx, ty), (bx, by)) in enumerate(zip(a["tops"], a["bases"])):
                p.text(b["texts"][k], "barv", [tx - 60, ty - 34, 120, 28])
                p.text(b["cats"][k], "barc", [bx - 70, by + 8, 140, 26])
        out.append({"id": p.id, "background": f"art/{i + 1:02d}-{p.id}.png", "copy": p.copy, "texts": p.texts})
    deck = {"schema_version": 1, "canvas": {"width_px": W, "height_px": H, "width_in": 13.333, "height_in": 7.5},
            "styles": STYLES, "pages": out}
    json.dump(deck, open(os.path.join(HERE, "deck.json"), "w"), ensure_ascii=False, indent=1)
    print(f"deck.json: {len(out)} 页, {sum(len(p['texts']) for p in out)} 个文字框")


if __name__ == "__main__":
    main()
