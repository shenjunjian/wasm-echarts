//! BoundingRect 值对象（对齐 zrender export.ts）

use rust_zrender::core::bbox::BoundingRect as InnerRect;
use wasm_bindgen::prelude::*;

use super::geometry_util::{parse_intersect_opt, parse_matrix, parse_rect_like, rect_to_js};

#[wasm_bindgen]
pub struct BoundingRect {
    inner: InnerRect,
}

#[wasm_bindgen]
impl BoundingRect {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> BoundingRect {
        BoundingRect {
            inner: InnerRect::new(x, y, width, height),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        self.inner.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: f64) {
        self.inner.y = y;
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 {
        self.inner.width
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, width: f64) {
        self.inner.width = width;
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 {
        self.inner.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, height: f64) {
        self.inner.height = height;
    }

    pub fn union(&mut self, other: &BoundingRect) {
        self.inner.union(&other.inner);
    }

    #[wasm_bindgen(js_name = applyTransform)]
    pub fn apply_transform(&mut self, matrix: JsValue) {
        let source = self.inner;
        InnerRect::apply_transform(&mut self.inner, &source, parse_matrix(&matrix).as_ref());
    }

    pub fn intersect(&self, other: &BoundingRect, opt: JsValue) -> bool {
        let touch_threshold = if opt.is_null() || opt.is_undefined() {
            0.0
        } else {
            parse_intersect_opt(&opt).touch_threshold
        };
        InnerRect::intersect(&self.inner, &other.inner, touch_threshold)
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        self.inner.contain(x, y)
    }

    pub fn clone(&self) -> BoundingRect {
        BoundingRect { inner: self.inner }
    }

    pub fn copy(&mut self, other: JsValue) {
        if let Some(rect) = parse_rect_like(&other) {
            self.inner.copy_from(&rect);
        }
    }

    pub fn plain(&self) -> JsValue {
        rect_to_js(&self.inner)
    }

    #[wasm_bindgen(js_name = isFinite)]
    pub fn is_finite(&self) -> bool {
        self.inner.is_finite()
    }

    #[wasm_bindgen(js_name = isZero)]
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    #[wasm_bindgen(js_name = create)]
    pub fn create(rect: JsValue) -> BoundingRect {
        if let Some(rect) = parse_rect_like(&rect) {
            BoundingRect { inner: rect }
        } else {
            BoundingRect {
                inner: InnerRect::default(),
            }
        }
    }

    #[wasm_bindgen(js_name = set)]
    pub fn set_rect(target: JsValue, x: f64, y: f64, width: f64, height: f64) {
        if let Some(obj) = target.dyn_ref::<js_sys::Object>() {
            let mut w = width;
            let mut h = height;
            let mut nx = x;
            let mut ny = y;
            if w < 0.0 {
                nx += w;
                w = -w;
            }
            if h < 0.0 {
                ny += h;
                h = -h;
            }
            let _ = js_sys::Reflect::set(obj, &"x".into(), &JsValue::from(nx));
            let _ = js_sys::Reflect::set(obj, &"y".into(), &JsValue::from(ny));
            let _ = js_sys::Reflect::set(obj, &"width".into(), &JsValue::from(w));
            let _ = js_sys::Reflect::set(obj, &"height".into(), &JsValue::from(h));
        }
    }
}

impl BoundingRect {
    pub(crate) fn inner(&self) -> InnerRect {
        self.inner
    }
}
