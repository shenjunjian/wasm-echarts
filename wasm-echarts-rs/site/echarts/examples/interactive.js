import echarts from '../../src/echarts/echarts.js';

async function main() {
  const chart = await echarts.init(document.getElementById('chart'), {
    width: 480,
    height: 360,
  });

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
  chart.on('click', ({ hit }) => {
    if (hit?.dataIndex == null) return;
    if (logEl) {
      logEl.textContent = `click select → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`;
    }
  });
}

main();
