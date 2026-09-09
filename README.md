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
   - 这个库重点是实现 canvas 绘制，如果遇到动画 api，直接让它执行完毕，不需要中间过程。如果遇到处理 dom 元素相关的，也可以忽略它。

## 结构

```
wasm-echarts/
├── wasm-echarts-rs/              # Rust workspace 根目录
│   ├── Cargo.toml
│   ├── site/                     # 文档站（Vite 多页 + 实例）
│   │   ├── index.html
│   │   ├── main.js
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
`wasm-echarts` 与 `wasm-zrender` 均只依赖 `rust-zrender`，彼此不依赖。

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

公开 JS 表面与官方 `echarts.init` / `chart.setOption` 同名同签名。**逐项清单以** [`.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md`](.cursor/plans/wasm-echarts_api_对齐_1d164d7d.plan.md) **为权威**。site 从 `@wasm-echarts`（`js/index.js`）导入；`pkg/` 只作内部 handle。

#### 1. 与官方一致的公开 API

`init` / `dispose` / `getInstanceByDom` / `getInstanceById` / `version`（`'6.1.0'`）/ `use`（只提示）；实例 `setOption(option)` / `resize` / `dispatchAction` / `getWidth` / `getHeight` / `getDevicePixelRatio` / `isDisposed` / `dispose` / `getDom` / `getId`。`init(canvas)` 后 `setOption` 自动上屏。

#### 2. 与官方不一致 / 例外

- `use(...)` 导出但不按需加载，只 `console.info` 提示已编进 WASM。
- `init(null)` 允许离屏；仅 canvas，无 SVG。
- 动画终态；`lazyUpdate` 同步；字体须 `registerFont`（尚未接到 wasm-echarts 模块）。
- `getZr()` 本波不导出。`setOption` 第二参数 `notMerge` 波次 2；当前仍可能被当 option 根字段剥掉。
- default export 是 wasm-bindgen `initWasm`，不是 echarts 命名空间对象。

#### 3. 多出来的非官方 API

因 WASM 离屏需要，**不要当官方 API 用**。`init(canvas)` 后普通用户不必再调 `refresh` / 手动 `putImageData`。

| 方法 | 用途 |
|------|------|
| `refresh` | 返回 RGBA；离屏无 canvas 时用 |
| `findHover` / `handlePointerMove` / `handlePointerLeave` | 指针 hatch（波次 3 由 `on`/`off` 消化） |
| `applyDataZoomWheel` / `getTooltipContent` / `benchmarkRender` | dataZoom / 自绘 tooltip / bench |

#### 4. 已实现 / 未实现

- **已实现（部分生效）**：line / bar / pie / scatter；单 cartesian；inside dataZoom 滚轮；hover / toggleSelect；string tooltip；竖线 axisPointer。
- **未实现**：legend / title / polar / gauge / 其余 chart、`connect`、主题、`getZr`、`getDataURL`、Loading、`showTip`/`hideTip`、完整 `convertToPixel`。详见 [AGENT.md](AGENT.md) 与 [echarts 文档](wasm-echarts-rs/site/echarts/docs/index.html)。

native `EChartsInstance`（`wasm_echarts.d.ts`）是内部 handle，不要从 site 直接 `new`。

### wasm-zrender 对外 API（公开入口是 `js/`，不是 `pkg/`）

site 与文档站用 Vite alias `@wasm-zrender` → `crates/wasm-zrender/js`。`js/index.js` 再加载内部 `pkg/wasm_zrender.js`。请勿把 `pkg/` 当作公开 import。

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

**允许例外（仅此四条）**：字体必须 `registerFont`；动画只写终点；仅 canvas 离屏（无 SVG / hover layer / dirty rect）；`init(canvas)` 可自动 `putImageData`，`init(null)` 仍用 opts 宽高。详见 [zrender 文档](wasm-echarts-rs/site/zrender/docs/index.html) 与 [AGENT.md](AGENT.md)。

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

site 文档站提供辅助模块 `site/src/zrender/fonts.js`（`loadFontFromUrl` / `ensureDefaultFont`），默认字体位于 `site/public/fonts/NotoSansSC-Regular.ttf`。text 示例见 [zrender/examples/text.html](wasm-echarts-rs/site/zrender/examples/text.html)。

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

- 首页：http://127.0.0.1:5173/
- echarts 实例：http://127.0.0.1:5173/echarts/examples/
- zrender 实例：http://127.0.0.1:5173/zrender/examples/

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

指针交互（hover / 点击 / 滚轮）目前由示例用 hatch 绑定：`handlePointerMove`、`findHover`、`dispatchAction`、`applyDataZoomWheel`。完整写法见 `site/echarts/examples/interactive.js`。波次 3 由 facade `on`/`off` 消化。

| 方法 | 说明 |
|------|------|
| `init(canvas, theme?, opts?)` | 创建实例；有 canvas 时 `setOption` 自动上屏 |
| `setOption(option)` | 设置 option 并重算图元 |
| `refresh()` | 离屏绘制，返回 RGBA（非官方） |
| `handlePointerMove(x, y)` | hover 高亮、axisPointer、tooltip 文本（非官方） |
| `findHover(x, y)` | 命中检测（非官方） |
| `dispatchAction(action)` | `toggleSelect` / `highlight` 等 |
| `applyDataZoomWheel(x, deltaY)` | inside dataZoom（非官方） |
| `benchmarkRender(n)` | 渲染均值耗时（毫秒，非官方） |
| `dispose()` | 释放实例 |

修改 Rust 源码后需重新执行 `wasm-pack build`，浏览器侧硬刷新（Ctrl+Shift+R）即可加载新 wasm。

## 当前能力（阶段 5 MVP）

- 图表类型：line / bar / pie / scatter
- option 中的 JS 函数：`tooltip.formatter`、`label.formatter`、`itemStyle.color` 等
- 交互：hover 命中检测、string tooltip、click `toggleSelect`、`highlight` / `downplay` action、inside dataZoom 滚轮
- 仅 canvas 渲染，无动画中间帧
- 公开 JS API：`init` / `setOption` / `dispose` / `use`（只提示）；事件与 `setOption` 第二参数仍在对齐中

尚未完整实现 echarts 全量 API。已实现 / 未实现与例外见上文四块。
