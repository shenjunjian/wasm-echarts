import initWasm, { ZRenderInstance } from '@wasm-zrender/wasm_zrender.js';

let wasmReady = null;

function ensureWasm() {
  if (!wasmReady) {
    wasmReady = initWasm();
  }
  return wasmReady;
}

function paintCanvas(instance, canvas) {
  const rgba = instance.refresh();
  const w = instance.width();
  const h = instance.height();
  const ctx = canvas.getContext('2d');
  const imageData = new ImageData(new Uint8ClampedArray(rgba), w, h);
  ctx.putImageData(imageData, 0, 0);
}

/** @param {HTMLElement} dom @param {{ width?: number, height?: number, devicePixelRatio?: number, scene?: string }} [opts] */
export async function createViewer(dom, opts = {}) {
  await ensureWasm();

  dom.style.position = dom.style.position || 'relative';
  dom.innerHTML = '';

  const w = Math.max(1, Math.floor(opts.width ?? 480));
  const h = Math.max(1, Math.floor(opts.height ?? 360));
  const dpr = opts.devicePixelRatio ?? window.devicePixelRatio ?? 1;

  const canvas = document.createElement('canvas');
  canvas.style.display = 'block';
  dom.appendChild(canvas);

  canvas.width = Math.floor(w * dpr);
  canvas.height = Math.floor(h * dpr);
  canvas.style.width = `${w}px`;
  canvas.style.height = `${h}px`;

  const instance = new ZRenderInstance(w, h, dpr);
  if (opts.scene) {
    instance.load_scene(opts.scene);
  }

  paintCanvas(instance, canvas);

  return {
    instance,
    canvas,
    dom,
    paint() {
      paintCanvas(instance, canvas);
    },
    loadScene(scene) {
      instance.load_scene(scene);
      paintCanvas(instance, canvas);
    },
    bindHitTest(onHit) {
      canvas.addEventListener('mousemove', (e) => {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        const hit = instance.find_hover(x, y);
        onHit(hit && hit !== null ? hit : null, e);
        paintCanvas(instance, canvas);
      });
      canvas.addEventListener('mouseleave', () => {
        onHit(null, null);
        paintCanvas(instance, canvas);
      });
    },
    bindStateToggle(onState) {
      let lastIndex = null;
      canvas.addEventListener('click', (e) => {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        const hit = instance.find_hover(x, y);
        if (hit && hit !== null && hit.pathIndex != null) {
          const idx = hit.pathIndex;
          if (lastIndex != null && lastIndex !== idx) {
            instance.downplay_path(lastIndex);
          }
          instance.highlight_path(idx);
          lastIndex = idx;
          onState?.(idx, hit);
          paintCanvas(instance, canvas);
        }
      });
    },
    dispose() {
      instance.free();
      canvas.remove();
    },
  };
}

export default { createViewer };
