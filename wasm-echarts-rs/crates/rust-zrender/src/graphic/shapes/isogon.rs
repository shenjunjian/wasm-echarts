//! 正多边形

use std::f64::consts::PI;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct IsogonShape {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub n: u32,
}

pub fn build_isogon_path(ctx: &mut PathProxy, shape: &IsogonShape) {
    let n = shape.n;
    if n < 2 {
        return;
    }
    let x = shape.x;
    let y = shape.y;
    let r = shape.r;
    if r <= 0.0 {
        return;
    }

    let d_step = 2.0 * PI / n as f64;
    let mut deg = -PI / 2.0;

    ctx.move_to(
        (x + r * deg.cos()) as f32,
        (y + r * deg.sin()) as f32,
    );
    for _ in 0..n - 1 {
        deg += d_step;
        ctx.line_to(
            (x + r * deg.cos()) as f32,
            (y + r * deg.sin()) as f32,
        );
    }
    ctx.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isogon_has_closed_path() {
        let mut proxy = PathProxy::new();
        build_isogon_path(
            &mut proxy,
            &IsogonShape {
                x: 100.0,
                y: 80.0,
                r: 50.0,
                n: 6,
            },
        );
        assert!(!proxy.is_empty());
    }
}
