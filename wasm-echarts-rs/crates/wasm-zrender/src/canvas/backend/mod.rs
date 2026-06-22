//! Canvas 2D 后端抽象，隔离 vl-convert API 与 zrender Painter

mod shadow;
mod vl_convert;

pub use shadow::ShadowPass;
pub use vl_convert::VlConvertBackend;

use std::sync::Arc;

use vl_convert_canvas2d::{
    ArcParams, CanvasGradient, CanvasPattern, CubicBezierParams, QuadraticBezierParams, RectParams,
};

use crate::core::matrix::Matrix;
use crate::core::types::RgbaBuffer;
use crate::graphic::path_proxy::PathProxy;
use crate::graphic::style::ShadowStyle;

/// 后端错误
#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("canvas backend error: {0}")]
    Canvas(String),
}

/// brush / PathProxy 使用的 Canvas 2D 上下文接口
pub trait CanvasContext {
    fn save(&mut self);
    fn restore(&mut self);

    fn set_transform(&mut self, matrix: &Matrix);
    fn reset_transform(&mut self);

    fn set_global_alpha(&mut self, alpha: f32);

    fn set_fill_style(&mut self, color: &str) -> Result<(), BackendError>;
    fn set_stroke_style(&mut self, color: &str) -> Result<(), BackendError>;
    fn set_fill_style_gradient(&mut self, gradient: CanvasGradient);
    fn set_stroke_style_gradient(&mut self, gradient: CanvasGradient);
    fn set_fill_style_pattern(&mut self, pattern: Arc<CanvasPattern>);
    fn set_stroke_style_pattern(&mut self, pattern: Arc<CanvasPattern>);

    fn create_pattern(
        &self,
        data: &[u8],
        width: u32,
        height: u32,
        repetition: &str,
    ) -> Result<Arc<CanvasPattern>, BackendError>;

    fn set_line_width(&mut self, width: f32);
    fn set_line_dash(&mut self, segments: Vec<f32>);
    fn set_line_dash_offset(&mut self, offset: f32);
    fn set_line_cap(&mut self, cap: vl_convert_canvas2d::LineCap);
    fn set_line_join(&mut self, join: vl_convert_canvas2d::LineJoin);

    fn begin_path(&mut self);
    fn move_to(&mut self, x: f32, y: f32);
    fn line_to(&mut self, x: f32, y: f32);
    fn cubic_bezier_to(&mut self, params: &CubicBezierParams);
    fn quadratic_curve_to(&mut self, params: &QuadraticBezierParams);
    fn arc(&mut self, params: &ArcParams);
    fn rect(&mut self, params: &RectParams);
    fn close_path(&mut self);

    fn fill(&mut self);
    fn stroke(&mut self);
    fn clip(&mut self);

    /// 绘制 RGBA 图像（非预乘 alpha）
    fn draw_image_rgba(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        dx: f32,
        dy: f32,
        dw: f32,
        dh: f32,
    ) -> Result<(), BackendError>;

    /// 阴影 pass：在 vl-convert 不支持 shadow 时由后端补齐
    fn draw_shadow(
        &mut self,
        path: &PathProxy,
        transform: &Matrix,
        shadow: &ShadowStyle,
        fill: bool,
        stroke: bool,
        line_width: f32,
    ) -> Result<(), BackendError> {
        let _ = (path, transform, shadow, fill, stroke, line_width);
        Ok(())
    }
}

/// Painter 使用的离屏 Canvas 后端
pub trait CanvasBackend: CanvasContext {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self);
    fn get_rgba(&self) -> RgbaBuffer;

    // 便捷方法（阶段 0 demo 保留）
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64);
    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64);
    fn fill_circle(&mut self, cx: f64, cy: f64, radius: f64);
    fn stroke_circle(&mut self, cx: f64, cy: f64, radius: f64);
}
