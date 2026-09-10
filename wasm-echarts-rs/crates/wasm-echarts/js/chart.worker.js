/**
 * wasm-echarts Worker：持有 EChartsInstance，主线程只 blit / 事件 / tooltip。
 * option 只走 postMessage；RGBA 优先写入 SAB，否则 Transferable ArrayBuffer。
 */
import initWasm, { native } from './native.js';
import {
  MSG,
  FRAME_SAB,
  FRAME_BUFFER,
  canUseSharedArrayBuffer,
} from './worker-protocol.js';

/** @type {Map<string, import('../pkg/wasm_echarts.js').EChartsInstance>} */
const charts = new Map();

/** @type {Map<string, { buffers: SharedArrayBuffer[], write: number }>} */
const sabState = new Map();

let sabEnabled = false;
let wasmReady = null;

function ensureWasm() {
  if (!wasmReady) {
    wasmReady = initWasm().then(() => {
      sabEnabled = canUseSharedArrayBuffer();
    });
  }
  return wasmReady;
}

function reply(reqId, chartId, value) {
  postMessage({ type: MSG.RESULT, reqId, chartId, value });
}

function fail(reqId, chartId, err) {
  const message = err && (err.stack || err.message) ? String(err.stack || err.message) : String(err);
  postMessage({ type: MSG.ERROR, reqId, chartId, message });
}

function getChart(chartId) {
  const handle = charts.get(chartId);
  if (!handle) {
    throw new Error(`[wasm-echarts worker] 未知实例 ${chartId}`);
  }
  return handle;
}

function pixelCount(handle) {
  return handle.width() * handle.height() * 4;
}

function ensureSab(chartId, handle) {
  if (!sabEnabled) {
    return null;
  }
  const need = pixelCount(handle);
  let state = sabState.get(chartId);
  if (!state || state.buffers[0].byteLength < need) {
    const buffers = [new SharedArrayBuffer(need), new SharedArrayBuffer(need)];
    state = { buffers, write: 0 };
    sabState.set(chartId, state);
    postMessage({
      type: MSG.SAB,
      chartId,
      buffers,
      byteLength: need,
    });
  }
  return state;
}

function copyRgba(handle) {
  const rgba = handle.refresh();
  if (!rgba) {
    return null;
  }
  return rgba instanceof Uint8Array ? rgba : new Uint8Array(rgba);
}

function sendFrame(chartId, handle, dirty) {
  const width = handle.width();
  const height = handle.height();
  if (!dirty) {
    postMessage({
      type: MSG.FRAME,
      chartId,
      dirty: false,
      width,
      height,
    });
    return;
  }
  const rgba = copyRgba(handle);
  if (!rgba) {
    postMessage({
      type: MSG.FRAME,
      chartId,
      dirty: false,
      width,
      height,
    });
    return;
  }
  const need = width * height * 4;
  const state = ensureSab(chartId, handle);
  if (state) {
    const index = state.write;
    const view = new Uint8Array(state.buffers[index], 0, need);
    view.set(rgba.subarray(0, need));
    state.write = index ^ 1;
    postMessage({
      type: MSG.FRAME,
      chartId,
      dirty: true,
      mode: FRAME_SAB,
      index,
      width,
      height,
    });
    return;
  }
  const copy = rgba.buffer.slice(rgba.byteOffset, rgba.byteOffset + rgba.byteLength);
  postMessage(
    {
      type: MSG.FRAME,
      chartId,
      dirty: true,
      mode: FRAME_BUFFER,
      width,
      height,
      buffer: copy,
    },
    [copy],
  );
}

function applyFonts(fonts) {
  if (!fonts || fonts.length === 0 || typeof native.registerFont !== 'function') {
    return;
  }
  for (const item of fonts) {
    const data = item && item.data;
    if (!data) {
      continue;
    }
    const bytes = data instanceof Uint8Array ? data : new Uint8Array(data);
    native.registerFont(bytes, item.opts || {});
  }
}

function applyMaps(mapList) {
  if (!mapList || mapList.length === 0 || typeof native.registerMap !== 'function') {
    return;
  }
  for (const item of mapList) {
    native.registerMap(item.name, item.geoJSON || null, item.specialAreas || null);
  }
}

function updateFont(handle) {
  if (handle && typeof handle.update_font_database === 'function') {
    handle.update_font_database();
  }
}

function serializeHit(hit) {
  if (hit == null) {
    return null;
  }
  return {
    seriesIndex: hit.seriesIndex,
    dataIndex: hit.dataIndex,
    dataType: hit.dataType,
    pathIndex: hit.pathIndex,
    componentType: hit.componentType,
  };
}

function pointerResult(handle, result, chartId, paint) {
  const dirty = !result || result.dirty !== false;
  if (paint && dirty) {
    sendFrame(chartId, handle, true);
  } else if (paint) {
    sendFrame(chartId, handle, false);
  }
  return {
    dirty: dirty,
    hit: serializeHit(result && result.hit),
    tooltip: result && result.tooltip != null ? result.tooltip : null,
    axisPointer: result && result.axisPointer != null ? result.axisPointer : null,
  };
}

function handleMessage(msg) {
  const { type, reqId, chartId } = msg;
  try {
    switch (type) {
      case MSG.CREATE: {
        applyFonts(msg.fonts);
        applyMaps(msg.maps);
        const handle = new native.EChartsInstance(msg.width, msg.height, msg.dpr);
        charts.set(chartId, handle);
        updateFont(handle);
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, { width: handle.width(), height: handle.height(), dpr: handle.dpr() });
        break;
      }
      case MSG.DISPOSE: {
        const handle = charts.get(chartId);
        if (handle) {
          if (typeof handle.dispose === 'function') {
            handle.dispose();
          }
          if (typeof handle.free === 'function') {
            handle.free();
          }
        }
        charts.delete(chartId);
        sabState.delete(chartId);
        reply(reqId, chartId, null);
        break;
      }
      case MSG.SET_OPTION: {
        const handle = getChart(chartId);
        handle.set_option(msg.option, msg.flags || {});
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, {
          hasOption: handle.has_option(),
          width: handle.width(),
          height: handle.height(),
        });
        break;
      }
      case MSG.RESIZE: {
        const handle = getChart(chartId);
        handle.resize(msg.width, msg.height, msg.dpr);
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, { width: handle.width(), height: handle.height(), dpr: handle.dpr() });
        break;
      }
      case MSG.REFRESH: {
        const handle = getChart(chartId);
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, { width: handle.width(), height: handle.height() });
        break;
      }
      case MSG.POINTER_MOVE: {
        const handle = getChart(chartId);
        const result = handle.handle_pointer_move(msg.x, msg.y);
        reply(reqId, chartId, pointerResult(handle, result, chartId, true));
        break;
      }
      case MSG.POINTER_DOWN: {
        const handle = getChart(chartId);
        const hit = typeof handle.handlePointerDown === 'function'
          ? handle.handlePointerDown(msg.x, msg.y)
          : null;
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, serializeHit(hit));
        break;
      }
      case MSG.POINTER_UP: {
        const handle = getChart(chartId);
        if (typeof handle.handlePointerUp === 'function') {
          handle.handlePointerUp(msg.x, msg.y);
        }
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, null);
        break;
      }
      case MSG.POINTER_CLICK: {
        const handle = getChart(chartId);
        let hit = handle.find_hover(msg.x, msg.y);
        if (typeof handle.handlePointerClick === 'function') {
          hit = handle.handlePointerClick(msg.x, msg.y) || hit;
        }
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, serializeHit(hit));
        break;
      }
      case MSG.POINTER_LEAVE: {
        const handle = getChart(chartId);
        handle.handle_pointer_leave();
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, null);
        break;
      }
      case MSG.DISPATCH: {
        const handle = getChart(chartId);
        handle.dispatch_action(msg.payload);
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, null);
        break;
      }
      case MSG.APPEND_DATA: {
        const handle = getChart(chartId);
        handle.appendData(msg.params);
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, { hasOption: handle.has_option() });
        break;
      }
      case MSG.APPLY_WHEEL: {
        const handle = getChart(chartId);
        handle.apply_data_zoom_wheel(msg.x, msg.deltaY);
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, null);
        break;
      }
      case MSG.FIND_HOVER: {
        const handle = getChart(chartId);
        reply(reqId, chartId, serializeHit(handle.find_hover(msg.x, msg.y)));
        break;
      }
      case MSG.GET_TOOLTIP: {
        const handle = getChart(chartId);
        const html = handle.get_tooltip_content(msg.seriesIndex, msg.dataIndex);
        reply(reqId, chartId, html == null ? null : html);
        break;
      }
      case MSG.CONVERT_TO: {
        const handle = getChart(chartId);
        reply(reqId, chartId, handle.convert_to_pixel(msg.finder, msg.value));
        break;
      }
      case MSG.CONVERT_FROM: {
        const handle = getChart(chartId);
        reply(reqId, chartId, handle.convert_from_pixel(msg.finder, msg.value));
        break;
      }
      case MSG.CONTAIN: {
        const handle = getChart(chartId);
        reply(reqId, chartId, !!handle.containPixel(msg.finder, msg.value));
        break;
      }
      case MSG.GET_OPTION: {
        const handle = getChart(chartId);
        reply(reqId, chartId, handle.get_option());
        break;
      }
      case MSG.HAS_OPTION: {
        const handle = getChart(chartId);
        reply(reqId, chartId, handle.has_option());
        break;
      }
      case MSG.REGISTER_FONT: {
        applyFonts([{ data: msg.data, opts: msg.opts }]);
        for (const handle of charts.values()) {
          updateFont(handle);
        }
        reply(reqId, chartId, null);
        break;
      }
      case MSG.CLEAR_FONTS: {
        if (typeof native.clearFonts === 'function') {
          native.clearFonts();
        }
        for (const handle of charts.values()) {
          updateFont(handle);
        }
        reply(reqId, chartId, null);
        break;
      }
      case MSG.REGISTER_MAP: {
        applyMaps([
          {
            name: msg.name,
            geoJSON: msg.geoJSON,
            specialAreas: msg.specialAreas,
          },
        ]);
        reply(reqId, chartId, null);
        break;
      }
      case MSG.UPDATE_FONT: {
        const handle = charts.get(chartId);
        updateFont(handle);
        reply(reqId, chartId, null);
        break;
      }
      case MSG.BENCHMARK: {
        const handle = getChart(chartId);
        const avg = handle.benchmark_render(msg.iterations);
        sendFrame(chartId, handle, true);
        reply(reqId, chartId, avg);
        break;
      }
      default:
        throw new Error(`[wasm-echarts worker] 未知消息 ${type}`);
    }
  } catch (err) {
    fail(reqId, chartId, err);
  }
}

self.onmessage = (event) => {
  const msg = event.data;
  if (!msg || typeof msg !== 'object') {
    return;
  }
  ensureWasm()
    .then(() => handleMessage(msg))
    .catch((err) => fail(msg.reqId, msg.chartId, err));
};

ensureWasm()
  .then(() => {
    postMessage({ type: MSG.READY, sabEnabled });
  })
  .catch((err) => {
    postMessage({
      type: MSG.ERROR,
      message: err && (err.stack || err.message) ? String(err.stack || err.message) : String(err),
    });
  });
