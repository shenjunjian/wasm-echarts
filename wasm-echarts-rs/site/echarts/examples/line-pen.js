/**
 * 官网折线示例：点击添加折线图拐点
 * https://echarts.apache.org/examples/zh/editor.html?c=line-pen
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Click to Add Points
    category: line
    titleCN: 点击添加折线图拐点
    difficulty: 9
    */
    const symbolSize = 20;
    const data = [
      [15, 0],
      [-50, 10],
      [-56.5, 20],
      [-46.5, 30],
      [-22.1, 40]
    ];
    option = {
      title: {
        text: 'Click to Add Points'
      },
      tooltip: {
        formatter: function (params) {
          var data = params.data || [0, 0];
          return data[0].toFixed(2) + ', ' + data[1].toFixed(2);
        }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: {
        min: -60,
        max: 20,
        type: 'value',
        axisLine: { onZero: false }
      },
      yAxis: {
        min: 0,
        max: 40,
        type: 'value',
        axisLine: { onZero: false }
      },
      series: [
        {
          id: 'a',
          type: 'line',
          smooth: true,
          symbolSize: symbolSize,
          data: data
        }
      ]
    };
    var zr = myChart.getZr();
    zr.on('click', function (params) {
      var pointInPixel = [params.offsetX, params.offsetY];
      var pointInGrid = myChart.convertFromPixel('grid', pointInPixel);
      if (myChart.containPixel('grid', pointInPixel)) {
        data.push(pointInGrid);
        myChart.setOption({
          series: [
            {
              id: 'a',
              data: data
            }
          ]
        });
      }
    });
    zr.on('mousemove', function (params) {
      var pointInPixel = [params.offsetX, params.offsetY];
      zr.setCursorStyle(
        myChart.containPixel('grid', pointInPixel) ? 'copy' : 'default'
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
