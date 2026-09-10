//! 平行坐标系：按 dim 把值投到对应轴上的点

use crate::coord::{scale_from_ratio, scale_ratio};
use crate::model::{GlobalModel, ParallelAxisSpec, ParallelSpec, SeriesModel};
use crate::option::OptionValue;

pub struct ParallelCoord<'a> {
    pub spec: &'a ParallelSpec,
}

impl<'a> ParallelCoord<'a> {
    pub fn new(model: &'a GlobalModel, index: usize) -> Option<Self> {
        model.parallels.get(index).map(|spec| Self { spec })
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Option<Self> {
        Self::new(model, series.parallel_index)
    }

    pub fn axis_at(&self, dim: usize) -> Option<&ParallelAxisSpec> {
        self.spec.axes.iter().find(|a| a.dim == dim).or_else(|| self.spec.axes.get(dim))
    }

    pub fn data_to_point(&self, value: f64, dim: usize) -> (f64, f64) {
        let n = self.spec.axes.len().max(1);
        let axis_i = self
            .spec
            .axes
            .iter()
            .position(|a| a.dim == dim)
            .unwrap_or(dim.min(n - 1));
        let t = if n <= 1 {
            0.5
        } else {
            axis_i as f64 / (n - 1) as f64
        };
        let r = self.spec.rect;
        let Some(axis) = self.spec.axes.get(axis_i) else {
            return (f64::NAN, f64::NAN);
        };
        let u = scale_ratio(&axis.axis, value);
        if self.spec.horizontal {
            (r.x + t * r.width, r.y + (1.0 - u) * r.height)
        } else {
            (r.x + u * r.width, r.y + t * r.height)
        }
    }

    pub fn point_to_data(&self, px: f64, py: f64, dim: usize) -> f64 {
        let r = self.spec.rect;
        let Some(axis) = self.axis_at(dim) else {
            return f64::NAN;
        };
        let u = if self.spec.horizontal {
            if r.height <= 0.0 {
                0.0
            } else {
                (1.0 - (py - r.y) / r.height).clamp(0.0, 1.0)
            }
        } else if r.width <= 0.0 {
            0.0
        } else {
            ((px - r.x) / r.width).clamp(0.0, 1.0)
        };
        scale_from_ratio(&axis.axis, u)
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        self.spec.rect.contains(x, y)
    }

    pub fn resolve_dim_value(&self, dim: usize, value: &OptionValue) -> f64 {
        if let Some(arr) = value.as_array() {
            return arr
                .get(dim)
                .and_then(|v| crate::data::numeric_or_time(v).or_else(|| v.as_f64()))
                .unwrap_or(0.0);
        }
        value.as_f64().unwrap_or(0.0)
    }
}
