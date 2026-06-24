import { mountExample } from '@shared/example-shell.js';
import echarts from '@echarts/echarts.js';
import { parseEchartsSource, formatOption } from '@shared/parse-source.js';

const STEP1 = {
  xAxis: { type: 'category', data: ['A', 'B', 'C'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
};

mountExample(document.getElementById('app'), {
  title: 'setOption 合并 · merge',
  description: '先 setOption 初始数据，再 merge 追加 series',
  backHref: '/echarts/examples/',
  defaultSource: formatOption(STEP1),
  run: async (source, previewEl, log) => {
    const first = parseEchartsSource(source);
    const chart = await echarts.init(previewEl, { renderer: 'canvas' });
    chart.setOption(first);
    chart.setOption({
      series: [{ type: 'line', name: '第二批', data: [15, 25, 18] }],
    });
    log('已完成两次 setOption（merge 模式）：bar + line 双 series');
    return { dispose: () => chart.dispose() };
  },
});
