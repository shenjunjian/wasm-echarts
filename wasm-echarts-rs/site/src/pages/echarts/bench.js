import { mountExample } from '@shared/example-shell.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { LINE_OPTION, optionSource } from '@echarts/options.js';

mountExample(document.getElementById('app'), {
  title: '性能基准 · bench',
  description: 'chart.benchmark(30) 渲染均值',
  backHref: '/echarts/examples/',
  defaultSource: optionSource(LINE_OPTION),
  run: (source, el, log) =>
    runEchartsExample(source, el, log, {
      onChart: (chart) => {
        const ms = chart.benchmark(30);
        log(`benchmark_render 均值 ${ms.toFixed(2)} ms（30 次）`);
      },
    }),
});
