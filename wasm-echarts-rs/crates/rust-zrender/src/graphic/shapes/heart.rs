//! 心形

use vl_convert_canvas2d::CubicBezierParams;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct HeartShape {
    pub cx: f64,
    pub cy: f64,
    pub width: f64,
    pub height: f64,
}

pub fn build_heart_path(ctx: &mut PathProxy, shape: &HeartShape) {
    let x = shape.cx;
    let y = shape.cy;
    let a = shape.width;
    let b = shape.height;
    if a <= 0.0 || b <= 0.0 {
        return;
    }

    ctx.move_to(x as f32, y as f32);
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x + a / 2.0) as f32,
        cp1y: (y - b * 2.0 / 3.0) as f32,
        cp2x: (x + a * 2.0) as f32,
        cp2y: (y + b / 3.0) as f32,
        x: x as f32,
        y: (y + b) as f32,
    });
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x - a * 2.0) as f32,
        cp1y: (y + b / 3.0) as f32,
        cp2x: (x - a / 2.0) as f32,
        cp2y: (y - b * 2.0 / 3.0) as f32,
        x: x as f32,
        y: y as f32,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heart_has_path() {
        let mut proxy = PathProxy::new();
        build_heart_path(
            &mut proxy,
            &HeartShape {
                cx: 100.0,
                cy: 60.0,
                width: 40.0,
                height: 50.0,
            },
        );
        assert!(!proxy.is_empty());
    }
}
