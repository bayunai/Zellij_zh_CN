#!/usr/bin/env bash
# 本地开发：替换布局占位符、写入 wasm 权限、启动 Zellij。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WASM="$ROOT/target/wasm32-wasip1/release/zellij-cn-ui.wasm"
LAYOUT="$ROOT/layouts/.dev.kdl"

if [[ ! -f "$WASM" ]]; then
  echo "未找到 wasm，请先构建：" >&2
  echo "  cargo build --release --target wasm32-wasip1" >&2
  exit 1
fi

sed "s#__PROJECT_DIR__#${ROOT}#g" "$ROOT/layouts/zellij-cn-ui.kdl" >"$LAYOUT"

if [[ "$(uname -s)" == "Darwin" ]]; then
  CACHE="$HOME/Library/Caches/org.Zellij-Contributors.Zellij"
else
  CACHE="$HOME/.cache/org.Zellij-Contributors.Zellij"
fi
mkdir -p "$CACHE"
PERMS="$CACHE/permissions.kdl"

grant_permission() {
  local key="$1"
  if [[ -f "$PERMS" ]] && grep -Fq "\"$key\"" "$PERMS"; then
    return
  fi
  cat >>"$PERMS" <<EOF
"$key" {
    ReadApplicationState
}
EOF
}

grant_permission "$WASM"
grant_permission "file:$WASM"

exec zellij --layout "$LAYOUT" "$@"
