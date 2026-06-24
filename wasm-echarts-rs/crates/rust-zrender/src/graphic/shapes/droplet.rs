//! 水滴形状

use vl_convert_canvas2d::CubicBezierParams;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct DropletShape {
    pub cx: f64,
    pub cy: f64,
    pub width: f64,
    pub height: f64,
}

pub fn build_droplet_path(ctx: &mut PathProxy, shape: &DropletShape) {
    let x = shape.cx;
    let y = shape.cy;
    let a = shape.width;
    let b = shape.height;
    if a <= 0.0 || b <= 0.0 {
        return;
    }

    ctx.move_to(x as f32, (y + a) as f32);
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x + a) as f32,
        cp1y: (y + a) as f32,
        cp2x: (x + a * 3.0 / 2.0) as f32,
        cp2y: (y - a / 3.0) as f32,
        x: x as f32,
        y: (y - b) as f32,
    });
    ctx.cubic_bezier_to(CubicBezierParams {
        cp1x: (x - a * 3.0 / 2.0) as f32,
        cp1y: (y - a / 3.0) as f32,
        cp2x: (x - a) as f32,
        cp2y: (y + a) as f32,
        x: x as f32,
        y: (y + a) as f32,
    });
    ctx.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn droplet_has_closed_path() {
        let mut proxy = PathProxy::new();
        build_droplet_path(
            &mut proxy,
            &DropletShape {
                cx: 100.0,
                cy: 80.0,
                width: 30.0,
                height: 60.0,
            },
        );
        assert!(!proxy.is_empty());
    }
}
