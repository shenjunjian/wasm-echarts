//! OrientedBoundingRect 值对象（对齐 zrender export.ts）

use js_sys::Reflect;
use rust_zrender::core::obb::OrientedBoundingRect as InnerObb;
use rust_zrender::core::point::Point2;
use wasm_bindgen::prelude::*;

use super::geometry_util::{parse_intersect_opt, parse_matrix, parse_rect_like};
use super::BoundingRect;

#[wasm_bindgen]
pub struct OrientedBoundingRect {
    inner: InnerObb,
}

#[wasm_bindgen]
impl OrientedBoundingRect {
    #[wasm_bindgen(constructor)]
    pub fn new(rect: JsValue, transform: JsValue) -> OrientedBoundingRect {
        let mut obb = InnerObb::new();
        if let Some(r) = parse_rect_like(&rect) {
            obb.from_bounding_rect_mut(&r, parse_matrix(&transform).as_ref());
        }
        OrientedBoundingRect { inner: obb }
    }

    #[wasm_bindgen(js_name = fromBoundingRect)]
    pub fn from_bounding_rect(&mut self, rect: &BoundingRect, transform: JsValue) {
        self.inner
            .from_bounding_rect_mut(&rect.inner(), parse_matrix(&transform).as_ref());
    }

    pub fn intersect(&self, other: &OrientedBoundingRect, mtv: JsValue, opt: JsValue) -> bool {
        let opt = parse_intersect_opt(&opt);
        let use_mtv = !mtv.is_null() && !mtv.is_undefined();
        let mut temp = Point2::new(0.0, 0.0);
        let mtv_ref = if use_mtv { Some(&mut temp) } else { None };
        let overlapped = self.inner.intersect(&other.inner, mtv_ref, &opt);
        if use_mtv {
            let _ = Reflect::set(&mtv, &"x".into(), &JsValue::from(temp.x));
            let _ = Reflect::set(&mtv, &"y".into(), &JsValue::from(temp.y));
        }
        overlapped
    }
}
