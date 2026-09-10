//! 单轴坐标系

use crate::coord::{scale_from_ratio, scale_ratio};
use crate::model::{AxisModel, AxisType, GlobalModel, SeriesModel, SingleAxisSpec};
use crate::option::OptionValue;

pub struct SingleCoord<'a> {
    pub spec: &'a SingleAxisSpec,
}

impl<'a> SingleCoord<'a> {
    pub fn new(model: &'a GlobalModel, index: usize) -> Option<Self> {
        model.single_axes.get(index).map(|spec| Self { spec })
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Option<Self> {
        Self::new(model, series.single_axis_index)
    }

    pub fn axis(&self) -> &AxisModel {
        &self.spec.axis
    }

    pub fn data_to_point(&self, value: f64) -> (f64, f64) {
        let t = axis_t(&self.spec.axis, value);
        let r = self.spec.rect;
        if self.spec.horizontal {
            (r.x + t * r.width, r.y + r.height * 0.5)
        } else {
            (r.x + r.width * 0.5, r.y + (1.0 - t) * r.height)
        }
    }

    pub fn point_for(&self, category_index: usize, x_value: Option<f64>, y_value: f64) -> (f64, f64) {
        let v = if self.spec.axis.axis_type.is_category() {
            category_index as f64
        } else {
            x_value.unwrap_or(y_value)
        };
        self.data_to_point(v)
    }

    pub fn point_to_data(&self, px: f64, py: f64) -> f64 {
        let r = self.spec.rect;
        let t = if self.spec.horizontal {
            if r.width <= 0.0 {
                0.0
            } else {
                ((px - r.x) / r.width).clamp(0.0, 1.0)
            }
        } else if r.height <= 0.0 {
            0.0
        } else {
            (1.0 - (py - r.y) / r.height).clamp(0.0, 1.0)
        };
        if self.spec.axis.axis_type.is_category() {
            (t * self.spec.axis.category_data.len().max(1) as f64).floor()
        } else {
            scale_from_ratio(&self.spec.axis, t)
        }
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        self.spec.rect.contains(x, y)
    }
}

fn axis_t(axis: &AxisModel, value: f64) -> f64 {
    if axis.axis_type.is_category() {
        let n = axis.category_data.len().max(1) as f64;
        let idx = if value.is_finite() {
            value.round().max(0.0)
        } else {
            0.0
        };
        (idx + 0.5) / n
    } else {
        scale_ratio(axis, value)
    }
}

pub fn resolve_axis_value(axis: &AxisModel, value: &OptionValue) -> f64 {
    if axis.axis_type.is_category() {
        if let Some(s) = value.as_str() {
            if let Some(idx) = axis.category_data.iter().position(|c| c == s) {
                return idx as f64;
            }
        }
        return value.as_f64().unwrap_or(0.0);
    }
    if axis.axis_type == AxisType::Time {
        return crate::data::numeric_or_time(value).unwrap_or(0.0);
    }
    crate::data::numeric_or_time(value)
        .or_else(|| value.as_f64())
        .unwrap_or(0.0)
}
