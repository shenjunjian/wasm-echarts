import initWasm, {
  init,
  Circle,
  Rect,
  RadialGradient,
} from '@wasm-zrender';

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  const width = window.innerWidth;
  const height = window.innerHeight;
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const zr = init(null, { width, height });
  const w = zr.getWidth();
  const h = zr.getHeight();

  const sun = new Circle({
    shape: {
      cx: 0,
      cy: 0,
      r: 50,
    },
    style: {
      fill: '#FF904F',
    },
    position: [w / 2, h / 2],
  });
  zr.add(sun);

  const water = new Rect({
    shape: {
      x: 0,
      y: 0,
      width: w,
      height: h / 2,
    },
    style: {
      fill: new RadialGradient(0.5, -0.1, 1, [
        { offset: 0, color: '#FFB166' },
        { offset: 0.2, color: '#D7C467' },
        { offset: 1, color: '#37B0FF' },
      ]),
    },
    position: [0, h / 2],
  });
  zr.add(water);

  const sky = new Rect({
    shape: {
      x: 0,
      y: 0,
      width: w,
      height: h,
    },
    style: {
      fill: '#D7F9FF',
    },
    zlevel: -1,
  });
  zr.add(sky);

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
