import { mountExample } from '@shared/example-shell.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { PIE_OPTION, optionSource } from '@echarts/options.js';

mountExample(document.getElementById('app'), {
  title: '饼图 · pie',
  description: '扇区 + item tooltip',
  backHref: '/echarts/examples/',
  defaultSource: optionSource(PIE_OPTION),
  run: runEchartsExample,
});
