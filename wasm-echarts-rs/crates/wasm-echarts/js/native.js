/**
 * wasm-bindgen 内部模块。公开 API 不要直接 export 这个 namespace。
 * 加载后立刻注入 wasm-zrender native-core，echarts 页只持有这一份 WASM。
 */
export { default } from '../pkg/wasm_echarts.js';
export * from '../pkg/wasm_echarts.js';
import * as native from '../pkg/wasm_echarts.js';
import { setNative as setZrNative } from '../../wasm-zrender/js/native-core.js';

setZrNative(native);

export { native };
