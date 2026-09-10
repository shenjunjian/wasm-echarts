/**
 * 官网示例：和弦图 minAngle
 * https://echarts.apache.org/examples/zh/editor.html?c=chord-minAngle
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Chord minAngle
    category: chord
    titleCN: 和弦图 minAngle
    difficulty: 1
    since: 6.0.0
    */
    option = {
      tooltip: {},
      legend: {},
      series: [
        {
          type: 'chord',
          label: { show: true },
          minAngle: 30,
          data: [
            { name: 'A' },
            { name: 'B' },
            { name: 'C' },
            { name: 'D' },
            { name: 'E' },
            { name: 'F' }
          ],
          links: [
            { source: 'A', target: 'B', value: 40 },
            { source: 'B', target: 'C', value: 20 },
            { source: 'E', target: 'A', value: 5 }
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
