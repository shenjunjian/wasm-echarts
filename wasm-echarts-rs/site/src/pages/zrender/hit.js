import { mountExample } from '@shared/example-shell.js';
import { runZrenderExample, defaultSource } from '@zrender/example-runner.js';

mountExample(document.getElementById('app'), {
  title: '命中检测 · hit',
  description: '鼠标移动查看 find_hover 结果',
  backHref: '/zrender/examples/',
  defaultSource: defaultSource('hit'),
  run: (source, el, log) => runZrenderExample(source, el, log, { interactive: 'hit' }),
});
