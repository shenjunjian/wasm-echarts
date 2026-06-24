import { mountExample } from '@shared/example-shell.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { LINE_OPTION, optionSource } from '@echarts/options.js';

mountExample(document.getElementById('app'), {
  title: '折线图 · line',
  description: 'category 轴 + inside dataZoom',
  backHref: '/echarts/examples/',
  defaultSource: optionSource(LINE_OPTION),
  run: (source, el, log) =>
    runEchartsExample(source, el, log, {
      onChart: (chart) => {
        chart.on('click', ({ hit }) => {
          if (hit?.dataIndex != null) {
            log(`click → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`);
          }
        });
      },
    }),
});
