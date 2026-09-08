import initWasm, {
  init,
  registerFont,
  Text,
  Rect,
} from '@wasm-zrender';

const FONT_URL = '/fonts/NotoSansSC-Regular.ttf';
const FONT_FAMILY = 'Noto Sans SC';

async function loadFont() {
  const response = await fetch(FONT_URL);
  if (!response.ok) {
    throw new Error(`字体加载失败: ${FONT_URL}`);
  }
  const bytes = new Uint8Array(await response.arrayBuffer());
  registerFont(bytes, {
    familyName: FONT_FAMILY,
    sansSerif: [FONT_FAMILY],
  });
}

async function main() {
  await initWasm();
  await loadFont();

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
  const fontSize = Math.max(48, Math.min(200, Math.floor(w / 4.2)));

  const t1 = new Text({
    style: {
      text: 'zrender',
      textAlign: 'center',
      textVerticalAlign: 'middle',
      fontSize,
      fontFamily: FONT_FAMILY,
      fontWeight: 'bolder',
      textFill: '#0ff',
      blend: 'lighten',
    },
    position: [w / 2 + 5, h / 2],
  });
  zr.add(t1);

  const t2 = new Text({
    style: {
      text: 'zrender',
      textAlign: 'center',
      textVerticalAlign: 'middle',
      fontSize,
      fontFamily: FONT_FAMILY,
      fontWeight: 'bolder',
      textFill: '#f0f',
      blend: 'lighten',
    },
    position: [w / 2, h / 2],
  });
  zr.add(t2);

  const lines = [];
  for (let i = 0; i < 16; ++i) {
    const line = new Rect({
      shape: {
        x: w * (Math.random() - 0.3),
        y: h * Math.random(),
        width: w * (Math.random() + 0.3),
        height: Math.random() * 8,
      },
      style: {
        fill: ['#ff0', '#f0f', '#0ff', '#00f'][Math.floor(Math.random() * 4)],
        blend: 'lighten',
        opacity: 0,
      },
    });
    zr.add(line);
    lines.push(line);
  }

  paint(zr, canvas);

  setInterval(() => {
    if (Math.random() > 0.2) {
      t2.attr('position', [w / 2 + Math.random() * 50, h / 2]);

      for (let i = 0; i < lines.length; ++i) {
        lines[i].attr('shape', {
          x: w * Math.random(),
          y: h * Math.random(),
          width: w * Math.random(),
          height: Math.random() * 8,
        });
        lines[i].attr('style', {
          opacity: 1,
        });
      }

      paint(zr, canvas);

      setTimeout(() => {
        t2.attr('position', [w / 2, h / 2]);
        for (let i = 0; i < lines.length; ++i) {
          lines[i].attr('style', {
            opacity: 0,
          });
        }
        paint(zr, canvas);
      }, 100);
    }
  }, 500);
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
