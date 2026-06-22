use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct LineShape {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub percent: f64,
}

pub fn build_line_path(ctx: &mut PathProxy, shape: &LineShape) {
    if shape.percent <= 0.0 {
        return;
    }
    let mut x2 = shape.x2;
    let mut y2 = shape.y2;
    if shape.percent < 1.0 {
        x2 = shape.x1 * (1.0 - shape.percent) + shape.x2 * shape.percent;
        y2 = shape.y1 * (1.0 - shape.percent) + shape.y2 * shape.percent;
    }
    ctx.move_to(shape.x1 as f32, shape.y1 as f32);
    ctx.line_to(x2 as f32, y2 as f32);
}
