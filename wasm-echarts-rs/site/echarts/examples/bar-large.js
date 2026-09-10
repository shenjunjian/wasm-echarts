/**
 * 官网示例：大数据量柱图
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-large
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Large Scale Bar Chart
    category: bar
    titleCN: 大数据量柱图
    difficulty: 5
    */
    const dataCount = 5e5;
    const data = generateData(dataCount);
    option = {
      title: {
        text: echarts.format.addCommas(dataCount) + ' Data',
        left: 10
      },
      toolbox: {
        feature: {
          dataZoom: {
            yAxisIndex: false
          },
          saveAsImage: {
            pixelRatio: 2
          }
        }
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'shadow'
        }
      },
      grid: {
        bottom: 90
      },
      dataZoom: [
        {
          type: 'inside'
        },
        {
          type: 'slider'
        }
      ],
      xAxis: {
        data: data.categoryData,
        silent: false,
        splitLine: {
          show: false
        },
        splitArea: {
          show: false
        }
      },
      yAxis: {
        splitArea: {
          show: false
        }
      },
      series: [
        {
          type: 'bar',
          data: data.valueData,
          // Set `large` for large data amount
          large: true
        }
      ]
    };
    function generateData(count) {
      let baseValue = Math.random() * 1000;
      let time = +new Date(2011, 0, 1);
      let smallBaseValue;
      function next(idx) {
        smallBaseValue =
          idx % 30 === 0
            ? Math.random() * 700
            : smallBaseValue + Math.random() * 500 - 250;
        baseValue += Math.random() * 20 - 10;
        return Math.max(0, Math.round(baseValue + smallBaseValue) + 3000);
      }
      const categoryData = [];
      const valueData = [];
      for (let i = 0; i < count; i++) {
        categoryData.push(
          echarts.format.formatTime('yyyy-MM-dd\nhh:mm:ss', time, false)
        );
        valueData.push(next(i).toFixed(2));
        time += 1000;
      }
      return {
        categoryData: categoryData,
        valueData: valueData
      };
    }
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
