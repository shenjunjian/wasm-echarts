//! 从 option 字段解析 color / formatter（含 JsFunction 回调）

use js_sys::{Function, Reflect};
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

fn param_str(params: &JsValue, key: &str) -> String {
    Reflect::get(params, &JsValue::from_str(key))
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default()
}

fn param_display(params: &JsValue, key: &str) -> String {
    let v = Reflect::get(params, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED);
    if let Some(s) = v.as_string() {
        return s;
    }
    if let Some(n) = v.as_f64() {
        return crate::utils::format_axis_number(n);
    }
    if js_sys::Array::is_array(&v) {
        let arr = js_sys::Array::from(&v);
        let mut parts = Vec::new();
        for i in 0..arr.length() {
            let item = arr.get(i);
            if let Some(n) = item.as_f64() {
                parts.push(crate::utils::format_axis_number(n));
            } else if let Some(s) = item.as_string() {
                parts.push(s);
            }
        }
        return parts.join(", ");
    }
    String::new()
}

/// `{a}` seriesName、`{b}` name、`{c}` value、`{d}` percent
pub fn format_tpl(template: &str, params: &JsValue) -> String {
    if !template.contains('{') {
        return template.to_string();
    }
    let a = param_str(params, "seriesName");
    let b = param_str(params, "name");
    let c = param_display(params, "value");
    let d = param_display(params, "percent");
    template
        .replace("{a}", &a)
        .replace("{b}", &b)
        .replace("{c}", &c)
        .replace("{d}", &d)
        .replace("{seriesName}", &a)
        .replace("{name}", &b)
        .replace("{value}", &c)
}

/// 解析 formatter 字段（label / tooltip）
pub fn resolve_formatter(value: Option<&OptionValue>, params: &JsValue) -> Option<String> {
    match value {
        Some(OptionValue::String(s)) => Some(format_tpl(s, params)),
        Some(OptionValue::Function(f)) => {
            try_call_formatter(&JsCallback::new(f.clone()), params)
        }
        _ => None,
    }
}

/// 解析 axisLabel.formatter: `(value, index) => string` 或 `'{value} kg'`
pub fn resolve_axis_formatter(
    value: Option<&OptionValue>,
    axis_value: &JsValue,
    index: u32,
    fallback: &str,
) -> String {
    match value {
        Some(OptionValue::String(s)) => s.replace("{value}", fallback),
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

pub fn resolve_symbol_size(
    value: Option<&OptionValue>,
    raw_value: &JsValue,
    params: &JsValue,
    fallback: f64,
) -> f64 {
    match value {
        Some(OptionValue::Number(n)) if n.is_finite() => *n,
        Some(OptionValue::Array(arr)) => arr
            .first()
            .and_then(|v| v.as_f64())
            .filter(|n| n.is_finite())
            .unwrap_or(fallback),
        Some(OptionValue::Function(f)) => match JsCallback::new(f.clone()).call_size(raw_value, params) {
            Ok(n) if n.is_finite() && n >= 0.0 => n,
            Err(err) => {
                web_sys::console::error_2(&JsValue::from_str("symbolSize callback error:"), &err);
                fallback
            }
            _ => fallback,
        },
        _ => fallback,
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
