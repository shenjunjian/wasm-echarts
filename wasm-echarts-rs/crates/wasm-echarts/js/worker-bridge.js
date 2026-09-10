/**
 * 主线程 Worker 桥：创建 module Worker、RPC、SAB/Transferable 回图。
 * 开发者仍调 init(canvas, null, { useWorker: true })，不必自己 new Worker。
 */
import {
  MSG,
  FRAME_SAB,
  FRAME_BUFFER,
  optionContainsFunction,
  WORKER_FUNCTION_OPTION_ERROR,
} from './worker-protocol.js';

/** @type {{ data: Uint8Array, opts: object }[]} */
const fontRecords = [];

/** @type {Map<string, ChartWorkerBridge>} */
const bridges = new Map();

/** @type {Worker | null} */
let sharedWorker = null;
let workerRefCount = 0;
/** @type {Promise<void> | null} */
let workerReady = null;
let workerReadyResolve = null;
let workerFailed = null;

export function rememberFont(data, opts) {
  const bytes = data instanceof Uint8Array ? new Uint8Array(data) : new Uint8Array(data);
  fontRecords.push({ data: bytes, opts: opts && typeof opts === 'object' ? { ...opts } : {} });
}

export function clearFontRecords() {
  fontRecords.length = 0;
}

export function getFontRecords() {
  return fontRecords;
}

function snapshotFonts() {
  return fontRecords.map((item) => ({
    data: item.data.slice(),
    opts: item.opts,
  }));
}

function snapshotMaps(maps) {
  const out = [];
  if (!maps) {
    return out;
  }
  for (const [name, record] of maps) {
    out.push({
      name,
      geoJSON: record && record.geoJSON,
      specialAreas: record && record.specialAreas,
    });
  }
  return out;
}

function onWorkerMessage(event) {
  const msg = event.data;
  if (!msg || typeof msg !== 'object') {
    return;
  }
  if (msg.type === MSG.READY) {
    if (workerReadyResolve) {
      workerReadyResolve();
      workerReadyResolve = null;
    }
    return;
  }
  if (msg.type === MSG.ERROR && (msg.chartId == null || msg.chartId === '')) {
    workerFailed = new Error(msg.message || '[wasm-echarts] Worker 初始化失败');
    if (workerReadyResolve) {
      workerReadyResolve();
      workerReadyResolve = null;
    }
    console.error(workerFailed);
    return;
  }
  const bridge = msg.chartId ? bridges.get(msg.chartId) : null;
  if (bridge) {
    bridge._onMessage(msg);
  }
}

function acquireWorker() {
  if (typeof Worker === 'undefined') {
    throw new Error('[wasm-echarts] 当前环境没有 Worker，无法 useWorker');
  }
  if (!sharedWorker) {
    workerFailed = null;
    workerReady = new Promise((resolve) => {
      workerReadyResolve = resolve;
    });
    sharedWorker = new Worker(new URL('./chart.worker.js', import.meta.url), {
      type: 'module',
    });
    sharedWorker.onmessage = onWorkerMessage;
    sharedWorker.onerror = (event) => {
      workerFailed = event.error || new Error(event.message || '[wasm-echarts] Worker 错误');
      console.error(workerFailed);
      if (workerReadyResolve) {
        workerReadyResolve();
        workerReadyResolve = null;
      }
    };
  }
  workerRefCount += 1;
  return sharedWorker;
}

function releaseWorker() {
  workerRefCount = Math.max(0, workerRefCount - 1);
  if (workerRefCount === 0 && sharedWorker) {
    sharedWorker.terminate();
    sharedWorker = null;
    workerReady = null;
    workerReadyResolve = null;
    workerFailed = null;
  }
}

export function workerAvailable() {
  return typeof Worker !== 'undefined';
}

/**
 * 把 registerFont / registerMap 推到已运行的 Worker（无实例时忽略）。
 */
export function notifyWorkerFonts(data, opts) {
  if (!sharedWorker) {
    return;
  }
  const bytes = data instanceof Uint8Array ? data : new Uint8Array(data);
  const copy = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
  sharedWorker.postMessage(
    { type: MSG.REGISTER_FONT, reqId: 0, chartId: '', data: copy, opts: opts || {} },
    [copy],
  );
}

export function notifyWorkerClearFonts() {
  if (!sharedWorker) {
    return;
  }
  sharedWorker.postMessage({ type: MSG.CLEAR_FONTS, reqId: 0, chartId: '' });
}

export function notifyWorkerMap(name, geoJSON, specialAreas) {
  if (!sharedWorker) {
    return;
  }
  sharedWorker.postMessage({
    type: MSG.REGISTER_MAP,
    reqId: 0,
    chartId: '',
    name,
    geoJSON,
    specialAreas,
  });
}

export class ChartWorkerBridge {
  /**
   * @param {{ chartId: string, width: number, height: number, dpr: number, maps?: Map<string, unknown> }} meta
   */
  constructor(meta) {
    this.chartId = meta.chartId;
    this._width = meta.width;
    this._height = meta.height;
    this._dpr = meta.dpr;
    this._hasOption = false;
    this._option = null;
    this._onFrame = null;
    this._sabBuffers = null;
    this._pending = new Map();
    this._reqId = 0;
    this._moveQueue = null;
    this._moveBusy = false;
    this._disposed = false;
    this._lastFramePixels = null;
    this._maps = meta.maps;
    this._worker = acquireWorker();
    bridges.set(this.chartId, this);
    this._ready = this._boot();
  }

  setFrameHandler(fn) {
    this._onFrame = fn;
  }

  width() {
    return this._width;
  }

  height() {
    return this._height;
  }

  dpr() {
    return this._dpr;
  }

  hasOption() {
    return this._hasOption;
  }

  getCachedOption() {
    return this._option;
  }

  async _boot() {
    await workerReady;
    if (workerFailed) {
      throw workerFailed;
    }
    return this._rpc(MSG.CREATE, {
      width: this._width,
      height: this._height,
      dpr: this._dpr,
      fonts: snapshotFonts(),
      maps: snapshotMaps(this._maps),
    });
  }

  _rpc(type, payload, transfer) {
    if (this._disposed && type !== MSG.DISPOSE) {
      return Promise.reject(new Error('Chart has been disposed'));
    }
    if (workerFailed && type !== MSG.DISPOSE) {
      return Promise.reject(workerFailed);
    }
    const reqId = ++this._reqId;
    return new Promise((resolve, reject) => {
      this._pending.set(reqId, { resolve, reject });
      this._worker.postMessage(
        { type, reqId, chartId: this.chartId, ...payload },
        transfer || [],
      );
    });
  }

  _call(type, payload, transfer) {
    return this._ready.then(() => this._rpc(type, payload, transfer));
  }

  _onMessage(msg) {
    if (msg.type === MSG.SAB) {
      this._sabBuffers = msg.buffers;
      return;
    }
    if (msg.type === MSG.FRAME) {
      this._applyFrame(msg);
      return;
    }
    if (msg.type === MSG.ERROR) {
      const err = new Error(msg.message || '[wasm-echarts] Worker 错误');
      const pending = this._pending.get(msg.reqId);
      if (pending) {
        this._pending.delete(msg.reqId);
        pending.reject(err);
      } else {
        console.error(err);
      }
      return;
    }
    if (msg.type === MSG.RESULT) {
      const pending = this._pending.get(msg.reqId);
      if (pending) {
        this._pending.delete(msg.reqId);
        pending.resolve(msg.value);
      }
    }
  }

  _applyFrame(msg) {
    this._width = msg.width || this._width;
    this._height = msg.height || this._height;
    if (!msg.dirty) {
      return;
    }
    let pixels = null;
    if (msg.mode === FRAME_SAB && this._sabBuffers && msg.index != null) {
      const sab = this._sabBuffers[msg.index];
      const need = msg.width * msg.height * 4;
      if (sab && sab.byteLength >= need) {
        pixels = new Uint8ClampedArray(sab, 0, need);
      }
    } else if (msg.mode === FRAME_BUFFER && msg.buffer) {
      pixels = new Uint8ClampedArray(msg.buffer);
    }
    if (!pixels) {
      return;
    }
    this._lastFramePixels = pixels;
    if (typeof this._onFrame === 'function') {
      this._onFrame({
        width: msg.width,
        height: msg.height,
        pixels,
      });
    }
  }

  async setOption(option, flags) {
    if (optionContainsFunction(option) || optionContainsFunction(flags)) {
      throw new Error(WORKER_FUNCTION_OPTION_ERROR);
    }
    const value = await this._call(MSG.SET_OPTION, { option, flags: flags || {} });
    this._option = option;
    this._hasOption = !!(value && value.hasOption);
    if (value && value.width) {
      this._width = value.width;
      this._height = value.height;
    }
    return value;
  }

  async resize(width, height, dpr) {
    const value = await this._call(MSG.RESIZE, { width, height, dpr });
    if (value) {
      this._width = value.width;
      this._height = value.height;
      this._dpr = value.dpr;
    }
    return value;
  }

  async refresh() {
    await this._call(MSG.REFRESH, {});
    return this._lastFramePixels;
  }

  async getOption() {
    const option = await this._call(MSG.GET_OPTION, {});
    if (option != null) {
      this._option = option;
      this._hasOption = true;
    }
    return option;
  }

  pointerMove(x, y) {
    this._moveQueue = { x, y };
    if (this._moveBusy) {
      return this._moveBusy;
    }
    const run = async () => {
      this._moveBusy = true;
      try {
        let last = null;
        while (this._moveQueue) {
          const next = this._moveQueue;
          this._moveQueue = null;
          last = await this._call(MSG.POINTER_MOVE, { x: next.x, y: next.y });
        }
        return last;
      } finally {
        this._moveBusy = false;
      }
    };
    this._moveBusy = run();
    return this._moveBusy;
  }

  pointerDown(x, y) {
    return this._call(MSG.POINTER_DOWN, { x, y });
  }

  pointerUp(x, y) {
    return this._call(MSG.POINTER_UP, { x, y });
  }

  pointerClick(x, y) {
    return this._call(MSG.POINTER_CLICK, { x, y });
  }

  pointerLeave() {
    this._moveQueue = null;
    return this._call(MSG.POINTER_LEAVE, {});
  }

  dispatchAction(payload) {
    if (optionContainsFunction(payload)) {
      throw new Error(WORKER_FUNCTION_OPTION_ERROR);
    }
    return this._call(MSG.DISPATCH, { payload });
  }

  async appendData(params) {
    const value = await this._call(MSG.APPEND_DATA, { params });
    this._hasOption = !!(value && value.hasOption);
    return value;
  }

  applyDataZoomWheel(x, deltaY) {
    return this._call(MSG.APPLY_WHEEL, { x, deltaY });
  }

  findHover(x, y) {
    return this._call(MSG.FIND_HOVER, { x, y });
  }

  getTooltipContent(seriesIndex, dataIndex) {
    return this._call(MSG.GET_TOOLTIP, { seriesIndex, dataIndex });
  }

  convertToPixel(finder, value) {
    return this._call(MSG.CONVERT_TO, { finder, value });
  }

  convertFromPixel(finder, value) {
    return this._call(MSG.CONVERT_FROM, { finder, value });
  }

  containPixel(finder, value) {
    return this._call(MSG.CONTAIN, { finder, value });
  }

  updateFontDatabase() {
    return this._call(MSG.UPDATE_FONT, {});
  }

  benchmarkRender(iterations) {
    return this._call(MSG.BENCHMARK, { iterations });
  }

  registerFont(data, opts) {
    const bytes = data instanceof Uint8Array ? data : new Uint8Array(data);
    const copy = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
    return this._call(MSG.REGISTER_FONT, { data: copy, opts: opts || {} }, [copy]);
  }

  clearFonts() {
    return this._call(MSG.CLEAR_FONTS, {});
  }

  async dispose() {
    if (this._disposed) {
      return;
    }
    try {
      await this._rpc(MSG.DISPOSE, {});
    } catch {
      // worker 可能已挂
    }
    this._disposed = true;
    for (const pending of this._pending.values()) {
      pending.reject(new Error('Chart has been disposed'));
    }
    this._pending.clear();
    bridges.delete(this.chartId);
    releaseWorker();
    this._onFrame = null;
    this._sabBuffers = null;
    this._lastFramePixels = null;
  }
}

export { optionContainsFunction, WORKER_FUNCTION_OPTION_ERROR };
