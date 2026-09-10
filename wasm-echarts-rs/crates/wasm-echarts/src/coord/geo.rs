//! 地理坐标系：lng/lat → 像素；区域来自 registerMap

use crate::model::{GeoRegion, GeoSpec, GlobalModel, SeriesModel};
use crate::option::OptionValue;

pub struct GeoCoord<'a> {
    pub spec: &'a GeoSpec,
}

impl<'a> GeoCoord<'a> {
    pub fn new(model: &'a GlobalModel, index: usize) -> Option<Self> {
        model.geos.get(index).map(|spec| Self { spec })
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Option<Self> {
        Self::new(model, series.geo_index)
    }

    fn raw_bbox(&self) -> Option<(f64, f64, f64, f64)> {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for region in &self.spec.regions {
            for poly in &region.polygons {
                for &(lng, lat) in poly {
                    let (x, y) = self.project_raw(lng, lat);
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }
        if min_x.is_finite() {
            Some((min_x, min_y, max_x, max_y))
        } else {
            None
        }
    }

    fn project_raw(&self, lng: f64, lat: f64) -> (f64, f64) {
        let x = if self.spec.invert_lng { -lng } else { lng };
        (x, lat * self.spec.aspect_scale)
    }

    pub fn data_to_point(&self, lng: f64, lat: f64) -> (f64, f64) {
        let Some((min_x, min_y, max_x, max_y)) = self.raw_bbox() else {
            return (self.spec.rect.x + self.spec.rect.width * 0.5, self.spec.rect.y + self.spec.rect.height * 0.5);
        };
        let (rx, ry) = self.project_raw(lng, lat);
        let r = self.spec.rect;
        let sx = (max_x - min_x).abs().max(f64::EPSILON);
        let sy = (max_y - min_y).abs().max(f64::EPSILON);
        let scale = (r.width / sx).min(r.height / sy);
        let cx = r.x + r.width * 0.5;
        let cy = r.y + r.height * 0.5;
        let mx = (min_x + max_x) * 0.5;
        let my = (min_y + max_y) * 0.5;
        (cx + (rx - mx) * scale, cy - (ry - my) * scale)
    }

    pub fn point_to_data(&self, px: f64, py: f64) -> (f64, f64) {
        let Some((min_x, min_y, max_x, max_y)) = self.raw_bbox() else {
            return (0.0, 0.0);
        };
        let r = self.spec.rect;
        let sx = (max_x - min_x).abs().max(f64::EPSILON);
        let sy = (max_y - min_y).abs().max(f64::EPSILON);
        let scale = (r.width / sx).min(r.height / sy).max(f64::EPSILON);
        let cx = r.x + r.width * 0.5;
        let cy = r.y + r.height * 0.5;
        let mx = (min_x + max_x) * 0.5;
        let my = (min_y + max_y) * 0.5;
        let rx = mx + (px - cx) / scale;
        let ry = my - (py - cy) / scale;
        let lng = if self.spec.invert_lng { -rx } else { rx };
        let lat = ry / self.spec.aspect_scale.max(f64::EPSILON);
        (lng, lat)
    }

    pub fn point_for(&self, _category_index: usize, x_value: Option<f64>, y_value: f64) -> (f64, f64) {
        self.data_to_point(x_value.unwrap_or(0.0), y_value)
    }

    pub fn named_point(&self, name: &str) -> Option<(f64, f64)> {
        let region = self.spec.regions.iter().find(|r| r.name == name)?;
        if let Some((lng, lat)) = region.center {
            return Some(self.data_to_point(lng, lat));
        }
        let poly = region.polygons.first()?;
        let (lng, lat) = centroid(poly)?;
        Some(self.data_to_point(lng, lat))
    }

    pub fn contain(&self, x: f64, y: f64) -> bool {
        self.spec.rect.contains(x, y)
    }

    pub fn regions(&self) -> &[GeoRegion] {
        &self.spec.regions
    }

    pub fn project_polygon(&self, poly: &[(f64, f64)]) -> Vec<(f64, f64)> {
        poly.iter().map(|&(lng, lat)| self.data_to_point(lng, lat)).collect()
    }

    pub fn resolve_lng_lat(value: &OptionValue) -> Option<(f64, f64)> {
        match value {
            OptionValue::Array(arr) if arr.len() >= 2 => Some((
                crate::data::numeric_or_time(&arr[0]).or_else(|| arr[0].as_f64())?,
                crate::data::numeric_or_time(&arr[1]).or_else(|| arr[1].as_f64())?,
            )),
            _ => None,
        }
    }
}

fn centroid(poly: &[(f64, f64)]) -> Option<(f64, f64)> {
    if poly.is_empty() {
        return None;
    }
    let n = poly.len() as f64;
    Some((
        poly.iter().map(|p| p.0).sum::<f64>() / n,
        poly.iter().map(|p| p.1).sum::<f64>() / n,
    ))
}
