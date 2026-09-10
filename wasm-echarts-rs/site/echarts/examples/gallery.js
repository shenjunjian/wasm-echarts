import { mountExampleGallery } from '../../src/shared/example-gallery.js';
import {
  INTERACTION_EXAMPLES,
  OFFICIAL_CATEGORY_GROUPS,
} from '../../src/echarts/official-gallery-meta.js';

const exampleSources = import.meta.glob(
  ['./*.js', '!./gallery.js', '!./official-*-catalog.js'],
  { query: '?raw', import: 'default', eager: true },
);

const officialCatalogs = import.meta.glob('./official-*-catalog.js', {
  eager: true,
});

function exampleSource(id) {
  const src = exampleSources[`./${id}.js`];
  if (!src) {
    throw new Error(`缺少示例源码: ${id}`);
  }
  return src;
}

function toGalleryExample(item) {
  return {
    id: item.id,
    title: item.title,
    description: item.description,
    previewUrl: `./${item.id}.html`,
    source: exampleSource(item.id),
  };
}

function catalogFor(category) {
  const mod = officialCatalogs[`./official-${category}-catalog.js`];
  return Array.isArray(mod?.officialExamples) ? mod.officialExamples : [];
}

function buildGroups() {
  const seen = new Set();
  const groups = [];

  for (const spec of OFFICIAL_CATEGORY_GROUPS) {
    const examples = [];
    for (const item of spec.handmade) {
      if (seen.has(item.id)) continue;
      seen.add(item.id);
      examples.push(toGalleryExample(item));
    }
    for (const item of catalogFor(spec.category)) {
      if (seen.has(item.id)) continue;
      seen.add(item.id);
      examples.push(toGalleryExample(item));
    }
    if (!examples.length) continue;
    groups.push({
      id: `cat-${spec.category}`,
      title: spec.title,
      examples,
    });
  }

  groups.push({
    id: 'cat-interaction',
    title: '交互合集',
    examples: INTERACTION_EXAMPLES.map(toGalleryExample),
  });

  return groups;
}

mountExampleGallery(document.getElementById('app'), {
  title: 'wasm-echarts 实例',
  description:
    '每个示例是完整独立脚本。含官网同步条目的类别：未实现的官方能力会在预览里报错，便于后续补齐。轴标签渲染前须 registerFont。',
  defaultId: 'line',
  groups: buildGroups(),
});
