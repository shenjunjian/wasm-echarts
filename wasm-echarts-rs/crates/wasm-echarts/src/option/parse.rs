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
    enrich_style_object(v, &mut map)?;
    Ok(OptionValue::Object(map))
}

fn reflect_value(v: &JsValue, key: &str) -> Result<JsValue, JsValue> {
    Reflect::get(v, &JsValue::from_str(key))
}

fn enrich_style_object(
    v: &JsValue,
    map: &mut IndexMap<String, OptionValue>,
) -> Result<(), JsValue> {
    let ty = reflect_value(v, "type")
        .ok()
        .and_then(|val| val.as_string());
    let keys: &[&str] = match ty.as_deref() {
        Some("linear") => &[
            "type", "x", "y", "x2", "y2", "colorStops", "global",
        ],
        Some("radial") => &["type", "x", "y", "r", "r0", "colorStops", "global"],
        Some("pattern") => &[
            "type",
            "image",
            "repeat",
            "x",
            "y",
            "scaleX",
            "scaleY",
            "rotation",
        ],
        _ => {
            if map.contains_key("colorStops")
                || reflect_value(v, "colorStops")
                    .ok()
                    .is_some_and(|val| !val.is_undefined() && !val.is_null())
            {
                &[
                    "type", "x", "y", "x2", "y2", "r", "r0", "colorStops", "global",
                ]
            } else {
                return Ok(());
            }
        }
    };
    for key in keys {
        if map.contains_key(*key) {
            continue;
        }
        let val = reflect_value(v, key)?;
        if val.is_undefined() {
            continue;
        }
        map.insert((*key).to_string(), parse_option_value(&val)?);
    }
    Ok(())
}

/// 把 OptionValue 转回普通 JS 值。函数字段保留为原 `Function`。
pub fn option_value_to_js(value: &OptionValue) -> Result<JsValue, JsValue> {
    match value {
        OptionValue::Null => Ok(JsValue::NULL),
        OptionValue::Bool(b) => Ok(JsValue::from(*b)),
        OptionValue::Number(n) => Ok(JsValue::from(*n)),
        OptionValue::String(s) => Ok(JsValue::from_str(s)),
        OptionValue::Function(f) => Ok(JsValue::from(f.clone())),
        OptionValue::Array(arr) => {
            let out = Array::new_with_length(arr.len() as u32);
            for (i, item) in arr.iter().enumerate() {
                out.set(i as u32, option_value_to_js(item)?);
            }
            Ok(out.into())
        }
        OptionValue::Object(map) => {
            let obj = Object::new();
            for (key, val) in map {
                Reflect::set(
                    &obj,
                    &JsValue::from_str(key),
                    &option_value_to_js(val)?,
                )?;
            }
            Ok(obj.into())
        }
    }
}
