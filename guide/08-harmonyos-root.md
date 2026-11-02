# HarmonyOS 设备的 root 与可采集能力

## 适用范围

本文说明 HarmonyOS（OpenHarmony）设备经 hdc 接入平台时，root 权限的获取方式、各项测量数据的来源，以及平台目前的支持情况。内容依据一台已 root 的 HarmonyOS 7.0 设备（OpenHarmony 7.0.0.120，Maleoon GPU）的只读检查整理；未经实测的结论均已注明。

## 获取 root 的方式

平台判定 root 的依据是 `hdc shell id` 直接返回 `uid=0`。设备层不经 `su` 提权，因为遥测脚本通过 `hdc shell sh` 执行。

| 方式 | 条件 | 平台支持 | 说明 |
|---|---|---|---|
| hdc shell 直接为 root | hdcd 以 root 运行且不降权 | 已支持，自动识别 | 判定见下方命令 |
| `hdc smode` | `const.debuggable=1` 的调试版系统 | 未接入 | 执行后 hdcd 重启，此后 hdc shell 为 root；`hdc smode -r` 恢复。依据 OpenHarmony hdc 的实现推断，尚未经 USB 连接实测 |
| `su` | 系统提供 su 程序 | 不支持 | 所检查的设备未提供 su |

判定命令：

```sh
hdc shell id                         # 含 uid=0 即为 root
hdc shell param get const.debuggable # 1 表示调试版，可使用 hdc smode
hdc shell param get const.hdc.secure # 0 表示 hdc 无需认证
hdc shell param get persist.hdc.root # 1 表示已处于 smode
```

所检查设备的状态为：hdcd 以 root 身份运行（init 配置中 `uid` 为 root，并允许 setuid），`const.debuggable=1`，`const.hdc.secure=0`，未设置 `persist.hdc.root`，系统中无 su 程序。

## 可采集能力

| 项目 | 来源 | 是否需要 root | 平台采集情况 |
|---|---|---|---|
| GPU 当前频率 | `/sys/class/devfreq/gpufreq/cur_freq`（Hz） | 否 | 已采集，字段 `gpu_freq_mhz` |
| GPU 利用率 | `/sys/devices/platform/*/gpu_utilisation`（「Gpu utilisation : N」） | 否 | 已采集，字段 `gpu_util_pct` |
| GPU 频点与调度策略 | `gpufreq/available_frequencies`、`governor`、`trans_stat` | 否（读取） | 未采集 |
| 内存频率 | `/sys/class/devfreq/ddrfreq*/cur_freq` | 否 | 未采集 |
| 温度 | `/sys/class/thermal/thermal_zone*/type`、`temp` | 否 | 已采集 |
| 整机功耗 | thermal zone `Total_power` | 否 | 原始值随温度一并采集；单位未确认 |
| 电池电流、电压、温度 | `hidumper -s BatteryService`；`/sys/class/power_supply/Battery/` | 否 | 已采集（hidumper） |
| 帧率 | `hidumper -s RenderService -a fpsCount` | 否 | 已采集 |
| 屏幕分辨率与刷新率模式 | `hidumper -s RenderService -a screen` | 否 | 未采集 |
| 帧率、GPU、功耗（系统工具） | SmartPerf 命令行 `SP_daemon`（`-f`、`-g`、`-p`、`-t`） | 否 | 未接入，可作为交叉核对 |
| 内核 trace | `/sys/kernel/tracing`，含 `gpu_scene_aware`、`hvgr` 事件 | 是 | 未接入 |
| 系统 trace 与性能剖析 | `hitrace`、`hiperf`、`hiprofiler_cmd` | 部分需要 | 未接入 |
| 进程 I/O、文件句柄、内存明细、内核日志 | `/proc/<pid>/io`、`fd`、`smaps_rollup`，`dmesg` | 是 | 已采集（仅 root 时） |

## Vulkan 层注入点

HarmonyOS 的 Vulkan 加载器支持隐式层，清单文件位于 `/system/etc/vulkan/implicit_layer.d/`。所检查设备上已有系统自带的隐式层，例如：

```json
{
    "file_format_version" : "1.0.0",
    "layer" : {
        "name": "VK_LAYER_HUAWEI_GameAware",
        "type": "GLOBAL",
        "library_path": "libGameAwareLayer.so",
        "api_version": "1.3.231",
        "implementation_version": "1",
        "description": "game aware core layer",
        "disable_environment": {
            "DISABLE_HUAWEI_GAMEAWARE_LAYER": "1"
        }
    }
}
```

因此，自研超分辨率与插帧层在 HarmonyOS 上可沿用同一机制：放入清单与层库，并以 `enable_environment` 或 `disable_environment` 控制作用范围。写入 `/system` 需要 root 且分区可写；层本身还需针对 HarmonyOS 的图形栈重新编译与适配。以上尚未实测。

## 待验证事项

1. 经 USB 连接时 `hdc smode` 的实际行为，以及恢复后 hdc shell 的权限。
2. 隐式层在游戏进程中的加载情况与作用范围控制。
3. `Total_power` 的单位与刷新周期，以及与电池放电功率的对应关系。
4. 平台在 HarmonyOS root 设备上的设备锁、任务结束后的状态恢复，以及需要 root 的测量项（内核 trace 等）的接入。
