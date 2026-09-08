//! Text 图元

use wasm_bindgen::prelude::*;

use crate::element::api;
use crate::animation::Animator;
use crate::graphic::BoundingRect;
use crate::registry::register_text;

#[wasm_bindgen]
pub struct Text {
    id: u32,
}

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(opts: JsValue) -> Text {
        let element = register_text(&opts);
        Text {
            id: element.raw_id(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn element_type(&self) -> String {
        "text".into()
    }

    pub fn animate(&self, path: JsValue, looping: JsValue) -> Animator {
        api::element_animate(self.id, path, looping)
    }

    pub fn on(&self, event: &str, handler: JsValue) -> Text {
        api::element_on(self.id, event, handler);
        Text { id: self.id }
    }

    pub fn off(&self, event: JsValue, handler: JsValue) -> Text {
        api::element_off(self.id, event, handler);
        Text { id: self.id }
    }

    pub fn trigger(&self, event: &str, packet: JsValue) -> Text {
        api::element_trigger(self.id, event, packet);
        Text { id: self.id }
    }

    pub fn hide(&self) {
        let _ = api::element_hide(self.id);
    }

    pub fn show(&self) {
        let _ = api::element_show(self.id);
    }

    pub fn attr(&self, key: JsValue, value: JsValue) -> Text {
        let _ = api::element_attr(self.id, key, value);
        Text { id: self.id }
    }

    #[wasm_bindgen(js_name = setStyle)]
    pub fn set_style(&self, style: JsValue) -> Text {
        let _ = api::element_set_style(self.id, &style);
        Text { id: self.id }
    }

    #[wasm_bindgen(js_name = getBoundingRect)]
    pub fn get_bounding_rect(&self) -> BoundingRect {
        BoundingRect::from_inner(api::element_get_bounding_rect(self.id))
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsValue {
        api::element_position_js(self.id)
    }

    #[wasm_bindgen(setter)]
    pub fn set_position(&self, value: JsValue) {
        let _ = api::element_set_position(self.id, &value);
    }
}
