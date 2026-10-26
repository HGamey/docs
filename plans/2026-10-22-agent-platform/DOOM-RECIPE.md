# 复用已有 Doom 分发 recipe

DIST-03已有首个recipe，直接复用harness的[初始化清单](https://github.com/HGamey/harness-ops/blob/main/node/examples/doom3-demo-initialize-v1.json)与[场景清单](https://github.com/HGamey/harness-ops/blob/main/node/examples/doom3-demo-scene-v1.json)。两份约10KiB，分别覆盖v73应用安装/初始化/资源部署和固定render-demo场景。2026-10-26只读取小元数据并通过当前格式解析；未重新下载、复制或计算大资产摘要，未运行设备操作。

版本分别为`doom3-demo/v73-demo-1.1.1286-initialize1`和`doom3-demo/v73-demo-1.1.1286-scene1`；应用`com.karin.idTech4Amm`，versionCode11073。不要另造重复manifest或将旧实验数据包装成新部署。

## 使用顺序

1. 在选定节点登记两份现有清单，读取`games show`。Web游戏页与CLI使用同一登记和作业接口。
2. 缓存缺失时仅上传实际所需资产：APK、demo00.pk4、weapon_base.script、shot_pf_state_001.demo。两版本共享按摘要标识的缓存，不需要重复上传。旧save不参与这些typed操作；不要对含它的旧initialize版本盲目整体prepare。
3. 依据已有安装事实选择install/initialize/deploy及对应check，逐步读取原job报告。设备绑定可从同节点devices-resolve取得后显式传入，recipe不固定真实serial。
4. 从已committed、完成收尾且无恢复义务的资源作业取得`deployment.json`和`game-deploy/session.json`的input-reference；用两份回包的`.source`组成`{"report":...,"state":...}`，交给scene的`--deployment-inputs`。场景沿固定`+playDemo shot_pf_state_001`启动，不执行任意manifest命令。

每个新attempt使用新request ID；结果不明先查原ID。需要恢复时沿原deployment-recover，不用新安装或新launch绕过。缺素材可通过现有异步协助请求记录，只阻断依赖方向。完整CLI接续示例保留在本机qa/resume-local-20261026/doom-recipe/REUSE.md。

## 仍缺的实际材料

manifest中PK4、兼容脚本与真实render-demo种子的source仍为unavailable；本次没有检查选定节点缓存。APK保留原官方URL声明，未触发下载。历史设备路径/摘要不能当作现成可上传文件，合成QA种子也不能替代真实场景。素材供给、真机准备和冷启同状态场景仍需各自的新事实；本条只完成可复用recipe，不恢复Doom实测。
