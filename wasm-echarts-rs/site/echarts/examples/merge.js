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
  // 默认深合并：series[0] 变成 line，轴保留
  chart.setOption({
    series: [{ type: 'line', name: '第二批', data: [15, 25, 18] }],
  });

  const merged = chart.getOption();
  if (!merged.series || merged.series[0].type !== 'line') {
    throw new Error('getOption 未返回合并后的 series');
  }

  // 对照（本例不调用，以免盖掉合并预览）：
  // chart.setOption({ series: [{ type: 'bar', data: [1, 2, 3] }] }, true);
  // chart.setOption({ series: [{ type: 'bar', data: [1, 2, 3] }] }, { replaceMerge: ['series'] });
}

main();
