use super::{BackendError, CanvasBackend, CanvasContext, ShadowPass};
use crate::core::matrix::{self, Matrix};
use crate::core::types::RgbaBuffer;
use crate::graphic::path_proxy::PathProxy;
use crate::graphic::style::ShadowStyle;
use std::sync::Arc;
use vl_convert_canvas2d::{
    ArcParams, Canvas2dContext, CanvasGradient, CanvasImageDataRef, CanvasPattern,
    CubicBezierParams, QuadraticBezierParams, RectParams,
};

/// 基于 vl-convert-canvas2d 的 Canvas 2D 后端
pub struct VlConvertBackend {
    ctx: Canvas2dContext,
}

impl VlConvertBackend {
    pub fn new(width: u32, height: u32) -> Result<Self, BackendError> {
        let ctx =
            Canvas2dContext::new(width, height).map_err(|e| BackendError::Canvas(e.to_string()))?;
        Ok(Self { ctx })
    }
}

impl CanvasContext for VlConvertBackend {
    fn save(&mut self) {
        self.ctx.save();
    }

    fn restore(&mut self) {
        self.ctx.restore();
    }

    fn set_transform(&mut self, m: &Matrix) {
        self.ctx.set_transform(matrix::to_dom_matrix(m));
    }

    fn reset_transform(&mut self) {
        self.ctx.reset_transform();
    }

    fn set_global_alpha(&mut self, alpha: f32) {
        self.ctx.set_global_alpha(alpha);
    }

    fn set_fill_style(&mut self, color: &str) -> Result<(), BackendError> {
        self.ctx
            .set_fill_style(color)
            .map_err(|e| BackendError::Canvas(e.to_string()))
    }

    fn set_stroke_style(&mut self, color: &str) -> Result<(), BackendError> {
        self.ctx
            .set_stroke_style(color)
            .map_err(|e| BackendError::Canvas(e.to_string()))
    }

    fn set_fill_style_gradient(&mut self, gradient: CanvasGradient) {
        self.ctx.set_fill_style_gradient(gradient);
    }

    fn set_stroke_style_gradient(&mut self, gradient: CanvasGradient) {
        self.ctx.set_stroke_style_gradient(gradient);
    }

    fn set_fill_style_pattern(&mut self, pattern: Arc<CanvasPattern>) {
        self.ctx.set_fill_style_pattern(pattern);
    }

    fn set_stroke_style_pattern(&mut self, pattern: Arc<CanvasPattern>) {
        self.ctx.set_stroke_style_pattern(pattern);
    }

    fn create_pattern(
        &self,
        data: &[u8],
        width: u32,
        height: u32,
        repetition: &str,
    ) -> Result<Arc<CanvasPattern>, BackendError> {
        self.ctx
            .create_pattern(data, width, height, repetition)
            .map_err(|e| BackendError::Canvas(e.to_string()))
    }

    fn set_line_width(&mut self, width: f32) {
        self.ctx.set_line_width(width);
    }

    fn set_line_dash(&mut self, segments: Vec<f32>) {
        self.ctx.set_line_dash(segments);
    }

    fn set_line_dash_offset(&mut self, offset: f32) {
        self.ctx.set_line_dash_offset(offset);
    }

    fn set_line_cap(&mut self, cap: vl_convert_canvas2d::LineCap) {
        self.ctx.set_line_cap(cap);
    }

    fn set_line_join(&mut self, join: vl_convert_canvas2d::LineJoin) {
        self.ctx.set_line_join(join);
    }

    fn begin_path(&mut self) {
        self.ctx.begin_path();
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.ctx.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.ctx.line_to(x, y);
    }

    fn cubic_bezier_to(&mut self, params: &CubicBezierParams) {
        self.ctx.bezier_curve_to(params);
    }

    fn quadratic_curve_to(&mut self, params: &QuadraticBezierParams) {
        self.ctx.quadratic_curve_to(params);
    }

    fn arc(&mut self, params: &ArcParams) {
        self.ctx.arc(params);
    }

    fn rect(&mut self, params: &RectParams) {
        self.ctx.rect(params);
    }

    fn close_path(&mut self) {
        self.ctx.close_path();
    }

    fn fill(&mut self) {
        self.ctx.fill();
    }

    fn stroke(&mut self) {
        self.ctx.stroke();
    }

    fn clip(&mut self) {
        self.ctx.clip();
    }

    fn draw_image_rgba(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        dx: f32,
        dy: f32,
        dw: f32,
        dh: f32,
    ) -> Result<(), BackendError> {
        let image = CanvasImageDataRef {
            data,
            width,
            height,
        };
        self.ctx
            .draw_image_data_scaled(&image, dx, dy, dw, dh);
        Ok(())
    }

    fn draw_shadow(
        &mut self,
        path: &PathProxy,
        transform: &Matrix,
        shadow: &ShadowStyle,
        fill: bool,
        stroke: bool,
        line_width: f32,
    ) -> Result<(), BackendError> {
        let rgba = ShadowPass::render(
            self.ctx.width(),
            self.ctx.height(),
            path,
            transform,
            shadow,
            fill,
            stroke,
            line_width,
        )?;
        self.draw_image_rgba(
            &rgba,
            self.ctx.width(),
            self.ctx.height(),
            0.0,
            0.0,
            self.ctx.width() as f32,
            self.ctx.height() as f32,
        )
    }
}

impl CanvasBackend for VlConvertBackend {
    fn create(width: u32, height: u32) -> Result<Self, BackendError> {
        Self::new(width, height)
    }

    fn width(&self) -> u32 {
        self.ctx.width()
    }

    fn height(&self) -> u32 {
        self.ctx.height()
    }

    fn clear(&mut self) {
        self.ctx.reset();
    }

    fn get_rgba(&self) -> RgbaBuffer {
        self.ctx
            .get_image_data(0, 0, self.ctx.width(), self.ctx.height())
    }

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.fill_rect(&RectParams {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        });
    }

    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.stroke_rect(&RectParams {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        });
    }

    fn fill_circle(&mut self, cx: f64, cy: f64, radius: f64) {
        self.begin_path();
        self.arc(&ArcParams {
            x: cx as f32,
            y: cy as f32,
            radius: radius as f32,
            start_angle: 0.0,
            end_angle: std::f32::consts::TAU,
            anticlockwise: false,
        });
        self.fill();
    }

    fn stroke_circle(&mut self, cx: f64, cy: f64, radius: f64) {
        self.begin_path();
        self.arc(&ArcParams {
            x: cx as f32,
            y: cy as f32,
            radius: radius as f32,
            start_angle: 0.0,
            end_angle: std::f32::consts::TAU,
            anticlockwise: false,
        });
        self.stroke();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_rect_and_circle() {
        let mut backend = VlConvertBackend::new(200, 150).unwrap();
        crate::canvas::demo::render_demo_shapes(&mut backend).unwrap();

        let rgba = backend.get_rgba();
        assert_eq!(rgba.len(), 200 * 150 * 4);
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }
}
