/**
 * 官网示例：多系列盒须图
 * https://echarts.apache.org/examples/zh/editor.html?c=boxplot-multi
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Multiple Categories
    category: boxplot
    titleCN: 多系列盒须图
    */
    // Generate data.
    function makeData() {
      let data = [];
      for (let i = 0; i < 18; i++) {
        let cate = [];
        for (let j = 0; j < 100; j++) {
          cate.push(Math.random() * 200);
        }
        data.push(cate);
      }
      return data;
    }
    const data0 = makeData();
    const data1 = makeData();
    const data2 = makeData();
    option = {
      title: {
        text: 'Multiple Categories',
        left: 'center'
      },
      dataset: [
        {
          source: data0
        },
        {
          source: data1
        },
        {
          source: data2
        },
        {
          fromDatasetIndex: 0,
          transform: { type: 'boxplot' }
        },
        {
          fromDatasetIndex: 1,
          transform: { type: 'boxplot' }
        },
        {
          fromDatasetIndex: 2,
          transform: { type: 'boxplot' }
        }
      ],
      legend: {
        top: '10%'
      },
      tooltip: {
        trigger: 'item',
        axisPointer: {
          type: 'shadow'
        }
      },
      grid: {
        left: '10%',
        top: '20%',
        right: '10%',
        bottom: '15%'
      },
      xAxis: {
        type: 'category',
        boundaryGap: true,
        nameGap: 30,
        splitArea: {
          show: true
        },
        splitLine: {
          show: false
        }
      },
      yAxis: {
        type: 'value',
        name: 'Value',
        min: -400,
        max: 600,
        splitArea: {
          show: false
        }
      },
      dataZoom: [
        {
          type: 'inside',
          start: 0,
          end: 20
        },
        {
          show: true,
          type: 'slider',
          top: '90%',
          xAxisIndex: [0],
          start: 0,
          end: 20
        }
      ],
      series: [
        {
          name: 'category0',
          type: 'boxplot',
          datasetIndex: 3
        },
        {
          name: 'category1',
          type: 'boxplot',
          datasetIndex: 4
        },
        {
          name: 'category2',
          type: 'boxplot',
          datasetIndex: 5
        }
      ]
    };
    return option;
  } catch (error) {
    if (option) {
      try {
        myChart.setOption(option);
      } catch {
        // 保留原始错误
      }
    }
    throw error;
  }
});
