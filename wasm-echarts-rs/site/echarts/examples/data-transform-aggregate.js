/**
 * 官网示例：简单的数据聚合
 * https://echarts.apache.org/examples/zh/editor.html?c=data-transform-aggregate
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
  title: Data Transform Simple Aggregate
  category: boxplot
  titleCN: 简单的数据聚合
  difficulty: 4
  */
  $.when(
    $.get(ROOT_PATH + '/data/asset/data/life-expectancy-table.json'),
    $.getScript(
      CDN_PATH + 'echarts-simple-transform/dist/ecSimpleTransform.min.js'
    )
  ).done(function (res) {
    run(res[0]);
  });
  function run(_rawData) {
    echarts.registerTransform(ecSimpleTransform.aggregate);
    option = {
      dataset: [
        {
          id: 'raw',
          source: _rawData
        },
        {
          id: 'since_year',
          fromDatasetId: 'raw',
          transform: [
            {
              type: 'filter',
              config: {
                dimension: 'Year',
                gte: 1950
              }
            }
          ]
        },
        {
          id: 'income_aggregate',
          fromDatasetId: 'since_year',
          transform: [
            {
              type: 'ecSimpleTransform:aggregate',
              config: {
                resultDimensions: [
                  { name: 'min', from: 'Income', method: 'min' },
                  { name: 'Q1', from: 'Income', method: 'Q1' },
                  { name: 'median', from: 'Income', method: 'median' },
                  { name: 'Q3', from: 'Income', method: 'Q3' },
                  { name: 'max', from: 'Income', method: 'max' },
                  { name: 'Country', from: 'Country' }
                ],
                groupBy: 'Country'
              }
            },
            {
              type: 'sort',
              config: {
                dimension: 'Q3',
                order: 'asc'
              }
            }
          ]
        }
      ],
      title: {
        text: 'Income since 1950'
      },
      tooltip: {
        trigger: 'axis',
        confine: true
      },
      xAxis: {
        name: 'Income',
        nameLocation: 'middle',
        nameGap: 30,
        scale: true
      },
      yAxis: {
        type: 'category'
      },
      grid: {
        bottom: 140
      },
      legend: {
        selected: { detail: false }
      },
      dataZoom: [
        {
          type: 'inside'
        },
        {
          type: 'slider',
          height: 20,
          bottom: 60
        }
      ],
      series: [
        {
          name: 'boxplot',
          type: 'boxplot',
          datasetId: 'income_aggregate',
          itemStyle: {
            color: '#b8c5f2'
          },
          encode: {
            x: ['min', 'Q1', 'median', 'Q3', 'max'],
            y: 'Country',
            itemName: ['Country'],
            tooltip: ['min', 'Q1', 'median', 'Q3', 'max']
          }
        },
        {
          name: 'detail',
          type: 'scatter',
          datasetId: 'since_year',
          symbolSize: 6,
          tooltip: {
            trigger: 'item'
          },
          label: {
            show: true,
            position: 'top',
            align: 'left',
            verticalAlign: 'middle',
            rotate: 90,
            fontSize: 12
          },
          itemStyle: {
            color: '#d00000'
          },
          encode: {
            x: 'Income',
            y: 'Country',
            label: 'Year',
            itemName: 'Year',
            tooltip: ['Country', 'Year', 'Income']
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
