/**
 * 官网示例：基础和弦图
 * https://echarts.apache.org/examples/zh/editor.html?c=chord-simple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Basic Chord
    category: chord
    titleCN: 基础和弦图
    difficulty: 0
    since: 6.0.0
    */
    option = {
      tooltip: {},
      legend: {},
      series: [
        {
          type: 'chord',
          clockwise: false,
          label: { show: true },
          lineStyle: { color: 'target' },
          data: [{ name: 'A' }, { name: 'B' }, { name: 'C' }, { name: 'D' }],
          links: [
            { source: 'A', target: 'B', value: 40 },
            { source: 'A', target: 'C', value: 20 },
            { source: 'B', target: 'D', value: 20 }
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
