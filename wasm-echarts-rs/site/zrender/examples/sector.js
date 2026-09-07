import initWasm, {
  init,
  Group,
  Sector,
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
  const cx = 240;
  const cy = 180;
  const r = 120;
  const values = [30, 70, 100, 50];
  const colors = ['#5470c6', '#91cc75', '#fac858', '#ee6666'];
  const total = values.reduce((sum, v) => sum + v, 0);
  let angle = -Math.PI / 2;

  values.forEach((value, i) => {
    const sweep = (value / total) * Math.PI * 2;
    const start = angle;
    const end = angle + sweep;
    angle = end;

    g.add(new Sector({
      shape: { cx, cy, r, startAngle: start, endAngle: end },
      style: { fill: colors[i], stroke: '#fff', lineWidth: 1 },
      seriesIndex: 0,
      dataIndex: i,
    }));
  });

  zr.add(g);
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
