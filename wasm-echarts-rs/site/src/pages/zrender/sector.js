import { mountExample } from '@shared/example-shell.js';
import { runZrenderExample, defaultSource } from '@zrender/example-runner.js';

mountExample(document.getElementById('app'), {
  title: '扇区 · sector',
  description: 'SectorShape 饼图扇区',
  backHref: '/zrender/examples/',
  defaultSource: defaultSource('sector'),
  run: runZrenderExample,
});
