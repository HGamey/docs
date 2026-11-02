#!/bin/bash
# 从 hgamey/_tmp 下各任务目录整理数据到 data/（排行榜、真机效应量、画面对比裁剪）。
# 之后重新构建：bash build.sh
set -euo pipefail
cd "$(dirname "$0")"
python3 -I scripts/refresh_data.py "$@"
