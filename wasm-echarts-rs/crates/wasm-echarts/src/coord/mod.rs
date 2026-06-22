//! 直角坐标系：dataToPoint / 轴布局

use crate::model::{AxisType, GlobalModel, GridRect};

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

    /// category index + value → 像素坐标
    pub fn data_to_point(&self, category_index: usize, value: f64) -> (f64, f64) {
        let grid = self.model.grid;
        let n = self.model.category_count().max(1);
        let x = grid.x + (category_index as f64 + 0.5) / n as f64 * grid.width;

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

    pub fn category_band_width(&self) -> f64 {
        let n = self.model.category_count().max(1);
        self.model.grid.width / n as f64
    }

    pub fn axis_x_type(&self) -> AxisType {
        self.model.x_axis.axis_type
    }
}
