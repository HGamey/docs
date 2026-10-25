# 上传更新沿节点配置目录保存凭据

2026-10-25，BASE-CONFIG-01，已发布 harness-ops `70b51ef0503e4d4dfe1daa51bd447d85561842b5`；未部署生产。

当前 dash、remote 和 launcher 支持 HG_CONFIG_DIR，但上传更新入口把 dash.pass/steer.pass 固定装进 ~/.config/hgamey。管理员隔离配置后，更新返回成功而服务仍使用另一处密码。此修复只闭合这个路径差异。

上传更新解析 HG_CONFIG_DIR；未设置/空值保持 ~/.config/hgamey，展开用户目录，相对值按 ~/node 固定为绝对路径，与 launcher 相同。只安装输入中实际存在的 dash.pass/steer.pass，目标权限600，新建配置目录700；不读取或输出正文，不顺便复制 node.token、远端节点 token、operator 登记或审计历史。成功安装后才删除本次上传凭据副本；写入失败保留可诊断输入，不称成功。

节点启动时保存选定的配置目录。固定 PF profile 会过滤普通环境，因此 dispatcher 仅给 `update_node` 补这一目录路径；不把服务 token 或整份宿主环境重新注入固定 profile。测试需覆盖经真实队列/worker 的这条调用链，不能只跑独立脚本就称平台接续完成。

更新器的代码/构建/数据目标仍是 ~/node；设置配置目录不是更改安装根，也不启用迁移。已有目录/凭据不自动发现、不删除、不复制。Web/CLI读取凭据的语义沿原实现，服务端密码更新是管理员操作，避免新增网页凭据编辑器。

验收使用临时 HOME、带空格的 HG_CONFIG_DIR 与只含两个合成密码的上传目录，执行真实上传分支：显式配置通过过滤环境的真实队列/worker，检查正确目标、权限、输入副本删除、默认旧文件保持；同一夹具检查省略配置和写入失败。无网络、构建、设备、HTTP服务或真实凭据；因修改dispatcher，本批按仓库规则运行一次完整协议测试，不另跑重复专项或浏览器链。

结果：唯一完整本地协议150项首轮通过（90.811秒）；新增一方法经真实Jobs/worker和原zsh上传脚本完成选定目录验证，默认与目标普通文件失败分支同夹具通过，worker自然结束并收尾。五项必需CI全部success后同SHA推main并核对。证据：`qa/node-config-{protocol-1.log,validation.json,check-runs.json,publish.json}`。语法检查通过，未改真实凭据、没有生产部署；更新器仍固定原安装布局。
