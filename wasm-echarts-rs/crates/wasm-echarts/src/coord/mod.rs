//! 直角坐标系：dataToPoint / 轴布局 / convertToPixel 最小集

use crate::model::{AxisType, GlobalModel, GridRect};
use crate::option::OptionValue;

pub struct Cartesian2D<'a> {
    model: &'a GlobalModel,
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

impl<'a> Cartesian2D<'a> {
    pub fn new(model: &'a GlobalModel) -> Self {
        Self { model }
    }

    pub fn grid(&self) -> GridRect {
        self.model.grid
    }

    /// category index + value → 像素坐标（category_index 为全局索引）
    pub fn data_to_point(&self, category_index: usize, value: f64) -> (f64, f64) {
        let x = self.x_value_to_pixel(category_index as f64);
        let y = self.y_value_to_pixel(value);
        (x, y)
    }

    pub fn base_y(&self) -> f64 {
        self.y_value_to_pixel(0.0)
            .clamp(self.model.grid.y, self.model.grid.y + self.model.grid.height)
    }

    /// 双 value 轴坐标（散点图）
    pub fn value_to_point(&self, x_val: f64, y_val: f64) -> (f64, f64) {
        let grid = self.model.grid;
        let px = self.x_value_to_pixel(x_val).clamp(grid.x, grid.x + grid.width);
        let py = self.y_value_to_pixel(y_val).clamp(grid.y, grid.y + grid.height);
        (px, py)
    }

    pub fn category_band_width(&self) -> f64 {
        let (_, end) = self.model.visible_category_range();
        let (start, _) = self.model.visible_category_range();
        let visible = (end - start).max(1);
        self.model.grid.width / visible as f64
    }

    /// 像素 x → 最近的全局 category 索引
    pub fn point_to_category_index(&self, x: f64) -> Option<usize> {
        let grid = self.model.grid;
        if x < grid.x || x > grid.x + grid.width {
            return None;
        }
        let (start, end) = self.model.visible_category_range();
        let visible = (end - start).max(1);
        let rel = ((x - grid.x) / grid.width).clamp(0.0, 0.999_999);
        let local = (rel * visible as f64).floor() as usize;
        Some((start + local).min(self.model.category_count().saturating_sub(1)))
    }

    pub fn x_value_to_pixel(&self, x_val: f64) -> f64 {
        let grid = self.model.grid;
        if self.model.x_axis.axis_type == AxisType::Category {
            let (start, end) = self.model.visible_category_range();
            let visible = (end - start).max(1);
            let idx = if x_val.is_finite() {
                x_val.round().max(0.0) as usize
            } else {
                0
            };
            let local = idx.saturating_sub(start);
            grid.x + (local as f64 + 0.5) / visible as f64 * grid.width
        } else {
            let xmin = self.model.x_axis.value_min();
            let xmax = self.model.x_axis.value_max();
            let xspan = (xmax - xmin).max(f64::EPSILON);
            let xr = ((x_val - xmin) / xspan).clamp(0.0, 1.0);
            grid.x + xr * grid.width
        }
    }

    pub fn y_value_to_pixel(&self, y_val: f64) -> f64 {
        let grid = self.model.grid;
        let ymin = self.model.y_axis.value_min();
        let ymax = self.model.y_axis.value_max();
        let span = (ymax - ymin).max(f64::EPSILON);
        let ratio = (y_val - ymin) / span;
        grid.y + grid.height * (1.0 - ratio)
    }

    pub fn pixel_to_x_value(&self, px: f64) -> f64 {
        if self.model.x_axis.axis_type == AxisType::Category {
            return self.point_to_category_index(px).unwrap_or(0) as f64;
        }
        let grid = self.model.grid;
        let xmin = self.model.x_axis.value_min();
        let xmax = self.model.x_axis.value_max();
        if grid.width <= 0.0 {
            return xmin;
        }
        let xr = ((px - grid.x) / grid.width).clamp(0.0, 1.0);
        xmin + xr * (xmax - xmin)
    }

    pub fn pixel_to_y_value(&self, py: f64) -> f64 {
        let grid = self.model.grid;
        let ymin = self.model.y_axis.value_min();
        let ymax = self.model.y_axis.value_max();
        if grid.height <= 0.0 {
            return ymin;
        }
        let yr = (1.0 - (py - grid.y) / grid.height).clamp(0.0, 1.0);
        ymin + yr * (ymax - ymin)
    }

    pub fn point_to_data(&self, px: f64, py: f64) -> (f64, f64) {
        (self.pixel_to_x_value(px), self.pixel_to_y_value(py))
    }
}

pub fn parse_convert_finder(finder: &OptionValue) -> ConvertTarget {
    if let Some(s) = finder.as_str() {
        return match s {
            "xAxis" => ConvertTarget::XAxis,
            "yAxis" => ConvertTarget::YAxis,
            _ => ConvertTarget::Cartesian,
        };
    }
    let series = finder_index(finder, &["seriesIndex", "series"]);
    let grid = finder_index(finder, &["gridIndex", "grid"]);
    let x_axis = finder_index(finder, &["xAxisIndex", "xAxis"]);
    let y_axis = finder_index(finder, &["yAxisIndex", "yAxis"]);
    if series.is_some() || grid.is_some() || (x_axis.is_some() && y_axis.is_some()) {
        ConvertTarget::Cartesian
    } else if x_axis.is_some() {
        ConvertTarget::XAxis
    } else if y_axis.is_some() {
        ConvertTarget::YAxis
    } else {
        ConvertTarget::Cartesian
    }
}

fn finder_index(finder: &OptionValue, keys: &[&str]) -> Option<i32> {
    for key in keys {
        if let Some(n) = finder.get(key).and_then(|v| v.as_f64()) {
            return Some(n as i32);
        }
    }
    None
}

fn resolve_x_data(model: &GlobalModel, value: &OptionValue) -> f64 {
    if let Some(s) = value.as_str() {
        if let Some(idx) = model.x_categories.iter().position(|c| c == s) {
            return idx as f64;
        }
        return s.parse().unwrap_or(0.0);
    }
    value.as_f64().unwrap_or(0.0)
}

fn resolve_y_data(value: &OptionValue) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

pub fn convert_to_pixel(
    model: &GlobalModel,
    finder: &OptionValue,
    value: &OptionValue,
) -> Option<ConvertResult> {
    if !model.has_cartesian_series() {
        return None;
    }
    let coord = Cartesian2D::new(model);
    match parse_convert_finder(finder) {
        ConvertTarget::XAxis => Some(ConvertResult::Scalar(
            coord.x_value_to_pixel(resolve_x_data(model, value)),
        )),
        ConvertTarget::YAxis => Some(ConvertResult::Scalar(
            coord.y_value_to_pixel(resolve_y_data(value)),
        )),
        ConvertTarget::Cartesian => {
            let (x_val, y_val) = match value {
                OptionValue::Array(arr) if arr.len() >= 2 => {
                    (resolve_x_data(model, &arr[0]), resolve_y_data(&arr[1]))
                }
                _ => return None,
            };
            let (px, py) = if model.x_axis.axis_type == AxisType::Category {
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
    let coord = Cartesian2D::new(model);
    match parse_convert_finder(finder) {
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
        assert!(px > model.grid.x && px < model.grid.x + model.grid.width);
        assert!(py > model.grid.y && py < model.grid.y + model.grid.height);
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
}
