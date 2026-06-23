//! Path 填充/描边命中检测

use kurbo::{BezPath, Point, Shape, Stroke, StrokeOpts};
use vl_convert_canvas2d::{ArcParams, RectParams};

use crate::core::matrix::Matrix;
use crate::graphic::path_proxy::{PathCmd, PathProxy};

/// 填充命中（等价 Canvas isPointInPath）
pub fn contain(path: &PathProxy, x: f64, y: f64) -> bool {
    let bez = to_bezpath(path);
    bez.winding(Point::new(x, y)) != 0
}

/// 填充命中（带变换矩阵，点在全局/视口坐标系）
pub fn contain_with_transform(path: &PathProxy, transform: &Matrix, x: f64, y: f64) -> bool {
    if let Some(local) = invert_transform_point(transform, x, y) {
        contain(path, local.x, local.y)
    } else {
        false
    }
}

/// 描边命中
pub fn contain_stroke(path: &PathProxy, line_width: f32, x: f64, y: f64) -> bool {
    if line_width <= 0.0 {
        return false;
    }
    let bez = to_bezpath(path);
    let pt = Point::new(x, y);
    let stroke = Stroke::new(line_width as f64);
    let stroked = kurbo::stroke(&bez, &stroke, &StrokeOpts::default(), 0.25);
    stroked.winding(pt) != 0
}

/// 描边命中（带变换矩阵）
pub fn contain_stroke_with_transform(
    path: &PathProxy,
    transform: &Matrix,
    line_width: f32,
    x: f64,
    y: f64,
) -> bool {
    if let Some(local) = invert_transform_point(transform, x, y) {
        let scale = transform_scale(transform);
        contain_stroke(path, line_width / scale, local.x, local.y)
    } else {
        false
    }
}

fn invert_transform_point(m: &Matrix, x: f64, y: f64) -> Option<Point> {
    let det = m[0] as f64 * m[3] as f64 - m[2] as f64 * m[1] as f64;
    if det.abs() < 1e-12 {
        return None;
    }
    let inv_det = 1.0 / det;
    let tx = x - m[4] as f64;
    let ty = y - m[5] as f64;
    Some(Point::new(
        (m[3] as f64 * tx - m[2] as f64 * ty) * inv_det,
        (-m[1] as f64 * tx + m[0] as f64 * ty) * inv_det,
    ))
}

fn transform_scale(m: &Matrix) -> f32 {
    let sx = (m[0] * m[0] + m[1] * m[1]).sqrt();
    let sy = (m[2] * m[2] + m[3] * m[3]).sqrt();
    sx.max(sy).max(1e-6)
}

fn to_bezpath(path: &PathProxy) -> BezPath {
    let mut bez = BezPath::new();
    for cmd in path.commands() {
        match cmd {
            PathCmd::MoveTo(x, y) => bez.move_to(Point::new(*x as f64, *y as f64)),
            PathCmd::LineTo(x, y) => bez.line_to(Point::new(*x as f64, *y as f64)),
            PathCmd::CubicBezier(p) => bez.curve_to(
                Point::new(p.cp1x as f64, p.cp1y as f64),
                Point::new(p.cp2x as f64, p.cp2y as f64),
                Point::new(p.x as f64, p.y as f64),
            ),
            PathCmd::QuadraticBezier(p) => bez.quad_to(
                Point::new(p.cpx as f64, p.cpy as f64),
                Point::new(p.x as f64, p.y as f64),
            ),
            PathCmd::Arc(p) => append_arc(&mut bez, p),
            PathCmd::Rect(p) => append_rect(&mut bez, p),
            PathCmd::ClosePath => bez.close_path(),
        }
    }
    bez
}

fn append_rect(bez: &mut BezPath, params: &RectParams) {
    let x = params.x as f64;
    let y = params.y as f64;
    let w = params.width as f64;
    let h = params.height as f64;
    bez.move_to(Point::new(x, y));
    bez.line_to(Point::new(x + w, y));
    bez.line_to(Point::new(x + w, y + h));
    bez.line_to(Point::new(x, y + h));
    bez.close_path();
}

fn append_arc(bez: &mut BezPath, params: &ArcParams) {
    let steps = 32;
    let start = params.start_angle as f64;
    let end = params.end_angle as f64;
    let mut delta = end - start;
    if params.anticlockwise {
        if delta > 0.0 {
            delta -= std::f64::consts::TAU;
        }
    } else if delta < 0.0 {
        delta += std::f64::consts::TAU;
    }

    for i in 0..=steps {
        let t = start + delta * (i as f64 / steps as f64);
        let x = params.x as f64 + params.radius as f64 * t.cos();
        let y = params.y as f64 + params.radius as f64 * t.sin();
        if i == 0 {
            bez.move_to(Point::new(x, y));
        } else {
            bez.line_to(Point::new(x, y));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vl_convert_canvas2d::RectParams;

    #[test]
    fn rect_fill_contain() {
        let mut proxy = PathProxy::new();
        proxy.rect(&RectParams {
            x: 10.0,
            y: 10.0,
            width: 80.0,
            height: 50.0,
        });

        assert!(contain(&proxy, 50.0, 35.0));
        assert!(!contain(&proxy, 5.0, 5.0));
    }

    #[test]
    fn rect_stroke_contain() {
        let mut proxy = PathProxy::new();
        proxy.rect(&RectParams {
            x: 10.0,
            y: 10.0,
            width: 80.0,
            height: 50.0,
        });

        assert!(contain_stroke(&proxy, 4.0, 10.0, 12.0));
        assert!(!contain_stroke(&proxy, 4.0, 0.0, 0.0));
    }
}
