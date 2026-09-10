/**
 * 官网示例：饼图标签对齐
 * https://echarts.apache.org/examples/zh/editor.html?c=pie-alignTo
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Pie Label Align
    category: pie
    titleCN: 饼图标签对齐
    difficulty: 3
    */
    const data = [
      {
        name: 'Apples',
        value: 70
      },
      {
        name: 'Strawberries',
        value: 68
      },
      {
        name: 'Bananas',
        value: 48
      },
      {
        name: 'Oranges',
        value: 40
      },
      {
        name: 'Pears',
        value: 32
      },
      {
        name: 'Pineapples',
        value: 27
      },
      {
        name: 'Grapes',
        value: 18
      }
    ];
    option = {
      title: [
        {
          text: 'Pie label alignTo',
          left: 'center'
        },
        {
          subtext: 'alignTo: "none" (default)',
          left: '16.67%',
          top: '75%',
          textAlign: 'center'
        },
        {
          subtext: 'alignTo: "labelLine"',
          left: '50%',
          top: '75%',
          textAlign: 'center'
        },
        {
          subtext: 'alignTo: "edge"',
          left: '83.33%',
          top: '75%',
          textAlign: 'center'
        }
      ],
      series: [
        {
          type: 'pie',
          radius: '25%',
          center: ['50%', '50%'],
          data: data,
          label: {
            position: 'outer',
            alignTo: 'none',
            bleedMargin: 5
          },
          left: 0,
          right: '66.6667%',
          top: 0,
          bottom: 0
        },
        {
          type: 'pie',
          radius: '25%',
          center: ['50%', '50%'],
          data: data,
          label: {
            position: 'outer',
            alignTo: 'labelLine',
            bleedMargin: 5
          },
          left: '33.3333%',
          right: '33.3333%',
          top: 0,
          bottom: 0
        },
        {
          type: 'pie',
          radius: '25%',
          center: ['50%', '50%'],
          data: data,
          label: {
            position: 'outer',
            alignTo: 'edge',
            margin: 20
          },
          left: '66.6667%',
          right: 0,
          top: 0,
          bottom: 0
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
