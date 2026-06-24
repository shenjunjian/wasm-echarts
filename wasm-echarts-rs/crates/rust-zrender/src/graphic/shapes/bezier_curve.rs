//! 贝塞尔曲线（二次或三次，支持 percent 截断）

use vl_convert_canvas2d::{CubicBezierParams, QuadraticBezierParams};

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct BezierCurveShape {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub cpx1: f64,
    pub cpy1: f64,
    pub cpx2: Option<f64>,
    pub cpy2: Option<f64>,
    pub percent: f64,
}

fn quadratic_subdivide(p0: f64, p1: f64, p2: f64, t: f64) -> (f64, f64, f64) {
    let p01 = (p1 - p0) * t + p0;
    let p12 = (p2 - p1) * t + p1;
    let p012 = (p12 - p01) * t + p01;
    (p0, p01, p012)
}

fn cubic_subdivide(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> (f64, f64, f64, f64) {
    let p01 = (p1 - p0) * t + p0;
    let p12 = (p2 - p1) * t + p1;
    let p23 = (p3 - p2) * t + p2;
    let p012 = (p12 - p01) * t + p01;
    let p123 = (p23 - p12) * t + p12;
    let p0123 = (p123 - p012) * t + p012;
    (p0, p01, p012, p0123)
}

pub fn build_bezier_curve_path(ctx: &mut PathProxy, shape: &BezierCurveShape) {
    if shape.percent <= 0.0 {
        return;
    }

    let x1 = shape.x1;
    let y1 = shape.y1;
    let mut x2 = shape.x2;
    let mut y2 = shape.y2;
    let mut cpx1 = shape.cpx1;
    let mut cpy1 = shape.cpy1;
    let percent = shape.percent;

    ctx.move_to(x1 as f32, y1 as f32);

    match (shape.cpx2, shape.cpy2) {
        (Some(mut cpx2), Some(mut cpy2)) => {
            if percent < 1.0 {
                let (_, b, c, d) = cubic_subdivide(x1, cpx1, cpx2, x2, percent);
                cpx1 = b;
                cpx2 = c;
                x2 = d;
                let (_, b, c, d) = cubic_subdivide(y1, cpy1, cpy2, y2, percent);
                cpy1 = b;
                cpy2 = c;
                y2 = d;
            }
            ctx.cubic_bezier_to(CubicBezierParams {
                cp1x: cpx1 as f32,
                cp1y: cpy1 as f32,
                cp2x: cpx2 as f32,
                cp2y: cpy2 as f32,
                x: x2 as f32,
                y: y2 as f32,
            });
        }
        _ => {
            if percent < 1.0 {
                let (_, b, c) = quadratic_subdivide(x1, cpx1, x2, percent);
                cpx1 = b;
                x2 = c;
                let (_, b, c) = quadratic_subdivide(y1, cpy1, y2, percent);
                cpy1 = b;
                y2 = c;
            }
            ctx.quadratic_curve_to(QuadraticBezierParams {
                cpx: cpx1 as f32,
                cpy: cpy1 as f32,
                x: x2 as f32,
                y: y2 as f32,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_bezier_has_commands() {
        let mut proxy = PathProxy::new();
        build_bezier_curve_path(
            &mut proxy,
            &BezierCurveShape {
                x1: 0.0,
                y1: 0.0,
                x2: 100.0,
                y2: 100.0,
                cpx1: 50.0,
                cpy1: 0.0,
                cpx2: None,
                cpy2: None,
                percent: 1.0,
            },
        );
        assert!(!proxy.is_empty());
    }

    #[test]
    fn cubic_bezier_has_commands() {
        let mut proxy = PathProxy::new();
        build_bezier_curve_path(
            &mut proxy,
            &BezierCurveShape {
                x1: 0.0,
                y1: 0.0,
                x2: 100.0,
                y2: 100.0,
                cpx1: 20.0,
                cpy1: 80.0,
                cpx2: Some(80.0),
                cpy2: Some(20.0),
                percent: 1.0,
            },
        );
        assert!(!proxy.is_empty());
    }
}
