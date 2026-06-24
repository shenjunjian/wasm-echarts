//! 轴对齐包围盒

use super::point::Point2;

pub type Matrix6 = [f64; 6];

const MATRIX_EPSILON: f64 = 1e-5;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BoundingRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        let mut rect = Self {
            x,
            y,
            width,
            height,
        };
        rect.normalize_negative_size();
        rect
    }

    pub fn set(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
        self.normalize_negative_size();
    }

    pub fn copy_from(&mut self, other: &BoundingRect) {
        self.x = other.x;
        self.y = other.y;
        self.width = other.width;
        self.height = other.height;
    }

    pub fn is_zero_area(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0.0 || self.height == 0.0
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
    }

    pub fn intersects_viewport(&self, vw: f64, vh: f64) -> bool {
        self.x + self.width >= 0.0
            && self.y + self.height >= 0.0
            && self.x <= vw
            && self.y <= vh
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        x >= self.x
            && x <= self.x + self.width
            && y >= self.y
            && y <= self.y + self.height
    }

    pub fn union(&mut self, other: &BoundingRect) {
        let x = other.x.min(self.x);
        let y = other.y.min(self.y);

        if self.x.is_finite() && self.width.is_finite() {
            self.width = (other.x + other.width).max(self.x + self.width) - x;
        } else {
            self.width = other.width;
        }

        if self.y.is_finite() && self.height.is_finite() {
            self.height = (other.y + other.height).max(self.y + self.height) - y;
        } else {
            self.height = other.height;
        }

        self.x = x;
        self.y = y;
    }

    pub fn apply_transform(target: &mut BoundingRect, source: &BoundingRect, m: Option<&Matrix6>) {
        let Some(m) = m else {
            if !std::ptr::eq(target as *const _, source as *const _) {
                target.copy_from(source);
            }
            return;
        };

        if m[1].abs() < MATRIX_EPSILON
            && m[2].abs() < MATRIX_EPSILON
        {
            let sx = m[0];
            let sy = m[3];
            let tx = m[4];
            let ty = m[5];
            target.x = source.x * sx + tx;
            target.y = source.y * sy + ty;
            target.width = source.width * sx;
            target.height = source.height * sy;
            target.normalize_negative_size();
            return;
        }

        let mut lt = Point2::new(source.x, source.y);
        let mut rt = Point2::new(source.x + source.width, source.y);
        let mut rb = Point2::new(source.x + source.width, source.y + source.height);
        let mut lb = Point2::new(source.x, source.y + source.height);

        lt.transform(m);
        rt.transform(m);
        rb.transform(m);
        lb.transform(m);

        let min_x = lt.x.min(rt.x).min(rb.x).min(lb.x);
        let min_y = lt.y.min(rt.y).min(rb.y).min(lb.y);
        let max_x = lt.x.max(rt.x).max(rb.x).max(lb.x);
        let max_y = lt.y.max(rt.y).max(rb.y).max(lb.y);

        target.x = min_x;
        target.y = min_y;
        target.width = max_x - min_x;
        target.height = max_y - min_y;
    }

    pub fn intersect(a: &BoundingRect, b: &BoundingRect, touch_threshold: f64) -> bool {
        let threshold = touch_threshold.max(0.0);
        let ax0 = a.x + threshold;
        let ax1 = a.x + a.width - threshold;
        let ay0 = a.y + threshold;
        let ay1 = a.y + a.height - threshold;
        let bx0 = b.x + threshold;
        let bx1 = b.x + b.width - threshold;
        let by0 = b.y + threshold;
        let by1 = b.y + b.height - threshold;

        if ax0 > ax1 || ay0 > ay1 || bx0 > bx1 || by0 > by1 {
            return false;
        }

        !(ax1 < bx0 || bx1 < ax0 || ay1 < by0 || by1 < ay0)
    }

    fn normalize_negative_size(&mut self) {
        if self.width < 0.0 {
            self.x += self.width;
            self.width = -self.width;
        }
        if self.height < 0.0 {
            self.y += self.height;
            self.height = -self.height;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contain_and_intersect() {
        let rect = BoundingRect::new(10.0, 20.0, 30.0, 40.0);
        assert!(rect.contain(10.0, 20.0));
        assert!(rect.contain(40.0, 60.0));
        assert!(!rect.contain(9.0, 20.0));

        let other = BoundingRect::new(35.0, 50.0, 10.0, 10.0);
        assert!(BoundingRect::intersect(&rect, &other, 0.0));

        let disjoint = BoundingRect::new(100.0, 100.0, 10.0, 10.0);
        assert!(!BoundingRect::intersect(&rect, &disjoint, 0.0));
    }

    #[test]
    fn union_merges_rects() {
        let mut a = BoundingRect::new(0.0, 0.0, 10.0, 10.0);
        a.union(&BoundingRect::new(5.0, 5.0, 10.0, 10.0));
        assert_eq!(a, BoundingRect::new(0.0, 0.0, 15.0, 15.0));
    }

    #[test]
    fn negative_size_normalized() {
        let rect = BoundingRect::new(10.0, 20.0, -30.0, -40.0);
        assert_eq!(rect, BoundingRect::new(-20.0, -20.0, 30.0, 40.0));
    }

    #[test]
    fn apply_transform_translate() {
        let source = BoundingRect::new(10.0, 20.0, 30.0, 40.0);
        let m = [1.0, 0.0, 0.0, 1.0, 5.0, 6.0];
        let mut target = BoundingRect::default();
        BoundingRect::apply_transform(&mut target, &source, Some(&m));
        assert_eq!(target, BoundingRect::new(15.0, 26.0, 30.0, 40.0));
    }
}
