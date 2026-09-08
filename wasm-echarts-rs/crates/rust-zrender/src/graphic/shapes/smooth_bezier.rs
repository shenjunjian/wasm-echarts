//! 对照官方 `graphic/helper/smoothBezier.ts`

use crate::core::types::Point;

fn v_clone(p: Point) -> Point {
    p
}

fn v_add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn v_sub(a: Point, b: Point) -> Point {
    (a.0 - b.0, a.1 - b.1)
}

fn v_scale(v: Point, s: f64) -> Point {
    (v.0 * s, v.1 * s)
}

fn v_dist(a: Point, b: Point) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

fn v_min(a: Point, b: Point) -> Point {
    (a.0.min(b.0), a.1.min(b.1))
}

fn v_max(a: Point, b: Point) -> Point {
    (a.0.max(b.0), a.1.max(b.1))
}

/// 返回与顶点交错的控制点：`[cp0, cp1, ...]`，每顶点一对。
pub fn smooth_bezier(
    points: &[Point],
    smooth: f64,
    is_loop: bool,
    constraint: Option<[Point; 2]>,
) -> Vec<Point> {
    let mut cps = Vec::new();
    if points.is_empty() {
        return cps;
    }

    let mut min = (f64::INFINITY, f64::INFINITY);
    let mut max = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    let constrained = if let Some(box_pts) = constraint {
        for p in points {
            min = v_min(min, *p);
            max = v_max(max, *p);
        }
        min = v_min(min, box_pts[0]);
        max = v_max(max, box_pts[1]);
        true
    } else {
        false
    };

    let len = points.len();
    for i in 0..len {
        let point = points[i];
        if !is_loop && (i == 0 || i == len - 1) {
            cps.push(v_clone(point));
            continue;
        }
        let prev = if is_loop {
            points[if i == 0 { len - 1 } else { i - 1 }]
        } else {
            points[i - 1]
        };
        let next = if is_loop {
            points[(i + 1) % len]
        } else {
            points[i + 1]
        };

        let mut v = v_sub(next, prev);
        v = v_scale(v, smooth);
        let mut d0 = v_dist(point, prev);
        let mut d1 = v_dist(point, next);
        let sum = d0 + d1;
        if sum != 0.0 {
            d0 /= sum;
            d1 /= sum;
        }
        let mut cp0 = v_add(point, v_scale(v, -d0));
        let mut cp1 = v_add(point, v_scale(v, d1));
        if constrained {
            cp0 = v_max(cp0, min);
            cp0 = v_min(cp0, max);
            cp1 = v_max(cp1, min);
            cp1 = v_min(cp1, max);
        }
        cps.push(cp0);
        cps.push(cp1);
    }

    if is_loop && !cps.is_empty() {
        let first = cps.remove(0);
        cps.push(first);
    }
    cps
}
