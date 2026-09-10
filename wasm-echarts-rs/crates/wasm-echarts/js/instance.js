import { wrapZRender } from '../../wasm-zrender/js/zrender.js';
import { clone, merge } from './util.js';
import { connectedGroups, themes } from './shared.js';
import {
  prepareIncomingOption,
  runProcessors,
  runLayoutAndVisual,
  dispatchRegisteredAction,
  createRegisteredCoordSystems,
  convertByRegisteredCoord,
  containByRegisteredCoord,
} from './extension.js';

/** @type {Map<string, ECharts>} */
export const instances = new Map();

const DOM_ATTRIBUTE_KEY = '_echarts_instance_';

const TOOLTIP_CSS =
  'position:fixed;display:none;padding:6px 10px;background:rgba(50,50,50,0.9);color:#fff;font:12px/1.4 system-ui,sans-serif;border-radius:4px;pointer-events:none;white-space:nowrap;z-index:10;';

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

function eventPoint(canvas, event) {
  const rect = canvas.getBoundingClientRect();
  return {
    x: event.clientX - rect.left,
    y: event.clientY - rect.top,
  };
}

function hoverKey(hit) {
  if (hit && hit.dataType) {
    return `c:${hit.dataType}:${hit.dataIndex ?? 0}`;
  }
  if (!hit || hit.seriesIndex == null || hit.dataIndex == null) {
    return null;
  }
  return `${hit.seriesIndex}:${hit.dataIndex}`;
}

function matchQuery(query, params) {
  if (query == null) {
    return true;
  }
  if (typeof query === 'string') {
    if (query === 'series') {
      return params.seriesIndex != null;
    }
    return params.componentType === query || params.seriesType === query;
  }
  if (typeof query === 'object') {
    if (query.seriesIndex != null && query.seriesIndex !== params.seriesIndex) {
      return false;
    }
    if (query.dataIndex != null && query.dataIndex !== params.dataIndex) {
      return false;
    }
    if (
      query.componentType != null &&
      query.componentType !== params.componentType
    ) {
      return false;
    }
    return true;
  }
  return true;
}

function resolveTheme(theme) {
  if (theme == null || theme === '') {
    return null;
  }
  if (typeof theme === 'string') {
    return themes.get(theme) || null;
  }
  if (typeof theme === 'object') {
    return theme;
  }
  return null;
}

function optionHasDataZoom(option) {
  if (!option || option.dataZoom == null) {
    return false;
  }
  const items = Array.isArray(option.dataZoom)
    ? option.dataZoom
    : [option.dataZoom];
  return items.some((item) => item && item.type === 'inside');
}

function tooltipAllowed(option) {
  if (!option) {
    return false;
  }
  const tip = option.tooltip;
  if (tip == null || tip === false) {
    return false;
  }
  if (typeof tip === 'object' && tip.show === false) {
    return false;
  }
  return true;
}

/**
 * 公开图表实例：camelCase 包装 native `EChartsInstance`。
 * `init(canvas)` 后自动 putImageData，并绑定指针事件。
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
    /** @type {Map<string, { handler: Function, query?: unknown }[]>} */
    this._listeners = new Map();
    this._hoverKey = null;
    this._hoverHit = null;
    this._lastClientX = 0;
    this._lastClientY = 0;
    this._tooltipEl = null;
    this._tooltipOn = false;
    this._wheelZoomOn = false;
    this._zr = null;
    this._loadingEl = null;
    /** @type {object[]} */
    this._coordSysList = [];
    this._onMove = this._onPointerMove.bind(this);
    this._onClick = this._onPointerClick.bind(this);
    this._onLeave = this._onPointerLeave.bind(this);
    this._onWheel = this._onPointerWheel.bind(this);
    this._onDown = this._onPointerDown.bind(this);
    this._onUp = this._onPointerUp.bind(this);
    this._pointerDown = null;
    this._didDrag = false;
    this._bindHost();
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

  _bindHost() {
    const canvas = this._dom;
    if (!canvas || !isCanvas(canvas)) {
      return;
    }
    canvas.addEventListener('mousemove', this._onMove);
    canvas.addEventListener('mousedown', this._onDown);
    canvas.addEventListener('mouseup', this._onUp);
    canvas.addEventListener('click', this._onClick);
    canvas.addEventListener('mouseleave', this._onLeave);
    canvas.addEventListener('wheel', this._onWheel, { passive: false });
    if (this._native && typeof this._native.attachHost === 'function') {
      this._native.attachHost(canvas);
    }
  }

  _unbindHost() {
    const canvas = this._dom;
    if (!canvas || !isCanvas(canvas)) {
      return;
    }
    canvas.removeEventListener('mousemove', this._onMove);
    canvas.removeEventListener('mousedown', this._onDown);
    canvas.removeEventListener('mouseup', this._onUp);
    canvas.removeEventListener('click', this._onClick);
    canvas.removeEventListener('mouseleave', this._onLeave);
    canvas.removeEventListener('wheel', this._onWheel);
    if (canvas.style) {
      canvas.style.cursor = '';
    }
  }

  _packEvent(type, event, hit) {
    const params = {
      type,
      event,
    };
    if (hit && hit.dataType) {
      params.componentType = hit.dataType;
      params.dataIndex = hit.dataIndex;
    } else if (hit && hit.seriesIndex != null && hit.dataIndex != null) {
      params.componentType = 'series';
      params.seriesIndex = hit.seriesIndex;
      params.dataIndex = hit.dataIndex;
      if (hit.dataType != null) {
        params.dataType = hit.dataType;
      }
    }
    return params;
  }

  _emit(event, params) {
    const list = this._listeners.get(event);
    if (!list || list.length === 0) {
      return;
    }
    for (const item of list.slice()) {
      if (!matchQuery(item.query, params)) {
        continue;
      }
      try {
        item.handler.call(this, params);
      } catch (err) {
        console.error(err);
      }
    }
  }

  _ensureTooltip() {
    if (this._tooltipEl || typeof document === 'undefined') {
      return this._tooltipEl;
    }
    const el = document.createElement('div');
    el.style.cssText = TOOLTIP_CSS;
    document.body.appendChild(el);
    this._tooltipEl = el;
    return el;
  }

  _showTooltip(html, clientX, clientY) {
    if (html == null || html === '') {
      this._hideTooltip();
      return;
    }
    const el = this._ensureTooltip();
    if (!el) {
      return;
    }
    el.innerHTML = String(html).replace(/\n/g, '<br/>');
    el.style.display = 'block';
    const x = clientX == null ? this._lastClientX : clientX;
    const y = clientY == null ? this._lastClientY : clientY;
    el.style.left = `${x + 12}px`;
    el.style.top = `${y + 12}px`;
  }

  _hideTooltip() {
    if (this._tooltipEl) {
      this._tooltipEl.style.display = 'none';
    }
  }

  _disposeTooltip() {
    if (this._tooltipEl && this._tooltipEl.parentNode) {
      this._tooltipEl.parentNode.removeChild(this._tooltipEl);
    }
    this._tooltipEl = null;
  }

  _syncCursor(hit) {
    const canvas = this._dom;
    if (!canvas || !canvas.style) {
      return;
    }
    canvas.style.cursor = hoverKey(hit) ? 'pointer' : 'default';
  }

  _readOptionSafe() {
    try {
      return this.getOption();
    } catch {
      return null;
    }
  }

  _syncOptionFlags() {
    const option = this._readOptionSafe();
    this._tooltipOn = tooltipAllowed(option);
    this._wheelZoomOn = optionHasDataZoom(option);
  }

  _onPointerMove(event) {
    if (this.isDisposed()) {
      return;
    }
    const canvas = this._dom;
    const { x, y } = eventPoint(canvas, event);
    this._lastClientX = event.clientX;
    this._lastClientY = event.clientY;
    const result = this.handlePointerMove(x, y);
    const hit = result && result.hit;
    const key = hoverKey(hit);
    if (key !== this._hoverKey) {
      if (this._hoverKey != null) {
        this._emit(
          'mouseout',
          this._packEvent('mouseout', event, this._hoverHit),
        );
      }
      if (key != null) {
        this._emit('mouseover', this._packEvent('mouseover', event, hit));
      }
      this._hoverKey = key;
      this._hoverHit = hit || null;
    }
    this._syncCursor(hit);
    if (this._pointerDown && this._native && typeof this._native.handlePointerMove === 'function') {
      const dx = x - this._pointerDown.x;
      const dy = y - this._pointerDown.y;
      if (dx * dx + dy * dy > 16) {
        this._didDrag = true;
      }
    }
    if (this._tooltipOn && result && result.tooltip) {
      this._showTooltip(result.tooltip, event.clientX, event.clientY);
    } else {
      this._hideTooltip();
    }
  }

  _onPointerDown(event) {
    if (this.isDisposed()) {
      return;
    }
    const { x, y } = eventPoint(this._dom, event);
    this._pointerDown = { x, y };
    this._didDrag = false;
    if (this._native && typeof this._native.handlePointerDown === 'function') {
      this._native.handlePointerDown(x, y);
      this._paintIfBound();
    }
  }

  _onPointerUp(event) {
    if (this.isDisposed()) {
      return;
    }
    const { x, y } = eventPoint(this._dom, event);
    if (this._native && typeof this._native.handlePointerUp === 'function') {
      this._native.handlePointerUp(x, y);
      this._paintIfBound();
    }
  }

  _onPointerClick(event) {
    if (this.isDisposed()) {
      return;
    }
    const { x, y } = eventPoint(this._dom, event);
    if (this._didDrag) {
      this._pointerDown = null;
      this._didDrag = false;
      return;
    }
    let hit = this.findHover(x, y);
    if (this._native && typeof this._native.handlePointerClick === 'function') {
      hit = this._native.handlePointerClick(x, y) || hit;
      this._paintIfBound();
    }
    this._pointerDown = null;
    if (hoverKey(hit) == null) {
      return;
    }
    this._emit('click', this._packEvent('click', event, hit));
  }

  _onPointerLeave(event) {
    if (this.isDisposed()) {
      return;
    }
    if (this._hoverKey != null) {
      this._emit(
        'mouseout',
        this._packEvent('mouseout', event, this._hoverHit),
      );
    }
    this.handlePointerLeave();
    this._hoverKey = null;
    this._hoverHit = null;
    this._hideTooltip();
    this._syncCursor(null);
    this._emit('globalout', this._packEvent('globalout', event, null));
  }

  _onPointerWheel(event) {
    if (this.isDisposed()) {
      return;
    }
    if (!this._wheelZoomOn) {
      return;
    }
    event.preventDefault();
    const { x } = eventPoint(this._dom, event);
    this.applyDataZoomWheel(x, event.deltaY);
  }

  _showTipFromPayload(payload) {
    if (!this._tooltipOn) {
      return;
    }
    let seriesIndex = payload && payload.seriesIndex;
    let dataIndex = payload && payload.dataIndex;
    if (
      (seriesIndex == null || dataIndex == null) &&
      payload &&
      payload.x != null &&
      payload.y != null
    ) {
      const hit = this.findHover(payload.x, payload.y);
      seriesIndex = hit && hit.seriesIndex;
      dataIndex = hit && hit.dataIndex;
    }
    if (seriesIndex == null || dataIndex == null) {
      return;
    }
    const html = this.getTooltipContent(seriesIndex, dataIndex);
    let clientX = this._lastClientX;
    let clientY = this._lastClientY;
    if (payload.x != null && payload.y != null && this._dom) {
      const rect = this._dom.getBoundingClientRect();
      clientX = rect.left + payload.x;
      clientY = rect.top + payload.y;
    } else if (!clientX && !clientY && this._dom) {
      const rect = this._dom.getBoundingClientRect();
      clientX = rect.left + rect.width / 2;
      clientY = rect.top + rect.height / 2;
    }
    this._showTooltip(html, clientX, clientY);
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
    let payload = option;
    const theme = resolveTheme(this._theme);
    if (theme && (flags.notMerge || !this.hasOption())) {
      payload = merge(clone(theme), option);
    }
    payload = prepareIncomingOption(payload, flags.notMerge || !this.hasOption());
    runProcessors(this, payload);
    this._native.set_option(payload, {
      notMerge: flags.notMerge,
      replaceMerge: flags.replaceMerge,
    });
    this._afterNativeOption();
    if (!this._tooltipOn) {
      this._hideTooltip();
    }
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
    this._refreshRegisteredCoords();
    this._paintIfBound();
  }

  clear() {
    this.setOption({ series: [] }, true);
  }

  dispatchAction(payload) {
    this._assertAlive();
    const registered = dispatchRegisteredAction(this, payload);
    if (registered.runNative) {
      this._native.dispatch_action(payload);
    }
    this._paintIfBound();
    const type = payload && payload.type;
    if (type === 'showTip') {
      this._showTipFromPayload(payload);
      this._emit('showTip', payload);
    } else if (type === 'hideTip') {
      this._hideTooltip();
      this._emit('hideTip', payload);
    } else if (registered.handled && registered.event) {
      this._emit(registered.event, payload);
    }
    this._forwardConnect(payload);
  }

  _forwardConnect(payload) {
    if (!payload || payload.__fromConnect) {
      return;
    }
    const group = this.group;
    if (!group || !connectedGroups.get(group)) {
      return;
    }
    for (const chart of instances.values()) {
      if (chart === this || chart.group !== group || chart.isDisposed()) {
        continue;
      }
      try {
        chart.dispatchAction({ ...payload, __fromConnect: true });
      } catch (err) {
        console.error(err);
      }
    }
  }

  /**
   * `on(event, handler)` 或 `on(event, query, handler)`。
   * 指针事件：`click` / `mouseover` / `mouseout` / `globalout`。
   */
  on(event, query, handler) {
    this._assertAlive();
    let q = query;
    let fn = handler;
    if (typeof query === 'function') {
      fn = query;
      q = undefined;
    }
    if (typeof fn !== 'function') {
      return this;
    }
    const list = this._listeners.get(event) || [];
    list.push({ handler: fn, query: q });
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
        list.filter((item) => item.handler !== handler),
      );
    }
    return this;
  }

  trigger(event, params) {
    this._emit(event, params || { type: event });
    return this;
  }

  dispose() {
    if (this.isDisposed()) {
      return;
    }
    const id = this.id;
    this._unbindHost();
    this._disposeTooltip();
    this.hideLoading();
    this._zr = null;
    if (this._native && typeof this._native.dispose === 'function') {
      this._native.dispose();
    }
    if (typeof this._native?.free === 'function') {
      this._native.free();
    }
    this._native = null;
    this._disposed = true;
    this._listeners.clear();
    this._hoverKey = null;
    this._hoverHit = null;
    instances.delete(id);
    const dom = this._dom;
    if (dom && getDomInstanceId(dom) === id) {
      setDomInstanceId(dom, '');
    }
    this._dom = null;
  }

  refresh() {
    this._assertAlive();
    return this._native.refresh();
  }

  /**
   * 非官方：把全局 fontdb 同步到本实例。`registerFont` 已自动调用，一般不必手调。
   */
  updateFontDatabase() {
    this._assertAlive();
    this._native.update_font_database();
  }

  findHover(x, y) {
    this._assertAlive();
    return this._native.find_hover(x, y);
  }

  /**
   * 非官方 hatch：离屏或自管指针时用。`init(canvas)` 时一般不需要。
   */
  handlePointerMove(x, y) {
    this._assertAlive();
    const result = this._native.handle_pointer_move(x, y);
    this._paintIfBound();
    return result;
  }

  handlePointerDown(x, y) {
    this._assertAlive();
    const hit = this._native.handlePointerDown(x, y);
    this._paintIfBound();
    return hit;
  }

  handlePointerUp(x, y) {
    this._assertAlive();
    this._native.handlePointerUp(x, y);
    this._paintIfBound();
  }

  handlePointerClick(x, y) {
    this._assertAlive();
    const hit = this._native.handlePointerClick(x, y);
    this._paintIfBound();
    return hit;
  }

  /**
   * 非官方 hatch：离屏或自管指针时用。`init(canvas)` 时一般不需要。
   */
  handlePointerLeave() {
    this._assertAlive();
    this._native.handle_pointer_leave();
    this._paintIfBound();
  }

  /**
   * 非官方 hatch：`init(canvas)` 时滚轮已绑定，一般不需要。
   */
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

  /**
   * cartesian 最小集：finder 为 `{ xAxisIndex }` / `{ yAxisIndex }` / `{ gridIndex }` / `{ seriesIndex }`
   * 或 `'xAxis'` / `'yAxis'` / `'grid'`。完整 finder 未实现。
   */
  convertToPixel(finder, value) {
    this._assertAlive();
    const custom = convertByRegisteredCoord(this, finder, value, true);
    if (custom != null) {
      return custom;
    }
    return this._native.convert_to_pixel(finder, value);
  }

  convertFromPixel(finder, value) {
    this._assertAlive();
    const custom = convertByRegisteredCoord(this, finder, value, false);
    if (custom != null) {
      return custom;
    }
    return this._native.convert_from_pixel(finder, value);
  }

  containPixel(finder, value) {
    this._assertAlive();
    const custom = containByRegisteredCoord(this, finder, value);
    if (custom != null) {
      return custom;
    }
    return !!this._native.containPixel(finder, value);
  }

  getZr() {
    this._assertAlive();
    if (!this._zr) {
      this._zr = wrapZRender(this._native.getZr());
    }
    return this._zr;
  }

  showLoading(name, cfg) {
    this._assertAlive();
    if (name != null && typeof name === 'object') {
      cfg = name;
    }
    this.hideLoading();
    const host = this._dom;
    if (!host || typeof document === 'undefined') {
      return;
    }
    const text = (cfg && (cfg.text || cfg.msg)) || 'loading';
    const parent = host.parentElement;
    const el = document.createElement('div');
    el.setAttribute('data-ec-loading', '1');
    el.style.cssText =
      'position:absolute;left:0;top:0;right:0;bottom:0;background:rgba(255,255,255,0.72);display:flex;align-items:center;justify-content:center;color:#666;font:13px/1.4 sans-serif;pointer-events:none;z-index:9;';
    el.textContent = String(text);
    if (parent) {
      const pos = parent.style && parent.style.position;
      if (!pos || pos === 'static') {
        parent.style.position = 'relative';
      }
      parent.appendChild(el);
    } else if (host.insertAdjacentElement) {
      host.insertAdjacentElement('afterend', el);
    }
    this._loadingEl = el;
  }

  hideLoading() {
    if (this._loadingEl && this._loadingEl.parentNode) {
      this._loadingEl.parentNode.removeChild(this._loadingEl);
    }
    this._loadingEl = null;
  }

  renderToCanvas(opts) {
    this._assertAlive();
    const rgba = this.refresh();
    const width = this.getWidth();
    const height = this.getHeight();
    let canvas = opts && opts.canvas;
    if (!canvas || typeof canvas.getContext !== 'function') {
      if (typeof document === 'undefined') {
        throw new Error('renderToCanvas requires a canvas or document');
      }
      canvas = document.createElement('canvas');
    }
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext('2d');
    if (ctx && rgba) {
      ctx.putImageData(
        new ImageData(new Uint8ClampedArray(rgba), width, height),
        0,
        0,
      );
    }
    return canvas;
  }

  getDataURL(opts) {
    this._assertAlive();
    const o = opts && typeof opts === 'object' ? opts : {};
    if (o.type === 'svg') {
      console.warn('[wasm-echarts] getDataURL(type: "svg") 未实现（无 SVG painter）');
      return '';
    }
    const canvas = this.renderToCanvas(o);
    const type = o.type === 'jpeg' ? 'image/jpeg' : 'image/png';
    return canvas.toDataURL(type, o.quality);
  }

  appendData(params) {
    this._assertAlive();
    this._native.appendData(params);
    this._syncOptionFlags();
    this._paintIfBound();
  }

  setTheme(theme) {
    this._assertAlive();
    this._theme = theme;
    if (!this.hasOption()) {
      return;
    }
    const current = this.getOption();
    const themeObj = resolveTheme(theme);
    const payload = themeObj ? merge(clone(themeObj), current) : current;
    this._native.set_option(payload, { notMerge: true, replaceMerge: [] });
    this._afterNativeOption();
    this._paintIfBound();
  }

  _afterNativeOption() {
    this._syncOptionFlags();
    this._refreshRegisteredCoords();
    const current = this.getOption();
    runLayoutAndVisual(this, current);
  }

  _refreshRegisteredCoords() {
    this._coordSysList = createRegisteredCoordSystems(this);
  }
}
