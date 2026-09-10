/**
 * 官网示例：置信带
 * https://echarts.apache.org/examples/zh/editor.html?c=confidence-band
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
  title: Confidence Band
  category: line
  titleCN: 置信带
  difficulty: 4
  */
  myChart.showLoading();
  $.get(ROOT_PATH + '/data/asset/data/confidence-band.json', function (data) {
    myChart.hideLoading();
    var base = -data.reduce(function (min, val) {
      return Math.floor(Math.min(min, val.l));
    }, Infinity);
    myChart.setOption(
      (option = {
        title: {
          text: 'Confidence Band',
          subtext: 'Example in MetricsGraphics.js',
          left: 'center'
        },
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            type: 'cross',
            animation: false,
            label: {
              backgroundColor: '#ccc',
              borderColor: '#aaa',
              borderWidth: 1,
              shadowBlur: 0,
              shadowOffsetX: 0,
              shadowOffsetY: 0,
              color: '#222'
            }
          },
          formatter: function (params) {
            return (
              params[2].name +
              '<br />' +
              ((params[2].value - base) * 100).toFixed(1) +
              '%'
            );
          }
        },
        grid: {
          left: '3%',
          right: '4%',
          bottom: '3%',
          containLabel: true
        },
        xAxis: {
          type: 'category',
          data: data.map(function (item) {
            return item.date;
          }),
          axisLabel: {
            formatter: function (value, idx) {
              var date = new Date(value);
              return idx === 0
                ? value
                : [date.getMonth() + 1, date.getDate()].join('-');
            }
          },
          boundaryGap: false
        },
        yAxis: {
          axisLabel: {
            formatter: function (val) {
              return (val - base) * 100 + '%';
            }
          },
          axisPointer: {
            label: {
              formatter: function (params) {
                return ((params.value - base) * 100).toFixed(1) + '%';
              }
            }
          },
          splitNumber: 3
        },
        series: [
          {
            name: 'L',
            type: 'line',
            data: data.map(function (item) {
              return item.l + base;
            }),
            lineStyle: {
              opacity: 0
            },
            stack: 'confidence-band',
            symbol: 'none'
          },
          {
            name: 'U',
            type: 'line',
            data: data.map(function (item) {
              return item.u - item.l;
            }),
            lineStyle: {
              opacity: 0
            },
            areaStyle: {
              color: '#ccc'
            },
            stack: 'confidence-band',
            symbol: 'none'
          },
          {
            type: 'line',
            data: data.map(function (item) {
              return item.value + base;
            }),
            itemStyle: {
              color: '#333'
            },
            showSymbol: false
          }
        ]
      })
    );
  });
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
