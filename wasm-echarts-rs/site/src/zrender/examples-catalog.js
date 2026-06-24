function buildZrenderSource(scene, insideMain = '') {
  return `// wasm-pack build --target web 后，从 pkg 引入 JS 与 WASM
import initWasm, { ZRenderInstance } from './pkg/wasm_zrender.js';

async function main() {
  // 1. 初始化 WASM 模块（加载 .wasm 并完成 bindgen 绑定）
  await initWasm();

  // 2. 创建 canvas 与 ZRender 实例
  const canvas = document.getElementById('canvas');
  const width = 480;
  const height = 360;
  const dpr = window.devicePixelRatio || 1;

  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = \`\${width}px\`;
  canvas.style.height = \`\${height}px\`;

  const zr = new ZRenderInstance(width, height, dpr);

  // 3. 加载内置场景并渲染到 RGBA 像素
  zr.load_scene('${scene}');
  paint(zr, canvas);
${insideMain}
}

function paint(zr, canvas) {
  const rgba = zr.refresh();
  const w = zr.width();
  const h = zr.height();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(new ImageData(new Uint8ClampedArray(rgba), w, h), 0, 0);
}

main();`;
}

export const ZRENDER_SOURCE = {
  shapes: buildZrenderSource('shapes'),
  text: buildZrenderSource('text'),
  sector: buildZrenderSource('sector'),
  hit: buildZrenderSource(
    'hit',
    `
  // 4. 命中检测：鼠标移动时查询 hover 目标
  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hit = zr.find_hover(e.clientX - rect.left, e.clientY - rect.top);
    console.log(hit);
    paint(zr, canvas);
  });
  canvas.addEventListener('mouseleave', () => {
    paint(zr, canvas);
  });`,
  ),
  state: buildZrenderSource(
    'state',
    `
  // 4. emphasis / downplay 状态切换
  let lastIndex = null;
  canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hit = zr.find_hover(e.clientX - rect.left, e.clientY - rect.top);
    if (hit?.pathIndex != null) {
      if (lastIndex != null && lastIndex !== hit.pathIndex) {
        zr.downplay_path(lastIndex);
      }
      zr.highlight_path(hit.pathIndex);
      lastIndex = hit.pathIndex;
      paint(zr, canvas);
    }
  });`,
  ),
};

export const ZRENDER_EXAMPLES = [
  {
    id: 'shapes',
    title: '基础图形 shapes',
    description: 'Rect / Circle / Line / Polygon / 渐变 / 虚线 / 阴影',
    previewUrl: './shapes.html',
    source: ZRENDER_SOURCE.shapes,
  },
  {
    id: 'text',
    title: '文本 text',
    description: 'fillText、对齐、中文',
    previewUrl: './text.html',
    source: ZRENDER_SOURCE.text,
  },
  {
    id: 'sector',
    title: '扇区 sector',
    description: 'SectorShape 饼图扇区',
    previewUrl: './sector.html',
    source: ZRENDER_SOURCE.sector,
  },
  {
    id: 'hit',
    title: '命中检测 hit',
    description: 'find_hover + ECData 回显',
    previewUrl: './hit.html',
    source: ZRENDER_SOURCE.hit,
  },
  {
    id: 'state',
    title: '状态 state',
    description: 'emphasis / downplay 切换',
    previewUrl: './state.html',
    source: ZRENDER_SOURCE.state,
  },
];
