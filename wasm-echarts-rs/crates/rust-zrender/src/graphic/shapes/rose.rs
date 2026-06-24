//! 玫瑰线

use std::f64::consts::PI;

use crate::graphic::path_proxy::PathProxy;

const RADIAN: f64 = PI / 180.0;

#[derive(Debug, Clone, Default)]
pub struct RoseShape {
    pub cx: f64,
    pub cy: f64,
    pub r: Vec<f64>,
    pub k: f64,
    pub n: u32,
}

pub fn build_rose_path(ctx: &mut PathProxy, shape: &RoseShape) {
    let k = shape.k;
    let n = shape.n;
    let x0 = shape.cx;
    let y0 = shape.cy;
    if shape.r.is_empty() || n == 0 {
        return;
    }

    ctx.move_to(x0 as f32, y0 as f32);

    for r in &shape.r {
        if *r <= 0.0 {
            continue;
        }
        let max_j = 360 * n;
        for j in 0..=max_j {
            let angle = j as f64 * RADIAN;
            let inner = ((k / n as f64 * j as f64) % 360.0) * RADIAN;
            let x = r * inner.sin() * angle.cos() + x0;
            let y = r * inner.sin() * angle.sin() + y0;
            ctx.line_to(x as f32, y as f32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rose_has_path() {
        let mut proxy = PathProxy::new();
        build_rose_path(
            &mut proxy,
            &RoseShape {
                cx: 100.0,
                cy: 80.0,
                r: vec![40.0],
                k: 3.0,
                n: 1,
            },
        );
        assert!(!proxy.is_empty());
    }
}
