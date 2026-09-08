import { Element, nativeHandle } from './element.js';
import { native } from './native.js';

/**
 * 官方 Group extends Element（不是 Displayable）。
 */
export class Group extends Element {
  constructor(opts) {
    super();
    this.isGroup = true;
    this._children = [];
    this._bindNative(new native.Group());
    if (opts != null) {
      this.attr(opts);
    }
  }

  get type() {
    return this._native ? this._native.type : 'group';
  }

  childrenRef() {
    return this._children;
  }

  children() {
    return this._children.slice();
  }

  childAt(idx) {
    return this._children[idx];
  }

  childOfName(name) {
    const children = this._children;
    for (let i = 0; i < children.length; i++) {
      if (children[i].name === name) {
        return children[i];
      }
    }
    return undefined;
  }

  childCount() {
    return this._children.length;
  }

  add(child) {
    if (child && child !== this && child.parent !== this) {
      detachFromParent(child);
      this._children.push(child);
      child.parent = this;
      if (this._native) {
        this._native.add(nativeHandle(child));
      }
    }
    return this;
  }

  addBefore(child, nextSibling) {
    if (
      child &&
      child !== this &&
      child.parent !== this &&
      nextSibling &&
      nextSibling.parent === this
    ) {
      const idx = this._children.indexOf(nextSibling);
      if (idx >= 0) {
        detachFromParent(child);
        this._children.splice(idx, 0, child);
        child.parent = this;
        if (this._native && typeof this._native.addBefore === 'function') {
          this._native.addBefore(nativeHandle(child), nativeHandle(nextSibling));
        }
      }
    }
    return this;
  }

  replace(oldChild, newChild) {
    const idx = this._children.indexOf(oldChild);
    if (idx >= 0) {
      this.replaceAt(newChild, idx);
    }
    return this;
  }

  replaceAt(child, index) {
    const children = this._children;
    const old = children[index];
    if (child && child !== this && child.parent !== this && child !== old) {
      detachFromParent(child);
      children[index] = child;
      if (old) {
        old.parent = null;
      }
      child.parent = this;
      if (this._native && typeof this._native.replace === 'function' && old) {
        this._native.replace(nativeHandle(old), nativeHandle(child));
      }
    }
    return this;
  }

  remove(child) {
    const idx = this._children.indexOf(child);
    if (idx < 0) {
      return this;
    }
    this._children.splice(idx, 1);
    if (child) {
      child.parent = null;
    }
    if (this._native) {
      this._native.remove(nativeHandle(child));
    }
    return this;
  }

  removeAll() {
    const children = this._children;
    for (let i = 0; i < children.length; i++) {
      children[i].parent = null;
    }
    children.length = 0;
    if (this._native && typeof this._native.removeAll === 'function') {
      this._native.removeAll();
    }
    return this;
  }

  eachChild(cb, context) {
    const children = this._children;
    for (let i = 0; i < children.length; i++) {
      cb.call(context, children[i], i);
    }
    return this;
  }

  traverse(cb, context) {
    for (let i = 0; i < this._children.length; i++) {
      const child = this._children[i];
      const stopped = cb.call(context, child);
      if (child.isGroup && !stopped) {
        child.traverse(cb, context);
      }
    }
    return this;
  }
}

function detachFromParent(child) {
  if (child && child.parent) {
    child.parent.remove(child);
  }
}

Group.prototype.isGroup = true;
