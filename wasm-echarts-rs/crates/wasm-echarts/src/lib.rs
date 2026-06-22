mod bridge;
mod instance;
mod option;
mod scene;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_zrender::ZRenderer;

pub use instance::EChartsInstance;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}

/// 阶段 1 渲染器：Storage → Painter → RGBA（保留供底层调试）
#[wasm_bindgen]
pub struct DemoRenderer {
    zr: ZRenderer,
}

#[wasm_bindgen]
impl DemoRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<DemoRenderer, JsValue> {
        utils::set_panic_hook();
        let zr = ZRenderer::new(width, height).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(DemoRenderer { zr })
    }

    pub fn width(&self) -> u32 {
        self.zr.width()
    }

    pub fn height(&self) -> u32 {
        self.zr.height()
    }

    pub fn render(&mut self) -> Result<Vec<u8>, JsValue> {
        scene::build_placeholder_scene(&mut self.zr, &option::OptionModel::new());
        self.zr
            .refresh()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
