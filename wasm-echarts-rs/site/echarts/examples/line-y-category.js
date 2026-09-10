/**
 * 官网示例：垂直折线图（Y轴为类目轴）
 * https://echarts.apache.org/examples/zh/editor.html?c=line-y-category
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import initWasm, * as echarts from '@wasm-echarts';
import { ROOT_PATH, CDN_PATH, $, app, sizeCanvas, showPreviewError } from '../../src/echarts/official-env.js';
import { ensureDefaultFont } from '../../src/echarts/fonts.js';

async function main() {
  await initWasm();
  await ensureDefaultFont();

  const canvas = document.getElementById('canvas');
  if (!canvas) {
    throw new Error('缺少 #canvas');
  }
  sizeCanvas(canvas);
  const myChart = echarts.init(canvas);
  window.addEventListener('resize', () => {
    if (myChart.isDisposed()) return;
    sizeCanvas(canvas);
    myChart.resize();
  });

  let option;
  /*
  title: Line Y Category
  category: line
  titleCN: 垂直折线图（Y轴为类目轴）
  difficulty: 8
  */
  option = {
    legend: {
      data: ['Altitude (km) vs. temperature (°C)']
    },
    tooltip: {
      trigger: 'axis',
      formatter: 'Temperature : <br/>{b}km : {c}°C'
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'value',
      axisLabel: {
        formatter: '{value} °C'
      }
    },
    yAxis: {
      type: 'category',
      axisLine: { onZero: false },
      axisLabel: {
        formatter: '{value} km'
      },
      boundaryGap: false,
      data: ['0', '10', '20', '30', '40', '50', '60', '70', '80']
    },
    series: [
      {
        name: 'Altitude (km) vs. temperature (°C)',
        type: 'line',
        symbolSize: 10,
        symbol: 'circle',
        smooth: true,
        lineStyle: {
          width: 3,
          shadowColor: 'rgba(0,0,0,0.3)',
          shadowBlur: 10,
          shadowOffsetY: 8
        },
        data: [15, -50, -56.5, -46.5, -22.1, -2.5, -27.7, -55.7, -76.5]
      }
    ]
  };
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
