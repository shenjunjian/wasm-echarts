import zrender from './zrender.js';
import { parseZrenderSource } from '@shared/parse-source.js';

export async function runZrenderExample(source, previewEl, log, extra = {}) {
  const config = parseZrenderSource(source);
  const viewer = await zrender.createViewer(previewEl, {
    width: config.width,
    height: config.height,
    scene: config.scene,
  });

  if (extra.interactive === 'hit' || config.scene === 'hit') {
    viewer.bindHitTest((hit) => {
      if (hit) {
        log(
          `hover → pathIndex=${hit.pathIndex ?? '-'} seriesIndex=${hit.seriesIndex ?? '-'} dataIndex=${hit.dataIndex ?? '-'}`,
        );
      } else {
        log('鼠标离开画布');
      }
    });
  }

  if (extra.interactive === 'state' || config.scene === 'state') {
    viewer.bindStateToggle((pathIndex, hit) => {
      log(`emphasis → pathIndex=${pathIndex} dataIndex=${hit?.dataIndex ?? '-'}`);
    });
  }

  log(`场景 "${config.scene}" 已加载 (${config.width}×${config.height})`);

  return { dispose: () => viewer.dispose() };
}

export const SCENE_DEFAULTS = {
  shapes: { scene: 'shapes', width: 480, height: 360 },
  text: { scene: 'text', width: 480, height: 360 },
  sector: { scene: 'sector', width: 480, height: 360 },
  hit: { scene: 'hit', width: 480, height: 360 },
  state: { scene: 'state', width: 480, height: 360 },
};

export function defaultSource(scene) {
  return JSON.stringify(SCENE_DEFAULTS[scene] ?? SCENE_DEFAULTS.shapes, null, 2);
}
