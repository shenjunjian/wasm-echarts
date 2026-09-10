//! 第 5 波坐标系组件：从 option 抽出 polar / radar / single / parallel / calendar / matrix / geo

use crate::data::numeric_or_time;
use crate::model::axis::{AxisModel, AxisType};
use crate::model::series::{CoordSysKind, SeriesModel};
use crate::model::GridRect;
use crate::option::OptionValue;
use crate::utils::{as_components, parse_percent};

#[derive(Debug, Clone)]
pub struct PolarSpec {
    pub center_x: f64,
    pub center_y: f64,
    pub r0: f64,
    pub r: f64,
    pub angle_axis: AxisModel,
    pub radius_axis: AxisModel,
    pub start_angle: f64,
    pub clockwise: bool,
}

#[derive(Debug, Clone)]
pub struct RadarIndicator {
    pub name: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct RadarSpec {
    pub center_x: f64,
    pub center_y: f64,
    pub r0: f64,
    pub r: f64,
    pub start_angle: f64,
    pub clockwise: bool,
    pub shape_circle: bool,
    pub split_number: usize,
    pub indicators: Vec<RadarIndicator>,
}

#[derive(Debug, Clone)]
pub struct SingleAxisSpec {
    pub rect: GridRect,
    pub axis: AxisModel,
    pub horizontal: bool,
}

#[derive(Debug, Clone)]
pub struct ParallelAxisSpec {
    pub dim: usize,
    pub name: String,
    pub axis: AxisModel,
}

#[derive(Debug, Clone)]
pub struct ParallelSpec {
    pub rect: GridRect,
    pub horizontal: bool,
    pub axes: Vec<ParallelAxisSpec>,
}

#[derive(Debug, Clone)]
pub struct CalendarSpec {
    pub rect: GridRect,
    pub start_ms: f64,
    pub end_ms: f64,
    pub cell_w: f64,
    pub cell_h: f64,
    pub first_day: i32,
    pub horizontal: bool,
}

#[derive(Debug, Clone)]
pub struct MatrixSpec {
    pub rect: GridRect,
    pub x_data: Vec<String>,
    pub y_data: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GeoRegion {
    pub name: String,
    pub polygons: Vec<Vec<(f64, f64)>>,
    pub center: Option<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct GeoSpec {
    pub rect: GridRect,
    pub map: String,
    pub regions: Vec<GeoRegion>,
    pub invert_lng: bool,
    pub aspect_scale: f64,
}

pub fn option_index(comp: &OptionValue, key: &str) -> usize {
    comp.get(key)
        .and_then(|v| v.as_f64())
        .map(|n| n.max(0.0) as usize)
        .unwrap_or(0)
}

pub fn parse_coord_kind(series: &OptionValue, series_type: &str) -> CoordSysKind {
    if series_type == "pie" {
        return CoordSysKind::None;
    }
    CoordSysKind::from_option(series.get("coordinateSystem").and_then(|v| v.as_str()))
}

pub fn parse_layout_rect(
    value: Option<&OptionValue>,
    width: f64,
    height: f64,
    default_left: f64,
    default_top: f64,
    default_right: f64,
    default_bottom: f64,
) -> GridRect {
    let map = value.and_then(|v| v.as_object());
    let left = parse_percent(map.and_then(|m| m.get("left")), width, default_left);
    let top = parse_percent(map.and_then(|m| m.get("top")), height, default_top);
    let has_width = map.map(|m| m.contains_key("width")).unwrap_or(false);
    let has_height = map.map(|m| m.contains_key("height")).unwrap_or(false);
    let gw = if has_width {
        parse_percent(
            map.and_then(|m| m.get("width")),
            width,
            (width - left - default_right).max(1.0),
        )
    } else {
        let right = parse_percent(map.and_then(|m| m.get("right")), width, default_right);
        (width - left - right).max(1.0)
    };
    let gh = if has_height {
        parse_percent(
            map.and_then(|m| m.get("height")),
            height,
            (height - top - default_bottom).max(1.0),
        )
    } else {
        let bottom = parse_percent(map.and_then(|m| m.get("bottom")), height, default_bottom);
        (height - top - bottom).max(1.0)
    };
    GridRect {
        x: left,
        y: top,
        width: gw.max(1.0),
        height: gh.max(1.0),
    }
}

fn parse_center(value: Option<&OptionValue>, width: f64, height: f64) -> (f64, f64) {
    match value {
        Some(OptionValue::Array(arr)) => (
            parse_percent(arr.first(), width, width * 0.5),
            parse_percent(arr.get(1), height, height * 0.5),
        ),
        Some(v) => {
            let c = parse_percent(Some(v), width.min(height), width * 0.5);
            (c, c)
        }
        None => (width * 0.5, height * 0.5),
    }
}

fn parse_radius_pair(value: Option<&OptionValue>, size: f64) -> (f64, f64) {
    match value {
        Some(OptionValue::Array(arr)) => (
            parse_percent(arr.first(), size, 0.0),
            parse_percent(arr.get(1), size, size * 0.8),
        ),
        Some(v) => (0.0, parse_percent(Some(v), size, size * 0.8)),
        None => (0.0, size * 0.8),
    }
}

fn parse_axis_from(comp: Option<&OptionValue>, fallback: AxisType) -> AxisModel {
    let Some(comp) = comp else {
        return AxisModel {
            axis_type: fallback,
            category_data: Vec::new(),
            min: None,
            max: None,
            grid_index: 0,
            log_base: 10.0,
        };
    };
    let axis_type = comp
        .get("type")
        .and_then(|v| v.as_str())
        .map(|s| AxisType::from_str(s, fallback))
        .unwrap_or(fallback);
    let categories: Vec<String> = comp
        .get("data")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(crate::data::cell_text).collect())
        .unwrap_or_default();
    AxisModel {
        axis_type,
        category_data: categories,
        min: parse_bound(comp.get("min"), axis_type),
        max: parse_bound(comp.get("max"), axis_type),
        grid_index: 0,
        log_base: comp
            .get("logBase")
            .and_then(|v| v.as_f64())
            .filter(|n| *n > 1.0)
            .unwrap_or(10.0),
    }
}

fn parse_bound(value: Option<&OptionValue>, axis_type: AxisType) -> Option<f64> {
    let value = value?;
    if axis_type == AxisType::Time {
        return numeric_or_time(value);
    }
    value.as_f64().or_else(|| numeric_or_time(value))
}

fn apply_extent(axis: AxisModel, min: f64, max: f64) -> AxisModel {
    if axis.axis_type.is_category() {
        axis
    } else {
        axis.with_data_range((min, max))
    }
}

fn pad_extent(min: f64, max: f64) -> (f64, f64) {
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 1.0);
    }
    if (max - min).abs() < f64::EPSILON {
        let pad = if min.abs() < 1.0 { 1.0 } else { min.abs() * 0.1 };
        return (min - pad, max + pad);
    }
    (min, max)
}

fn series_for_polar<'a>(series: &'a [SeriesModel], idx: usize) -> impl Iterator<Item = &'a SeriesModel> {
    series
        .iter()
        .filter(move |s| s.coord_sys == CoordSysKind::Polar && s.polar_index == idx)
}

pub fn parse_polars(root: &OptionValue, width: f64, height: f64, series: &[SeriesModel]) -> Vec<PolarSpec> {
    let comps = as_components(root.get("polar"));
    if comps.is_empty()
        && !series
            .iter()
            .any(|s| s.coord_sys == CoordSysKind::Polar)
    {
        return Vec::new();
    }
    let list = if comps.is_empty() { vec![None] } else { comps.into_iter().map(Some).collect() };
    let angle_comps = as_components(root.get("angleAxis"));
    let radius_comps = as_components(root.get("radiusAxis"));
    list.into_iter()
        .enumerate()
        .map(|(i, polar)| {
            let (cx, cy) = parse_center(polar.and_then(|p| p.get("center")), width, height);
            let size = width.min(height) / 2.0;
            let (r0, r) = parse_radius_pair(polar.and_then(|p| p.get("radius")), size);
            let angle_opt = angle_comps.iter().copied().find(|a| option_index(a, "polarIndex") == i);
            let radius_opt = radius_comps.iter().copied().find(|a| option_index(a, "polarIndex") == i);
            let angle_axis = parse_axis_from(angle_opt.or(angle_comps.first().copied()), AxisType::Category);
            let radius_axis = parse_axis_from(radius_opt.or(radius_comps.first().copied()), AxisType::Value);
            let start_angle = angle_opt
                .and_then(|a| a.get("startAngle"))
                .and_then(|v| v.as_f64())
                .unwrap_or(90.0);
            let clockwise = angle_opt
                .and_then(|a| a.get("clockwise"))
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let mut angle_min = f64::INFINITY;
            let mut angle_max = f64::NEG_INFINITY;
            let mut radius_min = f64::INFINITY;
            let mut radius_max = f64::NEG_INFINITY;
            for s in series_for_polar(series, i) {
                for (di, p) in s.data.iter().enumerate() {
                    let (rv, av) = polar_data_pair(p, di, angle_axis.axis_type.is_category());
                    if rv.is_finite() {
                        radius_min = radius_min.min(rv.min(p.stack_base));
                        radius_max = radius_max.max(rv.max(p.stacked_value));
                    }
                    if av.is_finite() {
                        angle_min = angle_min.min(av);
                        angle_max = angle_max.max(av);
                    }
                }
            }
            PolarSpec {
                center_x: cx,
                center_y: cy,
                r0,
                r,
                angle_axis: apply_extent(angle_axis, pad_extent(angle_min, angle_max).0, pad_extent(angle_min, angle_max).1),
                radius_axis: apply_extent(
                    radius_axis,
                    pad_extent(radius_min, radius_max).0,
                    pad_extent(radius_min, radius_max).1,
                ),
                start_angle,
                clockwise,
            }
        })
        .collect()
}

pub fn polar_data_pair(p: &crate::model::DataPoint, index: usize, angle_is_category: bool) -> (f64, f64) {
    if angle_is_category {
        (p.stacked_value, index as f64)
    } else {
        (p.x_value.unwrap_or(p.value), p.value)
    }
}

pub fn parse_radars(root: &OptionValue, width: f64, height: f64) -> Vec<RadarSpec> {
    as_components(root.get("radar"))
        .into_iter()
        .map(|comp| {
            let (cx, cy) = parse_center(comp.get("center"), width, height);
            let size = width.min(height) / 2.0;
            let (r0, r) = parse_radius_pair(comp.get("radius"), size);
            let indicators = as_components(comp.get("indicator"))
                .into_iter()
                .map(|ind| RadarIndicator {
                    name: ind
                        .get("name")
                        .or_else(|| ind.get("text"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    min: ind.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    max: ind
                        .get("max")
                        .and_then(|v| v.as_f64())
                        .filter(|n| n.is_finite())
                        .unwrap_or(100.0),
                })
                .collect();
            RadarSpec {
                center_x: cx,
                center_y: cy,
                r0,
                r,
                start_angle: comp.get("startAngle").and_then(|v| v.as_f64()).unwrap_or(90.0),
                clockwise: comp.get("clockwise").and_then(|v| v.as_bool()).unwrap_or(false),
                shape_circle: comp.get("shape").and_then(|v| v.as_str()) == Some("circle"),
                split_number: comp
                    .get("splitNumber")
                    .and_then(|v| v.as_f64())
                    .map(|n| n.max(1.0) as usize)
                    .unwrap_or(5),
                indicators,
            }
        })
        .collect()
}

pub fn parse_single_axes(root: &OptionValue, width: f64, height: f64, series: &[SeriesModel]) -> Vec<SingleAxisSpec> {
    as_components(root.get("singleAxis"))
        .into_iter()
        .enumerate()
        .map(|(i, comp)| {
            let rect = parse_layout_rect(Some(comp), width, height, width * 0.05, height * 0.05, width * 0.05, height * 0.05);
            let mut axis = parse_axis_from(Some(comp), AxisType::Value);
            let horizontal = match comp.get("orient").and_then(|v| v.as_str()) {
                Some("vertical") => false,
                _ => true,
            };
            let mut min = f64::INFINITY;
            let mut max = f64::NEG_INFINITY;
            for s in series.iter().filter(|s| s.coord_sys == CoordSysKind::Single && s.single_axis_index == i)
            {
                for (di, p) in s.data.iter().enumerate() {
                    let v = p.x_value.unwrap_or(p.value);
                    let v = if v.is_finite() { v } else { di as f64 };
                    min = min.min(v);
                    max = max.max(v);
                }
            }
            axis = apply_extent(axis, pad_extent(min, max).0, pad_extent(min, max).1);
            SingleAxisSpec {
                rect,
                axis,
                horizontal,
            }
        })
        .collect()
}

pub fn parse_parallels(root: &OptionValue, width: f64, height: f64, series: &[SeriesModel]) -> Vec<ParallelSpec> {
    let comps = as_components(root.get("parallel"));
    if comps.is_empty() && root.get("parallelAxis").is_none() {
        return Vec::new();
    }
    let list = if comps.is_empty() { vec![None] } else { comps.into_iter().map(Some).collect() };
    list.into_iter()
        .enumerate()
        .map(|(i, parallel)| {
            let rect = parse_layout_rect(parallel, width, height, width * 0.08, height * 0.12, width * 0.08, height * 0.1);
            let horizontal = parallel
                .and_then(|p| p.get("layout"))
                .and_then(|v| v.as_str())
                != Some("vertical");
            let mut axes: Vec<ParallelAxisSpec> = as_components(root.get("parallelAxis"))
                .into_iter()
                .filter(|a| option_index(a, "parallelIndex") == i)
                .map(|a| {
                    let dim = option_index(a, "dim");
                    ParallelAxisSpec {
                        dim,
                        name: a.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        axis: parse_axis_from(Some(a), AxisType::Value),
                    }
                })
                .collect();
            if axes.is_empty() {
                let dims = series
                    .iter()
                    .filter(|s| s.coord_sys == CoordSysKind::Parallel && s.parallel_index == i)
                    .filter_map(|s| s.data.first())
                    .filter_map(|p| p.raw.as_array().map(|a| a.len()))
                    .max()
                    .unwrap_or(0);
                for dim in 0..dims {
                    axes.push(ParallelAxisSpec {
                        dim,
                        name: format!("dim{dim}"),
                        axis: parse_axis_from(None, AxisType::Value),
                    });
                }
            }
            axes.sort_by_key(|a| a.dim);
            for axis in &mut axes {
                let mut min = f64::INFINITY;
                let mut max = f64::NEG_INFINITY;
                for s in series.iter().filter(|s| s.coord_sys == CoordSysKind::Parallel && s.parallel_index == i)
                {
                    for p in &s.data {
                        if let Some(v) = p.raw.as_array().and_then(|a| a.get(axis.dim)).and_then(numeric_or_time)
                        {
                            min = min.min(v);
                            max = max.max(v);
                        }
                    }
                }
                axis.axis = apply_extent(axis.axis.clone(), pad_extent(min, max).0, pad_extent(min, max).1);
            }
            ParallelSpec {
                rect,
                horizontal,
                axes,
            }
        })
        .collect()
}

pub fn parse_calendars(root: &OptionValue, width: f64, height: f64) -> Vec<CalendarSpec> {
    as_components(root.get("calendar"))
        .into_iter()
        .map(|comp| {
            let rect = parse_layout_rect(Some(comp), width, height, 80.0, 60.0, 20.0, 20.0);
            let (start_ms, end_ms) = parse_calendar_range(comp.get("range"));
            let first_day = comp
                .get("dayLabel")
                .and_then(|v| v.get("firstDay"))
                .and_then(|v| v.as_f64())
                .map(|n| n as i32)
                .unwrap_or(0)
                .clamp(0, 6);
            let horizontal = comp.get("orient").and_then(|v| v.as_str()) != Some("vertical");
            let days = ((end_ms - start_ms) / 86_400_000.0).ceil().max(1.0);
            let weeks = (days / 7.0).ceil().max(1.0);
            let (cell_w, cell_h) = match comp.get("cellSize") {
                Some(OptionValue::Number(n)) => (*n, *n),
                Some(OptionValue::Array(arr)) => (
                    arr.first().and_then(|v| v.as_f64()).unwrap_or(rect.width / weeks),
                    arr.get(1).and_then(|v| v.as_f64()).unwrap_or(rect.height / 7.0),
                ),
                _ => {
                    if horizontal {
                        (rect.width / weeks, rect.height / 7.0)
                    } else {
                        (rect.width / 7.0, rect.height / weeks)
                    }
                }
            };
            CalendarSpec {
                rect,
                start_ms,
                end_ms,
                cell_w: cell_w.max(1.0),
                cell_h: cell_h.max(1.0),
                first_day,
                horizontal,
            }
        })
        .collect()
}

fn parse_calendar_range(value: Option<&OptionValue>) -> (f64, f64) {
    match value {
        Some(OptionValue::Array(arr)) if arr.len() >= 2 => {
            let start = year_start(&arr[0]).or_else(|| numeric_or_time(&arr[0])).unwrap_or(0.0);
            let end = year_end(&arr[1]).or_else(|| numeric_or_time(&arr[1])).unwrap_or(start + 86_400_000.0);
            (start, end)
        }
        Some(v) => {
            let start = year_start(v).or_else(|| numeric_or_time(v)).unwrap_or(0.0);
            let end = year_end(v).unwrap_or(start + 364.0 * 86_400_000.0);
            (start, end)
        }
        None => (0.0, 364.0 * 86_400_000.0),
    }
}

fn year_token(v: &OptionValue) -> Option<i32> {
    if let Some(s) = v.as_str() {
        let t = s.trim();
        if t.len() == 4 && t.chars().all(|c| c.is_ascii_digit()) {
            return t.parse().ok();
        }
    }
    if let Some(n) = v.as_f64() {
        if (1900.0..2100.0).contains(&n) && (n - n.round()).abs() < 1e-9 {
            return Some(n as i32);
        }
    }
    None
}

fn year_start(v: &OptionValue) -> Option<f64> {
    let year = year_token(v)?;
    crate::data::parse_time_value(&OptionValue::String(format!("{year:04}-01-01")))
}

fn year_end(v: &OptionValue) -> Option<f64> {
    let year = year_token(v)?;
    crate::data::parse_time_value(&OptionValue::String(format!("{year:04}-12-31")))
}

pub fn parse_matrices(root: &OptionValue, width: f64, height: f64) -> Vec<MatrixSpec> {
    as_components(root.get("matrix"))
        .into_iter()
        .map(|comp| {
            let rect = parse_layout_rect(Some(comp), width, height, 80.0, 60.0, 20.0, 40.0);
            let x_data = dim_data(comp.get("x"));
            let y_data = dim_data(comp.get("y"));
            MatrixSpec { rect, x_data, y_data }
        })
        .collect()
}

fn dim_data(dim: Option<&OptionValue>) -> Vec<String> {
    dim.and_then(|d| d.get("data"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(crate::data::cell_text).collect())
        .unwrap_or_default()
}

pub fn parse_geos(root: &OptionValue, width: f64, height: f64, lookup: impl Fn(&str) -> Vec<GeoRegion>) -> Vec<GeoSpec> {
    as_components(root.get("geo"))
        .into_iter()
        .map(|comp| {
            let rect = parse_layout_rect(Some(comp), width, height, 0.0, 0.0, 0.0, 0.0);
            let map = comp
                .get("map")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let regions = lookup(&map);
            GeoSpec {
                rect,
                map,
                regions,
                invert_lng: true,
                aspect_scale: 0.75,
            }
        })
        .collect()
}
