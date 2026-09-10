import { mountExampleGallery } from '../../src/shared/example-gallery.js';
import { officialLineExamples } from './official-line-catalog.js';
import lineSource from './line.js?raw';
import barSource from './bar.js?raw';
import pieSource from './pie.js?raw';
import scatterSource from './scatter.js?raw';
import interactiveSource from './interactive.js?raw';
import mergeSource from './merge.js?raw';
import benchSource from './bench.js?raw';
import fontsSource from './fonts.js?raw';

const officialSources = import.meta.glob(
  [
    './*.js',
    '!./gallery.js',
    '!./official-line-catalog.js',
    '!./line.js',
    '!./bar.js',
    '!./pie.js',
    '!./scatter.js',
    '!./interactive.js',
    '!./merge.js',
    '!./bench.js',
    '!./fonts.js',
  ],
  { query: '?raw', import: 'default', eager: true },
);

function officialSource(id) {
  const src = officialSources[`./${id}.js`];
  if (!src) {
    throw new Error(`缺少官网示例源码: ${id}`);
  }
  return src;
}

mountExampleGallery(document.getElementById('app'), {
  title: 'wasm-echarts 实例',
  description:
    '每个示例是完整独立脚本。折线图含官网同步条目：未实现的官方能力会在预览里报错，便于后续补齐。轴标签渲染前须 registerFont。',
  defaultId: 'line',
  groups: [
    {
      id: 'cat-line',
      title: '折线图',
      examples: [
        {
          id: 'line',
          title: '基础折线',
          description: 'category 轴折线图',
          previewUrl: './line.html',
          source: lineSource,
        },
        {
          id: 'fonts',
          title: '多字体',
          description: 'registerFont 引入雅黑 / 宋体 / 楷体，分别用于标题、副标题、图例与轴单位',
          previewUrl: './fonts.html',
          source: fontsSource,
        },
        ...officialLineExamples.map((item) => ({
          id: item.id,
          title: item.title,
          description: item.description,
          previewUrl: `./${item.id}.html`,
          source: officialSource(item.id),
        })),
      ],
    },
    {
      id: 'cat-bar',
      title: '柱状图',
      examples: [
        {
          id: 'bar',
          title: '基础柱状',
          description: 'value 轴柱状图',
          previewUrl: './bar.html',
          source: barSource,
        },
      ],
    },
    {
      id: 'cat-pie',
      title: '饼图',
      examples: [
        {
          id: 'pie',
          title: '基础饼图',
          description: '扇区饼图',
          previewUrl: './pie.html',
          source: pieSource,
        },
      ],
    },
    {
      id: 'cat-scatter',
      title: '散点图',
      examples: [
        {
          id: 'scatter',
          title: '基础散点',
          description: 'value 轴散点图',
          previewUrl: './scatter.html',
          source: scatterSource,
        },
      ],
    },
    {
      id: 'cat-interaction',
      title: '交互合集',
      examples: [
        {
          id: 'interactive',
          title: '点击 / tooltip / zoom',
          description: 'use() 提示 + on(click) / toggleSelect / tooltip / wheel zoom',
          previewUrl: './interactive.html',
          source: interactiveSource,
        },
        {
          id: 'merge',
          title: 'setOption 合并',
          description: '深合并、notMerge: true、dispose 后再 init',
          previewUrl: './merge.html',
          source: mergeSource,
        },
        {
          id: 'bench',
          title: '性能基准',
          description: 'benchmarkRender 30 次均值',
          previewUrl: './bench.html',
          source: benchSource,
        },
      ],
    },
  ],
});
