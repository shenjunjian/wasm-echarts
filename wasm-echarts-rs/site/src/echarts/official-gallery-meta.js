/**
 * 官网画廊分组顺序与中文标题。gallery.js 按此挂二级菜单；
 * 某类尚无 official-{category}-catalog.js 且无自写示例时不出现空组。
 */
export const OFFICIAL_CATEGORY_GROUPS = [
  {
    category: 'line',
    title: '折线图',
    handmade: [
      { id: 'line', title: '基础折线', description: 'category 轴折线图' },
      {
        id: 'fonts',
        title: '多字体',
        description:
          'registerFont 引入雅黑 / 宋体 / 楷体，分别用于标题、副标题、图例与轴单位',
      },
    ],
  },
  {
    category: 'bar',
    title: '柱状图',
    handmade: [{ id: 'bar', title: '基础柱状', description: 'value 轴柱状图' }],
  },
  {
    category: 'pie',
    title: '饼图',
    handmade: [{ id: 'pie', title: '基础饼图', description: '扇区饼图' }],
  },
  {
    category: 'scatter',
    title: '散点图',
    handmade: [{ id: 'scatter', title: '基础散点', description: 'value 轴散点图' }],
  },
  { category: 'candlestick', title: 'K 线图', handmade: [] },
  { category: 'boxplot', title: '盒须图', handmade: [] },
  { category: 'heatmap', title: '热力图', handmade: [] },
  { category: 'pictorialBar', title: '象形柱图', handmade: [] },
  { category: 'gauge', title: '仪表盘', handmade: [] },
  { category: 'radar', title: '雷达图', handmade: [] },
  { category: 'funnel', title: '漏斗图', handmade: [] },
  { category: 'chord', title: '和弦图', handmade: [] },
  { category: 'sunburst', title: '旭日图', handmade: [] },
  { category: 'tree', title: '树图', handmade: [] },
  { category: 'treemap', title: '矩形树图', handmade: [] },
  { category: 'graph', title: '关系图', handmade: [] },
  { category: 'sankey', title: '桑基图', handmade: [] },
  { category: 'themeRiver', title: '主题河流', handmade: [] },
  { category: 'calendar', title: '日历', handmade: [] },
  { category: 'matrix', title: '矩阵', handmade: [] },
  { category: 'parallel', title: '平行坐标', handmade: [] },
  { category: 'map', title: '地图', handmade: [] },
  { category: 'geo', title: '地理坐标', handmade: [] },
  { category: 'lines', title: '路径图', handmade: [] },
  { category: 'custom', title: '自定义系列', handmade: [] },
  { category: 'dataset', title: '数据集', handmade: [] },
  { category: 'graphic', title: '图形组件', handmade: [] },
];

export const INTERACTION_EXAMPLES = [
  {
    id: 'interactive',
    title: '点击 / tooltip / zoom',
    description: 'use() 提示 + on(click) / toggleSelect / tooltip / wheel zoom',
  },
  {
    id: 'merge',
    title: 'setOption 合并',
    description: '深合并、notMerge: true、dispose 后再 init',
  },
  {
    id: 'bench',
    title: '性能基准',
    description: 'benchmarkRender 30 次均值',
  },
];
