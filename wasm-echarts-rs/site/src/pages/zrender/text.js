import { mountExample } from '@shared/example-shell.js';
import { runZrenderExample, defaultSource } from '@zrender/example-runner.js';

mountExample(document.getElementById('app'), {
  title: '文本 · text',
  description: 'fillText、对齐/基线、中文渲染',
  backHref: '/zrender/examples/',
  defaultSource: defaultSource('text'),
  run: runZrenderExample,
});
