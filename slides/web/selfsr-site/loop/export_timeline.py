#!/usr/bin/env python3
"""export_timeline.py — 从平台数据库只读导出「超分研究链」的原始时间线，写 timeline.json（index.html 内嵌同一份数据）。

链 = 根任务 selfsr-genshin-ab-20261028 及所有 follows 接续任务。数据全部来自节点 HTTP API（ssh macmini3 后 curl
127.0.0.1:8765，只发 GET）和任务目录里的 meta.json / report.md / soak_summary.json（只读）。时间换算为北京时间。
文本保留原话，只做两类替换：手机型号与设备序列号（改为「root Android 手机」等），个人账号名（改为「前任研究者」）。

    python3 -I export_timeline.py            # 读平台，写 timeline.json，并把数据嵌入 index.html
    python3 -I export_timeline.py --raw raw.json   # 同时保存原始导出，便于核对
"""
import datetime
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "selfsr-genshin-ab-20261028"
NODE = "d944a2c6ec60405db5de445506d59360"

# 在节点上执行：curl 读 API（只 GET），读任务目录（只读）。输出一个 JSON。
REMOTE = r'''
import json, os, glob, subprocess, sys
T = open(os.path.expanduser("~/hgamey/.state/node.token")).read().strip()
def get(p):
    out = subprocess.run(["curl", "-s", "--noproxy", "*", "-H", "Authorization: Bearer " + T, "-H",
                          "X-Expected-Node-ID: %s", "http://127.0.0.1:8765" + p], capture_output=True, text=True).stdout
    return json.loads(out, strict=False)
def pages(p):
    out, cur = [], None
    while True:
        d = get(p + ("&cursor=" + cur if cur else ""))
        out += d.get("items", []); cur = d.get("next_cursor")
        if not cur or d.get("has_more") is False or not d.get("items"): return out
allr = pages("/research-state/research?limit=100")
ids, grew = {"%s"}, True
while grew:
    grew = False
    for r in allr:
        if r.get("follows") in ids and r["id"] not in ids: ids.add(r["id"]); grew = True
R = {"research": {}, "notes": [], "directions": {}, "requests": {}, "feedback": [], "jobs": []}
for i in sorted(ids):
    R["research"][i] = pages("/research-state/research/%%s/history?limit=100" %% i)
    R["notes"] += pages("/research-state/note?research_id=%%s&limit=100" %% i)
    for d in pages("/research-state/direction?research_id=%%s&limit=100" %% i):
        R["directions"][d["id"]] = pages("/research-state/direction/%%s/history?limit=100" %% d["id"])
    for q in pages("/research-state/request?research_id=%%s&limit=100" %% i):
        R["requests"][q["id"]] = pages("/research-state/request/%%s/history?limit=100" %% q["id"])
        for f in pages("/research-state/feedback?request_id=%%s&limit=100" %% q["id"]):
            R["feedback"] += pages("/research-state/feedback/%%s/history?limit=100" %% f["id"])
for m in glob.glob(os.path.expanduser("~/hgamey/phonefarm/_jobs/*/meta.json")):
    j = json.loads(open(m).read(), strict=False)
    if j.get("research_id") not in ids: continue
    jd = os.path.dirname(m)
    j = {k: j.get(k) for k in ("id", "argv", "actor_id", "research_id", "direction_id", "experiment", "request_id",
                               "started", "ended", "outcome", "exit")}
    j["reports"] = {p[len(jd) + 5:]: open(p).read() for p in glob.glob(jd + "/out/**/report.md", recursive=True)}
    s = glob.glob(jd + "/out/soak_summary.json")
    j["soak"] = {k: v for k, v in json.load(open(s[0])).items() if not isinstance(v, (list, dict)) or k == "invalid_reasons"} if s else None
    R["jobs"].append(j)
json.dump(R, sys.stdout, ensure_ascii=False)
''' % (NODE, ROOT)

SANITIZE = [(r"(?:RedMagic|红魔)(?:\s*NX809J)?|NX809J", "root Android 手机"), (r"91253241019A", "<设备>"), (r"HONOR(?: AAP-AN00)?|荣耀机?", "另一台手机"),
            (r"HUAWEI|华为", "另一台手机"), (r"ZTE ", "厂商 "), (r"BH3GEI", "前任研究者")]
KEY_ROWS = {"实际上屏帧率": "上屏fps", "每帧 GPU 忙碌": "GPU忙碌ms", "锐度 (拉普拉斯方差)": "锐度", "右上角固定图标区锐度": "HUD锐度",
            "游戏渲染 p95 帧时": "渲染p95ms", "整机功耗": "功耗mW", "窗口内电池温度": "电池°C", "SF 丢帧": "SF丢帧"}
# 人工通道写入的来源：只写日志（plans/2026-10-28-recovery.md）里有明确记载的；其余为「未记录」
PROVENANCE = {
    "n-6f351571f486": "用户要求（日志记用户原话「性能上可能也需要对比？不然优化都是自己吹的」「需要盲评」），写入者未记录",
    "research:212d97d0-60b6-417f-8640-a8692adf73af": "用户要求（同上），写入者未记录",
    "n-0547cde14df7": "用户要求（日志记用户原话「盲评我觉得得排序，不然肯定是原生大分辨率最好啊」），写入者未记录",
    "n-490d72c348f9": "用户要求（日志记用户原话「我们不能直接连着在一个任务上回复吗？」），写入者未记录",
    "n-dd6ca540347d": "用户要求（日志记「继承树，用户 10-31 要的」），写入者未记录",
    "n-202cf3b8852c": "运维 agent 以人的身份写入（日志：验收第五轮），依据第二个研究 agent 的复现",
    "n-27f13e2cd7d2": "运维 agent 以人的身份写入（日志：验收第五轮）",
    "n-9ce8d545a331": "运维 agent 以人的身份写入（日志：第七轮，BRIEF §15）",
    "research:4ee0d174-5f98-409d-8abc-a03dec57d7b8": "运维 agent 以人的身份建任务并指派（日志：第八轮）",
    "n-2048bd51785c": "日志记为运维 agent 布置（glm-ops §23）",
    "n-13089d2f2d94": "原文自述为转达维护者 agent 此前的要求",
    "feedback": "管理员 token 提交，记录里没有评判人；日志记用户确认不是本人所评，结果按单人评判使用",
}
# 3–4 个关键时刻：之前 → 人工指导 → 之后（全部引用原始记录 ID，文字取自记录）
MOMENTS = [
    {"title": "只报自己的数 → 必须和基线比", "before": ["n-40ca182884e4"], "steer": "n-6f351571f486",
     "after": ["n-453b40020fbe", "n-24282301aa0c", "n-115fa78a8f8d", "n-b860cc4d468d", "n-dc0a106041c2"],
     "gist": {"before": "只和原生比：上屏 29.81 → 97.47 fps（+227%），HUD 锐度 347 → 872。没有任何基线。",
              "steer": "画质和性能都要和基线比（双线性、GSR、原生全分辨率），画质请人盲评。",
              "after": "四组同口径 6 对配对：每帧 GPU 忙碌 超分+插帧 −50.0%、双线性 −40.6%、自研超分 −29.1%、GSR −27.4%；"
                       "同帧 PSNR 自研 37.9 / 双线性 37.0 / SGSR1 31.7 dB。对比中发现约一半的臂锁在 90 fps，"
                       "agent 追到根因并以 sfwait=off 消除，终版 29.78 → 117.75 fps（+295.4%，6/6 同向）。"}},
    {"title": "交付被复现推翻 → 撤回推荐、修好再验", "before": ["n-cfbbae450e77", "n-db7650ff9080"], "steer": "n-202cf3b8852c",
     "after": ["n-c96d37a41977", "n-018b7e4aefee", "n-c82c02d74143", "n-685f8cc49ee2"],
     "gist": {"before": "交付写「默认推荐 reassert」，而它依据的任务 a9a8ea236037 自己的汇总已是 valid=false（游戏前台 130/1800 秒）。"
                        "第二个 agent 原样复现，查出原任务与复现都没有加载超分层。",
              "steer": "没有有效数据就撤回推荐；查清层为什么没加载。",
              "after": "25 分钟后撤回推荐、方向改 deferred；查出两处计时错误（会话 1200 秒到期、按循环次数计时）。"
                       "改墙钟后取得有效 1800 秒样本（前台 360/360，超分 97 800 帧）再判成立；第二个 agent 按新交付复现成立。"}},
    {"title": "盲评 4 选 1 → 全排序、同一画面重录", "before": ["n-74218afb7f3c"], "steer": "n-0547cde14df7",
     "steer2": "n-c49122d931ba", "after": ["n-179e02dbb371", "n-c88d7fa4969e", "n-2276eeff3fdd", "n-8ce4ab2c533e", "n-14763780f86d"],
     "gist": {"before": "两组盲评都是 4 选 1，选项里有原生全分辨率；两天内 0 份答复。",
              "steer": "改成全排序并标差距；视频四段不是同一画面，重录。",
              "after": "全排序重发；同一会话、同一地点重录四段；agent 自查发现题目泄露了对应关系，关闭后用中性说明重发。"
                       "结果：图像 原生 ›小› 自研超分 ›中› SGSR1 ›中› 双线性；视频 超分+插帧 ›大› 自研超分 ›中› 双线性 ›小› 原生（单人评判，评判人未记录）。"}},
    {"title": "第二个 agent 独立复现 → 根任务三处说法被限定", "before": ["n-40ca182884e4", "n-09560fca6597", "n-b93f2daddbc0"],
     "steer": "n-2048bd51785c", "after": ["n-a343b198efc3"],
     "gist": {"before": "根任务写「29.8 → 97.5」「≥56°C 时 74–75 fps」、功耗 +165.9%。第二个 agent 独立复现得 29.773 → 118.563 fps，并指出三处说法需要限定。",
              "steer": "逐条回应三点，采纳的在下次交付更正。",
              "after": "19 分钟后三点全部采纳：97.5 注明为旧候选六臂均值（终版 117.75，与复现 118.56 一致）；"
                       "74–75 fps 改为高温段高频异常档而非必然；功耗改双口径 7.82 W（+165.9%）/ 干净两臂 7.36 W（+148%）。另自纠一处归因。"}},
]
# 每次人工干预带来的变化（整理说明，依据见对应回应留言、方向结论与交付修订）
EFFECT = {
    "research:f0806930-3f34-4ebc-ba12-306946f7256a": "开第 9 组正式 A/B；两轮 4 个有效对否定组合方案，方向 srip-vs-gfrc-g9 判 rejected，维持第 7 组",
    "n-13089d2f2d94": "实验代码 PR（experiments #2）、60 Hz 机制附录（docs #22）、带图留言、hold120-rerun7 判 verified",
    "n-e793379034b6": "第 9 组报告改为逐对表格、作废臂单列原因、补 60 Hz 上限证据链",
    "research:97f6adb9-1c8c-4d96-9225-54efbb4a9087": "开 30 分钟长稳；四轮后方向先判 rejected，复核后改 deferred",
    "n-26c6f2d49b53": "报告加游戏存活时间线；中断证据单列，随后对照跑撤回「游戏冻结」的说法",
    "n-ece7f392217d": "告知预留已释放；agent 预检设备后提交对照跑与 v3",
    "research:e736a7c5-3b69-4c30-a99d-13ec6e880a72": "开 60 Hz 凹陷缓解研究（warm-set 与 reassert 两种办法）",
    "n-9a4f581d8355": "两种办法合并为同一对比表；交付顶部标注两轮均停充、不外推充电场景",
    "research:c0196485-c285-440d-a29b-4da8f44b451b": "第二个 agent 原样复现缓解交付，查出原任务与复现均 valid=false、超分层未加载",
    "n-202cf3b8852c": "撤回「推荐 reassert」，方向改 deferred；定位会话 1200 秒到期与计时错误，修复后取得有效 1800 秒样本再判 verified",
    "n-27f13e2cd7d2": "逐轮核对后撤回「可长期用」，方向改 deferred；有效样本显示 42→57°C 无温度平台",
    "n-762a57c16f82": "交付引用的实验提交并入主线（experiments PR #7）",
    "research:b7c193ff-1a41-49a7-a45e-ec9d21b3a938": "第二个 agent 按修正后的交付再复现，有效样本与原交付一致",
    "research:212d97d0-60b6-417f-8640-a8692adf73af": "基线对比题目建立后并入根任务（同一题目只用一个任务）",
    "n-490d72c348f9": "拆出的任务收拢到根任务，此后不再另开任务",
    "n-6f351571f486": "新方向 sr-baselines：原生、双线性、SGSR1、GSR 上机与自研超分同口径对比；对比中发现 90 fps 双峰，引出 srip-cadence 与推荐配置更新",
    "n-37bab3d33954": "解释负责人为空的原因，任务改派",
    "n-9ce8d545a331": "发出两组盲评（4 选 1），四段视频列为交付产物，补测 GSR 上机",
    "n-dd6ca540347d": "三条方向继承写入 based_on；agent 自查发现更新会清除已下结论并恢复",
    "research:4ee0d174-5f98-409d-8abc-a03dec57d7b8": "第二个 agent 独立复现终版推荐配置：29.773→118.563 fps，并对根任务提出三处限定",
    "n-0547cde14df7": "两组盲评改为全排序并标注相邻差距，旧请求关闭",
    "n-c49122d931ba": "四段视频改为同一会话、同一地点重录；agent 自查题目泄露对应关系后再次重发",
    "n-2048bd51785c": "三处限定全部采纳：97.5 注明为旧候选均值、74–75 fps 改为非必然、功耗改为双口径；另自纠一处归因",
}
TERMINAL = {"verified", "rejected", "deferred"}
REPRO = {"c0196485-c285-440d-a29b-4da8f44b451b", "b7c193ff-1a41-49a7-a45e-ec9d21b3a938", "4ee0d174-5f98-409d-8abc-a03dec57d7b8"}  # 第二个 agent 的复现任务


def clean(s):
    for a, b in SANITIZE:
        s = re.sub(a, b, s or "")
    return s


def bj(t):
    return datetime.datetime.fromtimestamp(t, datetime.timezone(datetime.timedelta(hours=8))).strftime("%Y-%m-%dT%H:%M:%S+08:00")


def trim(s, n=240):
    s = clean(s).strip()
    return s if len(s) <= n else s[:n].rstrip() + "…"


def who(label, channel):
    if channel == "dashboard_steer" or label == "human":
        return "人工"
    return label or ("管理员 token（未署名）" if channel == "admin_token" else "未记录")


def report_numbers(j):
    nums = {}
    for path, md in sorted(j["reports"].items()):
        arm = path.split("/")[-2]
        m = re.search(r"有效 (\d+) 对", md)
        row = {"有效对": int(m.group(1))} if m else {}
        m = re.search(r"作废臂: ([^。]*)。", md)
        if m and m.group(1) != "无":
            row["作废臂"] = m.group(1)[:120]
        for line in md.splitlines():
            cells = [c.strip() for c in line.strip("|").split("|")]
            for key, short in KEY_ROWS.items():
                if len(cells) == 7 and cells[0].startswith(key) and cells[6] != "未测成":
                    row[short] = {"A": cells[1], "B": cells[2], "差%": cells[4], "p": cells[6]}
        nums[arm] = row
    if j["soak"]:
        s = j["soak"]
        nums["soak_summary"] = {k: s.get(k) for k in ("valid", "invalid_reasons", "total_s", "fg_game_s", "fg_game_samples", "dip_seconds",
                                                      "reassert_events", "temp_min", "temp_max", "hz_mean") if s.get(k) is not None}
    return nums


def build(R):
    ev = []

    def add(t, actor, kind, text, research, refs, **kw):
        ev.append({"t": bj(t), "ts": round(t, 3), "actor": actor, "kind": kind, "research_id": research, "text": trim(text),
                   "refs": {k: v for k, v in refs.items() if v}, **{k: v for k, v in kw.items() if v}})

    research = {}
    for rid, hist in R["research"].items():
        hist.sort(key=lambda r: r["revision"])
        first = hist[0]
        research[rid] = {"id": rid, "title": clean(first["data"]["title"]), "follows": first["data"].get("follows"),
                         "state": hist[-1]["data"]["state"], "revisions": len(hist)}
        a = first["audit"]
        add(a["at"], who(a["actor_label"], a["auth_channel"]), "task created", first["data"]["title"], rid,
            {"revision": 1, "follows": first["data"].get("follows")}, channel=a["auth_channel"], provenance=PROVENANCE.get("research:" + rid))
        for prev, r in zip(hist, hist[1:]):
            a = r["audit"]
            if a.get("reason") == "交付":
                add(a["at"], who(a["actor_label"], a["auth_channel"]), "delivery",  # 交付全文很长，只取开头
                    "交付 rev%d：%s" % (r["revision"], re.sub(r"\n\s*\n+", "\n", (r["data"].get("deliverable") or {}).get("summary") or "")[:150]), rid,
                    {"revision": r["revision"], "artifacts": len((r["data"].get("deliverable") or {}).get("artifacts") or [])})
            elif r["data"]["state"] == "closed" and prev["data"]["state"] != "closed":
                add(a["at"], who(a["actor_label"], a["auth_channel"]), "human steer", "关闭任务：" + (a.get("reason") or ""), rid,
                    {"revision": r["revision"]}, channel=a["auth_channel"])
    directions = {}
    for did, hist in R["directions"].items():
        hist.sort(key=lambda r: r["revision"])
        d0, seen, prev = hist[0], set(), None
        directions[did] = {"id": did, "research_id": d0["research_id"], "title": clean(hist[-1]["data"]["title"]),
                           "state": hist[-1]["data"]["state"], "based_on": hist[-1]["data"].get("based_on"),
                           "verdicts": [], "revisions": len(hist)}
        add(d0["audit"]["at"], who(d0["audit"]["actor_label"], d0["audit"]["auth_channel"]), "direction proposed",
            d0["data"]["title"] + "。假设：" + (d0["data"].get("hypothesis") or ""), d0["research_id"], {"direction": did, "revision": 1})
        for r in hist:
            st, reason = r["data"]["state"], (r["audit"].get("reason") or "")
            if st in TERMINAL and reason and reason not in seen and not reason.startswith("恢复"):
                jobs = sorted({e["job_id"] for e in r["data"].get("evidence_refs") or [] if e.get("job_id")})
                add(r["audit"]["at"], who(r["audit"]["actor_label"], r["audit"]["auth_channel"]), "verdict",
                    "%s → %s：%s" % (did, st, reason), d0["research_id"], {"direction": did, "revision": r["revision"], "jobs": jobs},
                    verdict=st, previous=prev if prev in TERMINAL and prev != st else None)
                directions[did]["verdicts"].append({"revision": r["revision"], "state": st, "t": bj(r["audit"]["at"])})
            seen.add(reason)
            prev = st
    notes = sorted(R["notes"], key=lambda n: n["created_at"])
    for n in notes:
        actor = who(n["actor_label"], n["auth_channel"])
        if n["auth_channel"] == "dashboard_steer":
            kind = "human steer"
        elif n["decision"]:
            kind = "agent response " + n["decision"]
        elif n["actor_label"] == "claude-lead" or n["auth_channel"] == "admin_token":
            kind = "maintainer note"
        elif n["research_id"] in REPRO:
            kind = "reproduction"
        else:
            kind = "agent note"
        jobs = sorted({e["job_id"] for e in n["evidence_refs"] if e.get("job_id")})
        add(n["created_at"], actor, kind, n["text"], n["research_id"],
            {"note": n["id"], "reply_to": n["reply_to"], "to": n["to"], "jobs": jobs}, channel=n["auth_channel"],
            provenance=PROVENANCE.get(n["id"]), full=clean(n["text"]) if len(n["text"]) > 240 else None)
    for qid, hist in R["requests"].items():
        q = sorted(hist, key=lambda r: r["revision"])
        d, a = q[0]["data"], q[0]["audit"]
        add(a["at"], who(a["actor_label"], a["auth_channel"]), "blind review request",
            "%s。要求：%s%s" % (d["title"], d.get("need") or "", "（关闭：%s）" % q[-1]["data"]["close_reason"] if q[-1]["data"].get("close_reason") else ""),
            d["research_id"], {"request": qid, "pick": (d.get("evaluation") or {}).get("pick"), "items": len((d.get("evaluation") or {}).get("items") or [])})
    for f in R["feedback"]:
        d, a = f["data"], f["audit"]
        add(a["at"], who(a["actor_label"], a["auth_channel"]), "blind review answer", d["text"], f["research_id"],
            {"feedback": f["id"], "request": d["request_id"]}, channel=a["auth_channel"], provenance=PROVENANCE["feedback"],
            ranking=(d.get("evaluation") or {}).get("picked"), gaps=(d.get("evaluation") or {}).get("gaps"))
    for j in R["jobs"]:
        actor = j["actor_id"] or "未记录（无 agent 身份）"
        refs = {"job": j["id"], "direction": j.get("direction_id"), "commit": ((j.get("experiment") or {}).get("commit") or "")[:8],
                "request_id": j["request_id"] if j["request_id"] and len(j["request_id"]) < 48 else None}
        add(j["started"], actor, "job submitted", clean(" ".join(j["argv"] or []))[:200], j["research_id"], refs)
        if j.get("ended"):
            add(j["ended"], actor, "job result", "%s：%s，exit %s" % (j["id"], j["outcome"], j["exit"]), j["research_id"], refs,
                outcome=j["outcome"], numbers=report_numbers(j) or None)
    ev.sort(key=lambda e: e["ts"])
    for i, e in enumerate(ev):
        e["seq"] = i + 1
    return ev, research, directions, R["jobs"]


def stats(ev, research, directions, jobs):
    from collections import Counter
    agents = [e for e in ev if e["actor"] in ("glm-researcher", "kimi-researcher", "claude-lead")]
    notes = [e for e in ev if e["refs"].get("note")]
    steers = [e for e in ev if e["actor"] == "人工" and e["kind"] in ("human steer", "task created")]
    agent_notes = [e for e in notes if e["actor"] in ("glm-researcher", "kimi-researcher")]
    lag = []
    for s in steers:
        if s["kind"] == "human steer" and not s["refs"].get("note"):
            continue  # 关闭任务不计响应时间
        if s["kind"] == "human steer" and s["refs"].get("note"):
            r = [e for e in agent_notes if e["refs"].get("reply_to") == s["refs"]["note"]]
        else:
            r = [e for e in ev if e["research_id"] == s["research_id"] and e["ts"] > s["ts"] and e["actor"] in ("glm-researcher", "kimi-researcher")]
        if r:
            r = min(r, key=lambda e: e["ts"])
            lag.append({"steer": s["refs"].get("note") or "research:" + s["research_id"], "t": s["t"], "kind": s["kind"],
                        "response": r["refs"].get("note") or r["kind"], "responder": r["actor"], "minutes": round((r["ts"] - s["ts"]) / 60, 1),
                        "provenance": s.get("provenance") or "未记录",
                        "text": s["text"][:120], "effect": EFFECT.get(s["refs"].get("note") or "research:" + s["research_id"])})
    mins = sorted(x["minutes"] for x in lag)
    first, last = ev[0]["ts"], ev[-1]["ts"]
    return {
        "start": ev[0]["t"], "end": ev[-1]["t"], "duration_hours": round((last - first) / 3600, 1),
        "tasks": len(research), "events": len(ev),
        "agent_writes": dict(Counter(e["actor"] for e in agents)),
        "notes": dict(Counter(e["actor"] for e in notes)),
        "deliveries": sum(e["kind"] == "delivery" for e in ev),
        "jobs": {"total": len(jobs), "by_outcome": dict(Counter(j["outcome"] for j in jobs)),
                 "by_actor": dict(Counter(j["actor_id"] or "未记录" for j in jobs))},
        "directions": {"total": len(directions), "final": dict(Counter(d["state"] for d in directions.values())),
                       "overturned": [d["id"] for d in directions.values() if len({v["state"] for v in d["verdicts"]}) > 1]},
        "human": {"steer_notes": sum(e["kind"] == "human steer" and bool(e["refs"].get("note")) for e in ev),
                  "tasks_created": sum(e["kind"] == "task created" and e["actor"] == "人工" for e in ev),
                  "task_closed": sum(e["kind"] == "human steer" and not e["refs"].get("note") for e in ev),
                  "blind_review_answers": sum(e["kind"] == "blind review answer" for e in ev),
                  "agent_decisions": dict(Counter(e["kind"].split()[-1] for e in ev if e["kind"].startswith("agent response")))},
        "steer_to_response": {"items": lag, "median_minutes": mins[len(mins) // 2] if mins else None,
                              "max_minutes": mins[-1] if mins else None, "min_minutes": mins[0] if mins else None},
    }


def main():
    raw = None
    if "--from" in sys.argv:
        R = json.load(open(sys.argv[sys.argv.index("--from") + 1]))
    else:
        out = subprocess.run(["ssh", "macmini3", "python3", "-"], input=REMOTE, capture_output=True, text=True, check=True).stdout
        R = json.loads(out, strict=False)
        if "--raw" in sys.argv:
            raw = sys.argv[sys.argv.index("--raw") + 1]
            json.dump(R, open(raw, "w"), ensure_ascii=False)
    ev, research, directions, jobs = build(R)
    by_note = {e["refs"]["note"]: e for e in ev if e["refs"].get("note")}
    moments = [{**m, **{k: [by_note[i]["seq"] for i in m[k]] for k in ("before", "after")},
                "steer": by_note[m["steer"]]["seq"], **({"steer2": by_note[m["steer2"]]["seq"]} if m.get("steer2") else {})} for m in MOMENTS]
    doc = {"schema": "hgamey/loop-timeline@1", "root": ROOT, "node_id": NODE,
           "exported_at": bj(datetime.datetime.now().timestamp()), "timezone": "北京时间 UTC+8",
           "note": "只读导出。文本为原始记录原话（超长截断，full 为全文），仅替换手机型号、设备序列号与个人账号名。"
                   "actor「人工」表示网页指导通道（dashboard_steer）或人工身份写入；provenance 只写日志里有明确记载的来源。"
                   "任务事件时间取 meta.json 的 started/ended（排队时间未记录）。",
           "stats": stats(ev, research, directions, jobs), "research": list(research.values()),
           "directions": list(directions.values()), "moments": moments, "events": ev}
    text = json.dumps(doc, ensure_ascii=False, indent=1)
    open(os.path.join(HERE, "timeline.json"), "w", encoding="utf-8").write(text + "\n")
    page = os.path.join(HERE, "index.html")
    if os.path.exists(page):
        html = open(page, encoding="utf-8").read()
        html = re.sub(r'(<script id="data" type="application/json">).*?(</script>)',
                      lambda m: m.group(1) + json.dumps(doc, ensure_ascii=False).replace("</", "<\\/") + m.group(2), html, flags=re.S)
        open(page, "w", encoding="utf-8").write(html)
    print("timeline.json：%d 个事件，%d 个任务，%d 个方向，%d 次真机任务" % (len(ev), len(research), len(directions), len(jobs)))


if __name__ == "__main__":
    main()
