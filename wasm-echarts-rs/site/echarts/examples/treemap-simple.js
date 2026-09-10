/**
 * 官网示例：基础矩形树图
 * https://echarts.apache.org/examples/zh/editor.html?c=treemap-simple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Basic Treemap
    category: treemap
    titleCN: 基础矩形树图
    */
    option = {
      series: [
        {
          type: 'treemap',
          data: [
            {
              name: 'nodeA',
              value: 10,
              children: [
                {
                  name: 'nodeAa',
                  value: 4
                },
                {
                  name: 'nodeAb',
                  value: 6
                }
              ]
            },
            {
              name: 'nodeB',
              value: 20,
              children: [
                {
                  name: 'nodeBa',
                  value: 20,
                  children: [
                    {
                      name: 'nodeBa1',
                      value: 20
                    }
                  ]
                }
              ]
            }
          ]
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
