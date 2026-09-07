import echarts from '../../src/echarts/echarts.js';

async function main() {
  const chart = await echarts.init(document.getElementById('chart'), {
    width: 480,
    height: 360,
  });

  chart.setOption({
    tooltip: { trigger: 'item', axisPointer: { type: 'line' } },
    dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
    xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
    yAxis: { type: 'value' },
    series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
  });

  const avgMs = chart.benchmark(30);
  const logEl = document.getElementById('log');
  if (logEl) {
    logEl.textContent = `benchmark_render 均值 ${avgMs.toFixed(2)} ms（30 次）`;
  }
}

main();
