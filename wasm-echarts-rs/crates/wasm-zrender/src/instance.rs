//! ZRenderInstance：rust-zrender 的 wasm-bindgen 薄封装

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use rust_zrender::{STATE_EMPHASIS, STATE_NORMAL, ZRenderer};

use crate::scene::load_scene;

#[wasm_bindgen]
pub struct ZRenderInstance {
    zr: ZRenderer,
    scene: String,
}

#[wasm_bindgen]
impl ZRenderInstance {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, dpr: f64) -> Result<ZRenderInstance, JsValue> {
        crate::utils::set_panic_hook();
        let zr = ZRenderer::new_with_dpr(width, height, dpr)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut inst = ZRenderInstance {
            zr,
            scene: "shapes".into(),
        };
        inst.load_scene("shapes")?;
        Ok(inst)
    }

    pub fn width(&self) -> u32 {
        self.zr.width()
    }

    pub fn height(&self) -> u32 {
        self.zr.height()
    }

    pub fn dpr(&self) -> f64 {
        self.zr.dpr()
    }

    /// 加载内置场景：shapes | text | sector | hit | state
    pub fn load_scene(&mut self, scene: &str) -> Result<(), JsValue> {
        self.scene = scene.to_string();
        load_scene(&mut self.zr, scene);
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32, dpr: f64) -> Result<(), JsValue> {
        self.zr
            .resize_with_dpr(width, height, dpr)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        load_scene(&mut self.zr, &self.scene);
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<Vec<u8>, JsValue> {
        self.zr
            .refresh()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn find_hover(&mut self, x: f64, y: f64) -> JsValue {
        match self.zr.find_hover(x, y) {
            Some(hit) => {
                let obj = Object::new();
                let _ = Reflect::set(&obj, &JsValue::from_str("x"), &JsValue::from(hit.x));
                let _ = Reflect::set(&obj, &JsValue::from_str("y"), &JsValue::from(hit.y));
                let _ = Reflect::set(
                    &obj,
                    &JsValue::from_str("pathIndex"),
                    &JsValue::from(hit.path_index as u32),
                );
                if let Some(si) = hit.ec_data.series_index {
                    let _ = Reflect::set(
                        &obj,
                        &JsValue::from_str("seriesIndex"),
                        &JsValue::from(si),
                    );
                }
                if let Some(di) = hit.ec_data.data_index {
                    let _ = Reflect::set(
                        &obj,
                        &JsValue::from_str("dataIndex"),
                        &JsValue::from(di),
                    );
                }
                obj.into()
            }
            None => JsValue::NULL,
        }
    }

    pub fn highlight_path(&mut self, path_index: u32) {
        self.zr.set_path_state(path_index as usize, STATE_EMPHASIS);
    }

    pub fn downplay_path(&mut self, path_index: u32) {
        self.zr.set_path_state(path_index as usize, STATE_NORMAL);
    }
}
