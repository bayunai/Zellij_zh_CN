#!/usr/bin/env bash
# 安装为 Zellij 默认布局（~/.config/zellij/layouts/default.kdl）
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WASM_SRC="$ROOT/target/wasm32-wasip1/release/zellij-cn-ui.wasm"
PLUGIN_DIR="${HOME}/.local/share/zellij/plugins"
WASM_DST="${PLUGIN_DIR}/zellij-cn-ui.wasm"
LAYOUT_DIR="${HOME}/.config/zellij/layouts"
LAYOUT_DST="${LAYOUT_DIR}/default.kdl"
THEME_SRC="${ROOT}/themes/zellij-cn-ui-colors.kdl"
THEME_DIR="${HOME}/.config/zellij/themes"
THEME_DST="${THEME_DIR}/zellij-cn-ui-colors.kdl"
CONFIG="${HOME}/.config/zellij/config.kdl"

if [[ ! -f "$WASM_SRC" ]]; then
  echo "正在构建 wasm …"
  (cd "$ROOT" && cargo build --release --target wasm32-wasip1)
fi

mkdir -p "$PLUGIN_DIR" "$LAYOUT_DIR" "$THEME_DIR"
cp "$WASM_SRC" "$WASM_DST"
cp "$THEME_SRC" "$THEME_DST"

cat >"$LAYOUT_DST" <<EOF
layout {
    default_tab_template {
        pane size=1 borderless=true {
            plugin location="file:${WASM_DST}" {
                skip_plugin_cache true
                bar "tab"
            }
        }
        children
        pane size=2 borderless=true {
            plugin location="file:${WASM_DST}" {
                skip_plugin_cache true
                bar "status"
            }
        }
    }
}
EOF

if [[ "$(uname -s)" == "Darwin" ]]; then
  CACHE="${HOME}/Library/Caches/org.Zellij-Contributors.Zellij"
else
  CACHE="${HOME}/.cache/org.Zellij-Contributors.Zellij"
fi
mkdir -p "$CACHE"
PERMS="${CACHE}/permissions.kdl"
for key in "$WASM_DST" "file:$WASM_DST"; do
  if [[ ! -f "$PERMS" ]] || ! grep -Fq "\"$key\"" "$PERMS"; then
    cat >>"$PERMS" <<PEOF
"$key" {
    ReadApplicationState
}
PEOF
  fi
done

mkdir -p "$(dirname "$CONFIG")"
touch "$CONFIG"

set_config_value() {
  local key="$1"
  local value="$2"

  if grep -Eq "^[[:space:]]*${key}[[:space:]]+" "$CONFIG"; then
    sed -i.bak -E "s|^[[:space:]]*${key}[[:space:]].*|${key} ${value}|" "$CONFIG"
  elif grep -Eq "^[[:space:]]*//[[:space:]]*${key}[[:space:]]+" "$CONFIG"; then
    sed -i.bak -E "s|^[[:space:]]*//[[:space:]]*${key}[[:space:]].*|${key} ${value}|" "$CONFIG"
  else
    {
      echo ""
      echo "${key} ${value}"
    } >>"$CONFIG"
  fi
}

set_config_value "default_layout" '"default"'
set_config_value "theme" '"zellij-cn-ui-colors"'

echo "已安装："
echo "  wasm:   ${WASM_DST}"
echo "  布局:   ${LAYOUT_DST}"
echo "  主题:   ${THEME_DST}"
echo "  配置:   ${CONFIG}"
echo ""
echo "请关闭旧会话后重新运行 zellij："
echo "  zellij kill-all-sessions"
echo "  zellij"
