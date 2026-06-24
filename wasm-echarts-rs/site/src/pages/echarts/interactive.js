import { mountPreview } from '@shared/mount-preview.js';
import echarts from '@echarts/echarts.js';
import { parseEchartsSourceStripComments, formatOption } from '@shared/parse-source.js';
import { INTERACTIVE_OPTION } from '@echarts/options.js';

const BASE = { ...INTERACTIVE_OPTION };
delete BASE.tooltip;

const defaultSource = `${formatOption(BASE)}

// 运行时会自动注入 tooltip.formatter 函数（JSON 无法表示函数）`;

mountPreview(async (host, log) => {
  const option = parseEchartsSourceStripComments(defaultSource);
  option.tooltip = {
    trigger: 'item',
    axisPointer: { type: 'line' },
    formatter(params) {
      return `${params.seriesName}<br/>${params.name}: ${params.value}`;
    },
  };

  const chart = await echarts.init(host, { renderer: 'canvas' });
  chart.setOption(option);
  chart.on('click', ({ hit }) => {
    if (hit?.dataIndex != null) {
      log(`click select → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`);
    }
  });
  log('已加载含 formatter 的折线图；试试 hover / 点击 / 滚轮缩放');
  return { dispose: () => chart.dispose() };
});
