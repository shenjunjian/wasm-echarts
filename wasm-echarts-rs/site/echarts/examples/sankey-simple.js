/**
 * 官网示例：基础桑基图
 * https://echarts.apache.org/examples/zh/editor.html?c=sankey-simple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Basic Sankey
    category: sankey
    titleCN: 基础桑基图
    difficulty: 0
    */
    option = {
      series: {
        type: 'sankey',
        layout: 'none',
        emphasis: {
          focus: 'adjacency'
        },
        data: [
          {
            name: 'a'
          },
          {
            name: 'b'
          },
          {
            name: 'a1'
          },
          {
            name: 'a2'
          },
          {
            name: 'b1'
          },
          {
            name: 'c'
          }
        ],
        links: [
          {
            source: 'a',
            target: 'a1',
            value: 5
          },
          {
            source: 'a',
            target: 'a2',
            value: 3
          },
          {
            source: 'b',
            target: 'b1',
            value: 8
          },
          {
            source: 'a',
            target: 'b1',
            value: 3
          },
          {
            source: 'b1',
            target: 'a1',
            value: 1
          },
          {
            source: 'b1',
            target: 'c',
            value: 2
          }
        ]
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
