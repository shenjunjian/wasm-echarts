/**
 * 通用工具（官方 util 命名空间）。按签名重写。
 */
import { platformApi } from './platform.js';

const objToString = Object.prototype.toString;
const hasOwnProp = Object.prototype.hasOwnProperty;
const protoKey = '__proto__';
const primitiveKey = '__ec_primitive__';
const nativeSlice = Array.prototype.slice;

const BUILTIN = {
  '[object Function]': true,
  '[object RegExp]': true,
  '[object Date]': true,
  '[object Error]': true,
  '[object CanvasGradient]': true,
  '[object CanvasPattern]': true,
  '[object Image]': true,
  '[object Canvas]': true,
};

const TYPED = {
  '[object Int8Array]': true,
  '[object Uint8Array]': true,
  '[object Uint8ClampedArray]': true,
  '[object Int16Array]': true,
  '[object Uint16Array]': true,
  '[object Int32Array]': true,
  '[object Uint32Array]': true,
  '[object Float32Array]': true,
  '[object Float64Array]': true,
};

let idStart = 0x0907;
const MAX_SAFE_INTEGER = Math.pow(2, 53) - 1;

export function guid() {
  if (idStart >= MAX_SAFE_INTEGER) {
    idStart = 0;
  }
  return idStart++;
}

export function logError() {
  if (typeof console !== 'undefined') {
    console.error.apply(console, arguments);
  }
}

export function isArray(value) {
  return Array.isArray ? Array.isArray(value) : objToString.call(value) === '[object Array]';
}

export function isFunction(value) {
  return typeof value === 'function';
}

export function isString(value) {
  return typeof value === 'string';
}

export function isStringSafe(value) {
  return objToString.call(value) === '[object String]';
}

export function isNumber(value) {
  return typeof value === 'number';
}

export function isObject(value) {
  const type = typeof value;
  return type === 'function' || (!!value && type === 'object');
}

export function isBuiltInObject(value) {
  return !!BUILTIN[objToString.call(value)];
}

export function isTypedArray(value) {
  return !!TYPED[objToString.call(value)];
}

export function isDom(value) {
  return (
    typeof value === 'object' &&
    value != null &&
    typeof value.nodeType === 'number' &&
    typeof value.ownerDocument === 'object'
  );
}

export function isGradientObject(value) {
  return value != null && value.colorStops != null;
}

export function isImagePatternObject(value) {
  return value != null && value.image != null;
}

export function isRegExp(value) {
  return objToString.call(value) === '[object RegExp]';
}

export function eqNaN(value) {
  return value !== value;
}

export function isArrayLike(data) {
  if (!data || typeof data === 'string') {
    return false;
  }
  return typeof data.length === 'number';
}

export function hasOwn(own, prop) {
  return hasOwnProp.call(own, prop);
}

export function setAsPrimitive(obj) {
  obj[primitiveKey] = true;
}

export function isPrimitive(obj) {
  return !!(obj && obj[primitiveKey]);
}

export function clone(source) {
  if (source == null || typeof source !== 'object') {
    return source;
  }
  const typeStr = objToString.call(source);
  if (typeStr === '[object Array]') {
    if (isPrimitive(source)) {
      return source;
    }
    const result = [];
    for (let i = 0; i < source.length; i++) {
      result[i] = clone(source[i]);
    }
    return result;
  }
  if (TYPED[typeStr]) {
    if (isPrimitive(source)) {
      return source;
    }
    const Ctor = source.constructor;
    if (Ctor.from) {
      return Ctor.from(source);
    }
    const copied = new Ctor(source.length);
    for (let i = 0; i < source.length; i++) {
      copied[i] = source[i];
    }
    return copied;
  }
  if (!BUILTIN[typeStr] && !isPrimitive(source) && !isDom(source)) {
    const result = {};
    for (const key in source) {
      if (hasOwn(source, key) && key !== protoKey) {
        result[key] = clone(source[key]);
      }
    }
    return result;
  }
  return source;
}

export function merge(target, source, overwrite) {
  if (!isObject(source) || !isObject(target)) {
    return overwrite ? clone(source) : target;
  }
  for (const key in source) {
    if (hasOwn(source, key) && key !== protoKey) {
      const targetProp = target[key];
      const sourceProp = source[key];
      if (
        isObject(sourceProp) &&
        isObject(targetProp) &&
        !isArray(sourceProp) &&
        !isArray(targetProp) &&
        !isDom(sourceProp) &&
        !isDom(targetProp) &&
        !isBuiltInObject(sourceProp) &&
        !isBuiltInObject(targetProp) &&
        !isPrimitive(sourceProp) &&
        !isPrimitive(targetProp)
      ) {
        merge(targetProp, sourceProp, overwrite);
      } else if (overwrite || !(key in target)) {
        target[key] = clone(source[key]);
      }
    }
  }
  return target;
}

export function mergeAll(targetAndSources, overwrite) {
  let result = targetAndSources[0];
  for (let i = 1; i < targetAndSources.length; i++) {
    result = merge(result, targetAndSources[i], overwrite);
  }
  return result;
}

export function extend(target, source) {
  if (Object.assign) {
    Object.assign(target, source);
  } else {
    for (const key in source) {
      if (hasOwn(source, key) && key !== protoKey) {
        target[key] = source[key];
      }
    }
  }
  return target;
}

export function assignProps(tar, src, props) {
  tar = tar || {};
  for (let i = 0; i < props.length; i++) {
    const prop = props[i];
    tar[prop] = src[prop];
  }
  return tar;
}

export function keys(obj) {
  if (!obj) {
    return [];
  }
  if (Object.keys) {
    return Object.keys(obj);
  }
  const list = [];
  for (const key in obj) {
    if (hasOwn(obj, key)) {
      list.push(key);
    }
  }
  return list;
}

export function defaults(target, source, overlay) {
  const keyList = keys(source);
  for (let i = 0; i < keyList.length; i++) {
    const key = keyList[i];
    if (overlay ? source[key] != null : target[key] == null) {
      target[key] = source[key];
    }
  }
  return target;
}

export function createCanvas() {
  return platformApi.createCanvas();
}

export function indexOf(array, value) {
  if (!array) {
    return -1;
  }
  if (array.indexOf) {
    return array.indexOf(value);
  }
  for (let i = 0; i < array.length; i++) {
    if (array[i] === value) {
      return i;
    }
  }
  return -1;
}

export function inherits(clazz, baseClazz) {
  const proto = clazz.prototype;
  function F() {}
  F.prototype = baseClazz.prototype;
  clazz.prototype = new F();
  for (const prop in proto) {
    if (hasOwn(proto, prop)) {
      clazz.prototype[prop] = proto[prop];
    }
  }
  clazz.prototype.constructor = clazz;
  clazz.superClass = baseClazz;
}

export function mixin(target, source, override) {
  target = 'prototype' in target ? target.prototype : target;
  source = 'prototype' in source ? source.prototype : source;
  if (Object.getOwnPropertyNames) {
    const keyList = Object.getOwnPropertyNames(source);
    for (let i = 0; i < keyList.length; i++) {
      const key = keyList[i];
      if (key !== 'constructor') {
        if (override ? source[key] != null : target[key] == null) {
          target[key] = source[key];
        }
      }
    }
  } else {
    defaults(target, source, override);
  }
}

export function each(arr, cb, context) {
  if (!(arr && cb)) {
    return;
  }
  if (arr.forEach && arr.forEach === Array.prototype.forEach) {
    arr.forEach(cb, context);
  } else if (arr.length === +arr.length) {
    for (let i = 0; i < arr.length; i++) {
      cb.call(context, arr[i], i, arr);
    }
  } else {
    for (const key in arr) {
      if (hasOwn(arr, key)) {
        cb.call(context, arr[key], key, arr);
      }
    }
  }
}

export function map(arr, cb, context) {
  if (!arr) {
    return [];
  }
  if (!cb) {
    return slice(arr);
  }
  if (arr.map && arr.map === Array.prototype.map) {
    return arr.map(cb, context);
  }
  const result = [];
  for (let i = 0; i < arr.length; i++) {
    result.push(cb.call(context, arr[i], i, arr));
  }
  return result;
}

export function reduce(arr, cb, memo, context) {
  if (!(arr && cb)) {
    return;
  }
  for (let i = 0; i < arr.length; i++) {
    memo = cb.call(context, memo, arr[i], i, arr);
  }
  return memo;
}

export function filter(arr, cb, context) {
  if (!arr) {
    return [];
  }
  if (!cb) {
    return slice(arr);
  }
  if (arr.filter && arr.filter === Array.prototype.filter) {
    return arr.filter(cb, context);
  }
  const result = [];
  for (let i = 0; i < arr.length; i++) {
    if (cb.call(context, arr[i], i, arr)) {
      result.push(arr[i]);
    }
  }
  return result;
}

export function find(arr, cb, context) {
  if (!(arr && cb)) {
    return;
  }
  for (let i = 0; i < arr.length; i++) {
    if (cb.call(context, arr[i], i, arr)) {
      return arr[i];
    }
  }
}

export function bind(func, ctx) {
  const extra = nativeSlice.call(arguments, 2);
  return function () {
    return func.apply(ctx, extra.concat(nativeSlice.call(arguments)));
  };
}

export function curry(func) {
  const extra = nativeSlice.call(arguments, 1);
  return function () {
    return func.apply(this, extra.concat(nativeSlice.call(arguments)));
  };
}

export function retrieve() {
  for (let i = 0; i < arguments.length; i++) {
    if (arguments[i] != null) {
      return arguments[i];
    }
  }
}

export function retrieve2(value0, value1) {
  return value0 != null ? value0 : value1;
}

export function retrieve3(value0, value1, value2) {
  return value0 != null ? value0 : value1 != null ? value1 : value2;
}

export function slice(arr) {
  return nativeSlice.apply(arr, nativeSlice.call(arguments, 1));
}

export function normalizeCssArray(val) {
  if (typeof val === 'number') {
    return [val, val, val, val];
  }
  const n = val.length;
  if (n === 2) {
    return [val[0], val[1], val[0], val[1]];
  }
  if (n === 3) {
    return [val[0], val[1], val[2], val[1]];
  }
  return val;
}

export function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

export function trim(str) {
  if (str == null) {
    return null;
  }
  return typeof str.trim === 'function'
    ? str.trim()
    : str.replace(/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g, '');
}

export class HashMap {
  constructor(obj) {
    this.data = new Map();
    const isArr = isArray(obj);
    const visit = (value, key) => {
      if (isArr) {
        this.set(value, key);
      } else {
        this.set(key, value);
      }
    };
    if (obj instanceof HashMap) {
      obj.each(visit);
    } else if (obj) {
      each(obj, visit);
    }
  }

  hasKey(key) {
    return this.data.has(key);
  }

  get(key) {
    return this.data.get(key);
  }

  set(key, value) {
    this.data.set(key, value);
    return value;
  }

  each(cb, context) {
    this.data.forEach((value, key) => {
      cb.call(context, value, key);
    });
  }

  keys() {
    return Array.from(this.data.keys());
  }

  removeKey(key) {
    this.data.delete(key);
  }
}

export function createHashMap(obj) {
  return new HashMap(obj);
}

export function concatArray(a, b) {
  const Ctor = a.constructor;
  const out = new Ctor(a.length + b.length);
  for (let i = 0; i < a.length; i++) {
    out[i] = a[i];
  }
  for (let i = 0; i < b.length; i++) {
    out[i + a.length] = b[i];
  }
  return out;
}

export function createObject(proto, properties) {
  let obj;
  if (Object.create) {
    obj = Object.create(proto);
  } else {
    function Ctor() {}
    Ctor.prototype = proto;
    obj = new Ctor();
  }
  if (properties) {
    extend(obj, properties);
  }
  return obj;
}

export function disableUserSelect(dom) {
  if (!dom || !dom.style) {
    return;
  }
  const style = dom.style;
  style.webkitUserSelect = 'none';
  style.userSelect = 'none';
  style.webkitTapHighlightColor = 'rgba(0,0,0,0)';
  style['-webkit-touch-callout'] = 'none';
}

export function noop() {}

export const RADIAN_TO_DEGREE = 180 / Math.PI;
export const EPSILON = Number.EPSILON || Math.pow(2, -52);
