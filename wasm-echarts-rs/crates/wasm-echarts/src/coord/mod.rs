//! 直角坐标系：dataToPoint / 轴布局 / convertToPixel（含 time / log / 多轴）

use crate::data::{format_time_label, numeric_or_time};
use crate::model::{AxisModel, AxisType, GlobalModel, GridRect, SeriesModel};
use crate::option::OptionValue;

pub struct Cartesian2D<'a> {
    model: &'a GlobalModel,
    grid: GridRect,
    x_axis: &'a AxisModel,
    y_axis: &'a AxisModel,
    x_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertTarget {
    Cartesian,
    XAxis,
    YAxis,
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
        }
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
        axis_value_to_pixel(self.y_axis, y_val, self.grid.y, self.grid.height, true, self.model, 0)
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
        let (start, end) = model.visible_category_range_of(x_index);
        let visible = (end - start).max(1);
        let idx = if value.is_finite() {
            value.round().max(0.0) as usize
        } else {
            0
        };
        let local = idx.saturating_sub(start);
        let t = (local as f64 + 0.5) / visible as f64;
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

fn scale_ratio(axis: &AxisModel, value: f64) -> f64 {
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

fn scale_from_ratio(axis: &AxisModel, t: f64) -> f64 {
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
                _ => ConvertTarget::Cartesian,
            },
            series: None,
            grid: None,
            x_axis: None,
            y_axis: None,
        };
    }
    let series = finder_index(finder, &["seriesIndex", "series"]);
    let grid = finder_index(finder, &["gridIndex", "grid"]);
    let x_axis = finder_index(finder, &["xAxisIndex", "xAxis"]);
    let y_axis = finder_index(finder, &["yAxisIndex", "yAxis"]);
    let target = if series.is_some() || grid.is_some() || (x_axis.is_some() && y_axis.is_some()) {
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
    if !model.has_cartesian_series() {
        return None;
    }
    let finder = parse_finder(finder);
    let coord = resolve_cartesian(model, finder);
    match finder.target {
        ConvertTarget::XAxis => Some(ConvertResult::Scalar(
            coord.x_value_to_pixel(resolve_x_data(coord.x_axis, value)),
        )),
        ConvertTarget::YAxis => Some(ConvertResult::Scalar(
            coord.y_value_to_pixel(resolve_y_data(coord.y_axis, value)),
        )),
        ConvertTarget::Cartesian => {
            let (x_val, y_val) = match value {
                OptionValue::Array(arr) if arr.len() >= 2 => (
                    resolve_x_data(coord.x_axis, &arr[0]),
                    resolve_y_data(coord.y_axis, &arr[1]),
                ),
                _ => return None,
            };
            let (px, py) = if coord.x_axis.axis_type.is_category() {
                coord.data_to_point(x_val.round().max(0.0) as usize, y_val)
            } else {
                coord.value_to_point(x_val, y_val)
            };
            Some(ConvertResult::Point(px, py))
        }
    }
}

pub fn convert_from_pixel(
    model: &GlobalModel,
    finder: &OptionValue,
    value: &OptionValue,
) -> Option<ConvertResult> {
    if !model.has_cartesian_series() {
        return None;
    }
    let finder = parse_finder(finder);
    let coord = resolve_cartesian(model, finder);
    match finder.target {
        ConvertTarget::XAxis => {
            let px = value.as_f64()?;
            Some(ConvertResult::Scalar(coord.pixel_to_x_value(px)))
        }
        ConvertTarget::YAxis => {
            let py = value.as_f64()?;
            Some(ConvertResult::Scalar(coord.pixel_to_y_value(py)))
        }
        ConvertTarget::Cartesian => {
            let (px, py) = match value {
                OptionValue::Array(arr) if arr.len() >= 2 => {
                    (arr[0].as_f64()?, arr[1].as_f64()?)
                }
                _ => return None,
            };
            let (x, y) = coord.point_to_data(px, py);
            Some(ConvertResult::Point(x, y))
        }
    }
}

pub fn contain_pixel(model: &GlobalModel, finder: &OptionValue, x: f64, y: f64) -> bool {
    if !model.has_cartesian_series() && model.grids.is_empty() {
        return false;
    }
    let finder = parse_finder(finder);
    if finder.series.is_some() || finder.grid.is_some() || finder.x_axis.is_some() || finder.y_axis.is_some()
    {
        return resolve_cartesian(model, finder).grid.contains(x, y);
    }
    model.grids.iter().any(|g| g.contains(x, y))
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
}
