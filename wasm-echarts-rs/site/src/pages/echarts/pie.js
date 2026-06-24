import { mountPreview } from '@shared/mount-preview.js';
import { runEchartsExample } from '@echarts/example-runner.js';
import { PIE_OPTION, optionSource } from '@echarts/options.js';

mountPreview((host, log) => runEchartsExample(optionSource(PIE_OPTION), host, log));
