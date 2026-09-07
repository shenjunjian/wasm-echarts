//! Group 容器

use wasm_bindgen::prelude::*;

use crate::element::api;
use crate::element::js::element_from_js;
use crate::animation::Animator;
use crate::graphic::BoundingRect;
use crate::registry::{group_add_child, group_remove_child, register_group, ELEMENT_REGISTRY};

#[wasm_bindgen]
pub struct Group {
    id: u32,
}

#[wasm_bindgen]
impl Group {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Group {
        let element = register_group();
        Group {
            id: element.raw_id(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn element_type(&self) -> String {
        "group".into()
    }

    pub fn add(&self, child: JsValue) -> Result<(), JsValue> {
        let child = element_from_js(&child)?;
        group_add_child(self.id, child.raw_id())
    }

    pub fn remove(&self, child: JsValue) -> Result<(), JsValue> {
        let child = element_from_js(&child)?;
        group_remove_child(self.id, child.raw_id())
    }

    #[wasm_bindgen(js_name = removeAll)]
    pub fn remove_all(&self) -> Result<(), JsValue> {
        let child_ids = ELEMENT_REGISTRY.with(|reg| reg.borrow().children_of(self.id));
        for child_id in child_ids {
            group_remove_child(self.id, child_id)?;
        }
        Ok(())
    }

    pub fn animate(&self, path: JsValue, looping: JsValue) -> Animator {
        api::element_animate(self.id, path, looping)
    }

    pub fn on(&self, event: &str, handler: JsValue) -> Group {
        api::element_on(self.id, event, handler);
        Group { id: self.id }
    }

    #[wasm_bindgen(getter)]
    pub fn draggable(&self) -> JsValue {
        crate::handler::element_draggable_js(self.id)
    }

    #[wasm_bindgen(setter)]
    pub fn set_draggable(&self, value: JsValue) {
        crate::handler::element_set_draggable(self.id, &value);
    }

    pub fn attr(&self, key: JsValue, value: JsValue) -> Group {
        let _ = api::element_attr(self.id, key, value);
        Group { id: self.id }
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
