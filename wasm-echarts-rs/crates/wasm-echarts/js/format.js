/**
 * echarts.format：按官方 export/api/format.ts 签名重写。
 */
import { parseDate } from './number.js';
import { format as timeFormat } from './time.js';

export function addCommas(x) {
  if (x == null) {
    return '';
  }
  const str = String(x);
  const parts = str.split('.');
  parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ',');
  return parts.join('.');
}

export function toCamelCase(str, upper) {
  const camel = String(str || '').replace(/-(\w)/g, (_, c) => c.toUpperCase());
  if (upper && camel) {
    return camel.charAt(0).toUpperCase() + camel.slice(1);
  }
  return camel;
}

export function normalizeCssArray(val) {
  if (typeof val === 'number') {
    return [val, val, val, val];
  }
  if (Array.isArray(val)) {
    if (val.length === 1) {
      return [val[0], val[0], val[0], val[0]];
    }
    if (val.length === 2) {
      return [val[0], val[1], val[0], val[1]];
    }
    if (val.length === 3) {
      return [val[0], val[1], val[2], val[1]];
    }
    return [val[0] || 0, val[1] || 0, val[2] || 0, val[3] || 0];
  }
  return [0, 0, 0, 0];
}

export function encodeHTML(source) {
  return String(source == null ? '' : source)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

export function formatTpl(tpl, params) {
  if (typeof tpl !== 'string') {
    return String(tpl ?? '');
  }
  const list = Array.isArray(params) ? params : [params];
  return tpl.replace(/\{(\w+)\}/g, (match, key) => {
    for (const item of list) {
      if (item && item[key] != null) {
        return String(item[key]);
      }
    }
    return match;
  });
}

export function getTooltipMarker(opt) {
  const color = typeof opt === 'string' ? opt : opt && opt.color;
  if (!color) {
    return '';
  }
  return `<span style="display:inline-block;margin-right:4px;border-radius:10px;width:10px;height:10px;background-color:${encodeHTML(color)};"></span>`;
}

/**
 * 旧 API：`formatTime(tpl, value, isUTC?)`。新代码请用 `echarts.time.format`。
 */
export function formatTime(tpl, value, isUTC) {
  if (
    tpl === 'week' ||
    tpl === 'month' ||
    tpl === 'quarter' ||
    tpl === 'half-year' ||
    tpl === 'year'
  ) {
    tpl = 'MM-dd\nyyyy';
  }
  const date = parseDate(value);
  const utc = !!isUTC;
  const y = utc ? date.getUTCFullYear() : date.getFullYear();
  const M = (utc ? date.getUTCMonth() : date.getMonth()) + 1;
  const d = utc ? date.getUTCDate() : date.getDate();
  const h = utc ? date.getUTCHours() : date.getHours();
  const m = utc ? date.getUTCMinutes() : date.getMinutes();
  const s = utc ? date.getUTCSeconds() : date.getSeconds();
  const S = utc ? date.getUTCMilliseconds() : date.getMilliseconds();
  if (tpl && tpl.indexOf('{') >= 0) {
    return timeFormat(date, tpl, utc);
  }
  return String(tpl || '')
    .replace('yyyy', String(y))
    .replace('yyyy', String(y))
    .replace('MM', pad(M, 2))
    .replace('M', String(M))
    .replace('dd', pad(d, 2))
    .replace('d', String(d))
    .replace('HH', pad(h, 2))
    .replace('H', String(h))
    .replace('mm', pad(m, 2))
    .replace('m', String(m))
    .replace('ss', pad(s, 2))
    .replace('s', String(s))
    .replace('SSS', pad(S, 3));
}

function pad(n, len) {
  const s = String(n);
  return s.length >= len ? s : '0'.repeat(len - s.length) + s;
}

export function capitalFirst(str) {
  if (!str) {
    return str;
  }
  return str.charAt(0).toUpperCase() + str.slice(1);
}

export function truncateText(text) {
  return text;
}

export function getTextRect() {
  return { x: 0, y: 0, width: 0, height: 0 };
}
