import initWasm, {
  init,
  Group,
  Rect,
} from '@wasm-zrender/wasm_zrender.js';

const width = 480;
const height = 360;
const dpr = window.devicePixelRatio || 1;

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const zr = init(null, { width, height, devicePixelRatio: dpr });
  const g = new Group();
  const colors = ['#5470c6', '#91cc75', '#fac858', '#ee6666', '#73c0de', '#3ba272'];
  const rects = [];

  for (let row = 0; row < 2; row++) {
    for (let col = 0; col < 3; col++) {
      const i = row * 3 + col;
      const rect = new Rect({
        shape: { x: 40 + col * 130, y: 60 + row * 100, width: 100, height: 70 },
        style: { fill: colors[i] },
        seriesIndex: 0,
        dataIndex: i,
      });
      rect.setStateStyle('emphasis', { fill: '#ee6666', lineWidth: 4 });
      rects.push(rect);
      g.add(rect);
    }
  }

  zr.add(g);
  paint(zr, canvas);

  const logEl = document.getElementById('log');
  const log = (msg) => {
    if (logEl) logEl.textContent = msg;
  };

  let active = null;
  canvas.addEventListener('click', (e) => {
    const box = canvas.getBoundingClientRect();
    const hover = zr.findHover(e.clientX - box.left, e.clientY - box.top);
    const target = hover?.target && rects.find((r) => r.id === hover.target.id);
    if (!target) return;
    if (active && active.id !== target.id) active.useState('normal');
    target.useState('emphasis');
    active = target;
    log(`emphasis → type=${target.type} id=${target.id}`);
    paint(zr, canvas);
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
