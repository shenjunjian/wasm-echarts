import { mountExample } from '@shared/example-shell.js';
import { runZrenderExample, defaultSource } from '@zrender/example-runner.js';

mountExample(document.getElementById('app'), {
  title: '状态 · state',
  description: '点击图元切换 emphasis 高亮',
  backHref: '/zrender/examples/',
  defaultSource: defaultSource('state'),
  run: (source, el, log) => runZrenderExample(source, el, log, { interactive: 'state' }),
});
