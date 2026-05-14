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

布局放到 `~/.config/zellij/layouts` 后，可以这样启动：

```bash
zellij --layout zellij-cn-ui
```

如果想让 Zellij 默认就启动中文界面，可以把布局文件复制成默认布局文件名：

```bash
cp ~/.config/zellij/layouts/zellij-cn-ui.kdl ~/.config/zellij/layouts/default.kdl
zellij
```

这样以后直接运行 `zellij`，就会优先使用这个中文布局。另一种方式是在 `~/.config/zellij/config.kdl` 中设置 `default_layout "zellij-cn-ui"`。

如果已经有旧会话在运行，建议先退出或杀掉旧会话，再重新打开：

```bash
zellij list-sessions
zellij kill-session 会话名
zellij --layout zellij-cn-ui
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
4. 仍有问题时查看日志。先运行 `zellij setup --check`，它会显示当前机器的日志和缓存目录：

```bash
zellij setup --check
```

## 开发

想修改源码、自己构建 wasm、或打包 Release，请看 [开发文档](DEVELOPMENT.md)。
