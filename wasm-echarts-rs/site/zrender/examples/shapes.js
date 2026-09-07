import initWasm, {
  init,
  Group,
  Rect,
  Circle,
  Line,
  Polygon,
  Arc,
  Ellipse,
  Ring,
  BezierCurve,
  LinearGradient,
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

  g.add(new Rect({
    shape: { x: 20, y: 20, width: 100, height: 60 },
    style: {
      fill: new LinearGradient(0, 0, 1, 0, [
        { offset: 0, color: '#5470c6' },
        { offset: 1, color: '#91cc75' },
      ]),
    },
  }));

  g.add(new Circle({
    shape: { cx: 180, cy: 80, r: 40 },
    style: {
      fill: 'rgba(145, 204, 117, 0.8)',
      stroke: '#ee6666',
      lineWidth: 3,
    },
    z: 1,
  }));

  g.add(new Line({
    shape: { x1: 20, y1: 120, x2: 280, y2: 120 },
    style: { stroke: '#333', lineWidth: 2, lineDash: [6, 4] },
  }));

  g.add(new Polygon({
    shape: { points: [[240, 30], [300, 60], [270, 100]] },
    style: { fill: '#fac858' },
  }));

  g.add(new Arc({
    shape: { cx: 380, cy: 80, r: 50, startAngle: 0, endAngle: Math.PI * 1.2 },
    style: { stroke: '#5470c6', lineWidth: 4 },
  }));

  g.add(new Ellipse({
    shape: { cx: 120, cy: 240, rx: 70, ry: 40 },
    style: { fill: 'rgba(238, 102, 102, 0.7)' },
  }));

  g.add(new Ring({
    shape: { cx: 300, cy: 260, r: 55, r0: 30 },
    style: { fill: '#73c0de' },
  }));

  g.add(new BezierCurve({
    shape: { x1: 20, y1: 300, x2: 460, y2: 300, cpx1: 240, cpy1: 180 },
    style: { stroke: '#3ba272', lineWidth: 3 },
  }));

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
