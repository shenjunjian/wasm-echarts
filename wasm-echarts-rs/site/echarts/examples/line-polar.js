/**
 * 官网折线示例：极坐标双数值轴
 * https://echarts.apache.org/examples/zh/editor.html?c=line-polar
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Two Value-Axes in Polar
    category: line
    titleCN: 极坐标双数值轴
    difficulty: 10
    */
    const data = [];
    for (let i = 0; i <= 100; i++) {
      let theta = (i / 100) * 360;
      let r = 5 * (1 + Math.sin((theta / 180) * Math.PI));
      data.push([r, theta]);
    }
    option = {
      title: {
        text: 'Two Value-Axes in Polar'
      },
      legend: {
        data: ['line']
      },
      polar: {},
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'cross'
        }
      },
      angleAxis: {
        type: 'value',
        startAngle: 0
      },
      radiusAxis: {},
      series: [
        {
          coordinateSystem: 'polar',
          name: 'line',
          type: 'line',
          data: data
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
