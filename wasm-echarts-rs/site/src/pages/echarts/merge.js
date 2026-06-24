import { mountPreview } from '@shared/mount-preview.js';
import echarts from '@echarts/echarts.js';
import { parseEchartsSource, formatOption } from '@shared/parse-source.js';

const STEP1 = {
  xAxis: { type: 'category', data: ['A', 'B', 'C'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
};

mountPreview(async (host, log) => {
  const chart = await echarts.init(host, { renderer: 'canvas' });
  chart.setOption(parseEchartsSource(formatOption(STEP1)));
  chart.setOption({
    series: [{ type: 'line', name: '第二批', data: [15, 25, 18] }],
  });
  log('已完成两次 setOption（merge 模式）：bar + line 双 series');
  return { dispose: () => chart.dispose() };
});
