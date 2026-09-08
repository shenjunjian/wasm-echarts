/**
 * SVG path 字符串工具（官方 path 命名空间）。
 * createFromString 落到 Path + shape.pathData。
 */
import { Path } from '../path.js';

function mergeOpts(str, opts) {
  const next = opts ? { ...opts } : {};
  next.shape = { ...(opts && opts.shape), pathData: str };
  return next;
}

export function createFromString(str, opts) {
  return new Path(mergeOpts(str, opts));
}

export function extendFromString(str, defaultOpts) {
  return class SVGPath extends Path {
    constructor(opts) {
      super(
        mergeOpts(str, {
          ...(defaultOpts || {}),
          ...(opts || {}),
          shape: {
            ...(defaultOpts && defaultOpts.shape),
            ...(opts && opts.shape),
            pathData: str,
          },
        }),
      );
    }
  };
}

function pathDataOf(el) {
  if (!el) {
    return '';
  }
  if (el.shape && el.shape.pathData) {
    return el.shape.pathData;
  }
  if (typeof el.getPathData === 'function') {
    return el.getPathData() || '';
  }
  return '';
}

export function mergePath(pathEls, opts) {
  const parts = [];
  const list = pathEls || [];
  for (let i = 0; i < list.length; i++) {
    const d = pathDataOf(list[i]);
    if (d) {
      parts.push(d);
    }
  }
  const next = opts ? { ...opts } : {};
  next.shape = { ...(opts && opts.shape), pathData: parts.join(' ') };
  return new Path(next);
}

export function clonePath(sourcePath, opts) {
  opts = opts || {};
  const path = new Path();
  if (sourcePath && sourcePath.shape) {
    path.setShape({ ...sourcePath.shape });
  }
  if (sourcePath && sourcePath.style) {
    path.setStyle({ ...sourcePath.style });
  }
  if (sourcePath && !opts.bakeTransform) {
    path.attr({
      x: sourcePath.x,
      y: sourcePath.y,
      scaleX: sourcePath.scaleX,
      scaleY: sourcePath.scaleY,
      rotation: sourcePath.rotation,
      originX: sourcePath.originX,
      originY: sourcePath.originY,
    });
  }
  if (sourcePath) {
    if (sourcePath.z != null) path.z = sourcePath.z;
    if (sourcePath.z2 != null) path.z2 = sourcePath.z2;
    if (sourcePath.zlevel != null) path.zlevel = sourcePath.zlevel;
  }
  return path;
}
