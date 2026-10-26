# stable 发布与部署边界

用户于2026-10-26确认：开发继续在main，每完成一批验收后推进stable。用户负责实际部署节点的自动更新脚本；开发者维护分支、配套版本和验收记录。分支推进不等于服务已更新或手机能力已验收。

## 推进流程

1. 一批代码、网页/CLI/API和相关行为验收完成；原始失败与未验范围保留。
2. 将同一提交推verify分支，必需code-only与适用的node-tests通过后，非强制推main。变基产生新SHA需重新检查。
3. phonefarm等待同SHA的node-build构建与发布成功。macOS/Android产物不能用来证明Linux宿主可运行。
4. 核对既有stable是目标SHA的祖先；三个仓库按一组精确SHA记录并非强制快进。任何并发变化先核对，不强制覆盖。三个Git ref不存在跨仓原子事务，部署端应冻结并验证完整配套清单，遇不一致保留旧版本。
5. 记录推送后远端实际ref与CI链接。远端运行程序、配置、数据目录、维护/恢复状态、服务重启和手机验收仍分别核对。

三个stable另设独立保护：禁删除、禁强推、无bypass、允许直接提交。GOL必须code-only，harness必须code-only及Python3.11/3.14和两个kernel job，phonefarm必须code-only与release检查。main原保护保留。规则回读已核，未引入PR要求。当前配套SHA见[stable-current.json](stable-current.json)。

## 首次基线

| 仓库 | stable SHA | 验证 |
|---|---|---|
| phonefarm | `3494bf2a97b6cbb1169362fc7565a42a3421025b` | code-only通过，node-build 40283045885成功 |
| harness-ops | `f5a7ea4d27cbb2f4882296e99fd9bd6a6f094673` | code-only 40293859867、node-tests 40293859441全部成功 |
| game_opt_loop | `36a46ad766408b00d8d133a495ca77d801b5e753` | 现有同SHA code-only通过 |

phonefarm从旧stable `537a62f`快进；另两仓补建，已逐一读回远端确认。基线只表示源码/隔离验证的可部署候选。原件为本机 `qa/resume-local-20261026/stable-release/baseline-promotion.json`。

## 精确构建产物的已发现缺口

当前node-build发布mutable build.json，SHA命名资产仅保留最近三次，git构建镜像仅保留八次。main继续前进后，旧stable可能无法从latest manifest取得匹配的摘要，最终资产也可能被清理。因此不能从mutable最新release猜stable产物，不能在摘要/版本不符时继续替换。本批正在修复按SHA发布manifest及当前stable保留规则，修复及云端实测完成前，此项保持待验。部署脚本应固定完整SHA、校验来源/平台/摘要，并遵守现有安全更新与维护屏障；不可通过直接覆盖正在运行的二进制跳过这些条件。

不新增独立stable重复构建去争用同一个mutable release；验收使用main上已经成功的同SHA产物。既有定时或生产更新流程本次未操作。

## 2026-10-26 完整提交组能量批

PF `d9c7ad09c437c71b62fdb661a90e2f72664b0258`（node-build40296587424成功）、harness `b42f798307e78d5d1e00e237ac494df27f439c0f`、GOL `4bbc7c3a76e431fa93470702edc47295823d5648`的同SHA必需检查成功后已全部快进stable。规则实际放行并逐仓读回确认；完整验收范围见[ENERGY-WINDOW](ENERGY-WINDOW.md)。本批不修改生产服务。
