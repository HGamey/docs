#!/usr/bin/env python3
"""刷新 content.json 里的 auto 段（PR 计数、同事的 PR/issue 清单），其余手写字段不动。

用法: python3 -I collect.py content-vNN.json [--day YYYY-MM-DD] [--since "YYYY-MM-DD HH:MM"]
      --day 缺省今天，列「当天合并」；--since 给上一版的截止时刻（太平洋时间），列「上一版之后合并」
依赖: 已登录的 gh CLI。只读 GitHub，不改任何仓库。
"""
import json, subprocess, sys, datetime, zoneinfo, os

OURS = "BH3GEI"
SINCE = "2026-09-11"
REPOS = ["BH3GEI/phonefarm", "HGamey/game_opt_loop", "HGamey/refbench", "HGamey/sr_loop",
         "HGamey/docs", "HGamey/.github", "HGamey/knobs", "HGamey/harness-ops", "HGamey/phonefarm"]
SYNC_REPO = "HGamey/phonefarm"   # BH3GEI/phonefarm 的镜像, 我们在这里的 PR 是同步, 不算新工作
ISSUE_REPOS = ["HGamey/.github", "HGamey/game_opt_loop", "HGamey/refbench", "HGamey/docs"]
SJ = zoneinfo.ZoneInfo("America/Los_Angeles")


def gh(*args):
    return json.loads(subprocess.run(["gh", *args], check=True, capture_output=True, text=True).stdout or "[]")


def sj(ts):
    return datetime.datetime.fromisoformat(ts.replace("Z", "+00:00")).astimezone(SJ)


def main():
    args = sys.argv[1:]
    day = None
    since_ts = None
    if "--since" in args:   # 上一版截止时刻（太平洋时间 "YYYY-MM-DD HH:MM"），列出之后合并的
        i = args.index("--since"); since_ts = datetime.datetime.fromisoformat(args[i + 1]).replace(tzinfo=SJ); del args[i:i + 2]
    if "--day" in args:
        i = args.index("--day"); day = args[i + 1]; del args[i:i + 2]
    if not args:
        sys.exit("用法: python3 -I collect.py content-vNN.json [--day YYYY-MM-DD]")
    path = args[0]
    day = day or datetime.datetime.now(SJ).strftime("%Y-%m-%d")
    today, after = [], []
    content = json.load(open(path, encoding="utf-8"))
    since = datetime.datetime.fromisoformat(SINCE).replace(tzinfo=SJ)
    our_counts, colleague_prs, our_recent, sync_n, by_day = [], [], [], 0, {}
    for r in REPOS:
        prs = gh("pr", "list", "-R", r, "--state", "all", "--limit", "500",
                 "--json", "number,title,author,state,createdAt,mergedAt,isDraft,baseRefName")
        mine = [p for p in prs if p["author"]["login"] == OURS and p["state"] == "MERGED" and sj(p["mergedAt"]) >= since]
        if r == SYNC_REPO:
            sync_n = len(mine)
        else:
            our_counts.append({"repo": r, "merged": len(mine), "numbers": sorted(p["number"] for p in mine),
                               "docs": sum(1 for p in mine if p["title"].startswith("docs"))})
            if since_ts:
                after += [{"repo": r, "number": p["number"], "title": p["title"]} for p in mine if sj(p["mergedAt"]) > since_ts]
            today += [{"repo": r, "number": p["number"], "title": p["title"]} for p in mine if sj(p["mergedAt"]).strftime("%Y-%m-%d") == day]
            for p in mine:
                by_day[sj(p["mergedAt"]).strftime("%m-%d")] = by_day.get(sj(p["mergedAt"]).strftime("%m-%d"), 0) + 1
            our_recent += [{"repo": r, "number": p["number"], "title": p["title"], "merged": sj(p["mergedAt"]).strftime("%m-%d %H:%M")}
                           for p in mine if sj(p["mergedAt"]) >= datetime.datetime.now(SJ) - datetime.timedelta(days=2)]
        for p in prs:
            if p["author"]["login"] != OURS and sj(p["createdAt"]) >= since:
                colleague_prs.append({"repo": r, "number": p["number"], "author": p["author"]["login"], "title": p["title"],
                                      "state": p["state"], "draft": p["isDraft"], "base": p["baseRefName"],
                                      "created": sj(p["createdAt"]).strftime("%m-%d %H:%M"),
                                      "merged": sj(p["mergedAt"]).strftime("%m-%d") if p["mergedAt"] else None})
    issues = []
    for r in ISSUE_REPOS:
        try:
            its = gh("issue", "list", "-R", r, "--state", "all", "--limit", "200", "--json", "number,title,author,state,createdAt")
        except subprocess.CalledProcessError:
            continue  # 仓库关了 issue
        issues += [{"repo": r, "number": i["number"], "author": i["author"]["login"], "title": i["title"], "state": i["state"],
                    "created": sj(i["createdAt"]).strftime("%m-%d %H:%M")}
                   for i in its if i["author"]["login"] != OURS and sj(i["createdAt"]) >= since]
    colleague_prs.sort(key=lambda p: (p["repo"], p["number"]))
    our_counts.sort(key=lambda c: -c["merged"])
    content["auto"] = {
        "updated_at": datetime.datetime.now(SJ).strftime("%Y-%m-%d %H:%M"),
        "since": SINCE,
        "our_pr_counts": our_counts,
        "our_total": sum(c["merged"] for c in our_counts),
        "day": day,
        "our_today": sorted(today, key=lambda p: (p["repo"], p["number"])),
        "our_today_n": len(today),
        "since_prev": since_ts.strftime("%Y-%m-%d %H:%M") if since_ts else None,
        "our_since": sorted(after, key=lambda p: (p["repo"], p["number"])),
        "our_since_n": len(after),
        "our_docs": sum(c["docs"] for c in our_counts),
        "our_by_day": dict(sorted(by_day.items())),
        "sync_prs": sync_n,
        "our_recent": sorted(our_recent, key=lambda p: p["merged"], reverse=True),
        "colleague_prs": colleague_prs,
        "colleague_open": sum(1 for p in colleague_prs if p["state"] == "OPEN"),
        "colleague_merged": sum(1 for p in colleague_prs if p["state"] == "MERGED"),
        "colleague_issues": issues,
    }
    json.dump(content, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    a = content["auto"]
    print(f"auto 已刷新 {a['updated_at']}: 我们合并 {a['our_total']} (另同步 {a['sync_prs']}), "
          f"同事 PR open {a['colleague_open']} / merged {a['colleague_merged']}, issue {len(issues)}")


if __name__ == "__main__":
    main()
