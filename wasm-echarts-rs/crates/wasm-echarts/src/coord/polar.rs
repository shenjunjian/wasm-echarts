//! 极坐标系：angleAxis + radiusAxis；data 维顺序为 [radius, angle]

use std::f64::consts::PI;

use crate::coord::{scale_from_ratio, scale_ratio};
use crate::model::{polar_data_pair, AxisModel, AxisType, GlobalModel, PolarSpec, SeriesModel};
use crate::option::OptionValue;

pub struct PolarCoord<'a> {
    pub spec: &'a PolarSpec,
}

impl<'a> PolarCoord<'a> {
    pub fn new(model: &'a GlobalModel, index: usize) -> Option<Self> {
        model.polars.get(index).map(|spec| Self { spec })
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Option<Self> {
        Self::new(model, series.polar_index)
    }

    pub fn angle_axis(&self) -> &AxisModel {
        &self.spec.angle_axis
    }

    pub fn radius_axis(&self) -> &AxisModel {
        &self.spec.radius_axis
    }

    pub fn data_to_point(&self, radius_val: f64, angle_val: f64) -> (f64, f64) {
        let r = self.radius_to_pixel(radius_val);
        let deg = self.angle_to_degree(angle_val);
        self.coord_to_point(r, deg)
    }

    pub fn point_for(&self, category_index: usize, x_value: Option<f64>, y_value: f64) -> (f64, f64) {
        let dummy = crate::model::DataPoint {
            value: y_value,
            x_value,
            name: None,
            raw_index: category_index,
            raw: OptionValue::Null,
            stack_base: 0.0,
            stacked_value: y_value,
        };
        let (radius, angle) = polar_data_pair(
            &dummy,
            category_index,
            self.spec.angle_axis.axis_type.is_category(),
        );
        self.data_to_point(radius, angle)
    }

    pub fn area_base(&self, category_index: usize, x_value: Option<f64>, y_value: f64, origin: Option<&OptionValue>) -> (f64, f64) {
        let dummy = crate::model::DataPoint {
            value: y_value,
            x_value,
            name: None,
            raw_index: category_index,
            raw: OptionValue::Null,
            stack_base: 0.0,
            stacked_value: y_value,
        };
        let (_, angle) = polar_data_pair(
            &dummy,
            category_index,
            self.spec.angle_axis.axis_type.is_category(),
        );
        let r_val = match origin {
            Some(OptionValue::String(s)) if s == "start" => self.spec.radius_axis.value_min(),
            Some(OptionValue::String(s)) if s == "end" => self.spec.radius_axis.value_max(),
            Some(OptionValue::Number(n)) if n.is_finite() => *n,
            _ => self.spec.radius_axis.value_min(),
        };
        self.data_to_point(r_val, angle)
    }

    pub fn coord_to_point(&self, radius_px: f64, angle_deg: f64) -> (f64, f64) {
        let rad = angle_deg * PI / 180.0;
        (
            self.spec.center_x + radius_px * rad.cos(),
            self.spec.center_y - radius_px * rad.sin(),
        )
    }

    pub fn radius_to_pixel(&self, value: f64) -> f64 {
        let t = if self.spec.radius_axis.axis_type.is_category() {
            category_t(&self.spec.radius_axis, value)
        } else {
            scale_ratio(&self.spec.radius_axis, value)
        };
        self.spec.r0 + t * (self.spec.r - self.spec.r0)
    }

    pub fn pixel_to_radius(&self, px: f64) -> f64 {
        let span = (self.spec.r - self.spec.r0).abs().max(f64::EPSILON);
        let t = ((px - self.spec.r0) / span).clamp(0.0, 1.0);
        if self.spec.radius_axis.axis_type.is_category() {
            (t * self.spec.radius_axis.category_data.len().max(1) as f64).floor()
        } else {
            scale_from_ratio(&self.spec.radius_axis, t)
        }
    }

    pub fn angle_to_degree(&self, value: f64) -> f64 {
        let t = if self.spec.angle_axis.axis_type.is_category() {
            category_t(&self.spec.angle_axis, value)
        } else {
            scale_ratio(&self.spec.angle_axis, value)
        };
        let span = if self.spec.clockwise { -360.0 } else { 360.0 };
        self.spec.start_angle + t * span
    }

    pub fn degree_to_angle(&self, deg: f64) -> f64 {
        let span = if self.spec.clockwise { -360.0 } else { 360.0 };
        let mut t = (deg - self.spec.start_angle) / span;
        t -= t.floor();
        if self.spec.angle_axis.axis_type.is_category() {
            (t * self.spec.angle_axis.category_data.len().max(1) as f64).floor()
        } else {
            scale_from_ratio(&self.spec.angle_axis, t)
        }
    }

    pub fn point_to_data(&self, px: f64, py: f64) -> (f64, f64) {
        let dx = px - self.spec.center_x;
        let dy = py - self.spec.center_y;
        let radius_px = (dx * dx + dy * dy).sqrt();
        let deg = (-dy).atan2(dx).to_degrees();
        let radius = self.pixel_to_radius(radius_px);
        let angle = self.degree_to_angle(deg);
        (radius, angle)
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        let dx = x - self.spec.center_x;
        let dy = y - self.spec.center_y;
        let d = (dx * dx + dy * dy).sqrt();
        d >= self.spec.r0 - 1e-4 && d <= self.spec.r + 1e-4
    }

    pub fn angle_band(&self) -> f64 {
        let n = if self.spec.angle_axis.axis_type.is_category() {
            self.spec.angle_axis.category_data.len().max(1)
        } else {
            12
        };
        360.0 / n as f64
    }
}

fn category_t(axis: &AxisModel, value: f64) -> f64 {
    let n = axis.category_data.len().max(1) as f64;
    let idx = if value.is_finite() {
        value.round().max(0.0)
    } else {
        0.0
    };
    (idx + 0.5) / n
}

pub fn resolve_polar_value(axis: &AxisModel, value: &OptionValue) -> f64 {
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
