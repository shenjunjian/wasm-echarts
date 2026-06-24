//! 已实现的基础 shape

use wasm_bindgen::prelude::*;

use crate::bridge::opts::parse_path_style_patch;
use crate::graphic::path::Path;
use crate::registry::{path_set_state_style, path_use_state, register_path};

macro_rules! impl_shape {
    ($name:ident, $type_name:expr) => {
        #[wasm_bindgen]
        pub struct $name {
            path: Path,
        }

        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new(opts: JsValue) -> Result<$name, JsValue> {
                let element = register_path($type_name, &opts)?;
                Ok($name {
                    path: Path::new(element.raw_id()),
                })
            }

            #[wasm_bindgen(getter)]
            pub fn id(&self) -> u32 {
                self.path.raw_id()
            }

            #[wasm_bindgen(getter, js_name = type)]
            pub fn element_type(&self) -> String {
                $type_name.to_string()
            }

            #[wasm_bindgen(js_name = useState)]
            pub fn use_state(&self, state: &str) -> Result<(), JsValue> {
                path_use_state(self.path.raw_id(), state)
            }

            #[wasm_bindgen(js_name = setStateStyle)]
            pub fn set_state_style(&self, state: &str, style: JsValue) -> Result<(), JsValue> {
                let patch = parse_path_style_patch(&style);
                path_set_state_style(self.path.raw_id(), state, patch)
            }
        }
    };
}

impl_shape!(Rect, "rect");
impl_shape!(Circle, "circle");
impl_shape!(Line, "line");
impl_shape!(Polygon, "polygon");
impl_shape!(Polyline, "polyline");
impl_shape!(Sector, "sector");
