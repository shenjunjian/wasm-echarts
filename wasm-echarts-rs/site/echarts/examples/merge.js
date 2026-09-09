import initWasm, { init } from '@wasm-echarts';

const width = 480;
const height = 360;

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = init(canvas);
  chart.setOption({
    xAxis: { type: 'category', data: ['A', 'B', 'C'] },
    yAxis: { type: 'value' },
    series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
  });
  chart.setOption({
    series: [{ type: 'line', name: '第二批', data: [15, 25, 18] }],
  });
}

main();
