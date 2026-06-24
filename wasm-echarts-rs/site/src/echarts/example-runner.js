import echarts from './echarts.js';
import { parseEchartsSource } from '@shared/parse-source.js';

export async function runEchartsExample(source, previewEl, log, hooks = {}) {
  const option = parseEchartsSource(source);
  const chart = await echarts.init(previewEl, { renderer: 'canvas' });
  chart.setOption(option);

  hooks.onChart?.(chart);

  const seriesType = option.series?.[0]?.type ?? 'chart';
  log(`已渲染 ${seriesType} 图表`);

  return { dispose: () => chart.dispose() };
}
