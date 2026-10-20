// 在 NPU 上跑一次模型: 读 uint8 输入文件, 写 uint8 输出文件, 用来核对 NPU 输出画质
// 用法: npu_run <model.tflite> <in.u8> <out.u8> [性能档位, 缺省 8=balanced]
#include <tensorflow/lite/c/c_api.h>
#include "QnnTFLiteDelegate.h"
#include <cstdio>
#include <cstdlib>
#include <string>
#include <vector>
#include <unistd.h>

int main(int argc, char** argv) {
    if (argc < 4) { fprintf(stderr, "usage: npu_run model in.u8 out.u8 [perf]\n"); return 2; }
    std::string dir = getenv("NPU_DIR") ? getenv("NPU_DIR") : "/data/local/tmp/npu";
    std::string lib = dir + "/libQnnHtp.so";
    TfLiteModel* model = TfLiteModelCreateFromFile(argv[1]);
    if (!model) { fprintf(stderr, "model load failed\n"); return 1; }
    TfLiteQnnDelegateOptions o = TfLiteQnnDelegateOptionsDefault();
    o.backend_type = kHtpBackend;
    o.library_path = lib.c_str();
    o.skel_library_dir = dir.c_str();
    o.htp_options.performance_mode = (TfLiteQnnDelegateHtpPerformanceMode)(argc > 4 ? atoi(argv[4]) : kHtpBalanced);
    TfLiteDelegate* del = TfLiteQnnDelegateCreate(&o);
    if (!del) { fprintf(stderr, "delegate create failed\n"); return 1; }
    TfLiteInterpreterOptions* io = TfLiteInterpreterOptionsCreate();
    TfLiteInterpreterOptionsAddDelegate(io, del);
    TfLiteInterpreter* it = TfLiteInterpreterCreate(model, io);
    if (!it || TfLiteInterpreterAllocateTensors(it) != kTfLiteOk) { fprintf(stderr, "interpreter failed\n"); return 1; }
    TfLiteTensor* ti = TfLiteInterpreterGetInputTensor(it, 0);
    const TfLiteTensor* to = TfLiteInterpreterGetOutputTensor(it, 0);
    std::vector<uint8_t> in(TfLiteTensorByteSize(ti)), out(TfLiteTensorByteSize(to));
    FILE* f = fopen(argv[2], "rb");
    if (!f || fread(in.data(), 1, in.size(), f) != in.size()) { fprintf(stderr, "input read failed\n"); return 1; }
    fclose(f);
    TfLiteTensorCopyFromBuffer(ti, in.data(), in.size());
    if (TfLiteInterpreterInvoke(it) != kTfLiteOk) { fprintf(stderr, "invoke failed\n"); return 1; }
    TfLiteTensorCopyToBuffer(to, out.data(), out.size());
    f = fopen(argv[3], "wb");
    fwrite(out.data(), 1, out.size(), f);
    fclose(f);
    printf("ok %zu -> %zu bytes\n", in.size(), out.size());
    fflush(stdout);
    _exit(0);   // 同 handoff: 退出时析构会 abort
}
