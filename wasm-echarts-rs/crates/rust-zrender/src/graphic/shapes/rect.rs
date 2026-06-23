use vl_convert_canvas2d::RectParams;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct RectShape {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn build_rect_path(ctx: &mut PathProxy, shape: &RectShape) {
    ctx.rect(&RectParams {
        x: shape.x as f32,
        y: shape.y as f32,
        width: shape.width as f32,
        height: shape.height as f32,
    });
}
