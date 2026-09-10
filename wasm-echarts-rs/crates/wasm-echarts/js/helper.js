/**
 * echarts.helper：扩展用辅助。按签名提供最小实现，未完成项 console.warn。
 */
import { parsePercent } from './number.js';
import { normalizeCssArray } from './format.js';

function warn(name) {
  console.warn(`[wasm-echarts] helper.${name} 尚未完整实现`);
}

export function getLayoutRect(positionInfo, containerRect, margin) {
  const info = positionInfo || {};
  const box = containerRect || { x: 0, y: 0, width: 0, height: 0 };
  const m = normalizeCssArray(margin || 0);
  const cw = box.width;
  const ch = box.height;
  let width = parsePercent(info.width, cw);
  let height = parsePercent(info.height, ch);
  let left = parsePercent(info.left, cw);
  let top = parsePercent(info.top, ch);
  const right = parsePercent(info.right, cw);
  const bottom = parsePercent(info.bottom, ch);
  if (!Number.isFinite(width)) {
    width = cw - m[1] - m[3];
    if (Number.isFinite(left) && Number.isFinite(right)) {
      width = cw - left - right;
    } else if (Number.isFinite(left)) {
      width = cw - left - m[1];
    } else if (Number.isFinite(right)) {
      width = cw - right - m[3];
    }
  }
  if (!Number.isFinite(height)) {
    height = ch - m[0] - m[2];
    if (Number.isFinite(top) && Number.isFinite(bottom)) {
      height = ch - top - bottom;
    } else if (Number.isFinite(top)) {
      height = ch - top - m[2];
    } else if (Number.isFinite(bottom)) {
      height = ch - bottom - m[0];
    }
  }
  if (!Number.isFinite(left)) {
    left = Number.isFinite(right) ? cw - right - width : m[3];
  }
  if (!Number.isFinite(top)) {
    top = Number.isFinite(bottom) ? ch - bottom - height : m[0];
  }
  return {
    x: (box.x || 0) + left,
    y: (box.y || 0) + top,
    width: Math.max(0, width),
    height: Math.max(0, height),
  };
}

export function createList() {
  warn('createList');
  return null;
}

export function createDimensions() {
  warn('createDimensions');
  return [];
}

export const dataStack = {
  isDimensionStacked() {
    return false;
  },
  enableDataStack() {},
  getStackedDimension() {
    return null;
  },
};

export function createSymbol() {
  warn('createSymbol');
  return null;
}

export function createScale() {
  warn('createScale');
  return null;
}

export function mixinAxisModelCommonMethods() {
  warn('mixinAxisModelCommonMethods');
}

export function getECData(el) {
  return (el && el.data) || {};
}

export function enableHoverEmphasis() {
  warn('enableHoverEmphasis');
}

export function createTextStyle(textStyle) {
  return { ...(textStyle || {}) };
}
