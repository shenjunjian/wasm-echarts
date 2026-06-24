//! 几何类型 JS 解析辅助

use js_sys::{Array, Object, Reflect};
use rust_zrender::core::bbox::BoundingRect as InnerRect;
use rust_zrender::core::obb::IntersectOpt;
use wasm_bindgen::prelude::*;

pub fn parse_f64(value: &JsValue, key: &str, default: f64) -> f64 {
    Reflect::get(value, &key.into())
        .ok()
        .and_then(|v| v.as_f64())
        .filter(|v| v.is_finite())
        .unwrap_or(default)
}

pub fn parse_rect_like(value: &JsValue) -> Option<InnerRect> {
    if value.is_null() || value.is_undefined() {
        return None;
    }
    Some(InnerRect::new(
        parse_f64(value, "x", 0.0),
        parse_f64(value, "y", 0.0),
        parse_f64(value, "width", 0.0),
        parse_f64(value, "height", 0.0),
    ))
}

pub fn parse_matrix(value: &JsValue) -> Option<[f64; 6]> {
    if value.is_null() || value.is_undefined() {
        return None;
    }
    let arr = Array::from(value);
    if arr.length() < 6 {
        return None;
    }
    let mut m = [0.0; 6];
    for (idx, slot) in m.iter_mut().enumerate() {
        *slot = arr.get(idx as u32).as_f64()?;
    }
    Some(m)
}

pub fn parse_intersect_opt(value: &JsValue) -> IntersectOpt {
    let mut opt = IntersectOpt {
        bidirectional: true,
        ..Default::default()
    };
    if value.is_null() || value.is_undefined() {
        return opt;
    }
    opt.touch_threshold = parse_f64(value, "touchThreshold", 0.0);
    if let Some(direction) = Reflect::get(value, &"direction".into())
        .ok()
        .and_then(|v| v.as_f64())
    {
        opt.direction = Some(direction);
    }
    if let Some(bidirectional) = Reflect::get(value, &"bidirectional".into())
        .ok()
        .and_then(|v| v.as_bool())
    {
        opt.bidirectional = bidirectional;
    }
    opt
}

pub fn rect_to_js(rect: &InnerRect) -> JsValue {
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"x".into(), &JsValue::from(rect.x));
    let _ = Reflect::set(&obj, &"y".into(), &JsValue::from(rect.y));
    let _ = Reflect::set(&obj, &"width".into(), &JsValue::from(rect.width));
    let _ = Reflect::set(&obj, &"height".into(), &JsValue::from(rect.height));
    obj.into()
}
