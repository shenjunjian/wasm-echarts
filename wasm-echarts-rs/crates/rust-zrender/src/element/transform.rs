//! Transformable：局部变换矩阵计算

use crate::core::matrix::{self, Matrix};

#[derive(Debug, Clone)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
    pub origin_x: f64,
    pub origin_y: f64,
    pub transform: Matrix,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            origin_x: 0.0,
            origin_y: 0.0,
            transform: matrix::create(),
        }
    }
}

impl Transform {
    pub fn need_local_transform(&self) -> bool {
        matrix::is_not_around_zero(self.rotation as f32)
            || matrix::is_not_around_zero(self.x as f32)
            || matrix::is_not_around_zero(self.y as f32)
            || matrix::is_not_around_zero((self.scale_x - 1.0) as f32)
            || matrix::is_not_around_zero((self.scale_y - 1.0) as f32)
    }

    pub fn get_local_transform(&self) -> Matrix {
        let mut out = matrix::create();
        let mut tmp = matrix::create();
        if self.origin_x != 0.0 || self.origin_y != 0.0 {
            matrix::translate(&mut tmp, &out, self.origin_x as f32, self.origin_y as f32);
            out = tmp;
        }
        if self.rotation != 0.0 {
            matrix::rotate(&mut tmp, &out, self.rotation as f32);
            out = tmp;
        }
        if self.scale_x != 1.0 || self.scale_y != 1.0 {
            matrix::scale(&mut tmp, &out, self.scale_x as f32, self.scale_y as f32);
            out = tmp;
        }
        if self.origin_x != 0.0 || self.origin_y != 0.0 {
            matrix::translate(
                &mut tmp,
                &out,
                -(self.origin_x as f32),
                -(self.origin_y as f32),
            );
            out = tmp;
        }
        if self.x != 0.0 || self.y != 0.0 {
            matrix::translate(&mut tmp, &out, self.x as f32, self.y as f32);
            out = tmp;
        }
        out
    }

    pub fn update_transform(&mut self, parent: Option<&Matrix>) {
        let need_local = self.need_local_transform();
        if !need_local && parent.is_none() {
            matrix::identity(&mut self.transform);
            return;
        }

        let local = if need_local {
            self.get_local_transform()
        } else {
            matrix::create()
        };

        if let Some(pt) = parent {
            matrix::mul(&mut self.transform, pt, &local);
        } else {
            self.transform = local;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_transform_applies_scale_and_translation() {
        let t = Transform {
            x: 10.0,
            y: 4.0,
            scale_x: 2.0,
            scale_y: 3.0,
            ..Default::default()
        };
        let m = t.get_local_transform();
        assert!((m[0] - 2.0).abs() < 1e-4);
        assert!((m[3] - 3.0).abs() < 1e-4);
        assert!((m[4] - 10.0).abs() < 1e-4);
        assert!((m[5] - 4.0).abs() < 1e-4);
    }

    #[test]
    fn need_local_transform_when_scaled() {
        let mut t = Transform::default();
        assert!(!t.need_local_transform());
        t.scale_x = 2.0;
        assert!(t.need_local_transform());
    }
}
