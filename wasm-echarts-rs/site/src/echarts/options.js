import { formatOption } from '@shared/parse-source.js';

export const LINE_OPTION = {
  tooltip: { trigger: 'item', axisPointer: { type: 'line' } },
  dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
  xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
  yAxis: { type: 'value' },
  series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
};

export const BAR_OPTION = {
  tooltip: { trigger: 'item', axisPointer: { type: 'line' } },
  dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
  xAxis: { type: 'category', data: ['A', 'B', 'C', 'D'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', name: '数量', data: [40, 90, 60, 120] }],
};

export const PIE_OPTION = {
  tooltip: { trigger: 'item' },
  series: [
    {
      type: 'pie',
      name: '占比',
      radius: '55%',
      data: [
        { name: '直接访问', value: 335 },
        { name: '邮件营销', value: 310 },
        { name: '联盟广告', value: 234 },
        { name: '视频广告', value: 135 },
        { name: '搜索引擎', value: 1548 },
      ],
    },
  ],
};

export const SCATTER_OPTION = {
  tooltip: { trigger: 'item' },
  xAxis: { type: 'value', scale: true },
  yAxis: { type: 'value', scale: true },
  series: [
    {
      type: 'scatter',
      name: '样本',
      data: [
        [10.0, 8.04],
        [8.07, 6.95],
        [13.0, 7.58],
        [9.05, 8.81],
        [11.0, 8.33],
        [14.0, 7.66],
        [13.4, 6.81],
        [10.0, 6.33],
        [14.0, 8.96],
        [12.5, 6.82],
      ],
    },
  ],
};

export const INTERACTIVE_OPTION = {
  tooltip: {
    trigger: 'item',
    formatter(params) {
      return `${params.seriesName}<br/>${params.name}: ${params.value}`;
    },
  },
  dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
  xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
  yAxis: { type: 'value' },
  series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
};

export function optionSource(option) {
  return formatOption(option);
}
