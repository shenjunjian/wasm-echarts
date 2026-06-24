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
