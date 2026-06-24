//! wasm-zrender：rust-zrender 的 wasm-bindgen 薄封装（对齐 zrender export.ts）

mod bridge;
mod element;
mod export;
mod graphic;
mod registry;
mod utils;
mod zrender;

use wasm_bindgen::prelude::*;

pub use bridge::hit::HoverResult;
pub use element::Element;
pub use export::{
    color, matrix, morph, parse_svg, path, set_platform_api, show_debug_dirty_rect, util, vector,
};
pub use graphic::{
    Arc, BezierCurve, BoundingRect, Circle, CompoundPath, Displayable, Droplet, Ellipse, Group,
    Heart, Image, IncrementalDisplayable, Isogon, Line, LinearGradient, OrientedBoundingRect,
    Path, Pattern, Point, Polygon, Polyline, RadialGradient, Rect, Ring, Rose, Sector, Star, Text,
    Trochoid, TSpan,
};
pub use zrender::{dispose, dispose_all, get_instance, init, ZRender};

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}
