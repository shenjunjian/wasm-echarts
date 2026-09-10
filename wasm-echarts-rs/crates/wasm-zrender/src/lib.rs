//! wasm-zrender：rust-zrender 的 wasm-bindgen 薄封装（对齐 zrender export.ts）

mod animation;
mod bridge;
mod element;
mod export;
mod font;
mod graphic;
mod handler;
mod registry;
mod utils;
mod zrender;

use wasm_bindgen::prelude::*;

pub use bridge::hit::HoverResult;
pub use element::Element;
pub use export::{
    color, matrix, morph, parse_svg, path, set_platform_api, show_debug_dirty_rect, util, vector,
};
pub use animation::{Animation, Animator};
pub use graphic::{
    Arc, BezierCurve, BoundingRect, Circle, CompoundPath, Displayable, Droplet, Ellipse, Group,
    Heart, Image, IncrementalDisplayable, Isogon, Line, LinearGradient, OrientedBoundingRect, Path,
    Pattern, Point, Polygon, Polyline, RadialGradient, Rect, Ring, Rose, Sector, Star, Text,
    Trochoid, TSpan,
};
pub use handler::Handler;
pub use registry::{
    dispose_renderer, insert_renderer, rematerialize_mounted_roots, with_zr,
};
pub use zrender::{dispose, dispose_all, get_instance, init, ZRender};
pub use font::{clear_fonts, register_font};

/// 把 canvas 绑到已有 ZRender（echarts `init(canvas)` 后 `getZr().on` 需要）。
pub fn attach_host(zr_id: u32, dom: &JsValue) -> Result<(), JsValue> {
    crate::handler::attach(zr_id, dom)
}

#[cfg(feature = "standalone")]
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    utils::set_panic_hook();
}
