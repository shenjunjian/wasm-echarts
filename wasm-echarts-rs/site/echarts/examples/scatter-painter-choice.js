/**
 * 官网示例：历代绘画大师的色彩运用
 * https://echarts.apache.org/examples/zh/editor.html?c=scatter-painter-choice
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
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
