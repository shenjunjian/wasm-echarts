/**
 * 官网示例：多漏斗图
 * https://echarts.apache.org/examples/zh/editor.html?c=funnel-mutiple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Multiple Funnels
    category: funnel
    titleCN: 多漏斗图
    */
    option = {
      title: {
        text: 'Funnel',
        left: 'left',
        top: 'bottom'
      },
      tooltip: {
        trigger: 'item',
        formatter: '{a} <br/>{b} : {c}%'
      },
      toolbox: {
        orient: 'vertical',
        top: 'center',
        feature: {
          dataView: { readOnly: false },
          restore: {},
          saveAsImage: {}
        }
      },
      legend: {
        orient: 'vertical',
        left: 'left',
        data: ['Show', 'Click', 'Visit', 'Inquiry', 'Order']
      },
      series: [
        {
          name: 'Funnel',
          type: 'funnel',
          width: '40%',
          height: '45%',
          left: '5%',
          top: '50%',
          data: [
            { value: 60, name: 'Visit' },
            { value: 30, name: 'Inquiry' },
            { value: 10, name: 'Order' },
            { value: 80, name: 'Click' },
            { value: 100, name: 'Show' }
          ]
        },
        {
          name: 'Pyramid',
          type: 'funnel',
          width: '40%',
          height: '45%',
          left: '5%',
          top: '5%',
          sort: 'ascending',
          data: [
            { value: 60, name: 'Visit' },
            { value: 30, name: 'Inquiry' },
            { value: 10, name: 'Order' },
            { value: 80, name: 'Click' },
            { value: 100, name: 'Show' }
          ]
        },
        {
          name: 'Funnel',
          type: 'funnel',
          width: '40%',
          height: '45%',
          left: '55%',
          top: '5%',
          label: {
            position: 'left'
          },
          data: [
            { value: 60, name: 'Visit' },
            { value: 30, name: 'Inquiry' },
            { value: 10, name: 'Order' },
            { value: 80, name: 'Click' },
            { value: 100, name: 'Show' }
          ]
        },
        {
          name: 'Pyramid',
          type: 'funnel',
          width: '40%',
          height: '45%',
          left: '55%',
          top: '50%',
          sort: 'ascending',
          label: {
            position: 'left'
          },
          data: [
            { value: 60, name: 'Visit' },
            { value: 30, name: 'Inquiry' },
            { value: 10, name: 'Order' },
            { value: 80, name: 'Click' },
            { value: 100, name: 'Show' }
          ]
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
