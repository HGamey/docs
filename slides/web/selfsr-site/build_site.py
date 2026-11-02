#!/usr/bin/env python3
"""build_site.py — 把超分辨率与插帧这次的全部材料组装成一个静态网站 dist/，部署到 Vercel 项目 selfsr-deck：

  /                 汇总页（主要结论 + 各项材料入口）
  /loop/            研究闭环实录（loop/：平台原始记录导出的时间线，由 loop/export_timeline.py 生成）
  /deck/            在线幻灯片（../selfsr-deck/dist，Makepad WASM）
  /files/*.pptx     PPTX 下载（slides/ 下的同名文件）
  /docs/...         部署与使用指导书、高低清数据集采集方案（由 docs 仓库的 Markdown 渲染为网页）

文档之间的链接改写为站内页面；指向仓库其他文件的链接改写为 GitHub 地址。整站沿用 selfsr-deck 的密码中间件
与 COOP/COEP 头（幻灯片的 WASM 多线程需要），因此页面不引用任何外部资源。

    python3 -I build_site.py        # 先在 ../selfsr-deck 运行 build.sh 生成其 dist/；需要 python-markdown
"""
import html
import os
import posixpath
import re
import shutil
import sys

import markdown

HERE = os.path.dirname(os.path.abspath(__file__))
DOCS = os.path.abspath(os.path.join(HERE, "../../.."))          # docs 仓库根
DECK = os.path.join(HERE, "../selfsr-deck")
OUT = os.path.join(HERE, "dist")
GH = "https://github.com/HGamey/docs/blob/main/"
PPTX = "2026-11-02-超分与插帧阶段结果.pptx"
PPTX_OUT = "selfsr-results-2026-11-02.pptx"

# 站内文档：仓库路径 → (站内路径, 页面标题)
GUIDE = [("guide/README.md", "总览"), ("guide/01-device-layer.md", "M1 设备层"), ("guide/02-node-service.md", "M2 节点服务与网页"),
         ("guide/03-execution-node.md", "M3 执行节点"), ("guide/04-agent.md", "M4 Agent 接入"),
         ("guide/05-public-access.md", "M5 公网入口"), ("guide/06-selfsr.md", "M6 自研超分辨率与插帧"),
         ("guide/07-demo-setup.md", "演示环境搭建")]
DATASET = ("plans/2026-10-22-agent-platform/DATASET-COLLECTION.md", "高低清数据集采集方案")
PAGES = {src: ("docs/" + src.replace("plans/2026-10-22-agent-platform/", "").replace(".md", ".html").replace("README.html", "index.html"), title)
         for src, title in GUIDE + [DATASET]}

CSS = """
:root { --bg:#F8F8F6; --paper:#FFFFFF; --ink:#202020; --ink2:#383A3C; --muted:#85827D; --line:#E2E0DA; --line2:#C4C4C2;
  --teal:#2E7D74; --teal-t:#E3F0EE; --navy:#33435D; --amber:#A8752A; --amber-t:#F7EEDD; --code:#F3F1EC;
  --font:"PingFang SC","Microsoft YaHei","Hiragino Sans GB","Noto Sans CJK SC",system-ui,sans-serif;
  --mono:ui-monospace,"SF Mono",Menlo,Consolas,monospace; color-scheme:light; }
@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) {
  --bg:#17181A; --paper:#1F2023; --ink:#ECEBE7; --ink2:#C9C7C2; --muted:#9A978F; --line:#303236; --line2:#45474C;
  --teal:#6FC2B6; --teal-t:#1D3532; --navy:#9FB2D3; --amber:#E0B26A; --amber-t:#3A2F1C; --code:#26282C; color-scheme:dark; } }
* { box-sizing:border-box; }
html { -webkit-text-size-adjust:100%; }
body { margin:0; background:var(--bg); color:var(--ink); font:16px/1.7 var(--font); }
a { color:var(--teal); text-decoration:none; } a:hover { text-decoration:underline; }
.wrap { max-width:1040px; margin:0 auto; padding:40px 24px 64px; }
header.top { display:flex; justify-content:space-between; align-items:baseline; gap:16px; flex-wrap:wrap;
  border-bottom:1px solid var(--line2); padding-bottom:14px; margin-bottom:28px; font-size:14px; color:var(--muted); }
header.top b { color:var(--ink); font-size:15px; }
h1 { font-size:30px; line-height:1.35; margin:0 0 10px; }
.lead { color:var(--ink2); margin:0 0 28px; }
h2 { font-size:21px; margin:40px 0 14px; }
h3 { font-size:17px; margin:28px 0 10px; }
.cards { display:grid; grid-template-columns:repeat(auto-fit,minmax(230px,1fr)); gap:16px; }
.card { background:var(--paper); border:1px solid var(--line); border-radius:10px; padding:20px 22px; display:block; color:inherit; }
a.card:hover { border-color:var(--teal); text-decoration:none; }
.card .k { font-size:13px; color:var(--muted); margin-bottom:6px; }
.card .t { font-size:18px; font-weight:700; color:var(--ink); margin-bottom:6px; }
.card .d { font-size:14px; color:var(--ink2); line-height:1.6; }
.card.main { border-color:var(--teal); background:var(--teal-t); }
.facts { background:var(--paper); border:1px solid var(--line); border-radius:10px; padding:8px 22px; }
.facts li { margin:10px 0; }
.num { font-weight:700; font-variant-numeric:tabular-nums; }
.tag { display:inline-block; font-size:12px; padding:1px 8px; border-radius:4px; background:var(--teal-t); color:var(--teal); margin-left:6px; vertical-align:2px; }
.tag.pend { background:var(--amber-t); color:var(--amber); }
.small { font-size:13px; color:var(--muted); }
ul.links { padding-left:20px; } ul.links li { margin:6px 0; }
/* 文档页 */
.doc { display:grid; grid-template-columns:220px 1fr; gap:36px; }
nav.side { font-size:14px; position:sticky; top:20px; align-self:start; }
nav.side a { display:block; padding:5px 10px; border-radius:6px; color:var(--ink2); }
nav.side a.on { background:var(--teal-t); color:var(--teal); font-weight:700; }
nav.side .grp { font-size:12px; color:var(--muted); margin:16px 10px 6px; }
article { background:var(--paper); border:1px solid var(--line); border-radius:10px; padding:28px 34px; min-width:0; }
article h1 { font-size:26px; }
article table { border-collapse:collapse; width:100%; font-size:14px; margin:14px 0; display:block; overflow-x:auto; }
article th, article td { border-bottom:1px solid var(--line); padding:8px 10px; text-align:left; vertical-align:top; }
article th { color:var(--muted); font-weight:700; border-bottom:1.5px solid var(--line2); white-space:nowrap; }
article code { font-family:var(--mono); font-size:.9em; background:var(--code); padding:1px 5px; border-radius:4px; }
article pre { background:var(--code); border-radius:8px; padding:14px 16px; overflow-x:auto; font-size:13.5px; line-height:1.55; }
article pre code { background:none; padding:0; }
article blockquote { margin:14px 0; padding:4px 16px; border-left:3px solid var(--line2); color:var(--ink2); }
@media (max-width:760px) { .wrap { padding:24px 16px 48px; } h1 { font-size:24px; }
  .doc { grid-template-columns:1fr; gap:16px; } nav.side { position:static; } article { padding:20px 18px; } }
"""


def page(title, body, rel):
    return f"""<!doctype html>
<html lang="zh-CN">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{html.escape(title)}</title>
<link rel="stylesheet" href="{rel}site.css">
</head>
<body><div class="wrap">
<header class="top"><b><a href="{rel}" style="color:inherit">移动游戏超分辨率与插帧：阶段结果</a></b><span>HGamey · 2026-11-02</span></header>
{body}
</div></body></html>
"""


def rewrite_links(md_text, src):
    """Markdown 链接：站内文档改为站内页面，其他仓库内相对路径改为 GitHub 地址。"""
    base = posixpath.dirname(src)

    def fix(m):
        label, target = m.group(1), m.group(2)
        if re.match(r"^[a-z]+://|^#|^mailto:", target):
            return m.group(0)
        path, _, frag = target.partition("#")
        repo_path = posixpath.normpath(posixpath.join(base, path))
        if repo_path in PAGES:
            here = posixpath.dirname(PAGES[src][0])
            url = posixpath.relpath(PAGES[repo_path][0], here) + (("#" + frag) if frag else "")
        else:
            url = GH + repo_path + (("#" + frag) if frag else "")
        return f"[{label}]({url})"
    return re.sub(r"(?<!!)\[([^\]]+)\]\(([^)\s]+)\)", fix, md_text)


def render_doc(src):
    out_path, title = PAGES[src]
    md_text = open(os.path.join(DOCS, src), encoding="utf-8").read()
    body_html = markdown.markdown(rewrite_links(md_text, src), extensions=["tables", "fenced_code", "sane_lists"])
    here = posixpath.dirname(out_path)
    rel = posixpath.relpath(".", here) + "/"
    side = ['<div class="grp">部署与使用指导书</div>']
    for s, t in GUIDE:
        side.append(f'<a href="{posixpath.relpath(PAGES[s][0], here)}"{" class=on" if s == src else ""}>{html.escape(t)}</a>')
    side.append('<div class="grp">数据集</div>')
    side.append(f'<a href="{posixpath.relpath(PAGES[DATASET[0]][0], here)}"{" class=on" if src == DATASET[0] else ""}>{DATASET[1]}</a>')
    body = f'<div class="doc"><nav class="side">{"".join(side)}</nav><article>{body_html}</article></div>'
    dst = os.path.join(OUT, out_path)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    open(dst, "w", encoding="utf-8").write(page(title, body, rel))


def index_page():
    body = f"""
<h1>Harness 驱动的移动游戏图形优化：自优化闭环与训练数据</h1>
<p class="lead">本页汇集本次汇报的全部材料。建议先阅读在线幻灯片：前三页依次说明背景、术语与核心观点，无需其他资料即可读懂。</p>

<div class="cards">
  <a class="card main" href="loop/"><div class="k">原始记录</div><div class="t">研究闭环实录</div>
    <div class="d">一条真实研究链的平台记录：agent 每一步做了什么、人工在哪里介入、结论如何被确认或推翻。</div></a>
  <a class="card" href="deck/"><div class="k">在线浏览</div><div class="t">幻灯片</div>
    <div class="d">12 页，含背景、术语、闭环、部署拓扑与数据方法；三维图表可拖动旋转。← → 翻页，O 键总览。</div></a>
  <a class="card" href="files/{PPTX_OUT}" download><div class="k">下载</div><div class="t">演示文稿（PPTX）</div>
    <div class="d">与在线幻灯片内容一致，文字均可编辑。</div></a>
  <a class="card" href="docs/guide/index.html"><div class="k">文档</div><div class="t">部署与使用指导书</div>
    <div class="d">按最小功能模块编写，含演示环境（两台 PC 与一台 root 手机）的搭建步骤。</div></a>
  <a class="card" href="docs/DATASET-COLLECTION.html"><div class="k">文档</div><div class="t">高低清数据集采集方案</div>
    <div class="d">原理、软硬件需求、操作流程、质量检查与数据划分。</div></a>
</div>

<p class="small" style="margin-top:14px">如浏览器无法流畅运行三维版，可使用<a href="slides/">图片版幻灯片</a>（内容相同，任何浏览器均可打开）。</p>

<h2>主要结论</h2>
<ul class="facts">
  <li><b>自优化研究闭环。</b>人提出目标与评判标准，AI agent 在真实手机上自主完成「提出假设—实验—统计判定—交付—独立复现」。截至目前共 <span class="num">15</span> 项研究任务（14 项已交付）、<span class="num">17</span> 个研究方向（14 个成立、2 个否决、1 个暂缓）、<span class="num">169</span> 次真机任务。<span class="tag">实测</span></li>
  <li><b>训练数据与数据生成方法。</b>为学习型超分辨率明确了五类高低清配对数据的采集方法及其真机验证状态，并将采集纳入同一平台，使数据可随研究持续产生、可追溯。<span class="tag">离线</span></li>
  <li><b>闭环产出。</b>超分辨率与插帧使上屏帧率由 <span class="num">29.8</span> fps 提升至 <span class="num">117.8</span> fps，每帧 GPU 忙碌时间降低 <span class="num">51%</span>，整机功耗增加约 <span class="num">166%</span>；相同低分辨率输入下，自研超分辨率的 PSNR（37.9 dB）高于 FSR1（36.6 dB）与 GSR（31.7 dB）。<span class="tag">实测</span></li>
</ul>
<p class="small">实测数据为原神默认设置下的真机配对测量；离线数据为对同一帧截图的计算。术语解释、详细口径与置信区间见幻灯片。</p>

<h2>代码与数据来源</h2>
<p class="small">以下链接供需要复现或修改的人员使用，阅读汇报内容无需打开。</p>
<ul class="links small">
  <li>FSR1 离线对比与层内移植：<a href="https://github.com/HGamey/experiments/pull/13">HGamey/experiments #13</a>、<a href="https://github.com/HGamey/experiments/pull/14">#14</a></li>
  <li>幻灯片与演示文稿源码：<a href="https://github.com/HGamey/docs/tree/main/slides/web/selfsr-deck">slides/web/selfsr-deck</a>、<a href="https://github.com/HGamey/docs/tree/main/slides/src/selfsr-pptx">slides/src/selfsr-pptx</a></li>
  <li>实验报告：<a href="https://github.com/HGamey/docs/tree/main/experiments/2026-10-24-selfsr">experiments/2026-10-24-selfsr</a></li>
</ul>
"""
    open(os.path.join(OUT, "index.html"), "w", encoding="utf-8").write(page("移动游戏超分辨率与插帧：阶段结果", body, ""))


VIEWER = """<!doctype html>
<html lang="zh-CN">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>幻灯片 · Harness 驱动的移动游戏图形优化</title>
<style>
html,body{margin:0;height:100%;background:#1E1E1E;color:#ECEBE7;font:14px/1.5 "PingFang SC","Microsoft YaHei",system-ui,sans-serif}
.stage{position:fixed;inset:0 0 52px 0;display:flex;align-items:center;justify-content:center}
.stage img{max-width:100%;max-height:100%;display:none;box-shadow:0 2px 16px rgba(0,0,0,.4)}
.stage img.on{display:block}
.bar{position:fixed;left:0;right:0;bottom:0;height:52px;display:flex;align-items:center;justify-content:center;gap:12px;background:#151515}
.bar button,.bar a{background:#2C2C2C;color:#ECEBE7;border:0;border-radius:6px;padding:7px 14px;font:inherit;cursor:pointer;text-decoration:none}
.bar button:hover,.bar a:hover{background:#3A3A3A}
#pos{min-width:64px;text-align:center;font-variant-numeric:tabular-nums}
@media (max-width:760px){body{overflow:auto}.stage{position:static;display:block;padding:12px 12px 64px}
 .stage img{display:block;margin:0 auto 12px;width:100%}.bar{position:fixed}#prev,#next,#pos{display:none}}
</style>
</head>
<body>
<div class="stage" id="stage">__IMGS__</div>
<div class="bar"><a href="../">返回首页</a><button id="prev" aria-label="上一页">‹ 上一页</button><span id="pos"></span><button id="next" aria-label="下一页">下一页 ›</button><a href="../files/__PPTX__" download>下载 PPTX</a></div>
<script>
(function(){var im=[].slice.call(document.querySelectorAll('.stage img')),c=0,pos=document.getElementById('pos');
function show(i){c=Math.max(0,Math.min(im.length-1,i));im.forEach(function(x,k){x.classList.toggle('on',k===c)});pos.textContent=(c+1)+' / '+im.length;try{history.replaceState(null,'','#'+(c+1))}catch(e){}}
try{var h=parseInt(location.hash.slice(1),10);if(h>0)c=h-1}catch(e){}
document.getElementById('prev').onclick=function(){show(c-1)};document.getElementById('next').onclick=function(){show(c+1)};
addEventListener('keydown',function(e){if(['ArrowRight','PageDown',' '].indexOf(e.key)>=0){e.preventDefault();show(c+1)}if(['ArrowLeft','PageUp'].indexOf(e.key)>=0){e.preventDefault();show(c-1)}if(e.key==='Home')show(0);if(e.key==='End')show(im.length-1)});
document.getElementById('stage').addEventListener('click',function(e){if(innerWidth<=760)return;show(c+(e.clientX>innerWidth/3?1:-1))});
var x0=null;addEventListener('touchstart',function(e){x0=e.touches[0].clientX},{passive:true});
addEventListener('touchend',function(e){if(x0===null||innerWidth<=760)return;var d=e.changedTouches[0].clientX-x0;x0=null;if(Math.abs(d)>50)show(c+(d<0?1:-1))});
show(c)})();
</script>
</body>
</html>
"""


def slides_viewer():
    """轻量图片版幻灯片：slides-img/*.jpg（由 PPTX 渲染）逐页显示，任何浏览器均可流畅翻页。"""
    src = os.path.join(HERE, "slides-img")
    imgs = sorted(f for f in os.listdir(src) if f.endswith(".jpg"))
    os.makedirs(os.path.join(OUT, "slides"))
    for f in imgs:
        shutil.copy(os.path.join(src, f), os.path.join(OUT, "slides", f))
    tags = "".join(f'<img src="{f}" alt="第 {i + 1} 页" decoding="async">' for i, f in enumerate(imgs))
    open(os.path.join(OUT, "slides", "index.html"), "w", encoding="utf-8").write(VIEWER.replace("__IMGS__", tags).replace("__PPTX__", PPTX_OUT))


def main():
    deck_dist = os.path.join(DECK, "dist")
    if not os.path.exists(os.path.join(deck_dist, "index.html")):
        sys.exit("缺少 ../selfsr-deck/dist：先在 selfsr-deck 运行 build.sh")
    shutil.rmtree(OUT, ignore_errors=True)
    shutil.copytree(deck_dist, os.path.join(OUT, "deck"),
                    ignore=shutil.ignore_patterns(".vercel", "middleware.js", "vercel.json", "package.json"))
    os.makedirs(os.path.join(OUT, "files"))
    shutil.copy(os.path.join(DOCS, "slides", PPTX), os.path.join(OUT, "files", PPTX_OUT))
    open(os.path.join(OUT, "site.css"), "w").write(CSS)
    for src in PAGES:
        render_doc(src)
    slides_viewer()
    shutil.copytree(os.path.join(HERE, "loop"), os.path.join(OUT, "loop"), ignore=shutil.ignore_patterns("*.py", "__pycache__"))
    index_page()
    for f in ("middleware.js", "vercel.json"):
        shutil.copy(os.path.join(DECK, "deploy", f), OUT)
    open(os.path.join(OUT, "package.json"), "w").write('{\n  "name": "selfsr-deck",\n  "private": true\n}\n')
    n = sum(len(fs) for _, _, fs in os.walk(OUT))
    print(f"dist/ 就绪：{n} 个文件")


if __name__ == "__main__":
    main()
