//! Image 图元 wasm 类

use wasm_bindgen::prelude::*;

use crate::element::api;
use crate::animation::Animator;
use crate::graphic::BoundingRect;
use crate::registry::register_image;

#[wasm_bindgen]
pub struct Image {
    id: u32,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(opts: JsValue) -> Result<Image, JsValue> {
        let element = register_image(&opts)?;
        Ok(Image {
            id: element.raw_id(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn element_type(&self) -> String {
        "image".to_string()
    }

    pub fn animate(&self, path: JsValue, looping: JsValue) -> Animator {
        api::element_animate(self.id, path, looping)
    }

    pub fn on(&self, event: &str, handler: JsValue) -> Image {
        api::element_on(self.id, event, handler);
        Image { id: self.id }
    }

    pub fn off(&self, event: JsValue, handler: JsValue) -> Image {
        api::element_off(self.id, event, handler);
        Image { id: self.id }
    }

    pub fn trigger(&self, event: &str, packet: JsValue) -> Image {
        api::element_trigger(self.id, event, packet);
        Image { id: self.id }
    }

    pub fn hide(&self) {
        let _ = api::element_hide(self.id);
    }

    pub fn show(&self) {
        let _ = api::element_show(self.id);
    }

    pub fn attr(&self, key: JsValue, value: JsValue) -> Image {
        let _ = api::element_attr(self.id, key, value);
        Image { id: self.id }
    }

    #[wasm_bindgen(js_name = setClipPath)]
    pub fn set_clip_path(&self, clip: JsValue) -> Image {
        let _ = api::element_set_clip_path(self.id, clip);
        Image { id: self.id }
    }

    #[wasm_bindgen(js_name = removeClipPath)]
    pub fn remove_clip_path(&self) -> Image {
        let _ = api::element_remove_clip_path(self.id);
        Image { id: self.id }
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
