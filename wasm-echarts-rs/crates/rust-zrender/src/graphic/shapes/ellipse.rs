//! 椭圆（4 段 cubic bezier 近似）

use vl_convert_canvas2d::CubicBezierParams;

use crate::graphic::path_proxy::PathProxy;

const K: f64 = 0.5522848;

#[derive(Debug, Clone, Default)]
pub struct EllipseShape {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

pub fn build_ellipse_path(ctx: &mut PathProxy, shape: &EllipseShape) {
    let x = shape.cx;
    let y = shape.cy;
    let a = shape.rx;
    let b = shape.ry;
    if a <= 0.0 || b <= 0.0 {
        return;
    }
    let ox = a * K;
    let oy = b * K;

    ctx.move_to((x - a) as f32, y as f32);
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x - a) as f32,
        cp1y: (y - oy) as f32,
        cp2x: (x - ox) as f32,
        cp2y: (y - b) as f32,
        x: x as f32,
        y: (y - b) as f32,
    });
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x + ox) as f32,
        cp1y: (y - b) as f32,
        cp2x: (x + a) as f32,
        cp2y: (y - oy) as f32,
        x: (x + a) as f32,
        y: y as f32,
    });
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x + a) as f32,
        cp1y: (y + oy) as f32,
        cp2x: (x + ox) as f32,
        cp2y: (y + b) as f32,
        x: x as f32,
        y: (y + b) as f32,
    });
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x - ox) as f32,
        cp1y: (y + b) as f32,
        cp2x: (x - a) as f32,
        cp2y: (y + oy) as f32,
        x: (x - a) as f32,
        y: y as f32,
    });
    ctx.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ellipse_has_closed_path() {
        let mut proxy = PathProxy::new();
        build_ellipse_path(
            &mut proxy,
            &EllipseShape {
                cx: 100.0,
                cy: 80.0,
                rx: 60.0,
                ry: 40.0,
            },
        );
        assert!(!proxy.is_empty());
    }
}
