import { mountExampleGallery } from '@shared/example-gallery.js';
import { ZRENDER_EXAMPLES } from '@zrender/examples-catalog.js';

mountExampleGallery(document.getElementById('app'), {
  title: 'wasm-zrender 实例',
  description: '左侧切换示例，右侧查看完整接入代码与 iframe 预览。',
  examples: ZRENDER_EXAMPLES,
  defaultId: 'shapes',
});
