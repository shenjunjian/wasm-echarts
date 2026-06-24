//! 内外旋轮曲线

use std::f64::consts::PI;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone)]
pub struct TrochoidShape {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub r0: f64,
    pub d: f64,
    pub location: String,
}

impl Default for TrochoidShape {
    fn default() -> Self {
        Self {
            cx: 0.0,
            cy: 0.0,
            r: 0.0,
            r0: 0.0,
            d: 0.0,
            location: "out".to_string(),
        }
    }
}

pub fn build_trochoid_path(ctx: &mut PathProxy, shape: &TrochoidShape) {
    let big_r = shape.r;
    let small_r = shape.r0;
    let d = shape.d;
    let offset_x = shape.cx;
    let offset_y = shape.cy;
    let delta = if shape.location == "out" { 1.0 } else { -1.0 };

    if !shape.location.is_empty() && big_r <= small_r {
        return;
    }
    if small_r == 0.0 {
        return;
    }

    let mut num = 0u32;
    let denom = big_r + delta * small_r;
    if denom.abs() < f64::EPSILON {
        return;
    }

    loop {
        num += 1;
        if (small_r * num as f64) % denom == 0.0 {
            break;
        }
        if num > 10_000 {
            return;
        }
    }

    let x1 = (big_r + delta * small_r) * 0.0_f64.cos() - delta * d * 0.0_f64.cos() + offset_x;
    let y1 = (big_r + delta * small_r) * 0.0_f64.sin() - d * 0.0_f64.sin() + offset_y;
    ctx.move_to(x1 as f32, y1 as f32);

    let limit = (small_r * num as f64) / denom * 360.0;
    let mut i = 1u32;
    while i as f64 <= limit {
        let theta = PI / 180.0 * i as f64;
        let x2 = (big_r + delta * small_r) * theta.cos()
            - delta * d * ((big_r / small_r + delta) * theta).cos()
            + offset_x;
        let y2 = (big_r + delta * small_r) * theta.sin()
            - d * ((big_r / small_r + delta) * theta).sin()
            + offset_y;
        ctx.line_to(x2 as f32, y2 as f32);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trochoid_has_path() {
        let mut proxy = PathProxy::new();
        build_trochoid_path(
            &mut proxy,
            &TrochoidShape {
                cx: 100.0,
                cy: 80.0,
                r: 80.0,
                r0: 20.0,
                d: 30.0,
                location: "out".to_string(),
            },
        );
        assert!(!proxy.is_empty());
    }
}
