# 读 pw.log / arms.log, 算各臂相对相邻空闲段的功耗增量
# 电量计电流有十几秒平滑: 每段只取最后 TAIL 毫秒
# 用法: python3 -I analyze.py <目录> [TAIL_ms]
import re, sys, statistics as st

d = sys.argv[1]
TAIL = int(sys.argv[2]) if len(sys.argv) > 2 else 25000
samples = []
for ln in open(f"{d}/pw.log"):
    p = ln.split()
    if len(p) < 6:
        continue
    try:
        t, bi, bv = int(p[0]), int(p[3]), int(p[4])
    except ValueError:
        continue
    samples.append((t, abs(bi) / 1e6 * bv / 1e6))

arms = []
for ln in open(f"{d}/arms.log"):
    m = re.match(r"(BEGIN|END) (\S+) (\d+)", ln)
    if m and m.group(1) == "BEGIN":
        arms.append({"name": m.group(2), "t0": int(m.group(3))})
    elif m:
        arms[-1]["t1"] = int(m.group(3))
arms = [a for a in arms if "t1" in a]

def tail_power(a):
    xs = [w for t, w in samples if a["t1"] - TAIL <= t <= a["t1"]]
    return st.mean(xs) if xs else None

# 每个非空闲臂减去紧挨在它前面的那段空闲
res, idle = {}, []
for i, a in enumerate(arms):
    if a["name"] == "idle":
        idle.append(tail_power(a))
        continue
    prev = next((arms[j] for j in range(i - 1, -1, -1) if arms[j]["name"] == "idle"), None)
    if prev is None:
        continue
    res.setdefault(a["name"], []).append(tail_power(a) - tail_power(prev))

def ci(xs):
    t = {2: 12.71, 3: 4.30, 4: 3.18, 5: 2.78, 6: 2.57, 7: 2.45, 8: 2.36}.get(len(xs), 1.96)
    return t * st.stdev(xs) / len(xs) ** 0.5 if len(xs) > 1 else float("nan")

print(f"段尾 {TAIL/1000:.0f} s; 空闲 {st.mean(idle):.3f} W (n={len(idle)}, sd {st.stdev(idle):.3f})")
for k, v in res.items():
    print(f"{k:15s} 增量 {st.mean(v)*1000:6.0f} ± {ci(v)*1000:4.0f} mW (n={len(v)}) | 每帧 {st.mean(v)/30*1000:5.1f} mJ | 各轮 {', '.join(f'{x*1000:.0f}' for x in v)}")
