//! 日历坐标系：日期 → 格子中心

use crate::data::{numeric_or_time, weekday_sun0};
use crate::model::{CalendarSpec, GlobalModel, SeriesModel};
use crate::option::OptionValue;

pub struct CalendarCoord<'a> {
    pub spec: &'a CalendarSpec,
}

impl<'a> CalendarCoord<'a> {
    pub fn new(model: &'a GlobalModel, index: usize) -> Option<Self> {
        model.calendars.get(index).map(|spec| Self { spec })
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Option<Self> {
        Self::new(model, series.calendar_index)
    }

    pub fn data_to_point(&self, time_ms: f64) -> (f64, f64) {
        if !time_ms.is_finite() {
            return (f64::NAN, f64::NAN);
        }
        let start_wd = weekday_sun0(self.spec.start_ms);
        let day_index = ((time_ms - self.spec.start_ms) / 86_400_000.0).floor();
        if day_index < -0.5 {
            return (f64::NAN, f64::NAN);
        }
        let loc = (start_wd - self.spec.first_day).rem_euclid(7) as f64 + day_index;
        let week = (loc / 7.0).floor();
        let weekday = loc.rem_euclid(7.0);
        let r = self.spec.rect;
        if self.spec.horizontal {
            (
                r.x + (week + 0.5) * self.spec.cell_w,
                r.y + (weekday + 0.5) * self.spec.cell_h,
            )
        } else {
            (
                r.x + (weekday + 0.5) * self.spec.cell_w,
                r.y + (week + 0.5) * self.spec.cell_h,
            )
        }
    }

    pub fn point_for(&self, _category_index: usize, x_value: Option<f64>, y_value: f64) -> (f64, f64) {
        self.data_to_point(x_value.unwrap_or(y_value))
    }

    pub fn point_to_data(&self, px: f64, py: f64) -> f64 {
        let r = self.spec.rect;
        let (week, weekday) = if self.spec.horizontal {
            (
                ((px - r.x) / self.spec.cell_w).floor(),
                ((py - r.y) / self.spec.cell_h).floor(),
            )
        } else {
            (
                ((py - r.y) / self.spec.cell_h).floor(),
                ((px - r.x) / self.spec.cell_w).floor(),
            )
        };
        let start_wd = weekday_sun0(self.spec.start_ms);
        let offset = (start_wd - self.spec.first_day).rem_euclid(7) as f64;
        let day_index = week * 7.0 + weekday - offset;
        self.spec.start_ms + day_index * 86_400_000.0
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        self.spec.rect.contains(x, y)
    }

    pub fn resolve_time(value: &OptionValue) -> f64 {
        if let Some(arr) = value.as_array() {
            return arr
                .first()
                .and_then(numeric_or_time)
                .unwrap_or(0.0);
        }
        numeric_or_time(value).or_else(|| value.as_f64()).unwrap_or(0.0)
    }
}
