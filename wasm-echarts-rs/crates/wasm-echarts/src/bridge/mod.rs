//! 阶段 4b：JsCallback 桥接层骨架（CallbackDataParams + callN）

#![allow(dead_code)]

mod callback;
mod params;
mod resolve;

pub use callback::{try_call_formatter, JsCallback};
pub use params::build_data_params;
pub use resolve::{default_series_color, resolve_axis_formatter, resolve_color, resolve_formatter};
