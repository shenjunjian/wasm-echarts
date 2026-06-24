//! Displayable 基类（抽象，对齐 zrender export.ts）

use wasm_bindgen::prelude::*;

/// Path / Text / Image / Group 等图元的公共 displayable 属性由构造 opts 传入。
/// 本类不可直接实例化，仅用于 API 对齐与文档说明。
#[wasm_bindgen]
pub struct Displayable;

#[wasm_bindgen]
impl Displayable {
    #[wasm_bindgen(constructor)]
    pub fn new(_opts: JsValue) -> Result<Displayable, JsValue> {
        Err(JsValue::from_str(
            "Displayable is abstract; use Rect, Circle, Text, Image, Group, etc.",
        ))
    }
}
