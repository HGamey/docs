# 旧挂层执行入口退役

2026-10-23，PRODUCER-RECOVERY-03B4C2D3实施合同，PF f7b4012已检查成功直推main。只源码与无设备拦截验证；不部署/操作手机。此前reference/quality已迁到持续会话客户端，test_loadop已迁一个专属Session（PF4282d09，网页模板harness53bb414已发布）。

## 问题与决定

旧gray-layer的probe/loadop/passdump/frameaudit/frameaudit-ts/target在命令期间持锁，安装/修改全局属性后退出，没有完整持久owner与恢复义务。旧keep-prop在安装、force-stop和清旧输出后不启动；新gray-layer session始终安装并启动，keep-prop只是跳过marker等待/自动narrow。两者不能直接alias。当前PF/GOL/harness可执行源码没有剩余旧mount caller；refbench稀疏checkout另用git grep HEAD核对tracked app/tools/tests/selftest/contract，同样没有旧入口caller，不能以工作树目录枚举替代。历史证据中的命令保留。

本批直接退役旧六模式，在解析后、gate/Lease/ADB前结构化status=retired、exit2，明确replacement为显式serial/package/out/mode/max-seconds的已有session。绝不自动启动替代会话。copyprobe/upop继续原unsupported合同，不扩注入能力。新增专门legacy_mount_rejection供旧cli_admission和run_gray_layer共用；不能污染新control/install_control使用的mode_admission_rejection。

删除仅供旧mount_mode使用的安装/启动/setprops私有raw写链。保留共享adb_stdout、layer_so_path、constants/storeop规则与既有typed off/deactivate及status诊断、历史/离线分析。enable_layer.sh不再执行raw ADB：挂层直接结构化拒绝；status/off薄转已有Rust typed入口，保留显式REFBENCH_SERIAL默认与CLI覆盖，不再全盘find删除SO、删全局settings或清历史marker。该wrapper不兜底选择旧phonefarm.new/硬编码宿主路径。

后续若需要“仅安装等待，再typed launch”，可扩已有control会话阶段；不为兼容旧退出流程建第二owner/恢复框架，也不把未实现的等待能力写成可用。

## 验证和分工

platform独占src/graylayer.rs和knobs/gray/enable_layer.sh；root main帮助/AGENTS/docs/Cargo/Git；project只准备一组实际CLI和shell拦截（退休模式包括keep-prop/customSO等，零gate/ADB），冻结签名binary后一次执行。选相关纯admission、既有typed off合同测试，shared control/新refbench仍通过admission；不重跑控制/采样/恢复整链，不改已发布D2资源行为。CLI与Web新会话原语保留，旧命令的Web job也只返回同一明确拒绝和迁移说明。
