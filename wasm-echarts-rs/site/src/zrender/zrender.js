import initWasm, { init, dispose } from '@wasm-zrender/wasm_zrender.js';

let wasmReady = null;

export function ensureWasm() {
  if (!wasmReady) {
    wasmReady = initWasm();
  }
  return wasmReady;
}

/** @param {import('@wasm-zrender/wasm_zrender.js').ZRender} zr @param {HTMLCanvasElement} canvas */
export function paintCanvas(zr, canvas) {
  const rgba = zr.refresh();
  const w = zr.width();
  const h = zr.height();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(new ImageData(new Uint8ClampedArray(rgba), w, h), 0, 0);
}

/**
 * 创建 canvas 宿主并初始化 ZRender
 * @param {HTMLElement} dom
 * @param {{ width?: number, height?: number, devicePixelRatio?: number }} [opts]
 */
export async function createZrenderView(dom, opts = {}) {
  await ensureWasm();

  dom.style.position = dom.style.position || 'relative';
  dom.innerHTML = '';

  const width = Math.max(1, Math.floor(opts.width ?? 480));
  const height = Math.max(1, Math.floor(opts.height ?? 360));
  const dpr = opts.devicePixelRatio ?? window.devicePixelRatio ?? 1;

  const canvas = document.createElement('canvas');
  canvas.style.display = 'block';
  dom.appendChild(canvas);

  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const zr = init(null, { width, height, devicePixelRatio: dpr });

  return {
    zr,
    canvas,
    width,
    height,
    dpr,
    paint() {
      paintCanvas(zr, canvas);
    },
    /** @param {MouseEvent} e */
    clientToLocal(e) {
      const rect = canvas.getBoundingClientRect();
      return {
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
      };
    },
    dispose() {
      dispose(zr);
      canvas.remove();
    },
  };
}

export default { ensureWasm, paintCanvas, createZrenderView };
