import { mountPreview } from '@shared/mount-preview.js';
import { runZrenderExample, defaultSource } from '@zrender/example-runner.js';

mountPreview((host, log) =>
  runZrenderExample(defaultSource('state'), host, log, { interactive: 'state' }),
);
