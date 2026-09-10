/**
 * 官网示例：利润分布直方图
 * https://echarts.apache.org/examples/zh/editor.html?c=custom-profit
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
  title: Profit
  category: custom
  titleCN: 利润分布直方图
  difficulty: 1
  */
  const colorList = [
    '#4f81bd',
    '#c0504d',
    '#9bbb59',
    '#604a7b',
    '#948a54',
    '#e46c0b'
  ];
  const data = [
    [10, 16, 3, 'A'],
    [16, 18, 15, 'B'],
    [18, 26, 12, 'C'],
    [26, 32, 22, 'D'],
    [32, 56, 7, 'E'],
    [56, 62, 17, 'F']
  ].map(function (item, index) {
    return {
      value: item,
      itemStyle: {
        color: colorList[index]
      }
    };
  });
  option = {
    title: {
      text: 'Profit',
      left: 'center'
    },
    tooltip: {},
    xAxis: {
      scale: true
    },
    yAxis: {},
    series: [
      {
        type: 'custom',
        renderItem: function (params, api) {
          var yValue = api.value(2);
          var start = api.coord([api.value(0), yValue]);
          var size = api.size([api.value(1) - api.value(0), yValue]);
          var style = api.style();
          return {
            type: 'rect',
            shape: {
              x: start[0],
              y: start[1],
              width: size[0],
              height: size[1]
            },
            style: style
          };
        },
        label: {
          show: true,
          position: 'top'
        },
        dimensions: ['from', 'to', 'profit'],
        encode: {
          x: [0, 1],
          y: 2,
          tooltip: [0, 1, 2],
          itemName: 3
        },
        data: data
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
