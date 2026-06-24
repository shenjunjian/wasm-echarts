//! HitResult → HoverResult（{ target, topTarget }）

use rust_zrender::{HitResult, HitTarget};
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
    let target = element_from_hit_target(hit.target)?;
    let top_target = element_from_hit_target(hit.top_target)?;
    Some(HoverResult {
        target,
        top_target,
    })
}

fn element_from_hit_target(target: HitTarget) -> Option<Element> {
    match target {
        HitTarget::Path(path_index) => ELEMENT_REGISTRY.with(|reg| {
            reg.borrow()
                .find_by_storage(ElementKind::Path, path_index)
                .map(Element::from_id)
        }),
        HitTarget::Image(image_index) => ELEMENT_REGISTRY.with(|reg| {
            reg.borrow()
                .find_by_storage(ElementKind::Image, image_index)
                .map(Element::from_id)
        }),
        HitTarget::Text(text_index) => ELEMENT_REGISTRY.with(|reg| {
            reg.borrow()
                .find_by_storage(ElementKind::Text, text_index)
                .map(Element::from_id)
        }),
    }
}
