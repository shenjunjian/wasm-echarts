/**
 * 对齐官方 Animator：不播中间帧。
 * `when` 只保留最后一帧；`start()` 立刻写入目标属性，再调用 during(percent=1) 与 done。
 */
export class Animator {
  constructor(el, path, looping) {
    this._el = el;
    this._path = path;
    this._loop = !!looping;
    this._lastProps = null;
    this._during = [];
    this._done = [];
  }

  when(_time, props) {
    if (props != null && typeof props === 'object') {
      this._lastProps = props;
    }
    return this;
  }

  during(cb) {
    if (typeof cb === 'function') {
      this._during.push(cb);
    }
    return this;
  }

  done(cb) {
    if (typeof cb === 'function') {
      this._done.push(cb);
    }
    return this;
  }

  delay(_time) {
    return this;
  }

  start(_easing) {
    const el = this._el;
    const path = this._path;
    const props = this._lastProps;
    if (el && props) {
      applyEndState(el, path, props);
    }
    const target = animationTarget(el, path);
    for (let i = 0; i < this._during.length; i++) {
      this._during[i](target, 1);
    }
    for (let i = 0; i < this._done.length; i++) {
      this._done[i]();
    }
    return this;
  }

  stop() {}
}

function animationTarget(el, path) {
  if (!el) {
    return el;
  }
  if (path && path !== '') {
    return el[path] != null ? el[path] : el;
  }
  return el;
}

function applyEndState(el, path, props) {
  if (path === 'shape' && typeof el.setShape === 'function') {
    el.setShape(props);
    return;
  }
  if (path === 'style' && typeof el.setStyle === 'function') {
    el.setStyle(props);
    return;
  }
  if (!path || path === '') {
    if (typeof el.attr === 'function') {
      el.attr(props);
    }
    return;
  }
  const current = el[path];
  if (current && typeof current === 'object' && !Array.isArray(props)) {
    Object.assign(current, props);
    if (typeof el.attr === 'function') {
      el.attr(path, current);
    }
  } else if (typeof el.attr === 'function') {
    el.attr(path, props);
  }
}
