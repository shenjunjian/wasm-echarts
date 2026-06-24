import { mountPreview } from '@shared/mount-preview.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { SCATTER_OPTION, optionSource } from '@echarts/options.js';

mountPreview((host, log) => runEchartsExample(optionSource(SCATTER_OPTION), host, log));
