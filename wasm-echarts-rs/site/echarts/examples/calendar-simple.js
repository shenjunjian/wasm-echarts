/**
 * 官网示例：基础日历图
 * https://echarts.apache.org/examples/zh/editor.html?c=calendar-simple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Simple Calendar
    titleCN: 基础日历图
    category: calendar
    difficulty: 0
    */
    function getVirtualData(year) {
      const date = +echarts.time.parse(year + '-01-01');
      const end = +echarts.time.parse(year + '-12-31');
      const dayTime = 3600 * 24 * 1000;
      const data = [];
      for (let time = date; time <= end; time += dayTime) {
        data.push([
          echarts.time.format(time, '{yyyy}-{MM}-{dd}', false),
          Math.floor(Math.random() * 10000)
        ]);
      }
      return data;
    }
    option = {
      visualMap: {
        show: false,
        min: 0,
        max: 10000
      },
      calendar: {
        range: '2017'
      },
      series: {
        type: 'heatmap',
        coordinateSystem: 'calendar',
        data: getVirtualData('2017')
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
