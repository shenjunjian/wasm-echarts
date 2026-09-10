//! 坐标系：cartesian + polar/radar/single/parallel/calendar/matrix/geo

mod calendar;
mod geo;
mod matrix;
mod parallel;
mod polar;
mod radar;
mod single;

pub use calendar::CalendarCoord;
pub use geo::GeoCoord;
pub use matrix::MatrixCoord;
pub use parallel::ParallelCoord;
pub use polar::PolarCoord;
pub use radar::RadarCoord;
pub use single::SingleCoord;

use crate::data::{format_time_label, numeric_or_time};
use crate::model::{AxisModel, AxisType, CoordSysKind, GlobalModel, GridRect, SeriesModel};
use crate::option::OptionValue;

pub struct Cartesian2D<'a> {
    model: &'a GlobalModel,
    grid: GridRect,
    x_axis: &'a AxisModel,
    y_axis: &'a AxisModel,
    x_index: usize,
    y_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertTarget {
    Cartesian,
    XAxis,
    YAxis,
    Polar,
    AngleAxis,
    RadiusAxis,
    Radar,
    Single,
    Parallel,
    Calendar,
    Matrix,
    Geo,
}

pub enum SeriesCoord<'a> {
    Cartesian(Cartesian2D<'a>),
    Polar(PolarCoord<'a>),
    Single(SingleCoord<'a>),
    Calendar(CalendarCoord<'a>),
    Matrix(MatrixCoord<'a>),
    Geo(GeoCoord<'a>),
}

impl<'a> SeriesCoord<'a> {
    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Self {
        match series.coord_sys {
            CoordSysKind::Polar => PolarCoord::for_series(model, series)
                .map(SeriesCoord::Polar)
                .unwrap_or_else(|| SeriesCoord::Cartesian(Cartesian2D::for_series(model, series))),
            CoordSysKind::Single => SingleCoord::for_series(model, series)
                .map(SeriesCoord::Single)
                .unwrap_or_else(|| SeriesCoord::Cartesian(Cartesian2D::for_series(model, series))),
            CoordSysKind::Calendar => CalendarCoord::for_series(model, series)
                .map(SeriesCoord::Calendar)
                .unwrap_or_else(|| SeriesCoord::Cartesian(Cartesian2D::for_series(model, series))),
            CoordSysKind::Matrix => MatrixCoord::for_series(model, series)
                .map(SeriesCoord::Matrix)
                .unwrap_or_else(|| SeriesCoord::Cartesian(Cartesian2D::for_series(model, series))),
            CoordSysKind::Geo => GeoCoord::for_series(model, series)
                .map(SeriesCoord::Geo)
                .unwrap_or_else(|| SeriesCoord::Cartesian(Cartesian2D::for_series(model, series))),
            _ => SeriesCoord::Cartesian(Cartesian2D::for_series(model, series)),
        }
    }

    pub fn point_for(&self, category_index: usize, x_value: Option<f64>, y_value: f64) -> (f64, f64) {
        match self {
            SeriesCoord::Cartesian(c) => c.point_for(category_index, x_value, y_value),
            SeriesCoord::Polar(c) => c.point_for(category_index, x_value, y_value),
            SeriesCoord::Single(c) => c.point_for(category_index, x_value, y_value),
            SeriesCoord::Calendar(c) => c.point_for(category_index, x_value, y_value),
            SeriesCoord::Matrix(c) => c.point_for(category_index, x_value, y_value),
            SeriesCoord::Geo(c) => c.point_for(category_index, x_value, y_value),
        }
    }

    pub fn map_point(&self, point: &crate::model::DataPoint, index: usize) -> (f64, f64) {
        match self {
            SeriesCoord::Geo(c) => {
                if let Some((lng, lat)) = point
                    .raw
                    .as_array()
                    .filter(|a| a.len() >= 2)
                    .and_then(|a| GeoCoord::resolve_lng_lat(&OptionValue::Array(a.to_vec())))
                {
                    return c.data_to_point(lng, lat);
                }
                if let Some(name) = point.name.as_deref() {
                    if let Some(pt) = c.named_point(name) {
                        return pt;
                    }
                }
                c.point_for(index, point.x_value, point.value)
            }
            SeriesCoord::Matrix(c) => {
                if let Some(arr) = point.raw.as_array() {
                    if arr.len() >= 2 {
                        let xk = crate::data::cell_text(&arr[0]);
                        let yk = crate::data::cell_text(&arr[1]);
                        return c.data_to_point(&xk, &yk);
                    }
                }
                c.point_for(index, point.x_value, point.value)
            }
            SeriesCoord::Calendar(c) => {
                if let Some(arr) = point.raw.as_array() {
                    if let Some(t) = arr.first().and_then(numeric_or_time) {
                        return c.data_to_point(t);
                    }
                }
                if let Some(t) = point.x_value {
                    return c.data_to_point(t);
                }
                c.point_for(index, point.x_value, point.value)
            }
            _ => self.point_for(index, point.x_value, point.value),
        }
    }

    pub fn area_base(
        &self,
        category_index: usize,
        x_value: Option<f64>,
        origin: Option<&OptionValue>,
        cartesian_y: f64,
    ) -> (f64, f64) {
        match self {
            SeriesCoord::Polar(c) => c.area_base(category_index, x_value, 0.0, origin),
            SeriesCoord::Cartesian(c) => {
                let (px, _) = c.point_for(category_index, x_value, 0.0);
                (px, cartesian_y)
            }
            other => {
                let (px, py) = other.point_for(category_index, x_value, 0.0);
                (px, py)
            }
        }
    }

    pub fn as_cartesian(&self) -> Option<&Cartesian2D<'a>> {
        match self {
            SeriesCoord::Cartesian(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_polar(&self) -> Option<&PolarCoord<'a>> {
        match self {
            SeriesCoord::Polar(c) => Some(c),
            _ => None,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.as_cartesian().map(|c| c.is_horizontal()).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConvertResult {
    Scalar(f64),
    Point(f64, f64),
}

#[derive(Debug, Clone, Copy)]
struct FinderRef {
    target: ConvertTarget,
    series: Option<usize>,
    grid: Option<usize>,
    x_axis: Option<usize>,
    y_axis: Option<usize>,
    polar: Option<usize>,
    radar: Option<usize>,
    geo: Option<usize>,
    calendar: Option<usize>,
    single: Option<usize>,
    parallel: Option<usize>,
    matrix: Option<usize>,
    parallel_axis: Option<usize>,
}

impl<'a> Cartesian2D<'a> {
    pub fn new(model: &'a GlobalModel) -> Self {
        Self::for_axes(model, 0, 0)
    }

    pub fn for_series(model: &'a GlobalModel, series: &SeriesModel) -> Self {
        Self::for_axes(model, series.x_axis_index, series.y_axis_index)
    }

    pub fn for_axes(model: &'a GlobalModel, x_index: usize, y_index: usize) -> Self {
        let x_axis = model.x_axis_at(x_index);
        let y_axis = model.y_axis_at(y_index);
        let grid = model.grid_at(x_axis.grid_index);
        Self {
            model,
            grid,
            x_axis,
            y_axis,
            x_index,
            y_index,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.y_axis.axis_type.is_category() && !self.x_axis.axis_type.is_category()
    }

    pub fn grid(&self) -> GridRect {
        self.grid
    }

    pub fn x_axis(&self) -> &'a AxisModel {
        self.x_axis
    }

    pub fn y_axis(&self) -> &'a AxisModel {
        self.y_axis
    }

    pub fn data_to_point(&self, category_index: usize, value: f64) -> (f64, f64) {
        let x = self.x_value_to_pixel(category_index as f64);
        let y = self.y_value_to_pixel(value);
        (x, y)
    }

    pub fn point_for(&self, category_index: usize, x_value: Option<f64>, y_value: f64) -> (f64, f64) {
        if self.is_horizontal() {
            let x = self.x_value_to_pixel(y_value);
            let y = self.y_value_to_pixel(category_index as f64);
            return (x, y);
        }
        if self.x_axis.axis_type.is_category() {
            self.data_to_point(category_index, y_value)
        } else {
            self.value_to_point(x_value.unwrap_or(category_index as f64), y_value)
        }
    }

    pub fn base_y(&self) -> f64 {
        self.y_value_to_pixel(0.0)
            .clamp(self.grid.y, self.grid.y + self.grid.height)
    }

    pub fn value_to_point(&self, x_val: f64, y_val: f64) -> (f64, f64) {
        let px = self.x_value_to_pixel(x_val).clamp(self.grid.x, self.grid.x + self.grid.width);
        let py = self.y_value_to_pixel(y_val).clamp(self.grid.y, self.grid.y + self.grid.height);
        (px, py)
    }

    pub fn category_band_width(&self) -> f64 {
        if self.is_horizontal() {
            let n = self.y_axis.category_data.len().max(1);
            return self.grid.height / n as f64;
        }
        let (start, end) = self.model.visible_category_range_of(self.x_index);
        let visible = (end - start).max(1);
        self.grid.width / visible as f64
    }

    pub fn point_to_category_index(&self, x: f64) -> Option<usize> {
        if x < self.grid.x || x > self.grid.x + self.grid.width {
            return None;
        }
        let (start, end) = self.model.visible_category_range_of(self.x_index);
        let visible = (end - start).max(1);
        let rel = ((x - self.grid.x) / self.grid.width).clamp(0.0, 0.999_999);
        let local = (rel * visible as f64).floor() as usize;
        Some((start + local).min(self.model.category_count_of(self.x_index).saturating_sub(1)))
    }

    pub fn x_value_to_pixel(&self, x_val: f64) -> f64 {
        axis_value_to_pixel(self.x_axis, x_val, self.grid.x, self.grid.width, false, self.model, self.x_index)
    }

    pub fn y_value_to_pixel(&self, y_val: f64) -> f64 {
        axis_value_to_pixel(
            self.y_axis,
            y_val,
            self.grid.y,
            self.grid.height,
            true,
            self.model,
            self.y_index,
        )
    }

    pub fn base_x(&self) -> f64 {
        self.x_value_to_pixel(0.0)
            .clamp(self.grid.x, self.grid.x + self.grid.width)
    }

    pub fn pixel_to_x_value(&self, px: f64) -> f64 {
        if self.x_axis.axis_type.is_category() {
            return self.point_to_category_index(px).unwrap_or(0) as f64;
        }
        pixel_to_axis_value(self.x_axis, px, self.grid.x, self.grid.width, false)
    }

    pub fn pixel_to_y_value(&self, py: f64) -> f64 {
        pixel_to_axis_value(self.y_axis, py, self.grid.y, self.grid.height, true)
    }

    pub fn point_to_data(&self, px: f64, py: f64) -> (f64, f64) {
        (self.pixel_to_x_value(px), self.pixel_to_y_value(py))
    }
}

fn axis_value_to_pixel(
    axis: &AxisModel,
    value: f64,
    origin: f64,
    size: f64,
    invert: bool,
    model: &GlobalModel,
    x_index: usize,
) -> f64 {
        if axis.axis_type.is_category() {
        let (start, end) = if invert {
            let n = axis.category_data.len().max(1);
            (0, n)
        } else {
            model.visible_category_range_of(x_index)
        };
        let visible = (end - start).max(1);
        let idx = if value.is_finite() {
            value.round().max(0.0) as usize
        } else {
            0
        };
        let local = idx.saturating_sub(start);
        let mut t = (local as f64 + 0.5) / visible as f64;
        if invert {
            t = 1.0 - t;
        }
        return origin + t * size;
    }
    let t = scale_ratio(axis, value);
    let t = if invert { 1.0 - t } else { t };
    origin + t * size
}

fn pixel_to_axis_value(axis: &AxisModel, px: f64, origin: f64, size: f64, invert: bool) -> f64 {
    if size <= 0.0 {
        return axis.value_min();
    }
    let mut t = ((px - origin) / size).clamp(0.0, 1.0);
    if invert {
        t = 1.0 - t;
    }
    scale_from_ratio(axis, t)
}

pub(crate) fn scale_ratio(axis: &AxisModel, value: f64) -> f64 {
    match axis.axis_type {
        AxisType::Log => {
            let base = axis.log_base.max(1.000_000_1);
            let min = axis.value_min().max(f64::MIN_POSITIVE);
            let max = axis.value_max().max(min * base);
            let v = value.max(f64::MIN_POSITIVE);
            let lmin = min.log(base);
            let lmax = max.log(base);
            let span = (lmax - lmin).abs().max(f64::EPSILON);
            ((v.log(base) - lmin) / span).clamp(0.0, 1.0)
        }
        _ => {
            let min = axis.value_min();
            let max = axis.value_max();
            let span = (max - min).abs().max(f64::EPSILON);
            ((value - min) / span).clamp(0.0, 1.0)
        }
    }
}

pub(crate) fn scale_from_ratio(axis: &AxisModel, t: f64) -> f64 {
    match axis.axis_type {
        AxisType::Log => {
            let base = axis.log_base.max(1.000_000_1);
            let min = axis.value_min().max(f64::MIN_POSITIVE);
            let max = axis.value_max().max(min * base);
            let lmin = min.log(base);
            let lmax = max.log(base);
            base.powf(lmin + t * (lmax - lmin))
        }
        _ => {
            let min = axis.value_min();
            let max = axis.value_max();
            min + t * (max - min)
        }
    }
}

fn parse_finder(finder: &OptionValue) -> FinderRef {
    if let Some(s) = finder.as_str() {
        return FinderRef {
            target: match s {
                "xAxis" => ConvertTarget::XAxis,
                "yAxis" => ConvertTarget::YAxis,
                "polar" => ConvertTarget::Polar,
                "angleAxis" => ConvertTarget::AngleAxis,
                "radiusAxis" => ConvertTarget::RadiusAxis,
                "radar" => ConvertTarget::Radar,
                "singleAxis" | "single" => ConvertTarget::Single,
                "parallel" => ConvertTarget::Parallel,
                "calendar" => ConvertTarget::Calendar,
                "matrix" => ConvertTarget::Matrix,
                "geo" => ConvertTarget::Geo,
                _ => ConvertTarget::Cartesian,
            },
            series: None,
            grid: None,
            x_axis: None,
            y_axis: None,
            polar: None,
            radar: None,
            geo: None,
            calendar: None,
            single: None,
            parallel: None,
            matrix: None,
            parallel_axis: None,
        };
    }
    let series = finder_index(finder, &["seriesIndex", "series"]);
    let grid = finder_index(finder, &["gridIndex", "grid"]);
    let x_axis = finder_index(finder, &["xAxisIndex", "xAxis"]);
    let y_axis = finder_index(finder, &["yAxisIndex", "yAxis"]);
    let polar = finder_index(finder, &["polarIndex", "polar"]);
    let radar = finder_index(finder, &["radarIndex", "radar"]);
    let geo = finder_index(finder, &["geoIndex", "geo"]);
    let calendar = finder_index(finder, &["calendarIndex", "calendar"]);
    let single = finder_index(finder, &["singleAxisIndex", "singleAxis"]);
    let parallel = finder_index(finder, &["parallelIndex", "parallel"]);
    let matrix = finder_index(finder, &["matrixIndex", "matrix"]);
    let parallel_axis = finder_index(finder, &["parallelAxisIndex", "parallelAxis"]);
    let target = if polar.is_some() || finder.get("angleAxisIndex").is_some() || finder.get("radiusAxisIndex").is_some() {
        if finder.get("angleAxisIndex").is_some() || finder.get("angleAxis").is_some() {
            ConvertTarget::AngleAxis
        } else if finder.get("radiusAxisIndex").is_some() || finder.get("radiusAxis").is_some() {
            ConvertTarget::RadiusAxis
        } else {
            ConvertTarget::Polar
        }
    } else if radar.is_some() {
        ConvertTarget::Radar
    } else if geo.is_some() {
        ConvertTarget::Geo
    } else if calendar.is_some() {
        ConvertTarget::Calendar
    } else if single.is_some() {
        ConvertTarget::Single
    } else if parallel.is_some() || parallel_axis.is_some() {
        ConvertTarget::Parallel
    } else if matrix.is_some() {
        ConvertTarget::Matrix
    } else if series.is_some() || grid.is_some() || (x_axis.is_some() && y_axis.is_some()) {
        ConvertTarget::Cartesian
    } else if x_axis.is_some() {
        ConvertTarget::XAxis
    } else if y_axis.is_some() {
        ConvertTarget::YAxis
    } else {
        ConvertTarget::Cartesian
    };
    FinderRef {
        target,
        series,
        grid,
        x_axis,
        y_axis,
        polar,
        radar,
        geo,
        calendar,
        single,
        parallel,
        matrix,
        parallel_axis,
    }
}

fn finder_index(finder: &OptionValue, keys: &[&str]) -> Option<usize> {
    for key in keys {
        if let Some(n) = finder.get(key).and_then(|v| v.as_f64()) {
            return Some(n.max(0.0) as usize);
        }
    }
    None
}

fn resolve_cartesian<'a>(model: &'a GlobalModel, finder: FinderRef) -> Cartesian2D<'a> {
    if let Some(si) = finder.series {
        if let Some(series) = model.series.get(si) {
            return Cartesian2D::for_series(model, series);
        }
    }
    let mut x_idx = finder.x_axis.unwrap_or(0);
    let mut y_idx = finder.y_axis.unwrap_or(0);
    if let Some(gi) = finder.grid {
        if finder.x_axis.is_none() {
            x_idx = model
                .x_axes
                .iter()
                .position(|a| a.grid_index == gi)
                .unwrap_or(0);
        }
        if finder.y_axis.is_none() {
            y_idx = model
                .y_axes
                .iter()
                .position(|a| a.grid_index == gi)
                .unwrap_or(0);
        }
    }
    Cartesian2D::for_axes(model, x_idx, y_idx)
}

fn resolve_x_data(axis: &AxisModel, value: &OptionValue) -> f64 {
    if axis.axis_type.is_category() {
        if let Some(s) = value.as_str() {
            if let Some(idx) = axis.category_data.iter().position(|c| c == s) {
                return idx as f64;
            }
        }
        return value.as_f64().unwrap_or(0.0);
    }
    if axis.axis_type == AxisType::Time {
        return numeric_or_time(value).unwrap_or(0.0);
    }
    numeric_or_time(value).or_else(|| value.as_f64()).unwrap_or(0.0)
}

fn resolve_y_data(axis: &AxisModel, value: &OptionValue) -> f64 {
    if axis.axis_type == AxisType::Time {
        return numeric_or_time(value).unwrap_or(0.0);
    }
    numeric_or_time(value).or_else(|| value.as_f64()).unwrap_or(0.0)
}

pub fn parse_convert_finder(finder: &OptionValue) -> ConvertTarget {
    parse_finder(finder).target
}

pub fn convert_to_pixel(
    model: &GlobalModel,
    finder: &OptionValue,
    value: &OptionValue,
) -> Option<ConvertResult> {
    let finder = parse_finder(finder);
    let finder = infer_finder(model, finder);
    match finder.target {
        ConvertTarget::Polar | ConvertTarget::AngleAxis | ConvertTarget::RadiusAxis => {
            convert_polar_to_pixel(model, finder, value)
        }
        ConvertTarget::Radar => convert_radar_to_pixel(model, finder, value),
        ConvertTarget::Geo => convert_geo_to_pixel(model, finder, value),
        ConvertTarget::Calendar => {
            let idx = finder.calendar.unwrap_or(0);
            let coord = CalendarCoord::new(model, idx)?;
            let (px, py) = coord.data_to_point(CalendarCoord::resolve_time(value));
            Some(ConvertResult::Point(px, py))
        }
        ConvertTarget::Single => {
            let idx = finder.single.or(finder.series).unwrap_or(0);
            let coord = SingleCoord::new(model, idx)?;
            let v = single::resolve_axis_value(coord.axis(), value);
            let (px, py) = coord.data_to_point(v);
            Some(ConvertResult::Point(px, py))
        }
        ConvertTarget::Parallel => {
            let idx = finder.parallel.unwrap_or(0);
            let coord = ParallelCoord::new(model, idx)?;
            let dim = finder.parallel_axis.unwrap_or(0);
            let v = coord.resolve_dim_value(dim, value);
            let (px, py) = coord.data_to_point(v, dim);
            Some(ConvertResult::Point(px, py))
        }
        ConvertTarget::Matrix => {
            let idx = finder.matrix.unwrap_or(0);
            let coord = MatrixCoord::new(model, idx)?;
            let (xk, yk) = MatrixCoord::resolve_keys(value);
            let (px, py) = coord.data_to_point(&xk, &yk);
            Some(ConvertResult::Point(px, py))
        }
        ConvertTarget::XAxis | ConvertTarget::YAxis | ConvertTarget::Cartesian => {
            if !model.has_cartesian_series() && model.grids.is_empty() {
                return None;
            }
            convert_cartesian_to_pixel(model, finder, value)
        }
    }
}

fn infer_finder(model: &GlobalModel, finder: FinderRef) -> FinderRef {
    if finder.target != ConvertTarget::Cartesian
        || finder.series.is_some()
        || finder.grid.is_some()
        || finder.x_axis.is_some()
        || finder.y_axis.is_some()
    {
        return finder;
    }
    if let Some(si) = finder.series {
        if let Some(s) = model.series.get(si) {
            let mut f = finder;
            f.target = match s.coord_sys {
                CoordSysKind::Polar => ConvertTarget::Polar,
                CoordSysKind::Radar => ConvertTarget::Radar,
                CoordSysKind::Geo => ConvertTarget::Geo,
                CoordSysKind::Calendar => ConvertTarget::Calendar,
                CoordSysKind::Single => ConvertTarget::Single,
                CoordSysKind::Parallel => ConvertTarget::Parallel,
                CoordSysKind::Matrix => ConvertTarget::Matrix,
                _ => ConvertTarget::Cartesian,
            };
            return f;
        }
    }
    if !model.has_cartesian_series() {
        let mut f = finder;
        if !model.polars.is_empty() {
            f.target = ConvertTarget::Polar;
        } else if !model.geos.is_empty() {
            f.target = ConvertTarget::Geo;
        } else if !model.calendars.is_empty() {
            f.target = ConvertTarget::Calendar;
        } else if !model.single_axes.is_empty() {
            f.target = ConvertTarget::Single;
        } else if !model.parallels.is_empty() {
            f.target = ConvertTarget::Parallel;
        } else if !model.matrices.is_empty() {
            f.target = ConvertTarget::Matrix;
        } else if !model.radars.is_empty() {
            f.target = ConvertTarget::Radar;
        }
        return f;
    }
    finder
}

fn convert_cartesian_to_pixel(
    model: &GlobalModel,
    finder: FinderRef,
    value: &OptionValue,
) -> Option<ConvertResult> {
    let coord = resolve_cartesian(model, finder);
    match finder.target {
        ConvertTarget::XAxis => Some(ConvertResult::Scalar(
            coord.x_value_to_pixel(resolve_x_data(coord.x_axis, value)),
        )),
        ConvertTarget::YAxis => Some(ConvertResult::Scalar(
            coord.y_value_to_pixel(resolve_y_data(coord.y_axis, value)),
        )),
        _ => {
            let (x_val, y_val) = match value {
                OptionValue::Array(arr) if arr.len() >= 2 => (
                    resolve_x_data(coord.x_axis, &arr[0]),
                    resolve_y_data(coord.y_axis, &arr[1]),
                ),
                _ => return None,
            };
            let (px, py) = if coord.is_horizontal() {
                coord.point_for(y_val.round().max(0.0) as usize, Some(x_val), x_val)
            } else if coord.x_axis.axis_type.is_category() {
                coord.data_to_point(x_val.round().max(0.0) as usize, y_val)
            } else {
                coord.value_to_point(x_val, y_val)
            };
            Some(ConvertResult::Point(px, py))
        }
    }
}

fn convert_polar_to_pixel(
    model: &GlobalModel,
    finder: FinderRef,
    value: &OptionValue,
) -> Option<ConvertResult> {
    let idx = finder
        .polar
        .or_else(|| finder.series.and_then(|si| model.series.get(si).map(|s| s.polar_index)))
        .unwrap_or(0);
    let coord = PolarCoord::new(model, idx)?;
    match finder.target {
        ConvertTarget::RadiusAxis => {
            let v = polar::resolve_polar_value(coord.radius_axis(), value);
            Some(ConvertResult::Scalar(coord.radius_to_pixel(v)))
        }
        ConvertTarget::AngleAxis => {
            let v = polar::resolve_polar_value(coord.angle_axis(), value);
            Some(ConvertResult::Scalar(coord.angle_to_degree(v)))
        }
        _ => {
            let (rv, av) = match value {
                OptionValue::Array(arr) if arr.len() >= 2 => (
                    polar::resolve_polar_value(coord.radius_axis(), &arr[0]),
                    polar::resolve_polar_value(coord.angle_axis(), &arr[1]),
                ),
                _ => return None,
            };
            let (px, py) = coord.data_to_point(rv, av);
            Some(ConvertResult::Point(px, py))
        }
    }
}

fn convert_radar_to_pixel(
    model: &GlobalModel,
    finder: FinderRef,
    value: &OptionValue,
) -> Option<ConvertResult> {
    let idx = finder.radar.unwrap_or(0);
    let coord = RadarCoord::new(model, idx)?;
    match value {
        OptionValue::Array(arr) if arr.len() >= 2 => {
            let indicator = arr[0].as_f64().unwrap_or(0.0) as usize;
            let v = arr[1].as_f64().unwrap_or(0.0);
            let (px, py) = coord.data_to_point(v, indicator);
            Some(ConvertResult::Point(px, py))
        }
        OptionValue::Number(n) => {
            let (px, py) = coord.data_to_point(*n, 0);
            Some(ConvertResult::Point(px, py))
        }
        _ => None,
    }
}

fn convert_geo_to_pixel(
    model: &GlobalModel,
    finder: FinderRef,
    value: &OptionValue,
) -> Option<ConvertResult> {
    let idx = finder.geo.unwrap_or(0);
    let coord = GeoCoord::new(model, idx)?;
    if let Some((lng, lat)) = GeoCoord::resolve_lng_lat(value) {
        let (px, py) = coord.data_to_point(lng, lat);
        return Some(ConvertResult::Point(px, py));
    }
    if let Some(name) = value.as_str() {
        let (px, py) = coord.named_point(name)?;
        return Some(ConvertResult::Point(px, py));
    }
    None
}

pub fn convert_from_pixel(
    model: &GlobalModel,
    finder: &OptionValue,
    value: &OptionValue,
) -> Option<ConvertResult> {
    let finder = infer_finder(model, parse_finder(finder));
    let (px, py) = match value {
        OptionValue::Array(arr) if arr.len() >= 2 => (arr[0].as_f64()?, arr[1].as_f64()?),
        OptionValue::Number(n) => (*n, *n),
        _ => return None,
    };
    match finder.target {
        ConvertTarget::Polar => {
            let coord = PolarCoord::new(model, finder.polar.unwrap_or(0))?;
            let (r, a) = coord.point_to_data(px, py);
            Some(ConvertResult::Point(r, a))
        }
        ConvertTarget::RadiusAxis => {
            let coord = PolarCoord::new(model, finder.polar.unwrap_or(0))?;
            let (r, _) = coord.point_to_data(px, py);
            Some(ConvertResult::Scalar(r))
        }
        ConvertTarget::AngleAxis => {
            let coord = PolarCoord::new(model, finder.polar.unwrap_or(0))?;
            let (_, a) = coord.point_to_data(px, py);
            Some(ConvertResult::Scalar(a))
        }
        ConvertTarget::Geo => {
            let coord = GeoCoord::new(model, finder.geo.unwrap_or(0))?;
            let (lng, lat) = coord.point_to_data(px, py);
            Some(ConvertResult::Point(lng, lat))
        }
        ConvertTarget::Calendar => {
            let coord = CalendarCoord::new(model, finder.calendar.unwrap_or(0))?;
            Some(ConvertResult::Scalar(coord.point_to_data(px, py)))
        }
        ConvertTarget::Single => {
            let coord = SingleCoord::new(model, finder.single.unwrap_or(0))?;
            Some(ConvertResult::Scalar(coord.point_to_data(px, py)))
        }
        ConvertTarget::Parallel => {
            let coord = ParallelCoord::new(model, finder.parallel.unwrap_or(0))?;
            let dim = finder.parallel_axis.unwrap_or(0);
            Some(ConvertResult::Scalar(coord.point_to_data(px, py, dim)))
        }
        ConvertTarget::Matrix => {
            let coord = MatrixCoord::new(model, finder.matrix.unwrap_or(0))?;
            let (x, y) = coord.point_to_data(px, py);
            Some(ConvertResult::Point(x, y))
        }
        ConvertTarget::Radar => {
            let coord = RadarCoord::new(model, finder.radar.unwrap_or(0))?;
            let dx = px - coord.spec.center_x;
            let dy = py - coord.spec.center_y;
            Some(ConvertResult::Point(dx.atan2(-dy), (dx * dx + dy * dy).sqrt()))
        }
        ConvertTarget::XAxis | ConvertTarget::YAxis | ConvertTarget::Cartesian => {
            if !model.has_cartesian_series() && model.grids.is_empty() {
                return None;
            }
            let coord = resolve_cartesian(model, finder);
            match finder.target {
                ConvertTarget::XAxis => Some(ConvertResult::Scalar(coord.pixel_to_x_value(px))),
                ConvertTarget::YAxis => Some(ConvertResult::Scalar(coord.pixel_to_y_value(py))),
                _ => {
                    let (x, y) = coord.point_to_data(px, py);
                    Some(ConvertResult::Point(x, y))
                }
            }
        }
    }
}

pub fn contain_pixel(model: &GlobalModel, finder: &OptionValue, x: f64, y: f64) -> bool {
    let finder = infer_finder(model, parse_finder(finder));
    match finder.target {
        ConvertTarget::Polar | ConvertTarget::AngleAxis | ConvertTarget::RadiusAxis => {
            PolarCoord::new(model, finder.polar.unwrap_or(0))
                .map(|c| c.contain(x, y))
                .unwrap_or(false)
        }
        ConvertTarget::Radar => RadarCoord::new(model, finder.radar.unwrap_or(0))
            .map(|c| c.contain(x, y))
            .unwrap_or(false),
        ConvertTarget::Geo => GeoCoord::new(model, finder.geo.unwrap_or(0))
            .map(|c| c.contain(x, y))
            .unwrap_or(false),
        ConvertTarget::Calendar => CalendarCoord::new(model, finder.calendar.unwrap_or(0))
            .map(|c| c.contain(x, y))
            .unwrap_or(false),
        ConvertTarget::Single => SingleCoord::new(model, finder.single.unwrap_or(0))
            .map(|c| c.contain(x, y))
            .unwrap_or(false),
        ConvertTarget::Parallel => ParallelCoord::new(model, finder.parallel.unwrap_or(0))
            .map(|c| c.contain(x, y))
            .unwrap_or(false),
        ConvertTarget::Matrix => MatrixCoord::new(model, finder.matrix.unwrap_or(0))
            .map(|c| c.contain(x, y))
            .unwrap_or(false),
        ConvertTarget::XAxis | ConvertTarget::YAxis | ConvertTarget::Cartesian => {
            if finder.series.is_some()
                || finder.grid.is_some()
                || finder.x_axis.is_some()
                || finder.y_axis.is_some()
            {
                return resolve_cartesian(model, finder).grid.contains(x, y);
            }
            if model.grids.iter().any(|g| g.contains(x, y)) {
                return true;
            }
            model.polars.iter().any(|p| {
                let dx = x - p.center_x;
                let dy = y - p.center_y;
                (dx * dx + dy * dy).sqrt() <= p.r + 1e-4
            }) || model.geos.iter().any(|g| g.rect.contains(x, y))
                || model.calendars.iter().any(|c| c.rect.contains(x, y))
                || model.single_axes.iter().any(|s| s.rect.contains(x, y))
                || model.parallels.iter().any(|p| p.rect.contains(x, y))
                || model.matrices.iter().any(|m| m.rect.contains(x, y))
                || model.radars.iter().any(|r| {
                    let dx = x - r.center_x;
                    let dy = y - r.center_y;
                    (dx * dx + dy * dy).sqrt() <= r.r + 1e-4
                })
        }
    }
}

pub fn format_axis_tick(axis: &AxisModel, value: f64, category: Option<&str>) -> String {
    if axis.axis_type.is_category() {
        return category.unwrap_or("").to_string();
    }
    if axis.axis_type == AxisType::Time {
        return format_time_label(value);
    }
    crate::utils::format_axis_number(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::GlobalModel;
    use crate::option::{OptionModel, OptionValue, SetOptionFlags};
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    fn category_model() -> GlobalModel {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "xAxis",
                    obj(vec![
                        ("type", OptionValue::String("category".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::String("A".into()),
                                OptionValue::String("B".into()),
                                OptionValue::String("C".into()),
                            ]),
                        ),
                    ]),
                ),
                ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("bar".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::Number(10.0),
                                OptionValue::Number(20.0),
                                OptionValue::Number(30.0),
                            ]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                ..Default::default()
            },
        );
        GlobalModel::from_option(&option, 400, 300)
    }

    #[test]
    fn convert_x_axis_category_roundtrip() {
        let model = category_model();
        let finder = obj(vec![("xAxisIndex", OptionValue::Number(0.0))]);
        let to = convert_to_pixel(&model, &finder, &OptionValue::Number(1.0)).unwrap();
        let ConvertResult::Scalar(px) = to else {
            panic!("expected scalar");
        };
        let back = convert_from_pixel(&model, &finder, &OptionValue::Number(px)).unwrap();
        let ConvertResult::Scalar(idx) = back else {
            panic!("expected scalar");
        };
        assert_eq!(idx, 1.0);
    }

    #[test]
    fn convert_grid_point() {
        let model = category_model();
        let finder = obj(vec![("gridIndex", OptionValue::Number(0.0))]);
        let value = OptionValue::Array(vec![
            OptionValue::Number(1.0),
            OptionValue::Number(20.0),
        ]);
        let ConvertResult::Point(px, py) = convert_to_pixel(&model, &finder, &value).unwrap() else {
            panic!("expected point");
        };
        let g = model.grid();
        assert!(px > g.x && px < g.x + g.width);
        assert!(py > g.y && py < g.y + g.height);
        let back = convert_from_pixel(
            &model,
            &finder,
            &OptionValue::Array(vec![OptionValue::Number(px), OptionValue::Number(py)]),
        )
        .unwrap();
        let ConvertResult::Point(x, y) = back else {
            panic!("expected point");
        };
        assert_eq!(x, 1.0);
        assert!((y - 20.0).abs() < 3.0);
    }

    #[test]
    fn finder_x_axis_string() {
        assert_eq!(
            parse_convert_finder(&OptionValue::String("xAxis".into())),
            ConvertTarget::XAxis
        );
    }

    #[test]
    fn convert_time_and_log() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                ("xAxis", obj(vec![("type", OptionValue::String("time".into()))])),
                ("yAxis", obj(vec![("type", OptionValue::String("log".into()))])),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::Array(vec![
                                    OptionValue::String("2020-01-01".into()),
                                    OptionValue::Number(10.0),
                                ]),
                                OptionValue::Array(vec![
                                    OptionValue::String("2020-01-11".into()),
                                    OptionValue::Number(1000.0),
                                ]),
                            ]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                ..Default::default()
            },
        );
        let model = GlobalModel::from_option(&option, 400, 300);
        let finder = obj(vec![("gridIndex", OptionValue::Number(0.0))]);
        let value = OptionValue::Array(vec![
            OptionValue::String("2020-01-01".into()),
            OptionValue::Number(10.0),
        ]);
        let ConvertResult::Point(px, py) = convert_to_pixel(&model, &finder, &value).unwrap() else {
            panic!("expected point");
        };
        let g = model.grid();
        assert!(px >= g.x && px <= g.x + g.width);
        assert!(py >= g.y && py <= g.y + g.height);
        let y_finder = obj(vec![("yAxisIndex", OptionValue::Number(0.0))]);
        let ConvertResult::Scalar(p10) =
            convert_to_pixel(&model, &y_finder, &OptionValue::Number(10.0)).unwrap()
        else {
            panic!("scalar");
        };
        let ConvertResult::Scalar(p100) =
            convert_to_pixel(&model, &y_finder, &OptionValue::Number(100.0)).unwrap()
        else {
            panic!("scalar");
        };
        let ConvertResult::Scalar(p1000) =
            convert_to_pixel(&model, &y_finder, &OptionValue::Number(1000.0)).unwrap()
        else {
            panic!("scalar");
        };
        let d1 = (p10 - p100).abs();
        let d2 = (p100 - p1000).abs();
        assert!(
            (d1 - d2).abs() < 2.0,
            "log ticks should be even in pixel: {} {}",
            d1,
            d2
        );
    }

    fn apply(root: OptionValue) -> GlobalModel {
        let mut option = OptionModel::new();
        option.apply(
            root,
            SetOptionFlags {
                not_merge: true,
                ..Default::default()
            },
        );
        GlobalModel::from_option(&option, 400, 300)
    }

    #[test]
    fn polar_value_value_maps_zero_angle_to_three_oclock() {
        let model = apply(obj(vec![
            ("polar", obj(vec![])),
            (
                "angleAxis",
                obj(vec![
                    ("type", OptionValue::String("value".into())),
                    ("startAngle", OptionValue::Number(0.0)),
                    ("min", OptionValue::Number(0.0)),
                    ("max", OptionValue::Number(360.0)),
                ]),
            ),
            (
                "radiusAxis",
                obj(vec![
                    ("min", OptionValue::Number(0.0)),
                    ("max", OptionValue::Number(10.0)),
                ]),
            ),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("line".into())),
                    ("coordinateSystem", OptionValue::String("polar".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::Array(vec![
                            OptionValue::Number(10.0),
                            OptionValue::Number(0.0),
                        ])]),
                    ),
                ])]),
            ),
        ]));
        assert_eq!(model.polars.len(), 1);
        let finder = obj(vec![("polarIndex", OptionValue::Number(0.0))]);
        let value = OptionValue::Array(vec![OptionValue::Number(10.0), OptionValue::Number(0.0)]);
        let ConvertResult::Point(px, py) = convert_to_pixel(&model, &finder, &value).unwrap() else {
            panic!("point");
        };
        let p = &model.polars[0];
        assert!((px - (p.center_x + p.r)).abs() < 2.0, "0° should be 3 o'clock: {px} {py} cx={} r={}", p.center_x, p.r);
        assert!((py - p.center_y).abs() < 2.0);
        assert!(contain_pixel(&model, &finder, px, py));
    }

    #[test]
    fn y_category_is_horizontal() {
        let model = apply(obj(vec![
            ("xAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "yAxis",
                obj(vec![
                    ("type", OptionValue::String("category".into())),
                    (
                        "data",
                        OptionValue::Array(vec![
                            OptionValue::String("A".into()),
                            OptionValue::String("B".into()),
                        ]),
                    ),
                ]),
            ),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("bar".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::Number(10.0), OptionValue::Number(20.0)]),
                    ),
                ])]),
            ),
        ]));
        let coord = Cartesian2D::for_series(&model, &model.series[0]);
        assert!(coord.is_horizontal());
        let (x0, y0) = coord.point_for(0, None, 10.0);
        let (x1, y1) = coord.point_for(1, None, 20.0);
        assert!(
            y0 > y1,
            "category 0 at bottom: {y0} {y1}",
            y0 = y0,
            y1 = y1
        );
        assert!(
            x1 > x0,
            "larger value further right: {x0} {x1}",
            x0 = x0,
            x1 = x1
        );
    }

    #[test]
    fn calendar_date_lands_in_rect() {
        let model = apply(obj(vec![(
            "calendar",
            obj(vec![
                ("range", OptionValue::String("2017".into())),
                ("left", OptionValue::Number(20.0)),
                ("top", OptionValue::Number(20.0)),
                ("right", OptionValue::Number(20.0)),
                ("bottom", OptionValue::Number(20.0)),
            ]),
        )]));
        assert_eq!(model.calendars.len(), 1);
        let finder = obj(vec![("calendarIndex", OptionValue::Number(0.0))]);
        let ConvertResult::Point(px, py) =
            convert_to_pixel(&model, &finder, &OptionValue::String("2017-01-01".into())).unwrap()
        else {
            panic!("point");
        };
        assert!(model.calendars[0].rect.contains(px, py));
    }

    #[test]
    fn matrix_cell_center() {
        let model = apply(obj(vec![(
            "matrix",
            obj(vec![
                (
                    "x",
                    obj(vec![(
                        "data",
                        OptionValue::Array(vec![
                            OptionValue::String("A".into()),
                            OptionValue::String("B".into()),
                        ]),
                    )]),
                ),
                (
                    "y",
                    obj(vec![(
                        "data",
                        OptionValue::Array(vec![
                            OptionValue::String("1".into()),
                            OptionValue::String("2".into()),
                        ]),
                    )]),
                ),
            ]),
        )]));
        let finder = obj(vec![("matrixIndex", OptionValue::Number(0.0))]);
        let value = OptionValue::Array(vec![
            OptionValue::String("A".into()),
            OptionValue::String("1".into()),
        ]);
        let ConvertResult::Point(px, py) = convert_to_pixel(&model, &finder, &value).unwrap() else {
            panic!("point");
        };
        let r = model.matrices[0].rect;
        assert!(px > r.x && px < r.x + r.width / 2.0);
        assert!(py > r.y && py < r.y + r.height / 2.0);
    }

    #[test]
    fn geo_lng_lat_after_register_map() {
        crate::maps::register_map(
            "toy".into(),
            obj(vec![(
                "features",
                OptionValue::Array(vec![obj(vec![
                    (
                        "properties",
                        obj(vec![("name", OptionValue::String("box".into()))]),
                    ),
                    (
                        "geometry",
                        obj(vec![
                            ("type", OptionValue::String("Polygon".into())),
                            (
                                "coordinates",
                                OptionValue::Array(vec![OptionValue::Array(vec![
                                    OptionValue::Array(vec![
                                        OptionValue::Number(100.0),
                                        OptionValue::Number(20.0),
                                    ]),
                                    OptionValue::Array(vec![
                                        OptionValue::Number(120.0),
                                        OptionValue::Number(20.0),
                                    ]),
                                    OptionValue::Array(vec![
                                        OptionValue::Number(120.0),
                                        OptionValue::Number(40.0),
                                    ]),
                                    OptionValue::Array(vec![
                                        OptionValue::Number(100.0),
                                        OptionValue::Number(40.0),
                                    ]),
                                    OptionValue::Array(vec![
                                        OptionValue::Number(100.0),
                                        OptionValue::Number(20.0),
                                    ]),
                                ])]),
                            ),
                        ]),
                    ),
                ])]),
            )]),
            OptionValue::Null,
        );
        let model = apply(obj(vec![("geo", obj(vec![("map", OptionValue::String("toy".into()))]))]));
        assert_eq!(model.geos.len(), 1);
        assert_eq!(model.geos[0].regions.len(), 1);
        let finder = obj(vec![("geoIndex", OptionValue::Number(0.0))]);
        let value = OptionValue::Array(vec![
            OptionValue::Number(110.0),
            OptionValue::Number(30.0),
        ]);
        let ConvertResult::Point(px, py) = convert_to_pixel(&model, &finder, &value).unwrap() else {
            panic!("point");
        };
        assert!(model.geos[0].rect.contains(px, py));
    }

    #[test]
    fn radar_and_single_and_parallel_parse() {
        let model = apply(obj(vec![
            (
                "radar",
                obj(vec![(
                    "indicator",
                    OptionValue::Array(vec![
                        obj(vec![
                            ("name", OptionValue::String("A".into())),
                            ("max", OptionValue::Number(100.0)),
                        ]),
                        obj(vec![
                            ("name", OptionValue::String("B".into())),
                            ("max", OptionValue::Number(100.0)),
                        ]),
                    ]),
                )]),
            ),
            ("singleAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "parallelAxis",
                OptionValue::Array(vec![
                    obj(vec![("dim", OptionValue::Number(0.0)), ("name", OptionValue::String("x".into()))]),
                    obj(vec![("dim", OptionValue::Number(1.0)), ("name", OptionValue::String("y".into()))]),
                ]),
            ),
        ]));
        assert_eq!(model.radars.len(), 1);
        assert_eq!(model.radars[0].indicators.len(), 2);
        assert_eq!(model.single_axes.len(), 1);
        assert_eq!(model.parallels.len(), 1);
        assert_eq!(model.parallels[0].axes.len(), 2);
        let radar_finder = obj(vec![("radarIndex", OptionValue::Number(0.0))]);
        let ConvertResult::Point(px, py) = convert_to_pixel(
            &model,
            &radar_finder,
            &OptionValue::Array(vec![OptionValue::Number(0.0), OptionValue::Number(100.0)]),
        )
        .unwrap() else {
            panic!("radar");
        };
        assert!(px.is_finite() && py.is_finite());
    }
}
