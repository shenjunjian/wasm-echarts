import { mountExample } from '@shared/example-shell.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { SCATTER_OPTION, optionSource } from '@echarts/options.js';

mountExample(document.getElementById('app'), {
  title: '散点图 · scatter',
  description: '双 value 轴',
  backHref: '/echarts/examples/',
  defaultSource: optionSource(SCATTER_OPTION),
  run: runEchartsExample,
});
