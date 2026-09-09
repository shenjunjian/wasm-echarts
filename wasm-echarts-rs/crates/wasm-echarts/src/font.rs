//! 字体注册 API（JS / 宿主传入 font bytes）。
//! wasm-echarts 与 wasm-zrender 各自一份 WASM，fontdb 不共享。

use js_sys::{Array, Reflect};
use rust_zrender::{register_font as register_font_data, RegisterFontOptions};
use wasm_bindgen::prelude::*;

fn get_string(obj: &JsValue, key: &str) -> Option<String> {
    Reflect::get(obj, &JsValue::from_str(key))
        .ok()
        .and_then(|v| v.as_string())
}

fn get_string_array(obj: &JsValue, key: &str) -> Option<Vec<String>> {
    let value = Reflect::get(obj, &JsValue::from_str(key)).ok()?;
    if !value.is_instance_of::<Array>() {
        return None;
    }
    let array = Array::from(&value);
    let mut out = Vec::with_capacity(array.length() as usize);
    for item in array.iter() {
        if let Some(s) = item.as_string() {
            out.push(s);
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

/// 向全局 fontdb 注册字体文件。
///
/// `opts` 可选字段：
/// - `familyName`: 覆盖字体族名
/// - `sansSerif`: `string[]`，将 CSS `sans-serif` 映射到这些族名
#[wasm_bindgen(js_name = registerFont)]
pub fn register_font(data: &[u8], opts: JsValue) -> Result<(), JsValue> {
    let (family_name, sans_serif) = if opts.is_object() {
        (
            get_string(&opts, "familyName"),
            get_string_array(&opts, "sansSerif"),
        )
    } else {
        (None, None)
    };

    register_font_data(
        data.to_vec(),
        RegisterFontOptions {
            family_name,
            sans_serif,
        },
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// 清空已注册字体（主要用于测试）。
#[wasm_bindgen(js_name = clearFonts)]
pub fn clear_fonts() -> Result<(), JsValue> {
    rust_zrender::clear_fonts().map_err(|e| JsValue::from_str(&e.to_string()))
}
