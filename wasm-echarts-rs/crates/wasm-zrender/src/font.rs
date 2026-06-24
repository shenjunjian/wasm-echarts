//! 字体注册 API（JS / 宿主传入 font bytes）

use rust_zrender::{register_font as register_font_data, with_resolved_font_config, RegisterFontOptions};
use wasm_bindgen::prelude::*;

use crate::bridge::opts::{get_string, get_string_array};
use crate::registry::refresh_all_font_databases;

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
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    with_resolved_font_config(|resolved| {
        refresh_all_font_databases(resolved);
    })
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}

/// 清空已注册字体（主要用于测试）。
#[wasm_bindgen(js_name = clearFonts)]
pub fn clear_fonts() -> Result<(), JsValue> {
    rust_zrender::clear_fonts().map_err(|e| JsValue::from_str(&e.to_string()))?;
    with_resolved_font_config(|resolved| {
        refresh_all_font_databases(resolved);
    })
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}
