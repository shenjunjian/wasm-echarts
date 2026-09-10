/**
 * 官网示例：数据过滤
 * https://echarts.apache.org/examples/zh/editor.html?c=data-transform-filter
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
  title: Data Transform Filter
  category: line
  titleCN: 数据过滤
  difficulty: 3
  */
  $.get(
    ROOT_PATH + '/data/asset/data/life-expectancy-table.json',
    function (_rawData) {
      run(_rawData);
    }
  );
  function run(_rawData) {
    option = {
      dataset: [
        {
          id: 'dataset_raw',
          source: _rawData
        },
        {
          id: 'dataset_since_1950_of_germany',
          fromDatasetId: 'dataset_raw',
          transform: {
            type: 'filter',
            config: {
              and: [
                { dimension: 'Year', gte: 1950 },
                { dimension: 'Country', '=': 'Germany' }
              ]
            }
          }
        },
        {
          id: 'dataset_since_1950_of_france',
          fromDatasetId: 'dataset_raw',
          transform: {
            type: 'filter',
            config: {
              and: [
                { dimension: 'Year', gte: 1950 },
                { dimension: 'Country', '=': 'France' }
              ]
            }
          }
        }
      ],
      title: {
        text: 'Income of Germany and France since 1950'
      },
      tooltip: {
        trigger: 'axis'
      },
      xAxis: {
        type: 'category',
        nameLocation: 'middle'
      },
      yAxis: {
        name: 'Income'
      },
      series: [
        {
          type: 'line',
          datasetId: 'dataset_since_1950_of_germany',
          showSymbol: false,
          encode: {
            x: 'Year',
            y: 'Income',
            itemName: 'Year',
            tooltip: ['Income']
          }
        },
        {
          type: 'line',
          datasetId: 'dataset_since_1950_of_france',
          showSymbol: false,
          encode: {
            x: 'Year',
            y: 'Income',
            itemName: 'Year',
            tooltip: ['Income']
          }
        }
      ]
    };
    myChart.setOption(option);
  }
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
