/**
 * 官网示例：自定义多边形图
 * https://echarts.apache.org/examples/zh/editor.html?c=custom-cartesian-polygon
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Custom Cartesian Polygon
    titleCN: 自定义多边形图
    category: custom
    difficulty: 3
    */
    const data = [];
    const dataCount = 7;
    for (let i = 0; i < dataCount; i++) {
      data.push([
        echarts.number.round(Math.random() * 100),
        echarts.number.round(Math.random() * 400)
      ]);
    }
    option = {
      tooltip: {
        trigger: 'axis'
      },
      legend: {
        data: ['bar', 'error']
      },
      dataZoom: [
        {
          type: 'slider',
          filterMode: 'none'
        },
        {
          type: 'inside',
          filterMode: 'none'
        }
      ],
      xAxis: {},
      yAxis: {},
      series: [
        {
          type: 'custom',
          renderItem: function (params, api) {
            if (params.context.rendered) {
              return;
            }
            params.context.rendered = true;
            let points = [];
            for (let i = 0; i < data.length; i++) {
              points.push(api.coord(data[i]));
            }
            let color = api.visual('color');
            return {
              type: 'polygon',
              transition: ['shape'],
              shape: {
                points: points
              },
              style: api.style({
                fill: color,
                stroke: echarts.color.lift(color, 0.1)
              })
            };
          },
          clip: true,
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
