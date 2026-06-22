//! 阶段 4b：JsCallback 桥接层骨架（CallbackDataParams + callN）

#![allow(dead_code)]

mod callback;
mod params;

#[allow(unused_imports)]
pub use callback::{try_call_formatter, JsCallback};
#[allow(unused_imports)]
pub use params::build_data_params;
