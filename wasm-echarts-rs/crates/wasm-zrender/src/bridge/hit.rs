//! HitResult → HoverResult（{ target, topTarget }）

use rust_zrender::HitResult;
use wasm_bindgen::prelude::*;

use crate::element::Element;
use crate::registry::{ElementKind, ELEMENT_REGISTRY};

#[wasm_bindgen]
pub struct HoverResult {
    target: Element,
    top_target: Element,
}

#[wasm_bindgen]
impl HoverResult {
    #[wasm_bindgen(getter)]
    pub fn target(&self) -> Element {
        self.target
    }

    #[wasm_bindgen(getter, js_name = topTarget)]
    pub fn top_target(&self) -> Element {
        self.top_target
    }
}

pub fn hit_to_hover_result(hit: &HitResult) -> Option<HoverResult> {
    let target = element_from_path_index(hit.path_index)?;
    let top_target = element_from_path_index(hit.top_path_index)?;
    Some(HoverResult {
        target,
        top_target,
    })
}

fn element_from_path_index(path_index: usize) -> Option<Element> {
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow()
            .find_by_storage(ElementKind::Path, path_index)
            .map(Element::from_id)
    })
}
