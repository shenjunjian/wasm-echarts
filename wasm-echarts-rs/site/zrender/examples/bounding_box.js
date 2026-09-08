import initWasm, { init, Group, Circle, Rect } from '@wasm-zrender';

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

  let isMouseDown = false;
  zr.on('mousedown', () => {
    isMouseDown = true;
  });
  zr.on('mouseup', () => {
    isMouseDown = false;
  });

  const elementStyle = {
    stroke: '#ccc',
    fill: 'white',
  };

  const group = new Group();
  for (let i = 0; i < 10; ++i) {
    const r = 50 * Math.random() + 20;
    const circle = new Circle({
      shape: {
        cx: 0,
        cy: 0,
        r,
      },
      position: [
        (w * 0.6 - r * 2) * Math.random() + r + w * 0.2,
        (h * 0.6 - r * 2) * Math.random() + r + h * 0.2,
      ],
      style: elementStyle,
      draggable: true,
    }).on('mousemove', () => {
      if (isMouseDown) {
        const rect = group.getBoundingRect();
        boundingRect.setShape({
          x: rect.x,
          y: rect.y,
          width: rect.width,
          height: rect.height,
        });
        paint(zr, canvas);
      }
    });
    group.add(circle);
  }
  zr.add(group);

  const rect = group.getBoundingRect();
  const boundingRect = new Rect({
    shape: {
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
    },
    style: {
      fill: 'none',
      stroke: '#14f1ff',
    },
  });
  zr.add(boundingRect);

  paint(zr, canvas);

  canvas.addEventListener('mousedown', () => {
    isMouseDown = true;
  });
  canvas.addEventListener('mouseup', () => {
    isMouseDown = false;
  });
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
