#!/system/bin/sh
# NPU 性能档位对功耗的影响 (第二组): 停充 (满电时本机直接转电池放电, 量电池轨 V x I), 灭屏, 每段 60 秒, 4 轮
# 输出: $R/pw.log (毫秒 USB微伏 USB微安 电池微安 电池微伏 臂名), $R/arms.log
R=/data/local/tmp/npu
CS=/data/local/tmp/npu/charge_suspend.sh
cd $R
export LD_LIBRARY_PATH=$R ADSP_LIBRARY_PATH="$R;/vendor/lib/rfsa/adsp;/vendor/dsp/cdsp"
qd() { echo "--external_delegate_path=$R/libQnnTFLiteDelegate.so --external_delegate_options=backend_type:htp;library_path:$R/libQnnHtp.so;skel_library_dir:$R;htp_performance_mode:$1"; }
SEG=60       # 电量计电流有十几秒平滑, 每段 60 秒, 分析只取后 25 秒
ROUNDS=${ROUNDS:-3}
: > pw.log; : > arms.log
echo idle > arm.cur

trap 'sh $CS restore >> arms.log 2>&1; rm -f arm.cur' EXIT
trap 'exit 1' INT TERM
sh $CS suspend >> arms.log 2>&1 || { echo "suspend failed" >> arms.log; exit 1; }
input keyevent SLEEP

( while [ -f arm.cur ]; do
    echo "$(date +%s%3N) $(cat /sys/class/power_supply/usb/voltage_now) $(cat /sys/class/power_supply/usb/current_now) $(cat /sys/class/power_supply/battery/current_now) $(cat /sys/class/power_supply/battery/voltage_now) $(cat arm.cur)" >> pw.log
    sleep 0.2
  done ) &
SAMPLER=$!

arm() {   # $1 名字, 其余是 bm 参数; idle 时只等
  name=$1; shift
  echo "$name" > arm.cur
  echo "BEGIN $name $(date +%s%3N)" >> arms.log
  if [ "$name" = idle ]; then sleep $SEG
  elif [ "$name" = handoff_s_30hz ]; then ./handoff espcn_s_int8.tflite $((SEG * 30 - 60)) 30 npu 2>/dev/null >> arms.log
  else ./bm --num_runs=1000000 --max_secs=$SEG --warmup_runs=1 --warmup_min_secs=0 "$@" 2>&1 | grep -E "count=|Inference \(avg\)|Replacing" >> arms.log
  fi
  echo "END $name $(date +%s%3N)" >> arms.log
  echo idle > arm.cur
  sleep 4
}

sleep 30   # 停充后让电量计和温度稳定
i=0
while [ $i -lt $ROUNDS ]; do
  # 档位 8=balanced 4=power_saver 9=extreme_power_saver, 对照 GPU
  arm idle
  arm npu_s_m8_30hz --graph=espcn_s_int8.tflite --run_frequency=30 $(qd 8)
  arm idle
  arm npu_s_m4_30hz --graph=espcn_s_int8.tflite --run_frequency=30 $(qd 4)
  arm idle
  arm npu_s_m9_30hz --graph=espcn_s_int8.tflite --run_frequency=30 $(qd 9)
  arm idle
  arm gpu_s_30hz --graph=espcn_s_fp32.tflite --run_frequency=30 --use_gpu=true --gpu_precision_loss_allowed=true
  i=$((i+1))
done
rm arm.cur
wait $SAMPLER
echo DONE >> arms.log
