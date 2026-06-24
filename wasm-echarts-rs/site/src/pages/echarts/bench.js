import { mountPreview } from '@shared/mount-preview.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { LINE_OPTION, optionSource } from '@echarts/options.js';

mountPreview((host, log) =>
  runEchartsExample(optionSource(LINE_OPTION), host, log, {
    onChart: (chart) => {
      const ms = chart.benchmark(30);
      log(`benchmark_render 均值 ${ms.toFixed(2)} ms（30 次）`);
    },
  }),
);
