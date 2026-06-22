//! 直角坐标系：dataToPoint / 轴布局

use crate::model::{GlobalModel, GridRect};

pub struct Cartesian2D<'a> {
    model: &'a GlobalModel,
}

impl<'a> Cartesian2D<'a> {
    pub fn new(model: &'a GlobalModel) -> Self {
        Self { model }
    }

    pub fn grid(&self) -> GridRect {
        self.model.grid
    }

    /// category index + value → 像素坐标（category_index 为全局索引）
    pub fn data_to_point(&self, category_index: usize, value: f64) -> (f64, f64) {
        let grid = self.model.grid;
        let (start, end) = self.model.visible_category_range();
        let visible = (end - start).max(1);
        let local = category_index.saturating_sub(start);
        let x = grid.x + (local as f64 + 0.5) / visible as f64 * grid.width;

        let ymin = self.model.y_axis.value_min();
        let ymax = self.model.y_axis.value_max();
        let span = (ymax - ymin).max(f64::EPSILON);
        let ratio = (value - ymin) / span;
        let y = grid.y + grid.height * (1.0 - ratio);
        (x, y)
    }

    pub fn base_y(&self) -> f64 {
        let grid = self.model.grid;
        let ymin = self.model.y_axis.value_min();
        let ymax = self.model.y_axis.value_max();
        let span = (ymax - ymin).max(f64::EPSILON);
        let ratio = (0.0_f64 - ymin) / span;
        grid.y + grid.height * (1.0 - ratio.clamp(0.0, 1.0))
    }

    /// 双 value 轴坐标（散点图）
    pub fn value_to_point(&self, x_val: f64, y_val: f64) -> (f64, f64) {
        let grid = self.model.grid;
        let xmin = self.model.x_axis.value_min();
        let xmax = self.model.x_axis.value_max();
        let ymin = self.model.y_axis.value_min();
        let ymax = self.model.y_axis.value_max();
        let xspan = (xmax - xmin).max(f64::EPSILON);
        let yspan = (ymax - ymin).max(f64::EPSILON);
        let xr = ((x_val - xmin) / xspan).clamp(0.0, 1.0);
        let yr = ((y_val - ymin) / yspan).clamp(0.0, 1.0);
        let x = grid.x + xr * grid.width;
        let y = grid.y + grid.height * (1.0 - yr);
        (x, y)
    }

    pub fn category_band_width(&self) -> f64 {
        let (_, end) = self.model.visible_category_range();
        let (start, _) = self.model.visible_category_range();
        let visible = (end - start).max(1);
        self.model.grid.width / visible as f64
    }

    /// 像素 x → 最近的全局 category 索引
    pub fn point_to_category_index(&self, x: f64) -> Option<usize> {
        let grid = self.model.grid;
        if x < grid.x || x > grid.x + grid.width {
            return None;
        }
        let (start, end) = self.model.visible_category_range();
        let visible = (end - start).max(1);
        let rel = ((x - grid.x) / grid.width).clamp(0.0, 0.999_999);
        let local = (rel * visible as f64).floor() as usize;
        Some((start + local).min(self.model.category_count().saturating_sub(1)))
    }
}
