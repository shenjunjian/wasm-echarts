//! 有向包围盒（OBB），SAT 碰撞检测

use super::bbox::BoundingRect;
use super::point::Point2;

const NEAR_ZERO: f64 = 1e-10;

#[derive(Debug, Clone)]
pub struct OrientedBoundingRect {
    corners: [Point2; 4],
    axes: [Point2; 2],
    origin: [f64; 2],
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IntersectOpt {
    pub touch_threshold: f64,
    pub direction: Option<f64>,
    pub bidirectional: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct IntersectContext {
    min_tv: Point2,
    max_tv: Point2,
    dir_min_tv: Point2,
    touch_threshold: f64,
    use_dir: bool,
    bidirectional: bool,
    direction: f64,
    negative_size: bool,
}

impl IntersectContext {
    fn reset(&mut self, opt: &IntersectOpt, use_mtv: bool) {
        self.touch_threshold = opt.touch_threshold.max(0.0);
        self.negative_size = false;

        if !use_mtv {
            return;
        }

        self.min_tv.set(f64::INFINITY, f64::INFINITY);
        self.max_tv.set(0.0, 0.0);
        self.use_dir = false;

        if let Some(direction) = opt.direction {
            self.use_dir = true;
            self.dir_min_tv.copy_from(&self.min_tv);
            self.direction = direction;
            self.bidirectional = opt.bidirectional;
        }
    }

    fn calc_dir_mtv(&mut self) {
        let min_tv = self.min_tv;
        let square_mag = min_tv.y * min_tv.y + min_tv.x * min_tv.x;
        let dir_sin = self.direction.sin();
        let dir_cos = self.direction.cos();
        let dot_prod = dir_sin * min_tv.y + dir_cos * min_tv.x;

        if near_zero(dot_prod) {
            if near_zero(min_tv.x) && near_zero(min_tv.y) {
                self.dir_min_tv.set(0.0, 0.0);
            }
            return;
        }

        let dir_tmp = Point2::new(
            square_mag * dir_cos / dot_prod,
            square_mag * dir_sin / dot_prod,
        );

        if near_zero(dir_tmp.x) && near_zero(dir_tmp.y) {
            self.dir_min_tv.set(0.0, 0.0);
            return;
        }

        let bidirectional_ok = self.bidirectional
            || (dir_cos * dir_tmp.x + dir_sin * dir_tmp.y) > 0.0;

        if bidirectional_ok && dir_tmp.len() < self.dir_min_tv.len() {
            self.dir_min_tv.copy_from(&dir_tmp);
        }
    }
}

impl OrientedBoundingRect {
    pub fn new() -> Self {
        Self {
            corners: [Point2::default(); 4],
            axes: [Point2::default(); 2],
            origin: [0.0, 0.0],
        }
    }

    pub fn from_bounding_rect_mut(&mut self, rect: &BoundingRect, transform: Option<&[f64; 6]>) {
        let x = rect.x;
        let y = rect.y;
        let x2 = x + rect.width;
        let y2 = y + rect.height;

        self.corners[0].set(x, y);
        self.corners[1].set(x2, y);
        self.corners[2].set(x2, y2);
        self.corners[3].set(x, y2);

        if let Some(m) = transform {
            for corner in &mut self.corners {
                corner.transform(m);
            }
        }

        Point2::sub_out(&mut self.axes[0], &self.corners[1], &self.corners[0]);
        Point2::sub_out(&mut self.axes[1], &self.corners[3], &self.corners[0]);
        self.axes[0].normalize();
        self.axes[1].normalize();

        for i in 0..2 {
            self.origin[i] = self.axes[i].dot(&self.corners[0]);
        }
    }

    pub fn intersect(
        &self,
        other: &OrientedBoundingRect,
        mut mtv: Option<&mut Point2>,
        opt: &IntersectOpt,
    ) -> bool {
        let no_mtv = mtv.is_none();
        let mut ctx = IntersectContext::default();
        ctx.reset(opt, !no_mtv);

        if let Some(mtv) = mtv.as_mut() {
            mtv.set(0.0, 0.0);
        }

        let mut overlapped = true;

        if !Self::intersect_check_one_side(self, other, &mut ctx, no_mtv, 1.0) {
            overlapped = false;
            if no_mtv {
                return overlapped;
            }
        }
        if !Self::intersect_check_one_side(other, self, &mut ctx, no_mtv, -1.0) {
            overlapped = false;
            if no_mtv {
                return overlapped;
            }
        }

        if let Some(mtv) = mtv {
            if !ctx.negative_size {
                let chosen = if overlapped {
                    if ctx.use_dir {
                        ctx.dir_min_tv
                    } else {
                        ctx.min_tv
                    }
                } else {
                    ctx.max_tv
                };
                mtv.copy_from(&chosen);
            } else {
                mtv.set(0.0, 0.0);
            }
        }

        overlapped
    }

    fn intersect_check_one_side(
        self_obb: &OrientedBoundingRect,
        other: &OrientedBoundingRect,
        ctx: &mut IntersectContext,
        no_mtv: bool,
        inverse: f64,
    ) -> bool {
        let mut overlapped = true;
        let mut extent = [0.0, 0.0];
        let mut extent2 = [0.0, 0.0];

        for i in 0..2 {
            let axis = self_obb.axes[i];
            self_obb.get_proj_min_max_on_axis(i, &self_obb.corners, ctx, &mut extent);
            self_obb.get_proj_min_max_on_axis(i, &other.corners, ctx, &mut extent2);

            if ctx.negative_size || extent[1] < extent2[0] || extent[0] > extent2[1] {
                overlapped = false;
                if ctx.negative_size || no_mtv {
                    return overlapped;
                }
                let dist0 = (extent2[0] - extent[1]).abs();
                let dist1 = (extent[0] - extent2[1]).abs();

                if dist0.min(dist1) > ctx.max_tv.len() {
                    if dist0 < dist1 {
                        Point2::scale_out(&mut ctx.max_tv, &axis, -dist0 * inverse);
                    } else {
                        Point2::scale_out(&mut ctx.max_tv, &axis, dist1 * inverse);
                    }
                }
            } else if !no_mtv {
                let dist0 = (extent2[0] - extent[1]).abs();
                let dist1 = (extent[0] - extent2[1]).abs();

                if ctx.use_dir || dist0.min(dist1) < ctx.min_tv.len() {
                    if dist0 < dist1 || !ctx.bidirectional {
                        Point2::scale_out(&mut ctx.min_tv, &axis, dist0 * inverse);
                        if ctx.use_dir {
                            ctx.calc_dir_mtv();
                        }
                    }
                    if dist0 >= dist1 || !ctx.bidirectional {
                        Point2::scale_out(&mut ctx.min_tv, &axis, -dist1 * inverse);
                        if ctx.use_dir {
                            ctx.calc_dir_mtv();
                        }
                    }
                }
            }
        }
        overlapped
    }

    fn get_proj_min_max_on_axis(
        &self,
        dim: usize,
        corners: &[Point2; 4],
        ctx: &mut IntersectContext,
        out: &mut [f64; 2],
    ) {
        let axis = self.axes[dim];
        let origin = self.origin[dim];
        let proj = corners[0].dot(&axis) + origin;
        let mut min = proj;
        let mut max = proj;

        for corner in corners.iter().skip(1) {
            let proj = corner.dot(&axis) + origin;
            min = min.min(proj);
            max = max.max(proj);
        }

        out[0] = min + ctx.touch_threshold;
        out[1] = max - ctx.touch_threshold;
        ctx.negative_size = out[1] < out[0];
    }
}

fn near_zero(val: f64) -> bool {
    val.abs() < NEAR_ZERO
}

#[cfg(test)]
mod tests {
    use super::*;

    fn axis_aligned_obb(x: f64, y: f64, w: f64, h: f64) -> OrientedBoundingRect {
        let mut obb = OrientedBoundingRect::new();
        obb.from_bounding_rect_mut(&BoundingRect::new(x, y, w, h), None);
        obb
    }

    #[test]
    fn overlapping_axis_aligned_obbs_intersect() {
        let a = axis_aligned_obb(0.0, 0.0, 10.0, 10.0);
        let b = axis_aligned_obb(5.0, 5.0, 10.0, 10.0);
        assert!(a.intersect(&b, None, &IntersectOpt::default()));
    }

    #[test]
    fn disjoint_obbs_do_not_intersect() {
        let a = axis_aligned_obb(0.0, 0.0, 10.0, 10.0);
        let b = axis_aligned_obb(20.0, 20.0, 10.0, 10.0);
        assert!(!a.intersect(&b, None, &IntersectOpt::default()));
    }

    #[test]
    fn rotated_obb_intersects_overlapping_rect() {
        let mut a = axis_aligned_obb(0.0, 0.0, 20.0, 20.0);
        let mut b = OrientedBoundingRect::new();
        b.from_bounding_rect_mut(
            &BoundingRect::new(5.0, 5.0, 10.0, 10.0),
            Some(&[
                0.7071068, 0.7071068, -0.7071068, 0.7071068, 10.0, 0.0,
            ]),
        );
        assert!(a.intersect(&b, None, &IntersectOpt::default()));
        let _ = &mut a;
    }
}
