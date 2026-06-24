//! Element 基类：wasm 侧图元引用（handle + 元数据）

pub(crate) mod js;
pub(crate) mod pending;

use wasm_bindgen::prelude::*;

use crate::registry::{ElementKind, ELEMENT_REGISTRY};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Element {
    id: u32,
}

impl Element {
    pub(crate) fn from_id(id: u32) -> Self {
        Self { id }
    }

    pub(crate) fn raw_id(&self) -> u32 {
        self.id
    }
}

#[wasm_bindgen]
impl Element {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn element_type(&self) -> String {
        ELEMENT_REGISTRY.with(|reg| {
            reg.borrow()
                .element_type(self.id)
                .unwrap_or_else(|| "unknown".into())
        })
    }
}

pub(crate) fn require_element(id: u32) -> Result<Element, JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        if reg.borrow().contains(id) {
            Ok(Element::from_id(id))
        } else {
            Err(JsValue::from_str("invalid element"))
        }
    })
}

pub(crate) fn element_kind(id: u32) -> Result<ElementKind, JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow()
            .kind(id)
            .ok_or_else(|| JsValue::from_str("invalid element"))
    })
}
