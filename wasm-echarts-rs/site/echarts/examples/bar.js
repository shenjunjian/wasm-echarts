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
    xAxis: {
      type: 'category',
      data: ['A', 'B', 'C', 'D'],
      axisLabel: { formatter: '{value}' },
    },
    yAxis: { type: 'value', axisLabel: { formatter: '{value}' } },
    series: [
      {
        type: 'bar',
        name: '数量',
        data: [40, 90, 60, 120],
        label: { show: true },
      },
    ],
  });
}

main();
