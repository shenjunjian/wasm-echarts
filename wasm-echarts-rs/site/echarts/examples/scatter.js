import echarts from '../../src/echarts/echarts.js';

async function main() {
  const chart = await echarts.init(document.getElementById('chart'), {
    width: 480,
    height: 360,
  });

  chart.setOption({
    tooltip: { trigger: 'item' },
    xAxis: { type: 'value', scale: true },
    yAxis: { type: 'value', scale: true },
    series: [
      {
        type: 'scatter',
        name: '样本',
        data: [
          [10.0, 8.04],
          [8.07, 6.95],
          [13.0, 7.58],
          [9.05, 8.81],
          [11.0, 8.33],
          [14.0, 7.66],
          [13.4, 6.81],
          [10.0, 6.33],
          [14.0, 8.96],
          [12.5, 6.82],
        ],
      },
    ],
  });
}

main();
