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

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}
