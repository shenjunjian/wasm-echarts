import init, { DemoRenderer } from '../crates/wasm-echarts/pkg/wasm_echarts.js';

const CANVAS_WIDTH = 400;
const CANVAS_HEIGHT = 300;

async function main() {
  const status = document.getElementById('status');
  const canvas = document.getElementById('canvas');

  try {
    await init();
    status.textContent = 'WASM 已加载，正在渲染…';

    const renderer = new DemoRenderer(CANVAS_WIDTH, CANVAS_HEIGHT);
    const rgba = renderer.render();

    const ctx = canvas.getContext('2d');
    const imageData = new ImageData(
      new Uint8ClampedArray(rgba),
      CANVAS_WIDTH,
      CANVAS_HEIGHT,
    );
    ctx.putImageData(imageData, 0, 0);

    status.textContent = `渲染完成 (${CANVAS_WIDTH}×${CANVAS_HEIGHT}, ${rgba.length} bytes RGBA)`;
  } catch (err) {
    status.textContent = `错误: ${err}`;
    console.error(err);
  }
}

main();
