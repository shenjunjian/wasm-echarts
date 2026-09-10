/**
 * 官方扩展注册真表：preprocessor / processor / layout / visual /
 * action / coordinateSystem / customSeries。
 * StageHandler 拿到的是简化 ecModel（getOption / eachSeries），不是官方 GlobalModel。
 */

/** 与官方 `echarts.PRIORITY` 同名常量，供 registerProcessor / registerLayout / registerVisual 使用。 */
export const PRIORITY = {
  PROCESSOR: {
    SERIES_FILTER: 800,
    AXIS_STATISTICS: 920,
    FILTER: 1000,
    STATISTIC: 5000,
    STATISTICS: 5000,
  },
  VISUAL: {
    LAYOUT: 1000,
    PROGRESSIVE_LAYOUT: 1100,
    GLOBAL: 2000,
    CHART: 3000,
    POST_CHART_LAYOUT: 4600,
    COMPONENT: 4000,
    BRUSH: 5000,
    CHART_ITEM: 4500,
    ARIA: 6000,
    DECAL: 7000,
  },
};

const PRIORITY_PROCESSOR_DEFAULT = 2000;
const PRIORITY_VISUAL_LAYOUT = 1000;
const PRIORITY_VISUAL_CHART = 3000;

const preprocessors = [];
const processors = [];
const layouts = [];
const visuals = [];
const actions = new Map();
const coordinateSystems = new Map();
const customSeries = new Map();

let platformAPI = {};

const BUILTIN_ACTIONS = new Set([
  'highlight',
  'downplay',
  'select',
  'unselect',
  'toggleSelect',
  'dataZoom',
  'showTip',
  'hideTip',
  'legendToggleSelect',
  'legendSelect',
  'legendUnSelect',
  'restore',
  'timelineChange',
  'timelinePlayChange',
  'takeGlobalCursor',
  'brush',
  'brushEnd',
  'expandAxisBreak',
  'collapseAxisBreak',
  'toggleAxisBreak',
]);

function indexOfSame(list, raw) {
  return list.findIndex((item) => item.raw === raw);
}

function normalizeStage(priorityOrFn, fn, defaultPriority) {
  let priority = defaultPriority;
  let task = fn;
  if (typeof priorityOrFn === 'function' || (priorityOrFn && typeof priorityOrFn === 'object')) {
    task = priorityOrFn;
    priority = defaultPriority;
  } else if (typeof priorityOrFn === 'number') {
    priority = priorityOrFn;
  }
  return { priority, task, raw: task };
}

function pushStage(list, entry) {
  if (!entry.task) {
    return;
  }
  if (indexOfSame(list, entry.raw) >= 0) {
    return;
  }
  list.push(entry);
  list.sort((a, b) => a.priority - b.priority);
}

export function registerPreprocessor(fn) {
  if (typeof fn !== 'function') {
    return;
  }
  if (!preprocessors.includes(fn)) {
    preprocessors.push(fn);
  }
}

export function registerProcessor(priority, processor) {
  pushStage(processors, normalizeStage(priority, processor, PRIORITY_PROCESSOR_DEFAULT));
}

export function registerLayout(priority, layout) {
  pushStage(layouts, normalizeStage(priority, layout, PRIORITY_VISUAL_LAYOUT));
}

export function registerVisual(priority, visual) {
  pushStage(visuals, normalizeStage(priority, visual, PRIORITY_VISUAL_CHART));
}

/**
 * registerAction('type', 'event', fn)
 * registerAction('type', fn)
 * registerAction({ type, event, update, action }, fn?)
 */
export function registerAction(arg0, arg1, arg2) {
  let type;
  let eventType;
  let update;
  let action;
  if (arg0 && typeof arg0 === 'object') {
    type = arg0.type;
    eventType = arg0.event;
    update = arg0.update;
    action = arg2 || arg1 || arg0.action;
  } else {
    type = arg0;
    if (typeof arg1 === 'function') {
      action = arg1;
    } else {
      eventType = arg1;
      action = arg2;
    }
  }
  if (!type || typeof action !== 'function') {
    console.warn('[wasm-echarts] registerAction 需要 type 与 handler');
    return;
  }
  const key = String(type);
  if (actions.has(key)) {
    return;
  }
  actions.set(key, {
    type: key,
    event: String(eventType || key).toLowerCase(),
    update,
    action,
  });
}

export function registerCoordinateSystem(type, creator) {
  if (!type || !creator) {
    console.warn('[wasm-echarts] registerCoordinateSystem 需要 type 与 creator');
    return;
  }
  coordinateSystems.set(String(type), creator);
}

export function registerCustomSeries(seriesType, renderItem) {
  if (!seriesType || typeof renderItem !== 'function') {
    console.warn('[wasm-echarts] registerCustomSeries 需要 (seriesType, renderItem)');
    return;
  }
  customSeries.set(String(seriesType), renderItem);
}

export function setPlatformAPI(api) {
  if (!api || typeof api !== 'object') {
    return;
  }
  platformAPI = { ...platformAPI, ...api };
}

export function getPlatformAPI() {
  return platformAPI;
}

export function getRegisteredCustomSeries(type) {
  return customSeries.get(String(type));
}

export function hasRegisteredCustomSeries(type) {
  return customSeries.has(String(type));
}

function eachOptionUnit(option, visit) {
  if (!option || typeof option !== 'object') {
    return;
  }
  visit(option);
  if (option.baseOption && typeof option.baseOption === 'object') {
    visit(option.baseOption);
  }
  const options = option.options;
  if (Array.isArray(options)) {
    for (const item of options) {
      if (item && typeof item === 'object') {
        visit(item);
      }
    }
  }
  const media = option.media;
  if (Array.isArray(media)) {
    for (const unit of media) {
      if (unit && unit.option && typeof unit.option === 'object') {
        visit(unit.option);
      }
    }
  }
}

export function runPreprocessors(option, isNew) {
  eachOptionUnit(option, (unit) => {
    for (const fn of preprocessors) {
      try {
        fn(unit, isNew !== false);
      } catch (err) {
        console.error('[wasm-echarts] registerPreprocessor', err);
      }
    }
  });
}

function seriesListOf(option) {
  const series = option && option.series;
  if (Array.isArray(series)) {
    return series;
  }
  if (series && typeof series === 'object') {
    return [series];
  }
  return [];
}

export function applyCustomSeriesRegistry(option) {
  eachOptionUnit(option, (unit) => {
    for (const series of seriesListOf(unit)) {
      if (!series || typeof series !== 'object') {
        continue;
      }
      const ty = series.type;
      const renderItem = ty != null ? customSeries.get(String(ty)) : null;
      if (!renderItem) {
        continue;
      }
      if (typeof series.renderItem !== 'function') {
        series.renderItem = renderItem;
      }
    }
  });
}

function makeSeriesProxy(series, index) {
  const layouts = [];
  return {
    id: series && series.id,
    name: series && series.name,
    type: series && series.type,
    option: series,
    seriesIndex: index,
    get(path) {
      if (!series || path == null) {
        return undefined;
      }
      const keys = String(path).split('.');
      let cur = series;
      for (const key of keys) {
        if (cur == null) {
          return undefined;
        }
        cur = cur[key];
      }
      return cur;
    },
    getData() {
      const data = (series && series.data) || [];
      return {
        count() {
          return data.length;
        },
        getRawDataItem(i) {
          return data[i];
        },
        each(fn) {
          for (let i = 0; i < data.length; i++) {
            fn(data[i], i);
          }
        },
        getItemLayout(i) {
          return layouts[i];
        },
        setItemLayout(i, layout) {
          layouts[i] = layout;
        },
      };
    },
  };
}

export function makeStageContext(chart, option) {
  const seriesProxies = seriesListOf(option).map((s, i) => makeSeriesProxy(s, i));
  const ecModel = {
    option,
    getOption() {
      return option;
    },
    getWidth() {
      return chart.getWidth();
    },
    getHeight() {
      return chart.getHeight();
    },
    eachSeries(cb) {
      seriesProxies.forEach((s, i) => cb(s, i));
    },
    eachSeriesByType(type, cb) {
      seriesProxies.forEach((s, i) => {
        if (s.type === type) {
          cb(s, i);
        }
      });
    },
    getSeries() {
      return seriesProxies;
    },
    getSeriesByType(type) {
      return seriesProxies.filter((s) => s.type === type);
    },
    getComponent(mainType, index) {
      const value = option && option[mainType];
      const list = Array.isArray(value) ? value : value != null ? [value] : [];
      return list[index || 0];
    },
  };
  const api = {
    getWidth() {
      return chart.getWidth();
    },
    getHeight() {
      return chart.getHeight();
    },
    getDevicePixelRatio() {
      return chart.getDevicePixelRatio();
    },
    getOption() {
      return option;
    },
    dispatchAction(payload) {
      chart.dispatchAction(payload);
    },
  };
  return { ecModel, api };
}

function invokeStages(list, ecModel, api) {
  for (const item of list) {
    const task = item.task;
    try {
      if (typeof task === 'function') {
        task(ecModel, api);
      } else if (task && typeof task === 'object') {
        if (typeof task.overallReset === 'function') {
          task.overallReset(ecModel, api);
        }
        if (typeof task.reset === 'function') {
          ecModel.eachSeries((series) => {
            if (!task.seriesType || series.type === task.seriesType) {
              task.reset(series, ecModel, api);
            }
          });
        }
      }
    } catch (err) {
      console.error('[wasm-echarts] extension stage', err);
    }
  }
}

export function runProcessors(chart, option) {
  const { ecModel, api } = makeStageContext(chart, option);
  invokeStages(processors, ecModel, api);
}

export function runLayoutAndVisual(chart, option) {
  const { ecModel, api } = makeStageContext(chart, option);
  invokeStages(layouts, ecModel, api);
  invokeStages(visuals, ecModel, api);
}

export function prepareIncomingOption(option, isNew) {
  if (!option || typeof option !== 'object') {
    return option;
  }
  runPreprocessors(option, isNew);
  applyCustomSeriesRegistry(option);
  return option;
}

export function dispatchRegisteredAction(chart, payload) {
  const type = payload && payload.type;
  if (type == null) {
    return { handled: false, runNative: true };
  }
  const entry = actions.get(String(type));
  if (!entry) {
    return { handled: false, runNative: true };
  }
  try {
    const option = typeof chart.getOption === 'function' ? chart.getOption() : {};
    const { ecModel, api } = makeStageContext(chart, option);
    entry.action(payload, ecModel, api);
  } catch (err) {
    console.error('[wasm-echarts] registerAction', err);
  }
  return {
    handled: true,
    event: entry.event,
    runNative: BUILTIN_ACTIONS.has(String(type)),
  };
}

export function createRegisteredCoordSystems(chart) {
  const option = typeof chart.getOption === 'function' ? chart.getOption() : {};
  const { ecModel, api } = makeStageContext(chart, option);
  const list = [];
  for (const [type, creator] of coordinateSystems) {
    try {
      const created = typeof creator.create === 'function'
        ? creator.create(ecModel, api)
        : typeof creator === 'function'
          ? creator(ecModel, api)
          : null;
      const arr = Array.isArray(created) ? created : created ? [created] : [];
      for (const cs of arr) {
        if (!cs) {
          continue;
        }
        if (!cs.type) {
          cs.type = type;
        }
        list.push(cs);
      }
    } catch (err) {
      console.error('[wasm-echarts] registerCoordinateSystem', err);
    }
  }
  return list;
}

function finderCoordType(finder) {
  if (typeof finder === 'string') {
    return finder;
  }
  if (!finder || typeof finder !== 'object') {
    return null;
  }
  for (const key of Object.keys(finder)) {
    if (key.endsWith('Index') || key.endsWith('Id') || key.endsWith('Name')) {
      const main = key.replace(/Index$|Id$|Name$/, '');
      if (coordinateSystems.has(main)) {
        return main;
      }
    }
    if (coordinateSystems.has(key)) {
      return key;
    }
  }
  return null;
}

function pickCoord(list, type, finder) {
  const matches = list.filter((cs) => !type || cs.type === type);
  if (!matches.length) {
    return null;
  }
  if (finder && typeof finder === 'object') {
    const idx = finder[`${type}Index`];
    if (idx != null && matches[idx]) {
      return matches[idx];
    }
  }
  return matches[0];
}

export function convertByRegisteredCoord(chart, finder, value, toPixel) {
  const list = chart._coordSysList;
  if (!list || !list.length) {
    return undefined;
  }
  const type = finderCoordType(finder);
  const cs = pickCoord(list, type, finder);
  if (!cs) {
    return undefined;
  }
  try {
    if (toPixel && typeof cs.dataToPoint === 'function') {
      return cs.dataToPoint(value);
    }
    if (!toPixel && typeof cs.pointToData === 'function') {
      return cs.pointToData(value);
    }
  } catch (err) {
    console.error('[wasm-echarts] custom coordinateSystem convert', err);
  }
  return undefined;
}

export function containByRegisteredCoord(chart, finder, value) {
  const list = chart._coordSysList;
  if (!list || !list.length) {
    return undefined;
  }
  const type = finderCoordType(finder);
  const cs = pickCoord(list, type, finder);
  if (!cs || typeof cs.containPoint !== 'function') {
    return undefined;
  }
  try {
    const point = Array.isArray(value) ? value : [value, 0];
    return !!cs.containPoint(point);
  } catch (err) {
    console.error('[wasm-echarts] custom coordinateSystem contain', err);
    return undefined;
  }
}
