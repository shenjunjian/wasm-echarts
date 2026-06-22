mod utils;

use wasm_bindgen::prelude::*;
use wasm_zrender::{render_demo_shapes, CanvasBackend, VlConvertBackend};

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}

/// 阶段 0 demo 渲染器：离屏 canvas 绘制 Rect/Circle，导出 RGBA
#[wasm_bindgen]
pub struct DemoRenderer {
    backend: VlConvertBackend,
}

#[wasm_bindgen]
impl DemoRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<DemoRenderer, JsValue> {
        utils::set_panic_hook();
        let backend = VlConvertBackend::new(width, height)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(DemoRenderer { backend })
    }

    pub fn width(&self) -> u32 {
        self.backend.width()
    }

    pub fn height(&self) -> u32 {
        self.backend.height()
    }

    /// 绘制 demo 图形并返回 RGBA 像素缓冲
    pub fn render(&mut self) -> Result<Vec<u8>, JsValue> {
        render_demo_shapes(&mut self.backend)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(self.backend.get_rgba())
    }
}
