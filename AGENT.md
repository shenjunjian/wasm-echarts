# wasm-echarts-rs

Rust / WebAssembly workspace：用纯 Rust 重写 zrender 离屏 canvas 渲染内核，再通过 wasm-bindgen 分别暴露 **zrender 图元 API** 与 **ECharts option 管线**。目标是尽快把图表画到 canvas 上，**不追求动画中间帧**。SVG painter 与靠 DOM 实现的组件（DataView、SaveAsImage 下载条、Loading 旋转动画）不做；其余官方 canvas 绘制与纯计算 API 要与官方对齐。最主要的宗旨是：wasm-zrender, wasm-echarts的目标是要尽量和官方API保持一致。

本文按仓库梳理：目的、已实现内容、编译、启动、产物位置、文档站如何引用这些产物。

后续有新需求或实现变更时，一定要同步修改本文件！所有重要的需求和约束都要记录在本文件中。
---

## 文档说明

规划文件（仓库根目录 `.cursor/plans/`）：

| 规划 | 文件 | 作用 |
|------|------|------|
| 总实施规划 | `wasm-echarts_移植规划_692fb54c.plan.md` | 阶段 0–7 架构与模块设计 |
| 待办与 Demo | `wasm-echarts_待办与Demo规划.plan.md` | crate 拆分规范、已完成摘要、site 与后续 backlog |
| API 对齐 | `wasm-zrender_api_对齐_98a6aa98.plan.md` | 删除 scene 预设，对齐官方 `export.ts` 命名空间 |
| API 批量补录 | `wasm-zrender_api_批量补录_89d0d2a5.plan.md` | 把 stub 图元逐步换成真实实现 |
| zrender API 规范对齐 | `zrender_api_规范对齐_be3227a1.plan.md` | 硬规范 + JS facade；逐项清单以该计划为权威 |
| echarts API 规范对齐 | `wasm-echarts_api_对齐_1d164d7d.plan.md` | 公开入口对齐官方 core.ts（波次 0–5 已落地）；入口签名以该计划为权威 |
| echarts Canvas 全量对齐 | `echarts_canvas_全量对齐_3ceaa41e.plan.md` | SVG/DOM 例外以外的 canvas 语义、`getZr` 共享 Storage、22 种图与 canvas 组件。**第 0–8.9 波 YAML 已全部 completed**；视觉缺口以源码与下文「未实现」为准 |
| 折线缺口补齐 | `折线缺口补齐_96423c21.plan.md` | **已废弃**，并入 canvas 全量对齐 |
| 示例独立与 Worker | `示例独立与worker_72612d7b.plan.md` | 去掉 `runOfficialExample` 代管；facade `opts.useWorker`；option 不用 SAB，仅回图可用 SAB |
| wasm-zrender 文档重构 | `wasm-zrender_文档重构_d64ac165.plan.md` | 首页起因 + 三层定位；顶栏产品入口；zrender 五章多页文档 |

zrender API 规范对齐规划的 YAML todo 已全部 completed。wasm-echarts 公开入口（`init`/`setOption`/`on`）以 API 对齐计划与源码为准；canvas 全量对齐计划 YAML 已全部 completed，视觉缺口以**源码与下文「未实现」**为准。示例独立与 Worker 规划 YAML 已全部 completed。wasm-zrender 文档重构规划 YAML 已全部 completed。其它规划 YAML 里部分 todo 仍可能标 `pending`，以**源码为准**。下文「规划对照」会标明实际完成度。

只读参考源码（仓库根目录，禁止改）：`zrender-master/`、`echarts-master/`。

---

## 仓库总览

```
wasm-echarts/                         # 整个 git 仓库
├── wasm-echarts-rs/                  # 本 workspace（本文档所在目录）
│   ├── Cargo.toml                    # Rust workspace：members = crates/*
│   ├── Cargo.lock
│   ├── crates/
│   │   ├── rust-zrender/             # 纯 Rust rlib：渲染引擎
│   │   ├── wasm-zrender/             # wasm-pack cdylib + js/ facade：对齐 zrender export.ts
│   │   └── wasm-echarts/             # wasm-pack cdylib + js/ facade：对齐官方 init/setOption
│   ├── site/                         # Vite 多页文档站
│   └── scripts/serve-demo.sh         # 旧脚本（仍指向已删除的 demo/，请用 site）
├── echarts-master/                   # 官方 echarts 源码（只读）
└── zrender-master/                   # 官方 zrender 源码（只读）
```

四个「仓库」（crate / 前端工程）职责互不重叠：

| 名称 | 路径 | 类型 | 产物 |
|------|------|------|------|
| **rust-zrender** | `crates/rust-zrender/` | Cargo `rlib`，无 wasm-bindgen | `target/` 下的 `.rlib`，被另外两个 crate path 依赖 |
| **wasm-zrender** | `crates/wasm-zrender/` | `cdylib` + `rlib` + 手写 `js/` facade | `pkg/`（wasm-bindgen 内部）+ `js/`（公开 API） |
| **wasm-echarts** | `crates/wasm-echarts/` | `cdylib` + `rlib` + 手写 `js/` facade | `pkg/`（wasm-bindgen 内部）+ `js/`（公开 API） |
| **site** | `site/` | Vite 多页静态站 | 开发时直连 `js/` facade；`npm run build` 输出 `site/dist/` |

依赖规则：

```
site  ──import──►  wasm-zrender/js      （Vite alias @wasm-zrender；zrender 文档站）
                     └──► pkg/          （wasm-bindgen 内部 handle）
site  ──import──►  wasm-echarts/js      （Vite alias @wasm-echarts）
                     └──► pkg/          （echarts 运行时唯一 WASM）

wasm-zrender  ──path──►  rust-zrender
wasm-echarts  ──path──►  rust-zrender
wasm-echarts  ──path──►  wasm-zrender   （第 1 波落地；rlib，getZr / graphic 共用 Storage）
wasm-zrender  ✗ 不依赖  wasm-echarts
```

**第 1 波（当前源码）**：wasm-echarts path 依赖 wasm-zrender（rlib）。`wasm-echarts/pkg` 带上 `ZRender` / `LinearGradient` 等绑定。`getZr()` 与 ChartView 共用 `ZR_REGISTRY` 里同一份 Storage。echarts 页在 `js/native.js` 里 `setNative(echartsPkg)`，不得再加载 `wasm-zrender/pkg`。zrender 文档站继续用自己的 pkg。ChartView 每次 `Storage::new()` 后 `rematerialize_mounted_roots`，保留 `getZr().add` 的用户图元。

---

## 目标与约束

与仓库根 `README.md` 及总规划一致：

- **核心目标**：把 ECharts `option`（以及 zrender 图元树）快速画到 canvas。公开 API 尽量与官方一致。官方 canvas 绘制与纯计算 API 要齐；缺的按 [canvas 全量对齐计划](../.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md) 补，而不是永久砍掉。
- **渲染模式**：仅 canvas。不做 SVG painter。靠 DOM 实现的组件（toolbox DataView、SaveAsImage 下载条）不做。`showLoading`/`hideLoading` 导出，不播旋转 Loading 动画。
- **动画**：不播中间帧。`setOption` / 属性变更直接终态；zrender `animate` / `animateTo` / `when().start()` 立刻写入**最后一组**目标属性。`refresh()` / `flush()` 同步返回 RGBA。
- **上屏方式**：WASM 离屏绘制 → `Vec<u8>` / `Uint8Array` → JS `putImageData`。WASM 不直接操作 DOM canvas 2D context。
- **函数型 option**：不整包 serde。递归解析 `JsValue` 为 `OptionValue`，遇到 function 保留 `js_sys::Function`，在 visual / tooltip 阶段 `callN`。
- **Painter 后端**：`vl-convert-canvas2d`（tiny-skia + cosmic-text）。官方 canvas 有而它没有的能力，在 `canvas/backend/` 补齐（阴影、命中检测、径向渐变 r0 等）。

### wasm-zrender API 硬规则

公开表面与官方 `zrender-master/src/export.ts` / `zrender.ts` / `Element.ts` 对齐。**逐项清单以** [`.cursor/plans/zrender_api_规范对齐_be3227a1.plan.md`](../.cursor/plans/zrender_api_规范对齐_be3227a1.plan.md) **为权威**；下文只留可核对条目。禁止改官方目录，也禁止整文件复制官方实现（工具函数按签名重写）。

**允许例外（仅此四条）：**

- 字体：必须 `registerFont` / `register_font`，WASM 不读系统字体。
- 动画：不播中间帧；`animate` / `animateTo` / `when().start()` 立刻写入最后一组目标属性。
- 离屏：仅 canvas；`refresh()` / `flush()` 同步返回 RGBA；无 SVG painter / hover layer / dirty rect。
- 宿主：`init(canvas)` 可自动 `putImageData`；`init(null)` 仍用 opts 宽高。

**必须一致：**

- 公开入口与官方同构：`init` / `dispose` / `disposeAll` / `getInstance` / `version` / `registerPainter`，以及 `export.ts` 全部类型与 `matrix` / `vector` / `color` / `path` / `util` 命名空间。
- 图元原型链：`Rect instanceof Path instanceof Displayable instanceof Element`（Group 亦 `instanceof Element`）。
- 变换主属性：`x` `y` `scaleX` `scaleY` `rotation` `originX` `originY`；`attr` / `setShape` / `setStyle` 支持对象与 `key, value`。
- Group：`new Group(opts)`，以及 `children` / `childAt` / `childOfName` / `childCount` / `eachChild` / `traverse` / `addBefore` / `replace`。
- Shape 字段与官方默认 style（尤其 `Sector.r0`、`Rect.r`、Line 默认 stroke 无 fill、Text 默认 fill `#000`）。
- 实例生命周期：`zr.clear` / `zr.dispose` / `setBackgroundColor` / `trigger`；元素 `hide`/`show`、`on`/`off`/`trigger`、`clipPath`。

**本波不挡主路径（后置，不混进「已对齐」）：** `skewX/Y` `anchorX/Y`、`textContent` 自动布局、RichText、`morph` 形变、`IncrementalDisplayable` 增量语义、`Path.extend` 自定义 `buildPath` 走 PathProxy。

**文档硬规则（与实现对齐同等重要）：**

公开文档（`site/zrender/docs/`、根 README、本文件）必须始终有这五章，实现变更时同步改。`/zrender/docs/` 默认落在快速上手；起因介绍与三层定位写在站点首页，不在 zrender 文档侧栏复述。

1. **快速上手**：pkg 与 JS facade 分工；推荐顺序 `initWasm` → `registerFont`（有文字时）→ `init` → `add`；绑定 canvas / 离屏 `refresh()` 两段示例。
2. **字体引用**：浏览器 `ctx.font` 能画字、WASM 离屏读不到 OS 字体；必调 `registerFont`；调用顺序、热更新、`sans-serif` 映射。
3. **API 参考**：facade（`@wasm-zrender` → `js/`）为默认公开面；pkg 单独成章。每条有官方 / 补充 / 降级徽章 + 参数表 + 短示例。
4. **与官方差异**：允许例外（仅四条）/ 已对齐 / 降级后置 / 补充 API 分表。禁止把后置未做写成允许例外。
5. **底层原理**：rust-zrender crates 选型、相对 vl-convert 补齐、JS↔WASM 三种过桥。三层定位回链首页。

### wasm-echarts API 硬规则

公开表面与官方 `echarts-master/src/core/echarts.ts` / `export/core.ts` / `export/api.ts` 对齐。公开入口签名以 [`.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md`](../.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md) 为权威；canvas 图表/组件/命名空间/`getZr` 已按 [`.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md`](../.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md) 落地（第 0–8.9 波 YAML completed），视觉与宿主缺口以源码与下文「未实现」为准。禁止改官方目录，也禁止整文件复制官方实现。WASM 是整包模块，**不为减小体积做 `echarts.use` 动态加载**。

做不到或语义不同的，不改名糊弄，而是写进「与官方不一致」。多出来的非官方方法必须文档列出，避免当官方 API 用。已实现 / 未实现（图表、组件、action、option 字段）单独成表，随实现更新。剩余视觉/宿主缺口标「未实现」（不是永久砍掉官方 canvas API）；已声称支持的图，其 canvas option 族必须生效（禁止只 parse、画成直线）。

**允许例外（须在文档「与官方不一致」节列出）：**

- 字体：WASM 不读系统字体；需要 `registerFont`（可挂在 echarts 命名空间）。
- 动画：不播中间帧；`setOption` / `animation` / `universalTransition` / line grow / ripple 直接终态（第 7 波：`UniversalTransition` 只跳终态，不插值）。`lazyUpdate` 可同步执行（等价立刻 flush）。
- 离屏：仅 canvas；`init(canvas)` 自动 `putImageData`。无 SVG（`renderToSVGString` / `getSvgDataURL` / `zr.painter.getSvgDom` 只 `console.warn`）。
- 宿主：`init(canvas, theme?, opts?)`；`init(null, null, { width, height, devicePixelRatio })` 允许离屏（官方客户端无 dom 会抛错）。
- Worker：可选非官方 `opts.useWorker`（默认 `false`，同线程同步 `setOption`）。Worker 持 WASM；主线程 blit / 事件 / tooltip。含 `formatter` / `renderItem` 等函数的 option 不能进 Worker；option（含 TypedArray）一律 `postMessage`，不用 SAB。SAB 只用于 Worker → 主线程回传 RGBA（需 COOP/COEP）。
- `use(...)`：**导出且签名对齐**，但不按需加载。实现只 `console.info` 提示：已开发的模块都在 WASM 里，不必 `use`。调用可忽略参数并立即返回。
- Loading：`showLoading` / `hideLoading` **导出**；不播官方旋转动画（静态半透明遮罩 + 文案，或空操作）。
- DOM 组件：toolbox **DataView** 浮层、SaveAsImage 的 DOM 下载条不做。Tooltip 维持现有 string DOM（官方默认即 DOM）。
- `getZr()`：返回与 ChartView 共用 Storage 的 wasm-zrender 实例。不是第二份 zrender wasm；无 SVG painter / hover layer；动画终态。`painter.getSvgDom` 只 warn。

**必须一致（公开 JS 表面）：**

- 入口：`init` / `dispose` / `getInstanceByDom` / `getInstanceById` / `version`（`'6.1.0'`）/ `use`（提示实现）。
- 实例方法 camelCase，签名对齐：`setOption` / `getOption` / `resize` / `dispatchAction` / `on` / `off` / `getWidth` / `getHeight` / `getDevicePixelRatio` / `isDisposed` / `clear` / `dispose`。
- `convertToPixel` / `convertFromPixel` / `containPixel`：cartesian 多 grid / 多轴 finder（`gridIndex` / `xAxisIndex` / `yAxisIndex` / `seriesIndex`），含 `time` / `log` 轴；以及 `polar` / `geo` / `calendar` / `singleAxis` / `parallel` / `matrix` / `radar` finder。
- `setOption(option)` 与 `setOption(option, notMerge)` / `setOption(option, { notMerge, replaceMerge, silent })`。`notMerge` **不是** option 里的字段。
- `resize()` 无参（读 canvas 尺寸）与 `resize({ width, height, devicePixelRatio })`。
- `init(canvas)` 后指针事件由 facade 绑定；发出官方事件名 `click` / `mouseover` / `mouseout` / `globalout`。
- 已接线的 `dispatchAction` type 名保持官方字符串：`highlight` / `downplay` / `select` / `unselect` / `toggleSelect` / `dataZoom`；并补 `showTip` / `hideTip`、`legendToggleSelect` / `legendSelect` / `legendUnSelect`、`restore`、`timelineChange` / `timelinePlayChange`、`takeGlobalCursor`、`brush` / `brushEnd`、`expandAxisBreak` / `collapseAxisBreak` / `toggleAxisBreak`。
- `export/api.ts` 命名空间：`graphic` / `util` / `number` / `time` / `format` / `helper` / `matrix` / `vector` / `color`（第 1 波从 wasm-zrender 再导出或按签名重写）；以及 `throttle`。
- `getZr` / `getDataURL` / `renderToCanvas` / `appendData` / `setTheme` / `registerTheme` / `registerMap` / `connect` / `registerTransform` / `registerPreprocessor` / `registerProcessor` / `registerLayout` / `registerVisual` / `registerAction` / `registerCoordinateSystem` / `registerCustomSeries`：已接线。缺实现的同名入口 `console.warn`，不抛 `is not a function`。

**文档硬规则（与实现对齐同等重要）：**

公开文档（`site/echarts/docs/`、根 README、本文件）必须始终有这七章，实现变更时同步改。`/echarts/docs/` 默认落在快速上手。**公开文档不含 probe**：不写 ok/error/timeout 计数、波次编号、「第 x 波 YAML」。能力缺口用「库现在画不了 / 缺插件 / 缺增量绘制」表述。同步示例的 agent 仍可在本文件保留 probe 操作说明。

1. **快速上手**：pkg 与 JS facade 分工；推荐顺序 `initWasm` → `registerFont`（有文字时）→ `init` → `setOption`；绑定 canvas / 离屏 `refresh()` 两段示例。默认主线程，回链运行模式。
2. **字体引用**：必须从 `@wasm-echarts` 注册；echarts 页与 `getZr` 共用同一份 WASM / fontdb；`useWorker` 时同一份 bytes 进另一份 WASM。
3. **运行模式**：为何要 Worker、默认主线程、`opts.useWorker` 用法、函数 / DOM / `getZr` / SAB 限制。不要自己 `new Worker`。
4. **API 参考**：facade（`@wasm-echarts` → `js/`）为默认公开面；pkg 单独成章。每条有官方 / 补充 / 降级徽章 + 参数表 + 短示例。不要直接 `new EChartsInstance`。
5. **与官方差异**：允许例外 / 已对齐 / 降级后置 / 补充 API 分表。禁止把后置未做写成允许例外。Worker 开关是补充 API，限制写进运行模式，差异页只留一行回链。
6. **已实现 / 未实现**：图表 / 坐标系 / 组件 / 实例 API 能力表。未实现的官方方法仍导出同名 + `console.warn`。不写 probe 报表。
7. **底层原理**：option 管线、单 WASM、JS↔WASM 过桥、Worker 数据流。三层定位回链首页。

剩余缺口写进「未实现」表（视觉简化、缺插件、刻意不做的 SVG/DOM）。不要把「图类型还没做」写成「canvas 能力永久例外」。

---

## 架构与数据流

```
浏览器 DOM / 指针事件
        │
        ▼
site 示例 JS（创建 canvas；`echarts.init` / `setOption`）
        │
        ├── wasm-zrender：init / Group / Rect / … / refresh / findHover
        └── wasm-echarts：js/ facade init / setOption / dispose；native 为内部 handle
                │
                ├── 默认：主线程 EChartsInstance
                └── opts.useWorker：Worker 持实例；option postMessage；RGBA 经 SAB/Transferable 回主线程 blit
                │
                ▼
        rust-zrender::ZRenderer
                │
   Element 树 → Storage.updateDisplayList（z/z2/zlevel 排序）
                │
                ▼
        Painter.brush（Path / Image / Text，多 zlevel Layer）
                │
                ▼
        VlConvertBackend（vl-convert-canvas2d + shadow pass）
                │
                ▼
        RGBA buffer → JS ImageData → 页面 canvas
```

职责划分：

| 层 | Rust / WASM | site 示例 JS |
|----|-------------|--------------|
| option 解析 | `OptionValue`，函数保留为 `js_sys::Function` | 原样传入用户 option |
| 布局 / 坐标 / 绘制 | cartesian、ChartView、Painter | —；`useWorker` 时这段在 Worker 线程跑 |
| Worker 回图 | RGBA 写入 SAB（或 Transferable） | facade `putImageData`；option 不走 SAB |
| 命中检测 | Path winding + stroke 距离；Image/Text bbox | facade 绑指针并转坐标 |
| tooltip / 高亮 | formatter 得 string；hover / axisPointer / 拖拽变化才改 state 并 refresh | facade 内建 string tooltip DOM；`on('click')` |
| resize | 重算 layout + 全量 refresh | 示例按需调用 `resize` |
| 动画 | 跳过中间帧，写入终态 | `animate` / `when().start()` 写最后一组目标属性 |

---

## 规划对照：已完成移植项

### 总规划阶段 0–7（对照源码）

| 阶段 | 规划内容 | 源码现状 |
|------|----------|----------|
| **0 工程基础** | crate 结构、vl-convert、`CanvasBackend`、demo | **已完成**。三 crate + `site/`；`rust-zrender` 引入 `vl-convert-canvas2d 2.0.0-rc1` |
| **1 zrender P0** | Storage / Path / Painter / 基础 shape / brush | **已完成**。Rect/Circle/Line/Polygon/Polyline/Sector + fill/stroke/clip/transform/globalAlpha |
| **2 后端补齐** | shadow、isPointInPath、r0、渐变/虚线/Image | **主体完成**。conic gradient、CSS filter **未做**（刻意忽略或待办） |
| **3 交互基础** | Handler.findHover、ECData、emphasis/select | **已完成**。Path/Image/Text 均可命中 |
| **4 JS 薄壳 + API 骨架** | init/setOption/resize/事件、`OptionValue` | **部分完成**。`js/` facade 已有 `init`/`setOption(option, notMerge\|opts)`/`getOption`/`resize`/`clear`/`dispose`/`use`/`on`/`off`；`init(canvas)` 绑指针并内建 string tooltip |
| **4b 回调桥** | CallbackDataParams、call0/1/2 | **部分完成**。`formatter` / `itemStyle.color` / `axisLabel.formatter` / `symbolSize` 回调可用；params 含 `componentType`/`seriesType`/`percent`/`data`。`renderItem` + `api.coord`/`size`/`style`/`value` 已接线（第 6 波）；per-series 缓存 **未完成** |
| **5 echarts MVP** | GlobalModel、cartesian、line/bar、Scheduler | **部分完成**。line/bar 可渲染；`axisLabel.formatter`、`symbol`/`symbolSize`、`label.show`；`convertToPixel` 最小集。`replaceMerge` 仅为顶层 key。media query 已在第 7 波落地；完整 SeriesData 仍简化 |
| **6 交互完善** | hover/tooltip/dataZoom/axisPointer | **部分完成**。`init(canvas)` 绑指针；hover 高亮（同数据项 mousemove 不上屏）、`on('click')`、string tooltip DOM、`showTip`/`hideTip`、inside 滚轮 + slider 拖动手柄、十字 axisPointer、`tooltip.trigger: 'axis'`。pinch、HTMLElement tooltip **未完成** |
| **7 扩展与优化** | pie/scatter、RichText、脏矩形、视觉回归 | **部分完成**。官方 `export/charts.ts` 23 种图（含 chord）canvas 终态已接线（第 6 波）；RichText、脏矩形、golden PNG **未完成** |

### wasm-zrender API 规范对齐（波次 0–6 + 文档验收已完成）

规范已写入上文「目标与约束」；JS facade 骨架在 `crates/wasm-zrender/js/`，site / README 从 `@wasm-zrender`（`js/index.js`）导入，`pkg/` 只作内部 handle。波次 2：变换主属性、`attr`/`setShape`/`setStyle` 双参数、Group opts 与子树 API。波次 3：`Sector.r0` / `clockwise` / `cornerRadius`、`Rect.r`、Polygon.smooth、Line·Text 默认 style、`miterLimit`、`lineDash` 字符串、`LinearGradient.addColorStop`。波次 4：`js/tool/` 按官方签名实现 `matrix` / `vector` / `color` / `util` / `path`；`morph` / `parseSVG` / `showDebugDirtyRect` / `setPlatformAPI` 为签名齐全的最小实现。波次 5：`Animator` 写最后一组 `when`；`zr.clear` / 实例 `dispose` / `setBackgroundColor` / `trigger`；`el.hide`/`show`/`off`/`trigger`。波次 6：`getClipPath` / `removeClipPath`，clip 扩到 Group/Text/Image（Group clip 对子树生效）；`useStates` / `getState` / `ensureState` / `clearStates`；`zr.setCursorStyle` / `configLayer`；`Path.extend` 把 `buildPath` 录成 pathData；`IncrementalDisplayable` 按普通 Group 语义可构造；Point 静态方法、`BoundingRect.calculateTransform`。文档验收：AGENT.md / README / site 五章文档与导入路径已同步四条例外；`shapes` / `text` / `animation` / `bounding_box` 示例覆盖原型链、`Group.x`、`Rect.r`、`Sector.r0`、动画终态、`init(canvas)` 拖拽包围盒。余下后置项见该计划「本波不挡主路径」。

### wasm-zrender API 对齐（规划 todos 全部 completed）

已从源码删除 `scene.rs` / `load_scene` 预设场景。对外改为官方风格：

```javascript
const zr = init(null, { width, height });
zr.add(new Group());
const rgba = zr.refresh();
zr.findHover(x, y);
dispose(zr);
```

字体：`registerFont` / `clearFonts`；site text 示例走 `fetch` + 注册流程。

### wasm-zrender API 批量补录（规划 YAML 未改，源码已远超当时清单）

规划撰写时「已实现 8 类、其余 stub」。当前源码：

| Phase | 内容 | 现状 |
|-------|------|------|
| 1 渐变 / Pattern | `LinearGradient` / `RadialGradient` / `Pattern` + opts 解析 | **已完成** |
| 2 基础 Path | Arc / Ellipse / Ring / BezierCurve | **已完成** |
| 3 装饰 shape | Heart / Star / Isogon / Droplet / Rose / Trochoid | **已完成** |
| 4 Image / 通用 Path / CompoundPath | Storage 纳入 Image；`pathData`；CompoundPath | **已完成** |
| 5 Text / Displayable / TSpan | `ChildRef::Text`、displayList、hit-test；TSpan MVP；Displayable 抽象类 | **已完成**（TSpan 为单 run MVP） |
| 6 几何值对象 | Point / BoundingRect / OrientedBoundingRect | **已完成** |
| 7 工具模块 | color / matrix / vector / path / util / morph / parseSVG | **已完成**（`js/tool/`；`morph` 返回终点 path，`parseSVG` 解析基本 path/图形） |

`Displayable`：JS facade 可作原型祖先直接构造；rust / wasm-bindgen 的 `Displayable` 构造仍提示「抽象基类，请用具体图元」。`IncrementalDisplayable` 公开入口走 `js/incremental.js`（普通 Group 语义，可构造），不再抛错。

### 文档官网（待办规划当时最高优先级）

`docs-site` / `demo-zrender` / `demo-echarts` 在规划 YAML 仍为 pending，**site 工程已落地**：首页起因 + 三层定位（无产品卡片）；顶栏进 wasm-zrender / wasm-echarts；zrender 五章多页文档 + 侧栏；实例页左源码右预览。

### wasm-echarts API 规范对齐（波次 0–5 已完成）

规范已写入上文「目标与约束」；逐项清单以 [`.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md`](../.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md) 为权威。JS facade 骨架在 `crates/wasm-echarts/js/`：`init` / `dispose` / `getInstanceByDom` / `getInstanceById` / `version` / `use`（只 `console.info`）。site / README 从 `@wasm-echarts`（`js/index.js`）导入，`pkg/` 只作内部 handle。示例走官方 `init(canvas)` + `setOption`；`init(canvas)` 后自动 `putImageData` 并绑定指针。波次 2：`setOption(option, notMerge | opts)`、`getOption`、`resize()` / `resize({ width, height, devicePixelRatio })`、`clear` = `setOption({ series: [] }, true)`；`notMerge` 只作第二参数，不再从 option 根读取。波次 3：`on`/`off` 发出 `click` / `mouseover` / `mouseout` / `globalout`；内建 string tooltip DOM；`dispatchAction` 接 `showTip` / `hideTip`。波次 4：`axisLabel.formatter` 进轴 Text；pie `center`/`radius`/`startAngle`/`clockwise`；line/scatter `symbol`/`symbolSize`；`label.show` 画 Text；CallbackDataParams 补 `componentType`/`seriesType`/`percent`/`data`；`convertToPixel`/`convertFromPixel` cartesian 最小集。波次 5：文档四块（一致 / 不一致 / 非官方 API / 已实现与未实现）已写入 `site/echarts/docs/index.html`、根 README、本文件；示例 line/bar/pie/scatter/interactive/merge/bench 全部走 `init`；interactive 用 `on('click')` + `use()` 提示，不必 `handlePointer*`；merge 覆盖深合并、`notMerge: true`、dispose 后再 init。公开用法是官方 `init` / `setOption` / `on`，不要把 native `EChartsInstance` / `set_option` / `handlePointerMove` 当公开 API。

### wasm-echarts Canvas 全量对齐（第 0–8.9 波已落地）

权威计划：[`.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md`](../.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md)。废止「不是一次移植完官方全量 API」以及把 `graphic`/`util`/面积/`getZr` 当永久后置的写法。[折线缺口补齐](../.cursor/plans/折线缺口补齐_96423c21.plan.md) 已废弃并入本计划。第 0 波改文档口径；第 1 波已落地单 WASM、`getZr`、公开命名空间与挡脚本的实例 API。第 2 波已落地 dataset / transform / encode / stack / sampling、多 grid / 轴、`time` / `log`。第 3 波已接线 line/bar/pie/scatter 的 canvas option 族。第 4 波已接线 legend 筛选、mark*、graphic、visualMap、slider、axisPointer、brush、timeline、toolbox。第 5 波已接线 polar / radar / singleAxis / parallel / calendar / matrix / geo 坐标系。第 6 波已接线其余官方图表（radar/gauge/candlestick/boxplot/heatmap/pictorialBar/effectScatter/funnel/chord/sunburst/tree/treemap/graph/sankey/themeRiver/map/lines/parallel/custom）及 `renderItem` + `api`；Cargo feature **默认全开**。未识别的 `series.type` 会 `console.warn`，不再静默当 `Other` 后永远不管。第 7 波已落地扩展注册 / media / labelLayout / breaks / jitter。第 8.0–8.8 按类同步官网画廊；**8.9 已落地**：去重全量 probe 281/296 `ok`，失败写入上文「未实现」；`getZr` 文档写清同一份 wasm-zrender、无 SVG painter / hover layer、动画终态；缺官方实例方法同名导出 + `console.warn`。

### wasm-zrender 文档重构（已完成）

权威计划：[`.cursor/plans/wasm-zrender_文档重构_d64ac165.plan.md`](../.cursor/plans/wasm-zrender_文档重构_d64ac165.plan.md)。首页改为起因介绍 + rust-zrender / wasm-zrender / wasm-echarts 三层定位，去掉产品卡片。全站顶栏（`site-header.js`）品牌回首页，产品名进文档默认页，进入产品后旁挂「文档 / 实例」。zrender 文档拆成五章多页 + 侧栏（`docs-shell.js`）：`/zrender/docs/` 默认快速上手；字体、JS facade / pkg API、与官方差异、底层原理各成页。该波 echarts 文档正文不动（已由后续 echarts 文档重构拆成七章）。

### wasm-echarts 文档重构（已完成）

权威计划：[`.cursor/plans/wasm-echarts_文档重构_a2537acd.plan.md`](../.cursor/plans/wasm-echarts_文档重构_a2537acd.plan.md)。`/echarts/docs/` 默认快速上手；侧栏七章：上手 / 字体 / 运行模式 / API（facade + pkg）/ 差异 / 覆盖 / 原理。公开文档不含 probe。`docs-shell.js` 按路径切 `ECHARTS_DOC_NAV`。

---

## 环境要求

| 工具 | 说明 |
|------|------|
| [Rust](https://rustup.rs/) | 1.70+ |
| wasm32 target | `rustup target add wasm32-unknown-unknown` |
| [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) | 把 cdylib 编成浏览器 ES Module + `.wasm` |
| Node.js + npm 或 pnpm | 跑 `site/`（Vite 6） |

Release 配置在 workspace `Cargo.toml`：`[profile.release] opt-level = "s"`（偏体积）。

---

## 一、rust-zrender

### 目的

纯 Rust 的 zrender 渲染库。**没有** `wasm-bindgen`，可在原生 `cargo test`、未来 Wasmer / 服务端离屏场景直接使用。wasm-zrender 只依赖本层。wasm-echarts 额外 path 依赖 wasm-zrender rlib（getZr / graphic），绘制内核仍是 `rust_zrender::ZRenderer`。

Cargo 包名 `rust-zrender`，库名 `rust_zrender`。代码里写 `use rust_zrender::...`。

### 实现了哪些内容

源码根：`crates/rust-zrender/src/`。

#### `lib.rs` 公开模块

`canvas`、`contain`、`core`、`element`、`graphic`、`handler`、`storage`、`zrender`。

顶层导出：`ZRenderer`、`Storage`、`Painter`、`Handler`、`CanvasBackend` / `VlConvertBackend`、`register_font` / `clear_fonts`、图元与样式类型、`contain` / `contain_stroke`。

#### `zrender.rs` — `ZRenderer`

把 Storage + Painter + Handler 合成一个引擎实例：

- `new` / `new_with_dpr`
- `refresh()` → RGBA `Vec<u8>`
- `resize` / `resize_with_dpr`（重建 vl-convert 后端，mark dirty）
- `find_hover(x, y)`
- `set_path_state` / `set_path_state_style`（emphasis / select / normal）
- `set_path_ec_data`
- `update_font_database`（注册字体后热更新）

#### `storage/` — 场景图

- 节点池：`groups` / `paths` / `images` / `texts`
- 根列表 `roots: Vec<ChildRef>`
- `ChildRef`：`Group` | `Path` | `Image` | `Text`
- `add_root` / `del_root`、`group_add_child` / `group_remove_child`
- DFS 生成 `displayList`，按 z / z2 / zlevel timsort
- clipPath 链写入 `DisplayItem`（Path / Group / Text / Image 均可挂 clip；Group clip 对子树生效），Painter 绘制前 `clip()`

#### `canvas/` — Painter 与后端

| 文件 | 职责 |
|------|------|
| `backend/mod.rs` | `CanvasContext` / `CanvasBackend` trait |
| `backend/vl_convert.rs` | vl-convert-canvas2d 实现 |
| `backend/shadow.rs` | tiny-skia 阴影 pass（补官方 shadowBlur/Offset/Color） |
| `painter.rs` | 单层 / 多 zlevel Layer 合成 |
| `layer.rs` | 一层离屏 backend |
| `brush.rs` | Path fill/stroke/clip/transform/alpha |
| `helper.rs` | 纯色 / 线性渐变 / 径向渐变（含 r0）/ Pattern（含平移缩放旋转） |
| `image.rs` / `image_brush.rs` | RGBA `drawImage` |
| `text_brush.rs` | fillText（cosmic-text） |
| `font_registry.rs` | 全局 fontdb；wasm32 不读系统字体 |
| `demo.rs` | 原生测试用的简单图形场景 |

**已补齐相对 vl-convert 的缺口**：shadow、自实现 isPointInPath（见 contain）、径向渐变 r0、lineDash（含 `'dashed'` / `'dotted'`）/ lineCap / lineJoin / miterLimit、Image。

**未做**：conic gradient、CSS filter。

#### `graphic/` — 图元

- `path.rs` + `path_proxy.rs`：buildPath → rebuildPath，bbox
- `group.rs`：容器
- `image.rs`、`text.rs`
- `style.rs`：`FillStrokeStyle` = None | Color | LinearGradient | RadialGradient | Pattern；`ShadowStyle`；`miterLimit` 默认 10
- `shapes/`：见下表
- `displayable.rs`：z / z2 / zlevel / invisible / culling

Path shape（均写入 `PathProxy`，命中检测自动走 kurbo）：

| Shape | 文件 |
|-------|------|
| Rect（`shape.r` 圆角）, Circle, Line（默认 stroke `#000` 无 fill）, Polygon（`smooth`）, Polyline, Sector（`r0` / `clockwise` / `cornerRadius`） | `rect.rs` 等 |
| Arc, Ellipse, Ring, BezierCurve | Phase 2 |
| Isogon, Star, Heart, Droplet, Rose, Trochoid | Phase 3 |
| PathData（SVG `d` / pathData） | `path_data.rs` |
| CompoundPath | `compound_path.rs` |

#### `element/`

- 变换矩阵、dirty bit、`markRedraw`
- `EcData`：`series_index` / `data_index` / `data_type`（供 echarts 反查）
- 状态：`emphasis` / `select` / `normal` + `PathStylePatch`

#### `contain/` + `handler/`

- `contain/path.rs`：winding number 填充命中、stroke 距离（kurbo），带 transform
- `handler`：反向遍历 displayList；clip 链过滤；Path / Image / Text 分别 hit-test

#### `core/`

`matrix`、`bbox`、`point`、`obb`（旋转包围盒）、公共类型。

#### 字体

- **禁止**把默认字体打进 `.wasm`
- 宿主调用 `register_font(bytes, RegisterFontOptions { family_name, sans_serif })`
- wasm32：`load_system_fonts = false`
- 原生：默认可加载系统字体，并与已注册字体合并
- 未注册字体时画 Text 会报 `no default font found`

### 如何编译

在 workspace 根目录：

```bash
cd wasm-echarts-rs
cargo build -p rust-zrender
cargo build -p rust-zrender --release
```

它没有 `wasm-pack` 产物，也不单独给浏览器用。改 rust-zrender 后必须重新 `wasm-pack build` 两个 WASM crate，浏览器才能拿到新逻辑。

### 如何测试 /「启动」

这是库，没有进程入口。验证方式：

```bash
cd wasm-echarts-rs
cargo test -p rust-zrender
```

覆盖几何、brush、shadow、font、handler、各 shape 的 `build_path`、Storage 刷新出非空 RGBA 等。部分测试会写出固定尺寸像素，用于手工对照。

Wasmer / 原生宿主最小用法：

```rust
use rust_zrender::{register_font, RegisterFontOptions, ZRenderer};

register_font(font_bytes, RegisterFontOptions {
    family_name: Some("Noto Sans SC".into()),
    sans_serif: Some(vec!["Noto Sans SC".into()]),
})?;

let mut zr = ZRenderer::new(480, 360)?;
// 往 zr.storage 里 create_path / add_root ...
let rgba = zr.refresh()?;
```

### 编译产物在哪里

| 产物 | 位置 |
|------|------|
| 调试 rlib / 依赖 | `wasm-echarts-rs/target/debug/` |
| 发布 rlib | `wasm-echarts-rs/target/release/` |
| 增量缓存 | `wasm-echarts-rs/target/`（gitignore） |

浏览器**不会**直接引用这些文件。site 只引用 wasm-pack 的 `pkg/`。

---

## 二、wasm-zrender

### 目的

`rust-zrender` 的浏览器入口：手写 JS facade（原型链 + 官方命名导出）委托到 wasm-bindgen，再进 `rust-zrender`。对外命名空间对齐官方 zrender `src/export.ts` / `src/zrender.ts`：`init`、`Group`、`Rect`、`Circle`… 供文档站底层实例、以及「不用 ECharts option、直接构图」的集成。

Cargo：`crate-type = ["cdylib", "rlib"]`，依赖 `rust-zrender` path。公开 import 走 `js/`，`pkg/` 只作内部 handle。

### 实现了哪些内容

源码：`crates/wasm-zrender/js/`（公开 facade）与 `crates/wasm-zrender/src/`（wasm 桥）。

#### `js/` — 手写 facade（site 唯一公开入口）

| 文件 | 职责 |
|------|------|
| `index.js` | 官方命名导出；`default` 仍是 `initWasm`；`version = '6.1.0'` |
| `element.js` / `displayable.js` / `path.js` / `group.js` / `text.js` / `image.js` / `shapes/` | 原型链；实例持有 `_native` handle。变换主属性与 Group `_children` 在 JS 维护，变更同步 rust。`hide`/`show`/`on`/`off`/`trigger` |
| `animator.js` | `animate` / `when` / `start` 终态：写入最后一组目标，调用 `during(1)` / `done` |
| `zrender.js` | 包装 `init` / `dispose` / `getInstance` / `registerPainter`；`clear` / `setBackgroundColor` / `trigger` / `setCursorStyle` / `configLayer` |
| `wasm_zrender.js` | 兼容旧路径 `@wasm-zrender/wasm_zrender.js` |
| `tool/` | `matrix` / `vector` / `color` / `util` / `path` 按官方签名实现；`morph`（终态）/ `parseSVG`（基本 path）/ `showDebugDirtyRect`（no-op）/ `setPlatformAPI` |

构造约定：子类 `super()` 不创建 native，再 `_bindNative(new native.Rect(opts))`，然后 `attr(opts)` 写入 JS 变换主属性并同步 rust。绘制与命中仍走 wasm-bindgen。

#### `src/` — wasm-bindgen 桥

| 模块 | 职责 |
|------|------|
| `lib.rs` | 导出全部 wasm 类型与 `init` / `dispose` / `registerFont` |
| `zrender.rs` | `ZRender` 类 + 实例表；`init` / `dispose` / `disposeAll` / `getInstance` / `clear` / `setBackgroundColor` / `trigger` / `setCursorStyle` / `configLayer` |
| `animation.rs` | `Animator`：记录最后 `when`，`start()` 写回 shape/style/attr |
| `registry.rs` | `ElementRegistry`：JS 对象 ↔ Storage 索引；父子挂载；延迟 materialize |
| `bridge/opts.rs` | 解析 `{ shape, style, z, zlevel, silent, name, ... }` |
| `bridge/shape.rs` | 各 shape 的 `opts.shape` 字段 |
| `bridge/fill_stroke.rs` | 字符串色 / inline gradient / Gradient 类 / Pattern |
| `bridge/build.rs` | 把 pending 图元写入 Storage |
| `bridge/hit.rs` | `findHover` → `{ target, topTarget }` 的 `HoverResult` |
| `font.rs` | `registerFont` / `clearFonts`，并热更新已有实例 fontdb |
| `graphic/*` | 各 JS 类 |
| `export.rs` | pkg 内仍导出空对象（避免 wasm-bindgen 缺符号）；公开命名空间走 `js/tool/` |
| `graphic/stub.rs` | rust 侧仍导出 `IncrementalDisplayable` 占位（避免 wasm-bindgen 缺符号）；公开构造走 `js/incremental.js` |

#### 顶层函数

| 导出 | 行为 |
|------|------|
| `init(dom?, opts?)` | 创建实例。传入 canvas 时绑定指针事件并自动上屏；否则宽高 / dpr 来自 `opts` |
| `registerFont(data, opts?)` | `Uint8Array` → 全局 fontdb。`opts.familyName`、`opts.sansSerif` |
| `clearFonts()` | 清空字体（测试） |
| `dispose(zr)` / `disposeAll()` | 释放 |
| `getInstance(id)` | 按 id 取回 |

#### `ZRender` 实例方法

| 方法 | 说明 |
|------|------|
| `add(el)` / `remove(el)` / `clear()` | 根节点增删与清空；`init(canvas)` 时自动上屏 |
| `refresh()` | 同步返回 RGBA `Uint8Array` |
| `flush()` | 同 `refresh()`（无动画队列） |
| `resize(opts)` | 改尺寸，不重载任何预设 scene |
| `findHover(x, y)` | `{ target, topTarget }`，均为 wasm `Element` |
| `on(event, handler)` / `off(event, handler?)` / `trigger(event, packet?)` | 实例事件；`off` 可按函数取消 |
| `setBackgroundColor(color)` / `getBackgroundColor()` | 背景色（字符串）；下次 `refresh` 填底 |
| `setCursorStyle(cursor)` | 设置默认光标（有 canvas 时写 `style.cursor`） |
| `configLayer(zlevel, config)` | 无图层配置；触发 refresh |
| `dispose()` | 实例释放（与顶层 `dispose(zr)` 相同） |
| `handler.dispatch(name, { zrX, zrY })` | 无 DOM 时注入指针事件（测试 / Node） |
| `width()` / `height()` / `dpr()` / `id` | 尺寸与实例 id |

#### 已实现图元（可 `new` + `zr.add` / `group.add`）

**容器 / 基类**：`Group`（`new Group(opts)`，`add` / `addBefore` / `replace` / `remove` / `removeAll` / `children` / `childAt` / `childOfName` / `childCount` / `eachChild` / `traverse`）、`Path`（通用，`shape.pathData`）、`Displayable`（抽象，构造抛错说明）。

**Path 子类**（均有 `useState` / `useStates` / `setStateStyle`）：`Rect`、`Circle`、`Line`、`Polygon`、`Polyline`、`Sector`、`Arc`、`Ellipse`、`Ring`、`BezierCurve`、`Isogon`、`Star`、`Heart`、`Droplet`、`Rose`、`Trochoid`、`CompoundPath`。`Path.extend({ buildPath })` 把命令录成 `shape.pathData`。

**其它图元**：`Text`、`TSpan`（单 run MVP）、`Image`、`IncrementalDisplayable`（普通 Group 语义，非增量图层）。

**样式对象**：`LinearGradient(x, y, x2, y2, colorStops?, global?)`（`addColorStop`）、`RadialGradient(x, y, r, colorStops?, global?, r0?)`、`Pattern(image, repeat?)`。

**几何值对象**（不进 Storage）：`Point`、`BoundingRect`、`OrientedBoundingRect`。

构造 opts 对齐官方：`{ shape, style, z, zlevel, silent, name, ignore, draggable, position, x, y, scaleX, scaleY, rotation, originX, originY }`，以及 echarts 用的 `seriesIndex` / `dataIndex` 可写到 ECData。`attr` / `setShape` / `setStyle` 支持对象与 `key, value`。

#### 刻意与官方不同

只允许「目标与约束」里的四条例外。当前进度（未完成的不叫例外）：

- 动画为终态语义：`animate` / `animateTo` / `when().start()` 立刻写入最后一组目标；`during(percent=1)` 与 `done` 会调用；不播中间帧
- `morph` 只返回终点 path（形变后置）；`parseSVG` 只覆盖 g/path/基础图形
- `IncrementalDisplayable` 按普通 Group 语义（`addDisplayable`），不做增量图层
- `Path.extend` 把 `buildPath` 录成 `shape.pathData`，不走完整 PathProxy 引擎循环
- `configLayer` 不改变图层合成，只触发 refresh

#### 浏览器测试

`crates/wasm-zrender/tests/web.rs`：`wasm-bindgen-test`，覆盖 init+Group+Rect 出 RGBA、findHover、渐变、各 shape、Image、Text/字体、几何类、stub 抛错等。字体夹具：`crates/wasm-zrender/tests/fixtures/NotoSansSC-Regular.ttf`。

纯 JS 工具模块：`node crates/wasm-zrender/js/tool/selftest.js`（matrix / vector / color / util / morph / Animator 终态 / PathRecorder / Point 静态方法，不启 WASM）。

```bash
cd wasm-echarts-rs/crates/wasm-zrender
wasm-pack test --node
# 或按环境：--headless --chrome
```

### 如何编译

```bash
cd wasm-echarts-rs/crates/wasm-zrender
wasm-pack build --target web --dev      # 调试，体积大、带符号
wasm-pack build --target web --release  # 发布，opt-level = "s"
```

`--target web` 生成 ES Module，给 Vite / `<script type="module">` 用。`--out-dir` 默认就是 crate 下的 `pkg/`。

### 如何启动（配合文档站）

见下文「推荐工作流」。单独没有 HTTP 服务；必须先有 `pkg/`，再由 `site` 或自己的 HTML import。

### 编译产物在哪里

```
crates/wasm-zrender/pkg/
├── package.json           # name: "wasm-zrender"，main: wasm_zrender.js
├── wasm_zrender.js        # wasm-bindgen 胶水（ESM）
├── wasm_zrender.d.ts      # TypeScript 声明（init / Group / Rect / …）
├── wasm_zrender_bg.wasm   # WebAssembly 二进制
├── wasm_zrender_bg.wasm.d.ts
└── .gitignore / README.md
```

`pkg/` 提交进仓库（胶水 JS / `.wasm` / `package.json`）。`wasm-pack` 每次会把 `pkg/.gitignore` 写成 `*`，该文件本身忽略，不提交。改 Rust 后在本地重新 `wasm-pack` 再提交 `pkg/`。中间缓存仍在 workspace `target/`。

`package.json` 使其**可以**当 npm 包（`file:` 或发布到 registry）。当前文档站**没有**把它写进 `site/package.json` 的 `dependencies`，而是用 Vite alias 直接指到这个目录（见「文档站如何引用产物」）。

### 文档站如何用到本 crate

1. Vite alias `@wasm-zrender` → `crates/wasm-zrender/js`（`js/index.js` 再 import `../pkg/wasm_zrender.js`）
2. 每个实例是独立完整脚本：`site/zrender/examples/shapes.js` 等同名 HTML 成对出现，直接 `import` facade、建 canvas、构图、`refresh` → `putImageData`。canvas 位图与 `init` 宽高一律用 **CSS 像素**，不乘 `devicePixelRatio`（内核缓冲和命中检测都是 CSS 坐标；示例里乘 dpr 会导致画面与点选错位）
3. 画廊 `gallery.js` 用 Vite `?raw` 读这些 `.js` 作为左侧源码，iframe 加载同目录 HTML 预览
4. 字体：`text.js` 内联 `fetch` + `registerFont`；可选辅助 `site/src/zrender/fonts.js` 默认拉取 `/fonts/NotoSansSC-Regular.ttf`

实例页：`/zrender/examples/hello_world.html`、`animation.html`、`bounding_box.html`、`clip_path.html`、`glitched_text.html`、`particles.html`、`shapes.html`、`text.html`、`sector.html`、`hit.html`、`state.html`。

文档：`site/zrender/docs/` 五章多页（快速上手 / 字体 / API facade+pkg / 与官方差异 / 底层原理）。`/zrender/docs/` 默认快速上手；侧栏见 `src/shared/docs-shell.js`。项目缘由与三层定位回链站点首页。

---

## 三、wasm-echarts

### 目的

浏览器侧的 ECharts **canvas 管线**：接收官方形态的 `option`（含 JS 函数字段），在 Rust 里做 merge → GlobalModel → ChartView → `rust_zrender::ZRenderer`，再把 RGBA 交还给 JS。公开 JS 表面对齐官方 `init` / `setOption` / `on` / `dispatchAction`（见上文硬规则）。canvas 能力按 [全量对齐计划](../.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md) 补齐。

**当前源码**：wasm-echarts path 依赖 wasm-zrender（rlib），`getZr()` 与 ChartView 共用同一份 Storage；echarts 运行时只加载一份 WASM。native wasm-bindgen 类 `EChartsInstance` 是内部 handle，不是公开 API；由 `js/` facade 包装。site 从 `@wasm-echarts`（`js/index.js`）导入，写法为官方 `init` / `setOption`。

Cargo features（默认全开）：

```
chart-line, chart-bar, chart-pie, chart-scatter,
chart-radar, chart-gauge, chart-candlestick, chart-boxplot, chart-heatmap,
chart-pictorial, chart-effect-scatter, chart-funnel, chart-chord, chart-sunburst,
chart-tree, chart-treemap, chart-graph, chart-sankey, chart-theme-river,
chart-map, chart-lines, chart-parallel, chart-custom
```

可按需裁剪以减小 WASM 体积。`echarts.use` **不为体积做动态加载**（见硬规则）。

### 实现口径（随实现更新；公开文档七章见 `site/echarts/docs/`）

#### 1. 与官方一致的公开 API

可按官方文档调用（签名对齐）。

| 导出 / 方法 | 签名 |
|-------------|------|
| `init` | `init(canvas, theme?, opts?)`；`init(null, null, { width, height, devicePixelRatio })` 允许离屏。非官方 `opts.useWorker` 见例外 |
| `dispose` | `dispose(chart \| canvas \| id)` |
| `getInstanceByDom` / `getInstanceById` | 按 canvas / id 取回实例 |
| `version` | `'6.1.0'` |
| `use` | 导出且签名对齐；只 `console.info`，不加载模块 |
| `setOption` | `setOption(option)` / `setOption(option, notMerge, lazyUpdate?)` / `setOption(option, { notMerge, replaceMerge, silent, lazyUpdate })`。默认同线程同步；`useWorker` 时返回 Promise |
| `getOption` | 把已合并 option 转回普通 JSON；函数字段保留为原 Function |
| `resize` | `resize()` 无参读 canvas；`resize({ width, height, devicePixelRatio })`；`'auto'` 回退到 canvas；亦接受 native `resize(w, h, dpr)` |
| `clear` | `setOption({ series: [] }, true)` |
| `getWidth` / `getHeight` / `getDevicePixelRatio` / `isDisposed` / `dispose` | 实例方法 |
| `dispatchAction` | 已接线 type：`highlight` / `downplay` / `select` / `unselect` / `toggleSelect` / `dataZoom` / `showTip` / `hideTip` / `legendToggleSelect` / `legendSelect` / `legendUnSelect` / `restore` / `timelineChange` / `timelinePlayChange` / `takeGlobalCursor` / `brush` / `brushEnd` / `expandAxisBreak` / `collapseAxisBreak` / `toggleAxisBreak`；`registerAction` 登记的自定义 type 也会调用 |
| `getDom` / `getId` | `init` 后可取 |
| `getZr` | `getZr()` 返回与 ChartView 共用 Storage 的 wasm-zrender 实例（同一份 WASM，不是第二份 `wasm-zrender/pkg`）；可 `add` / `on` / `configLayer`。无 SVG painter / hover layer；动画终态。`useWorker` 时不可用（见例外） |
| `showLoading` / `hideLoading` | 静态半透明遮罩 + 文案；不播旋转动画 |
| `getDataURL` / `renderToCanvas` / `getRenderedCanvas` | 离屏 RGBA → PNG/JPEG 数据 URL / 画到 canvas；`getRenderedCanvas` 等价 `renderToCanvas`；`type: 'svg'` 不支持 |
| `containPixel` | cartesian 多 grid；finder 可指定 `gridIndex` / `seriesIndex` / 轴 |
| `appendData` | `{ seriesIndex, data }` 追加到 `series.data` 后重绘 |
| `connect` / `disconnect` | 按 `chart.group` 转发 `dispatchAction` |
| `registerTheme` / `setTheme` | 主题表；`init(dom, theme)` 与首次 `setOption` 把主题当默认项合并 |
| `registerMap` / `getMap` / `parseGeoJSON` | JS 表 + WASM 表；`geo` 组件按已注册 GeoJSON 画区域。`parseGeoJSON` / `parseGeoJson` 返回 `{ name, center, polygons }` |
| `registerTransform` | 内置 `filter` / `sort`；外部 `{ type, transform }` 登记后可被 dataset 调用 |
| `registerPreprocessor` / `registerProcessor` / `registerLayout` / `registerVisual` | 真表；preprocessor 在 `setOption` 前改 option；processor 同样在入 WASM 前跑；layout/visual 在 setOption 后跑。`PRIORITY` 与官方同名 |
| `registerAction` | 真表；`dispatchAction` 先调登记 handler，内建 type 仍走 native |
| `registerCoordinateSystem` | 真表；`create(ecModel, api)` 的实例若有 `dataToPoint` / `pointToData` / `containPoint`，`convertToPixel` / `containPixel` 会用 |
| `registerCustomSeries` | 真表；登记的 `series.type` 当 custom，缺 `renderItem` 时注入登记函数 |
| `setPlatformAPI` | 真表存储（WASM 不用 DOM canvas 2d，调用方可读回） |
| `graphic` / `util` / `number` / `time` / `format` / `helper` / `matrix` / `vector` / `color` / `env` / `throttle` | 命名空间已导出；`throttle(fn, delay?, debounce?)` 按官方签名重写 |

#### 2. 与官方不一致 / 例外

| 项 | 现状 / 约定 |
|----|-------------|
| `use(...)` | 导出但不按需加载；只提示「已编进 WASM，不必 use」 |
| `init(null)` | 允许离屏（官方客户端无 dom 会抛错） |
| 动画 | 不播中间帧；`lazyUpdate` 同步执行。`universalTransition` / `echarts.use(UniversalTransition)` 只跳终态，不做系列间插值 |
| 渲染 | 仅 canvas；无 SVG / `renderToSVGString` / `getSvgDataURL`。`zr.painter.getSvgDom` 只 `console.warn` |
| 字体 | 须从 `@wasm-echarts` 调 `registerFont`；WASM 不读系统字体。echarts 页只一份 WASM，与 `getZr` 共用 fontdb。zrender 文档站的 `wasm-zrender/pkg` 仍是另一份内存 |
| `notMerge` | 只作 `setOption` 第二参数；写在 option 根上不会当合并开关 |
| `replaceMerge` | 只按**顶层 key**整段替换，不按 component `id`（比官方弱） |
| `lazyUpdate` / `silent` | 同步 flush，忽略排队与静默 |
| 指针重绘 | `mousemove` 仅在 hover / axisPointer / 拖拽导致画面变化时 `refresh`+`putImageData`。同一数据项上滑动只更新 tooltip DOM，不上屏。无 dirty-rect / hover layer |
| `showLoading` / `hideLoading` | 导出；静态半透明遮罩 + 文案，不播旋转动画 |
| `getZr()` | 同一份 WASM 上的 wasm-zrender 实例，不是第二份 pkg。无 SVG painter（`painter.getSvgDom` 只 warn）/ hover layer / dirty rect；动画写终态 |
| default export | wasm-bindgen `initWasm`，不是 echarts 命名空间对象 |
| tooltip DOM | facade 内建简单 string HTML；不是官方 TooltipView；HTMLElement formatter 未实现 |
| DataView / SaveAsImage | toolbox DataView 浮层与 SaveAsImage 的 DOM 下载条明确不做 |
| 扩展 StageHandler | `registerProcessor` / `Layout` / `Visual` 拿到简化 `ecModel`（`getOption` / `eachSeries`），不是官方 GlobalModel / List |
| LabelLayout | 终态 AABB `hideOverlap` / `moveOverlap`（shiftX/Y、shuffleX/Y）与 `dx`/`dy`；不是官方完整 LabelManager / OBB |
| AxisBreak | `axis.breaks` 折叠比例尺 + 轴上折线标记；`expandAxisBreak` 等改 `isExpanded` 后终态重绘，无展开动画 |
| ScatterJitter | `axis.jitter` 终态错开；`jitterOverlap: true` 用稳定哈希而非 `Math.random` |
| `opts.useWorker` | **非官方**。默认 `false`：同线程同步 `setOption`。`true` 时 Worker 持 `EChartsInstance`，主线程只 blit / 指针 / tooltip DOM。无 `Worker` 时 `console.warn` 并回退主线程。不要自己 `new Worker`。画廊仅 `bar-large` / `scatter-large` / `candlestick-large` / `parallel-nutrients` 打开 |
| Worker 函数 option | `formatter` / `renderItem` / `itemStyle.color` 等函数不能结构化克隆。含函数的 option **必须**默认主线程 `init`；WASM 须与定义这些函数的 JS 在同一 realm。`setOption` / `dispatchAction` 遇到函数会抛错并提示走主线程 |
| Worker `setOption` | 返回 Promise（`await chart.setOption(option)`）。option（含 `series.data` / `dataset.source` 里的 TypedArray）一律 `postMessage` 克隆，**不用 SharedArrayBuffer** |
| Worker `getZr()` | ZRender 在 Worker 里；主线程 `getZr()` 只 `console.warn` 并返回 `undefined` |
| SharedArrayBuffer | **option 方向一律不用 SAB**。SAB 只用于 Worker → 主线程回传 RGBA（可双缓冲）。站点需 Cross-Origin Isolation：`Cross-Origin-Opener-Policy: same-origin` + `Cross-Origin-Embedder-Policy: require-corp`（Vite `server` / `preview` 已配）。不可用时回图 fallback Transferable `ArrayBuffer`。同线程 `init(canvas)` 不强制 SAB |

#### 3. 多出来的非官方 API

官方没有、因 WASM 离屏需要而多出来。**不要当官方 API 用**。`init(canvas)` 后普通用户不必再手动 `putImageData`，也不必再调指针 hatch 或自绘 tooltip。

| facade camelCase | native | 用途 |
|------------------|--------|------|
| `init` | `EChartsInstance` 构造 | 公开入口；native 构造不要直接用 |
| `refresh` | `refresh` | 返回 RGBA；离屏无 canvas 时用 |
| `findHover` | `find_hover` | 命中检测 |
| `handlePointerMove` / `handlePointerLeave` / `handlePointerDown` / `handlePointerUp` / `handlePointerClick` | `handle_pointer_move` / `handle_pointer_leave` / `handlePointerDown` / `handlePointerUp` / `handlePointerClick` | 非官方；`init(canvas)` 时一般不需要 |
| `applyDataZoomWheel` | `apply_data_zoom_wheel` | 非官方；`init(canvas)` 时滚轮已绑定 |
| `getTooltipContent` | `get_tooltip_content` | 自绘 tooltip；`init(canvas)` 时 facade 已内建 string DOM |
| `benchmarkRender` | `benchmark_render` | 渲染耗时 |
| `hasOption` / `optionHasFunctions` | `has_option` / `option_has_functions` | 状态查询 |
| `registerFont` / `clearFonts` | `registerFont` / `clearFonts` | WASM 字体例外；轴标签 / series label 渲染前必调；已有实例热更新 fontdb；`useWorker` 时同一份 bytes 会 `postMessage` 进 Worker |
| `opts.useWorker` | `ChartWorkerBridge` / `chart.worker.js` | 把实例放到 Worker；开发者仍写 `init` / `setOption` / `on`，不必自己 `new Worker` |

#### 4. 已实现 / 未实现

**已实现（部分生效）**

| 范围 | 现状 |
|------|------|
| 图表 | 官方 `export/charts.ts` 23 种（含 chord）均已接线 canvas 终态（feature flag 写死在 rust，不是 `echarts.use(BarChart)`）；默认全开 |
| cartesian | 多 `grid` / `xAxis[]` / `yAxis[]`（`gridIndex` / `xAxisIndex` / `yAxisIndex`）；`type: category \| value \| time \| log`；`breaks` 折叠未展开区间；`jitter` 给 scatter 终态错开；y 类目 + x 数值时横画（bar 水平柱、line 对调）；`convertToPixel`/`convertFromPixel`/`containPixel` 按 finder 取轴 |
| dataset | `source`（二维数组 / 对象行 / 列对象）/`dimensions` / `seriesLayoutBy` / `datasetIndex` / `datasetId` / `encode`；无 `series.data` 时从 dataset 取数 |
| transform | 内置 `filter` / `sort`（`fromDatasetIndex` / `fromDatasetId`）；`registerTransform` 接外部 JS |
| stack / sampling | `stack` + `stackStrategy`（samesign/all/positive/negative）line 与 bar 共用偏移；`sampling: lttb \| average` 按 grid 宽度降采样 |
| line | Polyline；`smooth`（bool→0.5 或数字）/`step`（start/middle/end）/`connectNulls`（`null`/`'-'` 保留为 NaN 缺口）；`areaStyle`（默认透明度 0.7，色可为 `LinearGradient`，`origin`：auto/start/end/数值）；`lineStyle.width`/`type`（solid/dashed/dotted 或 dash 数组）；`endLabel.show`；`symbol`/`symbolSize`；`stack`/`sampling`；value/time/log 用 `x_value`；`coordinateSystem: 'polar'` 按 [radius, angle] 画 |
| bar | Rect（cartesian，含 y 类目水平柱）；polar 下为 Sector；同轴多系列并排（同 `stack` 共用列）；`barWidth`/`barGap`（默认 10%）/`barCategoryGap`/`barMinHeight`；`itemStyle.borderRadius`；`stack` 从 `stack_base` 画到 `stacked_value` |
| pie | Sector；`center`/`radius`/`startAngle`/`clockwise`；`roseType: radius\|area`；`selectedMode` + `selectedOffset`（含 data.`selected`）；`minShowLabelAngle`；label 同侧 y 间距跳过（不是官方完整避让）+ `labelLine` |
| scatter | 双 value 轴；也可挂 polar / geo / calendar / single / matrix；`symbol`（circle/rect/roundRect/triangle/diamond/pin/arrow/star/line 及 empty*）；`symbolSize`（默认 10）；`large` 且点数 ≥ `largeThreshold`（默认 2000）时终态一次画完、不挂 emphasis 状态、不画 label；类目轴 `jitter` / `jitterOverlap` / `jitterMargin` |
| radar | 按 `radar.indicator` 把 `data[].value[]` 投到蛛网，画 Polygon；可读 `areaStyle` / `lineStyle` |
| gauge | 轴环 Sector + 指针终态 + detail/label；`min`/`max`/`startAngle`/`endAngle`/`center`/`radius` |
| candlestick / boxplot | cartesian；K 线 `[open,close,low,high]`（或带 x 的 5 项）；箱线 `[min,Q1,median,Q3,max]`；涨跌色 `itemStyle.color`/`color0` |
| heatmap | cartesian / calendar / matrix / geo 格子按值上色（visualMap `inRange.color` / pieces） |
| pictorialBar | 按柱高重复或缩放 `symbol`（终态） |
| effectScatter | scatter + `rippleEffect.scale`/`number` 同心圆终态（不播扩散动画） |
| funnel | 梯形；`sort` / `minSize`/`maxSize` / `gap` |
| tree / treemap / sunburst | 从 `data`/`children` 递归；树分层、矩形树切分、旭日多层 Sector |
| graph / chord / sankey | `data`/`nodes` + `links`/`edges`；circular/坐标点、和弦贝塞尔、桑基分层贝塞尔；边 `lineStyle.color` 支持 `source` / `target` / `gradient` |
| themeRiver | `singleAxis` 上按名称堆叠面积带 |
| map / lines | 按已注册 GeoJSON 填色；`coords` 折线（geo 或 cartesian） |
| parallel | 平行坐标每条数据一条 Polyline |
| custom | `renderItem(params, api)`；`api.coord`/`size`/`style`/`value`；产出 graphic 进同一 Zr |
| tooltip.formatter | 返回 string 时可用；CallbackDataParams 含 `componentType`/`seriesType`/`percent`（pie）/`data` |
| series.label | `label.show` + formatter（`{a}`/`{b}`/`{c}`/`{d}` 或函数）画 Text；`labelLayout` 终态 `hideOverlap` / `moveOverlap` / `dx`/`dy` |
| 轴标签 | `axisLabel.formatter` 函数或 `'{value}'` 模板真正进 Text |
| emphasis/select | 图元 state + `dispatchAction` 六个 type |
| dataZoom | `type: 'inside'` 滚轮改 start/end；`type: 'slider'` canvas 手柄/填充条可拖；读 `xAxisIndex`（只缩放对应轴） |
| axisPointer | 竖线或 `type: 'cross'` 十字 + 轴上 label；`tooltip.trigger: 'axis'` 按类目汇总各系列 |
| legend | 色块 + 系列名；`orient` / `selected`；点击筛选（灰显并隐藏系列）；`dispatchAction` `legendToggleSelect` 等 |
| title | `text` / `subtext`；`padding` / `left`/`right`/`top`/`bottom` / `textAlign` / `textVerticalAlign` / `itemGap` |
| mark* | `series.markPoint` / `markLine` / `markArea`（`type: min\|max\|average\|median`、`coord`、`xAxis`/`yAxis`） |
| `option.graphic` | rect/circle/text/group 等画进 ChartView 根组，与 `getZr` 共用 Storage |
| visualMap | continuous 色条；piecewise 色块 + 点击筛选；按 `pieces` / `inRange.color` 给 series 上色 |
| toolbox | canvas 按钮：`restore` / `magicType`（line↔bar）/ `dataZoom` 框选缩放；`dataView` / `saveAsImage` 只 `console.warn` |
| timeline | canvas 滑条；`currentIndex` 把 `options[i]` merge 进 `baseOption` 后终态重绘 |
| brush | canvas 框选矩形，松开后 `select` 落入点 |
| thumbnail | cartesian 缩略 + 窗口拖动改 dataZoom |
| 指针 / tooltip | `init(canvas)` 绑 mousemove/click/leave/wheel（mousemove 合入 rAF）；hover 未变不上屏；内建 string tooltip DOM；`on`/`off` 发出 `click`/`mouseover`/`mouseout`/`globalout` |
| Worker | 可选 `opts.useWorker`：Worker 持实例，主线程 blit/事件/tooltip；option `postMessage`（不用 SAB）；回图优先 SAB（需 COOP/COEP） |
| showTip / hideTip | `dispatchAction({ type: 'showTip', seriesIndex, dataIndex })` 或 `{ x, y }`；`hideTip` 关 DOM，不改 hover |
| `getZr` / 命名空间 | `getZr()` 返回与 ChartView 共用 Storage 的 wasm-zrender 实例（同一份 WASM）；`graphic`（含 `LinearGradient`/`Rect`）/`util`/`time`/`format`/`number`/`helper`/`matrix`/`vector`/`color`/`env`；`throttle`。无 SVG painter / hover layer；动画终态 |
| Loading / 导出图 | `showLoading` 静态遮罩；`getDataURL` / `renderToCanvas`（canvas PNG/JPEG） |
| 主题 / 地图表 / connect | `registerTheme`/`setTheme`；`registerMap`/`getMap`/`parseGeoJSON`；`geo` 画已注册地图；`connect` 转发 action |
| `appendData` / `containPixel` | 追加 series.data；`containPixel` 按 finder 查对应 grid / polar / geo 等 |
| media | `option.media` 按 `minWidth`/`maxWidth`/`minHeight`/`maxHeight`/`aspectRatio` 匹配；后者优先；无匹配用无 query 的默认项；`resize` 重算 |
| 扩展注册 | `registerPreprocessor` / `Processor` / `Layout` / `Visual` / `Action` / `CoordinateSystem` / `CustomSeries` 真表；`PRIORITY` |

**未实现（不是永久例外；probe 失败与视觉简化）**

已导出同名、内部只 `console.warn`：`registerLocale`、`convertToLayout`、`getVisual`、`renderToSVGString`、`getSvgDataURL`、`getModel`（官方为 private）。`getConnectedDataURL` 不按 connect 拼图，回退 `getDataURL`。`isSSR()` 恒为 `false`。`updateLabelLayout` 为空操作（labelLayout 已在 `setOption` 终态做完）。

Charts：未识别的 `series.type` 会 `console.warn`（不再静默当 `Other`；`registerCustomSeries` 登记过的 type 当 custom）。已接线图表的视觉/布局相对官方仍有简化（force 布局不迭代、桑基非完整节点平衡、tree 非 tidy、chord 用贝塞尔而非丝带、pictorialBar 非全部 symbolClip 语义）。大树 treemap / `levels` 色映射、bar/candlestick `large`、parallel `progressive`、缺 `ecStat` / `bmap`、未同步的 `lines-ny` bin 见下文全量 probe 表。

Components：geo roam / SVG 地图源。aria 写 DOM 属性非绘制，可后置。toolbox DataView DOM / SaveAsImage 下载条明确不做。

Actions：geo roam 等。

其它：完整 SeriesData。`smoothMonotone` 未接。pie label 不是官方完整 `avoidLabelOverlap` / `alignTo`。scatter `large` 仍每点一个 Path（跳过状态与 label，不是 IncrementalDisplayable）。bar 未接 `large`（50 万柱无增量绘制；画廊示例走 `useWorker` 避免主线程无响应）。`UniversalTransition` 只终态。

**名字在 option 里出现但未按官方做：** `smoothMonotone`；色板不是官方 palette 全套；pie 完整标签避让 / `padAngle` / `alignTo`；bar `showBackground`、polar `roundCap`、`realtimeSort` 动画（只终态）；parallel `progressive`；rich text 标签当普通 Text。

**官网全量 probe（第 8.9 波）**：27 类 `official-*-catalog.js` 去重 296 条，281 `ok`，3 timeout，12 error。失败（不改官方 option、不在 `official-env.js` 假实现）：

| 示例 | 原因 |
|------|------|
| `bar-large` | 50 万柱 + `large: true`；bar 无增量/采样绘制。示例已 `useWorker`，主线程不再卡死；Worker 内仍是同步长任务，probe 40s 仍可能 timeout |
| `candlestick-large` | 20 万 OHLC；无增量/采样绘制。示例已 `useWorker`，主线程不再卡死；Worker 内仍是同步长任务，probe 40s 仍可能 timeout |
| `parallel-nutrients` | 官网 `nutrients.json` 约 1.4 万行；parallel 每条数据一条 Path、未接 `progressive: 500`。示例已 `useWorker`，主线程不再卡死；Worker 内仍是同步长任务，probe 40s 仍可能 timeout |
| `scatter-clustering` / `scatter-clustering-process` / `scatter-exponential-regression` / `scatter-linear-regression` / `scatter-polynomial-regression` / `scatter-logarithmic-regression` | 依赖官网统计插件 `echarts-stat` 全局 `ecStat`；本宿主未注入 |
| `bar-histogram` | 同上，缺全局 `ecStat` |
| `heatmap-bmap` | 依赖百度地图扩展 `bmap`；`getModel` 已导出只 warn 并返回 `undefined`，表面错误变为 `getComponent` of undefined。即便返回模型也会因无 bmap 失败（`effectScatter-bmap` 的 `setOption` 不抛，probe 判 `ok`、底图空） |
| `lines-ny` | 官网 `links_ny_*.bin` 分片 URL 为字符串拼接，sync 未拉到资源；404 HTML 被当 `arraybuffer`，`Float32Array` 报 byte length 不是 4 的倍数 |
| `treemap-disk` / `treemap-show-parent` | 官网 `disk.tree.json` 大树；递归 parse / squarify 触发 WASM panic（`unreachable`） |
| `treemap-visual` | `series.levels.color` + `visualDimension` 渐变映射未按官方做，渲染时 WASM panic（`unreachable`） |

本轮相对此前分波记录的变化：`scatter-weibo` 与 `map-usa-projection` 现为 `ok`（后者在 CDN `d3-geo` 可加载时 `d3.geoAlbersUsa` 可用；不在 env 里假实现 d3）。gauge 12 + radar 5 全部 `ok`。

SVG 地图源仍未实现（`geo-svg-*` / `geo-beef-cuts` 等 probe 判 `ok` 因 `setOption` 不抛，底图为空）。`map: 'china'` / `'world'` 未预注册（`geo-lines` / `lines-airline` 等同样 `ok`、底图空）。graphic 描边/波浪/loading 因动画终态会跳到最后一帧。

`$` 宿主有 `get` / `getJSON` / `getScript` / `when`。不在 `official-env.js` 里假实现 `bmap` / `ecStat`。缺官方实例方法改为同名导出 + `console.warn`，避免 `xxx is not a function`。

画廊 catalog 条数（含跨组重复，gallery 按先声明组去重）：折线 40、柱状 46、饼图 19、散点 36、K 线 10、盒须 4、热力 7、象形柱 8、仪表盘 12、雷达 5、漏斗 4、和弦 4、旭日 7、树图 7、矩形树 7、关系图 13、桑基 7、主题河流 2、日历 9、矩阵 14、平行坐标 4、地图 25、地理 1、路径图 5、自定义系列 20、数据集 9、图形组件 5。

### 实现了哪些内容

源码：`crates/wasm-echarts/js/`（公开 facade）与 `crates/wasm-echarts/src/`（native）。

#### `js/` — 手写 facade（site 唯一公开入口）

| 文件 | 职责 |
|------|------|
| `index.js` | 官方命名导出；`default` 仍是 `initWasm`；`version = '6.1.0'`；另导出 `registerFont` / 命名空间 / `throttle` / `parseGeoJSON` / 扩展注册 |
| `echarts.js` | `init` / `dispose` / `use`；`connect` / `registerTheme` / `registerMap` / `parseGeoJSON` / `registerTransform` / 扩展注册再导出；命名空间与 `throttle` 再导出 |
| `extension.js` | `registerPreprocessor` / `Processor` / `Layout` / `Visual` / `Action` / `CoordinateSystem` / `CustomSeries` / `PRIORITY` / `setPlatformAPI` |
| `instance.js` | camelCase 实例；`getZr`（wasm-zrender，无 SVG painter）/ `showLoading` / `getDataURL` / `getRenderedCanvas` / `appendData` / `setTheme` / `containPixel`；缺实现的官方方法 `console.warn`；指针与 tooltip；`useWorker` 时委托 `ChartWorkerBridge` |
| `worker-protocol.js` | 主线程 / Worker 消息名；option 含函数检测；SAB 探测。option 方向不用 SAB |
| `worker-bridge.js` | 主线程 Worker 桥：创建 module Worker、RPC、SAB/Transferable blit、字体与地图同步。开发者不必 `new Worker` |
| `chart.worker.js` | Worker 内 `initWasm` + `EChartsInstance`；option 只收 `postMessage`；RGBA 优先写 SAB，否则 Transferable |
| `graphic.js` / `util.js` / `number.js` / `time.js` / `format.js` / `helper.js` / `env.js` / `throttle.js` | `export/api.ts` 命名空间与 `throttle` |
| `native.js` | 加载 `pkg/wasm_echarts.js` 并 `setNative` 注入 wasm-zrender |
| `wasm_echarts.js` | 兼容旧路径 `@wasm-echarts/wasm_echarts.js` |

#### 字体（复用 wasm-zrender 绑定）

`registerFont` / `clearFonts` 来自编入同一份 WASM 的 wasm-zrender 绑定：写入 `rust_zrender` 全局 fontdb，并刷新 `ZR_REGISTRY` 里所有实例。echarts 页与 `getZr` 共用这一份 fontdb；不要从 `@wasm-zrender` 入口再加载第二份 wasm。facade 在注册后仍遍历实例表调用 `update_font_database`。

#### `instance.rs` — `EChartsInstance`（wasm-bindgen 内部 handle）

| 方法 | 说明 |
|------|------|
| `new(width, height, dpr)` | 创建 `ZRenderer` 并登记进 `ZR_REGISTRY` |
| `get_zr()` / `attach_host(dom)` | 同一 id 的 `ZRender`；canvas 绑到 zrender Handler |
| `set_option(option, opts?)` | 解析 + merge（`notMerge` / `replaceMerge` 来自第二参数），全量 render |
| `get_option()` | 已合并 option 转回 JsValue（函数保留） |
| `refresh()` | RGBA |
| `update_font_database()` | 把全局 fontdb 同步到本实例；`registerFont` 后由 facade 调用 |
| `resize(w, h, dpr)` | 改画布并重绘 |
| `find_hover(x, y)` | `{ seriesIndex, dataIndex, pathIndex, ... }` |
| `handle_pointer_move(x, y)` | hover 高亮 + axisPointer + tooltip 文案；返回 `{ hit, tooltip, axisPointer, dirty }`。hover/axisPointer/拖拽未变时 `dirty: false`，不重建场景 |
| `handle_pointer_leave()` | 取消 hover / 结束拖拽 |
| `handlePointerDown` / `handlePointerUp` / `handlePointerClick` | slider / thumbnail / brush 拖拽；legend / toolbox / timeline / visualMap 点击 |
| `get_tooltip_content(si, di)` | 调 `tooltip.formatter`，string 或 null |
| `apply_data_zoom_wheel(x, deltaY)` | option 含 **inside** dataZoom 时缩放窗口（尊重 `xAxisIndex`） |
| `dispatch_action(action)` | 见下表 |
| `append_data` / `contain_pixel` | 追加 series.data；grid 是否包含像素 |
| `benchmark_render(n)` | 全量 render+refresh 平均毫秒 |
| `has_option()` / `option_has_functions()` / `dispose()` | 状态与释放（dispose 从 registry 卸 Zr） |
| `width()` / `height()` / `dpr()` | 尺寸 |
| `convert_to_pixel` / `convert_from_pixel` | cartesian / polar / geo / calendar / single / parallel / matrix / radar finder |

`dispatch_action` 已实现：`highlight`、`downplay`、`select`、`unselect`、`toggleSelect`、`dataZoom`（`start`/`end` 百分比）、`showTip`、`hideTip`、`legendToggleSelect` / `legendSelect` / `legendUnSelect`、`restore`、`timelineChange` / `timelinePlayChange`、`takeGlobalCursor`、`brush` / `brushEnd`。其它 type 会 `console.warn`。

#### `option/` — 解析与合并

- `OptionValue`：Null / Bool / Number / String / Array / Object / **Function**
- `parse_option_value`：递归走 `JsValue`；`LinearGradient` 等 wasm-bindgen 实例按 getter 收成 `type` / `colorStops` 等字段
- `merge_option`：深合并；函数字段用新值覆盖；数组按 index 合并对象
- `setOption` 第二参数：`notMerge` 替换整棵树；`replaceMerge` 顶层 key 整段替换；不再从 option 根读取这些字段
- timeline：`effective_root(index)` 把 `options[index]` merge 进 `baseOption`（或去掉 `options` 的根）

#### `bridge/` — 回调

- `JsCallback`：`call_formatter`、`call_color`、`call_render_item`、`call_axis_formatter`、`call_size`
- `build_data_params`：`componentType` / `seriesType` / `seriesIndex` / `dataIndex` / `seriesName` / `name` / `value` / `data` / `color` / pie `percent` / `$vars`
- `resolve_color` / `resolve_formatter`（`{a}{b}{c}{d}` 模板）/ `resolve_axis_formatter`（`{value}`）/ `resolve_symbol_size`
- `try_call_formatter`：JS throw 时 `console.error` 并降级

**已接线**：`itemStyle.color` / `lineStyle.color`、`tooltip.formatter`（string）、`label.formatter` + 图元、`axisLabel.formatter` 进轴 Text、`symbol` / `symbolSize`、`series.renderItem` + `api.coord`/`size`/`style`/`value`。

**未接线或未完成**：tooltip 返回 HTMLElement；常量回调缓存。

#### `data/` — dataset 管线

- `source`：二维数组（可识别表头）/ 对象行 / 列对象 → `DataTable`
- `transform`：内置 `filter`（`dimension` + `>`/`>=`/`<`/`<=`/`=`/`and`/`or`/`not`）与 `sort`；`registerTransform` 把 JS 函数登记进 WASM
- `encode` / `seriesLayoutBy` / `datasetIndex` / `datasetId`：无 `series.data` 时从 dataset 抽点
- `stack` / `stackStrategy`：同 stack 名的 line/bar 共用偏移
- `sampling`：`lttb` / `average`，点数相对 grid 宽度过大时降采样

#### `model/` + `scheduler.rs` + `render.rs`

- `GlobalModel`：`grids[]`、`x_axes[]` / `y_axes[]`、`polars` / `radars` / `single_axes` / `parallels` / `calendars` / `matrices` / `geos`、`Vec<SeriesModel>`、dataZoom 窗口
- `AxisType`：`category` / `value` / `time` / `log`（`logBase` 默认 10）
- `DataPoint`：`value` / `x_value` / `name` / `raw` / `stack_base` / `stacked_value`；`null` / `'-'` 保留为 NaN（`connectNulls`）
- Scheduler：单次全量 `run_update` → `render_chart`（`Storage::new()` 后 `rematerialize_mounted_roots`，再重建 `__ec_chart_root`，保留 `getZr().add` 的用户图元）

#### `coord/` — cartesian + 第 5 波坐标系

- `Cartesian2D`：按 series 的 `xAxisIndex` / `yAxisIndex` 取轴与 grid；y 类目 + x 数值时 `is_horizontal`
- category / value / time / log → 像素；time 收 ISO 日期或时间戳；log 按 `logBase`
- `PolarCoord`：`angleAxis` / `radiusAxis`，`center` / `radius` / `startAngle` / `clockwise`；data 维 `[radius, angle]`
- `RadarCoord`：`indicator` 轴 + 蛛网；`convertToPixel` 收 `[indicatorIndex, value]`
- `SingleCoord` / `ParallelCoord` / `CalendarCoord` / `MatrixCoord` / `GeoCoord`
- `convertToPixel` / `convertFromPixel` / `containPixel`：cartesian finder 以及 `polarIndex` / `geoIndex` / `calendarIndex` / `singleAxisIndex` / `parallelIndex` / `matrixIndex` / `radarIndex`
- `maps`：`registerMap` / `parseGeoJSON`（含官方 UTF8Encoding 解码）

#### `chart/` — ChartView

| 类型 | 图元 | 说明 |
|------|------|------|
| line | Polyline + Polygon（面积）+ symbol | `smooth`/`step`/`connectNulls`/`areaStyle`/`endLabel`/`lineStyle.width|type`；默认 emptyCircle；`stack`/`sampling`；polar / y 类目 |
| bar | Rect / Sector（polar） | `barWidth`/`barGap`/`barCategoryGap`/`borderRadius`/`barMinHeight`；同轴并排或 stack；y 类目水平柱 |
| pie | Sector | `center`/`radius`/`r0`/`startAngle`/`clockwise`/`roseType`/`selectedMode`；label 最小避让 + labelLine |
| scatter | symbol | 双 value 轴；也可挂 polar/geo/calendar/single/matrix；其余常用 symbol；`large` 终态一次画完 |
| radar | Polygon | `indicator` 维；`areaStyle` / `lineStyle` |
| gauge | Sector + 指针 | `min`/`max`/`startAngle`/`endAngle`；指针终态 |
| candlestick / boxplot | Rect + Line | OHLC / 五数概括；涨跌色 |
| heatmap | Rect | cartesian / calendar / matrix / geo 格子上色 |
| pictorialBar | symbol | 按柱高重复或缩放 |
| effectScatter | symbol + Circle | ripple 同心圆终态 |
| funnel | Polygon | 梯形；`sort`/`gap`/`minSize` |
| tree / treemap / sunburst | Line+symbol / Rect / Sector | `children` 递归 |
| graph / chord / sankey | Line / Bezier / Rect | nodes + links；边色 `source` / `target` / `gradient` |
| themeRiver | Polygon | singleAxis 堆叠带 |
| map / lines | Polygon / Polyline | 已注册地图填色；`coords` |
| parallel | Polyline | 每条数据一条折线 |
| custom | `option.graphic` 同类图元 | `renderItem` + `api.coord`/`size`/`style` |
| 坐标系底图 | polar 圆环与辐线、radar 蛛网、single/parallel 轴、calendar 格子、matrix 格子、geo 多边形 | 有对应 option 组件即画 |
| 组件 | 网格框、splitLine、轴标签/名称、title（padding/align）、legend（orient/selected/点击筛选）、markPoint/Line/Area、`option.graphic`、visualMap、dataZoom slider、十字 axisPointer、toolbox 按钮、timeline、brush 选区、thumbnail | `axisLabel.formatter` 已进 Text；`fontFamily` / `fontSize` / `color` 从 textStyle 读入 |

轴标签走 `ChildRef::Text` 挂到 group，`silent = true`（不抢 hover）。

#### `visual/` + `interaction.rs`

- 色板默认色 + 每点 `resolve_item_color`
- hover / 多选 set、legend 筛选、inside + slider dataZoom、visualMap piecewise 筛选、brush 框选、timeline `currentIndex`、toolbox restore/magicType、axisPointer 十字、`tooltip.trigger: 'axis'`

#### site 示例

每个 `site/echarts/examples/*.js` 都是完整独立脚本，**自己** `initWasm` → 字体 → `init(canvas)` → `setOption`，不再经 `runOfficialExample` 代管。自写示例导入 `{ init, registerFont }`；官网同步示例导入 `* as echarts`，option 正文原样嵌入。[`official-env.js`](wasm-echarts-rs/site/src/echarts/official-env.js) 只提供 `ROOT_PATH` / `CDN_PATH` / `$` / `app` / `sizeCanvas` / `showPreviewError`，**不创建图表**。轴标签 / series label / title / legend 走 cosmic-text，**未注册字体时 `refresh` 会 panic `no default font found`**。多字体示例 `fonts.js` 连续 registerFont 多份文件，再在 `title.textStyle` / `subtextStyle` / `legend.textStyle` / `nameTextStyle` / `axisLabel` 上设不同 `fontFamily`。有 canvas 时自动上屏并绑指针。交互合集 `interactive.js` 调 `use()`（只 `console.info`）并用 `on('click')` + `dispatchAction('toggleSelect')`；tooltip / hover / wheel 由 facade 消化。`merge.js` 对照深合并与 `notMerge: true`，并走 dispose 后再 init。大数据卡死例（`bar-large` / `scatter-large` / `candlestick-large` / `parallel-nutrients`）用 `init(canvas, null, { useWorker: true })` 并 `await setOption`；其余走默认主线程。不要把 `EChartsInstance` / `set_option` / `handlePointerMove` 当公开 API，也不要在示例里自己 `new Worker`。文档四块在 `site/echarts/docs/index.html`。可选辅助 `site/src/echarts/fonts.js`（`ensureDefaultFont`）默认拉取 `/fonts/NotoSansSC-Regular.ttf`。Windows 本机可把 `C:\Windows\Fonts\msyh.ttc`、`simsun.ttc`、`simkai.ttf` 复制到 `site/public/fonts/`（微软字体不要提交 git）。

浏览器测试 `crates/wasm-echarts/tests/web.rs` 目前几乎是占位（`1+1=2`）。Rust 单测在 `option` / `model` / `interaction` / `pie` 等模块内。

```bash
cd wasm-echarts-rs
cargo test -p wasm-echarts
```

### 如何编译

```bash
cd wasm-echarts-rs/crates/wasm-echarts
wasm-pack build --target web --dev
wasm-pack build --target web --release
```

只编部分图表：

```bash
wasm-pack build --target web --dev -- --no-default-features --features "console_error_panic_hook,chart-line,chart-bar"
```

### 如何启动

同样通过 `site`。公开写法：

```javascript
import initWasm, { init, registerFont } from '@wasm-echarts';

await initWasm();
const bytes = new Uint8Array(
  await (await fetch('/fonts/NotoSansSC-Regular.ttf')).arrayBuffer()
);
registerFont(bytes, { familyName: 'Noto Sans SC', sansSerif: ['Noto Sans SC'] });

const chart = init(canvas);
chart.setOption({
  xAxis: { type: 'category', data: ['A', 'B', 'C'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', data: [10, 20, 30] }],
});
```

`init(canvas)` 后 `setOption` 会自动 `putImageData`，不必再调 `refresh`。离屏用 `init(null, null, { width, height })`，再 `chart.refresh()` 拿 RGBA。有 canvas 时 facade 绑定指针：hover 高亮、string tooltip DOM、legend/toolbox/timeline 点击、slider/brush 拖拽、wheel inside dataZoom；用户 `chart.on('click', handler)` 即可。指针移动时若 hover / axisPointer / 拖拽画面没变，跳过整图 `refresh`（tooltip 仍随鼠标走）。

大数据、会卡死主线程时开 Worker（仍不必自己管 Worker）。`setOption` 返回 Promise；option 必须可结构化克隆（**不要**把 `formatter` / `renderItem` 等函数塞进去）：

```javascript
const chart = init(canvas, null, { useWorker: true });
await chart.setOption(option);
```

Worker 路径硬规范：option（含 TypedArray）一律 `postMessage`，**不用 SAB**；SAB 只用于 Worker → 主线程回图，站点需 COOP/COEP（见 Vite 配置）。不可用时回图 fallback Transferable `ArrayBuffer`。`getZr()` 在此模式下不可用。

### 编译产物在哪里

```
crates/wasm-echarts/pkg/
├── package.json           # name: "wasm-echarts"
├── wasm_echarts.js
├── wasm_echarts.d.ts      # EChartsInstance + registerFont / clearFonts + default init
├── wasm_echarts_bg.wasm
└── wasm_echarts_bg.wasm.d.ts
```

`pkg/` 提交进仓库。改 Rust 后必须在本地重新 wasm-pack 并提交更新后的 `pkg/`，浏览器硬刷新。公开 import 走 `js/`，`pkg/` 只作内部 handle。

### 文档站如何用到本 crate

1. Vite alias `@wasm-echarts` → `crates/wasm-echarts/js`（`js/index.js` 再 import `../pkg/wasm_echarts.js`）
2. 每个实例 JS：

```javascript
import initWasm, { init, registerFont } from '@wasm-echarts';
```

再 `await initWasm()` → `registerFont` → `init(canvas)` + `setOption`。native `EChartsInstance` 仍从 facade 再导出，仅兼容旧路径，不要当公开 API。
3. 每个实例是独立完整脚本：`site/echarts/examples/line.js` 等同名 HTML 成对出现（`<canvas id="canvas">`）；自写示例内联 `fetch` + `registerFont`（与 zrender `text.js` 相同）。**不再**经 `runOfficialExample` 代管。
4. 画廊 `gallery.js` 用 Vite `?raw` / `import.meta.glob` 读这些 `.js` 作为左侧源码，iframe 加载同目录 HTML 预览。echarts 画廊是二级菜单：第一层为图形类别，第二层为该类别下的示例。分组顺序与自写条目在 [`official-gallery-meta.js`](wasm-echarts-rs/site/src/echarts/official-gallery-meta.js)；官网同步条目来自 `official-{category}-catalog.js`（无 catalog 且无自写示例的类不出现空菜单）。已同步：折线 40、柱状 46、饼图 19、散点 36、K 线 10、盒须 4、热力 7、象形柱 8、仪表盘 12、雷达 5、漏斗 4、和弦 4、旭日 7、树图 7、矩形树 7、关系图 13、桑基 7、主题河流 2、日历 9、矩阵 14、平行坐标 4、地图 25、地理 1、路径图 5、自定义系列 20、数据集 9、图形组件 5。官网同步脚本自己 `initWasm` → `ensureDefaultFont` → `echarts.init(canvas)` → `setOption`（大数据卡死例为 `echarts.init(canvas, null, { useWorker: true })` 并 `await setOption`）；[`official-env.js`](wasm-echarts-rs/site/src/echarts/official-env.js) 只提供 `ROOT_PATH` / `CDN_PATH` / `$`（含 `get` / `getJSON` / `getScript` / `when`）与 `app` / `sizeCanvas` / `showPreviewError`，**不创建图表、不补齐未实现 echarts API**，报错显示在预览层。数据文件在 `public/echarts-official/`。按类拉取：`node scripts/sync-official-examples.mjs --category scatter --local-dir <echarts-examples 根目录>`（`--local-dir` / `ECHARTS_EXAMPLES_DIR` 优先读本地 `public/`，避免官网大文件超时；已同步条目改模板用 `--rewrite-existing`）；探测：`node scripts/probe-official-examples.mjs`（看 canvas 是否画出像素与 `.preview-error`；省略 `--category` 即全量；`--out file.json` 写报告；需已启动 Vite；单例超过 40s 判 timeout 并换浏览器进程）。旧文件名 `sync-official-line-examples.mjs` / `probe-official-line.mjs` 转发到 `--category line`。交互合集下挂 interactive / merge / bench。zrender 画廊仍用扁平 `examples`。
5. 页面：自写 line / fonts / bar / pie / scatter / interactive / merge / bench，外加已同步的官网 catalog（折线 40 + 柱状 46 + 饼图 19 + 散点 36 + K 线 10 + 盒须 4 + 热力 7 + 象形柱 8 + 仪表盘 12 + 雷达 5 + 漏斗 4 + 和弦 4 + 旭日 7 + 树图 7 + 矩形树 7 + 关系图 13 + 桑基 7 + 主题河流 2 + 日历 9 + 矩阵 14 + 平行坐标 4 + 地图 25 + 地理 1 + 路径图 5 + 自定义系列 20 + 数据集 9 + 图形组件 5）。第 8.9 波去重全量 probe 281/296 `ok`。

---

## 四、site 文档站

### 目的

对外说明仓库起因与 rust-zrender / wasm-zrender / wasm-echarts 定位；顶栏进入两套产品文档与实例。由规划中的 `demo/` 迁来，现为 Vite 多页工程。

### 实现了哪些内容

```
site/
├── index.html                 # 首页：起因介绍 + 三层定位 + 已知限制（无产品卡片）
├── package.json               # vite、shiki；不声明 wasm npm 依赖
├── vite.config.js             # 多页 HTML 入口 + alias + wasm MIME + COOP/COEP（Worker 回图 SAB）
├── public/                    # 静态资源（字体等，按本地/部署准备）
├── src/
│   ├── shared/                # layout.css、site-header.js、docs-shell.js、画廊 UI、源码高亮
│   ├── zrender/fonts.js       # 可选：字体加载辅助
│   └── echarts/               # ensureDefaultFont、官网薄环境 official-env.js、official-gallery-meta.js
├── zrender/
│   ├── index.html             # 薄枢纽：文档 / 实例卡片
│   ├── docs/
│   │   ├── index.html         # 快速上手（文档默认页）
│   │   ├── fonts.html
│   │   ├── differences.html
│   │   ├── internals.html
│   │   └── api/               # facade / pkg 分页
│   └── examples/              # 每个示例 = html + 完整 js
│       ├── gallery.js
│       ├── shapes.html / shapes.js
│       ├── text.html / text.js
│       └── …
└── echarts/
    ├── index.html             # 薄枢纽：文档 / 实例卡片
    ├── docs/
    │   ├── index.html         # 快速上手（文档默认页）
    │   ├── fonts.html
    │   ├── runtime.html       # 主线程 vs Worker
    │   ├── differences.html
    │   ├── coverage.html      # 已实现 / 未实现
    │   ├── internals.html
    │   └── api/               # facade / pkg 分页
    └── examples/              # 每个示例 = html + 完整 js
        ├── gallery.js
        ├── official-line-catalog.js / official-bar-catalog.js / official-pie-catalog.js / official-scatter-catalog.js
        ├── line.html / line.js
        ├── line-simple.html / bar-simple.html / pie-simple.html  # 官网同步…
        └── …
```

- 全站顶栏（`src/shared/site-header.js`）：品牌 → `/`；**wasm-zrender** → `/zrender/docs/`；**wasm-echarts** → `/echarts/docs/`。进入产品后旁挂「文档 / 实例」
- 首页 = 起因介绍 + 三层定位 + 已知限制；**没有**产品卡片，不写 API 长文、不嵌 canvas
- zrender 文档五章 + 侧栏（`docs-shell.js`）：快速上手（默认）/ 字体引用 / API 参考（facade + pkg）/ 与官方差异 / 底层原理。正文不再是单页全集，也不复述首页长文
- echarts 文档七章 + 侧栏（`docs-shell.js` 按 `/echarts/docs` 切 `ECHARTS_DOC_NAV`）：快速上手（默认）/ 字体引用 / 运行模式 / API 参考（facade + pkg）/ 与官方差异 / 已实现 / 未实现 / 底层原理。公开正文不含 probe / 波次 / ok 计数
- 实例画廊：左侧菜单 + 源码（即该示例 `.js` 全文），右侧 iframe 预览。echarts 为二级菜单（类别 → 示例）；zrender 为扁平列表
- 每个示例 JS 自包含：导入、构图、绘制、交互都写在同一个文件里。官网同步页自己 `init`/`setOption`，不经 `runOfficialExample`

#### 实例清单

**wasm-zrender**

| 页 | 验证点 |
|----|--------|
| hello_world | Circle + Rect + RadialGradient |
| animation | `animate().when().start()` 写入最后一组 when（圆停在右侧） |
| bounding_box | `init(canvas)` + draggable Circle，实时 Group boundingRect |
| clip_path | `Circle.setClipPath(Heart)` |
| glitched_text | Text + `attr(position/shape/style)` |
| particles | `animate` / `during` / `done` 终态 |
| shapes | `Rect instanceof Path`、`Group({x:10}).x`、`Rect.r`、`Sector.r0`、渐变 / 虚线 / 基础 Path |
| text | `registerFont` + 中文 Text、对齐 |
| sector | 循环 Sector 饼扇 |
| hit | `findHover` |
| state | `setStateStyle` + `useState('emphasis')` |

**wasm-echarts**（画廊按类别二级菜单；每类可继续加示例）

| 类别 | 页 | 验证点 |
|------|----|--------|
| 折线图 | line | ChartView line |
| 折线图 | fonts | 多 `registerFont` + title / legend / 轴名称 `fontFamily` |
| 折线图 | 官网 40 条 | 同步 [echarts 折线示例](https://echarts.apache.org/examples/zh/index.html#chart-type-line)；缺能力则预览报错，不在示例里补实现 |
| 柱状图 | bar | ChartView bar |
| 柱状图 | 官网 46 条 | 同步 `#chart-type-bar`；`bar-large` 走 `useWorker`（主线程不再卡死；bar 仍无增量绘制，probe 仍可能 timeout） |
| 饼图 | pie | ChartView pie |
| 饼图 | 官网 19 条 | 同步 `#chart-type-pie`；catalog 一经生成即自动进画廊 |
| 散点图 | scatter | ChartView scatter |
| 交互合集 | interactive | `on('click')` + 内建 tooltip / hover / wheel zoom |
| 交互合集 | merge | 二次 `setOption` 深合并 + `getOption` |
| 交互合集 | bench | `benchmarkRender(30)` |

规划里的独立 `function-option.html` 未单列，函数 option 合在 interactive。

### 如何编译 / 启动

```bash
cd wasm-echarts-rs/site
npm install          # 或 pnpm install
npm run dev          # http://127.0.0.1:5173/
npm run build        # 输出 site/dist/
npm run preview      # 预览构建结果
```

仓库已跟踪两个 crate 的 `pkg/`。克隆后可直接 `npm run dev`。改 Rust 后须在本地重新 `wasm-pack build --target web` 并提交 `pkg/`，否则 facade 引用的仍是旧产物。

浏览器入口：

| 页面 | URL |
|------|-----|
| 首页 | http://127.0.0.1:5173/ |
| zrender 文档 | http://127.0.0.1:5173/zrender/docs/ |
| zrender 实例 | http://127.0.0.1:5173/zrender/examples/ |
| echarts 文档（快速上手） | http://127.0.0.1:5173/echarts/docs/ |
| echarts 实例 | http://127.0.0.1:5173/echarts/examples/ |

`scripts/serve-demo.sh` 仍 `wasm-pack` wasm-echarts 后用 `python -m http.server` 指到已不存在的 `demo/`，不要用它启动文档站。

### 站点自己的产物

`npm run build` → `site/dist/`（gitignore）。其中会打包对已提交 `pkg/*.js` 与 `.wasm` 的引用。dev server 已用插件把 WASM MIME 设为 `application/wasm`；GitHub Pages 对 `.wasm` 亦如此。

GitHub Pages 项目站路径是 `/wasm-echarts/`。`SITE_BASE` 非空时 Vite `base` 带上此外前缀；JS 用 `src/shared/site-base.js` 的 `withBase`。本地 `npm run dev` / `npm run build` 不设 `SITE_BASE`，行为与原来相同。

### GitHub Pages 部署

`pkg/` 在本地 `wasm-pack` 后提交。`site/dist/` 不提交。CI（`.github/workflows/deploy-pages.yml`）只用仓库里的 `pkg/` 跑 `npm run build`，再用 `actions/deploy-pages` 发布；不在 CI 里编 WASM。

只手动触发，不在 push 时跑：

1. 仓库 Settings → Pages → Source 选 **GitHub Actions**（不要再用 Deploy from a branch）。
2. 推到 `main` 后：GitHub → Actions → **Deploy pages** → Run workflow（或 `cd wasm-echarts-rs/site && npm run deploy:pages`）。
3. 站点：https://shenjunjian.github.io/wasm-echarts/

GitHub Pages 不能设 COOP/COEP。Worker 回图走 Transferable，不走 SharedArrayBuffer。

---

## 文档站如何通过「包」引用 WASM 产物

站点 **没有** 在 `package.json` 里写：

```json
"dependencies": { "wasm-zrender": "...", "wasm-echarts": "..." }
```

而是用 **Vite `resolve.alias` 把「包名」映射到 wasm-pack 输出目录**，效果等同于本地 file 依赖，但跳过 npm link。

`site/vite.config.js` 关键配置：

```javascript
const repoRoot = resolve(root, '..'); // wasm-echarts-rs/

const COOP_COEP_HEADERS = {
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp',
};

resolve: {
  alias: {
    '@wasm-zrender': resolve(repoRoot, 'crates/wasm-zrender/js'),
    '@wasm-echarts': resolve(repoRoot, 'crates/wasm-echarts/js'),
  },
},
server: {
  fs: { allow: [repoRoot] },  // 允许开发服务器读 site 目录以外的 js/ 与 pkg/
  headers: COOP_COEP_HEADERS, // Cross-Origin Isolation，Worker 回图才能用 SAB
},
preview: {
  headers: COOP_COEP_HEADERS,
},
assetsInclude: ['**/*.wasm'],
optimizeDeps: {
  exclude: [
    '@wasm-zrender',
    '@wasm-echarts',
  ],
},
```

因此：

```javascript
import initWasm, { init, Group, Rect } from '@wasm-zrender';
import initWasm, { init as initChart } from '@wasm-echarts';
```

会解析为：

```
crates/wasm-zrender/js/index.js  →  ../pkg/wasm_zrender.js
crates/wasm-echarts/js/index.js  →  ../pkg/wasm_echarts.js
```

胶水 JS 再相对加载同目录的 `wasm_zrender_bg.wasm` / `wasm_echarts_bg.wasm`。`initWasm()`（wasm-bindgen 的 default export）负责实例化 WASM。

wasm-pack 生成的 `pkg/package.json`：

```json
{
  "name": "wasm-zrender",
  "type": "module",
  "main": "wasm_zrender.js",
  "types": "wasm_zrender.d.ts",
  "files": ["wasm_zrender_bg.wasm", "wasm_zrender.js", "wasm_zrender.d.ts"]
}
```

若要在其它前端工程用 **真正的 npm 包依赖**，可以：

```json
{
  "dependencies": {
    "wasm-zrender": "file:../wasm-echarts-rs/crates/wasm-zrender/js",
    "wasm-echarts": "file:../wasm-echarts-rs/crates/wasm-echarts/js"
  }
}
```

然后：

```javascript
import initZrender, { init, Rect } from 'wasm-zrender';
import initEcharts, { init as initChart } from 'wasm-echarts';
```

（具体子路径以该包 `main` 为准。）本仓库的 site 用 alias：`@wasm-zrender` → `js/`，`@wasm-echarts` → `js/`。改完 Rust 只需重新 wasm-pack。只改 `js/` 不必重编 WASM。

实例 JS 与 HTML 放在同一目录（如 `zrender/examples/shapes.js`、`echarts/examples/line.js`），画廊用 `?raw` 读取这份脚本作为源码展示。echarts 示例公开写法是 `init` + `setOption`。

---

## 推荐工作流（从零到打开页面）

```bash
# 0. 工具链（只需一次）
rustup target add wasm32-unknown-unknown
# 安装 wasm-pack

# 1. 编译两个 WASM 包（每次改 Rust 都要重做）
cd wasm-echarts-rs/crates/wasm-zrender
wasm-pack build --target web --dev

cd ../wasm-echarts
wasm-pack build --target web --dev

# 2. 可选：纯 Rust 回归
cd ../..
cargo test -p rust-zrender
cargo test -p wasm-echarts
cargo test -p wasm-zrender

# 3. 启动文档站
cd site
npm install
npm run dev
```

改 `rust-zrender` 或任一 wasm crate 的 `.rs` 后：重新 wasm-pack 对应包（改底层则两个都要编），浏览器 Ctrl+Shift+R。

只改 `site/src` 的 JS/CSS/HTML：保存后 Vite HMR / 刷新即可，不必重编 WASM。

---

## 尚未完成（规划 backlog，避免重复开发已完成项）

### rust-zrender / 渲染

- conic gradient；CSS filter（文档化为不支持即可）
- 脏矩形 `useDirtyRect`、与多 zlevel 联调
- 完整 RichText 排版（TSpan 目前单 run）
- strokeText 封装、Text 与官方完全一致的 group 变换细节

### wasm-zrender

对照 [`.cursor/plans/zrender_api_规范对齐_be3227a1.plan.md`](../.cursor/plans/zrender_api_规范对齐_be3227a1.plan.md) 余下后置项，不混进「已对齐」：

- `skewX/Y` `anchorX/Y`、`textContent` 自动布局、RichText
- `morph` 形变、`IncrementalDisplayable` 增量语义、`Path.extend` 自定义 `buildPath` 走完整 PathProxy

### wasm-echarts

对照 [`.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md`](../.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md)。入口对齐（`init`/`setOption`/`on`）与第 0–8.9 波已落地。剩余缺口出现在上文「未实现」表：

- 第 1 波（已落地）：单 WASM、`getZr` 共享 Storage、`graphic`/`util`/`time` 等命名空间、挡脚本的实例 API
- 第 2 波（已落地）：dataset / transform / encode / stack / sampling；多 grid / 轴；time / log
- 第 3 波（已落地）：line/bar/pie/scatter canvas option 族（`smooth`/`areaStyle`/`step`/`connectNulls`/`endLabel`、bar 宽距圆角、`roseType`/`selectedMode`、scatter `large` 与其余 symbol）
- 第 4 波（已落地）：legend 点击筛选、title 布局、mark*、`option.graphic`、visualMap、dataZoom slider、axisPointer 十字 + `tooltip.trigger: 'axis'`、brush、timeline、toolbox canvas 按钮、thumbnail
- 第 5 波（已落地）：polar / radar / singleAxis / parallel / calendar / matrix / geo
- 第 6 波（已落地）：其余官方图表 + `custom` `renderItem`/`api`；feature 默认全开
- 第 7 波（已落地）：扩展注册真表、`labelLayout`、`axis.breaks`、`axis.jitter`、`option.media`；UniversalTransition 终态
- 第 8 波（8.0–8.9 均已落地）：官网画廊按类同步 + probe。**8.9 全量去重 probe 296 条中 281 `ok`**（3 timeout / 12 error 写入上文「未实现」）。缺官方实例方法同名导出 + `console.warn`，避免 `is not a function`。
- 视觉回归（echarts `test/*.html` → golden PNG）；JS vs WASM 基准报告

### 明确不做

- SVG painter：`renderToSVGString` / `getSvgDataURL` / `zr.painter.getSvgDom`；hover layer / dirty rect
- 动画中间帧（终态语义要做；`morph` 形变仍按 zrender 计划）
- toolbox DataView DOM 浮层、SaveAsImage 的 DOM 下载条
- Loading 旋转动画（API 仍导出，静态遮罩或空操作）
- aria 写 `zr.dom` 无障碍属性（非绘制，可后置）

---

## 源码对照（只读）

实现时按行为参考 TypeScript，Rust 侧按习惯重写，不做 1:1 文件映射。常用对照：

- zrender：`Storage.ts`、`canvas/Painter.ts`、`Handler.ts`、`core/PathProxy.ts`、`graphic/shape/*.ts`、`src/export.ts`
- echarts：`core/echarts.ts`、`model/Global.ts`、`data/helper/sourceHelper.ts`、`component/transform/`、`processor/dataStack.ts`、`chart/line/LineView.ts`、`chart/bar/BarView.ts`、`util/types.ts`（CallbackDataParams）
