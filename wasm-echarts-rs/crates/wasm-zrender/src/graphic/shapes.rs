//! 已实现的基础 shape

use wasm_bindgen::prelude::*;

use crate::bridge::opts::parse_path_style_patch;
use crate::element::api;
use crate::animation::Animator;
use crate::graphic::path::Path;
use crate::graphic::BoundingRect;
use crate::registry::{path_set_state_style, path_use_state, register_path};

macro_rules! impl_shape {
    ($name:ident, $type_name:expr) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $name {
            path: Path,
        }

        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new(opts: JsValue) -> Result<$name, JsValue> {
                let element = register_path($type_name, &opts)?;
                Ok($name {
                    path: Path::from_id(element.raw_id()),
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

            #[wasm_bindgen(js_name = useStates)]
            pub fn use_states(&self, states: JsValue) -> Result<(), JsValue> {
                api::element_use_states(self.path.raw_id(), states)
            }

            #[wasm_bindgen(js_name = setStateStyle)]
            pub fn set_state_style(&self, state: &str, style: JsValue) -> Result<(), JsValue> {
                let patch = parse_path_style_patch(&style);
                path_set_state_style(self.path.raw_id(), state, patch)
            }

            pub fn animate(&self, path: JsValue, looping: JsValue) -> Animator {
                api::element_animate(self.path.raw_id(), path, looping)
            }

            pub fn on(&self, event: &str, handler: JsValue) -> $name {
                api::element_on(self.path.raw_id(), event, handler);
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            pub fn off(&self, event: JsValue, handler: JsValue) -> $name {
                api::element_off(self.path.raw_id(), event, handler);
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            pub fn trigger(&self, event: &str, packet: JsValue) -> $name {
                api::element_trigger(self.path.raw_id(), event, packet);
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            pub fn hide(&self) {
                let _ = api::element_hide(self.path.raw_id());
            }

            pub fn show(&self) {
                let _ = api::element_show(self.path.raw_id());
            }

            #[wasm_bindgen(getter)]
            pub fn draggable(&self) -> JsValue {
                crate::handler::element_draggable_js(self.path.raw_id())
            }

            #[wasm_bindgen(setter)]
            pub fn set_draggable(&self, value: JsValue) {
                crate::handler::element_set_draggable(self.path.raw_id(), &value);
            }

            pub fn attr(&self, key: JsValue, value: JsValue) -> $name {
                let _ = api::element_attr(self.path.raw_id(), key, value);
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            #[wasm_bindgen(js_name = setShape)]
            pub fn set_shape(&self, shape: JsValue) -> $name {
                let _ = api::element_set_shape(self.path.raw_id(), &shape);
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            #[wasm_bindgen(js_name = setStyle)]
            pub fn set_style(&self, style: JsValue) -> $name {
                let _ = api::element_set_style(self.path.raw_id(), &style);
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            #[wasm_bindgen(js_name = setClipPath)]
            pub fn set_clip_path(&self, clip: JsValue) -> $name {
                let _ = api::element_set_clip_path(self.path.raw_id(), clip);
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            #[wasm_bindgen(js_name = removeClipPath)]
            pub fn remove_clip_path(&self) -> $name {
                let _ = api::element_remove_clip_path(self.path.raw_id());
                $name {
                    path: Path::from_id(self.path.raw_id()),
                }
            }

            #[wasm_bindgen(js_name = getBoundingRect)]
            pub fn get_bounding_rect(&self) -> BoundingRect {
                BoundingRect::from_inner(api::element_get_bounding_rect(self.path.raw_id()))
            }

            #[wasm_bindgen(getter)]
            pub fn position(&self) -> JsValue {
                api::element_position_js(self.path.raw_id())
            }

            #[wasm_bindgen(setter)]
            pub fn set_position(&self, value: JsValue) {
                let _ = api::element_set_position(self.path.raw_id(), &value);
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
impl_shape!(Arc, "arc");
impl_shape!(Ellipse, "ellipse");
impl_shape!(Ring, "ring");
impl_shape!(BezierCurve, "bezier-curve");
impl_shape!(Isogon, "isogon");
impl_shape!(Star, "star");
impl_shape!(Heart, "heart");
impl_shape!(Droplet, "droplet");
impl_shape!(Rose, "rose");
impl_shape!(Trochoid, "trochoid");
impl_shape!(CompoundPath, "compound");
