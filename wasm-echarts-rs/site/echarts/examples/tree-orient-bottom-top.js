/**
 * 官网示例：从下到上树状图
 * https://echarts.apache.org/examples/zh/editor.html?c=tree-orient-bottom-top
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: From Bottom to Top Tree
    category: tree
    titleCN: 从下到上树状图
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
              left: '2%',
              right: '2%',
              top: '20%',
              bottom: '8%',
              symbol: 'emptyCircle',
              orient: 'BT',
              expandAndCollapse: true,
              label: {
                position: 'bottom',
                rotate: 90,
                verticalAlign: 'middle',
                align: 'right'
              },
              leaves: {
                label: {
                  position: 'top',
                  rotate: 90,
                  verticalAlign: 'middle',
                  align: 'left'
                }
              },
              emphasis: {
                focus: 'descendant'
              },
              animationDurationUpdate: 750
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
