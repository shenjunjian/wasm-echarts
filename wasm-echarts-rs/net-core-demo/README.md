# wasm-echarts NetCore demo

.NET 10 宿主：用 **Wasmtime** 直接加载 `pkg/*_bg.wasm`（不跑 `js/` facade，也不执行 `pkg/*.js`）。`pkg/*.js` 只给 `tools/GenWbgImports` 生成导入表。

## 环境

- .NET 10 SDK（WPF 画廊需要 Windows）
- 仓库里已提交的 `crates/wasm-echarts/pkg/wasm_echarts_bg.wasm` 与 `crates/wasm-zrender/pkg/wasm_zrender_bg.wasm`
- 站点字体 `site/public/fonts/`（构建时 Link 进输出目录，不另提交一份）

`wasm-pack` 更新 `pkg/` 之后必须重跑：

```bash
dotnet run --project tools/GenWbgImports
```

## WPF 画廊

```bash
dotnet run --project src/NetCoreDemo.Wpf
```

顶栏切换 `wasm-zrender` / `wasm-echarts`；左侧二级菜单 id 与站点文件名对齐；右侧是 AvalonEdit 只读 C# 源码 + `refresh()` RGBA 预览。

无窗口冒烟（写出 `artifacts/p1-*.bmp`）：

```bash
dotnet run --project src/NetCoreDemo.Wpf -- --smoke
```

## P0 尖刺

```bash
dotnet run --project src/P0Spike
```

成功时在 `artifacts/` 写出 `p0-echarts-line.bmp` 与 `p0-zrender-rect.bmp`。
