use std::f64::consts::PI;

use vl_convert_canvas2d::{ArcParams, RectParams};

use crate::graphic::path_proxy::PathProxy;

#[derive(Debug, Clone, Default)]
pub struct RectShape {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// 圆角：数字或 CSS 缩写数组，空表示直角
    pub r: Vec<f64>,
}

/// 官方 roundRect：r / [r] / [r1,r2] / [r1,r2,r3] / [r1,r2,r3,r4]
fn normalize_rect_radii(r: &[f64]) -> [f64; 4] {
    match r.len() {
        0 => [0.0, 0.0, 0.0, 0.0],
        1 => [r[0], r[0], r[0], r[0]],
        2 => [r[0], r[1], r[0], r[1]],
        3 => [r[0], r[1], r[2], r[1]],
        _ => [r[0], r[1], r[2], r[3]],
    }
}

fn path_arc(
    ctx: &mut PathProxy,
    x: f64,
    y: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) {
    ctx.arc(ArcParams {
        x: x as f32,
        y: y as f32,
        radius: radius as f32,
        start_angle: start_angle as f32,
        end_angle: end_angle as f32,
        anticlockwise: false,
    });
}

pub fn build_rect_path(ctx: &mut PathProxy, shape: &RectShape) {
    let mut x = shape.x;
    let mut y = shape.y;
    let mut width = shape.width;
    let mut height = shape.height;
    if width < 0.0 {
        x += width;
        width = -width;
    }
    if height < 0.0 {
        y += height;
        height = -height;
    }

    let has_radius = shape.r.iter().any(|v| *v > 0.0);
    if !has_radius {
        ctx.rect(&RectParams {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        });
        return;
    }

    let mut r = normalize_rect_radii(&shape.r);
    if r[0] + r[1] > width {
        let total = r[0] + r[1];
        r[0] *= width / total;
        r[1] *= width / total;
    }
    if r[2] + r[3] > width {
        let total = r[2] + r[3];
        r[2] *= width / total;
        r[3] *= width / total;
    }
    if r[1] + r[2] > height {
        let total = r[1] + r[2];
        r[1] *= height / total;
        r[2] *= height / total;
    }
    if r[0] + r[3] > height {
        let total = r[0] + r[3];
        r[0] *= height / total;
        r[3] *= height / total;
    }

    let (r1, r2, r3, r4) = (r[0], r[1], r[2], r[3]);
    ctx.move_to((x + r1) as f32, y as f32);
    ctx.line_to((x + width - r2) as f32, y as f32);
    if r2 != 0.0 {
        path_arc(ctx, x + width - r2, y + r2, r2, -PI / 2.0, 0.0);
    }
    ctx.line_to((x + width) as f32, (y + height - r3) as f32);
    if r3 != 0.0 {
        path_arc(ctx, x + width - r3, y + height - r3, r3, 0.0, PI / 2.0);
    }
    ctx.line_to((x + r4) as f32, (y + height) as f32);
    if r4 != 0.0 {
        path_arc(ctx, x + r4, y + height - r4, r4, PI / 2.0, PI);
    }
    ctx.line_to(x as f32, (y + r1) as f32);
    if r1 != 0.0 {
        path_arc(ctx, x + r1, y + r1, r1, PI, PI * 1.5);
    }
    ctx.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic::path_proxy::PathCmd;

    #[test]
    fn sharp_rect_uses_rect_command() {
        let mut proxy = PathProxy::new();
        build_rect_path(
            &mut proxy,
            &RectShape {
                x: 10.0,
                y: 20.0,
                width: 30.0,
                height: 40.0,
                r: Vec::new(),
            },
        );
        assert!(proxy.commands().iter().any(|c| matches!(c, PathCmd::Rect(_))));
        assert!(!proxy.commands().iter().any(|c| matches!(c, PathCmd::Arc(_))));
    }

    #[test]
    fn rounded_rect_uses_corner_arcs() {
        let mut proxy = PathProxy::new();
        build_rect_path(
            &mut proxy,
            &RectShape {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 60.0,
                r: vec![8.0],
            },
        );
        let arcs = proxy
            .commands()
            .iter()
            .filter(|c| matches!(c, PathCmd::Arc(_)))
            .count();
        assert_eq!(arcs, 4);
        assert!(!proxy.commands().iter().any(|c| matches!(c, PathCmd::Rect(_))));
    }
}
