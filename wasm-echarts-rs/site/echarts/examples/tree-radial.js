/**
 * 官网示例：径向树状图
 * https://echarts.apache.org/examples/zh/editor.html?c=tree-radial
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Radial Tree
    category: tree
    titleCN: 径向树状图
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/flare.json', function (data) {
      myChart.hideLoading();
      myChart.setOption(
        (option = {
          tooltip: {
            trigger: 'item',
            triggerOn: 'mousemove'
          },
          series: [
            {
              type: 'tree',
              data: [data],
              top: '18%',
              bottom: '14%',
              layout: 'radial',
              symbol: 'emptyCircle',
              symbolSize: 7,
              initialTreeDepth: 3,
              animationDurationUpdate: 750,
              emphasis: {
                focus: 'descendant'
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
