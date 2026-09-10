/**
 * 官网示例：指定数据到坐标轴的映射
 * https://echarts.apache.org/examples/zh/editor.html?c=dataset-encode1
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
  title: Encode and Matrix
  category: dataset
  titleCN: 指定数据到坐标轴的映射
  difficulty: 3
  */
  $.get(
    ROOT_PATH + '/data/asset/data/life-expectancy-table.json',
    function (data) {
      var sizeValue = '57%';
      var symbolSize = 2.5;
      option = {
        legend: {},
        tooltip: {},
        toolbox: {
          left: 'center',
          feature: {
            dataZoom: {}
          }
        },
        grid: [
          { right: sizeValue, bottom: sizeValue },
          { left: sizeValue, bottom: sizeValue },
          { right: sizeValue, top: sizeValue },
          { left: sizeValue, top: sizeValue }
        ],
        xAxis: [
          {
            type: 'value',
            gridIndex: 0,
            name: 'Income',
            axisLabel: { rotate: 50, interval: 0 }
          },
          {
            type: 'category',
            gridIndex: 1,
            name: 'Country',
            boundaryGap: false,
            axisLabel: { rotate: 50, interval: 0 }
          },
          {
            type: 'value',
            gridIndex: 2,
            name: 'Income',
            axisLabel: { rotate: 50, interval: 0 }
          },
          {
            type: 'value',
            gridIndex: 3,
            name: 'Life Expectancy',
            axisLabel: { rotate: 50, interval: 0 }
          }
        ],
        yAxis: [
          { type: 'value', gridIndex: 0, name: 'Life Expectancy' },
          { type: 'value', gridIndex: 1, name: 'Income' },
          { type: 'value', gridIndex: 2, name: 'Population' },
          { type: 'value', gridIndex: 3, name: 'Population' }
        ],
        dataset: {
          dimensions: [
            'Income',
            'Life Expectancy',
            'Population',
            'Country',
            { name: 'Year', type: 'ordinal' }
          ],
          source: data
        },
        series: [
          {
            type: 'scatter',
            symbolSize: symbolSize,
            xAxisIndex: 0,
            yAxisIndex: 0,
            encode: {
              x: 'Income',
              y: 'Life Expectancy',
              tooltip: [0, 1, 2, 3, 4]
            }
          },
          {
            type: 'scatter',
            symbolSize: symbolSize,
            xAxisIndex: 1,
            yAxisIndex: 1,
            encode: {
              x: 'Country',
              y: 'Income',
              tooltip: [0, 1, 2, 3, 4]
            }
          },
          {
            type: 'scatter',
            symbolSize: symbolSize,
            xAxisIndex: 2,
            yAxisIndex: 2,
            encode: {
              x: 'Income',
              y: 'Population',
              tooltip: [0, 1, 2, 3, 4]
            }
          },
          {
            type: 'scatter',
            symbolSize: symbolSize,
            xAxisIndex: 3,
            yAxisIndex: 3,
            encode: {
              x: 'Life Expectancy',
              y: 'Population',
              tooltip: [0, 1, 2, 3, 4]
            }
          }
        ]
      };
      myChart.setOption(option);
    }
  );
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
