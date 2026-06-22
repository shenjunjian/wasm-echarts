//! Canvas 2D 后端抽象，隔离 vl-convert API 与 zrender Painter

mod vl_convert;

pub use vl_convert::VlConvertBackend;

use crate::core::types::RgbaBuffer;

/// 后端错误
#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("canvas backend error: {0}")]
    Canvas(String),
}

/// Painter 使用的 Canvas 2D 后端 trait。
///
/// 后续 Painter / brush 只依赖此接口，便于替换或 mock 测试。
pub trait CanvasBackend {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn clear(&mut self);

    fn set_fill_style(&mut self, color: &str) -> Result<(), BackendError>;
    fn set_stroke_style(&mut self, color: &str) -> Result<(), BackendError>;
    fn set_line_width(&mut self, width: f32);

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64);
    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    fn fill_circle(&mut self, cx: f64, cy: f64, radius: f64);
    fn stroke_circle(&mut self, cx: f64, cy: f64, radius: f64);

    /// 导出 RGBA 像素（非预乘 alpha，4 字节/像素）
    fn get_rgba(&self) -> RgbaBuffer;
}
