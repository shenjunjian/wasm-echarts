//! 路径命中检测（isPointInPath），基于 kurbo winding number

mod path;

pub use path::{contain, contain_stroke, contain_stroke_with_transform, contain_with_transform};
