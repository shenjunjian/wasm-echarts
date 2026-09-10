/**
 * 官网示例：得分环
 * https://echarts.apache.org/examples/zh/editor.html?c=gauge-ring
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Ring Gauge
    titleCN: 得分环
    category: gauge
    difficulty: 5
    videoStart: 1000
    videoEnd: 8000
    */
    const gaugeData = [
      {
        value: 20,
        name: 'Perfect',
        title: {
          offsetCenter: ['0%', '-30%']
        },
        detail: {
          valueAnimation: true,
          offsetCenter: ['0%', '-20%']
        }
      },
      {
        value: 40,
        name: 'Good',
        title: {
          offsetCenter: ['0%', '0%']
        },
        detail: {
          valueAnimation: true,
          offsetCenter: ['0%', '10%']
        }
      },
      {
        value: 60,
        name: 'Commonly',
        title: {
          offsetCenter: ['0%', '30%']
        },
        detail: {
          valueAnimation: true,
          offsetCenter: ['0%', '40%']
        }
      }
    ];
    option = {
      series: [
        {
          type: 'gauge',
          startAngle: 90,
          endAngle: -270,
          pointer: {
            show: false
          },
          progress: {
            show: true,
            overlap: false,
            roundCap: true,
            clip: false,
            itemStyle: {
              borderWidth: 1,
              borderColor: '#464646'
            }
          },
          axisLine: {
            lineStyle: {
              width: 40
            }
          },
          splitLine: {
            show: false,
            distance: 0,
            length: 10
          },
          axisTick: {
            show: false
          },
          axisLabel: {
            show: false,
            distance: 50
          },
          data: gaugeData,
          title: {
            fontSize: 14
          },
          detail: {
            width: 50,
            height: 14,
            fontSize: 14,
            color: 'inherit',
            borderColor: 'inherit',
            borderRadius: 20,
            borderWidth: 1,
            formatter: '{value}%'
          }
        }
      ]
    };
    setInterval(function () {
      gaugeData[0].value = +(Math.random() * 100).toFixed(2);
      gaugeData[1].value = +(Math.random() * 100).toFixed(2);
      gaugeData[2].value = +(Math.random() * 100).toFixed(2);
      myChart.setOption({
        series: [
          {
            data: gaugeData,
            pointer: {
              show: false
            }
          }
        ]
      });
    }, 2000);
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
