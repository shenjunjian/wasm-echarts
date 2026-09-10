/**
 * 官网示例：桑基图层级自定义样式
 * https://echarts.apache.org/examples/zh/editor.html?c=sankey-levels
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Sankey with Levels Setting
    category: sankey
    titleCN: 桑基图层级自定义样式
    difficulty: 2
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/product.json', function (data) {
      myChart.hideLoading();
      myChart.setOption(
        (option = {
          title: {
            text: 'Sankey Diagram'
          },
          tooltip: {
            trigger: 'item',
            triggerOn: 'mousemove'
          },
          series: [
            {
              type: 'sankey',
              data: data.nodes,
              links: data.links,
              emphasis: {
                focus: 'adjacency'
              },
              levels: [
                {
                  depth: 0,
                  itemStyle: {
                    color: '#fbb4ae'
                  },
                  lineStyle: {
                    color: 'source',
                    opacity: 0.6
                  }
                },
                {
                  depth: 1,
                  itemStyle: {
                    color: '#b3cde3'
                  },
                  lineStyle: {
                    color: 'source',
                    opacity: 0.6
                  }
                },
                {
                  depth: 2,
                  itemStyle: {
                    color: '#ccebc5'
                  },
                  lineStyle: {
                    color: 'source',
                    opacity: 0.6
                  }
                },
                {
                  depth: 3,
                  itemStyle: {
                    color: '#decbe4'
                  },
                  lineStyle: {
                    color: 'source',
                    opacity: 0.6
                  }
                }
              ],
              lineStyle: {
                curveness: 0.5
              }
            }
          ]
        })
      );
    });
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
