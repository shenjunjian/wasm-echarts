/**
 * echarts.number：按官方 export/api/number.ts 签名重写，不复制官方源码。
 */

export const MAX_SAFE_INTEGER = 9007199254740991;

export function linearMap(val, domain, range, clamp) {
  const d0 = domain[0];
  const d1 = domain[1];
  const r0 = range[0];
  const r1 = range[1];
  const span = d1 - d0;
  if (span === 0) {
    return (r0 + r1) / 2;
  }
  let t = (val - d0) / span;
  if (clamp) {
    t = Math.min(Math.max(t, 0), 1);
  }
  return r0 + t * (r1 - r0);
}

export function round(x, precision) {
  if (precision == null) {
    precision = 10;
  }
  precision = Math.min(Math.max(0, Math.round(precision)), 20);
  return +(+x).toFixed(precision);
}

export function asc(arr) {
  arr.sort((a, b) => a - b);
  return arr;
}

export function getPrecision(val) {
  const str = String(val);
  const eIndex = str.indexOf('e');
  if (eIndex > 0) {
    const exp = +str.slice(eIndex + 1);
    return exp < 0 ? -exp : 0;
  }
  const dot = str.indexOf('.');
  return dot < 0 ? 0 : str.length - 1 - dot;
}

export function getPrecisionSafe(val) {
  return getPrecision(val);
}

export function getPixelPrecision(dataExtent, pixelExtent) {
  const log = Math.round(
    Math.log(Math.abs(dataExtent[1] - dataExtent[0]) / Math.abs(pixelExtent[1] - pixelExtent[0])) /
      Math.LN10,
  );
  return Math.max(0, -log);
}

export function getPercentWithPrecision(valueList, idx, precision) {
  const sum = valueList.reduce((s, v) => s + (Number(v) || 0), 0);
  if (sum === 0) {
    return 0;
  }
  return round(((Number(valueList[idx]) || 0) / sum) * 100, precision);
}

export function parsePercent(percent, all) {
  if (percent == null) {
    return NaN;
  }
  if (typeof percent === 'number') {
    return percent;
  }
  const str = String(percent);
  if (str === 'auto') {
    return NaN;
  }
  if (str.slice(-1) === '%') {
    return (parseFloat(str) / 100) * all;
  }
  const n = parseFloat(str);
  return Number.isFinite(n) ? n : NaN;
}

export function remRadian(radian) {
  const pi2 = Math.PI * 2;
  return ((radian % pi2) + pi2) % pi2;
}

export function isRadianAroundZero(val) {
  return val > -1e-4 && val < 1e-4;
}

export function parseDate(value) {
  if (value instanceof Date) {
    return value;
  }
  if (typeof value === 'number' || (typeof value === 'string' && /^\d+$/.test(value))) {
    return new Date(+value);
  }
  if (typeof value === 'string') {
    const normalized = value.replace(/-/g, '/');
    const parsed = Date.parse(normalized);
    if (!Number.isNaN(parsed)) {
      return new Date(parsed);
    }
  }
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? new Date(0) : date;
}

export function quantity(val) {
  return Math.pow(10, quantityExponent(val));
}

export function quantityExponent(val) {
  if (val === 0) {
    return 0;
  }
  return Math.floor(Math.log(Math.abs(val)) / Math.LN10);
}

export function nice(val, roundResult) {
  const exp = quantityExponent(val);
  const exp10 = Math.pow(10, exp);
  const f = val / exp10;
  let nf;
  if (roundResult) {
    if (f < 1.5) nf = 1;
    else if (f < 2.5) nf = 2;
    else if (f < 4) nf = 3;
    else if (f < 7) nf = 5;
    else nf = 10;
  } else if (f < 1) nf = 1;
  else if (f < 2) nf = 2;
  else if (f < 3) nf = 3;
  else if (f < 5) nf = 5;
  else nf = 10;
  return nf * exp10;
}

export function quantile(ascArr, p) {
  const n = ascArr.length;
  if (!n) {
    return 0;
  }
  const h = (n - 1) * p;
  const i = Math.floor(h);
  const a = ascArr[i];
  const b = ascArr[Math.min(i + 1, n - 1)];
  return a + (b - a) * (h - i);
}

export function reformIntervals(list) {
  return list;
}

export function isNumeric(value) {
  return numericToNumber(value) != null;
}

export function numericToNumber(val) {
  if (val == null || val === '') {
    return null;
  }
  const n = Number(val);
  return Number.isFinite(n) ? n : null;
}
