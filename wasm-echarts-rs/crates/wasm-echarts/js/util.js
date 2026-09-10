/**
 * echarts.util：官方 export/api/util.ts 子集，从 wasm-zrender util 再导出。
 */
export {
  map,
  each,
  indexOf,
  inherits,
  reduce,
  filter,
  bind,
  curry,
  isArray,
  isString,
  isObject,
  isFunction,
  extend,
  defaults,
  clone,
  merge,
} from '../../wasm-zrender/js/tool/util.js';
