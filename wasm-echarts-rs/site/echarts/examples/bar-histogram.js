/**
 * 官网示例：直方图（自定义系列）
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-histogram
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
  title: Histogram with Custom Series
  category: custom
  titleCN: 直方图（自定义系列）
  difficulty: 0
  */
  // See https://github.com/ecomfe/echarts-stat
  echarts.registerTransform(ecStat.transform.histogram);
  option = {
    dataset: [
      {
        source: [
          [8.3, 143],
          [8.6, 214],
          [8.8, 251],
          [10.5, 26],
          [10.7, 86],
          [10.8, 93],
          [11.0, 176],
          [11.0, 39],
          [11.1, 221],
          [11.2, 188],
          [11.3, 57],
          [11.4, 91],
          [11.4, 191],
          [11.7, 8],
          [12.0, 196],
          [12.9, 177],
          [12.9, 153],
          [13.3, 201],
          [13.7, 199],
          [13.8, 47],
          [14.0, 81],
          [14.2, 98],
          [14.5, 121],
          [16.0, 37],
          [16.3, 12],
          [17.3, 105],
          [17.5, 168],
          [17.9, 84],
          [18.0, 197],
          [18.0, 155],
          [20.6, 125]
        ]
      },
      {
        transform: {
          type: 'ecStat:histogram',
          config: {}
        }
      },
      {
        transform: {
          type: 'ecStat:histogram',
          // print: true,
          config: { dimensions: [1] }
        }
      }
    ],
    tooltip: {},
    grid: [
      {
        top: '50%',
        right: '50%'
      },
      {
        bottom: '52%',
        right: '50%'
      },
      {
        top: '50%',
        left: '52%'
      }
    ],
    xAxis: [
      {
        scale: true,
        gridIndex: 0
      },
      {
        type: 'category',
        scale: true,
        axisTick: { show: false },
        axisLabel: { show: false },
        axisLine: { show: false },
        gridIndex: 1
      },
      {
        scale: true,
        gridIndex: 2
      }
    ],
    yAxis: [
      {
        gridIndex: 0
      },
      {
        gridIndex: 1
      },
      {
        type: 'category',
        axisTick: { show: false },
        axisLabel: { show: false },
        axisLine: { show: false },
        gridIndex: 2
      }
    ],
    series: [
      {
        name: 'origianl scatter',
        type: 'scatter',
        xAxisIndex: 0,
        yAxisIndex: 0,
        encode: { tooltip: [0, 1] },
        datasetIndex: 0
      },
      {
        name: 'histogram',
        type: 'bar',
        xAxisIndex: 1,
        yAxisIndex: 1,
        barWidth: '99.3%',
        label: {
          show: true,
          position: 'top'
        },
        encode: { x: 0, y: 1, itemName: 4 },
        datasetIndex: 1
      },
      {
        name: 'histogram',
        type: 'bar',
        xAxisIndex: 2,
        yAxisIndex: 2,
        barWidth: '99.3%',
        label: {
          show: true,
          position: 'right'
        },
        encode: { x: 1, y: 0, itemName: 4 },
        datasetIndex: 2
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
