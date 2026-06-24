import { mountPreview } from '@shared/mount-preview.js';
import { runZrenderExample } from '@zrender/example-runner.js';

mountPreview((host, log) => runZrenderExample('text', host, log));
