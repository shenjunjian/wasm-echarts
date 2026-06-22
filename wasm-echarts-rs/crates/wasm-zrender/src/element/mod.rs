//! Element 基类：变换、dirty bit

pub mod constants;
pub mod transform;

pub use constants::{REDRAW_BIT, SHAPE_CHANGED_BIT, STYLE_CHANGED_BIT};
pub use transform::Transform;

use crate::core::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct ElementBase {
    pub transform_state: Transform,
    pub dirty: u32,
    pub ignore: bool,
    pub name: String,
}

impl Default for ElementBase {
    fn default() -> Self {
        Self {
            transform_state: Transform::default(),
            dirty: REDRAW_BIT,
            ignore: false,
            name: String::new(),
        }
    }
}

impl ElementBase {
    pub fn transform(&self) -> &Matrix {
        &self.transform_state.transform
    }

    pub fn mark_redraw(&mut self) {
        self.dirty |= REDRAW_BIT;
    }

    pub fn mark_shape_dirty(&mut self) {
        self.dirty |= REDRAW_BIT | SHAPE_CHANGED_BIT;
    }

    pub fn mark_style_dirty(&mut self) {
        self.dirty |= REDRAW_BIT | STYLE_CHANGED_BIT;
    }

    pub fn update_transform(&mut self, parent: Option<&Matrix>) {
        self.transform_state.update_transform(parent);
    }
}
