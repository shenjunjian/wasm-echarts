import { createZrenderView, ensureDefaultFont } from './zrender.js';
import { ZRENDER_SETUPS } from './example-setups.js';

const EXAMPLE_META = {
  shapes: { title: '基础图形 shapes', width: 480, height: 360 },
  text: { title: '文本 text', width: 480, height: 360 },
  sector: { title: '扇区 sector', width: 480, height: 360 },
  hit: { title: '命中检测 hit', width: 480, height: 360 },
  state: { title: '状态 state', width: 480, height: 360 },
};

/**
 * 运行 zrender 示例
 * @param {string} exampleId shapes | text | sector | hit | state
 * @param {HTMLElement} previewEl
 * @param {(msg: string) => void} log
 * @param {{ interactive?: 'hit' | 'state' }} [hooks]
 */
export async function runZrenderExample(exampleId, previewEl, log, hooks = {}) {
  const meta = EXAMPLE_META[exampleId];
  if (!meta) {
    throw new Error(`未知示例: ${exampleId}`);
  }

  const setup = ZRENDER_SETUPS[exampleId];
  if (exampleId === 'text') {
    await ensureDefaultFont();
  }

  const view = await createZrenderView(previewEl, {
    width: meta.width,
    height: meta.height,
  });

  const setupResult = setup(view.zr);
  view.paint();

  if (hooks.interactive === 'hit' || exampleId === 'hit') {
    view.canvas.addEventListener('mousemove', (e) => {
      const { x, y } = view.clientToLocal(e);
      const hover = view.zr.findHover(x, y);
      if (hover?.target) {
        log(`hover → type=${hover.target.type} id=${hover.target.id}`);
      } else {
        log('鼠标离开可命中区域');
      }
      view.paint();
    });
    view.canvas.addEventListener('mouseleave', () => {
      log('鼠标离开画布');
      view.paint();
    });
  }

  if (hooks.interactive === 'state' || exampleId === 'state') {
    /** @type {import('@wasm-zrender/wasm_zrender.js').Rect | null} */
    let active = null;
    const rects = Array.isArray(setupResult) ? setupResult : [];

    view.canvas.addEventListener('click', (e) => {
      const { x, y } = view.clientToLocal(e);
      const hover = view.zr.findHover(x, y);
      if (!hover?.target) return;

      const rect = rects.find((r) => r.id === hover.target.id);
      if (!rect) return;

      if (active && active.id !== rect.id) {
        active.useState('normal');
      }
      rect.useState('emphasis');
      active = rect;
      log(`emphasis → type=${rect.type} id=${rect.id}`);
      view.paint();
    });
  }

  log(`${meta.title} 已加载 (${meta.width}×${meta.height})`);

  return { dispose: () => view.dispose() };
}
