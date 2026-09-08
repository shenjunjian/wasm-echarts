/**
 * wasm-zrender 公开 API：官方 export.ts / zrender.ts 命名空间。
 * default 仍是 wasm-bindgen 的 initWasm。
 */
export { default } from './native.js';

export {
  init,
  dispose,
  disposeAll,
  getInstance,
  registerPainter,
  version,
  ZRender,
} from './zrender.js';

export { Element } from './element.js';
export { Displayable } from './displayable.js';
export { Path } from './path.js';
export { Group } from './group.js';
export { Text } from './text.js';
export { Image } from './image.js';
export { TSpan } from './tspan.js';
export { IncrementalDisplayable } from './incremental.js';

export {
  Arc,
  BezierCurve,
  Circle,
  CompoundPath,
  Droplet,
  Ellipse,
  Heart,
  Isogon,
  Line,
  Polygon,
  Polyline,
  Rect,
  Ring,
  Rose,
  Sector,
  Star,
  Trochoid,
} from './shapes/index.js';

export { Animator } from './animator.js';
export {
  registerFont,
  clearFonts,
  LinearGradient,
  RadialGradient,
  Pattern,
  OrientedBoundingRect,
  Animation,
  Handler,
  HoverResult,
} from './native.js';
export { Point } from './point.js';
export { BoundingRect } from './bounding_rect.js';

export * as matrix from './tool/matrix.js';
export * as vector from './tool/vector.js';
export * as color from './tool/color.js';
export * as path from './tool/path.js';
export * as util from './tool/util.js';
export * as morph from './tool/morph.js';
export { parseSVG } from './tool/parseSVG.js';
export { default as showDebugDirtyRect } from './tool/debug.js';
export { setPlatformAPI } from './tool/platform.js';
