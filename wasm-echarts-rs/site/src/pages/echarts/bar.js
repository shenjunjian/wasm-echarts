import { mountPreview } from '@shared/mount-preview.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { BAR_OPTION, optionSource } from '@echarts/options.js';

mountPreview((host, log) => runEchartsExample(optionSource(BAR_OPTION), host, log));
