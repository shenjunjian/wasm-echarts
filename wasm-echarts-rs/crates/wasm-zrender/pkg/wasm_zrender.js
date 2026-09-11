/* @ts-self-types="./wasm_zrender.d.ts" */

//#region exports

/**
 * 对齐 `zr.animation`：无帧循环；`on('frame')` 为空操作。
 */
export class Animation {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(Animation.prototype);
        obj.__wbg_ptr = ptr;
        AnimationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        AnimationFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_animation_free(ptr, 0);
    }
    /**
     * @param {any} _event
     * @param {any} _handler
     */
    off(_event, _handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.animation_off(this.__wbg_ptr, _event, _handler);
    }
    /**
     * @param {string} _event
     * @param {any} _handler
     */
    on(_event, _handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(_event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.animation_on(this.__wbg_ptr, ptr0, len0, _handler);
    }
}
if (Symbol.dispose) Animation.prototype[Symbol.dispose] = Animation.prototype.free;

/**
 * 对齐 zrender `Animator`：终态语义。
 */
export class Animator {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(Animator.prototype);
        obj.__wbg_ptr = ptr;
        AnimatorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        AnimatorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_animator_free(ptr, 0);
    }
    /**
     * @param {number} _time
     * @returns {Animator}
     */
    delay(_time) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.animator_delay(this.__wbg_ptr, _time);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} cb
     * @returns {Animator}
     */
    done(cb) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.animator_done(this.__wbg_ptr, cb);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} cb
     * @returns {Animator}
     */
    during(cb) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.animator_during(this.__wbg_ptr, cb);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} _easing
     * @returns {Animator}
     */
    start(_easing) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.animator_start(this.__wbg_ptr, _easing);
        return Animator.__wrap(ret);
    }
    stop() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.animator_stop(this.__wbg_ptr);
    }
    /**
     * @param {number} _time
     * @param {any} props
     * @returns {Animator}
     */
    when(_time, props) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.animator_when(this.__wbg_ptr, _time, props);
        return Animator.__wrap(ret);
    }
}
if (Symbol.dispose) Animator.prototype[Symbol.dispose] = Animator.prototype.free;

export class Arc {
    static __wrap(ptr) {
        const obj = Object.create(Arc.prototype);
        obj.__wbg_ptr = ptr;
        ArcFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ArcFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_arc_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Arc}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_attr(this.__wbg_ptr, key, value);
        return Arc.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.arc_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.arc_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        ArcFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Arc}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_off(this.__wbg_ptr, event, handler);
        return Arc.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Arc}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.arc_on(this.__wbg_ptr, ptr0, len0, handler);
        return Arc.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Arc}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_removeClipPath(this.__wbg_ptr);
        return Arc.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Arc}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_setClipPath(this.__wbg_ptr, clip);
        return Arc.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Arc}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_setShape(this.__wbg_ptr, shape);
        return Arc.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.arc_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Arc}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_setStyle(this.__wbg_ptr, style);
        return Arc.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.arc_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.arc_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.arc_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Arc}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.arc_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Arc.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.arc_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.arc_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.arc_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Arc.prototype[Symbol.dispose] = Arc.prototype.free;

export class BezierCurve {
    static __wrap(ptr) {
        const obj = Object.create(BezierCurve.prototype);
        obj.__wbg_ptr = ptr;
        BezierCurveFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BezierCurveFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_beziercurve_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {BezierCurve}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_attr(this.__wbg_ptr, key, value);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.beziercurve_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.beziercurve_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        BezierCurveFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {BezierCurve}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_off(this.__wbg_ptr, event, handler);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {BezierCurve}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.beziercurve_on(this.__wbg_ptr, ptr0, len0, handler);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BezierCurve}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_removeClipPath(this.__wbg_ptr);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {BezierCurve}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_setClipPath(this.__wbg_ptr, clip);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {BezierCurve}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_setShape(this.__wbg_ptr, shape);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.beziercurve_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {BezierCurve}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_setStyle(this.__wbg_ptr, style);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.beziercurve_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.beziercurve_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.beziercurve_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {BezierCurve}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.beziercurve_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return BezierCurve.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.beziercurve_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.beziercurve_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.beziercurve_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) BezierCurve.prototype[Symbol.dispose] = BezierCurve.prototype.free;

export class BoundingRect {
    static __wrap(ptr) {
        const obj = Object.create(BoundingRect.prototype);
        obj.__wbg_ptr = ptr;
        BoundingRectFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BoundingRectFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_boundingrect_free(ptr, 0);
    }
    /**
     * @param {any} matrix
     */
    applyTransform(matrix) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.boundingrect_applyTransform(this.__wbg_ptr, matrix);
    }
    /**
     * @param {BoundingRect} b
     * @returns {Float64Array}
     */
    calculateTransform(b) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(b, BoundingRect);
        if (b.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.boundingrect_calculateTransform(this.__wbg_ptr, b.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {BoundingRect}
     */
    clone() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_clone(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {boolean}
     */
    contain(x, y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_contain(this.__wbg_ptr, x, y);
        return ret !== 0;
    }
    /**
     * @param {any} other
     */
    copy(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.boundingrect_copy(this.__wbg_ptr, other);
    }
    /**
     * @param {any} rect
     * @returns {BoundingRect}
     */
    static create(rect) {
        const ret = wasm.boundingrect_create(rect);
        return BoundingRect.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get height() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_height(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {BoundingRect} other
     * @param {any} opt
     * @returns {boolean}
     */
    intersect(other, opt) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, BoundingRect);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.boundingrect_intersect(this.__wbg_ptr, other.__wbg_ptr, opt);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    isFinite() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_isFinite(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    isZero() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_isZero(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     */
    constructor(x, y, width, height) {
        const ret = wasm.boundingrect_new(x, y, width, height);
        this.__wbg_ptr = ret;
        BoundingRectFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {any}
     */
    plain() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_plain(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {any} target
     * @param {number} x
     * @param {number} y
     * @param {number} width
     * @param {number} height
     */
    static set(target, x, y, width, height) {
        wasm.boundingrect_set(target, x, y, width, height);
    }
    /**
     * @param {number} height
     */
    set height(height) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.boundingrect_set_height(this.__wbg_ptr, height);
    }
    /**
     * @param {number} width
     */
    set width(width) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.boundingrect_set_width(this.__wbg_ptr, width);
    }
    /**
     * @param {number} x
     */
    set x(x) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.boundingrect_set_x(this.__wbg_ptr, x);
    }
    /**
     * @param {number} y
     */
    set y(y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.boundingrect_set_y(this.__wbg_ptr, y);
    }
    /**
     * @param {BoundingRect} other
     */
    union(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, BoundingRect);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        wasm.boundingrect_union(this.__wbg_ptr, other.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get width() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_width(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get x() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.boundingrect_y(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) BoundingRect.prototype[Symbol.dispose] = BoundingRect.prototype.free;

export class Circle {
    static __wrap(ptr) {
        const obj = Object.create(Circle.prototype);
        obj.__wbg_ptr = ptr;
        CircleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CircleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_circle_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Circle}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_attr(this.__wbg_ptr, key, value);
        return Circle.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.circle_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.circle_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        CircleFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Circle}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_off(this.__wbg_ptr, event, handler);
        return Circle.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Circle}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.circle_on(this.__wbg_ptr, ptr0, len0, handler);
        return Circle.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Circle}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_removeClipPath(this.__wbg_ptr);
        return Circle.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Circle}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_setClipPath(this.__wbg_ptr, clip);
        return Circle.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Circle}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_setShape(this.__wbg_ptr, shape);
        return Circle.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.circle_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Circle}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_setStyle(this.__wbg_ptr, style);
        return Circle.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.circle_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.circle_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.circle_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Circle}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.circle_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Circle.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.circle_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.circle_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.circle_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Circle.prototype[Symbol.dispose] = Circle.prototype.free;

export class CompoundPath {
    static __wrap(ptr) {
        const obj = Object.create(CompoundPath.prototype);
        obj.__wbg_ptr = ptr;
        CompoundPathFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CompoundPathFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_compoundpath_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {CompoundPath}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_attr(this.__wbg_ptr, key, value);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.compoundpath_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.compoundpath_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        CompoundPathFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {CompoundPath}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_off(this.__wbg_ptr, event, handler);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {CompoundPath}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.compoundpath_on(this.__wbg_ptr, ptr0, len0, handler);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {CompoundPath}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_removeClipPath(this.__wbg_ptr);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {CompoundPath}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_setClipPath(this.__wbg_ptr, clip);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {CompoundPath}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_setShape(this.__wbg_ptr, shape);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.compoundpath_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {CompoundPath}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_setStyle(this.__wbg_ptr, style);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.compoundpath_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.compoundpath_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.compoundpath_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {CompoundPath}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.compoundpath_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return CompoundPath.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.compoundpath_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.compoundpath_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.compoundpath_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) CompoundPath.prototype[Symbol.dispose] = CompoundPath.prototype.free;

/**
 * Path / Text / Image / Group 等图元的公共 displayable 属性由构造 opts 传入。
 * 本类不可直接实例化，仅用于 API 对齐与文档说明。
 */
export class Displayable {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DisplayableFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_displayable_free(ptr, 0);
    }
    /**
     * @param {any} _opts
     */
    constructor(_opts) {
        const ret = wasm.displayable_new(_opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        DisplayableFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) Displayable.prototype[Symbol.dispose] = Displayable.prototype.free;

export class Droplet {
    static __wrap(ptr) {
        const obj = Object.create(Droplet.prototype);
        obj.__wbg_ptr = ptr;
        DropletFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DropletFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_droplet_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Droplet}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_attr(this.__wbg_ptr, key, value);
        return Droplet.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.droplet_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.droplet_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        DropletFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Droplet}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_off(this.__wbg_ptr, event, handler);
        return Droplet.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Droplet}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.droplet_on(this.__wbg_ptr, ptr0, len0, handler);
        return Droplet.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Droplet}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_removeClipPath(this.__wbg_ptr);
        return Droplet.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Droplet}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_setClipPath(this.__wbg_ptr, clip);
        return Droplet.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Droplet}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_setShape(this.__wbg_ptr, shape);
        return Droplet.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.droplet_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Droplet}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_setStyle(this.__wbg_ptr, style);
        return Droplet.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.droplet_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.droplet_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.droplet_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Droplet}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.droplet_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Droplet.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.droplet_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.droplet_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.droplet_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Droplet.prototype[Symbol.dispose] = Droplet.prototype.free;

class Element2 {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(Element2.prototype);
        obj.__wbg_ptr = ptr;
        Element2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Element2Finalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_element_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.element_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.element_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Element2.prototype[Symbol.dispose] = Element2.prototype.free;
export { Element2 as Element }

export class Ellipse {
    static __wrap(ptr) {
        const obj = Object.create(Ellipse.prototype);
        obj.__wbg_ptr = ptr;
        EllipseFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EllipseFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ellipse_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Ellipse}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_attr(this.__wbg_ptr, key, value);
        return Ellipse.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ellipse_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.ellipse_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        EllipseFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Ellipse}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_off(this.__wbg_ptr, event, handler);
        return Ellipse.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Ellipse}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ellipse_on(this.__wbg_ptr, ptr0, len0, handler);
        return Ellipse.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Ellipse}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_removeClipPath(this.__wbg_ptr);
        return Ellipse.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Ellipse}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_setClipPath(this.__wbg_ptr, clip);
        return Ellipse.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Ellipse}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_setShape(this.__wbg_ptr, shape);
        return Ellipse.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ellipse_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Ellipse}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_setStyle(this.__wbg_ptr, style);
        return Ellipse.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ellipse_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ellipse_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ellipse_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Ellipse}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ellipse_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Ellipse.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.ellipse_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ellipse_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ellipse_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Ellipse.prototype[Symbol.dispose] = Ellipse.prototype.free;

export class Group {
    static __wrap(ptr) {
        const obj = Object.create(Group.prototype);
        obj.__wbg_ptr = ptr;
        GroupFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GroupFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_group_free(ptr, 0);
    }
    /**
     * @param {any} child
     */
    add(child) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_add(this.__wbg_ptr, child);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} child
     * @param {any} next_sibling
     */
    addBefore(child, next_sibling) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_addBefore(this.__wbg_ptr, child, next_sibling);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Group}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_attr(this.__wbg_ptr, key, value);
        return Group.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.group_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    constructor() {
        const ret = wasm.group_new();
        this.__wbg_ptr = ret;
        GroupFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Group}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_off(this.__wbg_ptr, event, handler);
        return Group.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Group}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.group_on(this.__wbg_ptr, ptr0, len0, handler);
        return Group.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {any} child
     */
    remove(child) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_remove(this.__wbg_ptr, child);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    removeAll() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_removeAll(this.__wbg_ptr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @returns {Group}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_removeClipPath(this.__wbg_ptr);
        return Group.__wrap(ret);
    }
    /**
     * @param {any} old_child
     * @param {any} new_child
     */
    replace(old_child, new_child) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_replace(this.__wbg_ptr, old_child, new_child);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} clip
     * @returns {Group}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.group_setClipPath(this.__wbg_ptr, clip);
        return Group.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.group_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.group_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.group_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Group}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.group_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Group.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.group_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Group.prototype[Symbol.dispose] = Group.prototype.free;

/**
 * 对齐官方 `zr.handler`：无 DOM 时用 `dispatch` 注入指针事件。
 */
export class Handler {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(Handler.prototype);
        obj.__wbg_ptr = ptr;
        HandlerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HandlerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_handler_free(ptr, 0);
    }
    /**
     * @param {string} event_name
     * @param {any} event
     */
    dispatch(event_name, event) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.handler_dispatch(this.__wbg_ptr, ptr0, len0, event);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {string} cursor_style
     */
    setCursorStyle(cursor_style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(cursor_style, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.handler_setCursorStyle(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) Handler.prototype[Symbol.dispose] = Handler.prototype.free;

export class Heart {
    static __wrap(ptr) {
        const obj = Object.create(Heart.prototype);
        obj.__wbg_ptr = ptr;
        HeartFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HeartFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_heart_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Heart}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_attr(this.__wbg_ptr, key, value);
        return Heart.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.heart_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.heart_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        HeartFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Heart}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_off(this.__wbg_ptr, event, handler);
        return Heart.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Heart}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.heart_on(this.__wbg_ptr, ptr0, len0, handler);
        return Heart.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Heart}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_removeClipPath(this.__wbg_ptr);
        return Heart.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Heart}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_setClipPath(this.__wbg_ptr, clip);
        return Heart.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Heart}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_setShape(this.__wbg_ptr, shape);
        return Heart.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.heart_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Heart}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_setStyle(this.__wbg_ptr, style);
        return Heart.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.heart_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.heart_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.heart_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Heart}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.heart_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Heart.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.heart_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.heart_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.heart_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Heart.prototype[Symbol.dispose] = Heart.prototype.free;

export class HoverResult {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(HoverResult.prototype);
        obj.__wbg_ptr = ptr;
        HoverResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HoverResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hoverresult_free(ptr, 0);
    }
    /**
     * @returns {Element2}
     */
    get target() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.hoverresult_target(this.__wbg_ptr);
        return Element2.__wrap(ret);
    }
    /**
     * @returns {Element2}
     */
    get topTarget() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.hoverresult_topTarget(this.__wbg_ptr);
        return Element2.__wrap(ret);
    }
}
if (Symbol.dispose) HoverResult.prototype[Symbol.dispose] = HoverResult.prototype.free;

export class Image {
    static __wrap(ptr) {
        const obj = Object.create(Image.prototype);
        obj.__wbg_ptr = ptr;
        ImageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ImageFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_image_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Image}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_attr(this.__wbg_ptr, key, value);
        return Image.__wrap(ret);
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.image_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.image_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        ImageFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Image}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_off(this.__wbg_ptr, event, handler);
        return Image.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Image}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.image_on(this.__wbg_ptr, ptr0, len0, handler);
        return Image.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Image}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_removeClipPath(this.__wbg_ptr);
        return Image.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Image}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.image_setClipPath(this.__wbg_ptr, clip);
        return Image.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.image_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.image_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Image}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.image_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Image.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.image_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Image.prototype[Symbol.dispose] = Image.prototype.free;

export class IncrementalDisplayable {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IncrementalDisplayableFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_incrementaldisplayable_free(ptr, 0);
    }
    /**
     * @param {any} _opts
     */
    constructor(_opts) {
        const ret = wasm.incrementaldisplayable_new(_opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        IncrementalDisplayableFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) IncrementalDisplayable.prototype[Symbol.dispose] = IncrementalDisplayable.prototype.free;

export class Isogon {
    static __wrap(ptr) {
        const obj = Object.create(Isogon.prototype);
        obj.__wbg_ptr = ptr;
        IsogonFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IsogonFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_isogon_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Isogon}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_attr(this.__wbg_ptr, key, value);
        return Isogon.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.isogon_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.isogon_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        IsogonFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Isogon}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_off(this.__wbg_ptr, event, handler);
        return Isogon.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Isogon}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.isogon_on(this.__wbg_ptr, ptr0, len0, handler);
        return Isogon.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Isogon}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_removeClipPath(this.__wbg_ptr);
        return Isogon.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Isogon}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_setClipPath(this.__wbg_ptr, clip);
        return Isogon.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Isogon}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_setShape(this.__wbg_ptr, shape);
        return Isogon.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.isogon_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Isogon}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_setStyle(this.__wbg_ptr, style);
        return Isogon.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.isogon_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.isogon_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.isogon_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Isogon}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.isogon_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Isogon.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.isogon_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.isogon_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.isogon_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Isogon.prototype[Symbol.dispose] = Isogon.prototype.free;

export class Line {
    static __wrap(ptr) {
        const obj = Object.create(Line.prototype);
        obj.__wbg_ptr = ptr;
        LineFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_line_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Line}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_attr(this.__wbg_ptr, key, value);
        return Line.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.line_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.line_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        LineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Line}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_off(this.__wbg_ptr, event, handler);
        return Line.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Line}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.line_on(this.__wbg_ptr, ptr0, len0, handler);
        return Line.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Line}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_removeClipPath(this.__wbg_ptr);
        return Line.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Line}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_setClipPath(this.__wbg_ptr, clip);
        return Line.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Line}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_setShape(this.__wbg_ptr, shape);
        return Line.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.line_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Line}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_setStyle(this.__wbg_ptr, style);
        return Line.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.line_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.line_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.line_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Line}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.line_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Line.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.line_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.line_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.line_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Line.prototype[Symbol.dispose] = Line.prototype.free;

export class LinearGradient {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LinearGradientFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lineargradient_free(ptr, 0);
    }
    /**
     * @param {number} offset
     * @param {string} color
     */
    addColorStop(offset, color) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.lineargradient_addColorStop(this.__wbg_ptr, offset, ptr0, len0);
    }
    /**
     * @returns {any}
     */
    get colorStops() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.lineargradient_colorStops(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get global() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.lineargradient_global(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} x2
     * @param {number} y2
     * @param {any | null} [color_stops]
     * @param {boolean | null} [global_coord]
     */
    constructor(x, y, x2, y2, color_stops, global_coord) {
        if (!isLikeNone(global_coord)) {
            _assertBoolean(global_coord);
        }
        const ret = wasm.lineargradient_new(x, y, x2, y2, isLikeNone(color_stops) ? 0 : addToExternrefTable0(color_stops), isLikeNone(global_coord) ? 0xFFFFFF : global_coord ? 1 : 0);
        this.__wbg_ptr = ret;
        LinearGradientFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.lineargradient_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    get x() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.lineargradient_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get x2() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.lineargradient_x2(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.lineargradient_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y2() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.lineargradient_y2(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) LinearGradient.prototype[Symbol.dispose] = LinearGradient.prototype.free;

export class OrientedBoundingRect {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        OrientedBoundingRectFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_orientedboundingrect_free(ptr, 0);
    }
    /**
     * @param {BoundingRect} rect
     * @param {any} transform
     */
    fromBoundingRect(rect, transform) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(rect, BoundingRect);
        if (rect.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        wasm.orientedboundingrect_fromBoundingRect(this.__wbg_ptr, rect.__wbg_ptr, transform);
    }
    /**
     * @param {OrientedBoundingRect} other
     * @param {any} mtv
     * @param {any} opt
     * @returns {boolean}
     */
    intersect(other, mtv, opt) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, OrientedBoundingRect);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.orientedboundingrect_intersect(this.__wbg_ptr, other.__wbg_ptr, mtv, opt);
        return ret !== 0;
    }
    /**
     * @param {any} rect
     * @param {any} transform
     */
    constructor(rect, transform) {
        const ret = wasm.orientedboundingrect_new(rect, transform);
        this.__wbg_ptr = ret;
        OrientedBoundingRectFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) OrientedBoundingRect.prototype[Symbol.dispose] = OrientedBoundingRect.prototype.free;

export class Path {
    static __wrap(ptr) {
        const obj = Object.create(Path.prototype);
        obj.__wbg_ptr = ptr;
        PathFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PathFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_path_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Path}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_attr(this.__wbg_ptr, key, value);
        return Path.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.path_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.path_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        PathFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Path}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_off(this.__wbg_ptr, event, handler);
        return Path.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Path}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.path_on(this.__wbg_ptr, ptr0, len0, handler);
        return Path.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Path}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_removeClipPath(this.__wbg_ptr);
        return Path.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Path}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_setClipPath(this.__wbg_ptr, clip);
        return Path.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Path}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_setShape(this.__wbg_ptr, shape);
        return Path.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.path_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Path}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_setStyle(this.__wbg_ptr, style);
        return Path.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.path_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.path_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.path_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Path}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.path_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Path.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.path_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.path_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.path_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Path.prototype[Symbol.dispose] = Path.prototype.free;

export class Pattern {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PatternFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pattern_free(ptr, 0);
    }
    /**
     * @returns {Uint8Array}
     */
    get imageData() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_imageData(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get imageHeight() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_imageHeight(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get imageWidth() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_imageWidth(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} image
     * @param {string | null} [repeat]
     */
    constructor(image, repeat) {
        var ptr0 = isLikeNone(repeat) ? 0 : passStringToWasm0(repeat, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        const ret = wasm.pattern_new(image, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        PatternFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {string}
     */
    get repeat() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.pattern_repeat(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    get rotation() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_rotation(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get scaleX() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_scaleX(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get scaleY() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_scaleY(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.pattern_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    get x() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.pattern_y(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) Pattern.prototype[Symbol.dispose] = Pattern.prototype.free;

export class Point {
    static __wrap(ptr) {
        const obj = Object.create(Point.prototype);
        obj.__wbg_ptr = ptr;
        PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
    /**
     * @param {Point} other
     * @returns {Point}
     */
    add(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.point_add(this.__wbg_ptr, other.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {Point}
     */
    clone() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_clone(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} other
     * @returns {Point}
     */
    copy(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.point_copy(this.__wbg_ptr, other.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} other
     * @returns {number}
     */
    distance(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.point_distance(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point} other
     * @returns {number}
     */
    distanceSquare(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.point_distanceSquare(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point} other
     * @returns {number}
     */
    dot(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.point_dot(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point} other
     * @returns {boolean}
     */
    equal(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.point_equal(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {Float64Array} input
     */
    from_array(input) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passArrayF64ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.point_from_array(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    len() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_len(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    lenSquare() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_lenSquare(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Point}
     */
    negate() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_negate(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {number | null} [x]
     * @param {number | null} [y]
     */
    constructor(x, y) {
        if (!isLikeNone(x)) {
            _assertNum(x);
        }
        if (!isLikeNone(y)) {
            _assertNum(y);
        }
        const ret = wasm.point_new(!isLikeNone(x), isLikeNone(x) ? 0 : x, !isLikeNone(y), isLikeNone(y) ? 0 : y);
        this.__wbg_ptr = ret;
        PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Point}
     */
    normalize() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_normalize(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {number} scalar
     */
    scale(scalar) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.point_scale(this.__wbg_ptr, scalar);
    }
    /**
     * @param {Point} other
     * @param {number} scalar
     */
    scaleAndAdd(other, scalar) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        wasm.point_scaleAndAdd(this.__wbg_ptr, other.__wbg_ptr, scalar);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {Point}
     */
    set(x, y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_set(this.__wbg_ptr, x, y);
        return Point.__wrap(ret);
    }
    /**
     * @param {number} x
     */
    set x(x) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.point_set_x(this.__wbg_ptr, x);
    }
    /**
     * @param {number} y
     */
    set y(y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.point_set_y(this.__wbg_ptr, y);
    }
    /**
     * @param {Point} other
     * @returns {Point}
     */
    sub(other) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertClass(other, Point);
        if (other.__wbg_ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const ret = wasm.point_sub(this.__wbg_ptr, other.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Float64Array} out
     * @returns {Float64Array}
     */
    to_array(out) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        const ret = wasm.point_to_array(this.__wbg_ptr, ptr0, len0, out);
        var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v2;
    }
    /**
     * @param {any} matrix
     * @returns {Point}
     */
    transform(matrix) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_transform(this.__wbg_ptr, matrix);
        return Point.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get x() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.point_y(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

export class Polygon {
    static __wrap(ptr) {
        const obj = Object.create(Polygon.prototype);
        obj.__wbg_ptr = ptr;
        PolygonFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PolygonFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_polygon_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Polygon}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_attr(this.__wbg_ptr, key, value);
        return Polygon.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polygon_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.polygon_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        PolygonFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Polygon}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_off(this.__wbg_ptr, event, handler);
        return Polygon.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Polygon}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polygon_on(this.__wbg_ptr, ptr0, len0, handler);
        return Polygon.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Polygon}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_removeClipPath(this.__wbg_ptr);
        return Polygon.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Polygon}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_setClipPath(this.__wbg_ptr, clip);
        return Polygon.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Polygon}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_setShape(this.__wbg_ptr, shape);
        return Polygon.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polygon_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Polygon}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_setStyle(this.__wbg_ptr, style);
        return Polygon.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polygon_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polygon_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polygon_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Polygon}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polygon_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Polygon.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.polygon_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polygon_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polygon_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Polygon.prototype[Symbol.dispose] = Polygon.prototype.free;

export class Polyline {
    static __wrap(ptr) {
        const obj = Object.create(Polyline.prototype);
        obj.__wbg_ptr = ptr;
        PolylineFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PolylineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_polyline_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Polyline}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_attr(this.__wbg_ptr, key, value);
        return Polyline.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polyline_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.polyline_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        PolylineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Polyline}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_off(this.__wbg_ptr, event, handler);
        return Polyline.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Polyline}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polyline_on(this.__wbg_ptr, ptr0, len0, handler);
        return Polyline.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Polyline}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_removeClipPath(this.__wbg_ptr);
        return Polyline.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Polyline}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_setClipPath(this.__wbg_ptr, clip);
        return Polyline.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Polyline}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_setShape(this.__wbg_ptr, shape);
        return Polyline.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polyline_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Polyline}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_setStyle(this.__wbg_ptr, style);
        return Polyline.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polyline_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polyline_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.polyline_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Polyline}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polyline_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Polyline.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.polyline_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polyline_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.polyline_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Polyline.prototype[Symbol.dispose] = Polyline.prototype.free;

export class RadialGradient {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RadialGradientFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_radialgradient_free(ptr, 0);
    }
    /**
     * @param {number} offset
     * @param {string} color
     */
    addColorStop(offset, color) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.radialgradient_addColorStop(this.__wbg_ptr, offset, ptr0, len0);
    }
    /**
     * @returns {any}
     */
    get colorStops() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.radialgradient_colorStops(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get global() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.radialgradient_global(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} r
     * @param {any | null} [color_stops]
     * @param {boolean | null} [global_coord]
     * @param {number | null} [r0]
     */
    constructor(x, y, r, color_stops, global_coord, r0) {
        if (!isLikeNone(global_coord)) {
            _assertBoolean(global_coord);
        }
        if (!isLikeNone(r0)) {
            _assertNum(r0);
        }
        const ret = wasm.radialgradient_new(x, y, r, isLikeNone(color_stops) ? 0 : addToExternrefTable0(color_stops), isLikeNone(global_coord) ? 0xFFFFFF : global_coord ? 1 : 0, !isLikeNone(r0), isLikeNone(r0) ? 0 : r0);
        this.__wbg_ptr = ret;
        RadialGradientFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get r() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.radialgradient_r(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get r0() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.radialgradient_r0(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} r0
     */
    set r0(r0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.radialgradient_set_r0(this.__wbg_ptr, r0);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.radialgradient_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    get x() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.radialgradient_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.radialgradient_y(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) RadialGradient.prototype[Symbol.dispose] = RadialGradient.prototype.free;

export class Rect {
    static __wrap(ptr) {
        const obj = Object.create(Rect.prototype);
        obj.__wbg_ptr = ptr;
        RectFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RectFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rect_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Rect}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_attr(this.__wbg_ptr, key, value);
        return Rect.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rect_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.rect_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        RectFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Rect}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_off(this.__wbg_ptr, event, handler);
        return Rect.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Rect}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rect_on(this.__wbg_ptr, ptr0, len0, handler);
        return Rect.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Rect}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_removeClipPath(this.__wbg_ptr);
        return Rect.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Rect}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_setClipPath(this.__wbg_ptr, clip);
        return Rect.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Rect}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_setShape(this.__wbg_ptr, shape);
        return Rect.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rect_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Rect}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_setStyle(this.__wbg_ptr, style);
        return Rect.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rect_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rect_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rect_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Rect}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rect_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Rect.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.rect_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rect_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rect_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Rect.prototype[Symbol.dispose] = Rect.prototype.free;

export class Ring {
    static __wrap(ptr) {
        const obj = Object.create(Ring.prototype);
        obj.__wbg_ptr = ptr;
        RingFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RingFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ring_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Ring}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_attr(this.__wbg_ptr, key, value);
        return Ring.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ring_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.ring_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        RingFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Ring}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_off(this.__wbg_ptr, event, handler);
        return Ring.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Ring}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ring_on(this.__wbg_ptr, ptr0, len0, handler);
        return Ring.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Ring}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_removeClipPath(this.__wbg_ptr);
        return Ring.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Ring}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_setClipPath(this.__wbg_ptr, clip);
        return Ring.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Ring}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_setShape(this.__wbg_ptr, shape);
        return Ring.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ring_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Ring}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_setStyle(this.__wbg_ptr, style);
        return Ring.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ring_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ring_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.ring_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Ring}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ring_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Ring.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.ring_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ring_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.ring_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Ring.prototype[Symbol.dispose] = Ring.prototype.free;

export class Rose {
    static __wrap(ptr) {
        const obj = Object.create(Rose.prototype);
        obj.__wbg_ptr = ptr;
        RoseFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RoseFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rose_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Rose}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_attr(this.__wbg_ptr, key, value);
        return Rose.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rose_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.rose_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        RoseFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Rose}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_off(this.__wbg_ptr, event, handler);
        return Rose.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Rose}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rose_on(this.__wbg_ptr, ptr0, len0, handler);
        return Rose.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Rose}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_removeClipPath(this.__wbg_ptr);
        return Rose.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Rose}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_setClipPath(this.__wbg_ptr, clip);
        return Rose.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Rose}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_setShape(this.__wbg_ptr, shape);
        return Rose.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rose_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Rose}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_setStyle(this.__wbg_ptr, style);
        return Rose.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rose_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rose_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.rose_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Rose}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rose_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Rose.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.rose_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.rose_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.rose_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Rose.prototype[Symbol.dispose] = Rose.prototype.free;

export class Sector {
    static __wrap(ptr) {
        const obj = Object.create(Sector.prototype);
        obj.__wbg_ptr = ptr;
        SectorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SectorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sector_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Sector}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_attr(this.__wbg_ptr, key, value);
        return Sector.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.sector_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.sector_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        SectorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Sector}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_off(this.__wbg_ptr, event, handler);
        return Sector.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Sector}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sector_on(this.__wbg_ptr, ptr0, len0, handler);
        return Sector.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Sector}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_removeClipPath(this.__wbg_ptr);
        return Sector.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Sector}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_setClipPath(this.__wbg_ptr, clip);
        return Sector.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Sector}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_setShape(this.__wbg_ptr, shape);
        return Sector.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sector_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Sector}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_setStyle(this.__wbg_ptr, style);
        return Sector.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.sector_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.sector_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.sector_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Sector}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sector_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Sector.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.sector_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sector_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.sector_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Sector.prototype[Symbol.dispose] = Sector.prototype.free;

export class Star {
    static __wrap(ptr) {
        const obj = Object.create(Star.prototype);
        obj.__wbg_ptr = ptr;
        StarFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StarFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_star_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Star}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_attr(this.__wbg_ptr, key, value);
        return Star.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.star_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.star_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        StarFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Star}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_off(this.__wbg_ptr, event, handler);
        return Star.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Star}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.star_on(this.__wbg_ptr, ptr0, len0, handler);
        return Star.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Star}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_removeClipPath(this.__wbg_ptr);
        return Star.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Star}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_setClipPath(this.__wbg_ptr, clip);
        return Star.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Star}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_setShape(this.__wbg_ptr, shape);
        return Star.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.star_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Star}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_setStyle(this.__wbg_ptr, style);
        return Star.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.star_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.star_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.star_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Star}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.star_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Star.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.star_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.star_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.star_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Star.prototype[Symbol.dispose] = Star.prototype.free;

export class TSpan {
    static __wrap(ptr) {
        const obj = Object.create(TSpan.prototype);
        obj.__wbg_ptr = ptr;
        TSpanFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TSpanFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tspan_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.tspan_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {TSpan}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.tspan_attr(this.__wbg_ptr, key, value);
        return TSpan.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.tspan_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.tspan_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.tspan_new(opts);
        this.__wbg_ptr = ret;
        TSpanFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {TSpan}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.tspan_off(this.__wbg_ptr, event, handler);
        return TSpan.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {TSpan}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tspan_on(this.__wbg_ptr, ptr0, len0, handler);
        return TSpan.__wrap(ret);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.tspan_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {TSpan}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tspan_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return TSpan.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.tspan_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) TSpan.prototype[Symbol.dispose] = TSpan.prototype.free;

export class Text {
    static __wrap(ptr) {
        const obj = Object.create(Text.prototype);
        obj.__wbg_ptr = ptr;
        TextFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TextFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_text_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Text}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_attr(this.__wbg_ptr, key, value);
        return Text.__wrap(ret);
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.text_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.text_new(opts);
        this.__wbg_ptr = ret;
        TextFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Text}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_off(this.__wbg_ptr, event, handler);
        return Text.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Text}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.text_on(this.__wbg_ptr, ptr0, len0, handler);
        return Text.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Text}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_removeClipPath(this.__wbg_ptr);
        return Text.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Text}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_setClipPath(this.__wbg_ptr, clip);
        return Text.__wrap(ret);
    }
    /**
     * @param {any} style
     * @returns {Text}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.text_setStyle(this.__wbg_ptr, style);
        return Text.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.text_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.text_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Text}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.text_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Text.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.text_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Text.prototype[Symbol.dispose] = Text.prototype.free;

export class Trochoid {
    static __wrap(ptr) {
        const obj = Object.create(Trochoid.prototype);
        obj.__wbg_ptr = ptr;
        TrochoidFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TrochoidFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_trochoid_free(ptr, 0);
    }
    /**
     * @param {any} path
     * @param {any} looping
     * @returns {Animator}
     */
    animate(path, looping) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_animate(this.__wbg_ptr, path, looping);
        return Animator.__wrap(ret);
    }
    /**
     * @param {any} key
     * @param {any} value
     * @returns {Trochoid}
     */
    attr(key, value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_attr(this.__wbg_ptr, key, value);
        return Trochoid.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get draggable() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_draggable(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {BoundingRect}
     */
    getBoundingRect() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_getBoundingRect(this.__wbg_ptr);
        return BoundingRect.__wrap(ret);
    }
    hide() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.trochoid_hide(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} opts
     */
    constructor(opts) {
        const ret = wasm.trochoid_new(opts);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        TrochoidFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {Trochoid}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_off(this.__wbg_ptr, event, handler);
        return Trochoid.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {Trochoid}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.trochoid_on(this.__wbg_ptr, ptr0, len0, handler);
        return Trochoid.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    get position() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Trochoid}
     */
    removeClipPath() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_removeClipPath(this.__wbg_ptr);
        return Trochoid.__wrap(ret);
    }
    /**
     * @param {any} clip
     * @returns {Trochoid}
     */
    setClipPath(clip) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_setClipPath(this.__wbg_ptr, clip);
        return Trochoid.__wrap(ret);
    }
    /**
     * @param {any} shape
     * @returns {Trochoid}
     */
    setShape(shape) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_setShape(this.__wbg_ptr, shape);
        return Trochoid.__wrap(ret);
    }
    /**
     * @param {string} state
     * @param {any} style
     */
    setStateStyle(state, style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.trochoid_setStateStyle(this.__wbg_ptr, ptr0, len0, style);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} style
     * @returns {Trochoid}
     */
    setStyle(style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_setStyle(this.__wbg_ptr, style);
        return Trochoid.__wrap(ret);
    }
    /**
     * @param {any} value
     */
    set draggable(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.trochoid_set_draggable(this.__wbg_ptr, value);
    }
    /**
     * @param {any} value
     */
    set position(value) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.trochoid_set_position(this.__wbg_ptr, value);
    }
    show() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.trochoid_show(this.__wbg_ptr);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {Trochoid}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.trochoid_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return Trochoid.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get type() {
        let deferred1_0;
        let deferred1_1;
        try {
            if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.__wbg_ptr);
            const ret = wasm.trochoid_type(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} state
     */
    useState(state) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(state, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.trochoid_useState(this.__wbg_ptr, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} states
     */
    useStates(states) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.trochoid_useStates(this.__wbg_ptr, states);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Trochoid.prototype[Symbol.dispose] = Trochoid.prototype.free;

export class ZRender {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(ZRender.prototype);
        obj.__wbg_ptr = ptr;
        ZRenderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ZRenderFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_zrender_free(ptr, 0);
    }
    /**
     * @param {any} el
     */
    add(el) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_add(this.__wbg_ptr, el);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @returns {Animation}
     */
    get animation() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_animation(this.__wbg_ptr);
        return Animation.__wrap(ret);
    }
    clear() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_clear(this.__wbg_ptr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} _z_level
     * @param {any} _config
     */
    configLayer(_z_level, _config) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_configLayer(this.__wbg_ptr, _z_level, _config);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    dispose() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.zrender_dispose(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    dpr() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_dpr(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {HoverResult | undefined}
     */
    findHover(x, y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_findHover(this.__wbg_ptr, x, y);
        return ret === 0 ? undefined : HoverResult.__wrap(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    flush() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_flush(this.__wbg_ptr);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {any}
     */
    getBackgroundColor() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_getBackgroundColor(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    getHeight() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_getHeight(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    getWidth() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_getWidth(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {Handler}
     */
    get handler() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_handler(this.__wbg_ptr);
        return Handler.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    height() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get id() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {any} event
     * @param {any} handler
     * @returns {ZRender}
     */
    off(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_off(this.__wbg_ptr, event, handler);
        return ZRender.__wrap(ret);
    }
    /**
     * @param {string} event
     * @param {any} handler
     * @returns {ZRender}
     */
    on(event, handler) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.zrender_on(this.__wbg_ptr, ptr0, len0, handler);
        return ZRender.__wrap(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    refresh() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_refresh(this.__wbg_ptr);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {any} el
     */
    remove(el) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_remove(this.__wbg_ptr, el);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} opts
     */
    resize(opts) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_resize(this.__wbg_ptr, opts);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {any} color
     */
    setBackgroundColor(color) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_setBackgroundColor(this.__wbg_ptr, color);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {string} cursor_style
     */
    setCursorStyle(cursor_style) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(cursor_style, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.zrender_setCursorStyle(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} event
     * @param {any} packet
     * @returns {ZRender}
     */
    trigger(event, packet) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ptr0 = passStringToWasm0(event, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.zrender_trigger(this.__wbg_ptr, ptr0, len0, packet);
        return ZRender.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    width() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.zrender_width(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) ZRender.prototype[Symbol.dispose] = ZRender.prototype.free;

/**
 * 清空已注册字体（主要用于测试）。
 */
export function clearFonts() {
    const ret = wasm.clearFonts();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @returns {any}
 */
export function color() {
    const ret = wasm.color();
    return ret;
}

/**
 * @param {ZRender} zr
 */
export function dispose(zr) {
    _assertClass(zr, ZRender);
    if (zr.__wbg_ptr === 0) {
        throw new Error('Attempt to use a moved value');
    }
    wasm.dispose(zr.__wbg_ptr);
}

export function disposeAll() {
    wasm.disposeAll();
}

/**
 * @param {number} id
 * @returns {ZRender | undefined}
 */
export function getInstance(id) {
    _assertNum(id);
    const ret = wasm.getInstance(id);
    return ret === 0 ? undefined : ZRender.__wrap(ret);
}

/**
 * 创建 ZRender 实例（dom 参数忽略，尺寸来自 opts）
 * @param {any} dom
 * @param {any} opts
 * @returns {ZRender}
 */
export function init(dom, opts) {
    const ret = wasm.init(dom, opts);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ZRender.__wrap(ret[0]);
}

export function init_panic_hook() {
    wasm.init_panic_hook();
}

/**
 * @returns {any}
 */
export function matrix() {
    const ret = wasm.matrix();
    return ret;
}

/**
 * @returns {any}
 */
export function morph() {
    const ret = wasm.morph();
    return ret;
}

/**
 * @returns {any}
 */
export function parseSVG() {
    const ret = wasm.parseSVG();
    return ret;
}

/**
 * @returns {any}
 */
export function path() {
    const ret = wasm.path();
    return ret;
}

/**
 * 向全局 fontdb 注册字体文件。
 *
 * `opts` 可选字段：
 * - `familyName`: 覆盖字体族名
 * - `sansSerif`: `string[]`，将 CSS `sans-serif` 映射到这些族名
 * @param {Uint8Array} data
 * @param {any} opts
 */
export function registerFont(data, opts) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.registerFont(ptr0, len0, opts);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {any} _api
 */
export function setPlatformAPI(_api) {
    wasm.setPlatformAPI(_api);
}

export function showDebugDirtyRect() {
    wasm.showDebugDirtyRect();
}

/**
 * @returns {any}
 */
export function util() {
    const ret = wasm.util();
    return ret;
}

/**
 * @returns {any}
 */
export function vector() {
    const ret = wasm.vector();
    return ret;
}

//#endregion

//#region wasm imports
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_boolean_get_edaed31a367ce1bd: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? v : undefined;
            if (!isLikeNone(ret)) {
                _assertBoolean(ret);
            }
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_copy_to_typed_array_c5728021fabd0236: function(arg0, arg1, arg2) {
            new Uint8Array(arg2.buffer, arg2.byteOffset, arg2.byteLength).set(getArrayU8FromWasm0(arg0, arg1));
        },
        __wbg___wbindgen_debug_string_8a447059637473e2: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_acc5528be2b923f2: function(arg0) {
            const ret = typeof(arg0) === 'function';
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_null_6d937fbfb6478470: function(arg0) {
            const ret = arg0 === null;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_object_0beba4a1980d3eea: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_string_1fca8072260dd261: function(arg0) {
            const ret = typeof(arg0) === 'string';
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_undefined_721f8decd50c87a3: function(arg0) {
            const ret = arg0 === undefined;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_jsval_eq_4e8c38722cb8ff51: function(arg0, arg1) {
            const ret = arg0 === arg1;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_number_get_1cc01dd708740256: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            if (!isLikeNone(ret)) {
                _assertNum(ret);
            }
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_71bb4348194e31f0: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_ea4887a5f8f9a9db: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_33c39e13d73b25f6: function() { return logError(function (arg0) {
            arg0._wbg_cb_unref();
        }, arguments); },
        __wbg_addEventListener_ea90bc131475777e: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_call_0e855b388e315e17: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg0.call(arg1, arg2, arg3);
            return ret;
        }, arguments); },
        __wbg_call_5575218572ead796: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_call_8e98ed2f3c86c4b5: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_clientX_7d645a0b265349f0: function() { return logError(function (arg0) {
            const ret = arg0.clientX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_clientY_558c61970f6b424a: function() { return logError(function (arg0) {
            const ret = arg0.clientY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_createElement_9e23ac95e40e302c: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_data_411cafdcea483b74: function() { return logError(function (arg0, arg1) {
            const ret = arg1.data;
            const ptr1 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_document_2634180a4c694068: function() { return logError(function (arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_drawImage_baead0925fc14e5f: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.drawImage(arg1, arg2, arg3);
        }, arguments); },
        __wbg_element_new: function() { return logError(function (arg0) {
            const ret = Element2.__wrap(arg0);
            return ret;
        }, arguments); },
        __wbg_error_a6fa202b58aa1cd3: function() { return logError(function (arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        }, arguments); },
        __wbg_from_50138b2ca136f50c: function() { return logError(function (arg0) {
            const ret = Array.from(arg0);
            return ret;
        }, arguments); },
        __wbg_getBoundingClientRect_bf4a017da5494fc9: function() { return logError(function (arg0) {
            const ret = arg0.getBoundingClientRect();
            return ret;
        }, arguments); },
        __wbg_getContext_486aab500e1c34c9: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getImageData_bcdd3d54c5e0f471: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = arg0.getImageData(arg1, arg2, arg3, arg4);
            return ret;
        }, arguments); },
        __wbg_get_197a3fe98f169e38: function() { return logError(function (arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        }, arguments); },
        __wbg_get_dddb90ff5d27a080: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_unchecked_54a4374c38e08460: function() { return logError(function (arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        }, arguments); },
        __wbg_height_a04613570d793df2: function() { return logError(function (arg0) {
            const ret = arg0.height;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_height_cd732bbdc7b66e49: function() { return logError(function (arg0) {
            const ret = arg0.height;
            return ret;
        }, arguments); },
        __wbg_instanceof_Array_49598245ad12c736: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_CanvasRenderingContext2d_d0cab9e931424c52: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Element_7cd17aeb1babd05e: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_HtmlCanvasElement_8ce29a370a2b10a4: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLCanvasElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_HtmlImageElement_83e43a035d4d5033: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLImageElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Object_60be3eaa7a661141: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Object;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Uint8Array_f080092dc70f5d58: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Window_0d356b88a2f77c42: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_isArray_145a34fd0a38d37b: function() { return logError(function (arg0) {
            const ret = Array.isArray(arg0);
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_keys_e2132f5645c137bf: function() { return logError(function (arg0) {
            const ret = Object.keys(arg0);
            return ret;
        }, arguments); },
        __wbg_left_fd8e5732a10e0137: function() { return logError(function (arg0) {
            const ret = arg0.left;
            return ret;
        }, arguments); },
        __wbg_length_589238bdcf171f0e: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_length_c6054974c0a6cdb9: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_naturalHeight_7cbdc2f637f6914f: function() { return logError(function (arg0) {
            const ret = arg0.naturalHeight;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_naturalWidth_5c95710d6c0ff97a: function() { return logError(function (arg0) {
            const ret = arg0.naturalWidth;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_new_227d7c05414eb861: function() { return logError(function () {
            const ret = new Error();
            return ret;
        }, arguments); },
        __wbg_new_2e117a478906f062: function() { return logError(function () {
            const ret = new Object();
            return ret;
        }, arguments); },
        __wbg_new_36e147a8ced3c6e0: function() { return logError(function () {
            const ret = new Array();
            return ret;
        }, arguments); },
        __wbg_new_from_slice_543b875b27789a8f: function() { return logError(function (arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_with_u8_clamped_array_and_sh_adb3f647b0414eb2: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
            return ret;
        }, arguments); },
        __wbg_pointerId_2c4027fd33473f83: function() { return logError(function (arg0) {
            const ret = arg0.pointerId;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_prototypesetcall_d721637c7ca66eb8: function() { return logError(function (arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        }, arguments); },
        __wbg_push_f724b5db8acf89d2: function() { return logError(function (arg0, arg1) {
            const ret = arg0.push(arg1);
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_putImageData_d36ffa8305aea239: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.putImageData(arg1, arg2, arg3);
        }, arguments); },
        __wbg_querySelector_45b96634e7f85460: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_releasePointerCapture_746a323c70b5cff4: function() { return handleError(function (arg0, arg1) {
            arg0.releasePointerCapture(arg1);
        }, arguments); },
        __wbg_removeEventListener_c6782a9c30557d31: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_setPointerCapture_87f6b55ceca474a9: function() { return handleError(function (arg0, arg1) {
            arg0.setPointerCapture(arg1);
        }, arguments); },
        __wbg_setProperty_da5e4438a912e787: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_set_4564f7dc44fcb0c9: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_set_height_ad5056ea051acd78: function() { return logError(function (arg0, arg1) {
            arg0.height = arg1 >>> 0;
        }, arguments); },
        __wbg_set_width_031bdecd763c5855: function() { return logError(function (arg0, arg1) {
            arg0.width = arg1 >>> 0;
        }, arguments); },
        __wbg_stack_3b0d974bbf31e44f: function() { return logError(function (arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_static_accessor_GLOBAL_THIS_2fee5048bcca5938: function() { return logError(function () {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_GLOBAL_ce44e66a4935da8c: function() { return logError(function () {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_SELF_44f6e0cb5e67cdad: function() { return logError(function () {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_WINDOW_168f178805d978fe: function() { return logError(function () {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_style_f00fbf31e0a68f0e: function() { return logError(function (arg0) {
            const ret = arg0.style;
            return ret;
        }, arguments); },
        __wbg_top_b0669fa399851b2e: function() { return logError(function (arg0) {
            const ret = arg0.top;
            return ret;
        }, arguments); },
        __wbg_width_a86bcd32bd998060: function() { return logError(function (arg0) {
            const ret = arg0.width;
            return ret;
        }, arguments); },
        __wbg_width_c8740d5bdf596189: function() { return logError(function (arg0) {
            const ret = arg0.width;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000001: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("PointerEvent")], shim_idx: 47, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h7763fc9826fa485a);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000002: function() { return logError(function (arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000003: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        }, arguments); },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./wasm_zrender_bg.js": import0,
    };
}


//#endregion
function wasm_bindgen__convert__closures_____invoke__h7763fc9826fa485a(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen__convert__closures_____invoke__h7763fc9826fa485a(arg0, arg1, arg2);
}

const AnimationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_animation_free(ptr, 1));
const AnimatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_animator_free(ptr, 1));
const ArcFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_arc_free(ptr, 1));
const BezierCurveFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_beziercurve_free(ptr, 1));
const BoundingRectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_boundingrect_free(ptr, 1));
const CircleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_circle_free(ptr, 1));
const CompoundPathFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compoundpath_free(ptr, 1));
const DisplayableFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_displayable_free(ptr, 1));
const DropletFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_droplet_free(ptr, 1));
const Element2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_element_free(ptr, 1));
const EllipseFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ellipse_free(ptr, 1));
const GroupFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_group_free(ptr, 1));
const HandlerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_handler_free(ptr, 1));
const HeartFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_heart_free(ptr, 1));
const HoverResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_hoverresult_free(ptr, 1));
const ImageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_image_free(ptr, 1));
const IncrementalDisplayableFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_incrementaldisplayable_free(ptr, 1));
const IsogonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_isogon_free(ptr, 1));
const LineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_line_free(ptr, 1));
const LinearGradientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lineargradient_free(ptr, 1));
const OrientedBoundingRectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_orientedboundingrect_free(ptr, 1));
const PathFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_path_free(ptr, 1));
const PatternFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_pattern_free(ptr, 1));
const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr, 1));
const PolygonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_polygon_free(ptr, 1));
const PolylineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_polyline_free(ptr, 1));
const RadialGradientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_radialgradient_free(ptr, 1));
const RectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rect_free(ptr, 1));
const RingFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ring_free(ptr, 1));
const RoseFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rose_free(ptr, 1));
const SectorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sector_free(ptr, 1));
const StarFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_star_free(ptr, 1));
const TSpanFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tspan_free(ptr, 1));
const TextFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_text_free(ptr, 1));
const TrochoidFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_trochoid_free(ptr, 1));
const ZRenderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_zrender_free(ptr, 1));


//#region intrinsics
function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedUint8ClampedArrayMemory0 = null;
function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


//#endregion

//#region wasm loading
let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat64ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    cachedUint8ClampedArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('wasm_zrender_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
//#endregion
export { wasm as __wasm }
