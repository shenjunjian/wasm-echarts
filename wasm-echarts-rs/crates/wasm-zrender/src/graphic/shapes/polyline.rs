use crate::graphic::path_proxy::PathProxy;
use crate::graphic::shapes::polygon::{PolygonShape, build_polygon_path};

#[derive(Debug, Clone, Default)]
pub struct PolylineShape {
    pub points: Vec<(f64, f64)>,
    pub percent: f64,
}

pub fn build_polyline_path(ctx: &mut PathProxy, shape: &PolylineShape) {
    if shape.points.is_empty() {
        return;
    }
    let count = if shape.percent >= 1.0 {
        shape.points.len()
    } else {
        ((shape.points.len() as f64) * shape.percent).max(1.0) as usize
    };
    let points: Vec<(f64, f64)> = shape.points[..count.min(shape.points.len())].to_vec();
    build_polygon_path(
        ctx,
        &PolygonShape { points },
        false,
    );
}
