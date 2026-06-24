import { mountExample } from '@shared/example-shell.js';
import echarts from '@echarts/echarts.js';
import { parseEchartsSourceStripComments, formatOption } from '@shared/parse-source.js';
import { INTERACTIVE_OPTION } from '@echarts/options.js';

const BASE = { ...INTERACTIVE_OPTION };
delete BASE.tooltip;

mountExample(document.getElementById('app'), {
  title: '交互合集 · interactive',
  description: '含 tooltip.formatter 函数；hover / click select / 滚轮 dataZoom',
  backHref: '/echarts/examples/',
  defaultSource: `${formatOption(BASE)}

// 运行时会自动注入 tooltip.formatter 函数（JSON 无法表示函数）`,
  run: async (source, previewEl, log) => {
    const option = parseEchartsSourceStripComments(source);
    option.tooltip = {
      trigger: 'item',
      axisPointer: { type: 'line' },
      formatter(params) {
        return `${params.seriesName}<br/>${params.name}: ${params.value}`;
      },
    };

    const chart = await echarts.init(previewEl, { renderer: 'canvas' });
    chart.setOption(option);
    chart.on('click', ({ hit }) => {
      if (hit?.dataIndex != null) {
        log(`click select → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`);
      }
    });
    log('已加载含 formatter 的折线图；试试 hover / 点击 / 滚轮缩放');
    return { dispose: () => chart.dispose() };
  },
});
