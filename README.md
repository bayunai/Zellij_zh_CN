# Zellij 中文界面插件

这是一个独立的 Zellij Rust/WASM 插件，用同一个 wasm 产物替换默认 `tab-bar` 和 `status-bar`，显示中文标签栏与状态栏。

## 小白快速使用

推荐普通用户下载发布版 `zellij-cn-ui.wasm`，这样不需要安装 Rust，也不需要自己构建。

### 1. 下载发布版插件

创建一个固定目录存放插件：

```bash
mkdir -p ~/.local/share/zellij/plugins
```

然后把你从 Release 下载的 `zellij-cn-ui.wasm` 放到这个目录：

```text
~/.local/share/zellij/plugins/zellij-cn-ui.wasm
```

### 2. 把布局放进 Zellij layouts 目录

Zellij 默认会从 `~/.config/zellij/layouts` 读取自定义布局。创建目录：

```bash
mkdir -p ~/.config/zellij/layouts
```

创建布局文件：

```bash
cat > ~/.config/zellij/layouts/zellij-cn-ui.kdl <<'EOF'
layout {
    default_tab_template {
        pane size=1 borderless=true {
            plugin location="file:__PLUGIN_PATH__" {
                skip_plugin_cache true
                bar "tab"
            }
        }
        children
        pane size=2 borderless=true {
            plugin location="file:__PLUGIN_PATH__" {
                skip_plugin_cache true
                bar "status"
            }
        }
    }
}
EOF
```

把布局里的 `__PLUGIN_PATH__` 替换成刚才下载的 wasm 路径：

```bash
PLUGIN_PATH="$HOME/.local/share/zellij/plugins/zellij-cn-ui.wasm"
sed -i.bak "s#__PLUGIN_PATH__#$PLUGIN_PATH#g" ~/.config/zellij/layouts/zellij-cn-ui.kdl
```

### 3. 预授权插件权限

这个插件需要读取 Zellij 当前模式、标签、窗格数量。状态栏/标签栏是不可聚焦的 UI 插件，首次运行时权限提示可能无法接收键盘输入，所以建议先写入权限缓存。

macOS 默认缓存路径是：

```bash
mkdir -p "$HOME/Library/Caches/org.Zellij-Contributors.Zellij"
```

然后创建权限文件：

```bash
PLUGIN_PATH="$HOME/.local/share/zellij/plugins/zellij-cn-ui.wasm"
cat > "$HOME/Library/Caches/org.Zellij-Contributors.Zellij/permissions.kdl" <<'EOF'
"__PLUGIN_PATH__" {
    ReadApplicationState
}
"file:__PLUGIN_PATH__" {
    ReadApplicationState
}
EOF
sed -i.bak "s#__PLUGIN_PATH__#$PLUGIN_PATH#g" "$HOME/Library/Caches/org.Zellij-Contributors.Zellij/permissions.kdl"
```

Linux 用户的 Zellij 缓存目录通常是 `~/.cache/org.Zellij-Contributors.Zellij`，可以这样写：

```bash
PLUGIN_PATH="$HOME/.local/share/zellij/plugins/zellij-cn-ui.wasm"
mkdir -p "$HOME/.cache/org.Zellij-Contributors.Zellij"
cat > "$HOME/.cache/org.Zellij-Contributors.Zellij/permissions.kdl" <<'EOF'
"__PLUGIN_PATH__" {
    ReadApplicationState
}
"file:__PLUGIN_PATH__" {
    ReadApplicationState
}
EOF
sed -i.bak "s#__PLUGIN_PATH__#$PLUGIN_PATH#g" "$HOME/.cache/org.Zellij-Contributors.Zellij/permissions.kdl"
```

### 4. 启动中文界面布局

复制到 `~/.config/zellij/layouts` 后，可以这样启动：

```bash
zellij --layout zellij-cn-ui
```

如果已经有旧会话在运行，建议先退出或杀掉旧会话，再重新打开：

```bash
zellij list-sessions
zellij kill-session 会话名
zellij --layout zellij-cn-ui
```

## 源码构建

如果你想自己修改源码，或者自己打包发布版，再使用这一节。

### 1. 构建插件

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

如果你的 Rust 工具链仍使用旧目标名，可以先运行 `rustup target add wasm32-wasi`。

### 2. 本地源码布局

本仓库的 `layouts/zellij-cn-ui.kdl` 是模板，里面的 `__PROJECT_DIR__` 需要替换成当前项目目录：

```bash
PROJECT_DIR="$(pwd)"
sed -i.bak "s#__PROJECT_DIR__#$PROJECT_DIR#g" layouts/zellij-cn-ui.kdl
zellij --layout layouts/zellij-cn-ui.kdl
```

## 发布给别人

你可以把这个文件上传到 GitHub Release 或其他下载页面：

```text
target/wasm32-wasip1/release/zellij-cn-ui.wasm
```

普通用户只需要下载这个 `.wasm` 文件，放到 `~/.local/share/zellij/plugins/zellij-cn-ui.wasm`，再按“小白快速使用”创建布局即可。

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

## 功能

- 标签栏显示会话名、标签序号、标签名、活动标签和窗格数量；活动标签使用 `●` 标记，非活动标签使用 `○` 标记。
- 状态栏第一行显示当前模式、活动标签、窗格数和剪贴板提示。
- 状态栏第二行按模式分组显示快捷键；普通模式使用 `Ctrl  p 窗格  t 标签  n 调整  h 移动` 这类紧凑简写，窄屏时优先保留高频操作。
- 支持中文宽度截断，避免窄终端中换行污染栏位。
- 插件启动后会请求 `ReadApplicationState` 权限，用于读取 Zellij 模式、标签和窗格状态。

## 快捷键提示覆盖

提示内容按当前 `~/.config/zellij/config.kdl` 补全：

| 模式 | 主要提示 |
| --- | --- |
| 普通 | `Ctrl p 窗格`、`t 标签`、`n 调整`、`h 移动`、`s 滚动`、`o 会话`、`q 退出`、`Alt n 新窗格`、`f 浮动`、`i/o 移标签` |
| 窗格 | `d 下分屏`、`r 右分屏`、`n 自动窗格`、`s 堆叠`、`x 关闭`、`f 全屏`、`z 框架`、`e 浮动/嵌入`、`w 显示浮动`、`c 重命名`、`p 切焦点`、`hjkl/方向 切换` |
| 标签 | `n 新建`、`x 关闭`、`r 重命名`、`s 同步`、`1-9 跳转`、`Tab 最近标签`、`h/l/←→ 切换`、`[/] 拆分`、`b 拆出` |
| 调整 | `+/= 放大`、`- 缩小`、`hjkl/方向 调边界`、`Ctrl+n 返回` |
| 移动 | `hjkl/方向 移动`、`n/Tab 下个位置`、`p 上个位置`、`Ctrl+h 返回` |
| 滚动/搜索 | `s 输入关键词`、`n/p 上下匹配`、`PageUp/Down 翻页`、`e 编辑回滚`、`c 大小写`、`w 全词`、`o 循环` |
| 会话/Tmux | `d 分离`、`w 会话管理`、`p 插件管理`、`c 配置/新标签`、`% 右分屏`、`" 下分屏`、`, 重命名`、`[ 滚动` |

## 排查

如果顶部/底部栏位是空白：

1. 先按上面的权限缓存示例预授权 `ReadApplicationState`。
2. 确认使用的是新构建产物，并重新启动 Zellij 会话。
3. 开发调试时保留 `skip_plugin_cache true`，避免 Zellij 继续使用旧 wasm。
4. 仍有问题时查看日志：

```bash
zellij setup --check
tail -f /var/folders/70/zgm1mmkn27n3krn_sthq875c0000gn/T/zellij-501/zellij-log/zellij.log
```
