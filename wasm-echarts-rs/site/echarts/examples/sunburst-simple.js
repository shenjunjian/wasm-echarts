/**
 * 官网示例：基础旭日图
 * https://echarts.apache.org/examples/zh/editor.html?c=sunburst-simple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Basic Sunburst
    category: sunburst
    titleCN: 基础旭日图
    difficulty: 1
    */
    var data = [
      {
        name: 'Grandpa',
        children: [
          {
            name: 'Uncle Leo',
            value: 15,
            children: [
              {
                name: 'Cousin Jack',
                value: 2
              },
              {
                name: 'Cousin Mary',
                value: 5,
                children: [
                  {
                    name: 'Jackson',
                    value: 2
                  }
                ]
              },
              {
                name: 'Cousin Ben',
                value: 4
              }
            ]
          },
          {
            name: 'Father',
            value: 10,
            children: [
              {
                name: 'Me',
                value: 5
              },
              {
                name: 'Brother Peter',
                value: 1
              }
            ]
          }
        ]
      },
      {
        name: 'Nancy',
        children: [
          {
            name: 'Uncle Nike',
            children: [
              {
                name: 'Cousin Betty',
                value: 1
              },
              {
                name: 'Cousin Jenny',
                value: 2
              }
            ]
          }
        ]
      }
    ];
    option = {
      series: {
        type: 'sunburst',
        // emphasis: {
        //     focus: 'ancestor'
        // },
        data: data,
        radius: [0, '90%'],
        label: {
          rotate: 'radial'
        }
      }
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
