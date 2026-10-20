# 改原神 globalPerfData 里某一项设置的档位 (游戏必须已停)
# 只做文本级替换, 其余字节原样保留 (重新序列化会改动浮点数的写法)
# 用法: python3 -I genshin_prefs.py <in.xml> <out.xml> <entryType>=<index> ...
#   entryType 2 = 渲染精度 (4=极高; 其余档位以 pass 表实测为准), 1 = 帧率
import re, sys, urllib.parse

src, dst = sys.argv[1], sys.argv[2]
s = open(src, encoding="utf-8").read()
m = re.search(r'name="GENERAL_DATA">([^<]*)<', s)
t = urllib.parse.unquote(m.group(1))
assert urllib.parse.quote(t, safe="") == m.group(1), "编码方式不是 quote(safe='')"
for a in sys.argv[3:]:
    et, idx = a.split("=")
    pat = r'(\\"entryType\\":%s,\\"index\\":)(\d+)' % et
    t, n = re.subn(pat, lambda mm: mm.group(1) + idx, t)
    if n != 1:
        sys.exit(f"entryType {et} 出现 {n} 次, 不改")
out = s[:m.start(1)] + urllib.parse.quote(t, safe="") + s[m.end(1):]
open(dst, "w", encoding="utf-8").write(out)
print(re.findall(r'entryType\\":(\d+),\\"index\\":(\d+)', t))
