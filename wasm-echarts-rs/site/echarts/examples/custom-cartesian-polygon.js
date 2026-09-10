/**
 * 官网示例：自定义多边形图
 * https://echarts.apache.org/examples/zh/editor.html?c=custom-cartesian-polygon
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import initWasm, * as echarts from '@wasm-echarts';
import { ROOT_PATH, CDN_PATH, $, app, sizeCanvas, showPreviewError } from '../../src/echarts/official-env.js';
import { ensureDefaultFont } from '../../src/echarts/fonts.js';

async function main() {
  await initWasm();
  await ensureDefaultFont();

  const canvas = document.getElementById('canvas');
  if (!canvas) {
    throw new Error('缺少 #canvas');
  }
  sizeCanvas(canvas);
  const myChart = echarts.init(canvas);
  window.addEventListener('resize', () => {
    if (myChart.isDisposed()) return;
    sizeCanvas(canvas);
    myChart.resize();
  });

  let option;
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
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
