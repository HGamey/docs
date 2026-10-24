# BASE-RUNTIME-01：当前程序与作业实际入口身份

2026-10-25实施合同；BASE-04/JOB-04首批，父项的完整设备/依赖身份和兼容矩阵仍开放。源码、隔离宿主、CI验收，不升级运行节点或访问设备。身份用于诊断和结果追溯，不是恶意程序认证、lease或恢复证明。

## 程序声明

PF/GOL增加纯宿主`runtime-info --json`，返回`schema:"phonefarm/runtime-identity@1"`、`program:"phonefarm"|"game_opt_loop"`、`package_version`、`build:{source_revision,source_dirty,target,profile}`、`supported_protocols`。来源由构建入口显式传`HGAMEY_BUILD_REVISION/HGAMEY_BUILD_DIRTY`，缺失为null；build.rs只读这些环境及Cargo target/profile并声明rerun，不运行git。协议列表只列实际支持的身份/自报合同，不冒充全面设备能力。

两个main在gate/Clap/业务之前可选自报。消费并移除`PF_RUNTIME_REPORT`环境变量，绑定已有`PF_JOB_ID`；输出`schema:"phonefarm/program-start@1",job_id,pid,identity`。在已存在的out目录内以临时文件加原子首次发布写到`out/.program-runtime.json`；不覆盖、无stdout，写失败不改变业务结果。清除路径环境后子PF/GOL不会继承自报目的地。手工runtime-info不会读设备、运行数据或凭据。

PF CI与update_node本地编译传选定源码声明。下载的build.json作为下载资产出处保留在安装目录，不能把其签名前SHA称为签名后运行文件SHA。harness安装生成`runtime-build.json`（`schema:"hgamey/source-build@1",repository,revision,dirty`），固定worker复制自己的安装声明；缺失保持null。不每次hash、不推断所有Python已载入字节和源码一致。

## Node读取与刷新

原service.revision/dirty保留但明确为启动时源码树观察，增加`source_kind:"startup_checkout_observation"`、`instance_id`、`observed_at`、`runtime:{entry_path,python_version,observed_at,build}`。实例启动一次生成，持久node_id不变。

`execution.programs`为按已配置phonefarm/game_opt_loop名称的对象；每项`state:"observed"|"stale"|"unknown"`、`resolved_path`、`observed_at`、`file:{dev,ino,size,mtime_ns,ctime_ns}|null`、`identity|null`、`reason|null`。启动时只查询这两个已配置程序的runtime-info，时间/输出有界，查询前后stat不同则不绑定。普通GET只看缓存并核廉价stat，改变后stale，旧声明只作历史观察，不宣称当前有效；不得执行上传exe或自动重试。旧程序不认识入口为unknown，不按git填身份。

有权限用户可显式`POST /node/runtime/refresh`重新观察，返回/node同形；`pf-node node --refresh-runtime`和网页按钮同级。只读token不得触发执行；刷新不重启服务、不创建job、不操作设备、不改历史。远端目录里的node数据仍受目录快照时间约束。

## 作业冻结

不改用户request的canonical/幂等内容。实际启动的launch.json与meta.execution记录`service_at_launch:{node_id,service}|null`。独立worker记录自己的`worker_identity:{entry_path,python_version,observed_at,build}`，不能使用今天/node的服务版本替代。未启动旧任务或直接管理员调用缺服务快照保持null。

普通worker与管理员contained后端都只在实际Popen的直接child启动后读自报；固定gate exec保PID，按job_id及实际child_pid核。输出目录在contained交还前不由Node预写。统一`execution.program_identity:{state:"pending"|"reported"|"unknown",source:"program_self_report",record:<原记录>|null,reason:null|string}`；有效首次记录冻结，缺失/不符/坏记录为unknown，不探测或重跑该程序，不改变outcome/资源恢复。运行时未出记录可pending，终态没有记录则unknown。旧job缺字段在客户端显示未知，不改写历史、不从当前配置回填。主程序PF gate父进程先报，依赖工具/层/设备身份另列。

## 最小验证与展示

网页节点/任务与CLI同读上述记录，显式区分启动源码观察、配置缓存、该job当时自报；没有身份不说失败，没有新版本不说不兼容。使用一个集中HTTP/CLI链：A自报、原路径换B缓存stale、旧job仍A、新job为B、无自报仍成功、旧meta不回填、显式刷新不产生job。一个实际PF/GOL只读入口及启动自报验证，不跑设备或旧producer全套。contained复用已有Linux CI无害工作负载验证，public contained仍不可用。协议完整本地一次，失败只重验受影响链；网页冻结后单390流程。
