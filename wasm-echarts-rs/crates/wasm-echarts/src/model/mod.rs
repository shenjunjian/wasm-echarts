//! GlobalModel：从 OptionModel 提取布局、dataset 管线与 series 数据

mod axis;
mod series;

mod coords;

pub use axis::{AxisModel, AxisType};
pub use coords::{
    option_index, parse_coord_kind, polar_data_pair, CalendarSpec, GeoRegion, GeoSpec, MatrixSpec,
    ParallelAxisSpec, ParallelSpec, PolarSpec, RadarSpec, SingleAxisSpec,
};
pub use series::{CoordSysKind, DataPoint, SeriesModel, SeriesType};

use crate::data::{build_series, cell_text, numeric_or_time};
use crate::interaction::DataZoomRange;
use crate::option::{OptionModel, OptionValue};
use crate::utils::{as_components, parse_percent};

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
    pub grids: Vec<GridRect>,
    pub x_axes: Vec<AxisModel>,
    pub y_axes: Vec<AxisModel>,
    pub polars: Vec<PolarSpec>,
    pub radars: Vec<RadarSpec>,
    pub single_axes: Vec<SingleAxisSpec>,
    pub parallels: Vec<ParallelSpec>,
    pub calendars: Vec<CalendarSpec>,
    pub matrices: Vec<MatrixSpec>,
    pub geos: Vec<GeoSpec>,
    pub series: Vec<SeriesModel>,
    pub data_zoom: DataZoomRange,
    /// `None` = 所有 x 轴；`Some` 时只缩放这些轴（读 dataZoom.xAxisIndex）。
    pub data_zoom_x_axes: Option<Vec<usize>>,
}

impl GlobalModel {
    pub fn grid(&self) -> GridRect {
        self.grids.first().copied().unwrap_or(GridRect {
            x: 0.0,
            y: 0.0,
            width: self.width.max(1) as f64,
            height: self.height.max(1) as f64,
        })
    }

    pub fn x_axis(&self) -> &AxisModel {
        self.x_axis_at(0)
    }

    pub fn y_axis(&self) -> &AxisModel {
        self.y_axis_at(0)
    }

    pub fn x_axis_at(&self, index: usize) -> &AxisModel {
        self.x_axes
            .get(index)
            .or_else(|| self.x_axes.first())
            .expect("xAxis")
    }

    pub fn y_axis_at(&self, index: usize) -> &AxisModel {
        self.y_axes
            .get(index)
            .or_else(|| self.y_axes.first())
            .expect("yAxis")
    }

    pub fn grid_at(&self, index: usize) -> GridRect {
        self.grids.get(index).copied().unwrap_or_else(|| self.grid())
    }

    pub fn x_categories(&self) -> &[String] {
        &self.x_axis().category_data
    }

    pub fn from_option(option: &OptionModel, width: u32, height: u32) -> Self {
        Self::from_option_with_zoom(option, width, height, DataZoomRange::default())
    }

    pub fn from_option_with_zoom(
        option: &OptionModel,
        width: u32,
        height: u32,
        data_zoom: DataZoomRange,
    ) -> Self {
        let root = option.root();
        let w = width as f64;
        let h = height as f64;

        let grids = parse_grids(root.get("grid"), w, h);
        let mut x_axes = parse_axes(root.get("xAxis"), AxisType::Category, root, true);
        let mut y_axes = parse_axes(root.get("yAxis"), AxisType::Value, root, false);

        if x_axes.is_empty() {
            x_axes.push(default_axis(AxisType::Category, true, root));
        }
        if y_axes.is_empty() {
            y_axes.push(default_axis(AxisType::Value, false, root));
        }

        let series = build_series(root, &x_axes, |x_idx, _y_idx| {
            let grid_index = x_axes.get(x_idx).map(|a| a.grid_index).unwrap_or(0);
            grids
                .get(grid_index)
                .map(|g| g.width)
                .unwrap_or(w)
                .max(1.0)
        });

        fill_category_from_series(&mut x_axes, &series, true);
        fill_category_from_series(&mut y_axes, &series, false);

        let mut x_axes = apply_axis_extents(x_axes, &series, true, &y_axes);
        let mut y_axes = apply_axis_extents(y_axes, &series, false, &x_axes);
        let _ = (&mut x_axes, &mut y_axes);

        let polars = coords::parse_polars(root, w, h, &series);
        let radars = coords::parse_radars(root, w, h);
        let single_axes = coords::parse_single_axes(root, w, h, &series);
        let parallels = coords::parse_parallels(root, w, h, &series);
        let calendars = coords::parse_calendars(root, w, h);
        let matrices = coords::parse_matrices(root, w, h);
        let geos = coords::parse_geos(root, w, h, |map| crate::maps::lookup_map_regions(map));

        Self {
            width,
            height,
            grids,
            x_axes,
            y_axes,
            polars,
            radars,
            single_axes,
            parallels,
            calendars,
            matrices,
            geos,
            series,
            data_zoom,
            data_zoom_x_axes: parse_data_zoom_x_axes(root),
        }
    }

    pub fn has_cartesian_series(&self) -> bool {
        self.series.iter().any(|s| {
            s.coord_sys.is_cartesian() && s.series_type.is_cartesian_plot()
        })
    }

    pub fn has_pie_series(&self) -> bool {
        self.series.iter().any(|s| s.series_type == SeriesType::Pie)
    }

    pub fn visible_category_range(&self) -> (usize, usize) {
        self.data_zoom.category_window(self.category_count())
    }

    pub fn visible_category_range_of(&self, x_axis_index: usize) -> (usize, usize) {
        let total = self.category_count_of(x_axis_index);
        if let Some(axes) = &self.data_zoom_x_axes {
            if !axes.contains(&x_axis_index) {
                return (0, total);
            }
        }
        self.data_zoom.category_window(total)
    }

    pub fn category_count(&self) -> usize {
        self.category_count_of(0)
    }

    pub fn category_count_of(&self, x_axis_index: usize) -> usize {
        let axis = self.x_axis_at(x_axis_index);
        if !axis.category_data.is_empty() {
            axis.category_data.len()
        } else {
            self.series
                .iter()
                .filter(|s| s.x_axis_index == x_axis_index)
                .map(|s| s.data.len())
                .max()
                .unwrap_or(0)
        }
    }

    pub fn series_grid(&self, series: &SeriesModel) -> GridRect {
        let x = self.x_axis_at(series.x_axis_index);
        self.grid_at(x.grid_index)
    }
}

fn parse_data_zoom_x_axes(root: &OptionValue) -> Option<Vec<usize>> {
    for c in as_components(root.get("dataZoom")) {
        if let Some(v) = c.get("xAxisIndex") {
            return Some(match v {
                OptionValue::Number(n) => vec![*n as usize],
                OptionValue::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_f64().map(|n| n as usize))
                    .collect(),
                _ => continue,
            });
        }
    }
    None
}

fn parse_grids(value: Option<&OptionValue>, width: f64, height: f64) -> Vec<GridRect> {
    let comps = as_components(value);
    if comps.is_empty() {
        vec![parse_one_grid(None, width, height)]
    } else {
        comps
            .into_iter()
            .map(|c| parse_one_grid(Some(c), width, height))
            .collect()
    }
}

fn parse_one_grid(value: Option<&OptionValue>, width: f64, height: f64) -> GridRect {
    let default_left = 60.0;
    let default_right = 20.0;
    let default_top = 40.0;
    let default_bottom = 50.0;

    let map = value.and_then(|v| v.as_object());
    let left = parse_percent(
        map.and_then(|m| m.get("left")),
        width,
        default_left,
    );
    let top = parse_percent(map.and_then(|m| m.get("top")), height, default_top);
    let has_width = map.map(|m| m.contains_key("width")).unwrap_or(false);
    let has_height = map.map(|m| m.contains_key("height")).unwrap_or(false);
    let gw = if has_width {
        parse_percent(map.and_then(|m| m.get("width")), width, width - left - default_right)
    } else {
        let right = parse_percent(
            map.and_then(|m| m.get("right")),
            width,
            default_right,
        );
        (width - left - right).max(1.0)
    };
    let gh = if has_height {
        parse_percent(
            map.and_then(|m| m.get("height")),
            height,
            height - top - default_bottom,
        )
    } else {
        let bottom = parse_percent(
            map.and_then(|m| m.get("bottom")),
            height,
            default_bottom,
        );
        (height - top - bottom).max(1.0)
    };

    GridRect {
        x: left,
        y: top,
        width: gw.max(1.0),
        height: gh.max(1.0),
    }
}

fn parse_axes(
    value: Option<&OptionValue>,
    fallback_type: AxisType,
    root: &OptionValue,
    is_x: bool,
) -> Vec<AxisModel> {
    let comps = as_components(value);
    if comps.is_empty() {
        return vec![default_axis(fallback_type, is_x, root)];
    }
    comps
        .into_iter()
        .map(|c| parse_one_axis(c, fallback_type, is_x))
        .collect()
}

fn default_axis(fallback_type: AxisType, is_x: bool, root: &OptionValue) -> AxisModel {
    let mut axis = AxisModel {
        axis_type: fallback_type,
        category_data: Vec::new(),
        min: None,
        max: None,
        grid_index: 0,
        log_base: 10.0,
    };
    if is_x && axis.axis_type.is_category() {
        axis.category_data = fallback_categories(root);
    }
    axis
}

fn parse_one_axis(comp: &OptionValue, fallback_type: AxisType, _is_x: bool) -> AxisModel {
    let axis_type = comp
        .get("type")
        .and_then(|v| v.as_str())
        .map(|s| AxisType::from_str(s, fallback_type))
        .unwrap_or(fallback_type);
    let categories: Vec<String> = comp
        .get("data")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(cell_text).collect())
        .unwrap_or_default();
    AxisModel {
        axis_type,
        category_data: categories,
        min: parse_axis_bound(comp.get("min"), axis_type),
        max: parse_axis_bound(comp.get("max"), axis_type),
        grid_index: comp
            .get("gridIndex")
            .and_then(|v| v.as_f64())
            .map(|n| n.max(0.0) as usize)
            .unwrap_or(0),
        log_base: comp
            .get("logBase")
            .and_then(|v| v.as_f64())
            .filter(|n| *n > 1.0)
            .unwrap_or(10.0),
    }
}

fn parse_axis_bound(value: Option<&OptionValue>, axis_type: AxisType) -> Option<f64> {
    let value = value?;
    if axis_type == AxisType::Time {
        return numeric_or_time(value);
    }
    value.as_f64().or_else(|| numeric_or_time(value))
}

fn fallback_categories(root: &OptionValue) -> Vec<String> {
    if let Some(series) = root.get("series").and_then(|v| v.as_array()) {
        if let Some(first) = series.first() {
            if let Some(data) = first.get("data").and_then(|v| v.as_array()) {
                return (0..data.len()).map(|i| i.to_string()).collect();
            }
        }
    }
    Vec::new()
}

fn fill_category_from_series(axes: &mut [AxisModel], series: &[SeriesModel], is_x: bool) {
    for (idx, axis) in axes.iter_mut().enumerate() {
        if !axis.axis_type.is_category() || !axis.category_data.is_empty() {
            continue;
        }
        let names: Vec<String> = series
            .iter()
            .filter(|s| {
                if is_x {
                    s.x_axis_index == idx
                } else {
                    s.y_axis_index == idx
                }
            })
            .flat_map(|s| {
                s.data.iter().map(|p| {
                    p.name
                        .clone()
                        .unwrap_or_else(|| p.raw_index.to_string())
                })
            })
            .collect();
        if names.is_empty() {
            continue;
        }
        let mut unique = Vec::new();
        for n in names {
            if !unique.contains(&n) {
                unique.push(n);
            }
        }
        if unique.len() == 1 {
            if let Some(s) = series.iter().find(|s| {
                if is_x {
                    s.x_axis_index == idx
                } else {
                    s.y_axis_index == idx
                }
            }) {
                axis.category_data = s
                    .data
                    .iter()
                    .map(|p| p.name.clone().unwrap_or_else(|| p.raw_index.to_string()))
                    .collect();
            }
        } else {
            axis.category_data = unique;
        }
    }
}

fn apply_axis_extents(
    axes: Vec<AxisModel>,
    series: &[SeriesModel],
    is_x: bool,
    other_axes: &[AxisModel],
) -> Vec<AxisModel> {
    axes.into_iter()
        .enumerate()
        .map(|(idx, axis)| {
            if axis.axis_type.is_category() {
                return axis;
            }
            let (min, max) = if is_x {
                compute_x_extent_for(series, idx, axis.axis_type, other_axes)
            } else {
                compute_y_extent_for(series, idx, axis.axis_type)
            };
            axis.with_data_range((min, max))
        })
        .collect()
}

fn compute_x_extent_for(
    series: &[SeriesModel],
    axis_index: usize,
    axis_type: AxisType,
    y_axes: &[AxisModel],
) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for s in series.iter().filter(|s| s.x_axis_index == axis_index) {
        let y_is_category = y_axes
            .get(s.y_axis_index)
            .map(|a| a.axis_type.is_category())
            .unwrap_or(false);
        for (i, p) in s.data.iter().enumerate() {
            let x = if y_is_category {
                p.stacked_value
            } else {
                p.x_value.unwrap_or(i as f64)
            };
            if axis_type == AxisType::Log && x <= 0.0 {
                continue;
            }
            min = min.min(x);
            max = max.max(x);
        }
    }
    pad_extent(min, max, axis_type)
}

fn compute_y_extent_for(series: &[SeriesModel], axis_index: usize, axis_type: AxisType) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for s in series.iter().filter(|s| s.y_axis_index == axis_index) {
        for p in &s.data {
            for v in point_y_values(s, p) {
                if !v.is_finite() {
                    continue;
                }
                if axis_type == AxisType::Log && v <= 0.0 {
                    continue;
                }
                min = min.min(v);
                max = max.max(v);
            }
        }
    }
    pad_extent(min, max, axis_type)
}

fn point_y_values(series: &SeriesModel, p: &DataPoint) -> Vec<f64> {
    match series.series_type {
        SeriesType::Candlestick | SeriesType::Boxplot => {
            let nums = raw_finite_numbers(&p.raw);
            if nums.len() >= 4 {
                nums
            } else {
                vec![p.stack_base, p.stacked_value, p.value]
            }
        }
        SeriesType::Heatmap => {
            if let Some(arr) = p.raw.as_array() {
                if let Some(v) = arr.get(1).and_then(|v| crate::data::numeric_or_time(v).or_else(|| v.as_f64())) {
                    return vec![v];
                }
            }
            vec![p.value]
        }
        _ => vec![p.stack_base, p.stacked_value, p.value],
    }
}

fn raw_finite_numbers(raw: &OptionValue) -> Vec<f64> {
    let arr = match raw {
        OptionValue::Array(a) => a.as_slice(),
        OptionValue::Object(obj) => match obj.get("value") {
            Some(OptionValue::Array(a)) => a.as_slice(),
            _ => return Vec::new(),
        },
        _ => return Vec::new(),
    };
    arr.iter()
        .filter_map(|v| crate::data::numeric_or_time(v).or_else(|| v.as_f64()))
        .filter(|n| n.is_finite())
        .collect()
}

fn pad_extent(min: f64, max: f64, axis_type: AxisType) -> (f64, f64) {
    if !min.is_finite() || !max.is_finite() {
        return if axis_type == AxisType::Log {
            (1.0, 10.0)
        } else {
            (0.0, 100.0)
        };
    }
    if axis_type == AxisType::Log {
        let min = min.max(f64::MIN_POSITIVE);
        let max = max.max(min);
        if (max / min).ln().abs() < 1e-9 {
            return (min / 10.0, max * 10.0);
        }
        return (min, max);
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
    use crate::option::{OptionModel, SetOptionFlags};
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
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
    fn parse_grid_percent() {
        let mut m = IndexMap::new();
        m.insert("left".into(), OptionValue::String("10%".into()));
        m.insert("right".into(), OptionValue::Number(20.0));
        let grids = parse_grids(Some(&OptionValue::Object(m)), 400.0, 300.0);
        assert!((grids[0].x - 40.0).abs() < 0.01);
        assert!((grids[0].width - 340.0).abs() < 0.01);
    }

    #[test]
    fn dataset_fills_series() {
        let model = apply(obj(vec![
            (
                "dataset",
                obj(vec![(
                    "source",
                    OptionValue::Array(vec![
                        OptionValue::Array(vec![
                            OptionValue::String("product".into()),
                            OptionValue::String("2015".into()),
                        ]),
                        OptionValue::Array(vec![
                            OptionValue::String("A".into()),
                            OptionValue::Number(10.0),
                        ]),
                        OptionValue::Array(vec![
                            OptionValue::String("B".into()),
                            OptionValue::Number(20.0),
                        ]),
                    ]),
                )]),
            ),
            ("xAxis", obj(vec![("type", OptionValue::String("category".into()))])),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "series",
                OptionValue::Array(vec![obj(vec![("type", OptionValue::String("bar".into()))])]),
            ),
        ]));
        assert_eq!(model.series[0].data.len(), 2);
        assert_eq!(model.series[0].data[0].value, 10.0);
        assert_eq!(model.x_categories(), &["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn stack_offsets_bar() {
        let model = apply(obj(vec![
            (
                "xAxis",
                obj(vec![
                    ("type", OptionValue::String("category".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::String("A".into())]),
                    ),
                ]),
            ),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "series",
                OptionValue::Array(vec![
                    obj(vec![
                        ("type", OptionValue::String("bar".into())),
                        ("stack", OptionValue::String("total".into())),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::Number(10.0)]),
                        ),
                    ]),
                    obj(vec![
                        ("type", OptionValue::String("bar".into())),
                        ("stack", OptionValue::String("total".into())),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::Number(20.0)]),
                        ),
                    ]),
                ]),
            ),
        ]));
        assert!((model.series[1].data[0].stacked_value - 30.0).abs() < 1e-9);
        assert!((model.series[1].data[0].stack_base - 10.0).abs() < 1e-9);
    }

    #[test]
    fn multi_grid_and_axis_index() {
        let model = apply(obj(vec![
            (
                "grid",
                OptionValue::Array(vec![
                    obj(vec![
                        ("left", OptionValue::Number(10.0)),
                        ("right", OptionValue::Number(220.0)),
                    ]),
                    obj(vec![
                        ("left", OptionValue::Number(220.0)),
                        ("right", OptionValue::Number(10.0)),
                    ]),
                ]),
            ),
            (
                "xAxis",
                OptionValue::Array(vec![
                    obj(vec![
                        ("type", OptionValue::String("category".into())),
                        ("gridIndex", OptionValue::Number(0.0)),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::String("A".into())]),
                        ),
                    ]),
                    obj(vec![
                        ("type", OptionValue::String("category".into())),
                        ("gridIndex", OptionValue::Number(1.0)),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::String("B".into())]),
                        ),
                    ]),
                ]),
            ),
            (
                "yAxis",
                OptionValue::Array(vec![
                    obj(vec![("gridIndex", OptionValue::Number(0.0))]),
                    obj(vec![("gridIndex", OptionValue::Number(1.0))]),
                ]),
            ),
            (
                "series",
                OptionValue::Array(vec![
                    obj(vec![
                        ("type", OptionValue::String("bar".into())),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::Number(1.0)]),
                        ),
                    ]),
                    obj(vec![
                        ("type", OptionValue::String("line".into())),
                        ("xAxisIndex", OptionValue::Number(1.0)),
                        ("yAxisIndex", OptionValue::Number(1.0)),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::Number(2.0)]),
                        ),
                    ]),
                ]),
            ),
        ]));
        assert_eq!(model.grids.len(), 2);
        assert_eq!(model.series[1].x_axis_index, 1);
        assert!(model.grid_at(1).x > model.grid_at(0).x);
    }

    #[test]
    fn time_and_log_axes() {
        let model = apply(obj(vec![
            (
                "xAxis",
                obj(vec![("type", OptionValue::String("time".into()))]),
            ),
            (
                "yAxis",
                obj(vec![("type", OptionValue::String("log".into()))]),
            ),
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
                                OptionValue::String("2020-01-03".into()),
                                OptionValue::Number(100.0),
                            ]),
                        ]),
                    ),
                ])]),
            ),
        ]));
        assert_eq!(model.x_axis().axis_type, AxisType::Time);
        assert_eq!(model.y_axis().axis_type, AxisType::Log);
        assert!(model.series[0].data[0].x_value.unwrap() < model.series[0].data[1].x_value.unwrap());
        assert!(model.y_axis().value_min() > 0.0);
    }

    #[test]
    fn filter_transform_dataset() {
        let model = apply(obj(vec![
            (
                "dataset",
                OptionValue::Array(vec![
                    obj(vec![(
                        "source",
                        OptionValue::Array(vec![
                            OptionValue::Array(vec![
                                OptionValue::String("Year".into()),
                                OptionValue::String("V".into()),
                            ]),
                            OptionValue::Array(vec![
                                OptionValue::Number(1950.0),
                                OptionValue::Number(1.0),
                            ]),
                            OptionValue::Array(vec![
                                OptionValue::Number(2000.0),
                                OptionValue::Number(2.0),
                            ]),
                        ]),
                    )]),
                    obj(vec![(
                        "transform",
                        obj(vec![
                            ("type", OptionValue::String("filter".into())),
                            (
                                "config",
                                obj(vec![
                                    ("dimension", OptionValue::String("Year".into())),
                                    (">", OptionValue::Number(1960.0)),
                                ]),
                            ),
                        ]),
                    )]),
                ]),
            ),
            ("xAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("line".into())),
                    ("datasetIndex", OptionValue::Number(1.0)),
                    (
                        "encode",
                        obj(vec![
                            ("x", OptionValue::String("Year".into())),
                            ("y", OptionValue::String("V".into())),
                        ]),
                    ),
                ])]),
            ),
        ]));
        assert_eq!(model.series[0].data.len(), 1);
        assert_eq!(model.series[0].data[0].value, 2.0);
    }

    #[test]
    fn data_zoom_x_axis_index_skips_other_axes() {
        let cats: Vec<OptionValue> = (0..10)
            .map(|i| OptionValue::String(i.to_string()))
            .collect();
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "xAxis",
                    OptionValue::Array(vec![
                        obj(vec![
                            ("type", OptionValue::String("category".into())),
                            ("data", OptionValue::Array(cats.clone())),
                        ]),
                        obj(vec![
                            ("type", OptionValue::String("category".into())),
                            ("data", OptionValue::Array(cats)),
                        ]),
                    ]),
                ),
                (
                    "yAxis",
                    OptionValue::Array(vec![
                        obj(vec![("type", OptionValue::String("value".into()))]),
                        obj(vec![("type", OptionValue::String("value".into()))]),
                    ]),
                ),
                (
                    "dataZoom",
                    obj(vec![
                        ("type", OptionValue::String("inside".into())),
                        ("start", OptionValue::Number(0.0)),
                        ("end", OptionValue::Number(50.0)),
                        ("xAxisIndex", OptionValue::Number(0.0)),
                    ]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![
                        obj(vec![
                            ("type", OptionValue::String("line".into())),
                            ("xAxisIndex", OptionValue::Number(0.0)),
                            ("data", OptionValue::Array(vec![OptionValue::Number(1.0)])),
                        ]),
                        obj(vec![
                            ("type", OptionValue::String("line".into())),
                            ("xAxisIndex", OptionValue::Number(1.0)),
                            ("data", OptionValue::Array(vec![OptionValue::Number(1.0)])),
                        ]),
                    ]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                ..Default::default()
            },
        );
        let model = GlobalModel::from_option_with_zoom(
            &option,
            400,
            300,
            crate::interaction::DataZoomRange::clamped(0.0, 50.0),
        );
        assert_eq!(model.data_zoom_x_axes.as_deref(), Some(&[0][..]));
        assert_eq!(model.visible_category_range_of(0), (0, 5));
        assert_eq!(model.visible_category_range_of(1), (0, 10));
    }
}
