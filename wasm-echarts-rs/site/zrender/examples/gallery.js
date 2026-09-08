import { mountExampleGallery } from '../../src/shared/example-gallery.js';
import helloWorldSource from './hello_world.js?raw';
import animationSource from './animation.js?raw';
import boundingBoxSource from './bounding_box.js?raw';
import clipPathSource from './clip_path.js?raw';
import glitchedTextSource from './glitched_text.js?raw';
import particlesSource from './particles.js?raw';
import shapesSource from './shapes.js?raw';
import textSource from './text.js?raw';
import sectorSource from './sector.js?raw';
import hitSource from './hit.js?raw';
import stateSource from './state.js?raw';

mountExampleGallery(document.getElementById('app'), {
  title: 'wasm-zrender 实例',
  description: '对齐官方 zrender 示例；动画为终态语义（不播中间帧）。右侧查看完整接入代码与 iframe 预览。',
  defaultId: 'hello_world',
  examples: [
    {
      id: 'hello_world',
      title: 'Hello World!',
      description: 'Circle + Rect + RadialGradient',
      previewUrl: './hello_world.html',
      source: helloWorldSource,
    },
    {
      id: 'animation',
      title: 'Animation',
      description: 'animate().when().start() 写入最后一组 when（圆停在右侧）',
      previewUrl: './animation.html',
      source: animationSource,
    },
    {
      id: 'bounding_box',
      title: 'Bounding Box',
      description: 'init(canvas) + draggable Circle，实时 Group boundingRect',
      previewUrl: './bounding_box.html',
      source: boundingBoxSource,
    },
    {
      id: 'clip_path',
      title: 'ClipPath',
      description: 'Circle.setClipPath(Heart)',
      previewUrl: './clip_path.html',
      source: clipPathSource,
    },
    {
      id: 'glitched_text',
      title: 'Glitched Text',
      description: 'Text + attr(position/shape/style)',
      previewUrl: './glitched_text.html',
      source: glitchedTextSource,
    },
    {
      id: 'particles',
      title: 'Particles',
      description: 'animate/during/done 终态：during(t=1) 后 done 立刻 remove',
      previewUrl: './particles.html',
      source: particlesSource,
    },
    {
      id: 'shapes',
      title: '基础图形 shapes',
      description: 'Rect.r / Sector.r0、原型链 Rect instanceof Path、Group.x',
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
