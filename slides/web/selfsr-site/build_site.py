#!/usr/bin/env python3
"""build_site.py — 把超分辨率与插帧这次的全部材料组装成「单一页面」网站 dist/，部署到 Vercel 项目 selfsr-deck：

  /               唯一页面：摘要 → 研究闭环实录（loop/index.html 的原始记录时间线）→ 部署拓扑 → 训练数据 →
                  闭环产出 → 演示材料（三维幻灯片按需加载、PPTX 下载）→ 附录（部署与使用指导书、数据集采集方案，可折叠）
  /deck/          三维幻灯片本体（../selfsr-deck/dist，Makepad WASM），由页面内 iframe 按需加载
  /files/*.pptx   PPTX 下载
  /version.json   构建号；页面每分钟检查一次，与自身构建号不同即自动刷新（强制所有已打开的页面更新）
  旧地址（/loop/、/slides/、/docs/...）保留为跳转页，指向页面内对应章节。

整站沿用 selfsr-deck 的密码中间件与 COOP/COEP 头，因此页面不引用任何外部资源。

    python3 -I build_site.py        # 先在 ../selfsr-deck 运行 build.sh 生成其 dist/；需要 python-markdown
"""
import html
import json
import os
import posixpath
import re
import shutil
import sys
import time

import markdown

HERE = os.path.dirname(os.path.abspath(__file__))
DOCS = os.path.abspath(os.path.join(HERE, "../../.."))          # docs 仓库根
DECK = os.path.join(HERE, "../selfsr-deck")
OUT = os.path.join(HERE, "dist")
GH = "https://github.com/HGamey/docs/blob/main/"
PPTX = "2026-11-02-超分与插帧阶段结果.pptx"
PPTX_OUT = "selfsr-results-2026-11-02.pptx"
TITLE = "Harness 驱动的移动游戏图形优化：自优化闭环与训练数据"
PILOT = os.path.abspath(os.path.join(DOCS, "../dataset-pilot/v0/SUMMARY.md"))   # 试点数据小结（不在仓库内）

# 附录文档：仓库路径 → (页内锚点, 标题)
APPENDIX = [("guide/README.md", "guide", "部署与使用指导书：总览"), ("guide/01-device-layer.md", "guide-m1", "M1 设备层"),
            ("guide/02-node-service.md", "guide-m2", "M2 节点服务与网页"), ("guide/03-execution-node.md", "guide-m3", "M3 执行节点"),
            ("guide/04-agent.md", "guide-m4", "M4 Agent 接入"), ("guide/05-public-access.md", "guide-m5", "M5 公网入口"),
            ("guide/06-selfsr.md", "guide-m6", "M6 自研超分辨率与插帧"), ("guide/07-demo-setup.md", "guide-demo", "演示环境搭建"),
            ("plans/2026-10-22-agent-platform/DATASET-COLLECTION.md", "dataset-plan", "高低清数据集采集方案")]
ANCHOR = {src: a for src, a, _ in APPENDIX}
# 旧地址 → 页内锚点（跳转页）
REDIRECTS = {"loop/index.html": "loop", "slides/index.html": "materials", "docs/guide/index.html": "guide",
             "docs/DATASET-COLLECTION.html": "dataset-plan",
             **{f"docs/guide/{os.path.basename(s)[:-3]}.html": a for s, a, _ in APPENDIX if s.startswith("guide/0")}}

CSS_EXTRA = """
nav.toc { position:sticky; top:0; z-index:5; background:var(--bg); border-bottom:1px solid var(--line); margin:0 -24px 8px; padding:10px 24px;
  display:flex; gap:6px 16px; flex-wrap:wrap; font-size:14px; }
nav.toc a { color:var(--ink2); } nav.toc a:hover { color:var(--teal); }
.facts { background:var(--paper); border:1px solid var(--line); border-radius:10px; padding:6px 22px; margin:18px 0 0; }
.facts li { margin:10px 0; }
.sec { scroll-margin-top:56px; }
h2.part { font-size:23px; margin:56px 0 6px; padding-top:18px; border-top:1px solid var(--line2); }
.topo { display:grid; gap:6px; margin-top:14px; }
.topo .link { text-align:center; color:var(--muted); font-size:13px; }
.topo .tier { display:grid; gap:10px; grid-template-columns:repeat(auto-fit,minmax(200px,1fr)); }
.topo .nd { border:1.5px solid var(--navy); background:var(--paper); border-radius:8px; padding:9px 12px; }
.topo .nd b { display:block; font-size:14.5px; color:var(--navy); }
.topo .nd span { font-size:13px; color:var(--ink2); }
.topo .nd.teal { border-color:var(--teal); background:var(--teal-t); } .topo .nd.teal b { color:var(--teal); }
.topo .nd.amber { border-color:var(--amber); background:var(--amber-t); } .topo .nd.amber b { color:var(--amber); }
.topo .nd.beige { border-color:var(--line2); background:#F0EEEA; } .topo .nd.beige b { color:#6F675C; }
.steps { counter-reset:s; display:grid; gap:8px; margin-top:12px; }
.steps div { background:var(--paper); border:1px solid var(--line); border-radius:8px; padding:9px 14px 9px 44px; position:relative; font-size:14px; }
.steps div::before { counter-increment:s; content:counter(s); position:absolute; left:12px; top:9px; width:22px; height:22px; border-radius:50%;
  background:var(--navy); color:#fff; font-size:12px; text-align:center; line-height:22px; }
.steps b { margin-right:6px; }
.deckbox { position:relative; width:100%; aspect-ratio:16/9; background:#EDEBE6; border:1px solid var(--line); border-radius:10px; overflow:hidden; margin-top:12px; }
.deckbox iframe { position:absolute; inset:0; width:100%; height:100%; border:0; }
.deckbox .start { position:absolute; inset:0; display:flex; flex-direction:column; align-items:center; justify-content:center; gap:10px; }
.btn { display:inline-block; background:var(--teal); color:#fff; border:0; border-radius:8px; padding:10px 18px; font:inherit; font-size:15px; cursor:pointer; }
.btn.ghost { background:var(--paper); color:var(--teal); border:1px solid var(--teal); }
.btn:hover { text-decoration:none; filter:brightness(1.05); }
details.doc { background:var(--paper); border:1px solid var(--line); border-radius:10px; margin:10px 0; }
details.doc > summary { cursor:pointer; padding:12px 18px; font-weight:700; }
details.doc > div { padding:0 22px 16px; border-top:1px solid var(--line); }
details.doc table { border-collapse:collapse; width:100%; font-size:13.5px; display:block; overflow-x:auto; margin:10px 0; }
details.doc th, details.doc td { border-bottom:1px solid var(--line); padding:6px 8px; text-align:left; vertical-align:top; }
details.doc pre { background:var(--code); border-radius:8px; padding:12px 14px; overflow-x:auto; font-size:13px; line-height:1.5; }
details.doc code { background:var(--code); padding:1px 4px; border-radius:4px; } details.doc pre code { background:none; padding:0; }
details.doc h1 { font-size:20px; } details.doc h2 { font-size:17px; margin:22px 0 6px; }
.pilot img { max-width:100%; border-radius:8px; border:1px solid var(--line); }
.bar { position:static !important; }
@media (max-width:760px) { nav.toc { margin:0 -16px 8px; padding:8px 16px; } }
"""

REFRESH_JS = """<script>
// 构建号不同即刷新：每分钟及回到页面时检查 /version.json（不走缓存），网站更新后所有已打开的页面自动重新加载
(function () { var B = '__BUILD__';
  function chk() { fetch('/version.json?t=' + Date.now(), { cache: 'no-store' }).then(function (r) { return r.ok ? r.json() : null; })
    .then(function (v) { if (v && v.build && v.build !== B) location.reload(); }).catch(function () {}); }
  setInterval(chk, 60000); document.addEventListener('visibilitychange', function () { if (!document.hidden) chk(); });
})();
function loadDeck() { var b = document.getElementById('deckbox');
  b.innerHTML = '<iframe src="deck/?theme=light&v=__BUILD__" title="三维幻灯片" allow="fullscreen"></iframe>'; }
</script>"""


def md(text):
    return markdown.markdown(text, extensions=["tables", "fenced_code", "sane_lists"])


def md_doc(src):
    """附录文档：站内文档链接改为页内锚点，其余仓库相对路径改为 GitHub 地址。"""
    text = open(os.path.join(DOCS, src), encoding="utf-8").read()
    base = posixpath.dirname(src)

    def fix(m):
        label, target = m.group(1), m.group(2)
        if re.match(r"^[a-z]+://|^#|^mailto:", target):
            return m.group(0)
        path, _, frag = target.partition("#")
        rp = posixpath.normpath(posixpath.join(base, path))
        url = "#" + ANCHOR[rp] if rp in ANCHOR else GH + rp + (("#" + frag) if frag else "")
        return f"[{label}]({url})"
    return md(re.sub(r"(?<!!)\[([^\]]+)\]\(([^)\s]+)\)", fix, text))


def pilot_section():
    """试点数据：有小结时嵌入小结与其引用的图片（复制到 dist/pilot/），否则写明进行中。"""
    if not os.path.exists(PILOT):
        return '<p class="sub">首批试点数据正在采集，完成后更新于此（采集数量、场景分布、质量检查剔除率、数据划分与基线画质）。</p>'
    text = open(PILOT, encoding="utf-8").read()
    pdir = os.path.dirname(PILOT)
    os.makedirs(os.path.join(OUT, "pilot"), exist_ok=True)

    def img(m):
        alt, path = m.group(1), m.group(2)
        srcp = os.path.normpath(os.path.join(pdir, path))
        if os.path.exists(srcp):
            shutil.copy(srcp, os.path.join(OUT, "pilot", os.path.basename(srcp)))
            return f"![{alt}](pilot/{os.path.basename(srcp)})"
        return ""
    text = re.sub(r"!\[([^\]]*)\]\(([^)\s]+)\)", img, text)
    text = re.sub(r"^# .*\n", "", text, count=1)     # 页面上已有小节标题
    return f'<div class="pilot">{md(text)}</div>'


def sections_head():
    return f"""<header class="top"><b>{html.escape(TITLE)}</b><span>HGamey · 2026-11-02</span></header>
<nav class="toc" aria-label="目录"><a href="#summary">摘要</a><a href="#loop">一、研究闭环实录</a><a href="#topology">二、部署拓扑</a>
<a href="#dataset">三、训练数据</a><a href="#results">四、闭环产出</a><a href="#materials">五、演示材料</a><a href="#appendix">附录</a></nav>

<h1 id="summary" class="sec">{html.escape(TITLE)}</h1>
<p class="lead">本页汇集本阶段的全部材料。重点有两项：一是由人与 AI agent 共同运转、在真实手机上自动完成「提出假设—实验—判定—交付—复现」的研究闭环；二是面向学习型超分辨率的训练数据与数据生成方法。超分辨率与插帧的性能提升是该闭环的产出之一。</p>
<ul class="facts">
  <li><b>研究闭环。</b>以原神超分辨率与插帧研究链为例：4.7 天内 AI agent 留言 126 条、交付 31 次、运行真机任务 81 次；12 个研究方向中 2 个被否决、3 个改判；人工干预 24 次，自指导写入到 agent 回应的中位时间为 23 分钟。第一部分为平台原始记录。</li>
  <li><b>训练数据。</b>明确了五类高低清配对数据的采集方法及其真机验证状态，并将采集纳入同一平台，使数据可随研究持续产生、可追溯。见第三部分。</li>
  <li><b>闭环产出。</b>上屏帧率由 29.8 fps 提升至 117.8 fps，每帧 GPU 忙碌时间降低 51%，整机功耗增加约 166%；相同低分辨率输入下，自研超分辨率的 PSNR（37.9 dB）高于 FSR1（36.6 dB）与 GSR（31.7 dB）。见第四部分。</li>
</ul>

<h2 id="loop" class="part sec">一、研究闭环实录</h2>
"""


def sections_tail():
    appendix = "\n".join(f'<details class="doc" id="{a}"><summary>{html.escape(t)}</summary><div>{md_doc(s)}</div></details>'
                         for s, a, t in APPENDIX)
    return f"""
<h2 id="topology" class="part sec">二、部署拓扑</h2>
<p class="sub">中心节点集中保存研究、任务与结果；手机经 USB 连接节点；新增的执行电脑主动连接中心，无需入站端口。各模块可独立交付，部署步骤见附录。</p>
<div class="topo">
  <div class="tier"><div class="nd teal"><b>人员</b><span>浏览器：新建任务、查看过程、留言指导、盲评</span></div>
    <div class="nd teal"><b>外部 AI agent</b><span>研究、开发、运维等多个 agent；命令行 pf-node，各持独立身份，定时唤醒</span></div></div>
  <div class="link">↓ HTTPS（网页 / pf-node 命令行）</div>
  <div class="tier"><div class="nd beige"><b>公网入口（可选）</b><span>Cloudflare 隧道；仅在内网使用时可直连节点</span></div></div>
  <div class="link">↓</div>
  <div class="tier"><div class="nd"><b>中心节点：节点服务</b><span>任务队列与调度、研究库、网页看板、HTTP API</span></div>
    <div class="nd"><b>中心节点：设备层</b><span>设备锁、采集与评测、灰档 Vulkan 层</span></div>
    <div class="nd beige"><b>执行节点（可选）</b><span>其他电脑及其手机；出站长轮询主动连接中心</span></div></div>
  <div class="link">↓ USB（adb / hdc）</div>
  <div class="tier"><div class="nd amber"><b>Android 手机（root）</b><span>超分辨率、插帧与 A/B 测量</span></div>
    <div class="nd amber"><b>Android 手机</b><span>刷新率、温度等测量</span></div>
    <div class="nd amber"><b>HarmonyOS 手机</b><span>帧率与温度测量</span></div></div>
  <div class="link">↑ 按提交部署平台代码　　↑ 按提交号取实验代码</div>
  <div class="tier"><div class="nd beige"><b>代码仓库</b><span>平台代码 hgamey、实验脚本 experiments、文档与报告 docs</span></div></div>
</div>

<h2 id="dataset" class="part sec">三、训练数据</h2>
<p class="sub">学习型超分辨率需要同一游戏状态下的低清与高清配对。模拟配对成本低、覆盖广；真实配对包含真实低分辨率渲染的特征，用于微调与评测。详细方案见附录「高低清数据集采集方案」。</p>
<div class="tblwrap"><table class="tbl">
<tr><th>方法</th><th>原理</th><th>用途</th><th>真机验证状态</th></tr>
<tr><td>原生帧模拟降采样</td><td>采集原生高清帧，离线缩小生成低清；标注为模拟配对并注明缩放核</td><td>预训练、快速迭代</td><td>管线已在使用</td></tr>
<tr><td>同帧双分辨率回放</td><td>以 RenderDoc 捕获同一帧，分别按两种分辨率回放，得到严格对齐的真实配对</td><td>微调、真实测试集</td><td>一款游戏 18 对已完成</td></tr>
<tr><td>游戏模式真实低清</td><td>系统游戏模式降分辨率后截图；两次冷启动画面不同，不能作为配对</td><td>域差检查</td><td>截图路径已跑通</td></tr>
<tr><td>层内帧导出</td><td>在送显处无损逐帧读回游戏原始输出，替代经系统放大的截图</td><td>更高质量的采帧</td><td>代码已有，尚未上机</td></tr>
<tr><td>白盒负载同帧双分辨率</td><td>参考负载在同一帧内渲染两种分辨率</td><td>管线自检</td><td>仅通过宿主测试</td></tr>
</table></div>
<h3>生成流程：采集纳入同一闭环</h3>
<div class="steps">
  <div><b>采帧</b>agent 提交真机采集任务：自动进入游戏场景，保留有效画面，剔除菜单、加载页与冻结帧</div>
  <div><b>生成配对</b>模拟配对：高清缩小并记录缩放核；真实配对：同帧双分辨率回放</div>
  <div><b>质量检查</b>完整性校验、对齐偏移不超过 0.25 像素、逐块重建 PSNR 下限 28 dB；剔除超过 5% 整批不发布</div>
  <div><b>划分</b>按场景片段划分训练、验证与测试集，同一片段不跨集</div>
  <div><b>入库与基线锁定</b>清单记录来源、设备、游戏版本与任务号；每个版本固定双线性与双三次基线</div>
</div>
<h3>首批试点</h3>
{pilot_section()}

<h2 id="results" class="part sec">四、闭环产出：超分辨率与插帧</h2>
<p class="sub">实测为原神默认设置下的真机配对测量（新旧方案交替运行、每次重启游戏）；离线为对同一帧截图的计算。</p>
<div class="stats">
  <div class="stat"><div class="k">上屏帧率</div><div class="v">117.8<small>fps</small></div><div class="d">原生 29.8 fps；6 对，p = 0.031</div></div>
  <div class="stat"><div class="k">每帧 GPU 忙碌</div><div class="v">−51.1<small>%</small></div><div class="d">95% 置信区间 [−57.1%, −46.2%]</div></div>
  <div class="stat"><div class="k">整机功耗</div><div class="v">+166<small>%</small></div><div class="d">2.94 W 增至 7.82 W；每上屏帧能耗下降约 30%</div></div>
  <div class="stat"><div class="k">人工盲评</div><div class="v">第 2<small>名</small></div><div class="d">自研超分辨率仅次于原生（图片）</div></div>
</div>
<div class="tblwrap" style="margin-top:14px"><table class="tbl">
<tr><th>方法（0.56 倍渲染）</th><th>每帧 GPU</th><th>整机功耗</th><th>PSNR（离线）</th></tr>
<tr><td>双线性（系统放大）</td><td>−40.6%</td><td>−14.8%</td><td>37.0 dB</td></tr>
<tr><td><b>自研超分辨率</b></td><td>−29.1%</td><td>−7.2%</td><td><b>37.9 dB</b></td></tr>
<tr><td>高通 GSR（SGSR1 移植）</td><td>−27.4%</td><td>−10.7%</td><td>31.7 dB</td></tr>
<tr><td>AMD FSR1（层内移植）</td><td>−12.9%</td><td>−4.3%</td><td>36.6 dB</td></tr>
</table></div>
<p class="sub">GPU 各 6 对；功耗各 3 对（最小可能 p 值 0.25，仅供量级参考）。FSR1 为 fp32 逐式移植，未采用官方 fp16 优化。</p>

<h2 id="materials" class="part sec">五、演示材料</h2>
<p class="sub">三维幻灯片（12 页，含背景与术语说明）按需加载，图表可拖动旋转，← → 翻页，O 键总览。演示文稿与之内容一致，文字均可编辑。</p>
<div class="deckbox" id="deckbox"><div class="start"><button class="btn" onclick="loadDeck()">加载三维幻灯片</button>
  <span class="sub" style="margin:0">约 11 MB，首次加载需数秒</span></div></div>
<p style="margin-top:12px"><a class="btn ghost" href="deck/" target="_blank" rel="noopener">全屏打开幻灯片</a>　<a class="btn ghost" href="files/{PPTX_OUT}" download>下载演示文稿（PPTX）</a></p>

<h2 id="appendix" class="part sec">附录</h2>
<p class="sub">点击标题展开。代码与原始数据：<a href="https://github.com/HGamey/experiments/pull/13">experiments #13</a>、<a href="https://github.com/HGamey/experiments/pull/14">#14</a>；<a href="https://github.com/HGamey/docs/tree/main/experiments/2026-10-24-selfsr">实验报告</a>；时间线数据 <a href="timeline.json">timeline.json</a>。</p>
{appendix}
"""


def single_page(build):
    src = open(os.path.join(HERE, "loop", "index.html"), encoding="utf-8").read()
    src = re.sub(r"<title>.*?</title>", f"<title>{html.escape(TITLE)}</title>", src, count=1, flags=re.S)
    src = src.replace("</style>", CSS_EXTRA + "</style>", 1)
    # 闭环实录原有的页眉与 h1 换成：页眉 + 目录 + 摘要 + 第一部分标题
    head_re = re.compile(r'<header class="top">.*?</header>\s*<h1>[^<]*</h1>', re.S)
    assert head_re.search(src), "loop/index.html 结构变了（找不到页眉与 h1）"
    src = head_re.sub(lambda m: sections_head(), src, count=1)
    assert "<footer>" in src
    src = src.replace("<footer>", sections_tail() + "\n<footer>", 1)
    src = src.replace("</body>", REFRESH_JS.replace("__BUILD__", build) + "\n</body>", 1)
    open(os.path.join(OUT, "index.html"), "w", encoding="utf-8").write(src)


def redirects():
    for path, anchor in REDIRECTS.items():
        dst = os.path.join(OUT, path)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        up = "../" * path.count("/")
        open(dst, "w", encoding="utf-8").write(
            f'<!doctype html><meta charset="utf-8"><meta http-equiv="refresh" content="0; url={up}#{anchor}">'
            f'<a href="{up}#{anchor}">已合并至主页面</a>')


def main():
    deck_dist = os.path.join(DECK, "dist")
    if not os.path.exists(os.path.join(deck_dist, "index.html")):
        sys.exit("缺少 ../selfsr-deck/dist：先在 selfsr-deck 运行 build.sh")
    build = time.strftime("%Y%m%d%H%M%S")
    shutil.rmtree(OUT, ignore_errors=True)
    shutil.copytree(deck_dist, os.path.join(OUT, "deck"),
                    ignore=shutil.ignore_patterns(".vercel", "middleware.js", "vercel.json", "package.json"))
    os.makedirs(os.path.join(OUT, "files"))
    shutil.copy(os.path.join(DOCS, "slides", PPTX), os.path.join(OUT, "files", PPTX_OUT))
    shutil.copy(os.path.join(HERE, "loop", "timeline.json"), OUT)
    single_page(build)
    redirects()
    json.dump({"build": build}, open(os.path.join(OUT, "version.json"), "w"))
    for f in ("middleware.js", "vercel.json"):
        shutil.copy(os.path.join(DECK, "deploy", f), OUT)
    open(os.path.join(OUT, "package.json"), "w").write('{\n  "name": "selfsr-deck",\n  "private": true\n}\n')
    n = sum(len(fs) for _, _, fs in os.walk(OUT))
    print(f"dist/ 就绪：{n} 个文件，构建号 {build}")


if __name__ == "__main__":
    main()
