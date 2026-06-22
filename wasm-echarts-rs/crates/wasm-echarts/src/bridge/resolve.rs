//! 从 option 字段解析 color / formatter（含 JsFunction 回调）

use js_sys::Function;
use wasm_bindgen::JsValue;

use super::callback::{try_call_formatter, JsCallback};
use crate::option::OptionValue;

/// ECharts 默认色板
pub const DEFAULT_COLORS: &[&str] = &[
    "#5470c6", "#91cc75", "#fac858", "#ee6666", "#73c0de", "#3ba272", "#fc8452", "#9a60b4",
    "#ea7ccc",
];

/// 解析 itemStyle.color / lineStyle.color 等（常量或回调）
pub fn resolve_color(value: Option<&OptionValue>, params: &JsValue, fallback: &str) -> String {
    match value {
        Some(OptionValue::String(s)) => s.clone(),
        Some(OptionValue::Function(f)) => {
            let cb = JsCallback::new(f.clone());
            match cb.call_color(params) {
                Ok(ret) => ret.as_string().unwrap_or_else(|| fallback.to_string()),
                Err(err) => {
                    web_sys::console::error_2(&JsValue::from_str("color callback error:"), &err);
                    fallback.to_string()
                }
            }
        }
        _ => fallback.to_string(),
    }
}

/// 解析 formatter 字段（label / tooltip）
pub fn resolve_formatter(value: Option<&OptionValue>, params: &JsValue) -> Option<String> {
    match value {
        Some(OptionValue::String(s)) => Some(s.clone()),
        Some(OptionValue::Function(f)) => {
            try_call_formatter(&JsCallback::new(f.clone()), params)
        }
        _ => None,
    }
}

/// 解析 axisLabel.formatter: (value, index) => string
pub fn resolve_axis_formatter(
    value: Option<&OptionValue>,
    axis_value: &JsValue,
    index: u32,
    fallback: &str,
) -> String {
    match value {
        Some(OptionValue::String(s)) => s.clone(),
        Some(OptionValue::Function(f)) => {
            let cb = JsCallback::new(f.clone());
            match cb.call_axis_formatter(axis_value, index) {
                Ok(s) => s,
                Err(err) => {
                    web_sys::console::error_2(
                        &JsValue::from_str("axis formatter callback error:"),
                        &err,
                    );
                    fallback.to_string()
                }
            }
        }
        _ => fallback.to_string(),
    }
}

pub fn default_series_color(series_index: usize) -> &'static str {
    DEFAULT_COLORS[series_index % DEFAULT_COLORS.len()]
}

pub fn as_js_callback(value: &OptionValue) -> Option<JsCallback> {
    match value {
        OptionValue::Function(f) => Some(JsCallback::new(f.clone())),
        _ => None,
    }
}

pub fn extract_function(value: &OptionValue) -> Option<Function> {
    match value {
        OptionValue::Function(f) => Some(f.clone()),
        _ => None,
    }
}
