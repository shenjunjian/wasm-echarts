/**
 * 官网示例宿主：提供 option / myChart / echarts / ROOT_PATH / $ / app，
 * 不补齐 wasm-echarts 未实现的 API。报错会显示在预览层，方便后续对照补功能。
 */

import initWasm, * as echartsApi from '@wasm-echarts';
import { ensureDefaultFont } from './fonts.js';

export const OFFICIAL_ROOT_PATH = '/echarts-official';
export const OFFICIAL_CDN_PATH = 'https://fastly.jsdelivr.net/npm/';

const MISSING_NAMESPACES = [];

function missingNamespace(path) {
  return new Proxy(function missingOfficialApi() {}, {
    get(_target, prop) {
      if (prop === 'then' || prop === '$$typeof' || prop === 'toJSON') {
        return undefined;
      }
      if (prop === Symbol.toPrimitive || prop === Symbol.toStringTag) {
        return () => path;
      }
      throw new Error(`[wasm-echarts] ${path}.${String(prop)} 未实现`);
    },
    apply() {
      throw new Error(`[wasm-echarts] ${path}() 未实现`);
    },
    construct() {
      throw new Error(`[wasm-echarts] new ${path} 未实现`);
    },
  });
}

function attachMissingNamespaces(echarts) {
  for (const name of MISSING_NAMESPACES) {
    if (echarts[name] == null) {
      Object.defineProperty(echarts, name, {
        configurable: true,
        enumerable: false,
        get() {
          return missingNamespace(`echarts.${name}`);
        },
      });
    }
  }
  return echarts;
}

function showError(err) {
  if (typeof document === 'undefined') return;
  let el = document.querySelector('.preview-error');
  if (!el) {
    el = document.createElement('pre');
    el.className = 'preview-error';
    document.body.appendChild(el);
  }
  const message = err?.stack || err?.message || String(err);
  el.textContent = message;
}

function clearError() {
  document.querySelector('.preview-error')?.remove();
}

function sizeCanvas(canvas) {
  const host = canvas.parentElement || canvas;
  const width = Math.max(1, Math.round(host.clientWidth || window.innerWidth || 720));
  const height = Math.max(1, Math.round(host.clientHeight || window.innerHeight || 480));
  canvas.width = width;
  canvas.height = height;
  return { width, height };
}

function jqueryGet(url, maybeData, maybeCb) {
  const cb = typeof maybeData === 'function' ? maybeData : maybeCb;
  const request = fetch(url)
    .then(async (res) => {
      if (!res.ok) {
        throw new Error(`GET ${url} 失败 (${res.status})`);
      }
      const contentType = res.headers.get('content-type') || '';
      if (contentType.includes('json') || /\.json(\?|$)/i.test(url)) {
        return res.json();
      }
      const text = await res.text();
      try {
        return JSON.parse(text);
      } catch {
        return text;
      }
    })
    .then((data) => {
      if (typeof cb === 'function') cb(data);
      return data;
    })
    .catch((err) => {
      reportExampleError(err);
    });
  return request;
}

function jqueryWhen(...deferreds) {
  const promise = Promise.all(deferreds);
  const wrap = (results) => results.map((data) => [data, 'success', null]);
  const thenable = {
    done(cb) {
      promise.then((results) => cb(...wrap(results))).catch((err) => reportExampleError(err));
      return thenable;
    },
    fail(cb) {
      promise.catch(cb);
      return thenable;
    },
    then(onFulfilled, onRejected) {
      return promise.then(
        (results) => onFulfilled?.(...wrap(results)),
        onRejected,
      );
    },
  };
  return thenable;
}

function publishResult(partial) {
  const prev = window.__OFFICIAL_EXAMPLE_RESULT__ || {};
  const next = { ...prev, ...partial, updatedAt: Date.now() };
  window.__OFFICIAL_EXAMPLE_RESULT__ = next;
  try {
    window.parent?.postMessage(
      { type: 'official-example-result', ...next },
      '*',
    );
  } catch {
    // ignore cross-origin
  }
  return next;
}

function reportExampleError(err) {
  showError(err);
  console.error(err);
  publishResult({
    status: 'error',
    error: err?.message || String(err),
    stack: err?.stack || '',
  });
}

/**
 * @param {(env: {
 *   echarts: object;
 *   myChart: object;
 *   ROOT_PATH: string;
 *   CDN_PATH: string;
 *   $: { get: typeof jqueryGet; getJSON: typeof jqueryGet; when: typeof jqueryWhen };
 *   app: { config: Record<string, unknown>; configParameters: Record<string, unknown> };
 * }) => unknown} body
 */
export async function runOfficialExample(body) {
  publishResult({ status: 'pending', error: null, stack: '' });
  window.addEventListener('error', (event) => {
    reportExampleError(event.error || event.message);
  });
  window.addEventListener('unhandledrejection', (event) => {
    reportExampleError(event.reason);
  });

  try {
    await initWasm();
    await ensureDefaultFont();

    const canvas = document.getElementById('canvas');
    if (!canvas) {
      throw new Error('缺少 #canvas');
    }
    sizeCanvas(canvas);

    const echarts = attachMissingNamespaces({ ...echartsApi });
    const myChart = echarts.init(canvas);
    const originalSetOption = myChart.setOption.bind(myChart);
    let optionApplied = false;
    myChart.setOption = function wrappedSetOption(option, ...rest) {
      optionApplied = true;
      try {
        return originalSetOption(option, ...rest);
      } catch (err) {
        reportExampleError(err);
        throw err;
      }
    };

    window.addEventListener('resize', () => {
      if (myChart.isDisposed()) return;
      sizeCanvas(canvas);
      myChart.resize();
    });

    const app = { config: {}, configParameters: {} };
    const $ = { get: jqueryGet, getJSON: jqueryGet, when: jqueryWhen };
    const ROOT_PATH = OFFICIAL_ROOT_PATH;
    const CDN_PATH = OFFICIAL_CDN_PATH;
    globalThis.CDN_PATH = CDN_PATH;
    globalThis.ROOT_PATH = ROOT_PATH;

    let option = await body({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app });
    if (!optionApplied && option) {
      myChart.setOption(option);
    }

    clearError();
    publishResult({ status: 'ok', error: null, stack: '' });
  } catch (err) {
    reportExampleError(err);
  }
}
