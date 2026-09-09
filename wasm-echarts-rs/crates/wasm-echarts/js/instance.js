/** @type {Map<string, ECharts>} */
export const instances = new Map();

const DOM_ATTRIBUTE_KEY = '_echarts_instance_';

export function setDomInstanceId(dom, id) {
  if (!dom) {
    return;
  }
  try {
    dom[DOM_ATTRIBUTE_KEY] = id;
  } catch {
    // ignore non-extensible hosts
  }
}

export function getDomInstanceId(dom) {
  if (!dom) {
    return undefined;
  }
  try {
    return dom[DOM_ATTRIBUTE_KEY];
  } catch {
    return undefined;
  }
}

function isCanvas(dom) {
  return !!(dom && typeof dom.getContext === 'function');
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

function resolveDim(value, fallback) {
  if (value == null || value === 'auto') {
    return fallback;
  }
  const n = Number(value);
  if (!Number.isFinite(n) || n <= 0) {
    return fallback;
  }
  return Math.round(n);
}

function normalizeReplaceMerge(value) {
  if (value == null) {
    return [];
  }
  if (typeof value === 'string') {
    return [value];
  }
  if (Array.isArray(value)) {
    return value.filter((key) => typeof key === 'string');
  }
  return [];
}

/**
 * 官方 `setOption(option, notMerge?, lazyUpdate?)` 或 `setOption(option, opts)`。
 * `lazyUpdate` / `silent` 同步执行（立刻 flush）。
 */
function parseSetOptionFlags(notMergeOrOpts) {
  if (notMergeOrOpts != null && typeof notMergeOrOpts === 'object') {
    return {
      notMerge: !!notMergeOrOpts.notMerge,
      replaceMerge: normalizeReplaceMerge(notMergeOrOpts.replaceMerge),
    };
  }
  return {
    notMerge: !!notMergeOrOpts,
    replaceMerge: [],
  };
}

/**
 * 公开图表实例：camelCase 包装 native `EChartsInstance`。
 * `init(canvas)` 后 `setOption` / `resize` 等会自动 putImageData。
 */
export class ECharts {
  /**
   * @param {import('../pkg/wasm_echarts.js').EChartsInstance} handle
   * @param {{ id: string, dom?: HTMLElement | null, theme?: unknown, opts?: object }} meta
   */
  constructor(handle, meta) {
    this._native = handle;
    this.id = meta.id;
    this.group = '';
    this._dom = meta.dom || null;
    this._theme = meta.theme;
    this._opts = meta.opts || {};
    this._disposed = false;
    /** @type {Map<string, Function[]>} */
    this._listeners = new Map();
  }

  _assertAlive() {
    if (this.isDisposed()) {
      throw new Error('Chart has been disposed');
    }
  }

  _paintIfBound() {
    const canvas = this._dom;
    const handle = this._native;
    if (!canvas || !handle || !isCanvas(canvas)) {
      return;
    }
    const rgba = handle.refresh();
    const width = handle.width();
    const height = handle.height();
    if (canvas.width !== width) {
      canvas.width = width;
    }
    if (canvas.height !== height) {
      canvas.height = height;
    }
    const ctx = canvas.getContext('2d');
    if (!ctx || !rgba) {
      return;
    }
    ctx.putImageData(
      new ImageData(new Uint8ClampedArray(rgba), width, height),
      0,
      0,
    );
  }

  getDom() {
    return this._dom;
  }

  getId() {
    return this.id;
  }

  getWidth() {
    this._assertAlive();
    return this._native.width();
  }

  getHeight() {
    this._assertAlive();
    return this._native.height();
  }

  getDevicePixelRatio() {
    this._assertAlive();
    return this._native.dpr();
  }

  isDisposed() {
    return this._disposed || !this._native;
  }

  /**
   * `setOption(option)` / `setOption(option, notMerge, lazyUpdate?)` /
   * `setOption(option, { notMerge, replaceMerge, silent, lazyUpdate })`。
   * `notMerge` 不是 option 里的字段。`lazyUpdate` 同步 flush。
   */
  setOption(option, notMerge, lazyUpdate) {
    this._assertAlive();
    const flags = parseSetOptionFlags(notMerge);
    this._native.set_option(option, {
      notMerge: flags.notMerge,
      replaceMerge: flags.replaceMerge,
    });
    this._paintIfBound();
  }

  getOption() {
    this._assertAlive();
    return this._native.get_option();
  }

  /**
   * `resize()` 无参读 canvas；`resize({ width, height, devicePixelRatio })`；
   * 也接受 native 三位置参数 `resize(w, h, dpr)`。`width`/`height` 为 `'auto'` 时回退到 canvas。
   */
  resize(opts, heightArg, dprArg) {
    this._assertAlive();
    const currentW = this.getWidth();
    const currentH = this.getHeight();
    const currentDpr = this.getDevicePixelRatio();
    const fromDom = () => {
      if (!this._dom) {
        return { width: currentW, height: currentH };
      }
      return {
        width: this._dom.clientWidth || this._dom.width || currentW,
        height: this._dom.clientHeight || this._dom.height || currentH,
      };
    };

    let width;
    let height;
    let dpr = currentDpr;
    if (opts == null) {
      const size = fromDom();
      width = size.width;
      height = size.height;
    } else if (typeof opts === 'object') {
      const size = fromDom();
      width = resolveDim(opts.width, size.width);
      height = resolveDim(opts.height, size.height);
      dpr = toPositiveNumber(opts.devicePixelRatio ?? opts.dpr) ?? currentDpr;
    } else {
      width = resolveDim(opts, currentW);
      height = resolveDim(heightArg, currentH);
      dpr = toPositiveNumber(dprArg) ?? currentDpr;
    }
    this._native.resize(width, height, dpr);
    this._paintIfBound();
  }

  clear() {
    this.setOption({ series: [] }, true);
  }

  dispatchAction(payload) {
    this._assertAlive();
    this._native.dispatch_action(payload);
    this._paintIfBound();
  }

  on(event, handler) {
    this._assertAlive();
    if (typeof handler !== 'function') {
      return this;
    }
    const list = this._listeners.get(event) || [];
    list.push(handler);
    this._listeners.set(event, list);
    return this;
  }

  off(event, handler) {
    if (!event) {
      this._listeners.clear();
      return this;
    }
    if (!handler) {
      this._listeners.delete(event);
      return this;
    }
    const list = this._listeners.get(event);
    if (list) {
      this._listeners.set(
        event,
        list.filter((fn) => fn !== handler),
      );
    }
    return this;
  }

  dispose() {
    if (this.isDisposed()) {
      return;
    }
    const id = this.id;
    const dom = this._dom;
    if (this._native && typeof this._native.dispose === 'function') {
      this._native.dispose();
    }
    if (typeof this._native?.free === 'function') {
      this._native.free();
    }
    this._native = null;
    this._disposed = true;
    this._listeners.clear();
    instances.delete(id);
    if (dom && getDomInstanceId(dom) === id) {
      setDomInstanceId(dom, '');
    }
    this._dom = null;
  }

  refresh() {
    this._assertAlive();
    return this._native.refresh();
  }

  findHover(x, y) {
    this._assertAlive();
    return this._native.find_hover(x, y);
  }

  handlePointerMove(x, y) {
    this._assertAlive();
    const result = this._native.handle_pointer_move(x, y);
    this._paintIfBound();
    return result;
  }

  handlePointerLeave() {
    this._assertAlive();
    this._native.handle_pointer_leave();
    this._paintIfBound();
  }

  applyDataZoomWheel(x, deltaY) {
    this._assertAlive();
    this._native.apply_data_zoom_wheel(x, deltaY);
    this._paintIfBound();
  }

  getTooltipContent(seriesIndex, dataIndex) {
    this._assertAlive();
    return this._native.get_tooltip_content(seriesIndex, dataIndex);
  }

  benchmarkRender(iterations) {
    this._assertAlive();
    const avg = this._native.benchmark_render(iterations);
    this._paintIfBound();
    return avg;
  }

  hasOption() {
    this._assertAlive();
    return this._native.has_option();
  }

  optionHasFunctions() {
    this._assertAlive();
    return this._native.option_has_functions();
  }
}
