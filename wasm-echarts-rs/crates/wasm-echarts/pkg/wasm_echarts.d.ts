/* tslint:disable */
/* eslint-disable */

/**
 * 对齐 `zr.animation`：无帧循环；`on('frame')` 为空操作。
 */
export class Animation {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    off(_event: any, _handler: any): void;
    on(_event: string, _handler: any): void;
}

/**
 * 对齐 zrender `Animator`：终态语义。
 */
export class Animator {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    delay(_time: number): Animator;
    done(cb: any): Animator;
    during(cb: any): Animator;
    start(_easing: any): Animator;
    stop(): void;
    when(_time: number, props: any): Animator;
}

export class Arc {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Arc;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Arc;
    on(event: string, handler: any): Arc;
    removeClipPath(): Arc;
    setClipPath(clip: any): Arc;
    setShape(shape: any): Arc;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Arc;
    show(): void;
    trigger(event: string, packet: any): Arc;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class BezierCurve {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): BezierCurve;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): BezierCurve;
    on(event: string, handler: any): BezierCurve;
    removeClipPath(): BezierCurve;
    setClipPath(clip: any): BezierCurve;
    setShape(shape: any): BezierCurve;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): BezierCurve;
    show(): void;
    trigger(event: string, packet: any): BezierCurve;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class BoundingRect {
    free(): void;
    [Symbol.dispose](): void;
    applyTransform(matrix: any): void;
    calculateTransform(b: BoundingRect): Float64Array;
    clone(): BoundingRect;
    contain(x: number, y: number): boolean;
    copy(other: any): void;
    static create(rect: any): BoundingRect;
    intersect(other: BoundingRect, opt: any): boolean;
    isFinite(): boolean;
    isZero(): boolean;
    constructor(x: number, y: number, width: number, height: number);
    plain(): any;
    static set(target: any, x: number, y: number, width: number, height: number): void;
    union(other: BoundingRect): void;
    height: number;
    width: number;
    x: number;
    y: number;
}

export class Circle {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Circle;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Circle;
    on(event: string, handler: any): Circle;
    removeClipPath(): Circle;
    setClipPath(clip: any): Circle;
    setShape(shape: any): Circle;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Circle;
    show(): void;
    trigger(event: string, packet: any): Circle;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class CompoundPath {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): CompoundPath;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): CompoundPath;
    on(event: string, handler: any): CompoundPath;
    removeClipPath(): CompoundPath;
    setClipPath(clip: any): CompoundPath;
    setShape(shape: any): CompoundPath;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): CompoundPath;
    show(): void;
    trigger(event: string, packet: any): CompoundPath;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class CustomSeriesApi {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    coord(data: any): Array<any>;
    size(data_size: any, data_item: any): Array<any>;
    style(extra: any): any;
    value(dim: any): number;
}

/**
 * Path / Text / Image / Group 等图元的公共 displayable 属性由构造 opts 传入。
 * 本类不可直接实例化，仅用于 API 对齐与文档说明。
 */
export class Displayable {
    free(): void;
    [Symbol.dispose](): void;
    constructor(_opts: any);
}

export class Droplet {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Droplet;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Droplet;
    on(event: string, handler: any): Droplet;
    removeClipPath(): Droplet;
    setClipPath(clip: any): Droplet;
    setShape(shape: any): Droplet;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Droplet;
    show(): void;
    trigger(event: string, packet: any): Droplet;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class EChartsInstance {
    free(): void;
    [Symbol.dispose](): void;
    appendData(params: any): void;
    /**
     * 滚轮 dataZoom（option 含 dataZoom 时生效）
     */
    apply_data_zoom_wheel(x: number, delta_y: number): void;
    attachHost(dom: any): void;
    /**
     * 阶段 7：基准测试 setOption 管线 + refresh 平均耗时（毫秒）
     */
    benchmark_render(iterations: number): number;
    containPixel(finder: any, value: any): boolean;
    /**
     * `convertFromPixel(finder, value)`：与 `convertToPixel` 同一 finder 最小集。
     */
    convert_from_pixel(finder: any, value: any): any;
    /**
     * `convertToPixel(finder, value)`：cartesian / polar / geo / calendar / single / parallel / matrix / radar finder。
     */
    convert_to_pixel(finder: any, value: any): any;
    dispatch_action(action: any): void;
    dispose(): void;
    dpr(): number;
    find_hover(x: number, y: number): any;
    getZr(): ZRender;
    get_option(): any;
    /**
     * hover 时调用 tooltip.formatter，返回 string 或 null
     */
    get_tooltip_content(series_index: number, data_index: number): any;
    handlePointerClick(x: number, y: number): any;
    handlePointerDown(x: number, y: number): any;
    handlePointerUp(x: number, y: number): void;
    handle_pointer_leave(): void;
    /**
     * 阶段 6：pointer move 统一处理 hover 高亮、axisPointer、tooltip。
     * 画面没变时跳过全量 `run_update` / 状态切换，并令 `dirty: false` 让 facade 不上屏。
     */
    handle_pointer_move(x: number, y: number): any;
    has_option(): boolean;
    height(): number;
    constructor(width: number, height: number, dpr: number);
    option_has_functions(): boolean;
    refresh(): Uint8Array;
    resize(width: number, height: number, dpr: number): void;
    /**
     * `opts` 为官方第二参数：`boolean` 或 `{ notMerge, replaceMerge }`。可省略。
     */
    set_option(option: any, opts?: any | null): void;
    /**
     * 把全局 fontdb 同步到本实例（`registerFont` 之后由 facade 调用）。
     */
    update_font_database(): void;
    width(): number;
}

declare class Element2 {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly id: number;
    readonly type: string;
}
export { Element2 as Element }

export class Ellipse {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Ellipse;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Ellipse;
    on(event: string, handler: any): Ellipse;
    removeClipPath(): Ellipse;
    setClipPath(clip: any): Ellipse;
    setShape(shape: any): Ellipse;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Ellipse;
    show(): void;
    trigger(event: string, packet: any): Ellipse;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Group {
    free(): void;
    [Symbol.dispose](): void;
    add(child: any): void;
    addBefore(child: any, next_sibling: any): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Group;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor();
    off(event: any, handler: any): Group;
    on(event: string, handler: any): Group;
    remove(child: any): void;
    removeAll(): void;
    removeClipPath(): Group;
    replace(old_child: any, new_child: any): void;
    setClipPath(clip: any): Group;
    show(): void;
    trigger(event: string, packet: any): Group;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

/**
 * 对齐官方 `zr.handler`：无 DOM 时用 `dispatch` 注入指针事件。
 */
export class Handler {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    dispatch(event_name: string, event: any): void;
    setCursorStyle(cursor_style: string): void;
}

export class Heart {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Heart;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Heart;
    on(event: string, handler: any): Heart;
    removeClipPath(): Heart;
    setClipPath(clip: any): Heart;
    setShape(shape: any): Heart;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Heart;
    show(): void;
    trigger(event: string, packet: any): Heart;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class HoverResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly target: Element2;
    readonly topTarget: Element2;
}

export class Image {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Image;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Image;
    on(event: string, handler: any): Image;
    removeClipPath(): Image;
    setClipPath(clip: any): Image;
    show(): void;
    trigger(event: string, packet: any): Image;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class IncrementalDisplayable {
    free(): void;
    [Symbol.dispose](): void;
    constructor(_opts: any);
}

export class Isogon {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Isogon;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Isogon;
    on(event: string, handler: any): Isogon;
    removeClipPath(): Isogon;
    setClipPath(clip: any): Isogon;
    setShape(shape: any): Isogon;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Isogon;
    show(): void;
    trigger(event: string, packet: any): Isogon;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Line {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Line;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Line;
    on(event: string, handler: any): Line;
    removeClipPath(): Line;
    setClipPath(clip: any): Line;
    setShape(shape: any): Line;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Line;
    show(): void;
    trigger(event: string, packet: any): Line;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class LinearGradient {
    free(): void;
    [Symbol.dispose](): void;
    addColorStop(offset: number, color: string): void;
    constructor(x: number, y: number, x2: number, y2: number, color_stops?: any | null, global_coord?: boolean | null);
    readonly colorStops: any;
    readonly global: boolean;
    readonly type: string;
    readonly x: number;
    readonly x2: number;
    readonly y: number;
    readonly y2: number;
}

export class OrientedBoundingRect {
    free(): void;
    [Symbol.dispose](): void;
    fromBoundingRect(rect: BoundingRect, transform: any): void;
    intersect(other: OrientedBoundingRect, mtv: any, opt: any): boolean;
    constructor(rect: any, transform: any);
}

export class Path {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Path;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Path;
    on(event: string, handler: any): Path;
    removeClipPath(): Path;
    setClipPath(clip: any): Path;
    setShape(shape: any): Path;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Path;
    show(): void;
    trigger(event: string, packet: any): Path;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Pattern {
    free(): void;
    [Symbol.dispose](): void;
    constructor(image: any, repeat?: string | null);
    readonly imageData: Uint8Array;
    readonly imageHeight: number;
    readonly imageWidth: number;
    readonly repeat: string;
    readonly rotation: number;
    readonly scaleX: number;
    readonly scaleY: number;
    readonly type: string;
    readonly x: number;
    readonly y: number;
}

export class Point {
    free(): void;
    [Symbol.dispose](): void;
    add(other: Point): Point;
    clone(): Point;
    copy(other: Point): Point;
    distance(other: Point): number;
    distanceSquare(other: Point): number;
    dot(other: Point): number;
    equal(other: Point): boolean;
    from_array(input: Float64Array): void;
    len(): number;
    lenSquare(): number;
    negate(): Point;
    constructor(x?: number | null, y?: number | null);
    normalize(): Point;
    scale(scalar: number): void;
    scaleAndAdd(other: Point, scalar: number): void;
    set(x: number, y: number): Point;
    sub(other: Point): Point;
    to_array(out: Float64Array): Float64Array;
    transform(matrix: any): Point;
    x: number;
    y: number;
}

export class Polygon {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Polygon;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Polygon;
    on(event: string, handler: any): Polygon;
    removeClipPath(): Polygon;
    setClipPath(clip: any): Polygon;
    setShape(shape: any): Polygon;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Polygon;
    show(): void;
    trigger(event: string, packet: any): Polygon;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Polyline {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Polyline;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Polyline;
    on(event: string, handler: any): Polyline;
    removeClipPath(): Polyline;
    setClipPath(clip: any): Polyline;
    setShape(shape: any): Polyline;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Polyline;
    show(): void;
    trigger(event: string, packet: any): Polyline;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class RadialGradient {
    free(): void;
    [Symbol.dispose](): void;
    addColorStop(offset: number, color: string): void;
    constructor(x: number, y: number, r: number, color_stops?: any | null, global_coord?: boolean | null, r0?: number | null);
    readonly colorStops: any;
    readonly global: boolean;
    readonly r: number;
    r0: number;
    readonly type: string;
    readonly x: number;
    readonly y: number;
}

export class Rect {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Rect;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Rect;
    on(event: string, handler: any): Rect;
    removeClipPath(): Rect;
    setClipPath(clip: any): Rect;
    setShape(shape: any): Rect;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Rect;
    show(): void;
    trigger(event: string, packet: any): Rect;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Ring {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Ring;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Ring;
    on(event: string, handler: any): Ring;
    removeClipPath(): Ring;
    setClipPath(clip: any): Ring;
    setShape(shape: any): Ring;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Ring;
    show(): void;
    trigger(event: string, packet: any): Ring;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Rose {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Rose;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Rose;
    on(event: string, handler: any): Rose;
    removeClipPath(): Rose;
    setClipPath(clip: any): Rose;
    setShape(shape: any): Rose;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Rose;
    show(): void;
    trigger(event: string, packet: any): Rose;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Sector {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Sector;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Sector;
    on(event: string, handler: any): Sector;
    removeClipPath(): Sector;
    setClipPath(clip: any): Sector;
    setShape(shape: any): Sector;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Sector;
    show(): void;
    trigger(event: string, packet: any): Sector;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Star {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Star;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Star;
    on(event: string, handler: any): Star;
    removeClipPath(): Star;
    setClipPath(clip: any): Star;
    setShape(shape: any): Star;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Star;
    show(): void;
    trigger(event: string, packet: any): Star;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class TSpan {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): TSpan;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): TSpan;
    on(event: string, handler: any): TSpan;
    show(): void;
    trigger(event: string, packet: any): TSpan;
    readonly id: number;
    readonly type: string;
}

export class Text {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Text;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Text;
    on(event: string, handler: any): Text;
    removeClipPath(): Text;
    setClipPath(clip: any): Text;
    setStyle(style: any): Text;
    show(): void;
    trigger(event: string, packet: any): Text;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class Trochoid {
    free(): void;
    [Symbol.dispose](): void;
    animate(path: any, looping: any): Animator;
    attr(key: any, value: any): Trochoid;
    getBoundingRect(): BoundingRect;
    hide(): void;
    constructor(opts: any);
    off(event: any, handler: any): Trochoid;
    on(event: string, handler: any): Trochoid;
    removeClipPath(): Trochoid;
    setClipPath(clip: any): Trochoid;
    setShape(shape: any): Trochoid;
    setStateStyle(state: string, style: any): void;
    setStyle(style: any): Trochoid;
    show(): void;
    trigger(event: string, packet: any): Trochoid;
    useState(state: string): void;
    useStates(states: any): void;
    draggable: any;
    readonly id: number;
    position: any;
    readonly type: string;
}

export class ZRender {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    add(el: any): void;
    clear(): void;
    configLayer(_z_level: any, _config: any): void;
    dispose(): void;
    dpr(): number;
    findHover(x: number, y: number): HoverResult | undefined;
    flush(): Uint8Array;
    getBackgroundColor(): any;
    getHeight(): number;
    getWidth(): number;
    height(): number;
    off(event: any, handler: any): ZRender;
    on(event: string, handler: any): ZRender;
    refresh(): Uint8Array;
    remove(el: any): void;
    resize(opts: any): void;
    setBackgroundColor(color: any): void;
    setCursorStyle(cursor_style: string): void;
    trigger(event: string, packet: any): ZRender;
    width(): number;
    readonly animation: Animation;
    readonly handler: Handler;
    readonly id: number;
}

/**
 * 清空已注册字体（主要用于测试）。
 */
export function clearFonts(): void;

export function color(): any;

export function dispose(zr: ZRender): void;

export function disposeAll(): void;

export function getInstance(id: number): ZRender | undefined;

/**
 * 创建 ZRender 实例（dom 参数忽略，尺寸来自 opts）
 */
export function init(dom: any, opts: any): ZRender;

export function main(): void;

export function matrix(): any;

export function morph(): any;

export function parseGeoJSON(geo_json: any, name_property: any): any;

export function parseGeoJson(geo_json: any, name_property: any): any;

export function parseSVG(): any;

export function path(): any;

export function registerCustomSeriesType(ty: string): void;

/**
 * 向全局 fontdb 注册字体文件。
 *
 * `opts` 可选字段：
 * - `familyName`: 覆盖字体族名
 * - `sansSerif`: `string[]`，将 CSS `sans-serif` 映射到这些族名
 */
export function registerFont(data: Uint8Array, opts: any): void;

export function registerMap(name: string, geo_json: any, special_areas: any): void;

export function registerTransform(ty: string, func: Function): void;

export function setPlatformAPI(_api: any): void;

export function showDebugDirtyRect(): void;

export function util(): any;

export function vector(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_customseriesapi_free: (a: number, b: number) => void;
    readonly customseriesapi_coord: (a: number, b: any) => any;
    readonly customseriesapi_size: (a: number, b: any, c: any) => any;
    readonly customseriesapi_style: (a: number, b: any) => any;
    readonly customseriesapi_value: (a: number, b: any) => number;
    readonly __wbg_echartsinstance_free: (a: number, b: number) => void;
    readonly echartsinstance_appendData: (a: number, b: any) => [number, number];
    readonly echartsinstance_apply_data_zoom_wheel: (a: number, b: number, c: number) => [number, number];
    readonly echartsinstance_attachHost: (a: number, b: any) => [number, number];
    readonly echartsinstance_benchmark_render: (a: number, b: number) => number;
    readonly echartsinstance_containPixel: (a: number, b: any, c: any) => number;
    readonly echartsinstance_convert_from_pixel: (a: number, b: any, c: any) => any;
    readonly echartsinstance_convert_to_pixel: (a: number, b: any, c: any) => any;
    readonly echartsinstance_dispatch_action: (a: number, b: any) => [number, number];
    readonly echartsinstance_dispose: (a: number) => void;
    readonly echartsinstance_dpr: (a: number) => number;
    readonly echartsinstance_find_hover: (a: number, b: number, c: number) => any;
    readonly echartsinstance_getZr: (a: number) => number;
    readonly echartsinstance_get_option: (a: number) => [number, number, number];
    readonly echartsinstance_get_tooltip_content: (a: number, b: number, c: number) => any;
    readonly echartsinstance_handlePointerClick: (a: number, b: number, c: number) => any;
    readonly echartsinstance_handlePointerDown: (a: number, b: number, c: number) => any;
    readonly echartsinstance_handlePointerUp: (a: number, b: number, c: number) => [number, number];
    readonly echartsinstance_handle_pointer_leave: (a: number) => [number, number];
    readonly echartsinstance_handle_pointer_move: (a: number, b: number, c: number) => any;
    readonly echartsinstance_has_option: (a: number) => number;
    readonly echartsinstance_height: (a: number) => number;
    readonly echartsinstance_new: (a: number, b: number, c: number) => [number, number, number];
    readonly echartsinstance_option_has_functions: (a: number) => number;
    readonly echartsinstance_refresh: (a: number) => [number, number, number, number];
    readonly echartsinstance_resize: (a: number, b: number, c: number, d: number) => [number, number];
    readonly echartsinstance_set_option: (a: number, b: any, c: number) => [number, number];
    readonly echartsinstance_update_font_database: (a: number) => [number, number];
    readonly echartsinstance_width: (a: number) => number;
    readonly main: () => void;
    readonly parseGeoJSON: (a: any, b: any) => any;
    readonly parseGeoJson: (a: any, b: any) => any;
    readonly registerCustomSeriesType: (a: number, b: number) => void;
    readonly registerMap: (a: number, b: number, c: any, d: any) => void;
    readonly registerTransform: (a: number, b: number, c: any) => void;
    readonly __wbg_hoverresult_free: (a: number, b: number) => void;
    readonly hoverresult_target: (a: number) => number;
    readonly hoverresult_topTarget: (a: number) => number;
    readonly clearFonts: () => [number, number];
    readonly registerFont: (a: number, b: number, c: any) => [number, number];
    readonly __wbg_displayable_free: (a: number, b: number) => void;
    readonly displayable_new: (a: any) => [number, number, number];
    readonly __wbg_group_free: (a: number, b: number) => void;
    readonly group_add: (a: number, b: any) => [number, number];
    readonly group_addBefore: (a: number, b: any, c: any) => [number, number];
    readonly group_animate: (a: number, b: any, c: any) => number;
    readonly group_attr: (a: number, b: any, c: any) => number;
    readonly group_draggable: (a: number) => any;
    readonly group_getBoundingRect: (a: number) => number;
    readonly group_hide: (a: number) => void;
    readonly group_id: (a: number) => number;
    readonly group_new: () => number;
    readonly group_off: (a: number, b: any, c: any) => number;
    readonly group_on: (a: number, b: number, c: number, d: any) => number;
    readonly group_position: (a: number) => any;
    readonly group_remove: (a: number, b: any) => [number, number];
    readonly group_removeAll: (a: number) => [number, number];
    readonly group_removeClipPath: (a: number) => number;
    readonly group_replace: (a: number, b: any, c: any) => [number, number];
    readonly group_setClipPath: (a: number, b: any) => number;
    readonly group_set_draggable: (a: number, b: any) => void;
    readonly group_set_position: (a: number, b: any) => void;
    readonly group_show: (a: number) => void;
    readonly group_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly group_type: (a: number) => [number, number];
    readonly __wbg_orientedboundingrect_free: (a: number, b: number) => void;
    readonly orientedboundingrect_fromBoundingRect: (a: number, b: number, c: any) => void;
    readonly orientedboundingrect_intersect: (a: number, b: number, c: any, d: any) => number;
    readonly orientedboundingrect_new: (a: any, b: any) => number;
    readonly __wbg_text_free: (a: number, b: number) => void;
    readonly text_animate: (a: number, b: any, c: any) => number;
    readonly text_attr: (a: number, b: any, c: any) => number;
    readonly text_getBoundingRect: (a: number) => number;
    readonly text_hide: (a: number) => void;
    readonly text_id: (a: number) => number;
    readonly text_new: (a: any) => number;
    readonly text_off: (a: number, b: any, c: any) => number;
    readonly text_on: (a: number, b: number, c: number, d: any) => number;
    readonly text_position: (a: number) => any;
    readonly text_removeClipPath: (a: number) => number;
    readonly text_setClipPath: (a: number, b: any) => number;
    readonly text_setStyle: (a: number, b: any) => number;
    readonly text_set_position: (a: number, b: any) => void;
    readonly text_show: (a: number) => void;
    readonly text_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly text_type: (a: number) => [number, number];
    readonly __wbg_tspan_free: (a: number, b: number) => void;
    readonly tspan_animate: (a: number, b: any, c: any) => number;
    readonly tspan_attr: (a: number, b: any, c: any) => number;
    readonly tspan_hide: (a: number) => void;
    readonly tspan_id: (a: number) => number;
    readonly tspan_new: (a: any) => number;
    readonly tspan_off: (a: number, b: any, c: any) => number;
    readonly tspan_on: (a: number, b: number, c: number, d: any) => number;
    readonly tspan_show: (a: number) => void;
    readonly tspan_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly tspan_type: (a: number) => [number, number];
    readonly __wbg_animation_free: (a: number, b: number) => void;
    readonly __wbg_animator_free: (a: number, b: number) => void;
    readonly animation_off: (a: number, b: any, c: any) => void;
    readonly animation_on: (a: number, b: number, c: number, d: any) => void;
    readonly animator_delay: (a: number, b: number) => number;
    readonly animator_done: (a: number, b: any) => number;
    readonly animator_during: (a: number, b: any) => number;
    readonly animator_start: (a: number, b: any) => number;
    readonly animator_stop: (a: number) => void;
    readonly animator_when: (a: number, b: number, c: any) => number;
    readonly __wbg_path_free: (a: number, b: number) => void;
    readonly path_animate: (a: number, b: any, c: any) => number;
    readonly path_attr: (a: number, b: any, c: any) => number;
    readonly path_draggable: (a: number) => any;
    readonly path_getBoundingRect: (a: number) => number;
    readonly path_hide: (a: number) => void;
    readonly path_id: (a: number) => number;
    readonly path_new: (a: any) => [number, number, number];
    readonly path_off: (a: number, b: any, c: any) => number;
    readonly path_on: (a: number, b: number, c: number, d: any) => number;
    readonly path_position: (a: number) => any;
    readonly path_removeClipPath: (a: number) => number;
    readonly path_setClipPath: (a: number, b: any) => number;
    readonly path_setShape: (a: number, b: any) => number;
    readonly path_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly path_setStyle: (a: number, b: any) => number;
    readonly path_set_draggable: (a: number, b: any) => void;
    readonly path_set_position: (a: number, b: any) => void;
    readonly path_show: (a: number) => void;
    readonly path_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly path_type: (a: number) => [number, number];
    readonly path_useState: (a: number, b: number, c: number) => [number, number];
    readonly path_useStates: (a: number, b: any) => [number, number];
    readonly __wbg_incrementaldisplayable_free: (a: number, b: number) => void;
    readonly incrementaldisplayable_new: (a: any) => [number, number, number];
    readonly __wbg_arc_free: (a: number, b: number) => void;
    readonly __wbg_beziercurve_free: (a: number, b: number) => void;
    readonly __wbg_circle_free: (a: number, b: number) => void;
    readonly __wbg_compoundpath_free: (a: number, b: number) => void;
    readonly __wbg_droplet_free: (a: number, b: number) => void;
    readonly __wbg_ellipse_free: (a: number, b: number) => void;
    readonly __wbg_heart_free: (a: number, b: number) => void;
    readonly __wbg_isogon_free: (a: number, b: number) => void;
    readonly __wbg_line_free: (a: number, b: number) => void;
    readonly __wbg_polygon_free: (a: number, b: number) => void;
    readonly __wbg_polyline_free: (a: number, b: number) => void;
    readonly __wbg_rect_free: (a: number, b: number) => void;
    readonly __wbg_ring_free: (a: number, b: number) => void;
    readonly __wbg_rose_free: (a: number, b: number) => void;
    readonly __wbg_sector_free: (a: number, b: number) => void;
    readonly __wbg_star_free: (a: number, b: number) => void;
    readonly __wbg_trochoid_free: (a: number, b: number) => void;
    readonly arc_animate: (a: number, b: any, c: any) => number;
    readonly arc_attr: (a: number, b: any, c: any) => number;
    readonly arc_draggable: (a: number) => any;
    readonly arc_getBoundingRect: (a: number) => number;
    readonly arc_hide: (a: number) => void;
    readonly arc_id: (a: number) => number;
    readonly arc_new: (a: any) => [number, number, number];
    readonly arc_off: (a: number, b: any, c: any) => number;
    readonly arc_on: (a: number, b: number, c: number, d: any) => number;
    readonly arc_position: (a: number) => any;
    readonly arc_removeClipPath: (a: number) => number;
    readonly arc_setClipPath: (a: number, b: any) => number;
    readonly arc_setShape: (a: number, b: any) => number;
    readonly arc_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly arc_setStyle: (a: number, b: any) => number;
    readonly arc_set_draggable: (a: number, b: any) => void;
    readonly arc_set_position: (a: number, b: any) => void;
    readonly arc_show: (a: number) => void;
    readonly arc_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly arc_type: (a: number) => [number, number];
    readonly arc_useState: (a: number, b: number, c: number) => [number, number];
    readonly arc_useStates: (a: number, b: any) => [number, number];
    readonly beziercurve_animate: (a: number, b: any, c: any) => number;
    readonly beziercurve_attr: (a: number, b: any, c: any) => number;
    readonly beziercurve_draggable: (a: number) => any;
    readonly beziercurve_getBoundingRect: (a: number) => number;
    readonly beziercurve_hide: (a: number) => void;
    readonly beziercurve_id: (a: number) => number;
    readonly beziercurve_new: (a: any) => [number, number, number];
    readonly beziercurve_off: (a: number, b: any, c: any) => number;
    readonly beziercurve_on: (a: number, b: number, c: number, d: any) => number;
    readonly beziercurve_position: (a: number) => any;
    readonly beziercurve_removeClipPath: (a: number) => number;
    readonly beziercurve_setClipPath: (a: number, b: any) => number;
    readonly beziercurve_setShape: (a: number, b: any) => number;
    readonly beziercurve_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly beziercurve_setStyle: (a: number, b: any) => number;
    readonly beziercurve_set_draggable: (a: number, b: any) => void;
    readonly beziercurve_set_position: (a: number, b: any) => void;
    readonly beziercurve_show: (a: number) => void;
    readonly beziercurve_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly beziercurve_type: (a: number) => [number, number];
    readonly beziercurve_useState: (a: number, b: number, c: number) => [number, number];
    readonly beziercurve_useStates: (a: number, b: any) => [number, number];
    readonly circle_animate: (a: number, b: any, c: any) => number;
    readonly circle_attr: (a: number, b: any, c: any) => number;
    readonly circle_draggable: (a: number) => any;
    readonly circle_getBoundingRect: (a: number) => number;
    readonly circle_hide: (a: number) => void;
    readonly circle_id: (a: number) => number;
    readonly circle_new: (a: any) => [number, number, number];
    readonly circle_off: (a: number, b: any, c: any) => number;
    readonly circle_on: (a: number, b: number, c: number, d: any) => number;
    readonly circle_position: (a: number) => any;
    readonly circle_removeClipPath: (a: number) => number;
    readonly circle_setClipPath: (a: number, b: any) => number;
    readonly circle_setShape: (a: number, b: any) => number;
    readonly circle_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly circle_setStyle: (a: number, b: any) => number;
    readonly circle_set_draggable: (a: number, b: any) => void;
    readonly circle_set_position: (a: number, b: any) => void;
    readonly circle_show: (a: number) => void;
    readonly circle_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly circle_type: (a: number) => [number, number];
    readonly circle_useState: (a: number, b: number, c: number) => [number, number];
    readonly circle_useStates: (a: number, b: any) => [number, number];
    readonly compoundpath_animate: (a: number, b: any, c: any) => number;
    readonly compoundpath_attr: (a: number, b: any, c: any) => number;
    readonly compoundpath_draggable: (a: number) => any;
    readonly compoundpath_getBoundingRect: (a: number) => number;
    readonly compoundpath_hide: (a: number) => void;
    readonly compoundpath_id: (a: number) => number;
    readonly compoundpath_new: (a: any) => [number, number, number];
    readonly compoundpath_off: (a: number, b: any, c: any) => number;
    readonly compoundpath_on: (a: number, b: number, c: number, d: any) => number;
    readonly compoundpath_position: (a: number) => any;
    readonly compoundpath_removeClipPath: (a: number) => number;
    readonly compoundpath_setClipPath: (a: number, b: any) => number;
    readonly compoundpath_setShape: (a: number, b: any) => number;
    readonly compoundpath_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly compoundpath_setStyle: (a: number, b: any) => number;
    readonly compoundpath_set_draggable: (a: number, b: any) => void;
    readonly compoundpath_set_position: (a: number, b: any) => void;
    readonly compoundpath_show: (a: number) => void;
    readonly compoundpath_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly compoundpath_type: (a: number) => [number, number];
    readonly compoundpath_useState: (a: number, b: number, c: number) => [number, number];
    readonly compoundpath_useStates: (a: number, b: any) => [number, number];
    readonly droplet_animate: (a: number, b: any, c: any) => number;
    readonly droplet_attr: (a: number, b: any, c: any) => number;
    readonly droplet_draggable: (a: number) => any;
    readonly droplet_getBoundingRect: (a: number) => number;
    readonly droplet_hide: (a: number) => void;
    readonly droplet_id: (a: number) => number;
    readonly droplet_new: (a: any) => [number, number, number];
    readonly droplet_off: (a: number, b: any, c: any) => number;
    readonly droplet_on: (a: number, b: number, c: number, d: any) => number;
    readonly droplet_position: (a: number) => any;
    readonly droplet_removeClipPath: (a: number) => number;
    readonly droplet_setClipPath: (a: number, b: any) => number;
    readonly droplet_setShape: (a: number, b: any) => number;
    readonly droplet_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly droplet_setStyle: (a: number, b: any) => number;
    readonly droplet_set_draggable: (a: number, b: any) => void;
    readonly droplet_set_position: (a: number, b: any) => void;
    readonly droplet_show: (a: number) => void;
    readonly droplet_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly droplet_type: (a: number) => [number, number];
    readonly droplet_useState: (a: number, b: number, c: number) => [number, number];
    readonly droplet_useStates: (a: number, b: any) => [number, number];
    readonly ellipse_animate: (a: number, b: any, c: any) => number;
    readonly ellipse_attr: (a: number, b: any, c: any) => number;
    readonly ellipse_draggable: (a: number) => any;
    readonly ellipse_getBoundingRect: (a: number) => number;
    readonly ellipse_hide: (a: number) => void;
    readonly ellipse_id: (a: number) => number;
    readonly ellipse_new: (a: any) => [number, number, number];
    readonly ellipse_off: (a: number, b: any, c: any) => number;
    readonly ellipse_on: (a: number, b: number, c: number, d: any) => number;
    readonly ellipse_position: (a: number) => any;
    readonly ellipse_removeClipPath: (a: number) => number;
    readonly ellipse_setClipPath: (a: number, b: any) => number;
    readonly ellipse_setShape: (a: number, b: any) => number;
    readonly ellipse_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly ellipse_setStyle: (a: number, b: any) => number;
    readonly ellipse_set_draggable: (a: number, b: any) => void;
    readonly ellipse_set_position: (a: number, b: any) => void;
    readonly ellipse_show: (a: number) => void;
    readonly ellipse_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly ellipse_type: (a: number) => [number, number];
    readonly ellipse_useState: (a: number, b: number, c: number) => [number, number];
    readonly ellipse_useStates: (a: number, b: any) => [number, number];
    readonly heart_animate: (a: number, b: any, c: any) => number;
    readonly heart_attr: (a: number, b: any, c: any) => number;
    readonly heart_draggable: (a: number) => any;
    readonly heart_getBoundingRect: (a: number) => number;
    readonly heart_hide: (a: number) => void;
    readonly heart_id: (a: number) => number;
    readonly heart_new: (a: any) => [number, number, number];
    readonly heart_off: (a: number, b: any, c: any) => number;
    readonly heart_on: (a: number, b: number, c: number, d: any) => number;
    readonly heart_position: (a: number) => any;
    readonly heart_removeClipPath: (a: number) => number;
    readonly heart_setClipPath: (a: number, b: any) => number;
    readonly heart_setShape: (a: number, b: any) => number;
    readonly heart_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly heart_setStyle: (a: number, b: any) => number;
    readonly heart_set_draggable: (a: number, b: any) => void;
    readonly heart_set_position: (a: number, b: any) => void;
    readonly heart_show: (a: number) => void;
    readonly heart_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly heart_type: (a: number) => [number, number];
    readonly heart_useState: (a: number, b: number, c: number) => [number, number];
    readonly heart_useStates: (a: number, b: any) => [number, number];
    readonly isogon_animate: (a: number, b: any, c: any) => number;
    readonly isogon_attr: (a: number, b: any, c: any) => number;
    readonly isogon_draggable: (a: number) => any;
    readonly isogon_getBoundingRect: (a: number) => number;
    readonly isogon_hide: (a: number) => void;
    readonly isogon_id: (a: number) => number;
    readonly isogon_new: (a: any) => [number, number, number];
    readonly isogon_off: (a: number, b: any, c: any) => number;
    readonly isogon_on: (a: number, b: number, c: number, d: any) => number;
    readonly isogon_position: (a: number) => any;
    readonly isogon_removeClipPath: (a: number) => number;
    readonly isogon_setClipPath: (a: number, b: any) => number;
    readonly isogon_setShape: (a: number, b: any) => number;
    readonly isogon_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly isogon_setStyle: (a: number, b: any) => number;
    readonly isogon_set_draggable: (a: number, b: any) => void;
    readonly isogon_set_position: (a: number, b: any) => void;
    readonly isogon_show: (a: number) => void;
    readonly isogon_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly isogon_type: (a: number) => [number, number];
    readonly isogon_useState: (a: number, b: number, c: number) => [number, number];
    readonly isogon_useStates: (a: number, b: any) => [number, number];
    readonly line_animate: (a: number, b: any, c: any) => number;
    readonly line_attr: (a: number, b: any, c: any) => number;
    readonly line_draggable: (a: number) => any;
    readonly line_getBoundingRect: (a: number) => number;
    readonly line_hide: (a: number) => void;
    readonly line_id: (a: number) => number;
    readonly line_new: (a: any) => [number, number, number];
    readonly line_off: (a: number, b: any, c: any) => number;
    readonly line_on: (a: number, b: number, c: number, d: any) => number;
    readonly line_position: (a: number) => any;
    readonly line_removeClipPath: (a: number) => number;
    readonly line_setClipPath: (a: number, b: any) => number;
    readonly line_setShape: (a: number, b: any) => number;
    readonly line_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly line_setStyle: (a: number, b: any) => number;
    readonly line_set_draggable: (a: number, b: any) => void;
    readonly line_set_position: (a: number, b: any) => void;
    readonly line_show: (a: number) => void;
    readonly line_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly line_type: (a: number) => [number, number];
    readonly line_useState: (a: number, b: number, c: number) => [number, number];
    readonly line_useStates: (a: number, b: any) => [number, number];
    readonly polygon_animate: (a: number, b: any, c: any) => number;
    readonly polygon_attr: (a: number, b: any, c: any) => number;
    readonly polygon_draggable: (a: number) => any;
    readonly polygon_getBoundingRect: (a: number) => number;
    readonly polygon_hide: (a: number) => void;
    readonly polygon_id: (a: number) => number;
    readonly polygon_new: (a: any) => [number, number, number];
    readonly polygon_off: (a: number, b: any, c: any) => number;
    readonly polygon_on: (a: number, b: number, c: number, d: any) => number;
    readonly polygon_position: (a: number) => any;
    readonly polygon_removeClipPath: (a: number) => number;
    readonly polygon_setClipPath: (a: number, b: any) => number;
    readonly polygon_setShape: (a: number, b: any) => number;
    readonly polygon_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly polygon_setStyle: (a: number, b: any) => number;
    readonly polygon_set_draggable: (a: number, b: any) => void;
    readonly polygon_set_position: (a: number, b: any) => void;
    readonly polygon_show: (a: number) => void;
    readonly polygon_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly polygon_type: (a: number) => [number, number];
    readonly polygon_useState: (a: number, b: number, c: number) => [number, number];
    readonly polygon_useStates: (a: number, b: any) => [number, number];
    readonly polyline_animate: (a: number, b: any, c: any) => number;
    readonly polyline_attr: (a: number, b: any, c: any) => number;
    readonly polyline_draggable: (a: number) => any;
    readonly polyline_getBoundingRect: (a: number) => number;
    readonly polyline_hide: (a: number) => void;
    readonly polyline_id: (a: number) => number;
    readonly polyline_new: (a: any) => [number, number, number];
    readonly polyline_off: (a: number, b: any, c: any) => number;
    readonly polyline_on: (a: number, b: number, c: number, d: any) => number;
    readonly polyline_position: (a: number) => any;
    readonly polyline_removeClipPath: (a: number) => number;
    readonly polyline_setClipPath: (a: number, b: any) => number;
    readonly polyline_setShape: (a: number, b: any) => number;
    readonly polyline_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly polyline_setStyle: (a: number, b: any) => number;
    readonly polyline_set_draggable: (a: number, b: any) => void;
    readonly polyline_set_position: (a: number, b: any) => void;
    readonly polyline_show: (a: number) => void;
    readonly polyline_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly polyline_type: (a: number) => [number, number];
    readonly polyline_useState: (a: number, b: number, c: number) => [number, number];
    readonly polyline_useStates: (a: number, b: any) => [number, number];
    readonly rect_animate: (a: number, b: any, c: any) => number;
    readonly rect_attr: (a: number, b: any, c: any) => number;
    readonly rect_draggable: (a: number) => any;
    readonly rect_getBoundingRect: (a: number) => number;
    readonly rect_hide: (a: number) => void;
    readonly rect_id: (a: number) => number;
    readonly rect_new: (a: any) => [number, number, number];
    readonly rect_off: (a: number, b: any, c: any) => number;
    readonly rect_on: (a: number, b: number, c: number, d: any) => number;
    readonly rect_position: (a: number) => any;
    readonly rect_removeClipPath: (a: number) => number;
    readonly rect_setClipPath: (a: number, b: any) => number;
    readonly rect_setShape: (a: number, b: any) => number;
    readonly rect_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly rect_setStyle: (a: number, b: any) => number;
    readonly rect_set_draggable: (a: number, b: any) => void;
    readonly rect_set_position: (a: number, b: any) => void;
    readonly rect_show: (a: number) => void;
    readonly rect_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly rect_type: (a: number) => [number, number];
    readonly rect_useState: (a: number, b: number, c: number) => [number, number];
    readonly rect_useStates: (a: number, b: any) => [number, number];
    readonly ring_animate: (a: number, b: any, c: any) => number;
    readonly ring_attr: (a: number, b: any, c: any) => number;
    readonly ring_draggable: (a: number) => any;
    readonly ring_getBoundingRect: (a: number) => number;
    readonly ring_hide: (a: number) => void;
    readonly ring_id: (a: number) => number;
    readonly ring_new: (a: any) => [number, number, number];
    readonly ring_off: (a: number, b: any, c: any) => number;
    readonly ring_on: (a: number, b: number, c: number, d: any) => number;
    readonly ring_position: (a: number) => any;
    readonly ring_removeClipPath: (a: number) => number;
    readonly ring_setClipPath: (a: number, b: any) => number;
    readonly ring_setShape: (a: number, b: any) => number;
    readonly ring_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly ring_setStyle: (a: number, b: any) => number;
    readonly ring_set_draggable: (a: number, b: any) => void;
    readonly ring_set_position: (a: number, b: any) => void;
    readonly ring_show: (a: number) => void;
    readonly ring_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly ring_type: (a: number) => [number, number];
    readonly ring_useState: (a: number, b: number, c: number) => [number, number];
    readonly ring_useStates: (a: number, b: any) => [number, number];
    readonly rose_animate: (a: number, b: any, c: any) => number;
    readonly rose_attr: (a: number, b: any, c: any) => number;
    readonly rose_draggable: (a: number) => any;
    readonly rose_getBoundingRect: (a: number) => number;
    readonly rose_hide: (a: number) => void;
    readonly rose_id: (a: number) => number;
    readonly rose_new: (a: any) => [number, number, number];
    readonly rose_off: (a: number, b: any, c: any) => number;
    readonly rose_on: (a: number, b: number, c: number, d: any) => number;
    readonly rose_position: (a: number) => any;
    readonly rose_removeClipPath: (a: number) => number;
    readonly rose_setClipPath: (a: number, b: any) => number;
    readonly rose_setShape: (a: number, b: any) => number;
    readonly rose_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly rose_setStyle: (a: number, b: any) => number;
    readonly rose_set_draggable: (a: number, b: any) => void;
    readonly rose_set_position: (a: number, b: any) => void;
    readonly rose_show: (a: number) => void;
    readonly rose_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly rose_type: (a: number) => [number, number];
    readonly rose_useState: (a: number, b: number, c: number) => [number, number];
    readonly rose_useStates: (a: number, b: any) => [number, number];
    readonly sector_animate: (a: number, b: any, c: any) => number;
    readonly sector_attr: (a: number, b: any, c: any) => number;
    readonly sector_draggable: (a: number) => any;
    readonly sector_getBoundingRect: (a: number) => number;
    readonly sector_hide: (a: number) => void;
    readonly sector_id: (a: number) => number;
    readonly sector_new: (a: any) => [number, number, number];
    readonly sector_off: (a: number, b: any, c: any) => number;
    readonly sector_on: (a: number, b: number, c: number, d: any) => number;
    readonly sector_position: (a: number) => any;
    readonly sector_removeClipPath: (a: number) => number;
    readonly sector_setClipPath: (a: number, b: any) => number;
    readonly sector_setShape: (a: number, b: any) => number;
    readonly sector_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly sector_setStyle: (a: number, b: any) => number;
    readonly sector_set_draggable: (a: number, b: any) => void;
    readonly sector_set_position: (a: number, b: any) => void;
    readonly sector_show: (a: number) => void;
    readonly sector_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly sector_type: (a: number) => [number, number];
    readonly sector_useState: (a: number, b: number, c: number) => [number, number];
    readonly sector_useStates: (a: number, b: any) => [number, number];
    readonly star_animate: (a: number, b: any, c: any) => number;
    readonly star_attr: (a: number, b: any, c: any) => number;
    readonly star_draggable: (a: number) => any;
    readonly star_getBoundingRect: (a: number) => number;
    readonly star_hide: (a: number) => void;
    readonly star_id: (a: number) => number;
    readonly star_new: (a: any) => [number, number, number];
    readonly star_off: (a: number, b: any, c: any) => number;
    readonly star_on: (a: number, b: number, c: number, d: any) => number;
    readonly star_position: (a: number) => any;
    readonly star_removeClipPath: (a: number) => number;
    readonly star_setClipPath: (a: number, b: any) => number;
    readonly star_setShape: (a: number, b: any) => number;
    readonly star_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly star_setStyle: (a: number, b: any) => number;
    readonly star_set_draggable: (a: number, b: any) => void;
    readonly star_set_position: (a: number, b: any) => void;
    readonly star_show: (a: number) => void;
    readonly star_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly star_type: (a: number) => [number, number];
    readonly star_useState: (a: number, b: number, c: number) => [number, number];
    readonly star_useStates: (a: number, b: any) => [number, number];
    readonly trochoid_animate: (a: number, b: any, c: any) => number;
    readonly trochoid_attr: (a: number, b: any, c: any) => number;
    readonly trochoid_draggable: (a: number) => any;
    readonly trochoid_getBoundingRect: (a: number) => number;
    readonly trochoid_hide: (a: number) => void;
    readonly trochoid_id: (a: number) => number;
    readonly trochoid_new: (a: any) => [number, number, number];
    readonly trochoid_off: (a: number, b: any, c: any) => number;
    readonly trochoid_on: (a: number, b: number, c: number, d: any) => number;
    readonly trochoid_position: (a: number) => any;
    readonly trochoid_removeClipPath: (a: number) => number;
    readonly trochoid_setClipPath: (a: number, b: any) => number;
    readonly trochoid_setShape: (a: number, b: any) => number;
    readonly trochoid_setStateStyle: (a: number, b: number, c: number, d: any) => [number, number];
    readonly trochoid_setStyle: (a: number, b: any) => number;
    readonly trochoid_set_draggable: (a: number, b: any) => void;
    readonly trochoid_set_position: (a: number, b: any) => void;
    readonly trochoid_show: (a: number) => void;
    readonly trochoid_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly trochoid_type: (a: number) => [number, number];
    readonly trochoid_useState: (a: number, b: number, c: number) => [number, number];
    readonly trochoid_useStates: (a: number, b: any) => [number, number];
    readonly __wbg_point_free: (a: number, b: number) => void;
    readonly point_add: (a: number, b: number) => number;
    readonly point_clone: (a: number) => number;
    readonly point_copy: (a: number, b: number) => number;
    readonly point_distance: (a: number, b: number) => number;
    readonly point_distanceSquare: (a: number, b: number) => number;
    readonly point_dot: (a: number, b: number) => number;
    readonly point_equal: (a: number, b: number) => number;
    readonly point_from_array: (a: number, b: number, c: number) => void;
    readonly point_len: (a: number) => number;
    readonly point_lenSquare: (a: number) => number;
    readonly point_negate: (a: number) => number;
    readonly point_new: (a: number, b: number, c: number, d: number) => number;
    readonly point_normalize: (a: number) => number;
    readonly point_scale: (a: number, b: number) => void;
    readonly point_scaleAndAdd: (a: number, b: number, c: number) => void;
    readonly point_set: (a: number, b: number, c: number) => number;
    readonly point_set_x: (a: number, b: number) => void;
    readonly point_set_y: (a: number, b: number) => void;
    readonly point_sub: (a: number, b: number) => number;
    readonly point_to_array: (a: number, b: number, c: number, d: any) => [number, number];
    readonly point_transform: (a: number, b: any) => number;
    readonly point_x: (a: number) => number;
    readonly point_y: (a: number) => number;
    readonly __wbg_handler_free: (a: number, b: number) => void;
    readonly handler_dispatch: (a: number, b: number, c: number, d: any) => [number, number];
    readonly handler_setCursorStyle: (a: number, b: number, c: number) => void;
    readonly __wbg_image_free: (a: number, b: number) => void;
    readonly image_animate: (a: number, b: any, c: any) => number;
    readonly image_attr: (a: number, b: any, c: any) => number;
    readonly image_getBoundingRect: (a: number) => number;
    readonly image_hide: (a: number) => void;
    readonly image_id: (a: number) => number;
    readonly image_new: (a: any) => [number, number, number];
    readonly image_off: (a: number, b: any, c: any) => number;
    readonly image_on: (a: number, b: number, c: number, d: any) => number;
    readonly image_position: (a: number) => any;
    readonly image_removeClipPath: (a: number) => number;
    readonly image_setClipPath: (a: number, b: any) => number;
    readonly image_set_position: (a: number, b: any) => void;
    readonly image_show: (a: number) => void;
    readonly image_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly image_type: (a: number) => [number, number];
    readonly __wbg_element_free: (a: number, b: number) => void;
    readonly element_id: (a: number) => number;
    readonly element_type: (a: number) => [number, number];
    readonly color: () => any;
    readonly matrix: () => any;
    readonly morph: () => any;
    readonly parseSVG: () => any;
    readonly path: () => any;
    readonly setPlatformAPI: (a: any) => void;
    readonly showDebugDirtyRect: () => void;
    readonly util: () => any;
    readonly vector: () => any;
    readonly __wbg_boundingrect_free: (a: number, b: number) => void;
    readonly boundingrect_applyTransform: (a: number, b: any) => void;
    readonly boundingrect_calculateTransform: (a: number, b: number) => [number, number];
    readonly boundingrect_clone: (a: number) => number;
    readonly boundingrect_contain: (a: number, b: number, c: number) => number;
    readonly boundingrect_copy: (a: number, b: any) => void;
    readonly boundingrect_create: (a: any) => number;
    readonly boundingrect_height: (a: number) => number;
    readonly boundingrect_intersect: (a: number, b: number, c: any) => number;
    readonly boundingrect_isFinite: (a: number) => number;
    readonly boundingrect_isZero: (a: number) => number;
    readonly boundingrect_new: (a: number, b: number, c: number, d: number) => number;
    readonly boundingrect_plain: (a: number) => any;
    readonly boundingrect_set: (a: any, b: number, c: number, d: number, e: number) => void;
    readonly boundingrect_set_height: (a: number, b: number) => void;
    readonly boundingrect_set_width: (a: number, b: number) => void;
    readonly boundingrect_set_x: (a: number, b: number) => void;
    readonly boundingrect_set_y: (a: number, b: number) => void;
    readonly boundingrect_union: (a: number, b: number) => void;
    readonly boundingrect_width: (a: number) => number;
    readonly boundingrect_x: (a: number) => number;
    readonly boundingrect_y: (a: number) => number;
    readonly __wbg_zrender_free: (a: number, b: number) => void;
    readonly dispose: (a: number) => void;
    readonly disposeAll: () => void;
    readonly getInstance: (a: number) => number;
    readonly init: (a: any, b: any) => [number, number, number];
    readonly zrender_add: (a: number, b: any) => [number, number];
    readonly zrender_animation: (a: number) => number;
    readonly zrender_clear: (a: number) => [number, number];
    readonly zrender_configLayer: (a: number, b: any, c: any) => [number, number];
    readonly zrender_dispose: (a: number) => void;
    readonly zrender_dpr: (a: number) => number;
    readonly zrender_findHover: (a: number, b: number, c: number) => number;
    readonly zrender_flush: (a: number) => [number, number, number, number];
    readonly zrender_getBackgroundColor: (a: number) => any;
    readonly zrender_getHeight: (a: number) => number;
    readonly zrender_getWidth: (a: number) => number;
    readonly zrender_handler: (a: number) => number;
    readonly zrender_height: (a: number) => number;
    readonly zrender_id: (a: number) => number;
    readonly zrender_off: (a: number, b: any, c: any) => number;
    readonly zrender_on: (a: number, b: number, c: number, d: any) => number;
    readonly zrender_refresh: (a: number) => [number, number, number, number];
    readonly zrender_remove: (a: number, b: any) => [number, number];
    readonly zrender_resize: (a: number, b: any) => [number, number];
    readonly zrender_setBackgroundColor: (a: number, b: any) => [number, number];
    readonly zrender_setCursorStyle: (a: number, b: number, c: number) => void;
    readonly zrender_trigger: (a: number, b: number, c: number, d: any) => number;
    readonly zrender_width: (a: number) => number;
    readonly __wbg_lineargradient_free: (a: number, b: number) => void;
    readonly __wbg_pattern_free: (a: number, b: number) => void;
    readonly __wbg_radialgradient_free: (a: number, b: number) => void;
    readonly lineargradient_addColorStop: (a: number, b: number, c: number, d: number) => void;
    readonly lineargradient_colorStops: (a: number) => any;
    readonly lineargradient_global: (a: number) => number;
    readonly lineargradient_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly lineargradient_type: (a: number) => [number, number];
    readonly lineargradient_x: (a: number) => number;
    readonly lineargradient_x2: (a: number) => number;
    readonly lineargradient_y: (a: number) => number;
    readonly lineargradient_y2: (a: number) => number;
    readonly pattern_imageData: (a: number) => any;
    readonly pattern_imageHeight: (a: number) => number;
    readonly pattern_imageWidth: (a: number) => number;
    readonly pattern_new: (a: any, b: number, c: number) => [number, number, number];
    readonly pattern_repeat: (a: number) => [number, number];
    readonly pattern_rotation: (a: number) => number;
    readonly pattern_scaleX: (a: number) => number;
    readonly pattern_scaleY: (a: number) => number;
    readonly pattern_type: (a: number) => [number, number];
    readonly pattern_x: (a: number) => number;
    readonly pattern_y: (a: number) => number;
    readonly radialgradient_addColorStop: (a: number, b: number, c: number, d: number) => void;
    readonly radialgradient_colorStops: (a: number) => any;
    readonly radialgradient_global: (a: number) => number;
    readonly radialgradient_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
    readonly radialgradient_r: (a: number) => number;
    readonly radialgradient_r0: (a: number) => number;
    readonly radialgradient_set_r0: (a: number, b: number) => void;
    readonly radialgradient_type: (a: number) => [number, number];
    readonly radialgradient_x: (a: number) => number;
    readonly radialgradient_y: (a: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__heeefd82ab16651e1: (a: number, b: number, c: any) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
