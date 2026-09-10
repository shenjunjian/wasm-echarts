import { Displayable } from './displayable.js';
import { nativeHandle } from './element.js';
import { native } from './native-core.js';

/**
 * 按普通 Group 语义落地：可构造、可 addDisplayable，不做增量图层。
 */
export class IncrementalDisplayable extends Displayable {
  constructor(opts) {
    super();
    this._displayables = [];
    this._temporaryDisplayables = [];
    this._children = [];
    this._cursor = 0;
    this.notClear = true;
    this.incremental = true;
    this.isGroup = true;
    this._bindNative(new native.Group());
    if (opts != null) {
      this.attr(opts);
    }
  }

  get type() {
    return 'incremental';
  }

  addDisplayable(displayable, notPersistent) {
    if (!displayable) {
      return this;
    }
    if (notPersistent) {
      this._temporaryDisplayables.push(displayable);
    } else {
      this._displayables.push(displayable);
    }
    attachChild(this, displayable);
    return this;
  }

  addDisplayables(displayables, notPersistent) {
    const list = displayables || [];
    for (let i = 0; i < list.length; i++) {
      this.addDisplayable(list[i], notPersistent);
    }
    return this;
  }

  getDisplayables() {
    return this._displayables;
  }

  getTemporalDisplayables() {
    return this._temporaryDisplayables;
  }

  clearDisplaybles() {
    return this.clearDisplayables();
  }

  clearDisplayables() {
    const children = this._children.slice();
    for (let i = 0; i < children.length; i++) {
      detachChild(this, children[i]);
    }
    this._displayables = [];
    this._temporaryDisplayables = [];
    this._cursor = 0;
    this.notClear = false;
    return this;
  }

  clearTemporalDisplayables() {
    const temps = this._temporaryDisplayables.slice();
    this._temporaryDisplayables = [];
    for (let i = 0; i < temps.length; i++) {
      detachChild(this, temps[i]);
    }
    return this;
  }

  eachPendingDisplayable(cb) {
    if (typeof cb !== 'function') {
      return this;
    }
    for (let i = this._cursor; i < this._displayables.length; i++) {
      cb(this._displayables[i]);
    }
    for (let i = 0; i < this._temporaryDisplayables.length; i++) {
      cb(this._temporaryDisplayables[i]);
    }
    return this;
  }

  getCursor() {
    return this._cursor;
  }

  innerAfterBrush() {
    this._cursor = this._displayables.length;
  }

  traverse(cb, context) {
    cb.call(context, this);
    return this;
  }
}

function attachChild(host, child) {
  if (!child || child === host || child.parent === host) {
    return;
  }
  if (child.parent && typeof child.parent.remove === 'function') {
    child.parent.remove(child);
  }
  host._children.push(child);
  child.parent = host;
  if (host._native && typeof host._native.add === 'function') {
    host._native.add(nativeHandle(child));
  }
}

function detachChild(host, child) {
  const idx = host._children.indexOf(child);
  if (idx >= 0) {
    host._children.splice(idx, 1);
  }
  if (child && child.parent === host) {
    child.parent = null;
  }
  if (host._native && typeof host._native.remove === 'function') {
    host._native.remove(nativeHandle(child));
  }
}
