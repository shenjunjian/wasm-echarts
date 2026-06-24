---
name: wasm-echarts 待办与 Demo 规划
overview: 基于 [wasm-echarts_移植规划_692fb54c.plan.md](wasm-echarts_移植规划_692fb54c.plan.md)，wasm-zrender / wasm-echarts 核心开发已基本完成；**当前最高优先级为 `site/` 文档官网**（首页缘由/限制 + 双产品文档/实例，实例页左源码右预览）。本文件梳理剩余 backlog 与推荐实施顺序。
todos:
  - id: docs-site
    content: 文档官网：site/ 站点框架 + 首页（缘由/限制）+ 双产品入口
    status: pending
  - id: demo-zrender
    content: site/zrender：文档版块 + 实例页（左源码右预览，图形/文本/命中/交互）
    status: pending
  - id: demo-echarts
    content: site/echarts：文档版块 + 多实例页（左源码右预览，分图表/交互/函数 option/基准）
    status: pending
  - id: zrender-wasm-api
    content: wasm-zrender：ZRenderInstance WASM 薄封装（load_scene/find_hover/resize）；待补 add_element 动态 API
    status: pending
  - id: zrender-backend-gaps
    content: rust-zrender：conic 渐变、Text 场景图集成、字体 JS 注入、CSS filter（可选）
    status: pending
  - id: echarts-callback-full
    content: echarts 桥接：renderItem/api、axisLabel.formatter、symbol/symbolSize、formatter 缓存
    status: pending
  - id: echarts-model-pipeline
    content: echarts 核心：SeriesData/Source 完整管道、media query、lazyUpdate/replaceMerge 语义
    status: pending
  - id: echarts-components
    content: echarts 组件：legend 绘制、dataZoom 组件（slider）、tooltip HTMLElement 路径
    status: pending
  - id: echarts-charts-expand
    content: echarts 图表：面积图、gauge、polar、多 series 混排、饼图 label/roseType
    status: pending
  - id: echarts-interaction-gaps
    content: echarts 交互：pinch dataZoom、showTip/hideTip、axisPointer 十字/多轴、highlight 按 dataIndex
    status: pending
  - id: echarts-text-richtext
    content: echarts/zrender 文本：series label 渲染、RichText/TSpan、自定义 fontFamily
    status: pending
  - id: echarts-performance
    content: 性能：脏矩形 partial refresh、离屏 Worker、常量回调 per-series 缓存
    status: pending
  - id: testing-visual
    content: 测试：echarts test/*.html option 视觉回归（golden PNG）+ JS vs WASM 基准报告
    status: pending
isProject: false
---

# wasm-echarts 待办与 Demo 规划

> **关联文档**：[wasm-echarts_移植规划_692fb54c.plan.md](wasm-echarts_移植规划_692fb54c.plan.md)（总规划，阶段 0–7 设计说明）  
> **代码现状**：阶段 0–6 主体已落地，阶段 7 完成 pie/scatter/轴标签/基础 Text/benchmark 初版；**Crate 已拆分为 rust-zrender + wasm-zrender + wasm-echarts**（见下节）。  
> **当前重点**：wasm-zrender / wasm-echarts 任务开发已基本完成 → **第一步推进文档官网建设**（`site/` 工程 + 文档 + 实例预览）。

---

## Crate 架构规范（已落地）

三层依赖关系：**底层纯 Rust lib** + **两个 wasm-bindgen WASM 产物**，彼此 sibling，不互相依赖。

```mermaid
flowchart TB
    subgraph native [纯 Rust · 可 cargo test]
        RZ[rust-zrender<br/>crate: rust-zrender<br/>lib: rust_zrender]
    end

    subgraph wasm [wasm-pack · cdylib]
        WZ[wasm-zrender<br/>ZRenderInstance 薄封装]
        WE[wasm-echarts<br/>EChartsInstance]
    end

    WZ --> RZ
    WE --> RZ
```

| Crate | 路径 | 类型 | 职责 |
|-------|------|------|------|
| **rust-zrender** | `crates/rust-zrender/` | 普通 `rlib` | Storage / Painter / Handler / 图元 / vl-convert 后端；**无 wasm-bindgen** |
| **wasm-zrender** | `crates/wasm-zrender/` | `cdylib` + `rlib` | 薄封装 `ZRenderInstance`：`load_scene` / `refresh` / `find_hover` / `resize`；供 zrender 级 Demo |
| **wasm-echarts** | `crates/wasm-echarts/` | `cdylib` + `rlib` | `EChartsInstance` + option 管线；内部直接使用 `rust_zrender::ZRenderer` |

**依赖规则**：

- `wasm-echarts` → `rust-zrender`（**不**依赖 `wasm-zrender`）
- `wasm-zrender` → `rust-zrender`
- 浏览器集成：`site/` 分别 `import` 各 crate 的 `pkg/wasm_*.js`

**编译命令**：

```bash
cd wasm-echarts-rs/crates/wasm-zrender && wasm-pack build --target web
cd wasm-echarts-rs/crates/wasm-echarts && wasm-pack build --target web
cd wasm-echarts-rs && cargo test -p rust-zrender
```

**Rust 引用约定**：代码中 `use rust_zrender::...`；npm/pkg 名仍为 `wasm-zrender` / `wasm-echarts`。

---

## 已完成摘要（不再重复开发）

| 阶段 | 已完成能力 |
|------|-----------|
| 0 | crate 结构、vl-convert 后端、`CanvasBackend` trait、`site/` 目录（由原 `demo/` 迁移） |
| 1 | Storage / Path / Painter / brush、基础 shape、单元测试 RGBA 输出 |
| 2（部分） | shadow pass、isPointInPath、径向渐变 r0、Linear/Radial/Pattern、lineDash、Image |
| 3 | Handler.findHover、ECData、emphasis/select 状态 |
| 4 | JS 薄壳、`OptionValue` 解析合并、`EChartsInstance` WASM API |
| 4b（部分） | `CallbackDataParams`、`formatter`/`color` 回调、`resolve_formatter` |
| 5（部分） | GlobalModel、cartesian、line/bar ChartView、Scheduler 单次全量 |
| 6（部分） | hover 高亮、toggleSelect、tooltip string、wheel dataZoom、axisPointer 竖线 |
| 7（部分） | pie/scatter、轴刻度 Text、feature flags、`benchmark_render` |
| 架构 | **rust-zrender / wasm-zrender / wasm-echarts** 三 crate 拆分 |

---

## 一、文档官网的建设（当前第一步）

wasm-zrender / wasm-echarts 核心能力已可运行，但缺少**对外可用的文档与实例入口**。本阶段在 `wasm-echarts-rs/site/` 建设文档官网（**目录由现有 `demo/` 迁移并重命名**），便于手工回归、对外展示与协作 onboarding。

### 1.1 站点总览

**根目录**：`wasm-echarts-rs/site/`（替代原 `demo/`）

**信息架构**：

```mermaid
flowchart TB
    Home[首页 index.html<br/>项目缘由 + 限制]
    Home --> WZ[wasm-zrender 入口]
    Home --> WE[wasm-echarts 入口]

    WZ --> WZDocs[文档]
    WZ --> WZEx[实例]

    WE --> WEDocs[文档]
    WE --> WEEx[实例]

    WZEx --> WZPage[实例页：左源码 · 右预览]
    WEEx --> WEPage[实例页：左源码 · 右预览]
```

**建议目录**：

```
wasm-echarts-rs/site/
├── index.html                    # 站点首页（仅缘由 + 限制 + 双产品入口）
├── shared/                       # 公共样式、布局、左码右预览 shell
│   ├── layout.css
│   └── example-shell.js          # 左编辑器 + 右 canvas 预览框架
├── zrender/
│   ├── index.html                # wasm-zrender 子站首页（文档 / 实例 导航）
│   ├── docs/
│   │   └── index.html            # 快速开始、API、编译、架构、已知限制
│   └── examples/
│       ├── index.html            # 实例列表
│       ├── shapes.html           # 各实例页：左源码 + 右预览
│       ├── text.html
│       ├── sector.html
│       ├── hit.html
│       └── state.html
└── echarts/
    ├── index.html                # wasm-echarts 子站首页（文档 / 实例 导航）
    ├── docs/
    │   └── index.html            # setOption、回调、feature flags、差异说明
    └── examples/
        ├── index.html            # 实例列表
        ├── line.html
        ├── bar.html
        ├── pie.html
        ├── scatter.html
        ├── interactive.html
        ├── function-option.html
        ├── merge.html
        └── bench.html            # 可选：WASM vs 官方 echarts 基准
```

**本地预览**（Vite）：

```bash
cd wasm-echarts-rs/site && npm install && npm run dev
# 打开 http://127.0.0.1:5173/
```

### 1.2 首页（`site/index.html`）

首页**只做项目说明与分流**，不承载文档正文、不嵌入预览 canvas。

| 区块 | 内容 |
|------|------|
| 项目缘由 | 为何用 Rust/WASM 重写 zrender/echarts 渲染层；三 crate 架构价值 |
| 已知限制 | 不支持项（SVG、动画 morph、DOM 组件等）；与官方 ECharts 差异摘要（链至各产品文档详情） |
| wasm-zrender 入口 | 卡片 → `site/zrender/`（再分 **文档** / **实例**） |
| wasm-echarts 入口 | 卡片 → `site/echarts/`（再分 **文档** / **实例**） |

**验收**：

- [ ] 首页无 API 长文、无实例预览，仅缘由 + 限制 + 两个产品入口
- [ ] 每个产品子站首页清晰区分「文档」「实例」两个版块
- [ ] README 编译说明指向 `site/` 路径

### 1.3 实例页布局（左源码 · 右预览）

所有 **实例** 页统一采用 **左右分栏** 布局（`shared/example-shell.js` 复用）：

```
┌─────────────────────────────────────────────────────────┐
│  实例标题 · 场景说明 · 返回列表                          │
├──────────────────────┬──────────────────────────────────┤
│  源码（左）           │  预览（右）                       │
│  · 可编辑 textarea    │  · canvas 渲染                   │
│    或只读 code 块     │  · 交互（hover / zoom 等）        │
│  · 运行 / 重置 按钮   │  · 命中信息 / 日志（按需）        │
└──────────────────────┴──────────────────────────────────┘
```

| 产品 | 左侧源码内容 | 右侧预览 |
|------|-------------|----------|
| wasm-zrender | 场景 JS（`load_scene` 参数、或后续 `add_path` 示例） | `ZRenderInstance` canvas 输出 |
| wasm-echarts | option JSON + 可选 formatter JS 片段 | `EChartsInstance` canvas + tooltip 等 |

**交互要求**：

- [ ] 修改左侧源码后点击「运行」刷新右侧预览（debounce 可选）
- [ ] 实例列表页链到各场景，URL 可直链（如 `examples/shapes.html`）
- [ ] 移动端可改为上下堆叠，但桌面默认左码右预览

### 1.4 wasm-zrender（文档 + 实例）

**文档**（`site/zrender/docs/`）：快速开始、`wasm-pack build`、`ZRenderInstance` API、与 rust-zrender 关系。

**实例**（`site/zrender/examples/`）：

| 实例 | 验证点 |
|------|--------|
| shapes | Rect/Circle/Line/Polygon/渐变/虚线/阴影 |
| text | fillText、对齐/基线、中文渲染 |
| sector | 饼图扇区 SectorShape |
| hit | find_hover + ECData 回显 |
| state | emphasis/select 切换 |

**依赖**：`crates/wasm-zrender/pkg`（`ZRenderInstance.load_scene`）。

**验收**：

- [ ] `wasm-pack build` 于 `crates/wasm-zrender`
- [ ] `http-server` 下 `site/zrender/examples/` 各页左码右预览正常
- [ ] 文档页含双 crate 编译说明

### 1.5 wasm-echarts（文档 + 实例）

**文档**（`site/echarts/docs/`）：`setOption`、函数回调注入、feature flags、与官方差异清单。

**实例**（`site/echarts/examples/`）：

| 实例 | 验证点 |
|------|--------|
| line | 折线 + formatter（补 label 后回归） |
| bar | 柱状 |
| pie | 饼图（补 label 后回归） |
| scatter | 散点 |
| interactive | hover / select / dataZoom / axisPointer / tooltip |
| function-option | color / formatter / tooltip 回调 |
| merge | notMerge / 二次 setOption |
| bench（可选） | WASM vs 官方 echarts 基准 |

**迁移说明**：原 `demo/` 已删除；echarts 合一页逻辑已迁入 `site/echarts/examples/`。文档站为 **Vite 多页工程**（`site/package.json`，`npm run dev` / `npm run build`）。

**验收**：

- [ ] 每个 chart type 至少一个左码右预览实例页
- [ ] 实例列表与文档目录结构一致
- [ ] （可选）与官方 echarts 同 option 截图对比说明

---

## 二、rust-zrender / wasm-zrender 待办

### 2.1 rust-zrender 后端与图元能力缺口

| 项 | 规划来源 | 现状 | 待办 |
|----|----------|------|------|
| conic gradient | 阶段 2 | 未实现 | 线性/径向近似或 tiny-skia 直绘 |
| CSS filter | 阶段 2 | 忽略 | 文档化不支持；按需评估 |
| Text 场景图 | 阶段 7 | 仅 `Storage.texts` 平铺列表，未进 Group/displayList | 纳入 `ChildRef::Text`、z 排序、transform |
| Text 命中检测 | 阶段 3 | 未做 | `measure_text` + bbox contain（轴标签可 silent） |
| 字体资源 | 风险缓解 | 使用 vl-convert 默认字体 | JS 预加载 font bytes → `fontdb` 共享 |
| strokeText | 阶段 7 | 未暴露 | 按需封装 `CanvasContext::stroke_text` |

### 2.2 wasm-zrender WASM 薄封装（阶段 3 规划）

**已完成**（`crates/wasm-zrender/`）：

- [x] 独立 `wasm-pack` crate（`cdylib`）
- [x] `ZRenderInstance`：`new` / `load_scene` / `refresh` / `find_hover` / `resize` / `highlight_path`
- [x] 内置场景：`shapes` | `text` | `sector` | `hit` | `state`
- [x] 与 `wasm-echarts` 解耦（均只依赖 `rust-zrender`）

**待办**（动态构图 API）：

```rust
// 目标扩展（示意）
impl ZRenderInstance {
    pub fn add_path(...) -> u32;
    pub fn add_text(...) -> u32;
    pub fn clear_scene(&mut self);
}
```

- [ ] 暴露 `add_path` / `add_text` / `add_group` 等增量 API（当前仅 `load_scene` 预设场景）
- [ ] `site/zrender/examples/` 实例页对接 `wasm-zrender/pkg`

### 2.3 性能（阶段 7）

- [ ] 脏矩形（`useDirtyRect`）：displayList 局部 brush + `DirtyRect` 局部回写
- [ ] 多 zlevel Layer 已初步支持，需与脏矩形联调
- [ ] 离屏 Worker 渲染（可选，依赖 site 实例页与 JS 薄壳设计）

---

## 三、wasm-echarts 待办

### 3.1 Option / 回调桥接（阶段 4b）

| 回调/能力 | 现状 | 待办 |
|-----------|------|------|
| `label.formatter` | 已解析，**未绘制 label** | visual 阶段调用 + Text/RichText 输出 |
| `axisLabel.formatter` | `resolve_axis_formatter` 存在，**轴渲染未调用** | 接入 `chart/axis.rs` |
| `symbol` / `symbolSize` | 未实现 | 折线/散点动态符号 |
| `series.renderItem` | `call_render_item` 存在，**无 CustomView** | 实现 `api` 对象子集 + graphicSpec → zrender |
| `tooltip.formatter` → HTMLElement | 仅 string | JS 薄壳检测 DOM 返回值并挂载 |
| formatter/color 缓存 | 无 | per-series 常量结果缓存 |
| `CallbackDataParams` 字段 | 最小子集 | 补 `percent`、`dimensionNames`、`encode` 等 |

### 3.2 核心模型与管线（阶段 5）

| 模块 | 现状 | 待办 |
|------|------|------|
| `OptionManager` / media query | 仅 deep merge | `media` 条件解析与切换 |
| `SeriesData` / `Source` | 简化 `DataPoint`  vec | 维度、encode、layout 管道 |
| `restoreData` / `dataProcessor` | 无 | 按 MVP 裁剪移植 |
| 面积图 `areaStyle` | 无 | line series 下 Polygon 填充 |
| 多 grid / 多 x/yAxis | 单轴 | 多组件 index 映射 |
| `lazyUpdate` / `replaceMerge` | 部分 meta 字段 | 完整 setOption 语义 |

### 3.3 组件（阶段 5 P1–P2、阶段 6）

| 组件 | 现状 | 待办 |
|------|------|------|
| legend | 未绘制 | LegendView → Text + 色块 hit |
| dataZoom slider | 仅 inside + wheel | 滑块 UI（WASM 或 JS 控件）+ extent 同步 |
| dataZoom pinch | 无 | touch 双指 → `apply_data_zoom` |
| tooltip | string + DOM 定位 | 富文本/HTML、confine、trigger: axis |
| axisPointer | 单竖线 | 十字线、多轴联动、label 背景 |
| dispatchAction | highlight/downplay/select/dataZoom | `showTip`/`hideTip`、`legendToggleSelect` 等 |

### 3.4 图表扩展（阶段 7）

| 类型 | 现状 | 待办 |
|------|------|------|
| pie | 基础扇区 | `radius`/`center` option、roseType、label 引导线 |
| scatter | 基础圆点 | 大号数据、symbol 回调 |
| gauge | 无 | 仪表盘 MVP |
| polar | 无 | 极坐标 + line/bar |
| 组合图 | 单 series 类型分派 | 同图 line+bar、双 y 轴 |

### 3.5 文本与样式（阶段 7）

- [ ] RichText / TSpan（参考 [zrender-master/src/graphic/Text.ts](e:\wasm-echarts\zrender-master\src\graphic\Text.ts)）
- [ ] series `label` / `labelLine` 布局
- [ ] `itemStyle` 渐变对象从 JS 回调解析

### 3.6 性能与体积

- [ ] 脏矩形与 echarts `render` 增量结合
- [ ] WASM 体积：按 chart feature 裁剪默认 feature 集
- [ ] 大数据量：跳过逐点 formatter（batch 或采样）

---

## 四、测试体系待办（阶段 7）

| 类型 | 现状 | 待办 |
|------|------|------|
| Rust 单元测试 | 24 项（几何/merge/zoom/pie） | 补 legend layout、renderItem 解析、coord 极坐标 |
| 浏览器集成测试 | `tests/web.rs` 空 | wasm-bindgen-test 注入 function option |
| 视觉回归 | 无 | 选 echarts `test/*.html` → option JSON → golden PNG diff |
| 性能基准 | `benchmark_render` + `?bench=1` | 固定用例集；输出 JS ECharts vs WASM 对比表 |
| vl-convert 差异 | 无文档 | 差异项清单 + 容忍阈值 |

---

## 五、推荐实施顺序

```mermaid
flowchart LR
    S0[site 首页 + 布局 shell] --> S1[zrender 文档/实例]
    S1 --> S2[echarts 文档/实例]
    S2 --> C1[legend + label]
    C1 --> C2[回调补全]
    C2 --> T1[视觉回归测试]
    P1[性能脏矩形] --> T1
```

1. **文档官网建设**（`site/` 框架 + 首页缘由/限制 + zrender/echarts 文档与左码右预览实例）— **当前最高优先级**  
2. **组件与文本**（legend、label、axisLabel.formatter）— 提升图表可读性  
3. **回调与 CustomSeries 子集**（renderItem）— 对齐官方 option 兼容目标  
4. **交互补全**（pinch、showTip、axis 模式 tooltip）  
5. **图表扩展**（gauge、polar、area）  
6. **性能 + 视觉回归 + 基准报告**

---

## 六、任务与规划阶段对照

| 本文件 todo id | 对应原规划 |
|----------------|-----------|
| （架构） | rust-zrender + wasm-zrender + wasm-echarts 三 crate |
| `docs-site` | **新增**：`site/` 首页（缘由/限制）+ 双产品入口 + 公共布局 |
| `demo-zrender` | `site/zrender/` 文档 + 左码右预览实例 |
| `demo-echarts` | `site/echarts/` 文档 + 左码右预览实例 + 基准 |
| `zrender-wasm-api` | 阶段 3 § WASM 导出（基础已完成，add_element 待补） |
| `zrender-backend-gaps` | 阶段 2 缺口 + 阶段 7 Text（rust-zrender） |
| `echarts-callback-full` | 阶段 4b 全文 |
| `echarts-model-pipeline` | 阶段 5 Global/SeriesData/Scheduler |
| `echarts-components` | 阶段 5 P1–P2 + 阶段 6 |
| `echarts-charts-expand` | 阶段 5 P3 + 阶段 7 |
| `echarts-interaction-gaps` | 阶段 6 未覆盖项 |
| `echarts-text-richtext` | 阶段 7 文本 |
| `echarts-performance` | 阶段 7 性能 + 风险缓解 |
| `testing-visual` | 阶段 7 测试体系 |

---

## 七、Out of Scope（明确不做）

- SVG 渲染模式
- 动画中间帧 / morph
- Loading、DataView 等 DOM 组件
- 完整 echarts 全量移植（严格 MVP + feature flag 扩展）
