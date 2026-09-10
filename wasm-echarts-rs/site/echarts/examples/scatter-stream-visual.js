/**
 * 官网示例：流式渲染和视觉映射操作
 * https://echarts.apache.org/examples/zh/editor.html?c=scatter-stream-visual
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Visual interaction with stream
    category: scatter
    titleCN: 流式渲染和视觉映射操作
    difficulty: 5
    */
    // Thanks to: 若怀冰
    // http://gallery.echartsjs.com/explore.html?u=bd-16906679
    // http://gallery.echartsjs.com/editor.html?c=xHJw-hVqjW
    $.getJSON(
      ROOT_PATH + '/data/asset/data/house-price-area2.json',
      function (data) {
        option = {
          title: {
            text: 'Dispersion of house price based on the area',
            left: 'center',
            top: 0
          },
          visualMap: {
            min: 15202,
            max: 159980,
            dimension: 1,
            orient: 'vertical',
            right: 10,
            top: 'center',
            text: ['HIGH', 'LOW'],
            calculable: true,
            inRange: {
              color: ['#f2c31a', '#24b7f2']
            }
          },
          tooltip: {
            trigger: 'item',
            axisPointer: {
              type: 'cross'
            }
          },
          xAxis: [
            {
              type: 'value'
            }
          ],
          yAxis: [
            {
              type: 'value'
            }
          ],
          series: [
            {
              name: 'price-area',
              type: 'scatter',
              symbolSize: 5,
              data: data
            }
          ]
        };
        myChart.setOption(option);
      }
    );
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
