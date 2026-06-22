#!/usr/bin/env bash
# 构建 wasm-echarts 并启动 demo 静态服务器
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CRATE="$ROOT/crates/wasm-echarts"

echo "==> wasm-pack build (target=web)"
wasm-pack build "$CRATE" --target web --out-dir pkg

echo "==> 启动 demo 服务器 http://localhost:8080/demo/"
cd "$ROOT"
python -m http.server 8080
