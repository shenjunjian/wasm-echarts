//! pkg 侧仍导出空对象，避免 wasm-bindgen 缺符号。
//! 公开 API 走 `js/tool/`（matrix / vector / color / util / path / morph / parseSVG）。

use js_sys::Object;
use wasm_bindgen::prelude::*;

fn empty_object() -> JsValue {
    Object::new().into()
}

#[wasm_bindgen]
pub fn matrix() -> JsValue {
    empty_object()
}

#[wasm_bindgen]
pub fn vector() -> JsValue {
    empty_object()
}

#[wasm_bindgen]
pub fn color() -> JsValue {
    empty_object()
}

#[wasm_bindgen]
pub fn path() -> JsValue {
    empty_object()
}

#[wasm_bindgen]
pub fn util() -> JsValue {
    empty_object()
}

#[wasm_bindgen]
pub fn morph() -> JsValue {
    empty_object()
}

#[wasm_bindgen(js_name = parseSVG)]
pub fn parse_svg() -> JsValue {
    empty_object()
}

#[wasm_bindgen(js_name = showDebugDirtyRect)]
pub fn show_debug_dirty_rect() {}

#[wasm_bindgen(js_name = setPlatformAPI)]
pub fn set_platform_api(_api: JsValue) {}
