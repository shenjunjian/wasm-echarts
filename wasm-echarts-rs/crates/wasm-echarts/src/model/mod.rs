//! GlobalModel：从 OptionModel 提取布局与 series 数据

mod axis;
mod series;

pub use axis::{AxisModel, AxisType};
pub use series::{DataPoint, SeriesModel, SeriesType};

use crate::option::{OptionModel, OptionValue};
use crate::visual::{parse_series_data, series_name_of, series_type_of};

/// 绘图区矩形（像素）
#[derive(Debug, Clone, Copy)]
pub struct GridRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl GridRect {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x
            && x <= self.x + self.width
            && y >= self.y
            && y <= self.y + self.height
    }
}

#[derive(Debug, Clone)]
pub struct GlobalModel {
    pub width: u32,
    pub height: u32,
    pub grid: GridRect,
    pub x_axis: AxisModel,
    pub y_axis: AxisModel,
    pub x_categories: Vec<String>,
    pub series: Vec<SeriesModel>,
}

impl GlobalModel {
    pub fn from_option(option: &OptionModel, width: u32, height: u32) -> Self {
        let root = option.root();
        let w = width as f64;
        let h = height as f64;

        let grid_opt = root.get("grid");
        let grid = parse_grid(grid_opt, w, h);

        let x_axis = parse_x_axis(root.get("xAxis"), root);
        let y_axis = parse_y_axis(root.get("yAxis"));

        let x_categories = x_axis.category_data.clone();

        let series: Vec<SeriesModel> = root
            .get("series")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .enumerate()
                    .map(|(i, s)| {
                        let st = series_type_of(s);
                        SeriesModel {
                            index: i,
                            name: series_name_of(s, i),
                            series_type: SeriesType::from_str(st),
                            data: parse_series_data(s.get("data")),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let y_axis = y_axis.with_data_range(compute_value_extent(&series));

        Self {
            width,
            height,
            grid,
            x_axis,
            y_axis,
            x_categories,
            series,
        }
    }

    pub fn category_count(&self) -> usize {
        if !self.x_categories.is_empty() {
            self.x_categories.len()
        } else {
            self.series
                .first()
                .map(|s| s.data.len())
                .unwrap_or(0)
        }
    }
}

fn parse_grid(value: Option<&OptionValue>, width: f64, height: f64) -> GridRect {
    let default_left = 60.0;
    let default_right = 20.0;
    let default_top = 40.0;
    let default_bottom = 50.0;

    let (left, right, top, bottom) = match value {
        Some(OptionValue::Object(map)) => (
            parse_margin(map.get("left"), width, default_left),
            parse_margin(map.get("right"), width, default_right),
            parse_margin(map.get("top"), height, default_top),
            parse_margin(map.get("bottom"), height, default_bottom),
        ),
        _ => (default_left, default_right, default_top, default_bottom),
    };

    GridRect {
        x: left,
        y: top,
        width: (width - left - right).max(1.0),
        height: (height - top - bottom).max(1.0),
    }
}

fn parse_margin(value: Option<&OptionValue>, total: f64, default_px: f64) -> f64 {
    match value {
        Some(OptionValue::Number(n)) => *n,
        Some(OptionValue::String(s)) if s.ends_with('%') => {
            s.trim_end_matches('%')
                .parse::<f64>()
                .map(|p| total * p / 100.0)
                .unwrap_or(default_px)
        }
        _ => default_px,
    }
}

fn first_component(value: Option<&OptionValue>) -> Option<&OptionValue> {
    match value {
        Some(OptionValue::Array(arr)) => arr.first(),
        Some(v) => Some(v),
        None => None,
    }
}

fn parse_x_axis(value: Option<&OptionValue>, root: &OptionValue) -> AxisModel {
    let comp = first_component(value);
    let axis_type = comp
        .and_then(|v| v.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("category");

    let mut categories: Vec<String> = comp
        .and_then(|v| v.get("data"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|item| match item {
                    OptionValue::String(s) => s.clone(),
                    OptionValue::Number(n) => n.to_string(),
                    _ => String::new(),
                })
                .collect()
        })
        .unwrap_or_default();

    if categories.is_empty() {
        if let Some(series) = root.get("series").and_then(|v| v.as_array()) {
            if let Some(first) = series.first() {
                if let Some(data) = first.get("data").and_then(|v| v.as_array()) {
                    categories = (0..data.len()).map(|i| i.to_string()).collect();
                }
            }
        }
    }

    AxisModel {
        axis_type: if axis_type == "value" {
            AxisType::Value
        } else {
            AxisType::Category
        },
        category_data: categories,
        min: None,
        max: None,
    }
}

fn parse_y_axis(value: Option<&OptionValue>) -> AxisModel {
    let comp = first_component(value);
    let axis_type = comp
        .and_then(|v| v.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("value");

    AxisModel {
        axis_type: if axis_type == "category" {
            AxisType::Category
        } else {
            AxisType::Value
        },
        category_data: Vec::new(),
        min: comp.and_then(|v| v.get("min")).and_then(|v| v.as_f64()),
        max: comp.and_then(|v| v.get("max")).and_then(|v| v.as_f64()),
    }
}

fn compute_value_extent(series: &[SeriesModel]) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for s in series {
        for p in &s.data {
            min = min.min(p.value);
            max = max.max(p.value);
        }
    }
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 100.0);
    }
    if (max - min).abs() < f64::EPSILON {
        if max == 0.0 {
            return (0.0, 1.0);
        }
        let pad = max.abs() * 0.1;
        return (min - pad, max + pad);
    }
    let span = max - min;
    let pad = span * 0.05;
    (min - pad, max + pad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grid_percent() {
        use crate::option::OptionValue;
        use indexmap::IndexMap;

        let mut m = IndexMap::new();
        m.insert("left".into(), OptionValue::String("10%".into()));
        m.insert("right".into(), OptionValue::Number(20.0));
        let grid = parse_grid(Some(&OptionValue::Object(m)), 400.0, 300.0);
        assert!((grid.x - 40.0).abs() < 0.01);
        assert!((grid.width - 340.0).abs() < 0.01);
    }
}
