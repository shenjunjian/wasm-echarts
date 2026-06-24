//! Text 图元

use wasm_bindgen::prelude::*;

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
}
