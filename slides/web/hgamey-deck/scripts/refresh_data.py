#!/usr/bin/env python3
"""把 HGamey 各任务的结果复制/整理到 deck 的 data/ 目录（只新建或覆盖 data/ 下自己生成的文件，不动源数据）。

产出：
  data/leaderboard.json   模型排行榜（_tmp/models/leaderboard.json，去掉本机绝对路径）
  data/effects.json       真机判定结果：每个指标的效应、95% 区间、p、对数（跳过 dry/模拟设备 运行）
  <私有目录>/crops.json + crops/*.png   画面对比：原生 | 候选 | 差值×8 的裁剪块，附裁剪块 PSNR 与原生自噪声参考
                          游戏画面裁剪不进公开的 docs 仓：写到私有 internal 仓 decks/hgamey-deck/private/
                          （或环境变量 HGAMEY_DECK_PRIVATE 指定的目录），构建时由 build.rs 从那里嵌入。
  data/manifest.json      数据来源、生成时间、源文件 sha256

用法：python3 -I scripts/refresh_data.py [--hgamey /path/to/hgamey]
"""
import argparse, datetime, glob, hashlib, json, math, os, sys

try:
    from PIL import Image, ImageChops
except ImportError:  # pragma: no cover
    sys.exit("需要 Pillow：pip install pillow")

HERE = os.path.dirname(os.path.abspath(__file__))
DECK = os.path.dirname(HERE)
OUT = os.path.join(DECK, "data")
PRIV = os.environ.get("HGAMEY_DECK_PRIVATE") or os.path.abspath(os.path.join(DECK, "../../../../internal/decks/hgamey-deck/private"))

CROP_W, CROP_H = 192, 96          # 原图上的裁剪尺寸（像素）
DIFF_GAIN = 8                     # 差值放大倍数


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()


def strip_paths(o):
    if isinstance(o, dict):
        return {k: strip_paths(v) for k, v in o.items()}
    if isinstance(o, list):
        return [strip_paths(v) for v in o]
    if isinstance(o, str) and o.startswith("/Users/"):
        p = o.split("/hgamey/", 1)
        return ("hgamey/" + p[1]) if len(p) == 2 else os.path.basename(o)
    return o


def leaderboard(root, man):
    src = os.path.join(root, "_tmp/models/leaderboard.json")
    if not os.path.exists(src):
        return
    d = json.load(open(src, encoding="utf-8"))
    json.dump(strip_paths(d), open(os.path.join(OUT, "leaderboard.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    man["sources"].append({"what": "leaderboard", "path": "hgamey/_tmp/models/leaderboard.json", "sha256": sha256(src), "generated": d.get("generated")})


METRIC_ZH = {"frame_p95": "帧时 p95 (ms)", "fps_mean": "平均帧率 (fps)", "power_w_mean": "整机功耗 (W)", "missed_frac": "超时帧占比"}


def collect_per_metric(obj, out, ctx):
    """递归找 per_metric 字典。"""
    if isinstance(obj, dict):
        if "per_metric" in obj and isinstance(obj["per_metric"], dict):
            rows = []
            for k, m in obj["per_metric"].items():
                if not isinstance(m, dict) or "ci95" not in m:
                    continue
                ci = m.get("ci95") or [None, None]
                rows.append({
                    "metric": k, "metric_zh": METRIC_ZH.get(k, k),
                    "a_mean": m.get("a_mean"), "b_mean": m.get("b_mean"),
                    "diff": m.get("diff_mean"), "diff_pct": m.get("diff_pct"),
                    "ci_lo": ci[0], "ci_hi": ci[1], "p": m.get("p"),
                    "n": m.get("n", obj.get("n_pairs", obj.get("n"))),
                })
            if rows:
                out.append({**ctx, "compare": obj.get("x", "B") + " 对 " + obj.get("ref", "A") if "x" in obj else "改写臂 对 对照臂",
                            "alpha_win": obj.get("alpha_win"), "n_pairs": obj.get("n_pairs", obj.get("n")), "metrics": rows})
        for k, v in obj.items():
            if k != "per_metric":
                collect_per_metric(v, out, ctx)
    elif isinstance(obj, list):
        for v in obj:
            collect_per_metric(v, out, ctx)


def effects(root, man):
    pats = [
        ("测量链考题（红魔）", "_tmp/ruler/device-runner/raw/*/*/verdict.json"),
        ("原神 + NPU 三臂（N3）", "_tmp/npu/N3/runs/*/verdict.json"),
        ("刷新率对齐（O1）", "_tmp/opt/O1/runs/*/*/result.json"),
        ("刷新率对齐（O1）", "_tmp/opt/O1/runs/*/*/report.json"),
    ]
    res, skipped = [], []
    for src_name, pat in pats:
        for f in sorted(glob.glob(os.path.join(root, pat))):
            rel = os.path.relpath(f, root)
            low = rel.lower()
            if "dry" in low or "/sim" in low or "uiprobe" in low:
                skipped.append(rel)
                continue
            try:
                d = json.load(open(f, encoding="utf-8"))
            except Exception:
                continue
            parts = rel.split("/")
            ctx = {"source": src_name, "run": parts[-3], "case": parts[-2], "file": "hgamey/" + rel}
            collect_per_metric(d, res, ctx)
            man["sources"].append({"what": "verdict", "path": "hgamey/" + rel, "sha256": sha256(f)})
    # 功耗最小可分辨效应（device-runner 汇总）
    mde = []
    for f in sorted(glob.glob(os.path.join(root, "_tmp/ruler/device-runner/raw/power_*.json"))):
        d = json.load(open(f, encoding="utf-8"))
        rl = d.get("repeat_load", {})
        for k, v in (d.get("mde_power") or {}).items():
            mde.append({"file": "hgamey/" + os.path.relpath(f, root), "pairs": int(k), "windows": v.get("windows"),
                        "mde80_pct": v.get("mde80_pct"), "mde80_w": v.get("mde80_w"), "base_w": v.get("base_w"),
                        "false_pos_rate_at_0": v.get("false_pos_rate_at_0"),
                        "pair_diff_sd_w": rl.get("pair_diff_sd_w"), "sampler": d.get("sampler")})
        man["sources"].append({"what": "power_mde", "path": "hgamey/" + os.path.relpath(f, root), "sha256": sha256(f)})
    json.dump({"results": res, "mde_power": mde, "skipped_dry_runs": skipped},
              open(os.path.join(OUT, "effects.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def psnr(a, b):
    import array
    da = a.tobytes(); db = b.tobytes()
    n = len(da)
    if n == 0:
        return None
    se = 0
    for i in range(0, n):
        d = da[i] - db[i]
        se += d * d
    mse = se / n
    return 99.0 if mse == 0 else 10 * math.log10(255 * 255 / mse)


def crops(root, man):
    q1 = os.path.join(root, "_tmp/quality/Q1")
    diag_p = os.path.join(q1, "out/main/diagnose.json")
    if not os.path.exists(diag_p):
        return
    diag = json.load(open(diag_p, encoding="utf-8"))
    man["sources"].append({"what": "q1_diagnose", "path": "hgamey/_tmp/quality/Q1/out/main/diagnose.json", "sha256": sha256(diag_p)})
    # 位置：NPU int8 组每帧最差的 3 块（Q1 诊断），外扩成 CROP_W×CROP_H
    npu = next((g for g in diag["groups"] if any("npu_int8" in fr["cand"] for fr in g["frames"])), diag["groups"][0])
    W, H = diag["width"], diag["height"]
    spots = []
    for fr in npu["frames"]:
        for fx, fy, why in ((1000, 70, "固定位置：天空（平坦渐变）"), (1072, 470, "固定位置：画面中央（角色与场景）")):
            spots.append({"frame": fr["index"], "x": fx - CROP_W // 2, "y": fy - CROP_H // 2, "region": "", "why": why})
        for w in (fr.get("worst") or [])[:1]:
            cx = w.get("x", 0) + w.get("w", 32) // 2
            cy = w.get("y", 0) + w.get("h", 32) // 2
            if "x" not in w:  # 文件名里带坐标
                import re
                m = re.search(r"_x(\d+)_y(\d+)", w.get("crop", ""))
                if m:
                    cx, cy = int(m.group(1)) + 16, int(m.group(2)) + 16
            x0 = max(0, min(W - CROP_W, cx - CROP_W // 2))
            y0 = max(0, min(H - CROP_H, cy - CROP_H // 2))
            spots.append({"frame": fr["index"], "x": x0, "y": y0, "region": w.get("region", ""), "why": "NPU int8 最差块（Q1）"})
    cand_groups = [("npu_int8", "NPU int8（10-20 模型）"), ("bicubic", "双三次"), ("bilinear", "双线性")]
    os.makedirs(os.path.join(PRIV, "crops"), exist_ok=True)
    items = []
    for si, s in enumerate(spots):
        ref_p = os.path.join(q1, "frames", f"native_{s['frame']}.png")
        if not os.path.exists(ref_p):
            continue
        ref = Image.open(ref_p).convert("RGB")
        box = (s["x"], s["y"], s["x"] + CROP_W, s["y"] + CROP_H)
        rc = ref.crop(box)
        rname = f"s{si:02d}_native.png"
        rc.save(os.path.join(PRIV, "crops", rname))
        cands = []
        for g, gz in cand_groups:
            cp = os.path.join(q1, "frames", f"{g}_{s['frame']}.png")
            if not os.path.exists(cp):
                continue
            cc = Image.open(cp).convert("RGB").crop(box)
            diff = ImageChops.difference(rc, cc).point(lambda v: min(255, v * DIFF_GAIN))
            cn, dn = f"s{si:02d}_{g}.png", f"s{si:02d}_{g}_diff.png"
            cc.save(os.path.join(PRIV, "crops", cn))
            diff.save(os.path.join(PRIV, "crops", dn))
            cands.append({"group": g, "label": gz, "img": cn, "diff": dn, "psnr": round(psnr(rc, cc), 2)})
        items.append({**s, "native": rname, "cands": cands})
    # 原生自噪声参考：iq-judge 同一瞬间口径的参照线（排行榜里 psnr_med − m_psnr_med），以及原生两两稳定块 PSNR（image-quality 报告）
    ref_lines = {"same_instant_threshold_db": 49.0, "native_pairwise_stable_db_median": 51.5, "native_pairwise_stable_db_range": [48.7, 53.3],
                 "note": "参照线与原生自噪声来自 iq-judge 稳定块口径（全帧 32×32 块）；裁剪块 PSNR 只作直观对照，口径不同。"}
    json.dump({"crop_w": CROP_W, "crop_h": CROP_H, "diff_gain": DIFF_GAIN, "frames_src": "hgamey/_tmp/quality/Q1/frames（原神 ref_15–18，2144×976）",
               "ref": ref_lines, "items": items}, open(os.path.join(PRIV, "crops.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--hgamey", default=os.path.abspath(os.path.join(DECK, "../../../../")))
    a = ap.parse_args()
    root = a.hgamey
    if not os.path.isdir(os.path.join(root, "_tmp")):
        sys.exit(f"找不到 {root}/_tmp；用 --hgamey 指定 hgamey 根目录")
    os.makedirs(OUT, exist_ok=True)
    man = {"generated": datetime.datetime.now().astimezone().isoformat(timespec="seconds"), "sources": []}
    leaderboard(root, man)
    effects(root, man)
    crops(root, man)
    json.dump(man, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("data/ 已更新：", ", ".join(sorted(os.listdir(OUT))))


if __name__ == "__main__":
    main()
