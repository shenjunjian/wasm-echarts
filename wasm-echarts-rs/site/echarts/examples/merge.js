import initWasm, { EChartsInstance } from '@wasm-echarts';

const width = 480;
const height = 360;

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = new EChartsInstance(width, height, 1);
  chart.set_option({
    xAxis: { type: 'category', data: ['A', 'B', 'C'] },
    yAxis: { type: 'value' },
    series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
  });
  chart.set_option({
    series: [{ type: 'line', name: '第二批', data: [15, 25, 18] }],
  });

  paint(chart, canvas);
}

function paint(chart, canvas) {
  const rgba = chart.refresh();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(rgba), chart.width(), chart.height()),
    0,
    0,
  );
}

main();
