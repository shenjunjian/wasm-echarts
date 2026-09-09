# wasm-echarts-rs

Rust / WebAssembly workspace：用纯 Rust 重写 zrender 离屏 canvas 渲染内核，再通过 wasm-bindgen 分别暴露 **zrender 图元 API** 与 **ECharts option 管线**。目标是尽快把图表画到 canvas 上，**不追求动画中间帧**，也不做 SVG / DOM 组件。最主要的宗旨是：wasm-zrender, wasm-echarts的目标是要尽量和官方API保持一致。

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
| API 规范对齐 | `zrender_api_规范对齐_be3227a1.plan.md` | 硬规范 + JS facade；逐项清单以该计划为权威 |

API 规范对齐规划的 YAML todo 已全部 completed。其它规划 YAML 里部分 todo 仍可能标 `pending`，以**源码为准**。下文「规划对照」会标明实际完成度。

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
│   │   └── wasm-echarts/             # wasm-pack cdylib：EChartsInstance + option
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
| **wasm-echarts** | `crates/wasm-echarts/` | `cdylib` + `rlib` | `crates/wasm-echarts/pkg/`（JS + WASM） |
| **site** | `site/` | Vite 多页静态站 | 开发时直连 `pkg/`；`npm run build` 输出 `site/dist/` |

依赖规则（已落地）：

```
site  ──import──►  wasm-zrender/js      （Vite alias @wasm-zrender）
                     └──► pkg/          （wasm-bindgen 内部 handle）
site  ──import──►  wasm-echarts/pkg     （Vite alias @wasm-echarts）

wasm-zrender  ──path──►  rust-zrender
wasm-echarts  ──path──►  rust-zrender

wasm-echarts  ✗ 不依赖  wasm-zrender
wasm-zrender  ✗ 不依赖  wasm-echarts
```

echarts 图表管线在 Rust 里直接 `use rust_zrender::ZRenderer`，不经过 wasm-zrender 的 JS 图元类。wasm-zrender 只给「手动画 Rect/Circle」这类底层 Demo / 文档站实例用。

---

## 目标与约束

与仓库根 `README.md` 及总规划一致：

- **核心目标**：把 ECharts `option`（以及 zrender 图元树）快速画到 canvas。公开 API 尽量与官方一致。
- **渲染模式**：仅 canvas。忽略 SVG painter、Loading / DataView 等 DOM 组件。
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

---

## 架构与数据流

```
浏览器 DOM / 指针事件
        │
        ▼
site 示例 JS（创建 canvas、putImageData；交互示例自行绑 pointer / tooltip DOM）
        │
        ├── wasm-zrender：init / Group / Rect / … / refresh / findHover
        └── wasm-echarts：EChartsInstance.set_option / handle_pointer_move / …
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
| 布局 / 坐标 / 绘制 | cartesian、ChartView、Painter | — |
| 命中检测 | Path winding + stroke 距离；Image/Text bbox | 转发 pointer 坐标 |
| tooltip / 高亮 | formatter 得 string；改 element state 再 refresh | DOM 定位与 HTML（见 interactive 示例） |
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
| **4 JS 薄壳 + API 骨架** | init/setOption/resize/事件、`OptionValue` | **已完成**。`EChartsInstance` WASM API；site 示例直接 import `@wasm-echarts` |
| **4b 回调桥** | CallbackDataParams、call0/1/2 | **部分完成**。`formatter` / `itemStyle.color` 可用；`renderItem`/`api`、`axisLabel.formatter` 接入、per-series 缓存 **未完成** |
| **5 echarts MVP** | GlobalModel、cartesian、line/bar、Scheduler | **部分完成**。line/bar 可渲染；SeriesData/Source 完整管道、media query、lazyUpdate/replaceMerge 完整语义 **未完成** |
| **6 交互完善** | hover/tooltip/dataZoom/axisPointer | **部分完成**。hover 高亮、toggleSelect、string tooltip、wheel inside dataZoom、竖线 axisPointer。pinch、slider、HTMLElement tooltip、十字/多轴 **未完成** |
| **7 扩展与优化** | pie/scatter、RichText、脏矩形、视觉回归 | **部分完成**。pie/scatter、轴标签 Text、`benchmark_render`、feature flags。legend、gauge、polar、面积图、RichText、脏矩形、golden PNG **未完成** |

### wasm-zrender API 规范对齐（波次 0–6 + 文档验收已完成）

规范已写入上文「目标与约束」；JS facade 骨架在 `crates/wasm-zrender/js/`，site / README 从 `@wasm-zrender`（`js/index.js`）导入，`pkg/` 只作内部 handle。波次 2：变换主属性、`attr`/`setShape`/`setStyle` 双参数、Group opts 与子树 API。波次 3：`Sector.r0` / `clockwise` / `cornerRadius`、`Rect.r`、Polygon.smooth、Line·Text 默认 style、`miterLimit`、`lineDash` 字符串、`LinearGradient.addColorStop`。波次 4：`js/tool/` 按官方签名实现 `matrix` / `vector` / `color` / `util` / `path`；`morph` / `parseSVG` / `showDebugDirtyRect` / `setPlatformAPI` 为签名齐全的最小实现。波次 5：`Animator` 写最后一组 `when`；`zr.clear` / 实例 `dispose` / `setBackgroundColor` / `trigger`；`el.hide`/`show`/`off`/`trigger`。波次 6：`getClipPath` / `removeClipPath`，clip 扩到 Group/Text/Image（Group clip 对子树生效）；`useStates` / `getState` / `ensureState` / `clearStates`；`zr.setCursorStyle` / `configLayer`；`Path.extend` 把 `buildPath` 录成 pathData；`IncrementalDisplayable` 按普通 Group 语义可构造；Point 静态方法、`BoundingRect.calculateTransform`。文档验收：AGENT.md / README / site 文档与导入路径已同步四条例外；`shapes` / `text` / `animation` / `bounding_box` 示例覆盖原型链、`Group.x`、`Rect.r`、`Sector.r0`、动画终态、`init(canvas)` 拖拽包围盒。余下后置项见该计划「本波不挡主路径」。

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

`docs-site` / `demo-zrender` / `demo-echarts` 在规划 YAML 仍为 pending，**site 工程已落地**：首页缘由/限制 + 双产品文档/实例，实例页左源码右预览。

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

纯 Rust 的 zrender 渲染库。**没有** `wasm-bindgen`，可在原生 `cargo test`、未来 Wasmer / 服务端离屏场景直接使用。另外两个 WASM crate 都只依赖这一层。

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

`pkg/` 在 crate `.gitignore` 中（`/pkg`）。本地开发需自己 `wasm-pack build`。中间缓存仍在 workspace `target/`。

`package.json` 使其**可以**当 npm 包（`file:` 或发布到 registry）。当前文档站**没有**把它写进 `site/package.json` 的 `dependencies`，而是用 Vite alias 直接指到这个目录（见「文档站如何引用产物」）。

### 文档站如何用到本 crate

1. Vite alias `@wasm-zrender` → `crates/wasm-zrender/js`（`js/index.js` 再 import `../pkg/wasm_zrender.js`）
2. 每个实例是独立完整脚本：`site/zrender/examples/shapes.js` 等同名 HTML 成对出现，直接 `import` facade、建 canvas、构图、`refresh` → `putImageData`。canvas 位图与 `init` 宽高一律用 **CSS 像素**，不乘 `devicePixelRatio`（内核缓冲和命中检测都是 CSS 坐标；示例里乘 dpr 会导致画面与点选错位）
3. 画廊 `gallery.js` 用 Vite `?raw` 读这些 `.js` 作为左侧源码，iframe 加载同目录 HTML 预览
4. 字体：`text.js` 内联 `fetch` + `registerFont`；可选辅助 `site/src/zrender/fonts.js` 默认拉取 `/fonts/NotoSansSC-Regular.ttf`

实例页：`/zrender/examples/hello_world.html`、`animation.html`、`bounding_box.html`、`clip_path.html`、`glitched_text.html`、`particles.html`、`shapes.html`、`text.html`、`sector.html`、`hit.html`、`state.html`。

---

## 三、wasm-echarts

### 目的

浏览器侧的 ECharts **canvas MVP**：接收官方形态的 `option`（含 JS 函数字段），在 Rust 里做 merge → GlobalModel → ChartView → `rust_zrender::ZRenderer`，再把 RGBA 交还给 JS。交互（hover / select / inside dataZoom / tooltip 字符串）也在 WASM 内算完，JS 只做 DOM。

**不依赖** wasm-zrender。图表图元是 Rust 里直接 `storage.create_path`，不是 JS `new Rect()`。

Cargo features（默认全开）：

```
chart-line, chart-bar, chart-pie, chart-scatter
```

可按需裁剪以减小 WASM 体积。

### 实现了哪些内容

源码：`crates/wasm-echarts/src/`。

#### `instance.rs` — `EChartsInstance`（wasm 导出）

| 方法 | 说明 |
|------|------|
| `new(width, height, dpr)` | 内部创建 `ZRenderer` |
| `set_option(option)` | 解析 + merge，全量 render |
| `refresh()` | RGBA |
| `resize(w, h, dpr)` | 改画布并重绘 |
| `find_hover(x, y)` | `{ seriesIndex, dataIndex, pathIndex, ... }` |
| `handle_pointer_move(x, y)` | hover 高亮 + axisPointer + tooltip 文案，一次返回 |
| `handle_pointer_leave()` | 取消 hover |
| `get_tooltip_content(si, di)` | 调 `tooltip.formatter`，string 或 null |
| `apply_data_zoom_wheel(x, deltaY)` | option 含 inside dataZoom 时缩放 category 窗口 |
| `dispatch_action(action)` | 见下表 |
| `benchmark_render(n)` | 全量 render+refresh 平均毫秒 |
| `has_option()` / `option_has_functions()` / `dispose()` | 状态与释放 |
| `width()` / `height()` / `dpr()` | 尺寸 |

`dispatch_action` 已实现：`highlight`、`downplay`、`select`、`unselect`、`toggleSelect`、`dataZoom`（`start`/`end` 百分比）。其它 type 会 `console.warn`。

#### `option/` — 解析与合并

- `OptionValue`：Null / Bool / Number / String / Array / Object / **Function**
- `parse_option_value`：递归走 `JsValue`，不能用 serde 整包反序列化
- `merge_option`：深合并；函数字段用新值覆盖；数组按 index 合并对象
- `notMerge` 会替换整棵树；`lazyUpdate` / `replaceMerge` / `transition` 作为 meta 键剥掉（完整语义尚未做）

#### `bridge/` — 回调

- `JsCallback`：`call_formatter`、`call_color`、`call_render_item`、`call_axis_formatter`
- `build_data_params`：`seriesIndex` / `dataIndex` / `seriesName` / `name` / `value` / `color`（尚无 percent、encode 等完整 CallbackDataParams）
- `resolve_color` / `resolve_formatter` / `resolve_axis_formatter`
- `try_call_formatter`：JS throw 时 `console.error` 并降级

**已接线**：`itemStyle.color` / `lineStyle.color`、`tooltip.formatter`（string）、`label.formatter` 有 resolve（系列 label 图元绘制仍弱）。

**未接线或未完成**：`axisLabel.formatter` 虽有 resolve 函数，轴渲染仍用类目字符串/数值格式化；`symbol` / `symbolSize`；`series.renderItem` + `api`；tooltip 返回 HTMLElement；常量回调缓存。

#### `model/` + `scheduler.rs` + `render.rs`

- `GlobalModel`：grid 矩形、单 x/y 轴、category 或 value、`Vec<SeriesModel>`、dataZoom 窗口
- `DataPoint` 为简化 vec（value / 可选 x_value / name），不是完整 SeriesData/Source
- Scheduler：单次全量 `run_update` → `render_chart`（先 `Storage::new()` 再重建整棵树，无增量）

#### `coord/` — `Cartesian2D`

- category + value → 像素（line/bar）
- 双 value → 像素（scatter）
- 配合 dataZoom 的可见类目窗口

无 polar。

#### `chart/` — ChartView

| 类型 | 图元 | 说明 |
|------|------|------|
| line | Polyline + Circle 符号 | 色回调、emphasis/select 样式补丁、ECData |
| bar | Rect | 带宽 0.6、基线、ECData |
| pie | Sector | 默认 startAngle 90° 顺时针；半径由 grid 推；无 roseType / labelLine |
| scatter | Circle | 双 value 轴；固定 symbol 半径 6 |
| 组件 | 网格框、y 向 splitLine、轴标签 Text、竖线 axisPointer | legend / dataZoom slider 未画 |

轴标签走 `ChildRef::Text` 挂到 group，`silent = true`（不抢 hover）。

#### `visual/` + `interaction.rs`

- 色板默认色 + 每点 `resolve_item_color`
- hover / 多选 set、inside dataZoom 百分比窗口、滚轮缩放、axisPointer 是否开启（读 option）

#### site 示例直接使用 `@wasm-echarts`

每个 `site/echarts/examples/*.js` 都是完整独立脚本，不抽公共薄壳。导入包后自行建 canvas、`set_option`、`refresh` + `putImageData`。交互合集 `interactive.js` 内联：mousemove → `handle_pointer_move`、click → `toggleSelect`、wheel → `apply_data_zoom_wheel`、tooltip DOM。

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

同样通过 `site`。或在任意 ESM 页：

```javascript
import initWasm, { EChartsInstance } from '@wasm-echarts';

await initWasm();
const chart = new EChartsInstance(480, 360, 1);
chart.set_option({
  xAxis: { type: 'category', data: ['A', 'B', 'C'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', data: [10, 20, 30] }],
});
const rgba = chart.refresh();
ctx.putImageData(new ImageData(new Uint8ClampedArray(rgba), chart.width(), chart.height()), 0, 0);
```

指针事件与 tooltip DOM 不封装进公共层；需要交互时照 `interactive.js` 在同一文件里绑定。

### 编译产物在哪里

```
crates/wasm-echarts/pkg/
├── package.json           # name: "wasm-echarts"
├── wasm_echarts.js
├── wasm_echarts.d.ts      # 仅 EChartsInstance + default init
├── wasm_echarts_bg.wasm
└── wasm_echarts_bg.wasm.d.ts
```

crate `.gitignore` 含 `pkg/`。改 Rust 后必须重新 wasm-pack，浏览器硬刷新。

### 文档站如何用到本 crate

1. Vite alias `@wasm-echarts` → `crates/wasm-echarts/pkg/wasm_echarts.js`
2. 每个实例 JS 直接：

```javascript
import initWasm, { EChartsInstance } from '@wasm-echarts';
```

3. 每个实例是独立完整脚本：`site/echarts/examples/line.js` 等同名 HTML 成对出现（`<canvas id="canvas">`），自行 `initWasm` + `new EChartsInstance` + `set_option` + `refresh`
4. 画廊 `gallery.js` 用 Vite `?raw` 读这些 `.js` 作为左侧源码，iframe 加载同目录 HTML 预览
5. 页面：line / bar / pie / scatter / interactive / merge / bench

---

## 四、site 文档站

### 目的

对外说明项目缘由与限制，并提供 **wasm-zrender**、**wasm-echarts** 两套文档 + 左源码右预览实例。由规划中的 `demo/` 迁来，现为 Vite 多页工程。

### 实现了哪些内容

```
site/
├── index.html                 # 首页：缘由、限制、双产品入口
├── package.json               # vite、shiki；不声明 wasm npm 依赖
├── vite.config.js             # 多页 HTML 入口 + alias + wasm MIME
├── public/                    # 静态资源（字体等，按本地/部署准备）
├── src/
│   ├── shared/                # 布局 CSS、画廊 UI、源码高亮
│   └── zrender/fonts.js       # 可选：字体加载辅助
├── zrender/
│   ├── index.html
│   ├── docs/index.html
│   └── examples/              # 每个示例 = html + 完整 js
│       ├── gallery.js
│       ├── shapes.html / shapes.js
│       ├── text.html / text.js
│       └── …
└── echarts/
    ├── index.html
    ├── docs/index.html
    └── examples/              # 每个示例 = html + 完整 js
        ├── gallery.js
        ├── line.html / line.js
        └── …
```

- 首页不做 API 长文、不嵌 canvas
- 实例画廊：左侧菜单 + 源码（即该示例 `.js` 全文），右侧 iframe 预览
- 每个示例 JS 自包含：导入、构图、绘制、交互都写在同一个文件里

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

**wasm-echarts**

| 页 | 验证点 |
|----|--------|
| line / bar / pie / scatter | 对应 ChartView |
| interactive | formatter tooltip + hover / select / wheel zoom |
| merge | 二次 `set_option` 深合并 |
| bench | `benchmark_render(30)` |

规划里的独立 `function-option.html` 未单列，函数 option 合在 interactive。

### 如何编译 / 启动

```bash
cd wasm-echarts-rs/site
npm install          # 或 pnpm install
npm run dev          # http://127.0.0.1:5173/
npm run build        # 输出 site/dist/
npm run preview      # 预览构建结果
```

**必须先**对两个 WASM crate 执行 `wasm-pack build --target web`，否则 facade 引用的 `pkg/wasm_zrender.js` / `@wasm-echarts`（`pkg/wasm_echarts.js`）不存在，Vite 会解析失败。

浏览器入口：

| 页面 | URL |
|------|-----|
| 首页 | http://127.0.0.1:5173/ |
| zrender 文档 | http://127.0.0.1:5173/zrender/docs/ |
| zrender 实例 | http://127.0.0.1:5173/zrender/examples/ |
| echarts 文档 | http://127.0.0.1:5173/echarts/docs/ |
| echarts 实例 | http://127.0.0.1:5173/echarts/examples/ |

`scripts/serve-demo.sh` 仍 `wasm-pack` wasm-echarts 后用 `python -m http.server` 指到已不存在的 `demo/`，不要用它启动文档站。

### 站点自己的产物

`npm run build` → `site/dist/`（gitignore）。其中会打包对 `pkg/*.js` 与 `.wasm` 的引用；部署时需保证构建时 `pkg/` 已生成，且 WASM MIME 为 `application/wasm`（dev server 已用插件设置）。

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

resolve: {
  alias: {
    '@wasm-zrender': resolve(repoRoot, 'crates/wasm-zrender/js'),
    '@wasm-echarts': resolve(repoRoot, 'crates/wasm-echarts/pkg/wasm_echarts.js'),
  },
},
server: {
  fs: { allow: [repoRoot] },  // 允许开发服务器读 site 目录以外的 js/ 与 pkg/
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
import initWasm, { EChartsInstance } from '@wasm-echarts';
```

会解析为：

```
crates/wasm-zrender/js/index.js  →  ../pkg/wasm_zrender.js
crates/wasm-echarts/pkg/wasm_echarts.js
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
    "wasm-echarts": "file:../wasm-echarts-rs/crates/wasm-echarts/pkg"
  }
}
```

然后：

```javascript
import initZrender, { init, Rect } from 'wasm-zrender';
import initEcharts, { EChartsInstance } from 'wasm-echarts';
```

（具体子路径以该包 `main` 为准。）本仓库的 site 用 alias：`@wasm-zrender` → `js/`，`@wasm-echarts` → `pkg/wasm_echarts.js`，避免 workspace 安装步骤。改完 Rust 只需重新 wasm-pack。只改 `js/` 不必重编 WASM。

实例 JS 与 HTML 放在同一目录（如 `zrender/examples/shapes.js`、`echarts/examples/line.js`），画廊用 `?raw` 读取这份脚本作为源码展示。echarts 示例直接 `import initWasm, { EChartsInstance } from '@wasm-echarts'`，不抽公共薄壳。

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

- legend 绘制；dataZoom slider UI；pinch；`showTip` / `hideTip`
- axisPointer 十字 / 多轴；tooltip HTMLElement / confine / `trigger: 'axis'`
- `axisLabel.formatter` 真正画轴；series label / labelLine
- `symbol` / `symbolSize`；CustomSeries `renderItem` + `api`
- media query；完整 SeriesData；面积图；gauge；polar；多 grid / 双 y 轴
- 视觉回归（echarts `test/*.html` → golden PNG）；JS vs WASM 基准报告

### 明确不做

- SVG 渲染 / hover layer / dirty rect
- 动画中间帧（终态语义要做；`morph` 形变后置）
- Loading、DataView 等 DOM 组件
- 官方 ECharts 全量 API（feature flag 扩展，而不是一次移植完）

---

## 源码对照（只读）

实现时按行为参考 TypeScript，Rust 侧按习惯重写，不做 1:1 文件映射。常用对照：

- zrender：`Storage.ts`、`canvas/Painter.ts`、`Handler.ts`、`core/PathProxy.ts`、`graphic/shape/*.ts`、`src/export.ts`
- echarts：`core/echarts.ts`、`model/Global.ts`、`chart/line/LineView.ts`、`chart/bar/BarView.ts`、`util/types.ts`（CallbackDataParams）
