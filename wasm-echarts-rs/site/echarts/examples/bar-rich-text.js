/**
 * 官网示例：天气统计（富文本）
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-rich-text
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
  title: Weather Statistics
  category: 'bar, rich'
  titleCN: 天气统计（富文本）
  difficulty: 6
  */
  const weatherIcons = {
    Sunny: ROOT_PATH + '/data/asset/img/weather/sunny_128.png',
    Cloudy: ROOT_PATH + '/data/asset/img/weather/cloudy_128.png',
    Showers: ROOT_PATH + '/data/asset/img/weather/showers_128.png'
  };
  const seriesLabel = {
    show: true
  };
  option = {
    title: {
      text: 'Weather Statistics'
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'shadow'
      }
    },
    legend: {
      data: ['City Alpha', 'City Beta', 'City Gamma']
    },
    grid: {
      left: 100
    },
    toolbox: {
      show: true,
      feature: {
        saveAsImage: {}
      }
    },
    xAxis: {
      type: 'value',
      name: 'Days',
      axisLabel: {
        formatter: '{value}'
      }
    },
    yAxis: {
      type: 'category',
      inverse: true,
      data: ['Sunny', 'Cloudy', 'Showers'],
      axisLabel: {
        formatter: function (value) {
          return '{' + value + '| }\n{value|' + value + '}';
        },
        margin: 20,
        rich: {
          value: {
            lineHeight: 30,
            align: 'center'
          },
          Sunny: {
            height: 40,
            align: 'center',
            backgroundColor: {
              image: weatherIcons.Sunny
            }
          },
          Cloudy: {
            height: 40,
            align: 'center',
            backgroundColor: {
              image: weatherIcons.Cloudy
            }
          },
          Showers: {
            height: 40,
            align: 'center',
            backgroundColor: {
              image: weatherIcons.Showers
            }
          }
        }
      }
    },
    series: [
      {
        name: 'City Alpha',
        type: 'bar',
        data: [165, 170, 30],
        label: seriesLabel,
        markPoint: {
          symbolSize: 1,
          symbolOffset: [0, '50%'],
          label: {
            formatter: '{a|{a}\n}{b|{b} }{c|{c}}',
            backgroundColor: 'rgb(242,242,242)',
            borderColor: '#aaa',
            borderWidth: 1,
            borderRadius: 4,
            padding: [4, 10],
            lineHeight: 26,
            // shadowBlur: 5,
            // shadowColor: '#000',
            // shadowOffsetX: 0,
            // shadowOffsetY: 1,
            position: 'right',
            distance: 20,
            rich: {
              a: {
                align: 'center',
                color: '#fff',
                fontSize: 18,
                textShadowBlur: 2,
                textShadowColor: '#000',
                textShadowOffsetX: 0,
                textShadowOffsetY: 1,
                textBorderColor: '#333',
                textBorderWidth: 2
              },
              b: {
                color: '#333'
              },
              c: {
                color: '#ff8811',
                textBorderColor: '#000',
                textBorderWidth: 1,
                fontSize: 22
              }
            }
          },
          data: [
            { type: 'max', name: 'max days: ' },
            { type: 'min', name: 'min days: ' }
          ]
        }
      },
      {
        name: 'City Beta',
        type: 'bar',
        label: seriesLabel,
        data: [150, 105, 110]
      },
      {
        name: 'City Gamma',
        type: 'bar',
        label: seriesLabel,
        data: [220, 82, 63]
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
