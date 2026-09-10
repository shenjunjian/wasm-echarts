/**
 * 官网示例：历代绘画大师的色彩运用
 * https://echarts.apache.org/examples/zh/editor.html?c=scatter-painter-choice
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Master Painter Color Choices Throughout History
    category: scatter
    titleCN: 历代绘画大师的色彩运用
    difficulty: 9
    */
    myChart.showLoading();
    $.get(
      ROOT_PATH + '/data/asset/data/masterPainterColorChoice.json',
      function (json) {
        myChart.hideLoading();
        var data = json[0].x.map(function (x, idx) {
          return [+x, +json[0].y[idx]];
        });
        myChart.setOption(
          (option = {
            title: {
              text: 'Master Painter Color Choices Throughout History',
              subtext: 'Data From Plot.ly',
              left: 'right'
            },
            xAxis: {
              type: 'value',
              splitLine: {
                show: false
              },
              scale: true,
              splitNumber: 5,
              max: 'dataMax',
              axisLabel: {
                formatter: function (val) {
                  return val + 's';
                }
              }
            },
            yAxis: {
              type: 'value',
              min: 0,
              max: 360,
              interval: 60,
              name: 'Hue',
              splitLine: {
                show: false
              }
            },
            series: [
              {
                name: 'scatter',
                type: 'scatter',
                symbolSize: function (val, param) {
                  return (
                    json[0].marker.size[param.dataIndex] / json[0].marker.sizeref
                  );
                },
                itemStyle: {
                  color: function (param) {
                    return json[0].marker.color[param.dataIndex];
                  }
                },
                data: data
              }
            ]
          })
        );
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
