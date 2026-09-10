//! 从 option 读 fill / lineDash / smooth 等 canvas 样式

use rust_zrender::{
    ColorStop, FillStrokeStyle, LinearGradientStyle, RadialGradientStyle, normalize_line_dash,
};

use crate::option::OptionValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepMode {
    Start,
    Middle,
    End,
}

pub fn parse_smooth(value: Option<&OptionValue>) -> f64 {
    match value {
        Some(OptionValue::Bool(true)) => 0.5,
        Some(OptionValue::Number(n)) if n.is_finite() && *n > 0.0 => *n,
        _ => 0.0,
    }
}

pub fn parse_step(value: Option<&OptionValue>) -> Option<StepMode> {
    match value {
        Some(OptionValue::Bool(true)) => Some(StepMode::Start),
        Some(OptionValue::String(s)) => match s.as_str() {
            "start" => Some(StepMode::Start),
            "middle" => Some(StepMode::Middle),
            "end" => Some(StepMode::End),
            _ => None,
        },
        _ => None,
    }
}

pub fn parse_connect_nulls(value: Option<&OptionValue>) -> bool {
    value.and_then(|v| v.as_bool()).unwrap_or(false)
}

pub fn option_to_fill(value: Option<&OptionValue>, fallback: &str) -> FillStrokeStyle {
    match value {
        Some(OptionValue::String(s)) if s == "none" || s.is_empty() => FillStrokeStyle::none(),
        Some(OptionValue::String(s)) => FillStrokeStyle::color(s.as_str()),
        Some(OptionValue::Object(map)) => fill_from_object(map).unwrap_or_else(|| FillStrokeStyle::color(fallback)),
        _ => FillStrokeStyle::color(fallback),
    }
}

fn fill_from_object(map: &indexmap::IndexMap<String, OptionValue>) -> Option<FillStrokeStyle> {
    let ty = map.get("type").and_then(|v| v.as_str()).unwrap_or("");
    if ty == "radial" || (ty.is_empty() && map.get("r").is_some() && map.contains_key("colorStops")) {
        return Some(FillStrokeStyle::RadialGradient(RadialGradientStyle {
            x: map.get("x").and_then(|v| v.as_f64()).unwrap_or(0.5),
            y: map.get("y").and_then(|v| v.as_f64()).unwrap_or(0.5),
            r: map.get("r").and_then(|v| v.as_f64()).unwrap_or(0.5),
            r0: map.get("r0").and_then(|v| v.as_f64()).unwrap_or(0.0),
            color_stops: parse_color_stops(map.get("colorStops")),
            global: map.get("global").and_then(|v| v.as_bool()).unwrap_or(false),
        }));
    }
    if ty == "linear" || map.contains_key("colorStops") {
        return Some(FillStrokeStyle::LinearGradient(LinearGradientStyle {
            x: map.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0),
            y: map.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0),
            x2: map.get("x2").and_then(|v| v.as_f64()).unwrap_or(1.0),
            y2: map.get("y2").and_then(|v| v.as_f64()).unwrap_or(0.0),
            color_stops: parse_color_stops(map.get("colorStops")),
            global: map.get("global").and_then(|v| v.as_bool()).unwrap_or(false),
        }));
    }
    None
}

fn parse_color_stops(value: Option<&OptionValue>) -> Vec<ColorStop> {
    let Some(OptionValue::Array(arr)) = value else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|item| {
            let offset = item.get("offset").and_then(|v| v.as_f64())?;
            let color = item.get("color").and_then(|v| v.as_str())?.to_string();
            Some(ColorStop { offset, color })
        })
        .collect()
}

pub fn line_dash_from_style(line_style: Option<&OptionValue>, line_width: f32) -> Option<Vec<f32>> {
    if let Some(OptionValue::Array(arr)) = line_style.and_then(|s| s.get("type")) {
        let dash: Vec<f32> = arr.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
        if !dash.is_empty() {
            return Some(dash);
        }
    }
    let ty = line_style
        .and_then(|s| s.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("solid");
    normalize_line_dash(ty, line_width)
}

pub fn style_opacity(style: Option<&OptionValue>, default: f32) -> f32 {
    style
        .and_then(|s| s.get("opacity"))
        .and_then(|v| v.as_f64())
        .map(|n| n.clamp(0.0, 1.0) as f32)
        .unwrap_or(default)
}

pub fn style_width(style: Option<&OptionValue>, default: f32) -> f32 {
    style
        .and_then(|s| s.get("width"))
        .and_then(|v| v.as_f64())
        .filter(|n| n.is_finite())
        .map(|n| n as f32)
        .unwrap_or(default)
}

/// 把折线点转成阶梯折线（对照官方 `turnPointsIntoStep`，基轴为 x）。
pub fn turn_points_into_step(points: &[(f64, f64)], mode: StepMode) -> Vec<(f64, f64)> {
    if points.len() < 2 {
        return points.to_vec();
    }
    let mut out = Vec::with_capacity(points.len() * 2);
    for i in 0..points.len() - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];
        out.push((x0, y0));
        match mode {
            StepMode::End => out.push((x1, y0)),
            StepMode::Middle => {
                let mid = (x0 + x1) / 2.0;
                out.push((mid, y0));
                out.push((mid, y1));
            }
            StepMode::Start => out.push((x0, y1)),
        }
    }
    out.push(*points.last().unwrap());
    out
}

/// 按 `connectNulls` 切段：false 时在 NaN 处断开，true 时丢掉非法点连成一段。
pub fn line_segments(points: &[(f64, f64)], connect_nulls: bool) -> Vec<Vec<(f64, f64)>> {
    if connect_nulls {
        let finite: Vec<(f64, f64)> = points
            .iter()
            .copied()
            .filter(|(x, y)| x.is_finite() && y.is_finite())
            .collect();
        return if finite.is_empty() {
            Vec::new()
        } else {
            vec![finite]
        };
    }
    let mut segments = Vec::new();
    let mut cur = Vec::new();
    for &(x, y) in points {
        if x.is_finite() && y.is_finite() {
            cur.push((x, y));
        } else if !cur.is_empty() {
            segments.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        segments.push(cur);
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn smooth_bool_and_number() {
        assert!((parse_smooth(Some(&OptionValue::Bool(true))) - 0.5).abs() < 1e-9);
        assert!((parse_smooth(Some(&OptionValue::Number(0.6))) - 0.6).abs() < 1e-9);
        assert_eq!(parse_smooth(Some(&OptionValue::Bool(false))), 0.0);
    }

    #[test]
    fn step_true_is_start() {
        assert_eq!(parse_step(Some(&OptionValue::Bool(true))), Some(StepMode::Start));
        assert_eq!(parse_step(Some(&OptionValue::String("middle".into()))), Some(StepMode::Middle));
        assert_eq!(parse_step(Some(&OptionValue::Bool(false))), None);
    }

    #[test]
    fn step_start_inserts_horizontal_then_vertical() {
        let pts = turn_points_into_step(&[(0.0, 1.0), (10.0, 5.0)], StepMode::Start);
        assert_eq!(pts, vec![(0.0, 1.0), (0.0, 5.0), (10.0, 5.0)]);
    }

    #[test]
    fn connect_nulls_splits_or_joins() {
        let pts = [(0.0, 1.0), (f64::NAN, f64::NAN), (2.0, 3.0)];
        assert_eq!(line_segments(&pts, false).len(), 2);
        assert_eq!(line_segments(&pts, true).len(), 1);
        assert_eq!(line_segments(&pts, true)[0].len(), 2);
    }

    #[test]
    fn linear_gradient_from_option() {
        let g = obj(vec![
            ("type", OptionValue::String("linear".into())),
            ("x", OptionValue::Number(0.0)),
            ("y", OptionValue::Number(0.0)),
            ("x2", OptionValue::Number(0.0)),
            ("y2", OptionValue::Number(1.0)),
            (
                "colorStops",
                OptionValue::Array(vec![
                    obj(vec![
                        ("offset", OptionValue::Number(0.0)),
                        ("color", OptionValue::String("#f00".into())),
                    ]),
                    obj(vec![
                        ("offset", OptionValue::Number(1.0)),
                        ("color", OptionValue::String("#00f".into())),
                    ]),
                ]),
            ),
        ]);
        match option_to_fill(Some(&g), "#000") {
            FillStrokeStyle::LinearGradient(lg) => {
                assert!((lg.y2 - 1.0).abs() < 1e-9);
                assert_eq!(lg.color_stops.len(), 2);
                assert_eq!(lg.color_stops[0].color, "#f00");
            }
            other => panic!("expected linear gradient, got {:?}", other),
        }
    }

    #[test]
    fn dashed_line_type() {
        let style = obj(vec![("type", OptionValue::String("dashed".into()))]);
        assert_eq!(line_dash_from_style(Some(&style), 2.0), Some(vec![8.0, 4.0]));
    }
}
