import { native } from './native.js';
import {
  ECharts,
  instances,
  getDomInstanceId,
  setDomInstanceId,
} from './instance.js';

export { ECharts };
export const version = '6.1.0';

let idBase = Date.now();

function unimplemented(name) {
  return function unimplementedApi() {
    console.warn(`[wasm-echarts] ${name} 未实现`);
  };
}

function toPositiveInt(value) {
  if (value == null || value === 'auto') {
    return undefined;
  }
  const n = Number(value);
  if (!Number.isFinite(n) || n <= 0) {
    return undefined;
  }
  return Math.round(n);
}

function toPositiveNumber(value) {
  if (value == null) {
    return undefined;
  }
  const n = Number(value);
  if (!Number.isFinite(n) || n <= 0) {
    return undefined;
  }
  return n;
}

function isCanvas(dom) {
  return !!(dom && typeof dom.getContext === 'function');
}

function resolveSize(dom, opts) {
  const o = opts && typeof opts === 'object' ? opts : {};
  let width = toPositiveInt(o.width);
  let height = toPositiveInt(o.height);
  const dpr = toPositiveNumber(o.devicePixelRatio ?? o.dpr) ?? 1;

  if (width == null || height == null) {
    if (isCanvas(dom)) {
      if (width == null) {
        width = toPositiveInt(dom.width) || toPositiveInt(dom.clientWidth) || 300;
      }
      if (height == null) {
        height = toPositiveInt(dom.height) || toPositiveInt(dom.clientHeight) || 150;
      }
    } else if (dom && typeof dom.clientWidth === 'number') {
      if (width == null) {
        width = toPositiveInt(dom.clientWidth) || 300;
      }
      if (height == null) {
        height = toPositiveInt(dom.clientHeight) || 150;
      }
    } else {
      if (width == null) {
        width = 300;
      }
      if (height == null) {
        height = 150;
      }
    }
  }

  return { width, height, dpr };
}

/**
 * WASM 整包模块：已开发的图表/组件都编进 WASM，不必按需加载。
 */
export function use() {
  console.info(
    '[wasm-echarts] 已开发的图表与组件都编进 WASM，无需 echarts.use(...)。',
  );
}

export function getInstanceById(id) {
  if (id == null) {
    return undefined;
  }
  return instances.get(String(id));
}

export function getInstanceByDom(dom) {
  const id = getDomInstanceId(dom);
  if (!id) {
    return undefined;
  }
  return instances.get(id);
}

/**
 * @param {HTMLCanvasElement | null} dom
 * @param {string | object | null} [theme]
 * @param {{ width?: number, height?: number, devicePixelRatio?: number }} [opts]
 */
export function init(dom, theme, opts) {
  if (dom) {
    const exist = getInstanceByDom(dom);
    if (exist && !exist.isDisposed()) {
      console.warn(
        '[wasm-echarts] There is a chart instance already initialized on the dom.',
      );
      return exist;
    }
  }

  const { width, height, dpr } = resolveSize(dom, opts);
  const handle = new native.EChartsInstance(width, height, dpr);
  const id = `ec_${idBase++}`;
  const chart = new ECharts(handle, { id, dom: dom || null, theme, opts });
  instances.set(id, chart);
  if (dom) {
    setDomInstanceId(dom, id);
  }
  return chart;
}

/**
 * @param {ECharts | HTMLElement | string} chart
 */
export function dispose(chart) {
  if (chart == null) {
    return;
  }
  let instance = chart;
  if (typeof chart === 'string') {
    instance = getInstanceById(chart);
  } else if (!(chart instanceof ECharts)) {
    instance = getInstanceByDom(chart);
  }
  if (instance && typeof instance.dispose === 'function' && !instance.isDisposed()) {
    instance.dispose();
  }
}

export const connect = unimplemented('connect');
export const disconnect = unimplemented('disconnect');
export const registerTheme = unimplemented('registerTheme');
export const registerMap = unimplemented('registerMap');
export const getMap = unimplemented('getMap');
export const registerLocale = unimplemented('registerLocale');
export const setPlatformAPI = unimplemented('setPlatformAPI');
export const registerPreprocessor = unimplemented('registerPreprocessor');
