import { Animator } from './animator.js';

function hasNativeProp(handle, name) {
  return handle != null && name in handle;
}

function toNumber(value, fallback) {
  const n = Number(value);
  return Number.isFinite(n) ? n : fallback;
}

function asPatch(keyOrObj, value) {
  if (typeof keyOrObj === 'string') {
    return { [keyOrObj]: value };
  }
  return keyOrObj != null ? keyOrObj : {};
}

function nativeHandle(el) {
  return el && el._native ? el._native : el;
}

/**
 * 官方 Element 基类。子类 `super()` 不创建 native，再 `_bindNative(handle)`。
 */
export class Element {
  constructor() {
    this._native = null;
    this._x = 0;
    this._y = 0;
    this._scaleX = 1;
    this._scaleY = 1;
    this._rotation = 0;
    this._originX = 0;
    this._originY = 0;
    this._name = '';
    this._ignore = false;
    this._silent = false;
    this.parent = null;
    this.isGroup = false;
    this.clipPath = null;
    this.states = {};
    this.currentStates = [];
    this._normalState = null;
  }

  _bindNative(handle) {
    this._native = handle;
    return this;
  }

  _syncNative(key, value) {
    if (this._native && typeof this._native.attr === 'function') {
      this._native.attr(key, value);
    }
  }

  get id() {
    return this._native ? this._native.id : undefined;
  }

  get type() {
    return this._native ? this._native.type : 'element';
  }

  get x() {
    return this._x;
  }

  set x(value) {
    this._x = toNumber(value, 0);
    this._syncNative('x', this._x);
  }

  get y() {
    return this._y;
  }

  set y(value) {
    this._y = toNumber(value, 0);
    this._syncNative('y', this._y);
  }

  get scaleX() {
    return this._scaleX;
  }

  set scaleX(value) {
    this._scaleX = toNumber(value, 1);
    this._syncNative('scaleX', this._scaleX);
  }

  get scaleY() {
    return this._scaleY;
  }

  set scaleY(value) {
    this._scaleY = toNumber(value, 1);
    this._syncNative('scaleY', this._scaleY);
  }

  get rotation() {
    return this._rotation;
  }

  set rotation(value) {
    this._rotation = toNumber(value, 0);
    this._syncNative('rotation', this._rotation);
  }

  get originX() {
    return this._originX;
  }

  set originX(value) {
    this._originX = toNumber(value, 0);
    this._syncNative('originX', this._originX);
  }

  get originY() {
    return this._originY;
  }

  set originY(value) {
    this._originY = toNumber(value, 0);
    this._syncNative('originY', this._originY);
  }

  get name() {
    return this._name;
  }

  set name(value) {
    this._name = value == null ? '' : String(value);
    this._syncNative('name', this._name);
  }

  get ignore() {
    return this._ignore;
  }

  set ignore(value) {
    this._ignore = !!value;
    this._syncNative('ignore', this._ignore);
  }

  get silent() {
    return this._silent;
  }

  set silent(value) {
    this._silent = !!value;
    this._syncNative('silent', this._silent);
  }

  get draggable() {
    return hasNativeProp(this._native, 'draggable') ? this._native.draggable : false;
  }

  set draggable(value) {
    if (hasNativeProp(this._native, 'draggable')) {
      this._native.draggable = value;
    }
  }

  get position() {
    return [this.x, this.y];
  }

  set position(value) {
    if (value && value.length >= 2) {
      this._x = toNumber(value[0], 0);
      this._y = toNumber(value[1], 0);
      this._syncNative('position', [this._x, this._y]);
    }
  }

  attrKV(key, value) {
    if (key === 'clipPath') {
      if (!value) {
        this.removeClipPath();
      } else {
        this.setClipPath(value);
      }
    } else {
      this[key] = value;
    }
  }

  attr(keyOrObj, value) {
    if (typeof keyOrObj === 'string') {
      this.attrKV(keyOrObj, value);
    } else if (keyOrObj && typeof keyOrObj === 'object') {
      const keys = Object.keys(keyOrObj);
      for (let i = 0; i < keys.length; i++) {
        const key = keys[i];
        this.attrKV(key, keyOrObj[key]);
      }
    }
    return this;
  }

  setClipPath(clip) {
    this.clipPath = clip;
    if (this._native && typeof this._native.setClipPath === 'function') {
      this._native.setClipPath(nativeHandle(clip));
    }
    return this;
  }

  getClipPath() {
    return this.clipPath;
  }

  removeClipPath() {
    this.clipPath = null;
    if (this._native && typeof this._native.removeClipPath === 'function') {
      this._native.removeClipPath();
    }
    return this;
  }

  hasState() {
    return this.currentStates.length > 0;
  }

  getState(name) {
    return this.states[name];
  }

  ensureState(name) {
    if (!this.states[name]) {
      this.states[name] = {};
    }
    return this.states[name];
  }

  useState(stateName, keepCurrentStates) {
    if (!stateName) {
      return this.clearStates();
    }
    if (keepCurrentStates && this.currentStates.indexOf(stateName) < 0) {
      return this.useStates(this.currentStates.concat(stateName));
    }
    return this.useStates([stateName]);
  }

  useStates(states) {
    const names = !states ? [] : Array.isArray(states) ? states.slice() : [states];
    if (!names.length) {
      return this.clearStates();
    }
    if (!this._normalState) {
      this._normalState = captureNormalState(this);
    } else {
      restoreNormalState(this, this._normalState);
    }
    const merged = {};
    for (let i = 0; i < names.length; i++) {
      const stateObj = this.states[names[i]];
      if (stateObj && typeof stateObj === 'object') {
        mergeState(merged, stateObj);
      }
    }
    applyStateObj(this, merged);
    this.currentStates = names;
    if (this._native && typeof this._native.useStates === 'function') {
      this._native.useStates(names);
    } else if (this._native && typeof this._native.useState === 'function' && names.length === 1) {
      this._native.useState(names[0]);
    }
    return this;
  }

  clearStates() {
    if (this._normalState) {
      restoreNormalState(this, this._normalState);
    }
    this.currentStates = [];
    if (this._native && typeof this._native.useState === 'function') {
      this._native.useState('');
    }
    return this;
  }

  removeState(state) {
    const idx = this.currentStates.indexOf(state);
    if (idx >= 0) {
      const next = this.currentStates.slice();
      next.splice(idx, 1);
      this.useStates(next);
    }
    return this;
  }

  toggleState(state, enable) {
    if (enable) {
      this.useState(state, true);
    } else {
      this.removeState(state);
    }
    return this;
  }

  animate(path, looping) {
    return new Animator(this, path, looping);
  }

  animateTo(target, cfg) {
    if (target != null && typeof target === 'object') {
      this.attr(target);
    }
    const opts = cfg && typeof cfg === 'object' ? cfg : {};
    if (typeof opts.during === 'function') {
      opts.during(this, 1);
    }
    if (typeof opts.done === 'function') {
      opts.done();
    }
    return this;
  }

  hide() {
    this.ignore = true;
  }

  show() {
    this.ignore = false;
  }

  on(event, handler) {
    if (this._native && typeof this._native.on === 'function') {
      this._native.on(event, handler);
    }
    return this;
  }

  off(event, handler) {
    if (this._native && typeof this._native.off === 'function') {
      this._native.off(event, handler);
    }
    return this;
  }

  trigger(event, packet) {
    if (this._native && typeof this._native.trigger === 'function') {
      this._native.trigger(event, packet);
    }
    return this;
  }

  getBoundingRect() {
    if (this._native && typeof this._native.getBoundingRect === 'function') {
      return this._native.getBoundingRect();
    }
    return undefined;
  }
}

function captureNormalState(el) {
  return {
    x: el.x,
    y: el.y,
    scaleX: el.scaleX,
    scaleY: el.scaleY,
    rotation: el.rotation,
    originX: el.originX,
    originY: el.originY,
    z: el.z,
    z2: el.z2,
    zlevel: el.zlevel,
    silent: el.silent,
    ignore: el.ignore,
    style: el.style ? { ...el.style } : undefined,
    shape: el.shape ? { ...el.shape } : undefined,
  };
}

function restoreNormalState(el, normal) {
  applyStateObj(el, normal);
}

function mergeState(target, source) {
  const keys = Object.keys(source);
  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    const value = source[key];
    if (value && typeof value === 'object' && !Array.isArray(value) && target[key] && typeof target[key] === 'object') {
      target[key] = { ...target[key], ...value };
    } else {
      target[key] = value;
    }
  }
  return target;
}

function applyStateObj(el, state) {
  if (!state) {
    return;
  }
  if (state.style && typeof el.setStyle === 'function') {
    el.setStyle(state.style);
  }
  if (state.shape && typeof el.setShape === 'function') {
    el.setShape(state.shape);
  }
  const skip = { style: true, shape: true };
  const keys = Object.keys(state);
  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    if (!skip[key] && state[key] !== undefined) {
      el.attrKV(key, state[key]);
    }
  }
}

export { asPatch, nativeHandle, toNumber };
