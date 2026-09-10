//! `option.graphic`：与 getZr 共用同一份 Storage（画在 ChartView 根组内）

use rust_zrender::{
    ArcShape, BezierCurveShape, ChildRef, CircleShape, DisplayableProps, EllipseShape,
    FillStrokeStyle, LineShape, Path, PathStyle, PolygonShape, PolylineShape, RectShape, RingShape,
    SectorShape, Shape, ShadowStyle, Text, TextAlign, TextBaseline, TextStyle, ZRenderer,
};

use crate::chart::layout::layout_origin;
use crate::chart::style::{line_dash_from_style, option_to_fill, style_opacity, style_width};
use crate::model::GlobalModel;
use crate::option::{OptionModel, OptionValue};
use crate::utils::as_components;

pub(crate) fn add_graphic_element(
    zr: &mut ZRenderer,
    parent: usize,
    el: &OptionValue,
    parent_w: f64,
    parent_h: f64,
) {
    add_element(zr, parent, el, parent_w, parent_h, 0.0, 0.0);
}

pub fn render_graphic(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
) {
    let elems = as_components(option.root().get("graphic"));
    if elems.is_empty() {
        return;
    }
    let pw = model.width as f64;
    let ph = model.height as f64;
    for el in elems {
        if el.get("ignore").and_then(|v| v.as_bool()) == Some(true)
            || el.get("invisible").and_then(|v| v.as_bool()) == Some(true)
        {
            continue;
        }
        add_element(zr, group, el, pw, ph, 0.0, 0.0);
    }
}

fn add_element(
    zr: &mut ZRenderer,
    parent: usize,
    el: &OptionValue,
    parent_w: f64,
    parent_h: f64,
    origin_x: f64,
    origin_y: f64,
) {
    let ty = el.get("type").and_then(|v| v.as_str()).unwrap_or("group");
    let z = el.get("z").and_then(|v| v.as_f64()).unwrap_or(100.0);
    let rotation = el.get("rotation").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let (sw, sh) = intrinsic_size(el);
    let (lx, ly) = layout_origin(el, parent_w, parent_h, sw, sh, 0.0, 0.0);
    let x = origin_x + lx;
    let y = origin_y + ly;
    match ty {
        "group" => {
            let g = zr.storage.create_group();
            {
                let grp = zr.storage.group_mut(g);
                grp.base.transform_state.x = x;
                grp.base.transform_state.y = y;
                grp.base.transform_state.rotation = rotation;
            }
            zr.storage.group_add_child(parent, ChildRef::Group(g));
            if let Some(children) = el.get("children").and_then(|v| v.as_array()) {
                for child in children {
                    add_element(zr, g, child, 0.0, 0.0, 0.0, 0.0);
                }
            }
        }
        "text" => {
            let style = el.get("style");
            let content = style
                .and_then(|s| s.get("text"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if content.is_empty() {
                return;
            }
            let fill = style
                .and_then(|s| s.get("fill").or_else(|| s.get("color")))
                .and_then(|v| v.as_str())
                .unwrap_or("#000");
            let font_size = parse_font_size(style);
            let family = parse_font_family(style);
            let idx = zr.storage.create_text(
                Text::new(content, x, y)
                    .with_style(TextStyle {
                        fill: fill.into(),
                        font_size,
                        font_family: family,
                        align: TextAlign::Left,
                        baseline: TextBaseline::Top,
                    })
                    .with_displayable(DisplayableProps {
                        z,
                        ..Default::default()
                    }),
            );
            zr.storage.text_mut(idx).silent = el.get("silent").and_then(|v| v.as_bool()).unwrap_or(true);
            zr.storage.group_add_child(parent, ChildRef::Text(idx));
        }
        _ => {
            if let Some(shape) = shape_of(ty, el.get("shape"), x, y) {
                let mut path = Path::new(shape, path_style_of(el)).with_displayable(DisplayableProps {
                    z,
                    ..Default::default()
                });
                path.silent = el.get("silent").and_then(|v| v.as_bool()).unwrap_or(true);
                if rotation != 0.0 {
                    path.base.transform_state.rotation = rotation;
                    path.base.transform_state.origin_x = sw / 2.0;
                    path.base.transform_state.origin_y = sh / 2.0;
                }
                let idx = zr.storage.create_path(path);
                zr.storage.group_add_child(parent, ChildRef::Path(idx));
            }
        }
    }
}

fn intrinsic_size(el: &OptionValue) -> (f64, f64) {
    let shape = el.get("shape");
    let w = shape
        .and_then(|s| s.get("width"))
        .and_then(|v| v.as_f64())
        .or_else(|| el.get("style").and_then(|s| s.get("width")).and_then(|v| v.as_f64()))
        .unwrap_or(0.0);
    let h = shape
        .and_then(|s| s.get("height"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    (w, h)
}

fn shape_of(ty: &str, shape: Option<&OptionValue>, dx: f64, dy: f64) -> Option<Shape> {
    let s = shape;
    match ty {
        "rect" => Some(Shape::Rect(RectShape {
            x: dx + num(s, "x", 0.0),
            y: dy + num(s, "y", 0.0),
            width: num(s, "width", 0.0),
            height: num(s, "height", 0.0),
            r: radii(s),
        })),
        "circle" => Some(Shape::Circle(CircleShape {
            cx: dx + num(s, "cx", 0.0),
            cy: dy + num(s, "cy", 0.0),
            r: num(s, "r", 0.0),
        })),
        "ring" => Some(Shape::Ring(RingShape {
            cx: dx + num(s, "cx", 0.0),
            cy: dy + num(s, "cy", 0.0),
            r: num(s, "r", 0.0),
            r0: num(s, "r0", 0.0),
        })),
        "ellipse" => Some(Shape::Ellipse(EllipseShape {
            cx: dx + num(s, "cx", 0.0),
            cy: dy + num(s, "cy", 0.0),
            rx: num(s, "rx", 0.0),
            ry: num(s, "ry", 0.0),
        })),
        "sector" => Some(Shape::Sector(SectorShape {
            cx: dx + num(s, "cx", 0.0),
            cy: dy + num(s, "cy", 0.0),
            r: num(s, "r", 0.0),
            r0: num(s, "r0", 0.0),
            start_angle: num(s, "startAngle", 0.0),
            end_angle: num(s, "endAngle", std::f64::consts::PI * 2.0),
            clockwise: s
                .and_then(|v| v.get("clockwise"))
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            ..Default::default()
        })),
        "arc" => Some(Shape::Arc(ArcShape {
            cx: dx + num(s, "cx", 0.0),
            cy: dy + num(s, "cy", 0.0),
            r: num(s, "r", 0.0),
            start_angle: num(s, "startAngle", 0.0),
            end_angle: num(s, "endAngle", std::f64::consts::PI),
            clockwise: s
                .and_then(|v| v.get("clockwise"))
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
        })),
        "line" => Some(Shape::Line(LineShape {
            x1: dx + num(s, "x1", 0.0),
            y1: dy + num(s, "y1", 0.0),
            x2: dx + num(s, "x2", 0.0),
            y2: dy + num(s, "y2", 0.0),
            percent: 1.0,
        })),
        "polyline" => Some(Shape::Polyline(PolylineShape {
            points: shift_points(s, dx, dy),
            percent: 1.0,
            smooth: num(s, "smooth", 0.0),
            ..Default::default()
        })),
        "polygon" => Some(Shape::Polygon(PolygonShape {
            points: shift_points(s, dx, dy),
            smooth: num(s, "smooth", 0.0),
            ..Default::default()
        })),
        "bezierCurve" => Some(Shape::BezierCurve(BezierCurveShape {
            x1: dx + num(s, "x1", 0.0),
            y1: dy + num(s, "y1", 0.0),
            x2: dx + num(s, "x2", 0.0),
            y2: dy + num(s, "y2", 0.0),
            cpx1: dx + num(s, "cpx1", 0.0),
            cpy1: dy + num(s, "cpy1", 0.0),
            cpx2: s.and_then(|v| v.get("cpx2")).and_then(|v| v.as_f64()).map(|n| n + dx),
            cpy2: s.and_then(|v| v.get("cpy2")).and_then(|v| v.as_f64()).map(|n| n + dy),
            percent: 1.0,
        })),
        _ => None,
    }
}

fn num(shape: Option<&OptionValue>, key: &str, default: f64) -> f64 {
    shape
        .and_then(|s| s.get(key))
        .and_then(|v| v.as_f64())
        .unwrap_or(default)
}

fn radii(shape: Option<&OptionValue>) -> Vec<f64> {
    match shape.and_then(|s| s.get("r")) {
        Some(OptionValue::Number(n)) => vec![*n],
        Some(OptionValue::Array(arr)) => arr.iter().filter_map(|v| v.as_f64()).collect(),
        _ => Vec::new(),
    }
}

fn shift_points(shape: Option<&OptionValue>, dx: f64, dy: f64) -> Vec<(f64, f64)> {
    let Some(OptionValue::Array(arr)) = shape.and_then(|s| s.get("points")) else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|p| {
            let a = p.as_array()?;
            Some((
                a.first().and_then(|v| v.as_f64()).unwrap_or(0.0) + dx,
                a.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0) + dy,
            ))
        })
        .collect()
}

fn path_style_of(el: &OptionValue) -> PathStyle {
    let style = el.get("style");
    let fill = match style.and_then(|s| s.get("fill")) {
        Some(v) => option_to_fill(Some(v), "#000"),
        None => FillStrokeStyle::color("#000"),
    };
    let stroke = match style.and_then(|s| s.get("stroke")) {
        Some(OptionValue::String(s)) if s == "none" || s.is_empty() => FillStrokeStyle::none(),
        Some(v) => option_to_fill(Some(v), "#000"),
        None => FillStrokeStyle::none(),
    };
    let line_width = style_width(style, 0.0);
    let opacity = style_opacity(style, 1.0);
    let stroke_none = stroke.is_none();
    let dash = line_dash_from_style(style, line_width.max(1.0));
    let shadow = style.and_then(parse_shadow);
    PathStyle {
        fill,
        stroke,
        line_width: if line_width > 0.0 {
            line_width
        } else if !stroke_none {
            1.0
        } else {
            0.0
        },
        opacity,
        line_dash: dash,
        shadow,
        ..Default::default()
    }
}

fn parse_shadow(style: &OptionValue) -> Option<ShadowStyle> {
    let blur = style.get("shadowBlur").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let ox = style.get("shadowOffsetX").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let oy = style.get("shadowOffsetY").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    if blur <= 0.0 && ox == 0.0 && oy == 0.0 {
        return None;
    }
    Some(ShadowStyle {
        color: style
            .get("shadowColor")
            .and_then(|v| v.as_str())
            .unwrap_or("rgba(0,0,0,0.3)")
            .to_string(),
        blur,
        offset_x: ox,
        offset_y: oy,
    })
}

fn parse_font_size(style: Option<&OptionValue>) -> f32 {
    if let Some(n) = style.and_then(|s| s.get("fontSize")).and_then(|v| v.as_f64()) {
        return n as f32;
    }
    if let Some(font) = style.and_then(|s| s.get("font")).and_then(|v| v.as_str()) {
        for part in font.split_whitespace() {
            if let Some(px) = part.strip_suffix("px") {
                if let Ok(n) = px.parse::<f32>() {
                    return n;
                }
            }
        }
    }
    12.0
}

fn parse_font_family(style: Option<&OptionValue>) -> String {
    if let Some(f) = style.and_then(|s| s.get("fontFamily")).and_then(|v| v.as_str()) {
        return f.to_string();
    }
    if let Some(font) = style.and_then(|s| s.get("font")).and_then(|v| v.as_str()) {
        if let Some((_, rest)) = font.split_once("px ") {
            return rest.trim().trim_matches('"').to_string();
        }
    }
    "sans-serif".into()
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
    fn graphic_rect_and_text() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![(
                "graphic",
                OptionValue::Array(vec![
                    obj(vec![
                        ("type", OptionValue::String("rect".into())),
                        (
                            "shape",
                            obj(vec![
                                ("width", OptionValue::Number(40.0)),
                                ("height", OptionValue::Number(20.0)),
                            ]),
                        ),
                        (
                            "style",
                            obj(vec![("fill", OptionValue::String("#f00".into()))]),
                        ),
                        ("left", OptionValue::Number(10.0)),
                        ("top", OptionValue::Number(10.0)),
                    ]),
                    obj(vec![
                        ("type", OptionValue::String("text".into())),
                        (
                            "style",
                            obj(vec![("text", OptionValue::String("ECHARTS".into()))]),
                        ),
                    ]),
                ]),
            )]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 200, 150);
        let mut zr = ZRenderer::new(200, 150).unwrap();
        let group = zr.storage.create_group();
        render_graphic(&mut zr, group, &model, &option);
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, Shape::Rect(_))));
        assert!(zr.storage.texts().iter().any(|t| t.content == "ECHARTS"));
    }
}
