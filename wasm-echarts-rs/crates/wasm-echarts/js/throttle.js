/**
 * echarts.throttle：按官方 export/api.ts 与 util/throttle.ts 签名重写。
 * throttle(fn, delay?, debounce?) 返回带 clear / debounceNextCall 的函数。
 */

/**
 * @param {Function} fn
 * @param {number} [delay=0]
 * @param {boolean} [debounce=false]
 * @returns {Function & { clear(): void, debounceNextCall(ms: number): void }}
 */
export function throttle(fn, delay, debounce) {
  let lastCall = 0;
  let lastExec = 0;
  let timer = null;
  let scope;
  let args;
  let debounceNextCall;
  delay = delay || 0;

  function exec() {
    lastExec = Date.now();
    timer = null;
    fn.apply(scope, args || []);
  }

  function cb(...cbArgs) {
    const now = Date.now();
    scope = this;
    args = cbArgs;
    const thisDelay = debounceNextCall || delay;
    const thisDebounce = debounceNextCall || debounce;
    debounceNextCall = null;
    const diff = now - (thisDebounce ? lastCall : lastExec) - thisDelay;
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
    if (thisDebounce) {
      timer = setTimeout(exec, thisDelay);
    } else if (diff >= 0) {
      exec();
    } else {
      timer = setTimeout(exec, -diff);
    }
    lastCall = now;
  }

  cb.clear = function clear() {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
  };

  cb.debounceNextCall = function debounceNext(ms) {
    debounceNextCall = ms;
  };

  return cb;
}
