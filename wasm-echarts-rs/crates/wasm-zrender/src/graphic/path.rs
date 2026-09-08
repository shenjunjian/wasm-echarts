//! Path 基类：状态样式 API

use wasm_bindgen::prelude::*;

use crate::bridge::opts::parse_path_style_patch;
use crate::element::api;
use crate::animation::Animator;
use crate::graphic::BoundingRect;
use crate::registry::{path_set_state_style, path_use_state, register_path};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Path {
    id: u32,
}

impl Path {
    pub(crate) fn from_id(id: u32) -> Self {
        Self { id }
    }

    pub(crate) fn raw_id(&self) -> u32 {
        self.id
    }

    pub(crate) fn use_state_inner(&self, state: &str) -> Result<(), JsValue> {
        path_use_state(self.id, state)
    }

    pub(crate) fn set_state_style_inner(&self, state: &str, style: &JsValue) -> Result<(), JsValue> {
        let patch = parse_path_style_patch(style);
        path_set_state_style(self.id, state, patch)
    }
}

#[wasm_bindgen]
impl Path {
    #[wasm_bindgen(constructor)]
    pub fn new(opts: JsValue) -> Result<Path, JsValue> {
        let element = register_path("path", &opts)?;
        Ok(Path::from_id(element.raw_id()))
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn element_type(&self) -> String {
        "path".to_string()
    }

    #[wasm_bindgen(js_name = useState)]
    pub fn use_state(&self, state: &str) -> Result<(), JsValue> {
        self.use_state_inner(state)
    }

    #[wasm_bindgen(js_name = useStates)]
    pub fn use_states(&self, states: JsValue) -> Result<(), JsValue> {
        api::element_use_states(self.id, states)
    }

    #[wasm_bindgen(js_name = setStateStyle)]
    pub fn set_state_style(&self, state: &str, style: JsValue) -> Result<(), JsValue> {
        self.set_state_style_inner(state, &style)
    }

    pub fn animate(&self, path: JsValue, looping: JsValue) -> Animator {
        api::element_animate(self.id, path, looping)
    }

    pub fn on(&self, event: &str, handler: JsValue) -> Path {
        api::element_on(self.id, event, handler);
        Path::from_id(self.id)
    }

    pub fn off(&self, event: JsValue, handler: JsValue) -> Path {
        api::element_off(self.id, event, handler);
        Path::from_id(self.id)
    }

    pub fn trigger(&self, event: &str, packet: JsValue) -> Path {
        api::element_trigger(self.id, event, packet);
        Path::from_id(self.id)
    }

    pub fn hide(&self) {
        let _ = api::element_hide(self.id);
    }

    pub fn show(&self) {
        let _ = api::element_show(self.id);
    }

    #[wasm_bindgen(getter)]
    pub fn draggable(&self) -> JsValue {
        crate::handler::element_draggable_js(self.id)
    }

    #[wasm_bindgen(setter)]
    pub fn set_draggable(&self, value: JsValue) {
        crate::handler::element_set_draggable(self.id, &value);
    }

    pub fn attr(&self, key: JsValue, value: JsValue) -> Path {
        let _ = api::element_attr(self.id, key, value);
        Path::from_id(self.id)
    }

    #[wasm_bindgen(js_name = setShape)]
    pub fn set_shape(&self, shape: JsValue) -> Path {
        let _ = api::element_set_shape(self.id, &shape);
        Path::from_id(self.id)
    }

    #[wasm_bindgen(js_name = setStyle)]
    pub fn set_style(&self, style: JsValue) -> Path {
        let _ = api::element_set_style(self.id, &style);
        Path::from_id(self.id)
    }

    #[wasm_bindgen(js_name = setClipPath)]
    pub fn set_clip_path(&self, clip: JsValue) -> Path {
        let _ = api::element_set_clip_path(self.id, clip);
        Path::from_id(self.id)
    }

    #[wasm_bindgen(js_name = removeClipPath)]
    pub fn remove_clip_path(&self) -> Path {
        let _ = api::element_remove_clip_path(self.id);
        Path::from_id(self.id)
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
