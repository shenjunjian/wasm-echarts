/**
 * echarts.env：按官方 zrender env 字段提供宿主探测（WASM 无 SVG）。
 */
const hasWindow = typeof window !== 'undefined';
const ua =
  typeof navigator !== 'undefined' && typeof navigator.userAgent === 'string'
    ? navigator.userAgent
    : '';

const env = {
  browser: {
    firefox: /Firefox/i.test(ua),
    ie: /MSIE|Trident/i.test(ua),
    edge: /Edg/i.test(ua),
    newEdge: /Edg/i.test(ua),
    weChat: /micromessenger/i.test(ua),
    version: '',
  },
  node: typeof process !== 'undefined' && !!(process.versions && process.versions.node),
  wxa: false,
  worker: typeof importScripts === 'function' && typeof document === 'undefined',
  svgSupported: false,
  touchEventsSupported: hasWindow && 'ontouchstart' in window,
  pointerEventsSupported: hasWindow && typeof window.PointerEvent === 'function',
  domSupported: typeof document !== 'undefined',
  transformSupported: true,
  transform3dSupported: true,
  hasGlobalWindow: hasWindow,
};

export default env;
