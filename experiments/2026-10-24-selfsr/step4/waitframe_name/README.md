# 绕开红魔 SF waitframe 的原始记录 (2026-10-27/28)

各目录是一次 `selfsr_ab run` (每臂冷启动进大世界, 窗口 20 s, 屏幕 120 Hz, 红魔系统设置不动, 除非臂里写了 `sfwait=off`)。
`*_atrace.txt.gz` 是窗口后 3 s 的 SF atrace (`atrace -b 65536 -t 3 gfx view`)。

| 目录 | A 臂 | B 臂 | 看什么 |
|---|---|---|---|
| noreal_smoke | 原方案 | `sc_noreal=1` (游戏图层不送) | 子图层 89 → 80 |
| presig_smoke | `sc_presig=1` | `sc_noreal=1,sc_presig=1` | 85 / 61: 等子图层 fence 没用 |
| trace_smoke | 原方案 (作废: 窗口后屏幕回 60 Hz, trace 可用) | `sc_noreal=1` | A: commit 里 1 ms 轮询游戏 fence; B: flushTransactions 每帧固定空等 4.22 ms |
| trace_cmp | `sfwait=off` (root 对照) | `sc_noreal=1,sc_pacer=2` | A: 不等, 118 |
| name_smoke | `sc_noreal=1` + 改名 | 改名 (两臂都作废: 窗口后屏幕回 60 Hz) | B trace: 不再轮询 fence, 不跳帧 |
| name_ab | 原方案 | 改名 (全名) | 2 对: 89.8 → 117.3 |
