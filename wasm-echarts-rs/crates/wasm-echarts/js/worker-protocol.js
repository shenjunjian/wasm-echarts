/**
 * 主线程 facade 与 chart.worker.js 共用的消息名。
 * option 方向一律 postMessage 结构化克隆，不用 SharedArrayBuffer。
 * SAB 只用于 Worker → 主线程回传 RGBA。
 */

export const MSG = {
  READY: 'ready',
  CREATE: 'create',
  DISPOSE: 'dispose',
  SET_OPTION: 'setOption',
  RESIZE: 'resize',
  REFRESH: 'refresh',
  POINTER_MOVE: 'pointerMove',
  POINTER_DOWN: 'pointerDown',
  POINTER_UP: 'pointerUp',
  POINTER_CLICK: 'pointerClick',
  POINTER_LEAVE: 'pointerLeave',
  DISPATCH: 'dispatchAction',
  APPEND_DATA: 'appendData',
  APPLY_WHEEL: 'applyDataZoomWheel',
  FIND_HOVER: 'findHover',
  GET_TOOLTIP: 'getTooltipContent',
  CONVERT_TO: 'convertToPixel',
  CONVERT_FROM: 'convertFromPixel',
  CONTAIN: 'containPixel',
  REGISTER_FONT: 'registerFont',
  CLEAR_FONTS: 'clearFonts',
  REGISTER_MAP: 'registerMap',
  UPDATE_FONT: 'updateFontDatabase',
  BENCHMARK: 'benchmarkRender',
  GET_OPTION: 'getOption',
  HAS_OPTION: 'hasOption',
  SAB: 'sab',
  FRAME: 'frame',
  RESULT: 'result',
  ERROR: 'error',
};

export const FRAME_SAB = 'sab';
export const FRAME_BUFFER = 'buffer';

/**
 * option 里是否含函数 / symbol（结构化克隆会失败）。
 * TypedArray / ArrayBuffer 当叶子，不递归。
 */
export function optionContainsFunction(value, seen) {
  if (typeof value === 'function' || typeof value === 'symbol') {
    return true;
  }
  if (value == null || typeof value !== 'object') {
    return false;
  }
  if (typeof ArrayBuffer !== 'undefined') {
    if (value instanceof ArrayBuffer || ArrayBuffer.isView(value)) {
      return false;
    }
  }
  const seenSet = seen || new Set();
  if (seenSet.has(value)) {
    return false;
  }
  seenSet.add(value);
  if (Array.isArray(value)) {
    for (const item of value) {
      if (optionContainsFunction(item, seenSet)) {
        return true;
      }
    }
    return false;
  }
  for (const key of Object.keys(value)) {
    if (optionContainsFunction(value[key], seenSet)) {
      return true;
    }
  }
  return false;
}

export function canUseSharedArrayBuffer() {
  if (typeof SharedArrayBuffer !== 'function') {
    return false;
  }
  if (typeof crossOriginIsolated === 'boolean' && !crossOriginIsolated) {
    return false;
  }
  try {
    // SharedArrayBuffer 不可用时会抛 SecurityError
    new SharedArrayBuffer(8);
    return true;
  } catch {
    return false;
  }
}

export const WORKER_FUNCTION_OPTION_ERROR =
  '[wasm-echarts] useWorker 仅支持可结构化克隆的 option；formatter / renderItem 等函数请用默认主线程 init';
