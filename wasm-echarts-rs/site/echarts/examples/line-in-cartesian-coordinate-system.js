/**
 * 官网折线示例：双数值轴折线图
 * https://echarts.apache.org/examples/zh/editor.html?c=line-in-cartesian-coordinate-system
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Line Chart in Cartesian Coordinate System
    category: line
    titleCN: 双数值轴折线图
    difficulty: 7
    */
    option = {
      xAxis: {},
      yAxis: {},
      series: [
        {
          data: [
            [10, 40],
            [50, 100],
            [40, 20]
          ],
          type: 'line'
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
