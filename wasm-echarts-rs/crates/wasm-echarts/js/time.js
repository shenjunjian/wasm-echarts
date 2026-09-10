/**
 * echarts.time：按官方 export/api/time.ts 签名重写。
 */
import { parseDate } from './number.js';

const MONTHS = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
];
const MONTHS_ABBR = [
  'Jan',
  'Feb',
  'Mar',
  'Apr',
  'May',
  'Jun',
  'Jul',
  'Aug',
  'Sep',
  'Oct',
  'Nov',
  'Dec',
];
const DAYS = [
  'Sunday',
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday',
];
const DAYS_ABBR = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

function pad(n, len) {
  const s = String(n);
  return s.length >= len ? s : '0'.repeat(len - s.length) + s;
}

export const parse = parseDate;

/**
 * @param {unknown} time
 * @param {string} template
 * @param {boolean} [isUTC=false]
 */
export function format(time, template, isUTC) {
  const date = parseDate(time);
  const utc = !!isUTC;
  const y = utc ? date.getUTCFullYear() : date.getFullYear();
  const M = (utc ? date.getUTCMonth() : date.getMonth()) + 1;
  const q = Math.floor((M - 1) / 3) + 1;
  const d = utc ? date.getUTCDate() : date.getDate();
  const e = utc ? date.getUTCDay() : date.getDay();
  const H = utc ? date.getUTCHours() : date.getHours();
  const h = ((H - 1) % 12) + 1;
  const m = utc ? date.getUTCMinutes() : date.getMinutes();
  const s = utc ? date.getUTCSeconds() : date.getSeconds();
  const S = utc ? date.getUTCMilliseconds() : date.getMilliseconds();
  const a = H >= 12 ? 'pm' : 'am';
  const A = a.toUpperCase();
  return String(template || '')
    .replace(/{a}/g, a)
    .replace(/{A}/g, A)
    .replace(/{yyyy}/g, String(y))
    .replace(/{yy}/g, pad(y % 100, 2))
    .replace(/{Q}/g, String(q))
    .replace(/{MMMM}/g, MONTHS[M - 1])
    .replace(/{MMM}/g, MONTHS_ABBR[M - 1])
    .replace(/{MM}/g, pad(M, 2))
    .replace(/{M}/g, String(M))
    .replace(/{dd}/g, pad(d, 2))
    .replace(/{d}/g, String(d))
    .replace(/{eeee}/g, DAYS[e])
    .replace(/{ee}/g, DAYS_ABBR[e])
    .replace(/{e}/g, String(e))
    .replace(/{HH}/g, pad(H, 2))
    .replace(/{H}/g, String(H))
    .replace(/{hh}/g, pad(h, 2))
    .replace(/{h}/g, String(h))
    .replace(/{mm}/g, pad(m, 2))
    .replace(/{m}/g, String(m))
    .replace(/{ss}/g, pad(s, 2))
    .replace(/{s}/g, String(s))
    .replace(/{SSS}/g, pad(S, 3))
    .replace(/{S}/g, String(S));
}

export function roundTime(date, unit, isUTC) {
  const d = parseDate(date);
  const utc = !!isUTC;
  const set = (method, value) => {
    d[utc ? `setUTC${method}` : `set${method}`](value);
  };
  switch (unit) {
    case 'year':
      set('Month', 0);
      set('Date', 1);
      set('Hours', 0);
      set('Minutes', 0);
      set('Seconds', 0);
      set('Milliseconds', 0);
      break;
    case 'month':
      set('Date', 1);
      set('Hours', 0);
      set('Minutes', 0);
      set('Seconds', 0);
      set('Milliseconds', 0);
      break;
    case 'day':
      set('Hours', 0);
      set('Minutes', 0);
      set('Seconds', 0);
      set('Milliseconds', 0);
      break;
    case 'hour':
      set('Minutes', 0);
      set('Seconds', 0);
      set('Milliseconds', 0);
      break;
    case 'minute':
      set('Seconds', 0);
      set('Milliseconds', 0);
      break;
    case 'second':
      set('Milliseconds', 0);
      break;
    default:
      break;
  }
  return d;
}
