import { mountExampleGallery } from '../../src/shared/example-gallery.js';
import lineSource from './line.js?raw';
import barSource from './bar.js?raw';
import pieSource from './pie.js?raw';
import scatterSource from './scatter.js?raw';
import interactiveSource from './interactive.js?raw';
import mergeSource from './merge.js?raw';
import benchSource from './bench.js?raw';

mountExampleGallery(document.getElementById('app'), {
  title: 'wasm-echarts 实例',
  description: '每个示例是完整独立脚本，直接 import @wasm-echarts。右侧查看源码与 iframe 预览。',
  defaultId: 'line',
  examples: [
    {
      id: 'line',
      title: '折线图 line',
      description: 'category 轴折线图',
      previewUrl: './line.html',
      source: lineSource,
    },
    {
      id: 'bar',
      title: '柱状图 bar',
      description: '基础柱状',
      previewUrl: './bar.html',
      source: barSource,
    },
    {
      id: 'pie',
      title: '饼图 pie',
      description: '扇区饼图',
      previewUrl: './pie.html',
      source: pieSource,
    },
    {
      id: 'scatter',
      title: '散点图 scatter',
      description: 'value 轴',
      previewUrl: './scatter.html',
      source: scatterSource,
    },
    {
      id: 'interactive',
      title: '交互合集 interactive',
      description: 'formatter tooltip + hover / select / zoom',
      previewUrl: './interactive.html',
      source: interactiveSource,
    },
    {
      id: 'merge',
      title: 'setOption 合并 merge',
      description: '二次 setOption 深合并 + getOption',
      previewUrl: './merge.html',
      source: mergeSource,
    },
    {
      id: 'bench',
      title: '性能基准 bench',
      description: 'benchmarkRender 30 次均值',
      previewUrl: './bench.html',
      source: benchSource,
    },
  ],
});
