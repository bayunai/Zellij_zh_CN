# 开发文档

本文面向想修改源码、自己构建 wasm、或打包发布版的开发者。普通用户请看 [README.md](README.md)。

## 源码构建

先安装 WASM 编译目标，再构建插件：

```bash
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
```

构建成功后，插件文件在这里：

```text
target/wasm32-wasip1/release/zellij-cn-ui.wasm
```

开发时可以跑完整测试：

```bash
cargo test
cargo build --release --target wasm32-wasip1
```

如果你的 Rust 工具链仍使用旧目标名，可以先运行：

```bash
rustup target add wasm32-wasi
```

## 本地源码布局

`layouts/zellij-cn-ui.kdl` 是模板，内含占位符 `__PROJECT_DIR__`，**不能直接** `zellij --layout layouts/zellij-cn-ui.kdl`，否则插件路径无效、顶/底栏会空白。

推荐用开发脚本（自动替换路径并写入 wasm 权限）：

```bash
chmod +x scripts/zellij-dev.sh
./scripts/zellij-dev.sh
```

或手动替换后启动：

```bash
PROJECT_DIR="$(pwd)"
sed "s#__PROJECT_DIR__#$PROJECT_DIR#g" layouts/zellij-cn-ui.kdl > layouts/.dev.kdl
zellij --layout layouts/.dev.kdl
```

## 发布给别人

你可以把这个文件上传到 GitHub Release 或其他下载页面：

```text
target/wasm32-wasip1/release/zellij-cn-ui.wasm
```

推荐同时上传布局模板：

```text
dist/zellij-cn-ui.kdl
```

普通用户只需要下载 `.wasm` 文件，放到：

```text
~/.local/share/zellij/plugins/zellij-cn-ui.wasm
```

然后按 README 的“小白快速使用”创建布局即可，不需要 Rust 环境。

当前仓库也准备了发布目录：

```text
dist/zellij-cn-ui.wasm
dist/zellij-cn-ui.kdl
```

如果重新构建了 wasm，可以更新发布目录：

```bash
mkdir -p dist
cp target/wasm32-wasip1/release/zellij-cn-ui.wasm dist/zellij-cn-ui.wasm
cp layouts/zellij-cn-ui.kdl dist/zellij-cn-ui.kdl
```

## 进阶配置

除了使用布局文件，也可以把下面配置加入 Zellij 配置文件中的 `plugins` 块，替换 `/path/to/zellij-cn-ui` 为本仓库的绝对路径：

```bash
mkdir -p "$HOME/Library/Caches/org.Zellij-Contributors.Zellij"
cat > "$HOME/Library/Caches/org.Zellij-Contributors.Zellij/permissions.kdl" <<'EOF'
"/path/to/zellij-cn-ui/target/wasm32-wasip1/release/zellij-cn-ui.wasm" {
    ReadApplicationState
}
"file:/path/to/zellij-cn-ui/target/wasm32-wasip1/release/zellij-cn-ui.wasm" {
    ReadApplicationState
}
EOF
```

```kdl
plugins {
    tab-bar location="file:/path/to/zellij-cn-ui/target/wasm32-wasip1/release/zellij-cn-ui.wasm" {
        skip_plugin_cache true
        bar "tab"
    }
    status-bar location="file:/path/to/zellij-cn-ui/target/wasm32-wasip1/release/zellij-cn-ui.wasm" {
        skip_plugin_cache true
        bar "status"
    }
}
```

然后正常启动 Zellij 即可。如果只想临时测试，仍然推荐布局方式：

```bash
zellij --layout layouts/zellij-cn-ui.kdl
```
