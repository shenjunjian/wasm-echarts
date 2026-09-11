/**
 * 可注入的 wasm-bindgen 模块。standalone 由 native.js 注入 wasm-zrender/pkg；
 * echarts 页由 wasm-echarts/js/native.js 注入 wasm-echarts/pkg。不要在此静态 import pkg。
 *
 * Vite 生产分包可能先求值 bounding_rect / point，再求值 native.js。
 * 顶层不要读 native.Xxx；需要类时用 lazyNativeClass。
 */

let _native = null;

export function setNative(mod) {
  if (mod == null) {
    throw new Error('[wasm-zrender] setNative requires a wasm-bindgen module');
  }
  _native = mod;
}

export function getNative() {
  if (_native == null) {
    throw new Error(
      '[wasm-zrender] native 未注入。zrender 文档站请从 @wasm-zrender 入口导入；echarts 页请先加载 @wasm-echarts。',
    );
  }
  return _native;
}

export const native = new Proxy(
  {},
  {
    get(_target, prop) {
      if (prop === '__esModule') {
        return true;
      }
      return getNative()[prop];
    },
    has(_target, prop) {
      return prop in getNative();
    },
  },
);

/**
 * 延迟解析 native 构造器，避免模块求值时触发 getNative。
 * `patch` 在首次实际使用时对真正的 wasm-bindgen 类打补丁。
 * @param {string} name
 * @param {(Ctor: Function) => void} [patch]
 */
export function lazyNativeClass(name, patch) {
  let patched = false;

  function resolve() {
    const Ctor = getNative()[name];
    if (typeof Ctor !== 'function') {
      throw new Error(`[wasm-zrender] native.${name} 不是构造器`);
    }
    if (patch && !patched) {
      patch(Ctor);
      patched = true;
    }
    return Ctor;
  }

  function LazyCtor(...args) {
    return new (resolve())(...args);
  }
  Object.defineProperty(LazyCtor, 'name', { value: name, configurable: true });

  return new Proxy(LazyCtor, {
    construct(_target, args) {
      return new (resolve())(...args);
    },
    get(target, prop, receiver) {
      if (prop === '__esModule') {
        return false;
      }
      if (prop === 'name' || prop === 'length') {
        return Reflect.get(target, prop, receiver);
      }
      const Ctor = resolve();
      if (prop === 'prototype') {
        return Ctor.prototype;
      }
      const value = Ctor[prop];
      return typeof value === 'function' ? value.bind(Ctor) : value;
    },
    set(_target, prop, value) {
      resolve()[prop] = value;
      return true;
    },
    apply(_target, thisArg, args) {
      return resolve().apply(thisArg, args);
    },
    has(_target, prop) {
      return prop in resolve();
    },
    getPrototypeOf() {
      return resolve();
    },
  });
}
