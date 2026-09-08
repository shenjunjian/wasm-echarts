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

  // 终态语义：不播中间帧，start() 立刻写入最后一组 when（圆停在右侧）。
  circle.animate('shape', true)
    .when(5000, { cx: r })
    .when(10000, { cx: w - r })
    .start();

  if (circle.shape.cx !== w - r) {
    throw new Error('animate 终态应为最后一组 when 的 cx');
  }

  zr.add(circle);
  paint(zr, canvas);
}

function paint(zr, canvas) {
  const rgba = zr.refresh();
  const w = zr.width();
  const h = zr.height();
  const img = new ImageData(new Uint8ClampedArray(rgba), w, h);
  const ctx = canvas.getContext('2d');
  // refresh() 返回 CSS 像素缓冲；canvas 位图是 CSS×dpr。
  // 直接 putImageData(0,0) 只会画在物理画布左上角，1px 描边再被 CSS 缩小后会看不见。
  if (canvas.width === w && canvas.height === h) {
    ctx.putImageData(img, 0, 0);
    return;
  }
  const off = document.createElement('canvas');
  off.width = w;
  off.height = h;
  off.getContext('2d').putImageData(img, 0, 0);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.setTransform(canvas.width / w, 0, 0, canvas.height / h, 0, 0);
  ctx.drawImage(off, 0, 0);
}

main();
