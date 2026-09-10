/**
 * 可注入的 wasm-bindgen 模块。standalone 由 native.js 注入 wasm-zrender/pkg；
 * echarts 页由 wasm-echarts/js/native.js 注入 wasm-echarts/pkg。不要在此静态 import pkg。
 */

let _native = null;

export function setNative(mod) {
  if (mod == null) {
    throw new Error('[wasm-zrender] setNative requires a wasm-bindgen module');
  }
  _native = mod;
}

export function getNative() {
  if (_native == null) {
    throw new Error(
      '[wasm-zrender] native 未注入。zrender 文档站请从 @wasm-zrender 入口导入；echarts 页请先加载 @wasm-echarts。',
    );
  }
  return _native;
}

export const native = new Proxy(
  {},
  {
    get(_target, prop) {
      if (prop === '__esModule') {
        return true;
      }
      return getNative()[prop];
    },
    has(_target, prop) {
      return prop in getNative();
    },
  },
);
