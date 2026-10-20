cd /data/local/tmp/npu
export LD_LIBRARY_PATH=/data/local/tmp/npu ADSP_LIBRARY_PATH="/data/local/tmp/npu;/vendor/lib/rfsa/adsp;/vendor/dsp/cdsp"
Q="--external_delegate_path=/data/local/tmp/npu/libQnnTFLiteDelegate.so --external_delegate_options=backend_type:htp;library_path:/data/local/tmp/npu/libQnnHtp.so;skel_library_dir:/data/local/tmp/npu;htp_performance_mode:2"
Q16="--external_delegate_path=/data/local/tmp/npu/libQnnTFLiteDelegate.so --external_delegate_options=backend_type:htp;library_path:/data/local/tmp/npu/libQnnHtp.so;skel_library_dir:/data/local/tmp/npu;htp_performance_mode:2;htp_precision:1"
for m in s m l; do
  run1() { echo "$m $1 $(./bm --graph=$2 $3 --num_runs=50 --warmup_runs=5 2>&1 | grep -E 'Inference \(avg\)|Replacing' | sed -E 's/.*Replacing ([0-9]+ out of [0-9]+).*/[\1]/; s/.*Inference \(avg\): ([0-9.]+).*/avg_us=\1/' | tr '\n' ' ')"; }
  run1 npu_int8 espcn_${m}_int8.tflite "$Q"
  run1 npu_fp16 espcn_${m}_fp32.tflite "$Q16"
  run1 gpu_fp16 espcn_${m}_fp32.tflite "--use_gpu=true --gpu_precision_loss_allowed=true"
  run1 cpu4_int8 espcn_${m}_int8.tflite "--num_threads=4"
done
