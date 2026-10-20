// GPU <-> NPU 每帧交接代价测量
// 每帧: GPU 生成一帧 1072x488 RGBA8 -> 拷到主机可见缓冲 -> CPU 打包成 RGB 送 NPU 超分
//       -> CPU 解包成 2144x976 RGBA8 -> 拷回 GPU 图像
// 用法: handoff <model.tflite> <帧数> <帧率, 0=不限速> <npu|none> [zc]
//   none: 只做 GPU 读回和写回, 不跑 NPU, 用来拆分代价
//   zc:   NPU 输入输出放在 QNN 共享内存 (rpcmem), 省掉代理内部的拷贝
#include <vulkan/vulkan.h>
#include <tensorflow/lite/c/c_api.h>
#include <tensorflow/lite/c/c_api_experimental.h>
#include "QnnTFLiteDelegate.h"
#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <thread>
#include <vector>
#include <unistd.h>

static const uint32_t IW = 1072, IH = 488, OW = 2144, OH = 976;

#define VK(x) do { VkResult r_ = (x); if (r_ != VK_SUCCESS) { fprintf(stderr, "%s = %d\n", #x, r_); exit(1); } } while (0)

static double now_ms() {
    return std::chrono::duration<double, std::milli>(std::chrono::steady_clock::now().time_since_epoch()).count();
}

struct Vk {
    VkInstance inst; VkPhysicalDevice pd; VkDevice dev; VkQueue q; uint32_t qf = 0;
    VkCommandPool pool; VkFence fence;
    VkPhysicalDeviceMemoryProperties mp;

    uint32_t memtype(uint32_t bits, VkMemoryPropertyFlags want, VkMemoryPropertyFlags nice = 0) {
        for (int pass = 0; pass < 2; pass++)
            for (uint32_t i = 0; i < mp.memoryTypeCount; i++) {
                VkMemoryPropertyFlags f = mp.memoryTypes[i].propertyFlags;
                if ((bits & (1u << i)) && (f & want) == want && (pass == 1 || (f & nice) == nice)) return i;
            }
        fprintf(stderr, "no memtype\n"); exit(1);
    }
    void init() {
        VkApplicationInfo ai{VK_STRUCTURE_TYPE_APPLICATION_INFO};
        ai.apiVersion = VK_API_VERSION_1_1;
        VkInstanceCreateInfo ic{VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO};
        ic.pApplicationInfo = &ai;
        VK(vkCreateInstance(&ic, nullptr, &inst));
        uint32_t n = 1;
        vkEnumeratePhysicalDevices(inst, &n, &pd);
        vkGetPhysicalDeviceMemoryProperties(pd, &mp);
        uint32_t nq = 0;
        vkGetPhysicalDeviceQueueFamilyProperties(pd, &nq, nullptr);
        std::vector<VkQueueFamilyProperties> qp(nq);
        vkGetPhysicalDeviceQueueFamilyProperties(pd, &nq, qp.data());
        for (uint32_t i = 0; i < nq; i++) if (qp[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) { qf = i; break; }
        float pr = 1.f;
        VkDeviceQueueCreateInfo qc{VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO};
        qc.queueFamilyIndex = qf; qc.queueCount = 1; qc.pQueuePriorities = &pr;
        VkDeviceCreateInfo dc{VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO};
        dc.queueCreateInfoCount = 1; dc.pQueueCreateInfos = &qc;
        VK(vkCreateDevice(pd, &dc, nullptr, &dev));
        vkGetDeviceQueue(dev, qf, 0, &q);
        VkCommandPoolCreateInfo pc{VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO};
        pc.queueFamilyIndex = qf; pc.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
        VK(vkCreateCommandPool(dev, &pc, nullptr, &pool));
        VkFenceCreateInfo fc{VK_STRUCTURE_TYPE_FENCE_CREATE_INFO};
        VK(vkCreateFence(dev, &fc, nullptr, &fence));
    }
    VkImage image(uint32_t w, uint32_t h, VkDeviceMemory* m) {
        VkImageCreateInfo ic{VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO};
        ic.imageType = VK_IMAGE_TYPE_2D; ic.format = VK_FORMAT_R8G8B8A8_UNORM;
        ic.extent = {w, h, 1}; ic.mipLevels = 1; ic.arrayLayers = 1; ic.samples = VK_SAMPLE_COUNT_1_BIT;
        ic.tiling = VK_IMAGE_TILING_OPTIMAL;
        ic.usage = VK_IMAGE_USAGE_TRANSFER_SRC_BIT | VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
        VkImage img;
        VK(vkCreateImage(dev, &ic, nullptr, &img));
        VkMemoryRequirements r;
        vkGetImageMemoryRequirements(dev, img, &r);
        VkMemoryAllocateInfo a{VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO};
        a.allocationSize = r.size; a.memoryTypeIndex = memtype(r.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
        VK(vkAllocateMemory(dev, &a, nullptr, m));
        VK(vkBindImageMemory(dev, img, *m, 0));
        return img;
    }
    VkBuffer buffer(VkDeviceSize size, VkMemoryPropertyFlags nice, void** map) {
        VkBufferCreateInfo bc{VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO};
        bc.size = size; bc.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        VkBuffer b;
        VK(vkCreateBuffer(dev, &bc, nullptr, &b));
        VkMemoryRequirements r;
        vkGetBufferMemoryRequirements(dev, b, &r);
        VkMemoryAllocateInfo a{VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO};
        a.allocationSize = r.size;
        a.memoryTypeIndex = memtype(r.memoryTypeBits, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, nice);
        VkDeviceMemory m;
        VK(vkAllocateMemory(dev, &a, nullptr, &m));
        VK(vkBindBufferMemory(dev, b, m, 0));
        VK(vkMapMemory(dev, m, 0, size, 0, map));
        return b;
    }
    void submit_wait(VkCommandBuffer cb) {
        VkSubmitInfo si{VK_STRUCTURE_TYPE_SUBMIT_INFO};
        si.commandBufferCount = 1; si.pCommandBuffers = &cb;
        VK(vkQueueSubmit(q, 1, &si, fence));
        VK(vkWaitForFences(dev, 1, &fence, VK_TRUE, UINT64_MAX));
        VK(vkResetFences(dev, 1, &fence));
    }
};

static void barrier(VkCommandBuffer cb, VkImage img, VkImageLayout from, VkImageLayout to) {
    VkImageMemoryBarrier b{VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER};
    b.oldLayout = from; b.newLayout = to;
    b.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT | VK_ACCESS_TRANSFER_READ_BIT;
    b.dstAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT | VK_ACCESS_TRANSFER_READ_BIT | VK_ACCESS_HOST_READ_BIT;
    b.srcQueueFamilyIndex = b.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    b.image = img; b.subresourceRange = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1};
    vkCmdPipelineBarrier(cb, VK_PIPELINE_STAGE_ALL_COMMANDS_BIT, VK_PIPELINE_STAGE_ALL_COMMANDS_BIT | VK_PIPELINE_STAGE_HOST_BIT,
                         0, 0, nullptr, 0, nullptr, 1, &b);
}

struct Stat {
    std::vector<double> v;
    void add(double x) { v.push_back(x); }
    void print(const char* name) {
        if (v.empty()) return;
        std::vector<double> s = v;
        std::sort(s.begin(), s.end());
        double sum = 0;
        for (double x : s) sum += x;
        printf("%-10s mean %7.3f  p50 %7.3f  p95 %7.3f ms\n", name, sum / s.size(), s[s.size() / 2], s[s.size() * 95 / 100]);
    }
};

int main(int argc, char** argv) {
    if (argc < 5) { fprintf(stderr, "usage: handoff model frames hz npu|none [zc]\n"); return 2; }
    const char* model_path = argv[1];
    int frames = atoi(argv[2]);
    double hz = atof(argv[3]);
    bool use_npu = std::string(argv[4]) == "npu";
    bool zc = argc > 5 && std::string(argv[5]) == "zc";
    std::string dir = getenv("NPU_DIR") ? getenv("NPU_DIR") : "/data/local/tmp/npu";

    Vk vk;
    vk.init();
    VkDeviceMemory m1, m2;
    VkImage src = vk.image(IW, IH, &m1), dst = vk.image(OW, OH, &m2);
    void *rb_map, *up_map;
    VkBuffer rb = vk.buffer((VkDeviceSize)IW * IH * 4, VK_MEMORY_PROPERTY_HOST_CACHED_BIT, &rb_map);
    VkBuffer up = vk.buffer((VkDeviceSize)OW * OH * 4, 0, &up_map);
    VkCommandBuffer cb;
    VkCommandBufferAllocateInfo ca{VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO};
    ca.commandPool = vk.pool; ca.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY; ca.commandBufferCount = 1;
    VK(vkAllocateCommandBuffers(vk.dev, &ca, &cb));
    VkCommandBufferBeginInfo bi{VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO};
    bi.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;

    TfLiteInterpreter* interp = nullptr;
    TfLiteDelegate* del = nullptr;
    uint8_t *in = nullptr, *out = nullptr;
    std::vector<uint8_t> in_host((size_t)IW * IH * 3), out_host((size_t)OW * OH * 3);
    if (use_npu) {
        TfLiteModel* model = TfLiteModelCreateFromFile(model_path);
        if (!model) { fprintf(stderr, "model load failed\n"); return 1; }
        TfLiteQnnDelegateOptions o = TfLiteQnnDelegateOptionsDefault();
        std::string lib = dir + "/libQnnHtp.so";
        o.backend_type = kHtpBackend;
        o.library_path = lib.c_str();
        o.skel_library_dir = dir.c_str();
        // NPU_PERF: HTP 性能档位, 缺省 2 (burst); 8=balanced 4=power_saver
        o.htp_options.performance_mode = (TfLiteQnnDelegateHtpPerformanceMode)(getenv("NPU_PERF") ? atoi(getenv("NPU_PERF")) : kHtpBurst);
        del = TfLiteQnnDelegateCreate(&o);
        if (!del) { fprintf(stderr, "delegate create failed\n"); return 1; }
        TfLiteInterpreterOptions* io = TfLiteInterpreterOptionsCreate();
        TfLiteInterpreterOptionsAddDelegate(io, del);
        interp = TfLiteInterpreterCreate(model, io);
        if (!interp) { fprintf(stderr, "interpreter create failed\n"); return 1; }
        TfLiteTensor* ti = TfLiteInterpreterGetInputTensor(interp, 0);
        const TfLiteTensor* to = TfLiteInterpreterGetOutputTensor(interp, 0);
        if (TfLiteTensorByteSize(ti) != in_host.size() || TfLiteTensorByteSize(to) != out_host.size()) {
            fprintf(stderr, "tensor size mismatch %zu %zu\n", TfLiteTensorByteSize(ti), TfLiteTensorByteSize(to));
            return 1;
        }
        if (zc) {
            in = (uint8_t*)TfLiteQnnDelegateAllocCustomMem(in_host.size(), 64);
            out = (uint8_t*)TfLiteQnnDelegateAllocCustomMem(out_host.size(), 64);
            if (!in || !out) { fprintf(stderr, "rpcmem alloc failed\n"); return 1; }
            TfLiteCustomAllocation ai{in, in_host.size()}, ao{out, out_host.size()};
            int oi = TfLiteInterpreterGetOutputTensorIndex(interp, 0);
            int ii = TfLiteInterpreterGetInputTensorIndex(interp, 0);
            if (TfLiteInterpreterSetCustomAllocationForTensor(interp, ii, &ai, kTfLiteCustomAllocationFlagsNone) != kTfLiteOk ||
                TfLiteInterpreterSetCustomAllocationForTensor(interp, oi, &ao, kTfLiteCustomAllocationFlagsNone) != kTfLiteOk) {
                fprintf(stderr, "custom allocation failed\n"); return 1;
            }
        }
        if (TfLiteInterpreterAllocateTensors(interp) != kTfLiteOk) { fprintf(stderr, "allocate failed\n"); return 1; }
        if (!zc) {
            in = (uint8_t*)TfLiteTensorData(TfLiteInterpreterGetInputTensor(interp, 0));
            out = (uint8_t*)TfLiteTensorData(TfLiteInterpreterGetOutputTensor(interp, 0));
        }
    } else {
        in = in_host.data();
        out = out_host.data();
    }

    // 两张图先转成 TRANSFER 布局
    VK(vkBeginCommandBuffer(cb, &bi));
    barrier(cb, src, VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL);
    barrier(cb, dst, VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL);
    VK(vkEndCommandBuffer(cb));
    vk.submit_wait(cb);

    Stat s_rd, s_pack, s_npu, s_unpack, s_wr, s_total;
    double period = hz > 0 ? 1000.0 / hz : 0, next = now_ms();
    int warm = 10;
    for (int f = 0; f < frames + warm; f++) {
        double t0 = now_ms();
        // 1. GPU: 生成一帧并读回
        VK(vkBeginCommandBuffer(cb, &bi));
        VkClearColorValue c{};
        c.float32[0] = (f % 256) / 255.f; c.float32[1] = 0.5f; c.float32[2] = 0.25f; c.float32[3] = 1.f;
        VkImageSubresourceRange rr{VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1};
        vkCmdClearColorImage(cb, src, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, &c, 1, &rr);
        barrier(cb, src, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL);
        VkBufferImageCopy bic{};
        bic.imageSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 0, 1};
        bic.imageExtent = {IW, IH, 1};
        vkCmdCopyImageToBuffer(cb, src, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, rb, 1, &bic);
        barrier(cb, src, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL);
        VK(vkEndCommandBuffer(cb));
        vk.submit_wait(cb);
        double t1 = now_ms();
        // 2. CPU: RGBA8 -> RGB
        const uint8_t* p = (const uint8_t*)rb_map;
        for (size_t i = 0, n = (size_t)IW * IH; i < n; i++) { in[i * 3] = p[i * 4]; in[i * 3 + 1] = p[i * 4 + 1]; in[i * 3 + 2] = p[i * 4 + 2]; }
        double t2 = now_ms();
        // 3. NPU 超分
        if (use_npu && TfLiteInterpreterInvoke(interp) != kTfLiteOk) { fprintf(stderr, "invoke failed\n"); return 1; }
        double t3 = now_ms();
        // 4. CPU: RGB -> RGBA8
        uint8_t* u = (uint8_t*)up_map;
        for (size_t i = 0, n = (size_t)OW * OH; i < n; i++) { u[i * 4] = out[i * 3]; u[i * 4 + 1] = out[i * 3 + 1]; u[i * 4 + 2] = out[i * 3 + 2]; u[i * 4 + 3] = 255; }
        double t4 = now_ms();
        // 5. GPU: 写回图像
        VK(vkBeginCommandBuffer(cb, &bi));
        VkBufferImageCopy bo{};
        bo.imageSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 0, 1};
        bo.imageExtent = {OW, OH, 1};
        vkCmdCopyBufferToImage(cb, up, dst, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &bo);
        VK(vkEndCommandBuffer(cb));
        vk.submit_wait(cb);
        double t5 = now_ms();
        if (f >= warm) {
            s_rd.add(t1 - t0); s_pack.add(t2 - t1); s_npu.add(t3 - t2); s_unpack.add(t4 - t3); s_wr.add(t5 - t4); s_total.add(t5 - t0);
        }
        if (period > 0) {
            next += period;
            double w = next - now_ms();
            if (w > 0) std::this_thread::sleep_for(std::chrono::microseconds((long)(w * 1000)));
            else next = now_ms();
        }
    }
    printf("mode %s%s frames %d hz %.0f\n", use_npu ? "npu" : "none", zc ? " zc" : "", frames, hz);
    s_rd.print("gpu_read");
    s_pack.print("cpu_pack");
    s_npu.print("npu");
    s_unpack.print("cpu_unpack");
    s_wr.print("gpu_write");
    s_total.print("total");
    // QNN 代理与解释器的析构顺序会在退出时 abort, 测量已完成, 直接退出
    fflush(stdout);
    _exit(0);
}
