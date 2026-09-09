/**
 * wasm-bindgen 内部模块。公开 API 不要直接 export 这个 namespace。
 */
export { default } from '../pkg/wasm_echarts.js';
export * from '../pkg/wasm_echarts.js';
import * as native from '../pkg/wasm_echarts.js';
export { native };
