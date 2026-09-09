/**
 * wasm-echarts 公开 API：对齐官方 core.ts / export/core.ts 命名。
 * default 仍是 wasm-bindgen 的 initWasm。
 */
export { default } from './native.js';

export {
  init,
  dispose,
  getInstanceByDom,
  getInstanceById,
  version,
  use,
  connect,
  disconnect,
  registerTheme,
  registerMap,
  getMap,
  registerLocale,
  setPlatformAPI,
  registerPreprocessor,
  ECharts,
} from './echarts.js';

export { EChartsInstance } from './native.js';
