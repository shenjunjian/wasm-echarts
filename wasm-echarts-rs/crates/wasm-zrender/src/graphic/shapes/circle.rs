use std::f32::consts::TAU;

use vl_convert_canvas2d::ArcParams;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct CircleShape {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

pub fn build_circle_path(ctx: &mut PathProxy, shape: &CircleShape) {
    ctx.move_to((shape.cx + shape.r) as f32, shape.cy as f32);
    ctx.arc(ArcParams {
        x: shape.cx as f32,
        y: shape.cy as f32,
        radius: shape.r as f32,
        start_angle: 0.0,
        end_angle: TAU,
        anticlockwise: false,
    });
}
