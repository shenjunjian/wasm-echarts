//! 构造 CallbackDataParams（对齐 echarts util/types.ts）

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

/// 按需构造 hover / visual 阶段 params 对象
pub fn build_data_params(
    series_index: u32,
    data_index: u32,
    series_name: &str,
    name: &str,
    value: &JsValue,
    color: Option<&str>,
) -> JsValue {
    let obj = Object::new();
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("seriesIndex"),
        &JsValue::from(series_index),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("dataIndex"),
        &JsValue::from(data_index),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("seriesName"),
        &JsValue::from_str(series_name),
    );
    let _ = Reflect::set(&obj, &JsValue::from_str("name"), &JsValue::from_str(name));
    let _ = Reflect::set(&obj, &JsValue::from_str("value"), value);
    if let Some(c) = color {
        let _ = Reflect::set(&obj, &JsValue::from_str("color"), &JsValue::from_str(c));
    }
    obj.into()
}
