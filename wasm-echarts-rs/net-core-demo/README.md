# wasm-echarts NetCore demo

.NET 10 宿主：用 **Wasmtime** 直接加载 `pkg/*_bg.wasm`（不跑 `js/` facade，也不执行 `pkg/*.js`）。`pkg/*.js` 只给 `tools/GenWbgImports` 生成导入表。

## 环境

- .NET 10 SDK
- 仓库里已提交的 `crates/wasm-echarts/pkg/wasm_echarts_bg.wasm` 与 `crates/wasm-zrender/pkg/wasm_zrender_bg.wasm`

`wasm-pack` 更新 `pkg/` 之后必须重跑：

```bash
dotnet run --project tools/GenWbgImports
```

## P0 尖刺

```bash
dotnet run --project src/P0Spike
```

成功时在 `artifacts/` 写出 `p0-echarts-line.bmp` 与 `p0-zrender-rect.bmp`。
