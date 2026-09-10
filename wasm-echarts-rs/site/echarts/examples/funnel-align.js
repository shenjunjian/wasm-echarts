/**
 * 官网示例：漏斗图(对比)
 * https://echarts.apache.org/examples/zh/editor.html?c=funnel-align
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Funnel Compare
    category: funnel
    titleCN: 漏斗图(对比)
    */
    option = {
      title: {
        text: 'Funnel Compare',
        subtext: 'Fake Data',
        left: 'left',
        top: 'bottom'
      },
      tooltip: {
        trigger: 'item',
        formatter: '{a} <br/>{b} : {c}%'
      },
      toolbox: {
        show: true,
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
        data: ['Prod A', 'Prod B', 'Prod C', 'Prod D', 'Prod E']
      },
      series: [
        {
          name: 'Funnel',
          type: 'funnel',
          width: '40%',
          height: '45%',
          left: '5%',
          top: '50%',
          funnelAlign: 'right',
          data: [
            { value: 60, name: 'Prod C' },
            { value: 30, name: 'Prod D' },
            { value: 10, name: 'Prod E' },
            { value: 80, name: 'Prod B' },
            { value: 100, name: 'Prod A' }
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
          funnelAlign: 'right',
          data: [
            { value: 60, name: 'Prod C' },
            { value: 30, name: 'Prod D' },
            { value: 10, name: 'Prod E' },
            { value: 80, name: 'Prod B' },
            { value: 100, name: 'Prod A' }
          ]
        },
        {
          name: 'Funnel',
          type: 'funnel',
          width: '40%',
          height: '45%',
          left: '55%',
          top: '5%',
          funnelAlign: 'left',
          data: [
            { value: 60, name: 'Prod C' },
            { value: 30, name: 'Prod D' },
            { value: 10, name: 'Prod E' },
            { value: 80, name: 'Prod B' },
            { value: 100, name: 'Prod A' }
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
          funnelAlign: 'left',
          data: [
            { value: 60, name: 'Prod C' },
            { value: 30, name: 'Prod D' },
            { value: 10, name: 'Prod E' },
            { value: 80, name: 'Prod B' },
            { value: 100, name: 'Prod A' }
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
