use super::{BackendError, CanvasBackend};
use crate::core::types::RgbaBuffer;
use vl_convert_canvas2d::{ArcParams, Canvas2dContext, RectParams};

/// 基于 vl-convert-canvas2d 的 Canvas 2D 后端
pub struct VlConvertBackend {
    ctx: Canvas2dContext,
}

impl VlConvertBackend {
    pub fn new(width: u32, height: u32) -> Result<Self, BackendError> {
        let ctx = Canvas2dContext::new(width, height).map_err(|e| BackendError::Canvas(e.to_string()))?;
        Ok(Self { ctx })
    }
}

impl CanvasBackend for VlConvertBackend {
    fn width(&self) -> u32 {
        self.ctx.width()
    }

    fn height(&self) -> u32 {
        self.ctx.height()
    }

    fn clear(&mut self) {
        self.ctx.reset();
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

    fn set_line_width(&mut self, width: f32) {
        self.ctx.set_line_width(width);
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
        self.ctx.begin_path();
        self.ctx.arc(&ArcParams {
            x: cx as f32,
            y: cy as f32,
            radius: radius as f32,
            start_angle: 0.0,
            end_angle: std::f32::consts::TAU,
            anticlockwise: false,
        });
        self.ctx.fill();
    }

    fn stroke_circle(&mut self, cx: f64, cy: f64, radius: f64) {
        self.ctx.begin_path();
        self.ctx.arc(&ArcParams {
            x: cx as f32,
            y: cy as f32,
            radius: radius as f32,
            start_angle: 0.0,
            end_angle: std::f32::consts::TAU,
            anticlockwise: false,
        });
        self.ctx.stroke();
    }

    fn get_rgba(&self) -> RgbaBuffer {
        self.ctx
            .get_image_data(0, 0, self.ctx.width(), self.ctx.height())
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

        // 非全透明：至少有一个非零 alpha 像素
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }
}
