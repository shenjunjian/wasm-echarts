import { mountExample } from '@shared/example-shell.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { BAR_OPTION, optionSource } from '@echarts/options.js';

mountExample(document.getElementById('app'), {
  title: '柱状图 · bar',
  description: '基础柱状图',
  backHref: '/echarts/examples/',
  defaultSource: optionSource(BAR_OPTION),
  run: runEchartsExample,
});
