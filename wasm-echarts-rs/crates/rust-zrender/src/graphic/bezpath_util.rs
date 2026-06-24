//! BezPath ↔ PathProxy 转换

use kurbo::{BezPath, PathEl, Point, Shape as KurboShape};

use crate::graphic::path_proxy::PathProxy;
use vl_convert_canvas2d::{CubicBezierParams, QuadraticBezierParams};

pub fn append_bezpath(ctx: &mut PathProxy, bez: &BezPath) {
    for el in bez.elements() {
        match el {
            PathEl::MoveTo(p) => ctx.move_to(p.x as f32, p.y as f32),
            PathEl::LineTo(p) => ctx.line_to(p.x as f32, p.y as f32),
            PathEl::QuadTo(p1, p2) => ctx.quadratic_curve_to(QuadraticBezierParams {
                cpx: p1.x as f32,
                cpy: p1.y as f32,
                x: p2.x as f32,
                y: p2.y as f32,
            }),
            PathEl::CurveTo(p1, p2, p3) => ctx.cubic_bezier_to(CubicBezierParams {
                cp1x: p1.x as f32,
                cp1y: p1.y as f32,
                cp2x: p2.x as f32,
                cp2y: p2.y as f32,
                x: p3.x as f32,
                y: p3.y as f32,
            }),
            PathEl::ClosePath => ctx.close_path(),
        }
    }
}

pub fn bbox_from_bezpath(bez: &BezPath) -> (f64, f64, f64, f64) {
    let rect = bez.bounding_box();
    (rect.x0, rect.y0, rect.width(), rect.height())
}

pub fn bbox_from_svg(path_data: &str) -> Option<(f64, f64, f64, f64)> {
    BezPath::from_svg(path_data)
        .ok()
        .map(|bez| bbox_from_bezpath(&bez))
}

pub fn point_in_bezpath(bez: &BezPath, x: f64, y: f64) -> bool {
    bez.winding(Point::new(x, y)) != 0
}
