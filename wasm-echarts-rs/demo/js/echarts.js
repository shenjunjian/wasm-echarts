/**
 * wasm-echarts JS 薄壳（阶段 4）
 * 对齐 echarts canvas 模式 API：init / setOption / resize / on / dispatchAction / dispose
 */

import initWasm, { EChartsInstance } from '../../crates/wasm-echarts/pkg/wasm_echarts.js';

let wasmReady = null;

function ensureWasm() {
  if (!wasmReady) {
    wasmReady = initWasm();
  }
  return wasmReady;
}

/** 简单事件总线 */
class EventBus {
  constructor() {
    /** @type {Map<string, Set<Function>>} */
    this._handlers = new Map();
  }

  on(type, handler) {
    if (!this._handlers.has(type)) {
      this._handlers.set(type, new Set());
    }
    this._handlers.get(type).add(handler);
    return this;
  }

  off(type, handler) {
    const set = this._handlers.get(type);
    if (set) {
      set.delete(handler);
    }
    return this;
  }

  emit(type, payload) {
    const set = this._handlers.get(type);
    if (set) {
      for (const fn of set) {
        fn(payload);
      }
    }
  }
}

class EChartsWasmChart {
  /**
   * @param {EChartsInstance} wasm
   * @param {HTMLCanvasElement} canvas
   * @param {HTMLElement} dom
   */
  constructor(wasm, canvas, dom) {
    this._wasm = wasm;
    this._canvas = canvas;
    this._dom = dom;
    this._events = new EventBus();
    this._disposed = false;
    this._lastHover = null;
    /** @type {ResizeObserver|null} */
    this._resizeObserver = null;
    this._tooltipEl = null;

    this._createTooltip();
    this._bindEvents();
    this._observeResize();
    this._paint();
  }

  _createTooltip() {
    const tip = document.createElement('div');
    tip.style.cssText =
      'position:absolute;display:none;padding:6px 10px;background:rgba(50,50,50,0.9);color:#fff;font:12px/1.4 system-ui,sans-serif;border-radius:4px;pointer-events:none;white-space:nowrap;z-index:10;';
    this._dom.appendChild(tip);
    this._tooltipEl = tip;
  }

  _showTooltip(text, event) {
    if (!this._tooltipEl) return;
    this._tooltipEl.textContent = text;
    this._tooltipEl.style.display = 'block';
    const rect = this._dom.getBoundingClientRect();
    this._tooltipEl.style.left = `${event.clientX - rect.left + 12}px`;
    this._tooltipEl.style.top = `${event.clientY - rect.top + 12}px`;
  }

  _hideTooltip() {
    if (this._tooltipEl) {
      this._tooltipEl.style.display = 'none';
    }
  }

  _bindEvents() {
    const canvas = this._canvas;

    canvas.addEventListener('mousemove', (e) => {
      if (this._disposed) return;
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const hit = this._wasm.find_hover(x, y);

      if (hit && hit !== null) {
        const key = `${hit.seriesIndex ?? ''}:${hit.dataIndex ?? ''}:${hit.pathIndex ?? ''}`;
        const prevKey = this._lastHover
          ? `${this._lastHover.seriesIndex ?? ''}:${this._lastHover.dataIndex ?? ''}:${this._lastHover.pathIndex ?? ''}`
          : null;
        if (key !== prevKey) {
          this._lastHover = hit;
          if (hit.seriesIndex != null && hit.dataIndex != null) {
            const tip = this._wasm.get_tooltip_content(hit.seriesIndex, hit.dataIndex);
            if (tip) {
              this._showTooltip(tip, e);
            }
          }
          this._events.emit('mouseover', { event: e, hit });
        }
      } else if (this._lastHover) {
        this._lastHover = null;
        this._hideTooltip();
        this._events.emit('mouseout', { event: e });
      }
    });

    canvas.addEventListener('click', (e) => {
      if (this._disposed) return;
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const hit = this._wasm.find_hover(x, y);
      this._events.emit('click', { event: e, hit: hit && hit !== null ? hit : null });
    });
  }

  _observeResize() {
    if (typeof ResizeObserver === 'undefined') return;
    this._resizeObserver = new ResizeObserver(() => {
      this.resize();
    });
    this._resizeObserver.observe(this._dom);
  }

  _paint() {
    const rgba = this._wasm.refresh();
    const w = this._wasm.width();
    const h = this._wasm.height();
    const ctx = this._canvas.getContext('2d');
    const imageData = new ImageData(new Uint8ClampedArray(rgba), w, h);
    ctx.putImageData(imageData, 0, 0);
  }

  /**
   * @param {object} option
   * @param {object} [opts]
   */
  setOption(option, opts = {}) {
    const merged = { ...option, ...opts };
    this._wasm.set_option(merged);
    this._paint();
    return this;
  }

  resize(opts = {}) {
    const rect = this._dom.getBoundingClientRect();
    const w = Math.max(1, Math.floor(opts.width ?? rect.width));
    const h = Math.max(1, Math.floor(opts.height ?? rect.height));
    const dpr = opts.devicePixelRatio ?? window.devicePixelRatio ?? 1;

    this._canvas.width = Math.floor(w * dpr);
    this._canvas.height = Math.floor(h * dpr);
    this._canvas.style.width = `${w}px`;
    this._canvas.style.height = `${h}px`;

    this._wasm.resize(w, h, dpr);
    this._paint();
    return this;
  }

  /**
   * @param {string} type
   * @param {Function} handler
   */
  on(type, handler) {
    this._events.on(type, handler);
    return this;
  }

  off(type, handler) {
    this._events.off(type, handler);
    return this;
  }

  /**
   * @param {object} action
   */
  dispatchAction(action) {
    this._wasm.dispatch_action(action);
    this._paint();
    return this;
  }

  getOption() {
    return {
      hasOption: this._wasm.has_option(),
      hasFunctions: this._wasm.option_has_functions(),
    };
  }

  dispose() {
    if (this._disposed) return;
    this._disposed = true;
    this._resizeObserver?.disconnect();
    this._tooltipEl?.remove();
    this._wasm.dispose();
    this._canvas.remove();
    this._events = new EventBus();
  }
}

/**
 * 初始化图表（对齐 echarts.init(dom, { renderer: 'canvas' })）
 * @param {HTMLElement} dom
 * @param {{ renderer?: string, width?: number, height?: number, devicePixelRatio?: number }} [opts]
 */
export async function init(dom, opts = {}) {
  if (opts.renderer && opts.renderer !== 'canvas') {
    console.warn('wasm-echarts 仅支持 canvas renderer，已忽略:', opts.renderer);
  }

  await ensureWasm();

  dom.style.position = dom.style.position || 'relative';
  dom.innerHTML = '';

  const canvas = document.createElement('canvas');
  canvas.style.display = 'block';
  dom.appendChild(canvas);

  const rect = dom.getBoundingClientRect();
  const w = Math.max(1, Math.floor(opts.width ?? (rect.width || 400)));
  const h = Math.max(1, Math.floor(opts.height ?? (rect.height || 300)));
  const dpr = opts.devicePixelRatio ?? window.devicePixelRatio ?? 1;

  canvas.width = Math.floor(w * dpr);
  canvas.height = Math.floor(h * dpr);
  canvas.style.width = `${w}px`;
  canvas.style.height = `${h}px`;

  const wasm = new EChartsInstance(w, h, dpr);
  return new EChartsWasmChart(wasm, canvas, dom);
}

/** 默认导出命名空间式 API */
const echarts = { init };
export default echarts;
