//! TSpan 图元（MVP：单 run 文本，与 Text 共用 Storage 元素）

use wasm_bindgen::prelude::*;

use crate::element::api;
use crate::animation::Animator;
use crate::registry::register_tspan;

#[wasm_bindgen]
pub struct TSpan {
    id: u32,
}

#[wasm_bindgen]
impl TSpan {
    #[wasm_bindgen(constructor)]
    pub fn new(opts: JsValue) -> TSpan {
        let element = register_tspan(&opts);
        TSpan {
            id: element.raw_id(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn element_type(&self) -> String {
        "tspan".into()
    }

    pub fn animate(&self, path: JsValue, looping: JsValue) -> Animator {
        api::element_animate(self.id, path, looping)
    }

    pub fn on(&self, event: &str, handler: JsValue) -> TSpan {
        api::element_on(self.id, event, handler);
        TSpan { id: self.id }
    }

    pub fn attr(&self, key: JsValue, value: JsValue) -> TSpan {
        let _ = api::element_attr(self.id, key, value);
        TSpan { id: self.id }
    }
}
