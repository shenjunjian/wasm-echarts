//! n 角星（n >= 2）

use std::f64::consts::PI;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct StarShape {
    pub cx: f64,
    pub cy: f64,
    pub n: u32,
    pub r0: Option<f64>,
    pub r: f64,
}

fn default_r0(n: u32, r: f64) -> f64 {
    if n > 4 {
        r * (2.0 * PI / n as f64).cos() / (PI / n as f64).cos()
    } else {
        r / 3.0
    }
}

pub fn build_star_path(ctx: &mut PathProxy, shape: &StarShape) {
    let n = shape.n;
    if n < 2 {
        return;
    }
    let x = shape.cx;
    let y = shape.cy;
    let r = shape.r;
    if r <= 0.0 {
        return;
    }
    let r0 = shape.r0.unwrap_or_else(|| default_r0(n, r));

    let d_step = PI / n as f64;
    let mut deg = -PI / 2.0;
    let x_start = x + r * deg.cos();
    let y_start = y + r * deg.sin();
    deg += d_step;

    ctx.move_to(x_start as f32, y_start as f32);
    for i in 0..(n * 2 - 1) {
        let ri = if i % 2 == 0 { r0 } else { r };
        ctx.line_to(
            (x + ri * deg.cos()) as f32,
            (y + ri * deg.sin()) as f32,
        );
        deg += d_step;
    }
    ctx.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_has_closed_path() {
        let mut proxy = PathProxy::new();
        build_star_path(
            &mut proxy,
            &StarShape {
                cx: 100.0,
                cy: 80.0,
                n: 5,
                r0: None,
                r: 50.0,
            },
        );
        assert!(!proxy.is_empty());
    }
}
