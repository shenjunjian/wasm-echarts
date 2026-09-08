import initWasm, { init, Circle } from '@wasm-zrender';

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  const dpr = window.devicePixelRatio || 1;
  const width = window.innerWidth;
  const height = window.innerHeight;
  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const zr = init(null, { width, height, devicePixelRatio: dpr });
  const w = zr.getWidth();
  const h = zr.getHeight();

  const r = 30;
  const circle = new Circle({
    shape: {
      cx: r,
      cy: h / 2,
      r,
    },
    style: {
      fill: 'transparent',
      stroke: '#FF6EBE',
    },
    silent: true,
  });

  // 终态语义：不播中间帧，start() 立刻写入最后一组 when。
  circle.animate('shape', true)
    .when(5000, { cx: r })
    .when(10000, { cx: w - r })
    .start();

  zr.add(circle);
  paint(zr, canvas);
}

function paint(zr, canvas) {
  const rgba = zr.refresh();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(rgba), zr.width(), zr.height()),
    0,
    0,
  );
}

main();
