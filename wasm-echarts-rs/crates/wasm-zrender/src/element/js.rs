//! 从 JS 图元对象读取 registry id

use wasm_bindgen::prelude::*;

use super::Element;
use crate::registry::ELEMENT_REGISTRY;

pub fn element_id_from_js(v: &JsValue) -> Result<u32, JsValue> {
    js_sys::Reflect::get(v, &JsValue::from_str("id"))
        .ok()
        .and_then(|id| id.as_f64().map(|n| n as u32))
        .filter(|id| ELEMENT_REGISTRY.with(|reg| reg.borrow().contains(*id)))
        .ok_or_else(|| JsValue::from_str("expected a zrender element with valid id"))
}

pub fn element_from_js(v: &JsValue) -> Result<Element, JsValue> {
    Ok(Element::from_id(element_id_from_js(v)?))
}
