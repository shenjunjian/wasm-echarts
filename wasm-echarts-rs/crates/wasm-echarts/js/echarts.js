import { native } from './native.js';
import {
  ECharts,
  instances,
  getDomInstanceId,
  setDomInstanceId,
} from './instance.js';
import { themes, maps, connectedGroups } from './shared.js';
import * as graphic from './graphic.js';
import * as util from './util.js';
import * as number from './number.js';
import * as time from './time.js';
import * as format from './format.js';
import * as helper from './helper.js';
import * as matrix from '../../wasm-zrender/js/tool/matrix.js';
import * as vector from '../../wasm-zrender/js/tool/vector.js';
import * as color from '../../wasm-zrender/js/tool/color.js';
import env from './env.js';
import { throttle } from './throttle.js';
import { registerCustomSeries as registerCustomSeriesImpl } from './extension.js';

export { ECharts, graphic, util, number, time, format, helper, matrix, vector, color, env, throttle };
export const version = '6.1.0';

function refreshLiveFontDatabases() {
  for (const chart of instances.values()) {
    if (!chart.isDisposed()) {
      chart.updateFontDatabase();
    }
  }
}

/**
 * 注册字体 bytes 到本模块全局 fontdb（WASM 不读系统字体）。
 * 轴标签 / series label 渲染前必调。已创建的实例会热更新。
 * @param {Uint8Array} data
 * @param {{ familyName?: string, sansSerif?: string[] }} [opts]
 */
export function registerFont(data, opts) {
  native.registerFont(data, opts);
  refreshLiveFontDatabases();
}

/** 清空已注册字体（测试用）。 */
export function clearFonts() {
  native.clearFonts();
  refreshLiveFontDatabases();
}

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

export function connect(groupId) {
  if (Array.isArray(groupId)) {
    const charts = groupId;
    let id = null;
    for (const chart of charts) {
      if (chart && chart.group) {
        id = chart.group;
      }
    }
    id = id || `g_${idBase++}`;
    for (const chart of charts) {
      if (chart) {
        chart.group = id;
      }
    }
    groupId = id;
  }
  connectedGroups.set(String(groupId), true);
  return String(groupId);
}

export function disconnect(groupId) {
  connectedGroups.set(String(groupId), false);
}

export const disConnect = disconnect;

export function registerTheme(name, theme) {
  themes.set(String(name), theme);
}

export function registerMap(mapName, geoJson, specialAreas) {
  let record;
  if (
    geoJson &&
    typeof geoJson === 'object' &&
    (geoJson.geoJSON || geoJson.geoJson || geoJson.svg)
  ) {
    record = {
      geoJSON: geoJson.geoJSON || geoJson.geoJson || null,
      svg: geoJson.svg || null,
      specialAreas: geoJson.specialAreas || specialAreas || null,
    };
  } else {
    record = {
      geoJSON: geoJson || null,
      svg: null,
      specialAreas: specialAreas || null,
    };
  }
  maps.set(String(mapName), record);
  if (typeof native.registerMap === 'function') {
    native.registerMap(String(mapName), record.geoJSON, record.specialAreas);
  }
}

export function parseGeoJSON(geoJson, nameProperty) {
  if (typeof native.parseGeoJSON === 'function') {
    return native.parseGeoJSON(geoJson, nameProperty == null ? 'name' : nameProperty);
  }
  return [];
}

export const parseGeoJson = parseGeoJSON;

export function getMap(mapName) {
  return maps.get(String(mapName));
}

export const registerLocale = unimplemented('registerLocale');
export {
  PRIORITY,
  registerPreprocessor,
  registerProcessor,
  registerLayout,
  registerVisual,
  registerAction,
  registerCoordinateSystem,
  setPlatformAPI,
} from './extension.js';

export function registerCustomSeries(seriesType, renderItem) {
  registerCustomSeriesImpl(seriesType, renderItem);
  if (typeof native.registerCustomSeriesType === 'function') {
    native.registerCustomSeriesType(String(seriesType));
  }
}

const TRANSFORM_REGISTRY = new Map();

function wrapTransformFn(fn) {
  return function wrappedTransform(params) {
    const upstreamIn = params && params.upstream ? params.upstream : {};
    const data = upstreamIn.data || upstreamIn.source || [];
    const dimensions = upstreamIn.dimensions || [];
    const wrapped = {
      data,
      source: data,
      dimensions,
      count() {
        return data.length;
      },
      getRawDataItem(i) {
        return data[i];
      },
      retrieveValueFromItem(item, dimIdx) {
        if (Array.isArray(item)) {
          return item[dimIdx];
        }
        if (item && typeof item === 'object') {
          const name = dimensions[dimIdx];
          return name != null ? item[name] : undefined;
        }
        return item;
      },
      getDimensionInfo(dim) {
        if (typeof dim === 'number') {
          return { index: dim, name: dimensions[dim] };
        }
        const index = dimensions.indexOf(dim);
        return index >= 0 ? { index, name: dim } : null;
      },
      cloneRawData() {
        return data.slice();
      },
      cloneAllDimensionInfo() {
        return dimensions.slice();
      },
    };
    return fn({
      upstream: wrapped,
      config: params && params.config,
    });
  };
}

/**
 * @param {{ type: string, transform: Function }} transform
 */
export function registerTransform(transform) {
  const type = transform && transform.type;
  const fn = transform && transform.transform;
  if (!type || typeof fn !== 'function') {
    console.warn('[wasm-echarts] registerTransform 需要 { type, transform }');
    return;
  }
  TRANSFORM_REGISTRY.set(String(type), transform);
  if (typeof native.registerTransform === 'function') {
    native.registerTransform(String(type), wrapTransformFn(fn));
  }
}
