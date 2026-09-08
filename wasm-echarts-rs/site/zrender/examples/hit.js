import initWasm, {
  init,
  Group,
  Rect,
} from '@wasm-zrender';

const width = 480;
const height = 360;

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const zr = init(null, { width, height });
  const g = new Group();
  const colors = ['#5470c6', '#91cc75', '#fac858'];

  [
    { x: 40, y: 40, w: 90, h: 60, dataIndex: 0 },
    { x: 160, y: 50, w: 100, h: 70, dataIndex: 1 },
    { x: 300, y: 60, w: 80, h: 80, dataIndex: 2 },
  ].forEach((item, i) => {
    g.add(new Rect({
      shape: { x: item.x, y: item.y, width: item.w, height: item.h },
      style: { fill: colors[i] },
      seriesIndex: 0,
      dataIndex: item.dataIndex,
    }));
  });

  zr.add(g);
  paint(zr, canvas);

  const logEl = document.getElementById('log');
  const log = (msg) => {
    if (logEl) logEl.textContent = msg;
  };

  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hover = zr.findHover(e.clientX - rect.left, e.clientY - rect.top);
    if (hover?.target) {
      log(`hover → type=${hover.target.type} id=${hover.target.id}`);
    } else {
      log('鼠标离开可命中区域');
    }
    paint(zr, canvas);
  });

  canvas.addEventListener('mouseleave', () => {
    log('鼠标离开画布');
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
