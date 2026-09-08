use crate::graphic::path_proxy::PathProxy;
use crate::graphic::shapes::polygon::{PolygonShape, build_polygon_path};

#[derive(Debug, Clone)]
pub struct PolylineShape {
    pub points: Vec<(f64, f64)>,
    pub percent: f64,
    pub smooth: f64,
    pub smooth_constraint: Option<[(f64, f64); 2]>,
}

impl Default for PolylineShape {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            percent: 1.0,
            smooth: 0.0,
            smooth_constraint: None,
        }
    }
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
        &PolygonShape {
            points,
            smooth: shape.smooth,
            smooth_constraint: shape.smooth_constraint,
        },
        false,
    );
}
