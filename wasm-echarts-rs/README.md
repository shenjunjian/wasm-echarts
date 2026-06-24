# wasm-echarts-rs

Rust/WASM 图表渲染：`rust-zrender` + `wasm-zrender` + `wasm-echarts`。

## 编译 WASM

```bash
cd crates/wasm-zrender && wasm-pack build --target web
cd ../wasm-echarts && wasm-pack build --target web
cd .. && cargo test -p rust-zrender
```

## 文档站（Vite）

```bash
cd site
npm install
npm run dev
```

浏览器打开：

- 首页：<http://127.0.0.1:5173/>
- wasm-zrender 实例：<http://127.0.0.1:5173/zrender/examples/>
- wasm-echarts 实例：<http://127.0.0.1:5173/echarts/examples/>

生产构建：

```bash
cd site && npm run build && npm run preview
```

实例页为**左源码 · 右预览**布局，可编辑 JSON 后点击「运行」。
