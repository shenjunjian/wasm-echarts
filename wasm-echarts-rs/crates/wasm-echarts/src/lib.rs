mod bridge;
mod chart;
mod coord;
mod instance;
mod interaction;
mod model;
mod option;
mod render;
mod scheduler;
mod utils;
mod visual;

use wasm_bindgen::prelude::*;
pub use instance::EChartsInstance;
pub use wasm_zrender::{
    clear_fonts, register_font, Animation, Animator, Arc, BezierCurve, BoundingRect, Circle,
    CompoundPath, Displayable, Droplet, Ellipse, Group, Handler, Heart, HoverResult, Image,
    IncrementalDisplayable, Isogon, Line, LinearGradient, OrientedBoundingRect, Path, Pattern,
    Point, Polygon, Polyline, RadialGradient, Rect, Ring, Rose, Sector, Star, Text, Trochoid,
    TSpan, ZRender,
};

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}
