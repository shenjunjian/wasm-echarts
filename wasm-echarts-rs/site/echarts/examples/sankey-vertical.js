/**
 * 官网示例：垂直方向的桑基图
 * https://echarts.apache.org/examples/zh/editor.html?c=sankey-vertical
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Sankey Orient Vertical
    category: sankey
    titleCN: 垂直方向的桑基图
    difficulty: 1
    */
    option = {
      tooltip: {
        trigger: 'item',
        triggerOn: 'mousemove'
      },
      animation: false,
      series: [
        {
          type: 'sankey',
          bottom: '10%',
          emphasis: {
            focus: 'adjacency'
          },
          data: [
            { name: 'a' },
            { name: 'b' },
            { name: 'a1' },
            { name: 'b1' },
            { name: 'c' },
            { name: 'e' }
          ],
          links: [
            { source: 'a', target: 'a1', value: 5 },
            { source: 'e', target: 'b', value: 3 },
            { source: 'a', target: 'b1', value: 3 },
            { source: 'b1', target: 'a1', value: 1 },
            { source: 'b1', target: 'c', value: 2 },
            { source: 'b', target: 'c', value: 1 }
          ],
          orient: 'vertical',
          label: {
            position: 'top'
          },
          lineStyle: {
            color: 'source',
            curveness: 0.5
          }
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
