//! series.markPoint / markLine / markArea（终态绘制）

use rust_zrender::{
    ChildRef, DisplayableProps, FillStrokeStyle, LineShape, Path, PathStyle, RectShape, Shape,
    TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_plain_label;
use crate::chart::symbol::{add_symbol, SymbolSpec};
use crate::coord::Cartesian2D;
use crate::model::{GlobalModel, SeriesModel};
use crate::option::{OptionModel, OptionValue};
use crate::utils::{format_axis_number, parse_percent};
use crate::visual::VisualContext;

pub fn render_marks(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let Some(series_opt) = visual.series_option(series.index) else {
        return;
    };
    let coord = Cartesian2D::for_series(model, series);
    if let Some(mp) = series_opt.get("markPoint") {
        render_mark_point(zr, group, model, series, mp, &coord);
    }
    if let Some(ml) = series_opt.get("markLine") {
        render_mark_line(zr, group, model, option, series, ml, &coord);
    }
    if let Some(ma) = series_opt.get("markArea") {
        render_mark_area(zr, group, model, series, ma, &coord);
    }
}

fn render_mark_point(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    series: &SeriesModel,
    mark: &OptionValue,
    coord: &Cartesian2D,
) {
    if mark.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }
    let symbol = mark
        .get("symbol")
        .and_then(|v| v.as_str())
        .unwrap_or("pin")
        .to_string();
    let size = mark
        .get("symbolSize")
        .and_then(|v| v.as_f64())
        .unwrap_or(50.0);
    let color = mark
        .get("itemStyle")
        .and_then(|s| s.get("color"))
        .and_then(|v| v.as_str())
        .unwrap_or("#c23531")
        .to_string();
    let show_label = mark
        .get("label")
        .and_then(|l| l.get("show"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    for item in mark_items(mark) {
        let Some((x, y, value_text)) = resolve_point(series, model, item, coord) else {
            continue;
        };
        add_symbol(
            zr,
            group,
            &SymbolSpec {
                kind: item
                    .get("symbol")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&symbol)
                    .to_string(),
                size: item
                    .get("symbolSize")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(size),
                cx: x,
                cy: y,
                color: color.clone(),
                series_index: series.index,
                data_index: 0,
                attach_states: false,
            },
        );
        if show_label {
            let label = item
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or(value_text);
            add_plain_label(
                zr,
                group,
                label,
                x,
                y - 8.0,
                TextAlign::Center,
                TextBaseline::Bottom,
                "#fff",
                12.0,
            );
        }
    }
}

fn render_mark_line(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    _option: &OptionModel,
    series: &SeriesModel,
    mark: &OptionValue,
    coord: &Cartesian2D,
) {
    if mark.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }
    let color = mark
        .get("lineStyle")
        .and_then(|s| s.get("color"))
        .and_then(|v| v.as_str())
        .unwrap_or("#c23531");
    let width = mark
        .get("lineStyle")
        .and_then(|s| s.get("width"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as f32;
    let show_label = mark
        .get("label")
        .and_then(|l| l.get("show"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    for item in mark_items(mark) {
        if let OptionValue::Array(pair) = item {
            if pair.len() >= 2 {
                let p0 = resolve_point(series, model, &pair[0], coord);
                let p1 = resolve_point(series, model, &pair[1], coord);
                if let (Some((x1, y1, _)), Some((x2, y2, _))) = (p0, p1) {
                    add_line(zr, group, x1, y1, x2, y2, color, width);
                    if show_label {
                        let name = pair[1]
                            .get("name")
                            .or_else(|| pair[1].get("label").and_then(|l| l.get("formatter")))
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if !name.is_empty() {
                            add_plain_label(
                                zr,
                                group,
                                name.to_string(),
                                x2,
                                y2,
                                TextAlign::Left,
                                TextBaseline::Bottom,
                                color,
                                12.0,
                            );
                        }
                    }
                }
            }
            continue;
        }
        if let Some((x1, y1, x2, y2, label)) = resolve_line(series, model, item, coord) {
            add_line(zr, group, x1, y1, x2, y2, color, width);
            if show_label && !label.is_empty() {
                add_plain_label(
                    zr,
                    group,
                    label,
                    (x1 + x2) / 2.0,
                    y1.min(y2) - 2.0,
                    TextAlign::Center,
                    TextBaseline::Bottom,
                    color,
                    12.0,
                );
            }
        }
    }
}

fn render_mark_area(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    series: &SeriesModel,
    mark: &OptionValue,
    coord: &Cartesian2D,
) {
    if mark.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }
    let fill = mark
        .get("itemStyle")
        .and_then(|s| s.get("color"))
        .and_then(|v| v.as_str())
        .unwrap_or("rgba(194,53,49,0.15)");
    for item in mark_items(mark) {
        let pair = match item {
            OptionValue::Array(arr) if arr.len() >= 2 => arr,
            _ => continue,
        };
        let p0 = resolve_point(series, model, &pair[0], coord);
        let p1 = resolve_point(series, model, &pair[1], coord);
        let Some((x0, y0, _)) = p0 else { continue };
        let Some((x1, y1, _)) = p1 else { continue };
        let x = x0.min(x1);
        let y = y0.min(y1);
        let w = (x0 - x1).abs().max(1.0);
        let h = (y0 - y1).abs().max(1.0);
        let rect = zr.storage.create_path(
            Path::new(
                Shape::Rect(RectShape {
                    x,
                    y,
                    width: w,
                    height: h,
                    ..Default::default()
                }),
                PathStyle {
                    fill: FillStrokeStyle::color(fill),
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                z: 8.0,
                ..Default::default()
            }),
        );
        zr.storage.group_add_child(group, ChildRef::Path(rect));
    }
}

fn mark_items(mark: &OptionValue) -> &[OptionValue] {
    mark.get("data")
        .and_then(|v| v.as_array())
        .unwrap_or(&[])
}

fn resolve_point(
    series: &SeriesModel,
    model: &GlobalModel,
    item: &OptionValue,
    coord: &Cartesian2D,
) -> Option<(f64, f64, String)> {
    let g = coord.grid();
    let w = model.width as f64;
    let h = model.height as f64;
    if let (Some(px), Some(py)) = (item.get("x"), item.get("y")) {
        if !looks_axis_token(px) && !looks_axis_token(py) {
            let x = parse_percent(Some(px), w, g.x);
            let y = parse_percent(Some(py), h, g.y);
            return Some((x, y, String::new()));
        }
    }
    if let Some(arr) = item.get("coord").and_then(|v| v.as_array()) {
        if arr.len() >= 2 {
            let xv = axis_token_to_value(&arr[0], true, series, model);
            let yv = axis_token_to_value(&arr[1], false, series, model);
            let (x, y) = coord.value_to_point(xv, yv);
            return Some((x, y, format_axis_number(yv)));
        }
    }
    if let Some(ty) = item.get("type").and_then(|v| v.as_str()) {
        let (idx, val) = stat_point(series, ty)?;
        let (x, y) = coord.point_for(idx, series.data.get(idx).and_then(|p| p.x_value), val);
        return Some((x, y, format_axis_number(val)));
    }
    let xv = item
        .get("xAxis")
        .map(|v| axis_token_to_value(v, true, series, model));
    let yv = item
        .get("yAxis")
        .map(|v| axis_token_to_value(v, false, series, model));
    match (xv, yv) {
        (Some(x_val), Some(y_val)) => {
            let (x, y) = coord.value_to_point(x_val, y_val);
            Some((x, y, format_axis_number(y_val)))
        }
        (Some(x_val), None) => {
            let x = coord.x_value_to_pixel(x_val);
            Some((x, g.y + g.height / 2.0, format_axis_number(x_val)))
        }
        (None, Some(y_val)) => {
            let y = coord.y_value_to_pixel(y_val);
            Some((g.x + g.width / 2.0, y, format_axis_number(y_val)))
        }
        _ => None,
    }
}

fn resolve_line(
    series: &SeriesModel,
    model: &GlobalModel,
    item: &OptionValue,
    coord: &Cartesian2D,
) -> Option<(f64, f64, f64, f64, String)> {
    let g = coord.grid();
    if let Some(ty) = item.get("type").and_then(|v| v.as_str()) {
        let (_, val) = stat_point(series, ty)?;
        let y = coord.y_value_to_pixel(val);
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(ty)
            .to_string();
        return Some((g.x, y, g.x + g.width, y, name));
    }
    if let Some(xv) = item.get("xAxis") {
        let x = coord.x_value_to_pixel(axis_token_to_value(xv, true, series, model));
        return Some((x, g.y, x, g.y + g.height, String::new()));
    }
    if let Some(yv) = item.get("yAxis") {
        let y = coord.y_value_to_pixel(axis_token_to_value(yv, false, series, model));
        return Some((g.x, y, g.x + g.width, y, String::new()));
    }
    None
}

fn looks_axis_token(v: &OptionValue) -> bool {
    matches!(v.as_str(), Some("min" | "max" | "average" | "median"))
}

fn axis_token_to_value(v: &OptionValue, is_x: bool, series: &SeriesModel, model: &GlobalModel) -> f64 {
    match v {
        OptionValue::String(s) => match s.as_str() {
            "min" => {
                if is_x {
                    model.x_axis_at(series.x_axis_index).value_min()
                } else {
                    series
                        .data
                        .iter()
                        .filter(|p| p.value.is_finite())
                        .map(|p| p.value)
                        .fold(f64::INFINITY, f64::min)
                }
            }
            "max" => {
                if is_x {
                    model.x_axis_at(series.x_axis_index).value_max()
                } else {
                    series
                        .data
                        .iter()
                        .filter(|p| p.value.is_finite())
                        .map(|p| p.value)
                        .fold(f64::NEG_INFINITY, f64::max)
                }
            }
            "average" | "median" => stat_point(series, s).map(|(_, v)| v).unwrap_or(0.0),
            _ => s.parse::<f64>().unwrap_or(0.0),
        },
        OptionValue::Number(n) => *n,
        _ => 0.0,
    }
}

fn stat_point(series: &SeriesModel, ty: &str) -> Option<(usize, f64)> {
    let vals: Vec<(usize, f64)> = series
        .data
        .iter()
        .enumerate()
        .filter(|(_, p)| p.value.is_finite())
        .map(|(i, p)| (i, p.value))
        .collect();
    if vals.is_empty() {
        return None;
    }
    match ty {
        "max" => vals
            .iter()
            .copied()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)),
        "min" => vals
            .iter()
            .copied()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)),
        "average" => {
            let sum: f64 = vals.iter().map(|p| p.1).sum();
            let avg = sum / vals.len() as f64;
            let nearest = vals
                .iter()
                .copied()
                .min_by(|a, b| {
                    (a.1 - avg)
                        .abs()
                        .partial_cmp(&(b.1 - avg).abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(vals[0]);
            Some((nearest.0, avg))
        }
        "median" => {
            let mut sorted = vals.clone();
            sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            Some(sorted[sorted.len() / 2])
        }
        _ => None,
    }
}

fn add_line(
    zr: &mut ZRenderer,
    group: usize,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: &str,
    width: f32,
) {
    let line = zr.storage.create_path(
        Path::new(
            Shape::Line(LineShape {
                x1,
                y1,
                x2,
                y2,
                percent: 1.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::none(),
                stroke: FillStrokeStyle::color(color),
                line_width: width,
                line_dash: Some(vec![4.0, 4.0]),
                ..PathStyle::stroke_default()
            },
        )
        .with_displayable(DisplayableProps {
            z: 11.0,
            ..Default::default()
        }),
    );
    zr.storage.group_add_child(group, ChildRef::Path(line));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::option::SetOptionFlags;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn mark_point_max_and_mark_line_average() {
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
                                OptionValue::String("a".into()),
                                OptionValue::String("b".into()),
                                OptionValue::String("c".into()),
                            ]),
                        ),
                    ]),
                ),
                (
                    "yAxis",
                    obj(vec![("type", OptionValue::String("value".into()))]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::Number(1.0),
                                OptionValue::Number(5.0),
                                OptionValue::Number(2.0),
                            ]),
                        ),
                        (
                            "markPoint",
                            obj(vec![(
                                "data",
                                OptionValue::Array(vec![obj(vec![
                                    ("type", OptionValue::String("max".into())),
                                    ("name", OptionValue::String("Max".into())),
                                ])]),
                            )]),
                        ),
                        (
                            "markLine",
                            obj(vec![(
                                "data",
                                OptionValue::Array(vec![obj(vec![(
                                    "type",
                                    OptionValue::String("average".into()),
                                )])]),
                            )]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 400, 300);
        let visual = VisualContext::new(&option, &model);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        render_marks(&mut zr, group, &model, &option, &visual, &model.series[0]);
        assert!(
            zr.storage.texts().iter().any(|t| t.content == "Max"),
            "markPoint label missing: {:?}",
            zr.storage.texts().iter().map(|t| t.content.clone()).collect::<Vec<_>>()
        );
        assert!(
            zr.storage.paths().len() >= 2,
            "markLine/point paths: {}",
            zr.storage.paths().len()
        );
    }
}
