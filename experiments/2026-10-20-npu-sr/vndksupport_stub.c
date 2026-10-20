// libvndksupport 替身: 原版依赖链接器内部的 ld-android.so, 普通应用加载不了。
// 只在研究机上把 NPU 运行库塞进游戏目录时使用。
#include <dlfcn.h>
int android_is_in_vendor_process(void) { return 0; }
void* android_load_sphal_library(const char* name, int flag) { return dlopen(name, flag); }
int android_unload_sphal_library(void* handle) { return dlclose(handle); }
