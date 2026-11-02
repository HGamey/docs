#!/usr/bin/env python3
"""render_art.py — 用浏览器里的 three.js（art.html）把 art/*.json 渲染成 3840×2160 无字底图 art/*.png，
并把三维柱的投影坐标写到 art/anchors.json（layout.py spec 用它摆柱顶 / 柱底的原生文字）。

    python3 -m http.server 8124 --bind 127.0.0.1   # 在本目录起静态服务（art.html 用 importmap 从 CDN 取 three.js）
    python3 render_art.py                          # 需要 playwright（chromium，开 WebGL）

也可以在已连上的浏览器里调用 render(page)。
"""
import base64
import glob
import json
import os
import time

HERE = os.path.dirname(os.path.abspath(__file__))
URL = os.environ.get("ART_URL", "http://127.0.0.1:8124/art.html")


def render(page):
    page.set_viewport_size({"width": 1920, "height": 1080})
    page.goto(URL, wait_until="load")
    for _ in range(100):
        if page.evaluate("window.artReady === true"):
            break
        time.sleep(0.1)
    anchors = {}
    for f in sorted(glob.glob(os.path.join(HERE, "art", "[0-9][0-9]-*.json"))):
        spec = json.load(open(f, encoding="utf-8"))
        r = page.evaluate("s => window.renderSlide(s)", spec)
        png = base64.b64decode(r["png"].split(",", 1)[1])
        open(f[:-5] + ".png", "wb").write(png)
        anchors[os.path.basename(f)[:2]] = r["anchors"]
    json.dump(anchors, open(os.path.join(HERE, "art", "anchors.json"), "w"))
    return len(anchors)


if __name__ == "__main__":
    from playwright.sync_api import sync_playwright
    with sync_playwright() as p:
        b = p.chromium.launch(args=["--use-gl=angle", "--enable-webgl"])
        print("rendered", render(b.new_page()))
        b.close()
