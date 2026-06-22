//! ECharts WASM 实例（阶段 4 API 骨架）

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_zrender::{STATE_EMPHASIS, STATE_NORMAL, ZRenderer};

use crate::option::{parse_option_value, OptionModel, OptionValue};
use crate::scene::build_placeholder_scene;

/// 对外暴露的 ECharts 实例（对齐 echarts canvas 模式 WASM 侧）
#[wasm_bindgen]
pub struct EChartsInstance {
    zr: ZRenderer,
    option: OptionModel,
    width: u32,
    height: u32,
    dpr: f64,
}

#[wasm_bindgen]
impl EChartsInstance {
    /// 创建实例（对应 init 时的尺寸与 devicePixelRatio）
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, dpr: f64) -> Result<EChartsInstance, JsValue> {
        crate::utils::set_panic_hook();
        let zr = ZRenderer::new_with_dpr(width, height, dpr)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(EChartsInstance {
            zr,
            option: OptionModel::new(),
            width,
            height,
            dpr,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dpr(&self) -> f64 {
        self.dpr
    }

    /// 解析并合并 option（保留 function 字段）
    pub fn set_option(&mut self, option: JsValue) -> Result<(), JsValue> {
        self.option.set_option(&option)?;
        build_placeholder_scene(&mut self.zr, &self.option);
        Ok(())
    }

    /// 是否已接收过 option
    pub fn has_option(&self) -> bool {
        !self.option.is_empty()
    }

    /// option 中是否含 function 字段（供 JS 调试）
    pub fn option_has_functions(&self) -> bool {
        option_contains_function(self.option.root())
    }

    pub fn resize(&mut self, width: u32, height: u32, dpr: f64) -> Result<(), JsValue> {
        self.width = width;
        self.height = height;
        self.dpr = dpr;
        self.zr
            .resize_with_dpr(width, height, dpr)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        if !self.option.is_empty() {
            build_placeholder_scene(&mut self.zr, &self.option);
        }
        Ok(())
    }

    /// 刷新并返回 RGBA 像素缓冲
    pub fn refresh(&mut self) -> Result<Vec<u8>, JsValue> {
        if self.option.is_empty() {
            build_placeholder_scene(&mut self.zr, &self.option);
        }
        self.zr
            .refresh()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// 命中检测，返回 JS 对象或 null
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
                let _ = Reflect::set(
                    &obj,
                    &JsValue::from_str("silent"),
                    &JsValue::from(hit.silent),
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
                if let Some(ref dt) = hit.ec_data.data_type {
                    let _ = Reflect::set(
                        &obj,
                        &JsValue::from_str("dataType"),
                        &JsValue::from_str(dt),
                    );
                }
                obj.into()
            }
            None => JsValue::NULL,
        }
    }

    /// dispatchAction 简化版（highlight / downplay）
    pub fn dispatch_action(&mut self, action: JsValue) -> Result<(), JsValue> {
        let parsed = parse_option_value(&action)?;
        let action_type = parsed
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsValue::from_str("dispatchAction requires type"))?;

        match action_type {
            "highlight" => {
                if let Some(series_index) = parsed
                    .get("seriesIndex")
                    .and_then(|v| v.as_f64())
                    .map(|n| n as i32)
                {
                    self.highlight_series(series_index);
                }
            }
            "downplay" => {
                if let Some(series_index) = parsed
                    .get("seriesIndex")
                    .and_then(|v| v.as_f64())
                    .map(|n| n as i32)
                {
                    self.downplay_series(series_index);
                }
            }
            other => {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "dispatchAction type '{other}' not implemented yet"
                )));
            }
        }
        Ok(())
    }

    pub fn dispose(&mut self) {
        self.option.clear();
    }
}

impl EChartsInstance {
    fn highlight_series(&mut self, series_index: i32) {
        for i in 0..self.zr.storage.paths().len() {
            let ec = self.zr.storage.path(i).ec_data.clone();
            if ec.series_index == Some(series_index) {
                self.zr.set_path_state(i, STATE_EMPHASIS);
            }
        }
    }

    fn downplay_series(&mut self, series_index: i32) {
        for i in 0..self.zr.storage.paths().len() {
            let ec = self.zr.storage.path(i).ec_data.clone();
            if ec.series_index == Some(series_index) {
                self.zr.set_path_state(i, STATE_NORMAL);
            }
        }
    }
}

fn option_contains_function(value: &OptionValue) -> bool {
    match value {
        OptionValue::Function(_) => true,
        OptionValue::Array(arr) => arr.iter().any(option_contains_function),
        OptionValue::Object(map) => map.values().any(option_contains_function),
        _ => false,
    }
}
