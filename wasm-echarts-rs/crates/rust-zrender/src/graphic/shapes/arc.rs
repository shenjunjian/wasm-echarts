//! 圆弧（不连圆心）

use std::f64::consts::PI;

use vl_convert_canvas2d::ArcParams;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone)]
pub struct ArcShape {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    /// 默认 true，与 zrender ArcShape 一致
    pub clockwise: bool,
}

impl Default for ArcShape {
    fn default() -> Self {
        Self {
            cx: 0.0,
            cy: 0.0,
            r: 0.0,
            start_angle: 0.0,
            end_angle: PI * 2.0,
            clockwise: true,
        }
    }
}

pub fn build_arc_path(ctx: &mut PathProxy, shape: &ArcShape) {
    let r = shape.r.max(0.0);
    if r <= 0.0 {
        return;
    }
    let start = shape.start_angle;
    let unit_x = start.cos();
    let unit_y = start.sin();
    ctx.move_to(
        (shape.cx + unit_x * r) as f32,
        (shape.cy + unit_y * r) as f32,
    );
    ctx.arc(ArcParams {
        x: shape.cx as f32,
        y: shape.cy as f32,
        radius: r as f32,
        start_angle: start as f32,
        end_angle: shape.end_angle as f32,
        anticlockwise: !shape.clockwise,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_arc_has_commands() {
        let mut proxy = PathProxy::new();
        build_arc_path(
            &mut proxy,
            &ArcShape {
                cx: 50.0,
                cy: 50.0,
                r: 40.0,
                start_angle: 0.0,
                end_angle: PI / 2.0,
                clockwise: true,
            },
        );
        assert!(!proxy.is_empty());
    }
}
