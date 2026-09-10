/**
 * wasm-bindgen 内部模块（zrender 文档站 standalone）。
 * 公开 API 不要直接 export 这个 namespace。
 * echarts 页不要 import 本文件，否则会加载第二份 zrender wasm。
 */
import * as standalonePkg from '../pkg/wasm_zrender.js';
import { setNative, native, getNative } from './native-core.js';

setNative(standalonePkg);

export { default } from '../pkg/wasm_zrender.js';
export * from '../pkg/wasm_zrender.js';
export { native, setNative, getNative };
