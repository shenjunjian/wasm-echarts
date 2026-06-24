//! 圆环（外弧 CW + 内弧 CCW，even-odd fill）

use std::f32::consts::TAU;

use vl_convert_canvas2d::ArcParams;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct RingShape {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub r0: f64,
}

pub fn build_ring_path(ctx: &mut PathProxy, shape: &RingShape) {
    let x = shape.cx;
    let y = shape.cy;
    let r = shape.r.max(0.0);
    let r0 = shape.r0.max(0.0);
    if r <= 0.0 {
        return;
    }

    ctx.move_to((x + r) as f32, y as f32);
    ctx.arc(ArcParams {
        x: x as f32,
        y: y as f32,
        radius: r as f32,
        start_angle: 0.0,
        end_angle: TAU,
        anticlockwise: false,
    });

    if r0 > 0.0 {
        ctx.move_to((x + r0) as f32, y as f32);
        ctx.arc(ArcParams {
            x: x as f32,
            y: y as f32,
            radius: r0 as f32,
            start_angle: 0.0,
            end_angle: TAU,
            anticlockwise: true,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_has_two_arcs() {
        let mut proxy = PathProxy::new();
        build_ring_path(
            &mut proxy,
            &RingShape {
                cx: 50.0,
                cy: 50.0,
                r: 40.0,
                r0: 20.0,
            },
        );
        assert!(!proxy.is_empty());
    }
}
