import { mountExampleGallery } from '@shared/example-gallery.js';
import { ECHARTS_EXAMPLES } from '@echarts/examples-catalog.js';

mountExampleGallery(document.getElementById('app'), {
  title: 'wasm-echarts 实例',
  description: '左侧切换示例，右侧查看完整接入代码与 iframe 预览。',
  examples: ECHARTS_EXAMPLES,
  defaultId: 'line',
});
