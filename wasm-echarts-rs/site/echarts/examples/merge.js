import echarts from '../../src/echarts/echarts.js';

async function main() {
  const chart = await echarts.init(document.getElementById('chart'), {
    width: 480,
    height: 360,
  });

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
