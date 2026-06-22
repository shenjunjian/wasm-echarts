//! 从 JsValue 递归解析 option（function 保留为 js_sys::Function）

use super::OptionValue;
use indexmap::IndexMap;
use js_sys::{Array, Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};

pub fn parse_option_value(v: &JsValue) -> Result<OptionValue, JsValue> {
    if v.is_null() || v.is_undefined() {
        return Ok(OptionValue::Null);
    }
    if let Some(b) = v.as_bool() {
        return Ok(OptionValue::Bool(b));
    }
    if let Some(n) = v.as_f64() {
        return Ok(OptionValue::Number(n));
    }
    if let Some(s) = v.as_string() {
        return Ok(OptionValue::String(s));
    }
    if v.is_instance_of::<Function>() {
        return Ok(OptionValue::Function(v.clone().unchecked_into()));
    }
    if Array::is_array(v) {
        let arr = Array::from(v);
        let mut out = Vec::with_capacity(arr.length() as usize);
        for i in 0..arr.length() {
            let item = arr.get(i);
            out.push(parse_option_value(&item)?);
        }
        return Ok(OptionValue::Array(out));
    }
    if v.is_instance_of::<Object>() {
        return Ok(parse_object(v)?);
    }
    Ok(OptionValue::Null)
}

fn parse_object(v: &JsValue) -> Result<OptionValue, JsValue> {
    let obj: Object = v.clone().unchecked_into();
    let keys = Object::keys(&obj);
    let len = keys.length();
    let mut map = IndexMap::with_capacity(len as usize);
    for i in 0..len {
        let key = keys
            .get(i)
            .as_string()
            .ok_or_else(|| JsValue::from_str("option object key is not a string"))?;
        let val = Reflect::get(&obj, &JsValue::from_str(&key)).map_err(|_| {
            JsValue::from_str(&format!("failed to read option key: {key}"))
        })?;
        map.insert(key, parse_option_value(&val)?);
    }
    Ok(OptionValue::Object(map))
}
