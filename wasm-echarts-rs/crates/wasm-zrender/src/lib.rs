//! wasm-zrender：rust-zrender 的 wasm-bindgen 薄封装（浏览器 WASM 产物）

mod instance;
mod scene;
mod utils;

use wasm_bindgen::prelude::*;

pub use instance::ZRenderInstance;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}
