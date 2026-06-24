import { mountPreview } from '@shared/mount-preview.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { LINE_OPTION, optionSource } from '@echarts/options.js';

mountPreview((host, log) =>
  runEchartsExample(optionSource(LINE_OPTION), host, log, {
    onChart: (chart) => {
      chart.on('click', ({ hit }) => {
        if (hit?.dataIndex != null) {
          log(`click → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`);
        }
      });
    },
  }),
);
