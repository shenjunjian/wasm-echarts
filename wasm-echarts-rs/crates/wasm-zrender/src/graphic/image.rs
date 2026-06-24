//! Image 图元 wasm 类

use wasm_bindgen::prelude::*;

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
}
