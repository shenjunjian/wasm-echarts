//! wasm-zrender: Rust 版 zrender 渲染库（canvas 后端）

pub mod canvas;
pub mod contain;
pub mod core;
pub mod element;
pub mod graphic;
pub mod handler;
pub mod storage;
pub mod zrender;

pub use canvas::backend::{BackendError, CanvasBackend, CanvasContext, VlConvertBackend};
pub use canvas::demo::render_demo_shapes;
pub use canvas::draw_image_rgba;
pub use canvas::painter::Painter;
pub use contain::{contain, contain_stroke};
pub use element::{EcData, ElementStates, PathStylePatch, STATE_EMPHASIS, STATE_NORMAL, STATE_SELECT};
pub use handler::{Handler, HitResult};
pub use graphic::*;
pub use storage::Storage;
pub use zrender::ZRenderer;
