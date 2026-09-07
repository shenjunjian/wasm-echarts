import { mountExampleGallery } from '../../src/shared/example-gallery.js';
import shapesSource from './shapes.js?raw';
import textSource from './text.js?raw';
import sectorSource from './sector.js?raw';
import hitSource from './hit.js?raw';
import stateSource from './state.js?raw';

mountExampleGallery(document.getElementById('app'), {
  title: 'wasm-zrender 实例',
  description: '左侧切换示例，右侧查看完整接入代码与 iframe 预览。',
  defaultId: 'shapes',
  examples: [
    {
      id: 'shapes',
      title: '基础图形 shapes',
      description: 'Group + Rect / Circle / Line / Polygon',
      previewUrl: './shapes.html',
      source: shapesSource,
    },
    {
      id: 'text',
      title: '文本 text',
      description: 'registerFont + new Text({ style })',
      previewUrl: './text.html',
      source: textSource,
    },
    {
      id: 'sector',
      title: '扇区 sector',
      description: '循环 new Sector 饼图扇区',
      previewUrl: './sector.html',
      source: sectorSource,
    },
    {
      id: 'hit',
      title: '命中检测 hit',
      description: 'findHover + 鼠标移动',
      previewUrl: './hit.html',
      source: hitSource,
    },
    {
      id: 'state',
      title: '状态 state',
      description: 'setStateStyle + useState emphasis',
      previewUrl: './state.html',
      source: stateSource,
    },
  ],
});
