//! visualMap continuous / piecewise：控件 + 按 pieces / inRange 上色

use rust_zrender::{FillStrokeStyle, LinearGradientStyle, TextAlign, TextBaseline, ZRenderer};

use crate::chart::layout::{
    add_rect, add_text, component_ec, layout_origin, parse_orient, parse_padding, HIT_VISUAL_MAP,
};
use crate::chart::text_opt::parse_chart_text_style;
use crate::interaction::InteractionState;
use crate::model::{GlobalModel, SeriesModel};
use crate::option::{OptionModel, OptionValue};
use crate::utils::{as_components, format_axis_number};
use rust_zrender::ColorStop;

pub fn render_visual_map(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    for (vi, vm) in as_components(option.root().get("visualMap")).into_iter().enumerate() {
        if vm.get("show").and_then(|v| v.as_bool()) == Some(false) {
            continue;
        }
        let horizontal = parse_orient(vm.get("orient"));
        let pad = parse_padding(vm.get("padding"), 5.0);
        let (min, max) = value_extent(vm, model);
        let (bar_w, bar_h) = if horizontal {
            (vm.get("itemWidth").and_then(|v| v.as_f64()).unwrap_or(120.0), 14.0)
        } else {
            (14.0, vm.get("itemHeight").and_then(|v| v.as_f64()).unwrap_or(140.0))
        };
        let (x, y) = layout_origin(
            vm,
            model.width as f64,
            model.height as f64,
            bar_w + pad.left + pad.right + 36.0,
            bar_h + pad.top + pad.bottom,
            model.width as f64 - bar_w - 24.0,
            (model.height as f64 - bar_h) / 2.0,
        );
        let bx = x + pad.left;
        let by = y + pad.top;
        let ty = vm.get("type").and_then(|v| v.as_str()).unwrap_or("continuous");
        if ty == "piecewise" {
            let pieces = pieces_of(vm, min, max);
            let n = pieces.len().max(1) as f64;
            for (i, p) in pieces.iter().enumerate() {
                let on = interaction
                    .visual_map_selected
                    .get(&(vi * 100 + i))
                    .copied()
                    .unwrap_or(true);
                let color = if on {
                    p.color.clone()
                } else {
                    "#ccc".into()
                };
                let (rx, ry, rw, rh) = if horizontal {
                    (bx + i as f64 * (bar_w / n + 4.0), by, (bar_w / n).max(8.0), bar_h)
                } else {
                    (bx, by + i as f64 * (bar_h / n + 4.0), bar_w, (bar_h / n).max(8.0))
                };
                add_rect(
                    zr,
                    group,
                    rx,
                    ry,
                    rw,
                    rh,
                    FillStrokeStyle::color(color),
                    FillStrokeStyle::none(),
                    0.0,
                    12.0,
                    Some(component_ec(HIT_VISUAL_MAP, (vi * 100 + i) as i32)),
                );
            }
        } else {
            let colors = in_range_colors(vm);
            let fill = if colors.len() >= 2 {
                FillStrokeStyle::LinearGradient(LinearGradientStyle {
                    x: if horizontal { 0.0 } else { 0.0 },
                    y: if horizontal { 0.0 } else { 1.0 },
                    x2: if horizontal { 1.0 } else { 0.0 },
                    y2: if horizontal { 0.0 } else { 0.0 },
                    color_stops: colors
                        .iter()
                        .enumerate()
                        .map(|(i, c)| ColorStop {
                            offset: i as f64 / (colors.len() - 1).max(1) as f64,
                            color: c.clone(),
                        })
                        .collect(),
                    global: false,
                })
            } else {
                FillStrokeStyle::color(colors.first().map(|s| s.as_str()).unwrap_or("#5470c6"))
            };
            add_rect(
                zr,
                group,
                bx,
                by,
                bar_w,
                bar_h,
                fill,
                FillStrokeStyle::color("#999"),
                1.0,
                12.0,
                Some(component_ec(HIT_VISUAL_MAP, vi as i32)),
            );
            let style = parse_chart_text_style(vm.get("textStyle"), "#333", 11.0, "sans-serif");
            add_text(
                zr,
                group,
                &format_axis_number(max),
                if horizontal { bx + bar_w + 4.0 } else { bx + bar_w + 4.0 },
                if horizontal { by } else { by },
                style.to_text_style(TextAlign::Left, TextBaseline::Top),
                12.0,
                true,
                None,
            );
            add_text(
                zr,
                group,
                &format_axis_number(min),
                if horizontal { bx - 4.0 } else { bx + bar_w + 4.0 },
                if horizontal {
                    by
                } else {
                    by + bar_h - 12.0
                },
                style.to_text_style(
                    if horizontal {
                        TextAlign::Right
                    } else {
                        TextAlign::Left
                    },
                    TextBaseline::Top,
                ),
                12.0,
                true,
                None,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct VisualPiece {
    pub gt: Option<f64>,
    pub gte: Option<f64>,
    pub lt: Option<f64>,
    pub lte: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub value: Option<f64>,
    pub color: String,
}

impl VisualPiece {
    pub fn matches(&self, v: f64) -> bool {
        if !v.is_finite() {
            return false;
        }
        let mut ok = true;
        if let Some(n) = self.gt {
            ok &= v > n;
        }
        if let Some(n) = self.gte {
            ok &= v >= n;
        }
        if let Some(n) = self.lt {
            ok &= v < n;
        }
        if let Some(n) = self.lte {
            ok &= v <= n;
        }
        if let Some(n) = self.min {
            ok &= v >= n;
        }
        if let Some(n) = self.max {
            ok &= v <= n;
        }
        if let Some(n) = self.value {
            ok &= (v - n).abs() < 1e-9;
        }
        ok
    }
}

pub fn piecewise_hit(option: &OptionModel, data_index: i32) -> Option<(usize, usize)> {
    if data_index < 0 {
        return None;
    }
    let vi = (data_index / 100) as usize;
    let piece = (data_index % 100) as usize;
    let vms = as_components(option.root().get("visualMap"));
    let vm = vms.get(vi)?;
    if vm.get("type").and_then(|v| v.as_str()).unwrap_or("continuous") != "piecewise" {
        return None;
    }
    Some((vi, piece))
}

pub fn map_color(
    option: &OptionModel,
    series_index: usize,
    data_index: usize,
    series: &SeriesModel,
) -> Option<String> {
    let vms = as_components(option.root().get("visualMap"));
    if vms.is_empty() {
        return None;
    }
    let point = series.data.get(data_index)?;
    for vm in vms {
        if !applies_to_series(vm, series_index) {
            continue;
        }
        let dim = vm
            .get("dimension")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0) as usize;
        let value = if dim == 0 {
            point.x_value.unwrap_or(data_index as f64)
        } else {
            point.value
        };
        let ty = vm.get("type").and_then(|v| v.as_str()).unwrap_or("continuous");
        if ty == "piecewise" {
            let (min, max) = value_extent_from_series(vm, series);
            for (i, p) in pieces_of(vm, min, max).into_iter().enumerate() {
                if p.matches(value) {
                    if piece_selected(vm, i) {
                        return Some(p.color);
                    }
                    return Some("#ccc".into());
                }
            }
        } else {
            let (min, max) = value_extent_from_series(vm, series);
            let colors = in_range_colors(vm);
            if colors.is_empty() {
                continue;
            }
            let t = if (max - min).abs() < 1e-12 {
                0.0
            } else {
                ((value - min) / (max - min)).clamp(0.0, 1.0)
            };
            return Some(lerp_colors(&colors, t));
        }
    }
    None
}

fn piece_selected(vm: &OptionValue, index: usize) -> bool {
    vm.get("selected")
        .and_then(|v| v.as_object())
        .and_then(|m| m.get(&index.to_string()))
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

fn applies_to_series(vm: &OptionValue, series_index: usize) -> bool {
    match vm.get("seriesIndex") {
        None => true,
        Some(OptionValue::Number(n)) => *n as usize == series_index,
        Some(OptionValue::Array(arr)) => arr.iter().any(|v| v.as_f64().map(|n| n as usize) == Some(series_index)),
        Some(OptionValue::String(s)) if s == "all" => true,
        _ => true,
    }
}

fn value_extent(vm: &OptionValue, model: &GlobalModel) -> (f64, f64) {
    if let (Some(min), Some(max)) = (
        vm.get("min").and_then(|v| v.as_f64()),
        vm.get("max").and_then(|v| v.as_f64()),
    ) {
        return (min, max);
    }
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for s in &model.series {
        for p in &s.data {
            if p.value.is_finite() {
                min = min.min(p.value);
                max = max.max(p.value);
            }
        }
    }
    if !min.is_finite() {
        (0.0, 1.0)
    } else {
        (min, max)
    }
}

fn value_extent_from_series(vm: &OptionValue, series: &SeriesModel) -> (f64, f64) {
    if let (Some(min), Some(max)) = (
        vm.get("min").and_then(|v| v.as_f64()),
        vm.get("max").and_then(|v| v.as_f64()),
    ) {
        return (min, max);
    }
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for p in &series.data {
        if p.value.is_finite() {
            min = min.min(p.value);
            max = max.max(p.value);
        }
    }
    if !min.is_finite() {
        (0.0, 1.0)
    } else {
        (min, max)
    }
}

fn pieces_of(vm: &OptionValue, min: f64, max: f64) -> Vec<VisualPiece> {
    if let Some(arr) = vm.get("pieces").and_then(|v| v.as_array()) {
        return arr
            .iter()
            .map(|p| VisualPiece {
                gt: p.get("gt").and_then(|v| v.as_f64()),
                gte: p.get("gte").and_then(|v| v.as_f64()),
                lt: p.get("lt").and_then(|v| v.as_f64()),
                lte: p.get("lte").and_then(|v| v.as_f64()),
                min: p.get("min").and_then(|v| v.as_f64()),
                max: p.get("max").and_then(|v| v.as_f64()),
                value: p.get("value").and_then(|v| v.as_f64()),
                color: p
                    .get("color")
                    .and_then(|v| v.as_str())
                    .unwrap_or("#5470c6")
                    .to_string(),
            })
            .collect();
    }
    let split = vm
        .get("splitNumber")
        .and_then(|v| v.as_f64())
        .unwrap_or(5.0)
        .max(1.0) as usize;
    let colors = in_range_colors(vm);
    let span = (max - min) / split as f64;
    (0..split)
        .map(|i| VisualPiece {
            gt: None,
            gte: Some(min + span * i as f64),
            lt: None,
            lte: Some(min + span * (i + 1) as f64),
            min: None,
            max: None,
            value: None,
            color: colors
                .get(i % colors.len().max(1))
                .cloned()
                .unwrap_or_else(|| "#5470c6".into()),
        })
        .collect()
}

fn in_range_colors(vm: &OptionValue) -> Vec<String> {
    let in_range = vm.get("inRange");
    match in_range.and_then(|v| v.get("color")) {
        Some(OptionValue::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        Some(OptionValue::String(s)) => vec![s.clone()],
        _ => vec!["#bf444c".into(), "#d88273".into(), "#f6efa6".into()],
    }
}

fn lerp_colors(colors: &[String], t: f64) -> String {
    if colors.is_empty() {
        return "#5470c6".into();
    }
    if colors.len() == 1 || t <= 0.0 {
        return colors[0].clone();
    }
    if t >= 1.0 {
        return colors[colors.len() - 1].clone();
    }
    let scaled = t * (colors.len() - 1) as f64;
    let i = scaled.floor() as usize;
    let f = scaled - i as f64;
    let a = parse_rgb(&colors[i.min(colors.len() - 1)]);
    let b = parse_rgb(&colors[(i + 1).min(colors.len() - 1)]);
    format!(
        "rgb({},{},{})",
        (a.0 + (b.0 - a.0) * f).round() as i32,
        (a.1 + (b.1 - a.1) * f).round() as i32,
        (a.2 + (b.2 - a.2) * f).round() as i32
    )
}

fn parse_rgb(s: &str) -> (f64, f64, f64) {
    let t = s.trim();
    if let Some(hex) = t.strip_prefix('#') {
        let (r, g, b) = if hex.len() >= 6 {
            (
                u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
                u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
                u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
            )
        } else if hex.len() >= 3 {
            (
                u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0),
                u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0),
                u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0),
            )
        } else {
            (0, 0, 0)
        };
        return (r as f64, g as f64, b as f64);
    }
    (84.0, 112.0, 198.0)
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
    fn piecewise_matches_dimension_zero() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "visualMap",
                    obj(vec![
                        ("type", OptionValue::String("piecewise".into())),
                        ("show", OptionValue::Bool(false)),
                        ("dimension", OptionValue::Number(0.0)),
                        (
                            "pieces",
                            OptionValue::Array(vec![obj(vec![
                                ("gt", OptionValue::Number(1.0)),
                                ("lt", OptionValue::Number(3.0)),
                                ("color", OptionValue::String("#00f".into())),
                            ])]),
                        ),
                    ]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::Number(1.0),
                                OptionValue::Number(2.0),
                                OptionValue::Number(3.0),
                                OptionValue::Number(4.0),
                            ]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 200, 150);
        assert_eq!(
            map_color(&option, 0, 2, &model.series[0]).as_deref(),
            Some("#00f")
        );
        assert_eq!(map_color(&option, 0, 0, &model.series[0]), None);
    }

    #[test]
    fn unselected_piece_uses_out_of_range_color() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "visualMap",
                    obj(vec![
                        ("type", OptionValue::String("piecewise".into())),
                        ("show", OptionValue::Bool(false)),
                        ("dimension", OptionValue::Number(0.0)),
                        (
                            "selected",
                            obj(vec![("0", OptionValue::Bool(false))]),
                        ),
                        (
                            "pieces",
                            OptionValue::Array(vec![obj(vec![
                                ("gt", OptionValue::Number(1.0)),
                                ("lt", OptionValue::Number(3.0)),
                                ("color", OptionValue::String("#00f".into())),
                            ])]),
                        ),
                    ]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::Number(1.0),
                                OptionValue::Number(2.0),
                                OptionValue::Number(3.0),
                            ]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 200, 150);
        assert_eq!(
            map_color(&option, 0, 2, &model.series[0]).as_deref(),
            Some("#ccc")
        );
        assert_eq!(piecewise_hit(&option, 0), Some((0, 0)));
        assert_eq!(piecewise_hit(&option, 100), None);
    }
}
