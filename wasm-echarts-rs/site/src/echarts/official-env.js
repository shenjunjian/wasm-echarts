/**
 * 官网示例薄环境：只提供 ROOT_PATH / $ / app 等沙箱变量。
 * 不 init、不 setOption；每个 examples/*.js 自己创建图表。
 */

import { withBase } from '../shared/site-base.js';

export const ROOT_PATH = withBase('/echarts-official');
export const CDN_PATH = 'https://fastly.jsdelivr.net/npm/';

export const app = { config: {}, configParameters: {} };

if (typeof globalThis !== 'undefined') {
  globalThis.ROOT_PATH = ROOT_PATH;
  globalThis.CDN_PATH = CDN_PATH;
}

export function showPreviewError(err) {
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

export function sizeCanvas(canvas) {
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
      showPreviewError(err);
      console.error(err);
    });
  return request;
}

function jqueryGetScript(url, maybeCb) {
  const request = new Promise((resolve, reject) => {
    const script = document.createElement('script');
    script.src = url;
    script.async = true;
    script.onload = () => resolve(url);
    script.onerror = () => reject(new Error(`GET script ${url} 失败`));
    document.head.appendChild(script);
  })
    .then((loaded) => {
      if (typeof maybeCb === 'function') maybeCb();
      return loaded;
    })
    .catch((err) => {
      showPreviewError(err);
      console.error(err);
    });
  request.done = function done(cb) {
    request.then((loaded) => {
      if (loaded != null && typeof cb === 'function') cb();
    });
    return request;
  };
  request.fail = function fail(cb) {
    request.catch(cb);
    return request;
  };
  return request;
}

function jqueryWhen(...deferreds) {
  const promise = Promise.all(deferreds);
  const wrap = (results) => results.map((data) => [data, 'success', null]);
  const thenable = {
    done(cb) {
      promise.then((results) => cb(...wrap(results))).catch((err) => {
        showPreviewError(err);
        console.error(err);
      });
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

export const $ = {
  get: jqueryGet,
  getJSON: jqueryGet,
  getScript: jqueryGetScript,
  when: jqueryWhen,
};

if (typeof window !== 'undefined' && !window.__OFFICIAL_ENV_LISTENERS__) {
  window.__OFFICIAL_ENV_LISTENERS__ = true;
  window.addEventListener('error', (event) => {
    showPreviewError(event.error || event.message);
  });
  window.addEventListener('unhandledrejection', (event) => {
    showPreviewError(event.reason);
  });
}
