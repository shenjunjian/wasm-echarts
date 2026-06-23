//! 轴对齐包围盒

#[derive(Debug, Clone, Copy, Default)]
pub struct BoundingRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn is_zero_area(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    pub fn intersects_viewport(&self, vw: f64, vh: f64) -> bool {
        self.x + self.width >= 0.0
            && self.y + self.height >= 0.0
            && self.x <= vw
            && self.y <= vh
    }
}
