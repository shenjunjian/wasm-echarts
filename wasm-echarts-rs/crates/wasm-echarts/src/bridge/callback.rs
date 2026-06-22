//! JS 函数回调封装（visual / tooltip / renderItem 阶段调用）

use js_sys::Function;
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct JsCallback(Function);

impl JsCallback {
    pub fn new(f: Function) -> Self {
        Self(f)
    }

    pub fn inner(&self) -> &Function {
        &self.0
    }

    /// label / tooltip formatter: `(params) => string`
    pub fn call_formatter(&self, params: &JsValue) -> Result<String, JsValue> {
        let ret = self.0.call1(&JsValue::NULL, params)?;
        ret.as_string()
            .ok_or_else(|| JsValue::from_str("formatter must return string"))
    }

    /// itemStyle.color 等: `(params) => color`
    pub fn call_color(&self, params: &JsValue) -> Result<JsValue, JsValue> {
        self.0.call1(&JsValue::NULL, params)
    }

    /// renderItem: `(params, api) => graphicSpec`
    pub fn call_render_item(&self, params: &JsValue, api: &JsValue) -> Result<JsValue, JsValue> {
        self.0.call2(&JsValue::NULL, params, api)
    }

    /// axisLabel.formatter: `(value, index) => string`
    pub fn call_axis_formatter(&self, value: &JsValue, index: u32) -> Result<String, JsValue> {
        let ret = self
            .0
            .call2(&JsValue::NULL, value, &JsValue::from(index))?;
        ret.as_string()
            .ok_or_else(|| JsValue::from_str("axis formatter must return string"))
    }
}

/// 安全调用：捕获 JS throw，console.error 后返回 None
pub fn try_call_formatter(callback: &JsCallback, params: &JsValue) -> Option<String> {
    match callback.call_formatter(params) {
        Ok(s) => Some(s),
        Err(err) => {
            web_sys::console::error_2(&JsValue::from_str("formatter callback error:"), &err);
            None
        }
    }
}
