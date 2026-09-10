//! 矩阵坐标系：x/y 类目格子中心

use crate::model::{GlobalModel, MatrixSpec, SeriesModel};
use crate::option::OptionValue;

pub struct MatrixCoord<'a> {
    pub spec: &'a MatrixSpec,
}

impl<'a> MatrixCoord<'a> {
    pub fn new(model: &'a GlobalModel, index: usize) -> Option<Self> {
        model.matrices.get(index).map(|spec| Self { spec })
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Option<Self> {
        Self::new(model, series.matrix_index)
    }

    pub fn cell_size(&self) -> (f64, f64) {
        let nx = self.spec.x_data.len().max(1) as f64;
        let ny = self.spec.y_data.len().max(1) as f64;
        (self.spec.rect.width / nx, self.spec.rect.height / ny)
    }

    pub fn data_to_point(&self, x_key: &str, y_key: &str) -> (f64, f64) {
        let xi = resolve_dim(&self.spec.x_data, x_key);
        let yi = resolve_dim(&self.spec.y_data, y_key);
        self.index_to_point(xi, yi)
    }

    pub fn index_to_point(&self, xi: usize, yi: usize) -> (f64, f64) {
        let (cw, ch) = self.cell_size();
        let r = self.spec.rect;
        (
            r.x + (xi as f64 + 0.5) * cw,
            r.y + (yi as f64 + 0.5) * ch,
        )
    }

    pub fn point_for(&self, category_index: usize, x_value: Option<f64>, y_value: f64) -> (f64, f64) {
        let xi = x_value
            .filter(|n| n.is_finite())
            .map(|n| n.round().max(0.0) as usize)
            .unwrap_or(category_index);
        let yi = if y_value.is_finite() {
            y_value.round().max(0.0) as usize
        } else {
            0
        };
        self.index_to_point(xi, yi)
    }

    pub fn point_to_data(&self, px: f64, py: f64) -> (f64, f64) {
        let (cw, ch) = self.cell_size();
        let r = self.spec.rect;
        let xi = ((px - r.x) / cw).floor().max(0.0);
        let yi = ((py - r.y) / ch).floor().max(0.0);
        (xi, yi)
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        self.spec.rect.contains(x, y)
    }

    pub fn resolve_keys(value: &OptionValue) -> (String, String) {
        match value {
            OptionValue::Array(arr) if arr.len() >= 2 => (
                crate::data::cell_text(&arr[0]),
                crate::data::cell_text(&arr[1]),
            ),
            _ => (String::new(), String::new()),
        }
    }
}

fn resolve_dim(data: &[String], key: &str) -> usize {
    if let Some(i) = data.iter().position(|s| s == key) {
        return i;
    }
    key.parse::<f64>()
        .ok()
        .map(|n| n.round().max(0.0) as usize)
        .unwrap_or(0)
        .min(data.len().saturating_sub(1))
}
