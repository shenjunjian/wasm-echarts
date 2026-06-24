//! wasm-zrender：rust-zrender 的 wasm-bindgen 薄封装（对齐 zrender export.ts）

#![allow(dead_code)]

mod bridge;
mod element;
mod registry;
mod utils;
mod zrender;

use wasm_bindgen::prelude::*;

pub use bridge::hit::HoverResult;
pub use element::Element;
pub use zrender::{dispose, dispose_all, get_instance, init, ZRender};

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}
