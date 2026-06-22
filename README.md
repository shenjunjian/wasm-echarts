# wasm-echarts

echarts 的 wasm 移植版本

## 灵感

1. wasm 本质上是一个后台进程，如果将绘制过程使用 rust 语言在 wasm 离屏渲染，必然可以节省浏览器的主进程的时间
2. 先移植 zrender, 后移植 echarts
3. echarts 做为前端库，有一些参数是函数
