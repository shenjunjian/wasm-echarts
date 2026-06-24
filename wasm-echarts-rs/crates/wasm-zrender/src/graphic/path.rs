//! Path 基类：状态样式 API（不直接构造）

use wasm_bindgen::prelude::*;

use crate::bridge::opts::parse_path_style_patch;
use crate::registry::{path_set_state_style, path_use_state};

#[wasm_bindgen]
pub struct Path {
    id: u32,
}

impl Path {
    pub(crate) fn new(id: u32) -> Self {
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
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(js_name = useState)]
    pub fn use_state(&self, state: &str) -> Result<(), JsValue> {
        self.use_state_inner(state)
    }

    #[wasm_bindgen(js_name = setStateStyle)]
    pub fn set_state_style(&self, state: &str, style: JsValue) -> Result<(), JsValue> {
        self.set_state_style_inner(state, &style)
    }
}
