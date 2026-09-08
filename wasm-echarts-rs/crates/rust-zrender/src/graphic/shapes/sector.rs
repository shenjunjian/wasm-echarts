//! 扇形（饼图扇区），对照官方 `graphic/helper/roundSector.ts`

use std::f64::consts::PI;

use vl_convert_canvas2d::ArcParams;

use crate::graphic::path_proxy::PathProxy;

const E: f64 = 1e-4;
const PI2: f64 = PI * 2.0;

#[derive(Debug, Clone)]
pub struct SectorShape {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    /// 内半径；0 表示实心扇
    pub r0: f64,
    /// 弧度，0 为 3 点钟方向
    pub start_angle: f64,
    pub end_angle: f64,
    /// 默认 true，与官方 SectorShape 一致
    pub clockwise: bool,
    /// 四角圆角：内起、内终、外起、外终；空或全 0 表示无圆角
    pub corner_radius: Vec<f64>,
    /// 非官方字段，仅按角跨度比例截断；忽略则视为 1
    pub percent: f64,
}

impl Default for SectorShape {
    fn default() -> Self {
        Self {
            cx: 0.0,
            cy: 0.0,
            r: 0.0,
            r0: 0.0,
            start_angle: 0.0,
            end_angle: PI2,
            clockwise: true,
            corner_radius: Vec::new(),
            percent: 1.0,
        }
    }
}

struct CornerTangents {
    cx: f64,
    cy: f64,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

fn intersect(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> Option<(f64, f64)> {
    let dx10 = x1 - x0;
    let dy10 = y1 - y0;
    let dx32 = x3 - x2;
    let dy32 = y3 - y2;
    let mut t = dy32 * dx10 - dx32 * dy10;
    if t * t < E {
        return None;
    }
    t = (dx32 * (y0 - y2) - dy32 * (x0 - x2)) / t;
    Some((x0 + t * dx10, y0 + t * dy10))
}

fn compute_corner_tangents(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    radius: f64,
    cr: f64,
    clockwise: bool,
) -> CornerTangents {
    let x01 = x0 - x1;
    let y01 = y0 - y1;
    let lo = (if clockwise { cr } else { -cr }) / (x01 * x01 + y01 * y01).sqrt();
    let ox = lo * y01;
    let oy = -lo * x01;
    let x11 = x0 + ox;
    let y11 = y0 + oy;
    let x10 = x1 + ox;
    let y10 = y1 + oy;
    let x00 = (x11 + x10) / 2.0;
    let y00 = (y11 + y10) / 2.0;
    let dx = x10 - x11;
    let dy = y10 - y11;
    let d2 = dx * dx + dy * dy;
    let r = radius - cr;
    let s = x11 * y10 - x10 * y11;
    let d = (if dy < 0.0 { -1.0 } else { 1.0 }) * (r * r * d2 - s * s).max(0.0).sqrt();
    let mut cx0 = (s * dy - dx * d) / d2;
    let mut cy0 = (-s * dx - dy * d) / d2;
    let cx1 = (s * dy + dx * d) / d2;
    let cy1 = (-s * dx + dy * d) / d2;
    let dx0 = cx0 - x00;
    let dy0 = cy0 - y00;
    let dx1 = cx1 - x00;
    let dy1 = cy1 - y00;
    if dx0 * dx0 + dy0 * dy0 > dx1 * dx1 + dy1 * dy1 {
        cx0 = cx1;
        cy0 = cy1;
    }
    CornerTangents {
        cx: cx0,
        cy: cy0,
        x0: -ox,
        y0: -oy,
        x1: cx0 * (radius / r - 1.0),
        y1: cy0 * (radius / r - 1.0),
    }
}

/// 5 → [5,5,5,5]；[5] → [5,5,0,0]；[5,10] → [5,5,10,10]；[5,10,15] → [5,10,15,15]
fn normalize_corner_radius(cr: &[f64]) -> [f64; 4] {
    match cr.len() {
        0 => [0.0, 0.0, 0.0, 0.0],
        1 => [cr[0], cr[0], 0.0, 0.0],
        2 => [cr[0], cr[0], cr[1], cr[1]],
        3 => [cr[0], cr[1], cr[2], cr[2]],
        _ => [cr[0], cr[1], cr[2], cr[3]],
    }
}

fn path_arc(
    ctx: &mut PathProxy,
    x: f64,
    y: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    anticlockwise: bool,
) {
    ctx.arc(ArcParams {
        x: x as f32,
        y: y as f32,
        radius: radius as f32,
        start_angle: start_angle as f32,
        end_angle: end_angle as f32,
        anticlockwise,
    });
}

pub fn build_sector_path(ctx: &mut PathProxy, shape: &SectorShape) {
    let mut radius = shape.r.max(0.0);
    let mut inner_radius = shape.r0.max(0.0);
    let has_radius = radius > 0.0;
    let has_inner = inner_radius > 0.0;
    if !has_radius && !has_inner {
        return;
    }
    if !has_radius {
        radius = inner_radius;
        inner_radius = 0.0;
    }
    if inner_radius > radius {
        std::mem::swap(&mut radius, &mut inner_radius);
    }

    let start_angle = shape.start_angle;
    let mut end_angle = shape.end_angle;
    if shape.percent.is_finite() && (shape.percent - 1.0).abs() > f64::EPSILON {
        end_angle = start_angle + (end_angle - start_angle) * shape.percent;
    }
    if !start_angle.is_finite() || !end_angle.is_finite() {
        return;
    }

    let cx = shape.cx;
    let cy = shape.cy;
    let clockwise = shape.clockwise;

    let mut arc = (end_angle - start_angle).abs();
    let rem = if arc > PI2 { arc % PI2 } else { 0.0 };
    if rem > E {
        arc = rem;
    }

    if !(radius > E) {
        ctx.move_to(cx as f32, cy as f32);
        ctx.close_path();
        return;
    }

    if arc > PI2 - E {
        ctx.move_to(
            (cx + radius * start_angle.cos()) as f32,
            (cy + radius * start_angle.sin()) as f32,
        );
        path_arc(ctx, cx, cy, radius, start_angle, end_angle, !clockwise);
        if inner_radius > E {
            ctx.move_to(
                (cx + inner_radius * end_angle.cos()) as f32,
                (cy + inner_radius * end_angle.sin()) as f32,
            );
            path_arc(ctx, cx, cy, inner_radius, end_angle, start_angle, clockwise);
        }
        ctx.close_path();
        return;
    }

    let xrs = radius * start_angle.cos();
    let yrs = radius * start_angle.sin();
    let xire = inner_radius * end_angle.cos();
    let yire = inner_radius * end_angle.sin();
    let has_arc = arc > E;

    let mut icr_start = 0.0;
    let mut icr_end = 0.0;
    let mut ocr_start = 0.0;
    let mut ocr_end = 0.0;
    let mut limited_ocr_max = 0.0;
    let mut limited_icr_max = 0.0;
    let mut ocr_max = 0.0;
    let mut icr_max = 0.0;
    let mut xre = 0.0;
    let mut yre = 0.0;
    let mut xirs = 0.0;
    let mut yirs = 0.0;

    if has_arc {
        let has_corner = shape.corner_radius.iter().any(|v| *v > 0.0);
        if has_corner {
            let n = normalize_corner_radius(&shape.corner_radius);
            icr_start = n[0];
            icr_end = n[1];
            ocr_start = n[2];
            ocr_end = n[3];
        }
        let half_rd = (radius - inner_radius).abs() / 2.0;
        let ocrs = half_rd.min(ocr_start);
        let ocre = half_rd.min(ocr_end);
        let icrs = half_rd.min(icr_start);
        let icre = half_rd.min(icr_end);
        ocr_max = ocrs.max(ocre);
        icr_max = icrs.max(icre);
        limited_ocr_max = ocr_max;
        limited_icr_max = icr_max;

        if ocr_max > E || icr_max > E {
            xre = radius * end_angle.cos();
            yre = radius * end_angle.sin();
            xirs = inner_radius * start_angle.cos();
            yirs = inner_radius * start_angle.sin();
            if arc < PI {
                if let Some(it) = intersect(xrs, yrs, xirs, yirs, xre, yre, xire, yire) {
                    let x0 = xrs - it.0;
                    let y0 = yrs - it.1;
                    let x1 = xre - it.0;
                    let y1 = yre - it.1;
                    let denom = (x0 * x0 + y0 * y0).sqrt() * (x1 * x1 + y1 * y1).sqrt();
                    let a = 1.0 / (((x0 * x1 + y0 * y1) / denom).acos() / 2.0).sin();
                    let b = (it.0 * it.0 + it.1 * it.1).sqrt();
                    limited_ocr_max = ocr_max.min((radius - b) / (a + 1.0));
                    limited_icr_max = icr_max.min((inner_radius - b) / (a - 1.0));
                }
            }
        }
    }

    if !has_arc {
        ctx.move_to((cx + xrs) as f32, (cy + yrs) as f32);
    } else if limited_ocr_max > E {
        let cr_start = ocr_start.min(limited_ocr_max);
        let cr_end = ocr_end.min(limited_ocr_max);
        let ct0 = compute_corner_tangents(xirs, yirs, xrs, yrs, radius, cr_start, clockwise);
        let ct1 = compute_corner_tangents(xre, yre, xire, yire, radius, cr_end, clockwise);
        ctx.move_to(
            (cx + ct0.cx + ct0.x0) as f32,
            (cy + ct0.cy + ct0.y0) as f32,
        );
        if limited_ocr_max < ocr_max && cr_start == cr_end {
            path_arc(
                ctx,
                cx + ct0.cx,
                cy + ct0.cy,
                limited_ocr_max,
                ct0.y0.atan2(ct0.x0),
                ct1.y0.atan2(ct1.x0),
                !clockwise,
            );
        } else {
            if cr_start > 0.0 {
                path_arc(
                    ctx,
                    cx + ct0.cx,
                    cy + ct0.cy,
                    cr_start,
                    ct0.y0.atan2(ct0.x0),
                    ct0.y1.atan2(ct0.x1),
                    !clockwise,
                );
            }
            path_arc(
                ctx,
                cx,
                cy,
                radius,
                (ct0.cy + ct0.y1).atan2(ct0.cx + ct0.x1),
                (ct1.cy + ct1.y1).atan2(ct1.cx + ct1.x1),
                !clockwise,
            );
            if cr_end > 0.0 {
                path_arc(
                    ctx,
                    cx + ct1.cx,
                    cy + ct1.cy,
                    cr_end,
                    ct1.y1.atan2(ct1.x1),
                    ct1.y0.atan2(ct1.x0),
                    !clockwise,
                );
            }
        }
    } else {
        ctx.move_to((cx + xrs) as f32, (cy + yrs) as f32);
        path_arc(ctx, cx, cy, radius, start_angle, end_angle, !clockwise);
    }

    if !(inner_radius > E) || !has_arc {
        ctx.line_to((cx + xire) as f32, (cy + yire) as f32);
    } else if limited_icr_max > E {
        let cr_start = icr_start.min(limited_icr_max);
        let cr_end = icr_end.min(limited_icr_max);
        let ct0 = compute_corner_tangents(xire, yire, xre, yre, inner_radius, -cr_end, clockwise);
        let ct1 = compute_corner_tangents(xrs, yrs, xirs, yirs, inner_radius, -cr_start, clockwise);
        ctx.line_to(
            (cx + ct0.cx + ct0.x0) as f32,
            (cy + ct0.cy + ct0.y0) as f32,
        );
        if limited_icr_max < icr_max && cr_start == cr_end {
            path_arc(
                ctx,
                cx + ct0.cx,
                cy + ct0.cy,
                limited_icr_max,
                ct0.y0.atan2(ct0.x0),
                ct1.y0.atan2(ct1.x0),
                !clockwise,
            );
        } else {
            if cr_end > 0.0 {
                path_arc(
                    ctx,
                    cx + ct0.cx,
                    cy + ct0.cy,
                    cr_end,
                    ct0.y0.atan2(ct0.x0),
                    ct0.y1.atan2(ct0.x1),
                    !clockwise,
                );
            }
            path_arc(
                ctx,
                cx,
                cy,
                inner_radius,
                (ct0.cy + ct0.y1).atan2(ct0.cx + ct0.x1),
                (ct1.cy + ct1.y1).atan2(ct1.cx + ct1.x1),
                clockwise,
            );
            if cr_start > 0.0 {
                path_arc(
                    ctx,
                    cx + ct1.cx,
                    cy + ct1.cy,
                    cr_start,
                    ct1.y1.atan2(ct1.x1),
                    ct1.y0.atan2(ct1.x0),
                    !clockwise,
                );
            }
        }
    } else {
        ctx.line_to((cx + xire) as f32, (cy + yire) as f32);
        path_arc(
            ctx,
            cx,
            cy,
            inner_radius,
            end_angle,
            start_angle,
            clockwise,
        );
    }

    ctx.close_path();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic::path_proxy::PathCmd;

    fn arc_count(proxy: &PathProxy) -> usize {
        proxy
            .commands()
            .iter()
            .filter(|c| matches!(c, PathCmd::Arc(_)))
            .count()
    }

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
                ..Default::default()
            },
        );
        assert!(!proxy.is_empty());
        assert!(arc_count(&proxy) >= 1);
    }

    #[test]
    fn annulus_sector_has_inner_and_outer_arc() {
        let mut proxy = PathProxy::new();
        build_sector_path(
            &mut proxy,
            &SectorShape {
                cx: 50.0,
                cy: 50.0,
                r: 40.0,
                r0: 20.0,
                start_angle: 0.0,
                end_angle: PI / 2.0,
                clockwise: true,
                ..Default::default()
            },
        );
        assert!(!proxy.is_empty());
        assert!(
            arc_count(&proxy) >= 2,
            "r0 > 0 should stroke both outer and inner radius"
        );
        assert!(proxy.commands().iter().any(|c| matches!(c, PathCmd::ClosePath)));
    }

    #[test]
    fn full_ring_sector_has_two_arcs() {
        let mut proxy = PathProxy::new();
        build_sector_path(
            &mut proxy,
            &SectorShape {
                cx: 50.0,
                cy: 50.0,
                r: 40.0,
                r0: 18.0,
                start_angle: 0.0,
                end_angle: PI2,
                ..Default::default()
            },
        );
        assert_eq!(arc_count(&proxy), 2);
    }
}
