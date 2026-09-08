import { native } from './native.js';

/** @type {Map<number, ZRender>} */
const instances = new Map();

/** @type {Map<string, unknown>} */
const painters = new Map();

export const version = '6.1.0';

/**
 * 仅记录 painter。非 canvas 忽略（后端固定离屏 canvas）。
 */
export function registerPainter(name, Ctor) {
  painters.set(name, Ctor);
}

export function getRegisteredPainter(name) {
  return painters.get(name);
}

function toNative(zr) {
  return zr && zr._native ? zr._native : zr;
}

export class ZRender {
  constructor(handle) {
    this._native = handle;
  }

  get id() {
    return this._native.id;
  }

  get handler() {
    return this._native.handler;
  }

  get animation() {
    return this._native.animation;
  }

  width() {
    return this._native.width();
  }

  height() {
    return this._native.height();
  }

  getWidth() {
    return this._native.getWidth();
  }

  getHeight() {
    return this._native.getHeight();
  }

  dpr() {
    return this._native.dpr();
  }

  add(el) {
    this._native.add(el);
    return this;
  }

  remove(el) {
    this._native.remove(el);
    return this;
  }

  clear() {
    if (this._native && typeof this._native.clear === 'function') {
      this._native.clear();
    }
    return this;
  }

  dispose() {
    const handle = this._native;
    if (!handle) {
      return;
    }
    const id = handle.id;
    instances.delete(id);
    if (typeof handle.dispose === 'function') {
      handle.dispose();
    }
    this._native = null;
  }

  setBackgroundColor(color) {
    this._backgroundColor = color;
    if (this._native && typeof this._native.setBackgroundColor === 'function') {
      this._native.setBackgroundColor(color);
    }
    return this;
  }

  getBackgroundColor() {
    if (this._backgroundColor !== undefined) {
      return this._backgroundColor;
    }
    if (this._native && typeof this._native.getBackgroundColor === 'function') {
      return this._native.getBackgroundColor();
    }
    return undefined;
  }

  trigger(event, packet) {
    this._native.trigger(event, packet);
    return this;
  }

  refresh() {
    return this._native.refresh();
  }

  flush() {
    return this._native.flush();
  }

  resize(opts) {
    this._native.resize(opts);
    return this;
  }

  findHover(x, y) {
    return this._native.findHover(x, y);
  }

  on(event, handler) {
    this._native.on(event, handler);
    return this;
  }

  off(event, handler) {
    this._native.off(event, handler);
    return this;
  }

  setCursorStyle(cursorStyle) {
    this._cursorStyle = cursorStyle || 'default';
    if (this._native && typeof this._native.setCursorStyle === 'function') {
      this._native.setCursorStyle(this._cursorStyle);
    }
    return this;
  }

  configLayer(zLevel, config) {
    if (this._native && typeof this._native.configLayer === 'function') {
      this._native.configLayer(zLevel, config);
    } else if (this._native && typeof this._native.refresh === 'function') {
      this._native.refresh();
    }
    return this;
  }
}

function wrap(handle) {
  if (handle == null) {
    return undefined;
  }
  const id = handle.id;
  let zr = instances.get(id);
  if (!zr) {
    zr = new ZRender(handle);
    instances.set(id, zr);
  }
  return zr;
}

export function init(dom, opts) {
  return wrap(native.init(dom, opts));
}

export function dispose(zr) {
  if (zr && typeof zr.dispose === 'function' && zr._native) {
    zr.dispose();
    return;
  }
  const handle = toNative(zr);
  if (handle && handle.id != null) {
    instances.delete(handle.id);
  }
  if (handle) {
    native.dispose(handle);
  }
}

export function disposeAll() {
  instances.clear();
  native.disposeAll();
}

export function getInstance(id) {
  const cached = instances.get(id);
  if (cached) {
    return cached;
  }
  return wrap(native.getInstance(id));
}
