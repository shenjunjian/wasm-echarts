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
    tooltip: {
      trigger: 'item',
      axisPointer: { type: 'line' },
      formatter(params) {
        return `${params.seriesName}<br/>${params.name}: ${params.value}`;
      },
    },
    dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
    xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
    yAxis: { type: 'value' },
    series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
  });

  const logEl = document.getElementById('log');
  chart.on('click', (params) => {
    chart.dispatchAction({
      type: 'toggleSelect',
      seriesIndex: params.seriesIndex,
      dataIndex: params.dataIndex,
    });
    if (logEl) {
      logEl.textContent = `click select → seriesIndex=${params.seriesIndex}, dataIndex=${params.dataIndex}`;
    }
  });
}

main();
