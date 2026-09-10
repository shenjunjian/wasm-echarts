/**
 * 官网示例：多 Y 轴示例
 * https://echarts.apache.org/examples/zh/editor.html?c=multiple-y-axis
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
  title: Multiple Y Axes
  category: bar
  titleCN: 多 Y 轴示例
  difficulty: 4
  */
  const colors = ['#5070dd', '#b6d634', '#505372'];
  option = {
    color: colors,
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    grid: {
      right: '20%'
    },
    toolbox: {
      feature: {
        dataView: { show: true, readOnly: false },
        restore: { show: true },
        saveAsImage: { show: true }
      }
    },
    legend: {
      data: ['Evaporation', 'Precipitation', 'Temperature']
    },
    xAxis: [
      {
        type: 'category',
        axisTick: {
          alignWithLabel: true
        },
        // prettier-ignore
        data: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
      }
    ],
    yAxis: [
      {
        type: 'value',
        name: 'Evaporation',
        position: 'right',
        alignTicks: true,
        axisLine: {
          show: true,
          lineStyle: {
            color: colors[0]
          }
        },
        axisLabel: {
          formatter: '{value} ml'
        }
      },
      {
        type: 'value',
        name: 'Precipitation',
        position: 'right',
        alignTicks: true,
        offset: 80,
        axisLine: {
          show: true,
          lineStyle: {
            color: colors[1]
          }
        },
        axisLabel: {
          formatter: '{value} ml'
        }
      },
      {
        type: 'value',
        name: '温度',
        position: 'left',
        alignTicks: true,
        axisLine: {
          show: true,
          lineStyle: {
            color: colors[2]
          }
        },
        axisLabel: {
          formatter: '{value} °C'
        }
      }
    ],
    series: [
      {
        name: 'Evaporation',
        type: 'bar',
        data: [
          2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 135.6, 162.2, 32.6, 20.0, 6.4, 3.3
        ]
      },
      {
        name: 'Precipitation',
        type: 'bar',
        yAxisIndex: 1,
        data: [
          2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 175.6, 182.2, 48.7, 18.8, 6.0, 2.3
        ]
      },
      {
        name: 'Temperature',
        type: 'line',
        yAxisIndex: 2,
        data: [2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3, 23.4, 23.0, 16.5, 12.0, 6.2]
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
