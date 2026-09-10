//! 雷达坐标系：indicator 轴 + dataToPoint(value, indicatorIndex)

use std::f64::consts::PI;

use crate::model::{GlobalModel, RadarSpec, SeriesModel};

pub struct RadarCoord<'a> {
    pub spec: &'a RadarSpec,
}

impl<'a> RadarCoord<'a> {
    pub fn new(model: &'a GlobalModel, index: usize) -> Option<Self> {
        model.radars.get(index).map(|spec| Self { spec })
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Option<Self> {
        Self::new(model, series.radar_index)
    }

    pub fn indicator_count(&self) -> usize {
        self.spec.indicators.len()
    }

    pub fn indicator_angle(&self, index: usize) -> f64 {
        let n = self.spec.indicators.len().max(1) as f64;
        let sign = if self.spec.clockwise { -1.0 } else { 1.0 };
        (self.spec.start_angle * PI / 180.0) + sign * index as f64 * PI * 2.0 / n
    }

    pub fn data_to_point(&self, value: f64, indicator_index: usize) -> (f64, f64) {
        let Some(ind) = self.spec.indicators.get(indicator_index) else {
            return (f64::NAN, f64::NAN);
        };
        let span = (ind.max - ind.min).abs().max(f64::EPSILON);
        let t = ((value - ind.min) / span).clamp(0.0, 1.0);
        let coord = self.spec.r0 + t * (self.spec.r - self.spec.r0);
        self.coord_to_point(coord, indicator_index)
    }

    pub fn coord_to_point(&self, radius: f64, indicator_index: usize) -> (f64, f64) {
        let angle = self.indicator_angle(indicator_index);
        (
            self.spec.center_x + radius * angle.cos(),
            self.spec.center_y - radius * angle.sin(),
        )
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        let dx = x - self.spec.center_x;
        let dy = y - self.spec.center_y;
        (dx * dx + dy * dy).sqrt() <= self.spec.r + 1e-4
    }
}
