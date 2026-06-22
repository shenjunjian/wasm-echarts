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
│   ├── demo/                     # 浏览器 Demo（ES Module）
│   │   ├── index.html
│   │   ├── main.js
│   │   └── js/echarts.js         # 对齐 echarts 的 JS 薄壳
│   └── crates/
│       ├── wasm-echarts/         # wasm-pack 项目，编译为 wasm
│       │   ├── src/
│       │   └── pkg/              # wasm-pack 产物（见下文）
│       └── wasm-zrender/         # 普通 Rust lib，wasm-echarts 的依赖
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

在仓库根目录进入 `wasm-echarts-rs/crates/wasm-echarts` 后执行 wasm-pack。

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
cargo test -p wasm-echarts
cargo test -p wasm-zrender
```

## 编译产物

`wasm-pack build` 成功后，主要产物位于：

```
wasm-echarts-rs/crates/wasm-echarts/pkg/
├── wasm_echarts_bg.wasm    # WebAssembly 二进制
├── wasm_echarts.js         # wasm-bindgen 生成的 JS 胶水层（ES Module）
├── wasm_echarts.d.ts       # TypeScript 类型声明
├── wasm_echarts_bg.wasm.d.ts
└── package.json            # 可作为 npm 包引用
```

中间编译缓存位于 `wasm-echarts-rs/target/`（已在 `.gitignore` 中忽略，无需提交）。

### 对外暴露的 WASM API（`wasm_echarts.d.ts`）

| 类 / 方法 | 说明 |
|-----------|------|
| `EChartsInstance` | 核心实例 |
| `set_option(option)` | 传入 echarts option（支持 JS 函数字段） |
| `refresh()` | 离屏渲染，返回 RGBA `Uint8Array` |
| `resize(w, h, dpr)` | 调整画布逻辑尺寸与 DPR |
| `find_hover(x, y)` | 命中检测，返回 seriesIndex / dataIndex 等 |
| `get_tooltip_content(seriesIndex, dataIndex)` | 调用 option 中的 tooltip formatter |
| `dispatch_action(action)` | 触发 highlight / downplay 等 |
| `dispose()` | 释放 option |

## 使用方法

### 1. 跑 Demo（最快验证）

Demo 通过 ES Module 直接引用 `pkg/` 下的 wasm 文件，**必须先完成 wasm-pack 编译**。

```bash
# 1. 编译 wasm（若尚未编译）
cd wasm-echarts-rs/crates/wasm-echarts
wasm-pack build --target web --dev

# 2. 在 workspace 根目录启动静态服务
cd ../..
npx http-server -p 8080
```

浏览器打开：

- 折线图（默认）：http://127.0.0.1:8080/demo/
- 柱状图：http://127.0.0.1:8080/demo/?type=bar

页面底部 `#status` 应显示类似「阶段 5 Demo（line）：含函数 true，hover 显示 tooltip」，鼠标悬停数据点可看到 tooltip。

### 2. 在页面中接入（推荐 JS 薄壳）

复制或引用 `demo/js/echarts.js`，它封装了 canvas 创建、像素绘制、resize、事件与 tooltip：

```html
<div id="chart" style="width:480px;height:360px"></div>
<script type="module">
  import echarts from './js/echarts.js';

  const chart = await echarts.init(document.getElementById('chart'), {
    renderer: 'canvas', // 目前仅支持 canvas
  });

  chart.setOption({
    xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed'] },
    yAxis: { type: 'value' },
    series: [{ type: 'line', name: '销量', data: [120, 200, 150] }],
  });

  chart.on('click', ({ hit }) => {
    console.log(hit);
  });
</script>
```

薄壳 API 与 echarts canvas 模式对齐：

| 方法 | 说明 |
|------|------|
| `echarts.init(dom, opts?)` | 初始化，返回 Promise\<Chart\> |
| `chart.setOption(option, opts?)` | 设置 option 并立即重绘 |
| `chart.resize(opts?)` | 按容器或指定宽高重绘（内置 ResizeObserver） |
| `chart.on(type, handler)` | 监听 `click` / `mouseover` / `mouseout` |
| `chart.dispatchAction(action)` | 转发至 WASM |
| `chart.getOption()` | 返回 `{ hasOption, hasFunctions }` |
| `chart.dispose()` | 销毁实例 |

### 3. 直接使用 WASM 包（无薄壳）

适合自定义渲染管线（例如自行管理 canvas / WebGL）：

```javascript
import initWasm, { EChartsInstance } from './pkg/wasm_echarts.js';

await initWasm();

const w = 480, h = 360, dpr = window.devicePixelRatio || 1;
const instance = new EChartsInstance(w, h, dpr);

instance.set_option({
  xAxis: { type: 'category', data: ['A', 'B', 'C'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', data: [10, 20, 30] }],
});

const rgba = instance.refresh(); // Uint8Array，长度 w*h*dpr*dpr*4
const ctx = canvas.getContext('2d');
ctx.putImageData(new ImageData(new Uint8ClampedArray(rgba), w * dpr, h * dpr), 0, 0);
```

修改 Rust 源码后需重新执行 `wasm-pack build`，浏览器侧硬刷新（Ctrl+Shift+R）即可加载新 wasm。

## 当前能力（阶段 5 MVP）

- 图表类型：折线图、柱状图
- option 中的 JS 函数：`tooltip.formatter`、`label.formatter`、`itemStyle.color` 等
- 交互：hover 命中检测、tooltip、click 事件、`highlight` / `downplay` action
- 仅 canvas 渲染，无动画中间帧

尚未完整实现 echarts 全量 API，更多能力持续移植中。
