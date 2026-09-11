# wasm-echarts

echarts 的 wasm 移植版本。目的是快速将 echarts 的 option 选项绘制出来，不注重动画。

## 灵感

1. wasm 本质上是一个后台进程，如果将绘制过程使用 rust 语言在 wasm 离屏渲染，必然可以节省浏览器的主进程的时间
2. 先移植 zrender，后移植 echarts  
   Storage（场景图）→ Painter（遍历 displayList）→ ctx.fillRect / arc / fillText / drawImage ... 要移植到 Rust/Wasm，Painter 层需要这些能力：  
   使用 vl-convert-canvas2d 作为 Painter 后端。它不支持的能力，需要补齐，  
   比如无 shadow、CSS filter、isPointInPath（命中检测需自实现）、径向渐变内圆 r0 支持不完整、conic gradient
3. echarts 做为前端库，有一些参数是函数，需要设计如何传递。
4. 前端网页上的 canvas 的鼠标事件、滚轮缩放范围、resize 事件等是需要及时传递给 rust 中，让它能实时响应这些行为，比如高亮线段、显示 tooltip 等等。这些需要根据 echarts 的实现来设计方案。
5. api 对齐：
   - 只支持 canvas 模式即可，不需要支持 svg 模式。
   - 动画 API 直接执行到终态，不播中间帧。
   - 靠 DOM 实现的组件（toolbox DataView、SaveAsImage 下载条、Loading 旋转动画）可以忽略；其余官方 canvas 绘制与纯计算 API 要对齐。
   - 未完成项按 [echarts canvas 全量对齐](.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md) 补齐，不是永久砍掉。

## 结构

```
wasm-echarts/
├── wasm-echarts-rs/              # Rust workspace 根目录
│   ├── Cargo.toml
│   ├── site/                     # 文档站：首页介绍仓库定位；顶栏进 wasm-zrender / wasm-echarts
│   │   ├── index.html            # 起因 + 三层定位（无产品卡片）
│   │   ├── zrender/docs/         # 五章文档，默认快速上手
│   │   └── echarts/examples/     # 每个示例直接 import @wasm-echarts
│   └── crates/
│       ├── rust-zrender/         # 纯 Rust lib：zrender 渲染核心（底层依赖）
│       ├── wasm-zrender/         # wasm-pack：对齐 zrender export.ts 的 init/Group/Rect API
│       ├── wasm-echarts/         # wasm-pack + js/ facade：对齐官方 init/setOption
│       │   ├── src/
│       │   ├── js/               # 公开 API
│       │   └── pkg/              # wasm-bindgen 内部 handle
│       └── …
├── echarts-master/               # echarts@6.1 源码，只读参考
└── zrender-master/               # zrender@6.1 源码，只读参考
```

## 环境要求

| 工具 | 说明 |
|------|------|
| [Rust](https://rustup.rs/) | 1.70+，需安装 `wasm32-unknown-unknown` target |
| [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) | 将 Rust 编译为浏览器可用的 wasm 包 |
| 静态 HTTP 服务 | 本地跑 Demo，例如 `npx http-server` |

首次安装 wasm target：

```bash
rustup target add wasm32-unknown-unknown
```

## 编译

Workspace 含三个 Rust crate：**`rust-zrender`**（纯 lib）、**`wasm-zrender`** / **`wasm-echarts`**（wasm-bindgen 产物）。  
`wasm-echarts` path 依赖 `wasm-zrender`（rlib），echarts 运行时只加载一份 WASM，`getZr()` 与 ChartView 共用 Storage。zrender 文档站可继续单独用 `wasm-zrender/pkg`。详见 [AGENT.md](AGENT.md) 与 [canvas 全量对齐计划](.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md)。

### wasm-echarts（ECharts Demo）

```bash
cd wasm-echarts-rs/crates/wasm-echarts
wasm-pack build --target web --dev    # 开发
wasm-pack build --target web --release  # 发布
```

### wasm-zrender（zrender 底层 Demo，可选）

```bash
cd wasm-echarts-rs/crates/wasm-zrender
wasm-pack build --target web --dev
```

### 开发构建（推荐调试）

体积较大、带 debug 符号，编译快：

```bash
cd wasm-echarts-rs/crates/wasm-echarts
wasm-pack build --target web --dev
```

### 发布构建

开启 `opt-level = "s"`，体积更小，适合部署：

```bash
cd wasm-echarts-rs/crates/wasm-echarts
wasm-pack build --target web --release
```

### 运行单元测试

在 workspace 根目录执行（不产出 wasm，只测 Rust 逻辑）：

```bash
cd wasm-echarts-rs
cargo test -p rust-zrender
cargo test -p wasm-echarts
cargo test -p wasm-zrender
```

## 编译产物

`wasm-pack build` 成功后，各 crate 产物位于对应 `pkg/`：

```
wasm-echarts-rs/crates/wasm-echarts/pkg/   # wasm-bindgen 内部 handle
wasm-echarts-rs/crates/wasm-echarts/js/    # 公开 API（@wasm-echarts）
wasm-echarts-rs/crates/wasm-zrender/js/    # 公开 API（@wasm-zrender）
wasm-echarts-rs/crates/wasm-zrender/pkg/   # wasm-bindgen 内部 handle，勿直接 import
```

以 wasm-echarts 为例：

```
wasm-echarts-rs/crates/wasm-echarts/pkg/
├── wasm_echarts_bg.wasm    # WebAssembly 二进制
├── wasm_echarts.js         # wasm-bindgen 生成的 JS 胶水层（ES Module）
├── wasm_echarts.d.ts       # TypeScript 类型声明
├── wasm_echarts_bg.wasm.d.ts
└── package.json            # 可作为 npm 包引用
```

中间编译缓存位于 `wasm-echarts-rs/target/`（已在 `.gitignore` 中忽略，无需提交）。

### wasm-echarts 公开 API（对齐官方 core.ts）

公开 JS 表面与官方 `echarts.init` / `chart.setOption` 同名同签名。入口清单见 [`.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md`](.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md)；canvas 能力补齐见 [`.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md`](.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md)。site 从 `@wasm-echarts`（`js/index.js`）导入；`pkg/` 只作内部 handle。

#### 1. 与官方一致的公开 API

`init` / `dispose` / `getInstanceByDom` / `getInstanceById` / `version`（`'6.1.0'`）/ `use`（只提示）；`connect` / `disconnect` / `registerTheme` / `registerMap` / `getMap` / `parseGeoJSON`；`registerPreprocessor` / `registerProcessor` / `registerLayout` / `registerVisual` / `registerAction` / `registerCoordinateSystem` / `registerCustomSeries` / `PRIORITY`；命名空间 `graphic` / `util` / `number` / `time` / `format` / `helper` / `matrix` / `vector` / `color` / `env`；`throttle`。实例 `setOption` / `getOption` / `resize` / `clear` / `dispatchAction` / `on` / `off` / `getWidth` / `getHeight` / `getDevicePixelRatio` / `isDisposed` / `dispose` / `getDom` / `getId` / `convertToPixel` / `convertFromPixel` / `containPixel` / `getZr` / `showLoading` / `hideLoading` / `getDataURL` / `renderToCanvas` / `getRenderedCanvas` / `appendData` / `setTheme`。`init(canvas)` 后 `setOption` 自动上屏，并绑定指针（`click` / `mouseover` / `mouseout` / `globalout`）。`dispatchAction` 已接线：`highlight` / `downplay` / `select` / `unselect` / `toggleSelect` / `dataZoom` / `showTip` / `hideTip` / `legendToggleSelect` / `legendSelect` / `legendUnSelect` / `restore` / `timelineChange` / `timelinePlayChange` / `takeGlobalCursor` / `brush` / `brushEnd` / `expandAxisBreak` / `collapseAxisBreak` / `toggleAxisBreak`。

#### 2. 与官方不一致 / 例外

- `use(...)` 导出但不按需加载，只 `console.info` 提示已编进 WASM。
- `init(null)` 允许离屏；仅 canvas，无 SVG。
- 动画终态；`lazyUpdate` 同步。`universalTransition` 只跳终态，不插值。
- 扩展 StageHandler 拿到简化 `ecModel`（`getOption` / `eachSeries`），不是官方 GlobalModel。`labelLayout` / `axis.breaks` / `axis.jitter` 是 canvas 终态实现。
- 字体：WASM 不读系统字体。须从 `@wasm-echarts` 调 `registerFont`。echarts 页与 `getZr` 共用同一份 fontdb；zrender 文档站的 pkg 仍是另一份内存。
- `getZr()` 返回与 ChartView 共用 Storage 的 wasm-zrender 实例（同一份 WASM，不是第二份 `wasm-zrender/pkg`）。无 SVG painter（`zr.painter.getSvgDom` / `renderToSVGString` / `getSvgDataURL` 只 warn）；无 hover layer / dirty rect；动画终态。
- `showLoading` / `hideLoading` 导出，静态半透明遮罩 + 文案，不播旋转动画。
- toolbox DataView 浮层与 SaveAsImage 的 DOM 下载条明确不做。
- `notMerge` 只作 `setOption` 第二参数，写在 option 根上不会当合并开关。
- `replaceMerge` 只按顶层 key 整段替换，不按 component `id`。
- `lazyUpdate` / `silent` 同步 flush。
- default export 是 wasm-bindgen `initWasm`，不是 echarts 命名空间对象。
- tooltip 为 facade 内建简单 string HTML，不是官方 TooltipView。
- `opts.useWorker` 为非官方开关（默认 `false`）。`true` 时 Worker 持实例，主线程只 blit / 事件 / tooltip；无 Worker 时 warn 并回退主线程。不要自己 `new Worker`。
- Worker 仅支持可结构化克隆的 option：`formatter` / `renderItem` 等函数必须走默认主线程。`useWorker` 时 `setOption` 返回 Promise；`getZr()` 不可用。
- **option 方向一律不用 SharedArrayBuffer**（含 TypedArray 数据）。SAB 只用于 Worker → 主线程回传 RGBA，站点需 `COOP: same-origin` + `COEP: require-corp`；不可用时 fallback Transferable。同线程路径不强制 SAB。

#### 3. 多出来的非官方 API

因 WASM 离屏需要，**不要当官方 API 用**。`init(canvas)` 后普通用户不必再调 `refresh` / 手动 `putImageData`，也不必再绑指针或自绘 tooltip。

| 方法 | 用途 |
|------|------|
| `refresh` | 返回 RGBA；离屏无 canvas 时用 |
| `findHover` / `handlePointerMove` / `handlePointerLeave` | 非官方 hatch；`init(canvas)` 时一般不需要 |
| `applyDataZoomWheel` / `getTooltipContent` / `benchmarkRender` | 非官方；滚轮已绑定、tooltip 已内建 DOM、bench |
| `registerFont` / `clearFonts` | WASM 字体例外；轴标签 / series label 渲染前必调 |
| `hasOption` / `optionHasFunctions` | 状态查询（非官方） |
| `opts.useWorker` | 把实例放到 Worker；开发者仍写 `init` / `setOption` / `on` |

#### 4. 已实现 / 未实现

- **已实现（部分生效）**：官方 `export/charts.ts` 23 种图（含 chord）canvas 终态；`custom` `renderItem` + `api.coord`/`size`/`style`；line / bar / pie / scatter 的 canvas option 族（含 polar 与 y 类目横画）；多 cartesian（grid / 轴 / time / log / `breaks` / `jitter`）；polar / radar / singleAxis / parallel / calendar / matrix / geo 坐标系；dataset / transform / encode / stack / sampling；`option.media` 按宽高切 option；`labelLayout` 终态避让；`getZr` 共用 Storage；`graphic`/`util`/`time` 等命名空间与 `throttle`；legend 点击筛选、title 布局、mark*、`option.graphic`、visualMap、dataZoom slider + inside、十字 axisPointer、`tooltip.trigger: 'axis'`、brush、timeline、toolbox canvas 按钮、thumbnail；`registerTransform` 与扩展注册真表（`registerPreprocessor` / `Processor` / `Layout` / `Visual` / `Action` / `CoordinateSystem` / `CustomSeries`）；`showLoading` 静态遮罩；`getDataURL`/`renderToCanvas`；`connect`/`registerTheme`/`registerMap`/`parseGeoJSON`；`appendData`/`containPixel`；`axisLabel.formatter`；series label；CallbackDataParams；`convertToPixel` 含非 cartesian finder；`on`/`off` 指针事件；内建 string tooltip；`showTip`/`hideTip`；hover / toggleSelect；可选 `useWorker`（大数据示例）。
- **未实现（不是永久例外）**：已导出只 `console.warn` 的有 `registerLocale` / `convertToLayout` / `getVisual` / `renderToSVGString` / `getSvgDataURL` / `getModel`。`smoothMonotone`、完整 SeriesData、pie 完整 `avoidLabelOverlap` / `padAngle`、bar `large`（无增量绘制；画廊 `bar-large` 走 `useWorker` 避免主线程卡死）、scatter 统计插件 `ecStat`、`candlestick-large` / `parallel-nutrients`（同样走 `useWorker`，Worker 内仍是长任务）、`heatmap-bmap` 无百度地图扩展、treemap 大树 / `levels` 色映射 WASM panic、`lines-ny` 分片 bin 未同步，见 [AGENT.md](AGENT.md) 与 [echarts 文档](wasm-echarts-rs/site/echarts/docs/index.html)。未识别的 `series.type` 会 warn，不再静默忽略。官网 27 类 catalog 去重 296 条已进画廊；第 8.9 波全量 probe **281/296 ok**。每个 `examples/*.js` 自己 `init`/`setOption`，不经 `runOfficialExample`。

native `EChartsInstance`（`wasm_echarts.d.ts`）是内部 handle，不要从 site 直接 `new`。

### wasm-zrender 对外 API（公开入口是 `js/`，不是 `pkg/`）

site 与文档站用 Vite alias `@wasm-zrender` → `crates/wasm-zrender/js`。`js/index.js` 再加载内部 `pkg/wasm_zrender.js`。请勿把 `pkg/` 当作公开 import。分章文档从 [`/zrender/docs/`](wasm-echarts-rs/site/zrender/docs/index.html) 进入（默认快速上手；侧栏进字体 / API / 差异 / 原理）。

```javascript
import initWasm, { init, registerFont, Group, Rect, Text } from '@wasm-zrender';
```

对齐官方 zrender `export.ts` 命名空间导出：

| 导出 | 说明 |
|------|------|
| `init(dom?, opts?)` | 创建 ZRender 实例。见下文两种用法 |
| `registerFont(data, opts?)` | 注册字体 bytes（**Text 渲染前必调**，见下文） |
| `clearFonts()` | 清空已注册字体（测试用） |
| `dispose(zr)` / `disposeAll()` / `getInstance(id)` | 实例生命周期 |
| `version` / `registerPainter` | `version = '6.1.0'`；非 canvas painter 忽略 |
| `Group` / `Rect` / `Circle` / `Line` / `Polygon` / `Polyline` / `Sector` / `Text` / … | 已实现图元（`Rect instanceof Path instanceof Displayable instanceof Element`） |
| `ZRender.add` / `remove` / `clear` / `dispose` | 根节点与实例释放 |
| `ZRender.refresh()` / `flush()` | 同步离屏绘制，返回 RGBA；**不会** `putImageData` |
| `ZRender.findHover(x, y)` | 返回 `{ target, topTarget }` |
| `ZRender.setBackgroundColor` / `trigger` / `setCursorStyle` | 背景、事件、光标 |
| `el.hide` / `show` / `on` / `off` / `trigger` / `clipPath` | 元素可见性、事件、裁剪 |
| `animate` / `animateTo` / `when().start()` | 立刻写入最后一组目标属性（不播中间帧） |
| `matrix` / `vector` / `color` / `path` / `util` | 官方签名工具模块（`js/tool/`） |
| `morph` / `parseSVG` | 最小实现：morph 返回终点 path；parseSVG 解析基本图形 |
| `IncrementalDisplayable` | 按普通 Group 语义可构造（非增量图层） |

**允许例外（仅此四条）**：字体必须 `registerFont`；动画只写终点；仅 canvas 离屏（无 SVG / hover layer / dirty rect）；`init(canvas)` 可自动 `putImageData`，`init(null)` 仍用 opts 宽高。详见 [zrender 文档](wasm-echarts-rs/site/zrender/docs/index.html)（`/zrender/docs/` 现为快速上手，侧栏进字体 / API / 差异 / 原理）与 [AGENT.md](AGENT.md)。仓库起因与 rust-zrender / wasm-zrender / wasm-echarts 定位在站点[首页](wasm-echarts-rs/site/index.html)。

### `init` 的两种用法

`init(dom?, opts?)` 的第一个参数决定是否绑定页面上的 `<canvas>`。绘制始终在 WASM 内存里完成，**宽高只读 `opts`**（缺省 `300×150`），不会从 DOM 或 canvas 属性测量。`devicePixelRatio` / `dpr` 可选，默认 `1`。

#### 1. `init(null, opts)` — 离屏，自行上屏

不绑 canvas、不绑指针事件。`add` 只改内存中的图元树。调用 `zr.refresh()` 或 `zr.flush()`（二者等价）拿到 RGBA，再由 JS `putImageData`。适合批量加完图元后画一帧、SSR、自定义渲染管线。

```javascript
const zr = init(null, { width: 480, height: 360 });
zr.add(new Rect({ shape: { x: 10, y: 10, width: 80, height: 40 } }));
const rgba = zr.refresh(); // Uint8Array，不会写到任何 canvas
const ctx = canvas.getContext('2d');
ctx.putImageData(
  new ImageData(new Uint8ClampedArray(rgba), zr.getWidth(), zr.getHeight()),
  0,
  0,
);
```

#### 2. `init(canvas, opts)` — 绑定 canvas，自动上屏

把传入的 `HTMLCanvasElement` 交给 Handler：可接收 pointer 事件（点选 / 拖拽）；`add` / `remove` / `setStyle` / 改 `position` 等会立刻全量 `refresh` 并 `putImageData`。观感接近官方 `zrender.init(container)`。适合少量图元和交互 Demo。

```javascript
const zr = init(canvas, { width: canvas.width, height: canvas.height });
zr.add(new Circle({
  shape: { cx: 80, cy: 80, r: 30 },
  style: { fill: '#5470c6' },
}));
// 已上屏，不必再调用 refresh / flush
```

不要在循环里连续 `add` 几百个元素：每一次都会重绘整幅画布，主线程会卡住，浏览器也要等循环结束才能把 canvas 合成到屏幕。批量场景请用 `init(null)`，加完后再 `refresh` 一次。

#### 区别对照

| | `init(null, opts)` | `init(canvas, opts)` |
|---|---|---|
| 尺寸来源 | `opts.width` / `opts.height` | 同左，不读 canvas 的 `width`/`height` 属性 |
| 指针事件 | 无 | 绑到该 canvas |
| `add` / 改属性后 | 只更新场景图 | 立刻全量绘制并 `putImageData` |
| `refresh()` / `flush()` | 返回 RGBA，须自行上屏 | 同样只返回像素，**不会再贴一次**；上屏靠改图元时的自动 blit |
| 批量添加 | 循环后再 `refresh` 一次即可 | 每次 `add` 都全量重绘，大量图元会明显卡顿 |

`refresh()` / `flush()` 与官方同名，语义不同：官方是预约或立即画到 DOM canvas（返回 `void`）；wasm-zrender 是同步离屏绘制并返回 `Uint8Array`。动画同样只写终点，`animate().when().start()` 不会播中间帧。

离屏示例：[text.html](wasm-echarts-rs/site/zrender/examples/text.html)；绑定 canvas 示例：[bounding_box.html](wasm-echarts-rs/site/zrender/examples/bounding_box.html)。

### 字体加载（Text 必看）

wasm-zrender 在 Rust 离屏 Canvas 中绘制文字，**WASM 环境无法读取系统字体**。使用 `Text` 图元前，须由宿主将字体文件 bytes 注册到 fontdb：

```javascript
import initWasm, { init, registerFont, Text } from '@wasm-zrender';

await initWasm();

// 1. 加载字体（TTF / OTF / WOFF）
const bytes = new Uint8Array(
  await (await fetch('/fonts/NotoSansSC-Regular.ttf')).arrayBuffer()
);

// 2. 注册到 WASM fontdb（须在含 Text 的 refresh 之前）
registerFont(bytes, {
  familyName: 'Noto Sans SC',       // 可选，覆盖字体族名
  sansSerif: ['Noto Sans SC'],      // 可选，映射 CSS sans-serif
});

// 3. 正常使用
const zr = init(null, { width: 480, height: 360 });
zr.add(new Text({ style: { text: '中文', x: 24, y: 48, fontSize: 18, fill: '#333' } }));
const rgba = zr.refresh();
```

**Wasmer / 原生 Rust** 宿主可直接调用 `rust_zrender::register_font(bytes, RegisterFontOptions { ... })`，无需 JS。

site 文档站提供辅助模块 `site/src/zrender/fonts.js`（`loadFontFromUrl` / `ensureDefaultFont`），默认字体位于 `site/public/fonts/NotoSansSC-Regular.ttf`。字体专章见 [fonts.html](wasm-echarts-rs/site/zrender/docs/fonts.html)；text 示例见 [zrender/examples/text.html](wasm-echarts-rs/site/zrender/examples/text.html)。

未注册字体时渲染 `Text` 会报错 `no default font found`。

## 使用方法

### 1. 跑 Demo（最快验证）

文档站通过 Vite alias 引用 wasm-zrender 与 wasm-echarts 的 `js/` facade。**必须先完成 wasm-pack 编译**。

```bash
# 1. 编译 wasm（若尚未编译）
cd wasm-echarts-rs/crates/wasm-echarts
wasm-pack build --target web --dev

# 2. 启动文档站（Vite）
cd site
npm install
npm run dev
```

浏览器打开：

- 首页（起因 + 三层定位）：http://127.0.0.1:5173/
- wasm-zrender 文档（快速上手）：http://127.0.0.1:5173/zrender/docs/
- wasm-zrender 实例：http://127.0.0.1:5173/zrender/examples/
- wasm-echarts 文档：http://127.0.0.1:5173/echarts/docs/
- wasm-echarts 实例：http://127.0.0.1:5173/echarts/examples/

### 2. 在页面中接入（`@wasm-echarts`）

公开 API 是官方写法 `init(canvas)` + `setOption`。site 指向 `crates/wasm-echarts/js`。`refresh` / `handlePointerMove` 等是非官方 hatch，离屏或自定义交互才需要。

```html
<canvas id="canvas"></canvas>
<script type="module">
  import initWasm, { init } from '@wasm-echarts';

  await initWasm();

  const canvas = document.getElementById('canvas');
  const width = 480;
  const height = 360;
  canvas.width = width;
  canvas.height = height;

  const chart = init(canvas);
  chart.setOption({
    xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed'] },
    yAxis: { type: 'value' },
    series: [{ type: 'line', name: '销量', data: [120, 200, 150] }],
  });
</script>
```

`init(canvas)` 后 facade 绑定指针：hover 高亮、string tooltip、legend/toolbox 点击、slider/brush 拖拽、wheel inside dataZoom。用户写 `chart.on('click', handler)` 即可，不必再调 `handlePointerMove`。

| 方法 | 说明 |
|------|------|
| `init(canvas, theme?, opts?)` | 创建实例；有 canvas 时 `setOption` 自动上屏并绑指针 |
| `setOption(option, notMerge? \| opts?)` | 设置 option 并重算图元；`notMerge` 不是 option 字段 |
| `getOption()` | 已合并 option（函数字段保留） |
| `resize(opts?)` | 无参读 canvas；或 `{ width, height, devicePixelRatio }` |
| `clear()` | `setOption({ series: [] }, true)` |
| `on` / `off` | `click` / `mouseover` / `mouseout` / `globalout` |
| `dispatchAction(action)` | `toggleSelect` / `highlight` / `showTip` / `hideTip` 等 |
| `refresh()` | 离屏绘制，返回 RGBA（非官方） |
| `handlePointerMove(x, y)` | 非官方；`init(canvas)` 时一般不需要 |
| `findHover(x, y)` | 命中检测（非官方） |
| `applyDataZoomWheel(x, deltaY)` | 非官方；`init(canvas)` 时滚轮已绑定 |
| `benchmarkRender(n)` | 渲染均值耗时（毫秒，非官方） |
| `dispose()` | 释放实例 |

修改 Rust 源码后需重新执行 `wasm-pack build`，浏览器侧硬刷新（Ctrl+Shift+R）即可加载新 wasm。

## 当前能力（阶段 5 MVP）

- 图表类型：line / bar / pie / scatter
- option 中的 JS 函数：`tooltip.formatter`、`label.formatter`、`itemStyle.color` 等
- 交互：`init(canvas)` 绑指针、hover、string tooltip、legend 筛选、slider / inside dataZoom、十字 axisPointer、`showTip`/`hideTip`
- 仅 canvas 渲染，无动画中间帧
- 公开 JS API：`init` / `setOption` / `getOption` / `resize` / `clear` / `on` / `off` / `dispatchAction` / `dispose` / `use`（只提示）

canvas 能力按 [全量对齐计划](.cursor/plans/echarts_canvas_全量对齐_3ceaa41e.plan.md) 补齐。已实现 / 未实现与例外见上文四块。
