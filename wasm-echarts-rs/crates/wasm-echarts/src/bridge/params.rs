//! 构造 CallbackDataParams（对齐 echarts util/types.ts）

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;

pub struct DataParamsInput<'a> {
    pub series_index: u32,
    pub data_index: u32,
    pub series_name: &'a str,
    pub series_type: &'a str,
    pub name: &'a str,
    pub value: &'a JsValue,
    pub data: &'a JsValue,
    pub color: Option<&'a str>,
    pub percent: Option<f64>,
}

/// 按需构造 hover / visual 阶段 params 对象
pub fn build_data_params(input: DataParamsInput<'_>) -> JsValue {
    let obj = Object::new();
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("componentType"),
        &JsValue::from_str("series"),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("componentSubType"),
        &JsValue::from_str(input.series_type),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("componentIndex"),
        &JsValue::from(input.series_index),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("seriesType"),
        &JsValue::from_str(input.series_type),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("seriesIndex"),
        &JsValue::from(input.series_index),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("dataIndex"),
        &JsValue::from(input.data_index),
    );
    let _ = Reflect::set(
        &obj,
        &JsValue::from_str("seriesName"),
        &JsValue::from_str(input.series_name),
    );
    let _ = Reflect::set(&obj, &JsValue::from_str("name"), &JsValue::from_str(input.name));
    let _ = Reflect::set(&obj, &JsValue::from_str("value"), input.value);
    let _ = Reflect::set(&obj, &JsValue::from_str("data"), input.data);
    if let Some(c) = input.color {
        let _ = Reflect::set(&obj, &JsValue::from_str("color"), &JsValue::from_str(c));
    }
    if let Some(percent) = input.percent {
        let _ = Reflect::set(&obj, &JsValue::from_str("percent"), &JsValue::from(percent));
    }
    let vars = Array::new_with_length(3);
    vars.set(0, JsValue::from_str("seriesName"));
    vars.set(1, JsValue::from_str("name"));
    vars.set(2, JsValue::from_str("value"));
    let _ = Reflect::set(&obj, &JsValue::from_str("$vars"), &vars);
    obj.into()
}
