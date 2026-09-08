/**
 * morph 命名空间：签名齐全，形变本身后置。
 * 动画例外：不播中间帧，morphPath 直接返回终点 path。
 */

function finish(opts) {
  if (!opts) {
    return;
  }
  if (typeof opts.during === 'function') {
    opts.during(1);
  }
  if (typeof opts.done === 'function') {
    opts.done();
  }
}

export function alignBezierCurves(array1, array2) {
  return [array1, array2];
}

export function centroid(array) {
  if (!array || !array.length) {
    return [0, 0, 0];
  }
  let signedArea = 0;
  let cx = 0;
  let cy = 0;
  const len = array.length;
  for (let i = 0, j = len - 2; i < len; j = i, i += 2) {
    const x0 = array[j];
    const y0 = array[j + 1];
    const x1 = array[i];
    const y1 = array[i + 1];
    const a = x0 * y1 - x1 * y0;
    signedArea += a;
    cx += (x0 + x1) * a;
    cy += (y0 + y1) * a;
  }
  if (signedArea === 0) {
    return [array[0] || 0, array[1] || 0, 0];
  }
  return [cx / signedArea / 3, cy / signedArea / 3, signedArea];
}

export function isCombineMorphing(path) {
  return !!(path && path.__isCombineMorphing);
}

export function isMorphing(el) {
  return !!(el && el.__morphT >= 0);
}

export function morphPath(fromPath, toPath, animationOpts) {
  finish(animationOpts);
  return toPath;
}

function emptyReturn() {
  return { fromIndividuals: [], toIndividuals: [], count: 0 };
}

export function combineMorph(fromList, toPath, animationOpts) {
  finish(animationOpts);
  if (!toPath) {
    return emptyReturn();
  }
  return {
    fromIndividuals: [],
    toIndividuals: [toPath],
    count: 1,
  };
}

export function separateMorph(fromPath, toPathList, animationOpts) {
  finish(animationOpts);
  const list = toPathList || [];
  return {
    fromIndividuals: [],
    toIndividuals: list,
    count: list.length,
  };
}

export function defaultDividePath() {
  return [];
}
