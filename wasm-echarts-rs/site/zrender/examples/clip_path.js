import initWasm, { init, Circle, Heart } from '@wasm-zrender/wasm_zrender.js';

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

  const zr = init(canvas, { width, height, devicePixelRatio: dpr });
  const w = zr.getWidth();
  const h = zr.getHeight();

  const circle = new Circle({
    shape: {
      cx: w / 2,
      cy: h / 2,
      r: 50,
    },
    style: {
      fill: '#FF6EBE',
    },
    draggable: true,
  });

  const heart = new Heart({
    shape: {
      cx: w / 2 + 20,
      cy: h / 2 - 40,
      width: 60,
      height: 80,
    },
    draggable: true,
  });

  circle.setClipPath(heart);
  zr.add(circle);

  const borderA = new Circle({
    shape: {
      cx: w / 2,
      cy: h / 2,
      r: 50,
    },
    style: {
      fill: 'transparent',
      stroke: '#5ACFFF',
    },
  });
  zr.add(borderA);

  const borderB = new Heart({
    shape: {
      cx: w / 2 + 20,
      cy: h / 2 - 40,
      width: 60,
      height: 80,
    },
    style: {
      fill: 'transparent',
      stroke: '#5ACFFF',
    },
  });
  zr.add(borderB);
}

main();
