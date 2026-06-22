use crate::core::types::Point;
use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct PolygonShape {
    pub points: Vec<Point>,
}

pub fn build_polygon_path(ctx: &mut PathProxy, shape: &PolygonShape, close: bool) {
    let Some(first) = shape.points.first() else {
        return;
    };
    ctx.move_to(first.0 as f32, first.1 as f32);
    for p in shape.points.iter().skip(1) {
        ctx.line_to(p.0 as f32, p.1 as f32);
    }
    if close {
        ctx.close_path();
    }
}
