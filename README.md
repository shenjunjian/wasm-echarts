# wasm-echarts

echarts 的 wasm 移植版本。 目的是快速将echarts的options选项绘制出来，不注重动画。

## 灵感

1. wasm 本质上是一个后台进程，如果将绘制过程使用 rust 语言在 wasm 离屏渲染，必然可以节省浏览器的主进程的时间
2. 先移植 zrender, 后移植 echarts
    Storage（场景图）→ Painter（遍历 displayList）→ ctx.fillRect / arc / fillText / drawImage ...要移植到 Rust/Wasm，Painter 层需要这些能力：
      使用 vl-convert-canvas2d 作为 Painter 后端。它不支持的能力，需要补齐，
        比如无 shadow、CSS filter   isPointInPath（命中检测需自实现）径向渐变内圆 r0 支持不完整, conic gradient

3. echarts 做为前端库，有一些参数是函数，需要设计如何传递。
4. 前端网页上的canvas的鼠标事件，滚轮缩放范围，resize事件等是需要及时传递给rust中，让它能实时响应这些行为，比如高亮线段，显示tooltip等等。这些需要根据echarts的实现来设计方案。
5. api对齐：
   . 只支持 canvas 模式即可，不需要支持 svg 模式。
   . 这个库重点是实现canvas绘制，如果遇到动画api， 直接让它执行完毕，不需要中间过程。如果遇到处理dom元素相关的，也可以忽略它。
   . 



## 结构

- wasm-echarts-rs\crates\wasm-echarts： wasm-pack创建的项目，编译为wasm。
- wasm-echarts-rs\crates\wasm-zrender： 一个普通的rust lib项目, 是wasm-echarts的依赖项。
- echarts-master:  echarts@6.1的源码，可以参考它的实现，但不允许修改它的内容。
- zrender-master： zrender@6.1的源码，可以参考它的实现，但不允许修改它的内容。