const IMPORT_BLOCK = `import initWasm, {
  init,
  dispose,
  Group,
  Rect,
  Circle,
  Line,
  Polygon,
  Sector,
  Arc,
  Ellipse,
  Ring,
  BezierCurve,
  Text,
  LinearGradient,
} from '@wasm-zrender/wasm_zrender.js';`;

const TEXT_IMPORT_BLOCK = `import initWasm, {
  init,
  dispose,
  registerFont,
  Text,
} from '@wasm-zrender/wasm_zrender.js';`;

const BOILERPLATE = `${IMPORT_BLOCK}

const width = 480;
const height = 360;
const dpr = window.devicePixelRatio || 1;

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = \`\${width}px\`;
  canvas.style.height = \`\${height}px\`;

  const zr = init(null, { width, height, devicePixelRatio: dpr });
`;

const TEXT_BOILERPLATE = `${TEXT_IMPORT_BLOCK}

const width = 480;
const height = 360;
const dpr = window.devicePixelRatio || 1;
const FONT_URL = '/fonts/NotoSansSC-Regular.ttf';
const FONT_FAMILY = 'Noto Sans SC';

async function loadFont(url) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(\`字体加载失败: \${url}\`);
  }
  const bytes = new Uint8Array(await response.arrayBuffer());
  registerFont(bytes, {
    familyName: FONT_FAMILY,
    sansSerif: [FONT_FAMILY],
  });
}

async function main() {
  await initWasm();
  await loadFont(FONT_URL);

  const canvas = document.getElementById('canvas');
  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = \`\${width}px\`;
  canvas.style.height = \`\${height}px\`;

  const zr = init(null, { width, height, devicePixelRatio: dpr });
`;

const FOOTER = `
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

main();`;

export const ZRENDER_SOURCE = {
  shapes: `${BOILERPLATE}
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
${FOOTER}`,

  text: `${TEXT_BOILERPLATE}
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
      x: 440,
      y: 140,
      fill: '#666',
      fontSize: 12,
      textAlign: 'right',
    },
  }));
${FOOTER}`,

  sector: `${BOILERPLATE}
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
${FOOTER}`,

  hit: `${BOILERPLATE}
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

  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hover = zr.findHover(e.clientX - rect.left, e.clientY - rect.top);
    console.log(hover?.target?.type, hover?.target?.id);
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

main();`,

  state: `${BOILERPLATE}
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

  let active = null;
  canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hover = zr.findHover(e.clientX - rect.left, e.clientY - rect.top);
    const target = hover?.target && rects.find((r) => r.id === hover.target.id);
    if (!target) return;
    if (active && active.id !== target.id) active.useState('normal');
    target.useState('emphasis');
    active = target;
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

main();`,
};

export const ZRENDER_EXAMPLES = [
  {
    id: 'shapes',
    title: '基础图形 shapes',
    description: 'Group + Rect / Circle / Line / Polygon',
    previewUrl: './shapes.html',
    source: ZRENDER_SOURCE.shapes,
  },
  {
    id: 'text',
    title: '文本 text',
    description: 'new Text({ style }) 根节点渲染',
    previewUrl: './text.html',
    source: ZRENDER_SOURCE.text,
  },
  {
    id: 'sector',
    title: '扇区 sector',
    description: '循环 new Sector 饼图扇区',
    previewUrl: './sector.html',
    source: ZRENDER_SOURCE.sector,
  },
  {
    id: 'hit',
    title: '命中检测 hit',
    description: 'findHover + 鼠标移动',
    previewUrl: './hit.html',
    source: ZRENDER_SOURCE.hit,
  },
  {
    id: 'state',
    title: '状态 state',
    description: 'setStateStyle + useState emphasis',
    previewUrl: './state.html',
    source: ZRENDER_SOURCE.state,
  },
];
