//! 未实现图元 stub（构造时抛错，类名与 export.ts 对齐）

use wasm_bindgen::prelude::*;

macro_rules! stub_class {
    ($name:ident) => {
        #[wasm_bindgen]
        pub struct $name;

        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new(_opts: JsValue) -> Result<$name, JsValue> {
                Err(JsValue::from_str(concat!(
                    stringify!($name),
                    " is not implemented in wasm-zrender"
                )))
            }
        }
    };
}

stub_class!(IncrementalDisplayable);

#[wasm_bindgen]
pub struct Point;

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(_x: f64, _y: f64) -> Result<Point, JsValue> {
        Err(JsValue::from_str("Point is not implemented in wasm-zrender"))
    }
}

#[wasm_bindgen]
pub struct BoundingRect;

#[wasm_bindgen]
impl BoundingRect {
    #[wasm_bindgen(constructor)]
    pub fn new(_opts: JsValue) -> Result<BoundingRect, JsValue> {
        Err(JsValue::from_str(
            "BoundingRect is not implemented in wasm-zrender",
        ))
    }
}

#[wasm_bindgen]
pub struct OrientedBoundingRect;

#[wasm_bindgen]
impl OrientedBoundingRect {
    #[wasm_bindgen(constructor)]
    pub fn new(_opts: JsValue) -> Result<OrientedBoundingRect, JsValue> {
        Err(JsValue::from_str(
            "OrientedBoundingRect is not implemented in wasm-zrender",
        ))
    }
}

