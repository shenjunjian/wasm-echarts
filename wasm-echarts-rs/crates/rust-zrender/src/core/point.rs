//! 二维点，对齐 zrender Point.ts 核心运算

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn copy_from(&mut self, other: &Point2) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn add(&mut self, other: &Point2) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub(&mut self, other: &Point2) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn scale_and_add(&mut self, other: &Point2, scalar: f64) {
        self.x += other.x * scalar;
        self.y += other.y * scalar;
    }

    pub fn dot(&self, other: &Point2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn len_square(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
        }
    }

    pub fn distance(&self, other: &Point2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn distance_square(&self, other: &Point2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn transform(&mut self, m: &[f64; 6]) {
        let x = self.x;
        let y = self.y;
        self.x = m[0] * x + m[2] * y + m[4];
        self.y = m[1] * x + m[3] * y + m[5];
    }

    pub fn sub_out(out: &mut Point2, a: &Point2, b: &Point2) {
        out.x = a.x - b.x;
        out.y = a.y - b.y;
    }

    pub fn scale_out(out: &mut Point2, axis: &Point2, scalar: f64) {
        out.x = axis.x * scalar;
        out.y = axis.y * scalar;
    }
}
