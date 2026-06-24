import { formatOption } from '@shared/parse-source.js';
import { LINE_OPTION, BAR_OPTION, PIE_OPTION, SCATTER_OPTION } from '@echarts/options.js';

function buildEchartsSource(option, insideMain = '') {
  return `// wasm-pack build --target web 后，从 pkg 引入 JS 与 WASM
import initWasm, { EChartsInstance } from './pkg/wasm_echarts.js';

async function main() {
  // 1. 初始化 WASM 模块
  await initWasm();

  // 2. 准备 canvas 容器
  const dom = document.getElementById('chart');
  const canvas = document.createElement('canvas');
  dom.appendChild(canvas);

  const width = 480;
  const height = 360;
  const dpr = window.devicePixelRatio || 1;
  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = \`\${width}px\`;
  canvas.style.height = \`\${height}px\`;

  // 3. 创建 ECharts WASM 实例
  const chart = new EChartsInstance(width, height, dpr);

  // 4. 设置 option 并渲染
  chart.set_option(${formatOption(option)});
  paint(chart, canvas);
${insideMain}
  // 5. 交互：hover / click / 滚轮缩放
  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    chart.handle_pointer_move(e.clientX - rect.left, e.clientY - rect.top);
    paint(chart, canvas);
  });
  canvas.addEventListener('mouseleave', () => {
    chart.handle_pointer_leave();
    paint(chart, canvas);
  });
  canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hit = chart.find_hover(e.clientX - rect.left, e.clientY - rect.top);
    if (hit?.seriesIndex != null && hit?.dataIndex != null) {
      chart.dispatch_action({
        type: 'toggleSelect',
        seriesIndex: hit.seriesIndex,
        dataIndex: hit.dataIndex,
      });
      paint(chart, canvas);
    }
  });
  canvas.addEventListener('wheel', (e) => {
    e.preventDefault();
    const rect = canvas.getBoundingClientRect();
    chart.apply_data_zoom_wheel(e.clientX - rect.left, e.deltaY);
    paint(chart, canvas);
  }, { passive: false });
}

function paint(chart, canvas) {
  const rgba = chart.refresh();
  const w = chart.width();
  const h = chart.height();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(new ImageData(new Uint8ClampedArray(rgba), w, h), 0, 0);
}

main();`;
}

export const ECHARTS_SOURCE = {
  line: buildEchartsSource(LINE_OPTION),
  bar: buildEchartsSource(BAR_OPTION),
  pie: buildEchartsSource(PIE_OPTION),
  scatter: buildEchartsSource(SCATTER_OPTION),
  interactive: buildEchartsSource({
    tooltip: {
      trigger: 'item',
      formatter(params) {
        return `${params.seriesName}<br/>${params.name}: ${params.value}`;
      },
    },
    dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
    xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
    yAxis: { type: 'value' },
    series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
  }),
  merge: `import initWasm, { EChartsInstance } from './pkg/wasm_echarts.js';

async function main() {
  await initWasm();

  const dom = document.getElementById('chart');
  const canvas = document.createElement('canvas');
  dom.appendChild(canvas);

  const width = 480;
  const height = 360;
  const dpr = window.devicePixelRatio || 1;
  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = \`\${width}px\`;
  canvas.style.height = \`\${height}px\`;

  const chart = new EChartsInstance(width, height, dpr);

  // 第一次 setOption
  chart.set_option(${formatOption({
    xAxis: { type: 'category', data: ['A', 'B', 'C'] },
    yAxis: { type: 'value' },
    series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
  })});

  // 第二次 setOption（merge 追加 series）
  chart.set_option({
    series: [{ type: 'line', name: '第二批', data: [15, 25, 18] }],
  });

  paint(chart, canvas);
}

function paint(chart, canvas) {
  const rgba = chart.refresh();
  const w = chart.width();
  const h = chart.height();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(new ImageData(new Uint8ClampedArray(rgba), w, h), 0, 0);
}

main();`,
  bench: buildEchartsSource(
    LINE_OPTION,
    `
  const avgMs = chart.benchmark_render(30);
  console.log(\`benchmark_render 均值 \${avgMs.toFixed(2)} ms\`);
`,
  ),
};

export const ECHARTS_EXAMPLES = [
  {
    id: 'line',
    title: '折线图 line',
    description: 'category 轴 + inside dataZoom',
    previewUrl: './line.html',
    source: ECHARTS_SOURCE.line,
  },
  {
    id: 'bar',
    title: '柱状图 bar',
    description: '基础柱状',
    previewUrl: './bar.html',
    source: ECHARTS_SOURCE.bar,
  },
  {
    id: 'pie',
    title: '饼图 pie',
    description: '扇区 + tooltip',
    previewUrl: './pie.html',
    source: ECHARTS_SOURCE.pie,
  },
  {
    id: 'scatter',
    title: '散点图 scatter',
    description: 'value 轴',
    previewUrl: './scatter.html',
    source: ECHARTS_SOURCE.scatter,
  },
  {
    id: 'interactive',
    title: '交互合集 interactive',
    description: 'formatter tooltip + hover / select / zoom',
    previewUrl: './interactive.html',
    source: ECHARTS_SOURCE.interactive,
  },
  {
    id: 'merge',
    title: 'setOption 合并 merge',
    description: '二次 setOption 演示',
    previewUrl: './merge.html',
    source: ECHARTS_SOURCE.merge,
  },
  {
    id: 'bench',
    title: '性能基准 bench',
    description: 'benchmark_render 30 次均值',
    previewUrl: './bench.html',
    source: ECHARTS_SOURCE.bench,
  },
];
