/**
 * 官网示例：自定义雷达图
 * https://echarts.apache.org/examples/zh/editor.html?c=radar-custom
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
  title: Customized Radar Chart
  category: radar
  titleCN: 自定义雷达图
  difficulty: 2
  */
  option = {
    color: ['#67F9D8', '#FFE434', '#56A3F1', '#FF917C'],
    title: {
      text: 'Customized Radar Chart'
    },
    legend: {},
    radar: [
      {
        indicator: [
          { text: 'Indicator1' },
          { text: 'Indicator2' },
          { text: 'Indicator3' },
          { text: 'Indicator4' },
          { text: 'Indicator5' }
        ],
        center: ['25%', '50%'],
        radius: 120,
        startAngle: 90,
        splitNumber: 4,
        shape: 'circle',
        axisName: {
          formatter: '【{value}】',
          color: '#428BD4'
        },
        splitArea: {
          areaStyle: {
            color: ['#77EADF', '#26C3BE', '#64AFE9', '#428BD4'],
            shadowColor: 'rgba(0, 0, 0, 0.2)',
            shadowBlur: 10
          }
        },
        axisLine: {
          lineStyle: {
            color: 'rgba(211, 253, 250, 0.8)'
          }
        },
        splitLine: {
          lineStyle: {
            color: 'rgba(211, 253, 250, 0.8)'
          }
        }
      },
      {
        indicator: [
          { text: 'Indicator1', max: 150 },
          { text: 'Indicator2', max: 150 },
          { text: 'Indicator3', max: 150 },
          { text: 'Indicator4', max: 120 },
          { text: 'Indicator5', max: 108 },
          { text: 'Indicator6', max: 72 }
        ],
        center: ['75%', '50%'],
        radius: 120,
        axisName: {
          color: '#fff',
          backgroundColor: '#666',
          borderRadius: 3,
          padding: [3, 5]
        }
      }
    ],
    series: [
      {
        type: 'radar',
        emphasis: {
          lineStyle: {
            width: 4
          }
        },
        data: [
          {
            value: [100, 8, 0.4, -80, 2000],
            name: 'Data A'
          },
          {
            value: [60, 5, 0.3, -100, 1500],
            name: 'Data B',
            areaStyle: {
              color: 'rgba(255, 228, 52, 0.6)'
            }
          }
        ]
      },
      {
        type: 'radar',
        radarIndex: 1,
        data: [
          {
            value: [120, 118, 130, 100, 99, 70],
            name: 'Data C',
            symbol: 'rect',
            symbolSize: 12,
            lineStyle: {
              type: 'dashed'
            },
            label: {
              show: true,
              formatter: function (params) {
                return params.value;
              }
            }
          },
          {
            value: [100, 93, 50, 90, 70, 60],
            name: 'Data D',
            areaStyle: {
              color: new echarts.graphic.RadialGradient(0.1, 0.6, 1, [
                {
                  color: 'rgba(255, 145, 124, 0.1)',
                  offset: 0
                },
                {
                  color: 'rgba(255, 145, 124, 0.9)',
                  offset: 1
                }
              ])
            }
          }
        ]
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
