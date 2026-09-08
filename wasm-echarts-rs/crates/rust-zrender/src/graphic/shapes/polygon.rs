use vl_convert_canvas2d::CubicBezierParams;

use crate::core::types::Point;
use crate::graphic::path_proxy::PathProxy;
use crate::graphic::shapes::smooth_bezier::smooth_bezier;

#[derive(Debug, Clone, Default)]
pub struct PolygonShape {
    pub points: Vec<Point>,
    pub smooth: f64,
    pub smooth_constraint: Option<[Point; 2]>,
}

pub fn build_polygon_path(ctx: &mut PathProxy, shape: &PolygonShape, close: bool) {
    let points = &shape.points;
    if points.len() < 2 {
        return;
    }
    if shape.smooth != 0.0 {
        let control_points = smooth_bezier(points, shape.smooth, close, shape.smooth_constraint);
        ctx.move_to(points[0].0 as f32, points[0].1 as f32);
        let len = points.len();
        let n = if close { len } else { len.saturating_sub(1) };
        for i in 0..n {
            let Some(cp1) = control_points.get(i * 2) else {
                break;
            };
            let Some(cp2) = control_points.get(i * 2 + 1) else {
                break;
            };
            let p = points[(i + 1) % len];
            ctx.cubic_bezier_to(CubicBezierParams {
                cp1x: cp1.0 as f32,
                cp1y: cp1.1 as f32,
                cp2x: cp2.0 as f32,
                cp2y: cp2.1 as f32,
                x: p.0 as f32,
                y: p.1 as f32,
            });
        }
    } else {
        ctx.move_to(points[0].0 as f32, points[0].1 as f32);
        for p in points.iter().skip(1) {
            ctx.line_to(p.0 as f32, p.1 as f32);
        }
    }
    if close {
        ctx.close_path();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic::path_proxy::PathCmd;

    #[test]
    fn unsmoothed_polygon_uses_line_to() {
        let mut proxy = PathProxy::new();
        build_polygon_path(
            &mut proxy,
            &PolygonShape {
                points: vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)],
                ..Default::default()
            },
            true,
        );
        assert!(proxy.commands().iter().any(|c| matches!(c, PathCmd::LineTo(_, _))));
        assert!(!proxy
            .commands()
            .iter()
            .any(|c| matches!(c, PathCmd::CubicBezier(_))));
    }

    #[test]
    fn smooth_polygon_uses_cubic_beziers() {
        let mut proxy = PathProxy::new();
        build_polygon_path(
            &mut proxy,
            &PolygonShape {
                points: vec![(0.0, 0.0), (40.0, 10.0), (20.0, 50.0), (0.0, 30.0)],
                smooth: 0.3,
                ..Default::default()
            },
            true,
        );
        let curves = proxy
            .commands()
            .iter()
            .filter(|c| matches!(c, PathCmd::CubicBezier(_)))
            .count();
        assert_eq!(curves, 4);
        assert!(proxy.commands().iter().any(|c| matches!(c, PathCmd::ClosePath)));
    }
}
