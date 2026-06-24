import { mountExample } from '@shared/example-shell.js';
import { runZrenderExample, defaultSource } from '@zrender/example-runner.js';

mountExample(document.getElementById('app'), {
  title: '基础图形 · shapes',
  description: 'Rect / Circle / Line / Polygon / 渐变 / 虚线 / 阴影',
  backHref: '/zrender/examples/',
  defaultSource: defaultSource('shapes'),
  run: runZrenderExample,
});
