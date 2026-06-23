//! 扇形（饼图扇区）

use std::f64::consts::PI;

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone)]
pub struct SectorShape {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    /// 弧度，ECharts 惯例：0 为 3 点钟方向，顺时针
    pub start_angle: f64,
    pub end_angle: f64,
    pub percent: f64,
}

pub fn build_sector_path(ctx: &mut PathProxy, shape: &SectorShape) {
    let span = (shape.end_angle - shape.start_angle) * shape.percent;
    if span.abs() < f64::EPSILON || shape.r <= 0.0 {
        return;
    }
    let start = shape.start_angle;
    let cx = shape.cx;
    let cy = shape.cy;
    let r = shape.r;

    let sx = cx + r * start.cos();
    let sy = cy + r * start.sin();
    ctx.move_to(cx as f32, cy as f32);
    ctx.line_to(sx as f32, sy as f32);
    // 用多段线近似弧
    let steps = ((span.abs() / PI * 24.0).ceil() as usize).max(2);
    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let a = start + span * t;
        ctx.line_to((cx + r * a.cos()) as f32, (cy + r * a.sin()) as f32);
    }
    ctx.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_circle_sector_has_area() {
        let mut proxy = PathProxy::new();
        build_sector_path(
            &mut proxy,
            &SectorShape {
                cx: 50.0,
                cy: 50.0,
                r: 40.0,
                start_angle: -PI / 2.0,
                end_angle: PI * 1.5,
                percent: 1.0,
            },
        );
        assert!(!proxy.is_empty());
    }
}
