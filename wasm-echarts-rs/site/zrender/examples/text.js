import initWasm, {
  init,
  registerFont,
  Text,
} from '@wasm-zrender';
import { DEFAULT_FONT_URL as FONT_URL } from '../../src/shared/site-base.js';

const width = 480;
const height = 360;
const FONT_FAMILY = 'Noto Sans SC';

async function loadFont() {
  const response = await fetch(FONT_URL);
  if (!response.ok) {
    throw new Error(`字体加载失败: ${FONT_URL} (${response.status})`);
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
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const zr = init(null, { width, height });

  zr.add(new Text({
    style: {
      text: 'wasm-zrender 文本',
      x: 24,
      y: 48,
      fill: '#333',
      fontSize: 18,
      fontWeight: 'bold',
    },
  }));

  zr.add(new Text({
    style: {
      text: '对齐 · 中文 · fillText',
      x: 24,
      y: 96,
      fill: '#5470c6',
      fontSize: 14,
    },
  }));

  zr.add(new Text({
    style: {
      text: 'right align',
      x: 200,
      y: 140,
      fill: '#666',
      fontSize: 12,
      textAlign: 'right',
    },
  }));

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
