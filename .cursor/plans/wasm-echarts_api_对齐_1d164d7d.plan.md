---
name: wasm-echarts API 对齐
overview: 对照官方 echarts-master/src/core/echarts.ts 与 export/core.ts，用 JS facade 把公开入口/实例方法对齐到官方命名与签名。WASM 不分包按需加载，echarts.use 只打印提示。与官方不一致处、多出来的非官方 API、已实现/未实现范围都写进文档。
todos:
  - id: spec-agent
    content: 把例外/必须一致/后置项写入 AGENT.md；明确 use 只提示、公开 API 对齐、差异与非官方 API 必须文档化
    status: completed
  - id: facade-skeleton
    content: 新建 js/ facade：init/dispose/use（console 提示）/实例表/version；Vite alias 改指 js/；示例改为官方 init + setOption
    status: completed
  - id: setoption-resize
    content: setOption 第二参数 notMerge/opts；getOption；resize(opts)；clear/isDisposed；修掉 option 根上剔 notMerge
    status: pending
  - id: events-pointer
    content: on/off + init(canvas) 绑定指针与自动上屏；showTip/hideTip；非官方 hatch 仍导出并文档化
    status: pending
  - id: option-semantics
    content: 已有 4 类图：axisLabel.formatter、pie center/radius、symbol/symbolSize、label 图元、CallbackDataParams 补字段、convertToPixel 最小集
    status: pending
  - id: docs-verify
    content: 文档写清：与官方一致/不一致、多出来的非官方 API、已实现与未实现；示例改走 init；手工验收
    status: pending
isProject: false
---

# wasm-echarts API 规范与补齐

对照源（只读，禁止改）：[`echarts-master/src/core/echarts.ts`](echarts-master/src/core/echarts.ts)、[`echarts-master/src/export/core.ts`](echarts-master/src/export/core.ts)、[`echarts-master/src/export/api.ts`](echarts-master/src/export/api.ts)。禁止整文件复制官方实现。

当前公开入口是 wasm-bindgen 直出 [`EChartsInstance`](wasm-echarts-rs/crates/wasm-echarts/src/instance.rs)，site 这样用：

```javascript
import initWasm, { EChartsInstance } from '@wasm-echarts';
await initWasm();
const chart = new EChartsInstance(480, 360, 1);
chart.set_option({ ... });
const rgba = chart.refresh();
```

官方是：

```javascript
import * as echarts from 'echarts';
const chart = echarts.init(dom, theme?, opts?);
chart.setOption(option, notMerge?, lazyUpdate?);
chart.on('click', handler);
echarts.dispose(chart);
```

差距主要在 **公开 JS 表面**，不是「四个图表完全不能画」。line/bar/pie/scatter 的 option **形态**已经接近官方，但调用方式、方法名、第二参数、事件总线都未对齐。

公开 JS API **尽量与官方同名同签名**。做不到或语义不同的，不改名糊弄，而是在文档写明。多出来的非官方方法也要文档列出，避免用户当官方 API 用。已实现 / 未实现（图表、组件、action、option 字段）单独成表，随实现更新。

WASM 是整包模块，**不为减小体积做 `echarts.use` 动态加载**。`use` 仍导出（签名对齐），实现只 `console.info` 提示：已开发的图表/组件都编进 WASM 了，不必 `use`。

```mermaid
flowchart TB
  site["site examples"] --> facade["js/index.js 公开 API"]
  facade --> core["init dispose use setOption on dispatchAction"]
  core --> native["pkg/wasm_echarts.js 内部 handle"]
  native --> rust["wasm-echarts rust: OptionModel + ChartView"]
  rust --> zr["rust-zrender ZRenderer"]
```

---

## 规范（先写入 AGENT.md）

对齐 wasm-zrender 的写法：例外写死、必须一致可核对、后置项不混进「已对齐」。

**允许例外（须在文档「与官方不一致」节列出）：**

- 字体：WASM 不读系统字体；需要 `registerFont`（可挂在 echarts 命名空间）。
- 动画：不播中间帧；`setOption` / `animation` 直接终态。`lazyUpdate` 可同步执行（等价立刻 flush）。
- 离屏：仅 canvas；`init(canvas)` 自动 `putImageData`。无 SVG（`renderToSVGString` / `getSvgDataURL` 不实现，文档标明）。
- 宿主：`init(canvas, theme?, opts?)`；`init(null, null, { width, height, devicePixelRatio })` 允许离屏（官方客户端无 dom 会抛错）。
- `use(...)`：**导出且签名对齐**，但不按需加载。实现只打印提示（例如 `console.info`）：已开发的模块都在 WASM 里，不必 `use`。调用可忽略参数并立即返回。不为减小体积做动态 `use`。
- `getZr()` 因 crate 隔离（wasm-echarts 不依赖 wasm-zrender）本波不导出，文档写「未实现」。

**必须一致（公开 JS 表面）：**

- 入口：`init` / `dispose` / `getInstanceByDom` / `getInstanceById` / `version`（`'6.1.0'`）/ `use`（提示实现）。
- 实例方法 camelCase，签名对齐：`setOption` / `getOption` / `resize` / `dispatchAction` / `on` / `off` / `getWidth` / `getHeight` / `getDevicePixelRatio` / `isDisposed` / `clear` / `dispose`。
- `setOption(option)` 与 `setOption(option, notMerge)` / `setOption(option, { notMerge, replaceMerge, silent })`。`notMerge` **不是** option 里的字段（当前 [`option/mod.rs`](wasm-echarts-rs/crates/wasm-echarts/src/option/mod.rs) 从 option 根上剥 `notMerge`，这是错的）。
- `resize()` 无参（读 canvas 尺寸）与 `resize({ width, height, devicePixelRatio })`。
- `init(canvas)` 后指针事件由 facade 绑定；发出官方事件名 `click` / `mouseover` / `mouseout` / `globalout`。
- 已接线的 `dispatchAction` type 名保持官方字符串：`highlight` / `downplay` / `select` / `unselect` / `toggleSelect` / `dataZoom`；并补 `showTip` / `hideTip`。

**文档硬规则（与实现对齐同等重要）：**

公开文档（[`site/echarts/docs/index.html`](wasm-echarts-rs/site/echarts/docs/index.html)、根 README、AGENT.md）必须始终有这四块，实现变更时同步改：

1. **与官方一致的公开 API**：可按官方文档调用的方法与签名。
2. **与官方不一致 / 例外**：`use` 提示语义、`init(null)`、无 SVG、动画终态、字体、`lazyUpdate` 同步等。
3. **多出来的非官方 API**：`refresh`、`findHover` / `handlePointerMove` 等 WASM hatch；写清用途、何时该用、何时不该当官方 API。
4. **已实现 / 未实现**：图表类型、组件、`dispatchAction` type、option 字段生效范围。未实现的官方方法仍尽量导出同名，内部 `console.warn` 并在此表标「未实现」。

**本波不挡主路径（后置，不混进「已对齐」，但要出现在「未实现」表）：** `connect`/`disconnect`、`setTheme`/`registerTheme`、`registerMap`、`appendData`、`convertToPixel` 完整 finder、`getDataURL`/`renderToCanvas`、`showLoading`、`graphic`/`util`/`number`/`format` 命名空间、`registerPreprocessor` 等扩展注册、polar/gauge/其余 chart、legend 绘制、media query、完整 SeriesData、`getZr()`。

---

## 清单一：公开 JS API（相对 `core/echarts.ts`）

### 模块级

| 官方 | wasm-echarts 现状 | 判定 |
|------|-------------------|------|
| `init(dom, theme?, opts?)` | 无；`new EChartsInstance(w, h, dpr)` | 不一致 |
| `dispose(chart \| dom \| id)` | 仅实例 `dispose()`，无模块函数、无实例表 | 不一致 |
| `getInstanceByDom` / `getInstanceById` | 无 | 不一致 |
| `version` / `dependencies` | 无 | 不一致 |
| `connect` / `disconnect` | 无 | 后置（文档标未实现） |
| `registerTheme` / `registerMap` / `getMap` / `registerLocale` | 无 | 后置（文档标未实现） |
| `use(...)` | 无 | **本波导出**：只 `console.info` 提示已编进 WASM，不必 `use` |
| `registerAction` / `registerPreprocessor` / `registerProcessor` / `registerLayout` / `registerVisual` / `registerLoading` / `registerCoordinateSystem` / `registerCustomSeries` / `registerTransform` | 无 | 后置（文档标未实现；不为体积做动态加载） |
| `setPlatformAPI` / `setCanvasCreator` | 无 | 后置；离屏不创建 DOM canvas |
| `PRIORITY` / `dataTool` | 无 | 后置 |
| `export/api.ts`：`graphic` `util` `number` `time` `format` `helper` `matrix` `vector` `color` `env` `parseGeoJSON` `Model` `ChartView`… | 无 | 后置（工具/扩展） |

### 实例方法

| 官方 | wasm-echarts 现状 | 判定 |
|------|-------------------|------|
| `setOption(opt, notMerge?, lazyUpdate?)` 或 opts 对象 | `set_option(opt)`；把 `notMerge` 当 option 字段剥掉 | 名字与签名都不一致 |
| `getOption()` | 无 | 不一致 |
| `resize(opts?)` | `resize(w, h, dpr)` 三位置参数 | 不一致 |
| `getWidth` / `getHeight` / `getDevicePixelRatio` | `width()` / `height()` / `dpr()` | 语义近、名字不一致 |
| `dispatchAction(payload, opt?)` | `dispatch_action`；已实现 6 个 type，其余 `console.warn`；无 silent/flush | 部分一致 |
| `on` / `off` / `trigger`（Eventful） | 无；交互靠 `handle_pointer_move` 返回值 | 不一致 |
| `clear` / `isDisposed` / `dispose` | 仅 `dispose`（不清 Storage / 无 disposed 标志） | 部分 |
| `getDom` / `getId` / `group` | 无 | 不一致（init 后应有） |
| `convertToPixel` / `convertFromPixel` / `containPixel` / `convertToLayout` | 无 | 后置（cartesian 可先做最小版） |
| `getVisual` | 无 | 后置 |
| `appendData` | 无 | 后置 |
| `setTheme` | 无 | 后置 |
| `showLoading` / `hideLoading` | 无 | 明确不做（DOM） |
| `getDataURL` / `getConnectedDataURL` / `renderToCanvas` / `renderToSVGString` | 无；有内部 `refresh()`→RGBA | `refresh` 作例外 hatch；DataURL 后置 |
| `getZr` / `isSSR` / `updateLabelLayout` / `makeActionFromEvent` | 无 | 后置 / 不做 SSR |

### 当前多出来的非官方 API（必须文档化）

官方没有、本仓库因 WASM 离屏需要而多出来的方法，**不冒充官方 API**，但要在文档「多出来的非官方 API」里写清。

[`instance.rs`](wasm-echarts-rs/crates/wasm-echarts/src/instance.rs) 现有：`refresh`、`find_hover`、`handle_pointer_move`、`handle_pointer_leave`、`apply_data_zoom_wheel`、`get_tooltip_content`、`benchmark_render`、`has_option`、`option_has_functions`。

处理原则：

- **主路径消化**：`init(canvas)` 后自动上屏与绑指针，普通用户不必再调 `handle_pointer_*` / 手动 `putImageData`。
- **仍公开并文档化**：离屏无 canvas、自绘 tooltip、bench 等场景需要 hatch。facade 用 camelCase 再导出一层（如 `refresh`、`findHover`、`handlePointerMove`、`handlePointerLeave`、`applyDataZoomWheel`、`getTooltipContent`、`benchmarkRender`），并注明「非官方」。
- `registerFont` 同样是 WASM 例外，放进「与官方不一致」+「非官方补充」。

---

## 清单二：option / 图表 / 交互（相对官方 option 与 `export/charts.ts`）

「能 parse 进来」≠「按官方语义生效」。下面按 **生效程度**。

### 已接近官方形态（部分生效）

- **入口 option 树**：任意 JSON + 函数字段可进 [`OptionValue`](wasm-echarts-rs/crates/wasm-echarts/src/option/parse.rs)（这点与官方「option 含 formatter 函数」一致）。
- **深合并**：对象深合并、数组按 index 合；函数覆盖。缺官方 `replaceMerge` 真正按 component `id` 替换、缺 `notMerge` 第二参数。
- **series.type**：`line` / `bar` / `pie` / `scatter`（feature flag 与官方 chart install 对应，但是写死在 rust，不是 `echarts.use(BarChart)`）。
- **cartesian**：单 `xAxis`/`yAxis`；`type: category | value`；`grid.left/right/top/bottom`。
- **line**：Polyline + 固定半径 Circle 符号；`itemStyle.color` / `lineStyle.color` 可为函数。
- **bar**：Rect，带宽 0.6。
- **pie**：Sector；默认 startAngle 90°、顺时针（与官方默认一致）；半径由 grid 推，不是 `radius`/`center`。
- **scatter**：双 value 轴；固定 `symbolSize` 6。
- **tooltip.formatter**：返回 **string** 时可用；[`CallbackDataParams`](echarts-master/src/util/types.ts) 目前只有 `seriesIndex/dataIndex/seriesName/name/value/color`，缺 `componentType`、`percent`、`data`、`encode`、`$vars` 等。
- **emphasis/select**：图元 state patch + `dispatchAction` 六个 type。
- **dataZoom**：option 里 `type: 'inside'` 时滚轮改 start/end **百分比窗口**；无 slider UI、无 `startValue`/`endValue`/`xAxisIndex` 完整语义。
- **axisPointer**：option 开启时画 **竖线**；无十字、无 label 浮层、无 `trigger: 'axis'` tooltip。

### 名字在 option 里出现但未按官方做

- `setOption` 的 `notMerge` / `lazyUpdate` / `replaceMerge` / `transition`：被当 option 字段 `strip_meta_keys`，官方它们是 **第二参数**。
- `axisLabel.formatter`：bridge 有 resolve，轴渲染仍用类目/数值字符串。
- `series.label` / `labelLine`：`resolve_label` 有，图元未画。
- `symbol` / `symbolSize`：忽略，line/scatter 写死圆。
- pie：`radius` / `center` / `roseType` / `startAngle` / `clockwise` / `label` 未读。
- `tooltip.trigger: 'axis'`、HTMLElement formatter、`confine`、`position`。
- `color` 色板：只用默认色 + itemStyle，不是官方 palette 全套。
- 多 `grid` / 双 y 轴 / `xAxis: []` 数组。

### 官方有、本 crate 完全没有（后置）

**Charts**（[`export/charts.ts`](echarts-master/src/export/charts.ts)）：Radar、Map、Tree、Treemap、Graph、Chord、Gauge、Funnel、Parallel、Sankey、Boxplot、Candlestick、EffectScatter、Lines、Heatmap、PictorialBar、ThemeRiver、Sunburst、Custom（`renderItem`+`api`）。

**Components**（[`export/components.ts`](echarts-master/src/export/components.ts)）：legend（含绘制与 `legendToggleSelect`）、title、toolbox、visualMap、geo、polar、radar、singleAxis、calendar、graphic、brush、timeline、markPoint/Line/Area、dataset/transform、aria、thumbnail。grid 只有边距矩形，不是完整 Grid 组件。

**Actions 未接**：`showTip`/`hideTip`、legend\*、restore、brush、timeline、geo roam、pie 选中联动等。

---

## 补齐架构：JS facade（与 wasm-zrender 同一套路）

新建 [`wasm-echarts-rs/crates/wasm-echarts/js/`](wasm-echarts-rs/crates/wasm-echarts/js/)：

- `index.js`：官方核心命名导出；`default` 仍是 wasm-bindgen `initWasm`。
- `echarts.js`：`init` / `dispose` / `getInstanceByDom` / `getInstanceById` / `version` / `use`（只打印提示）；实例表。
- `instance.js`：包装 native `EChartsInstance`；camelCase；Eventful 风格 `on`/`off`；`init(canvas)` 绑 pointer + 自动 `putImageData`；wheel → 内部 `apply_data_zoom_wheel`。非官方 hatch 亦 camelCase 导出。
- 兼容：可再 `export { EChartsInstance }` 以免旧示例立刻碎，但文档与 site **改走官方写法**。

`use` 实现约定（不要做动态 import / 不要按 feature 再编 wasm）：

```javascript
export function use() {
  console.info(
    '[wasm-echarts] 已开发的图表与组件都编进 WASM，无需 echarts.use(...)。'
  );
}
```

[`site/vite.config.js`](wasm-echarts-rs/site/vite.config.js) 的 `@wasm-echarts` 改为指向 `js/`（同 zrender）。native 仍是 `pkg/`。

Rust 侧用 `js_name` 或保留 snake_case 给 native、由 facade 翻译均可；**不要**让 site 继续依赖 `set_option`。

---

## 逐项波次

每波：改 rust（若需要）→ facade → 改 site 示例 → 更新 AGENT.md。做完一波再开下一波。

### 波次 0 — 规范落盘

把上节写入 [`AGENT.md`](AGENT.md)「目标与约束」和「三、wasm-echarts」；本计划作为逐项清单。改掉「只有 EChartsInstance」的过时用法说明。文档四块（一致 / 不一致 / 非官方 API / 已实现与未实现）先搭好标题，后续波次填表。

### 波次 1 — facade 骨架 + 入口

- `init` / `dispose` / `getInstanceByDom` / `getInstanceById` / `version` / `use`（`console.info` 提示，无加载逻辑）。
- 实例：`setOption`→native `set_option`，`resize`/`getWidth`/`getHeight`/`dispose` 转调。
- `init(canvas)` 自动上屏（与 zrender 相同例外）。
- 非官方 hatch 同步 camelCase 导出，并在文档列名。
- site：[`bar.js`](wasm-echarts-rs/site/echarts/examples/bar.js) 等改为官方写法。

### 波次 2 — `setOption` / `resize` 签名

- `setOption(option, notMerge | opts)`；停止从 option 根读取 `notMerge`。
- `getOption()`：把 `OptionValue` 转回普通 JSON（函数字段可省略或保留为原 Function）。
- `resize()` / `resize({ width, height, devicePixelRatio })`；有 canvas 时无参读 `clientWidth/Height`。
- `clear` = `setOption({ series: [] }, true)`；`isDisposed`。
- `replaceMerge` 最小：顶层 key 整段替换（比官方按 id 弱，**文档标不一致**）。

### 波次 3 — 事件 + 指针（主路径消化，hatch 仍文档化）

- facade `on`/`off`：`click`、`mouseover`、`mouseout`、`globalout`。
- `init(canvas)`：mousemove/click/leave/wheel 全部进 facade；payload 尽量像官方 `ECElementEvent`（`event`、`seriesIndex`、`dataIndex`）。
- tooltip：facade 内建简单 DOM（string HTML），用户不必再抄 [`interactive.js`](wasm-echarts-rs/site/echarts/examples/interactive.js) 那套 `handlePointerMove`。HTMLElement formatter 仍后置。
- `dispatchAction({ type: 'showTip' | 'hideTip', seriesIndex, dataIndex })`。
- `handlePointerMove` 等仍导出，文档写「非官方；init(canvas) 时一般不需要」。

### 波次 4 — 已支持图表的 option 语义（画出来就错的字段）

只补 **当前 4 类图**，不新开 chart 类型：

- 轴：`axisLabel.formatter` 真正进 Text。
- pie：`center` / `radius` / `startAngle` / `clockwise` / `r0`（环形）。
- line/scatter：`symbol`（circle/rect/emptyCircle 先做）与 `symbolSize`（数或函数）。
- series `label.show` + formatter 画出 Text；pie `labelLine` 可再下一小步。
- `CallbackDataParams` 补 `componentType`/`seriesType`/`percent`（pie）/`data`。
- cartesian：`convertToPixel` / `convertFromPixel` 的 `xAxis`/`yAxis`/`grid` finder 最小集。

### 波次 5 — 文档与验收

文档必须写清四块（site 文档页为主，README / AGENT.md 同步）：

- **与官方一致**：`init` / `setOption` / `resize` / `dispatchAction` / `on` / `off` / `dispose` / `use`（仅提示）等可照官方调用的部分。
- **与官方不一致**：`use` 不加载模块、`init(null)`、动画终态、仅 canvas、字体、`lazyUpdate` 同步、`replaceMerge` 弱语义等。
- **多出来的非官方 API**：`refresh`、`findHover`、`handlePointerMove`、`handlePointerLeave`、`applyDataZoomWheel`、`getTooltipContent`、`benchmarkRender`、`registerFont` 等。
- **已实现 / 未实现**：line/bar/pie/scatter 与哪些 option 字段生效；legend/polar/其余 chart/`getZr`/`connect` 等标未实现。

其它：

- 示例：line/bar/pie/scatter/interactive/merge/bench 走 facade；interactive 优先 `on('click')`，不必再调 `handlePointer*`。
- 手工：`init(canvas)` 出图、二次 `setOption` 合并、`notMerge: true`、click `on`、`echarts.use()` 只出提示、`dispatchAction('toggleSelect')`、wheel dataZoom、dispose 后再 init。

### 明确仍后置（不进本计划验收，但进「未实现」表）

legend / title / polar / gauge / 面积图 / Custom `renderItem` / media / SeriesData 全管道 / `getZr` / `connect` / 主题 / `getDataURL` / Loading / 其余 20 种 chart / `registerPreprocessor` 等扩展注册。需要时另开计划，与「公开 API 已对齐」分开说。
