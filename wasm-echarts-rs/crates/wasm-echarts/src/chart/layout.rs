//! 组件盒模型：padding / left-right-top-bottom / 命中用 EcData

use rust_zrender::{
    ChildRef, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle, RectShape, Shape,
    Text, TextAlign, TextBaseline, TextStyle, ZRenderer,
};

use crate::option::OptionValue;
use crate::utils::parse_percent;

pub const HIT_LEGEND: &str = "legend";
pub const HIT_TOOLBOX: &str = "toolbox";
pub const HIT_DATA_ZOOM: &str = "dataZoom";
pub const HIT_TIMELINE: &str = "timeline";
pub const HIT_VISUAL_MAP: &str = "visualMap";
pub const HIT_THUMBNAIL: &str = "thumbnail";
#[allow(dead_code)]
pub const HIT_BRUSH: &str = "brush";

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Padding {
    pub fn uniform(v: f64) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
}

/// 数字、`[v,h]`、`[t,r,b,l]`。
pub fn parse_padding(value: Option<&OptionValue>, default: f64) -> Padding {
    match value {
        Some(OptionValue::Number(n)) if n.is_finite() => Padding::uniform(*n),
        Some(OptionValue::Array(arr)) => match arr.len() {
            0 => Padding::uniform(default),
            1 => Padding::uniform(arr[0].as_f64().unwrap_or(default)),
            2 => {
                let v = arr[0].as_f64().unwrap_or(default);
                let h = arr[1].as_f64().unwrap_or(default);
                Padding {
                    top: v,
                    right: h,
                    bottom: v,
                    left: h,
                }
            }
            _ => Padding {
                top: arr[0].as_f64().unwrap_or(default),
                right: arr.get(1).and_then(|v| v.as_f64()).unwrap_or(default),
                bottom: arr.get(2).and_then(|v| v.as_f64()).unwrap_or(default),
                left: arr.get(3).and_then(|v| v.as_f64()).unwrap_or(default),
            },
        },
        _ => Padding::uniform(default),
    }
}

pub fn parse_orient(value: Option<&OptionValue>) -> bool {
    !matches!(value.and_then(|v| v.as_str()), Some("vertical"))
}

/// 相对父盒计算元素左上角。`self_w`/`self_h` 为 0 时 `right`/`bottom` 表示锚点。
pub fn layout_origin(
    el: &OptionValue,
    parent_w: f64,
    parent_h: f64,
    self_w: f64,
    self_h: f64,
    default_x: f64,
    default_y: f64,
) -> (f64, f64) {
    let x = axis_origin(
        el.get("left"),
        el.get("right"),
        parent_w,
        self_w,
        default_x,
        "left",
        "right",
        "center",
    );
    let y = axis_origin(
        el.get("top"),
        el.get("bottom"),
        parent_h,
        self_h,
        default_y,
        "top",
        "bottom",
        "middle",
    );
    (x, y)
}

fn axis_origin(
    start: Option<&OptionValue>,
    end: Option<&OptionValue>,
    parent: f64,
    self_size: f64,
    default: f64,
    start_name: &str,
    end_name: &str,
    center_name: &str,
) -> f64 {
    if let Some(v) = start {
        return match v {
            OptionValue::String(s) if s == "center" || s == center_name => {
                (parent - self_size) / 2.0
            }
            OptionValue::String(s) if s == start_name || s == "left" || s == "top" => 0.0,
            OptionValue::String(s) if s == end_name || s == "right" || s == "bottom" => {
                parent - self_size
            }
            _ => parse_percent(Some(v), parent, default),
        };
    }
    if let Some(v) = end {
        let e = match v {
            OptionValue::String(s) if s == "center" || s == center_name => parent / 2.0,
            _ => parse_percent(Some(v), parent, 0.0),
        };
        return parent - e - self_size;
    }
    default
}

pub fn parse_align(value: Option<&OptionValue>, fallback: TextAlign) -> TextAlign {
    match value.and_then(|v| v.as_str()) {
        Some("center") => TextAlign::Center,
        Some("right") => TextAlign::Right,
        Some("left") => TextAlign::Left,
        _ => fallback,
    }
}

pub fn parse_baseline(value: Option<&OptionValue>, fallback: TextBaseline) -> TextBaseline {
    match value.and_then(|v| v.as_str()) {
        Some("middle") | Some("center") => TextBaseline::Middle,
        Some("bottom") => TextBaseline::Bottom,
        Some("top") => TextBaseline::Top,
        _ => fallback,
    }
}

pub fn component_ec(component: &str, index: i32) -> EcData {
    EcData {
        series_index: None,
        data_index: Some(index),
        data_type: Some(component.to_string()),
    }
}

pub fn add_rect(
    zr: &mut ZRenderer,
    group: usize,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    fill: FillStrokeStyle,
    stroke: FillStrokeStyle,
    line_width: f32,
    z: f64,
    ec: Option<EcData>,
) -> usize {
    let mut path = Path::new(
        Shape::Rect(RectShape {
            x,
            y,
            width: w,
            height: h,
            ..Default::default()
        }),
        PathStyle {
            fill,
            stroke,
            line_width,
            ..Default::default()
        },
    )
    .with_displayable(DisplayableProps {
        z,
        ..Default::default()
    });
    if let Some(ec) = ec {
        path = path.with_ec_data(ec);
    }
    let idx = zr.storage.create_path(path);
    zr.storage.group_add_child(group, ChildRef::Path(idx));
    idx
}

pub fn add_text(
    zr: &mut ZRenderer,
    group: usize,
    content: &str,
    x: f64,
    y: f64,
    style: TextStyle,
    z: f64,
    silent: bool,
    ec: Option<EcData>,
) -> Option<usize> {
    if content.is_empty() {
        return None;
    }
    let mut text = Text::new(content, x, y)
        .with_style(style)
        .with_displayable(DisplayableProps {
            z,
            ..Default::default()
        });
    if let Some(ec) = ec {
        text = text.with_ec_data(ec);
    }
    let idx = zr.storage.create_text(text);
    zr.storage.text_mut(idx).silent = silent;
    zr.storage.group_add_child(group, ChildRef::Text(idx));
    Some(idx)
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
    fn padding_css_shorthand() {
        let p = parse_padding(Some(&OptionValue::Number(8.0)), 5.0);
        assert!((p.left - 8.0).abs() < 1e-9);
        let p = parse_padding(
            Some(&OptionValue::Array(vec![
                OptionValue::Number(1.0),
                OptionValue::Number(2.0),
            ])),
            0.0,
        );
        assert!((p.top - 1.0).abs() < 1e-9 && (p.left - 2.0).abs() < 1e-9);
        let p = parse_padding(
            Some(&OptionValue::Array(vec![
                OptionValue::Number(1.0),
                OptionValue::Number(2.0),
                OptionValue::Number(3.0),
                OptionValue::Number(4.0),
            ])),
            0.0,
        );
        assert!((p.top - 1.0).abs() < 1e-9 && (p.right - 2.0).abs() < 1e-9);
        assert!((p.bottom - 3.0).abs() < 1e-9 && (p.left - 4.0).abs() < 1e-9);
    }

    #[test]
    fn layout_right_bottom_anchor() {
        let el = obj(vec![
            ("right", OptionValue::Number(110.0)),
            ("bottom", OptionValue::Number(110.0)),
        ]);
        let (x, y) = layout_origin(&el, 800.0, 600.0, 0.0, 0.0, 0.0, 0.0);
        assert!((x - 690.0).abs() < 1e-9);
        assert!((y - 490.0).abs() < 1e-9);
    }

    #[test]
    fn layout_center() {
        let el = obj(vec![
            ("left", OptionValue::String("center".into())),
            ("top", OptionValue::String("middle".into())),
        ]);
        let (x, y) = layout_origin(&el, 200.0, 100.0, 40.0, 20.0, 0.0, 0.0);
        assert!((x - 80.0).abs() < 1e-9);
        assert!((y - 40.0).abs() < 1e-9);
    }
}
