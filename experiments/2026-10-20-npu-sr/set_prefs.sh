#!/bin/bash
# set_prefs.sh <prefs.xml> — 原神停着时整文件写回 (cat 覆盖, 保留属主与 SELinux 标签)
set -euo pipefail
S="${SERIAL:-91253241019A}"; P=/data/data/com.miHoYo.Yuanshen/shared_prefs/com.miHoYo.Yuanshen.v2.playerprefs.xml
adb -s "$S" shell "su -c 'am force-stop com.miHoYo.Yuanshen'" </dev/null
adb -s "$S" push -q "$1" /data/local/tmp/gs_prefs.xml </dev/null
adb -s "$S" shell "su -c 'cat /data/local/tmp/gs_prefs.xml > $P && ls -lZ $P'" </dev/null
