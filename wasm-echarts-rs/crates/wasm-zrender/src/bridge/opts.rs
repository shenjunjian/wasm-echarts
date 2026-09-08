//! 从 JsValue 解析 init / element opts

use js_sys::{Array, Reflect};
use rust_zrender::{
    DisplayableProps, EcData, FillStrokeStyle, LineCap, LineJoin, PathStyle, PathStylePatch,
    ShadowStyle, TextAlign, TextBaseline, TextStyle, normalize_line_dash,
};
use wasm_bindgen::prelude::*;

use crate::bridge::fill_stroke::parse_fill_stroke;

#[derive(Debug, Clone, Copy)]
pub struct InitOpts {
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DraggableKind {
    #[default]
    None,
    All,
    Horizontal,
    Vertical,
}

impl DraggableKind {
    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    pub fn to_js(self) -> JsValue {
        match self {
            Self::None => JsValue::from(false),
            Self::All => JsValue::from(true),
            Self::Horizontal => JsValue::from_str("horizontal"),
            Self::Vertical => JsValue::from_str("vertical"),
        }
    }

    pub fn from_js(value: &JsValue) -> Self {
        if let Some(flag) = value.as_bool() {
            return if flag { Self::All } else { Self::None };
        }
        match value.as_string().as_deref() {
            Some("horizontal") => Self::Horizontal,
            Some("vertical") => Self::Vertical,
            Some("true") => Self::All,
            _ => Self::None,
        }
    }

    pub fn clamp_delta(self, dx: f64, dy: f64) -> (f64, f64) {
        match self {
            Self::None => (0.0, 0.0),
            Self::All => (dx, dy),
            Self::Horizontal => (dx, 0.0),
            Self::Vertical => (0.0, dy),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ElementCommonOpts {
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: Option<String>,
    pub ignore: bool,
    pub ec_data: EcData,
    pub draggable: DraggableKind,
}

pub fn parse_init_opts(_dom: &JsValue, opts: &JsValue) -> Result<InitOpts, JsValue> {
    let width = get_u32(opts, "width").unwrap_or(300);
    let height = get_u32(opts, "height").unwrap_or(150);
    let dpr = get_f64(opts, "devicePixelRatio")
        .or_else(|| get_f64(opts, "dpr"))
        .unwrap_or(1.0);
    if width == 0 || height == 0 {
        return Err(JsValue::from_str("width and height must be greater than 0"));
    }
    Ok(InitOpts { width, height, dpr })
}

pub fn parse_element_common(opts: &JsValue) -> ElementCommonOpts {
    let mut common = ElementCommonOpts {
        displayable: parse_displayable(opts),
        silent: get_bool(opts, "silent").unwrap_or(false),
        name: get_string(opts, "name"),
        ignore: get_bool(opts, "ignore").unwrap_or(false),
        ec_data: EcData::default(),
        draggable: parse_draggable(opts),
    };

    if let Some(series_index) = get_i32(opts, "seriesIndex") {
        let data_index = get_i32(opts, "dataIndex").unwrap_or(0);
        common.ec_data = EcData::new(series_index, data_index);
        if let Some(data_type) = get_string(opts, "dataType") {
            common.ec_data = common.ec_data.with_data_type(data_type);
        }
    }

    common
}

pub fn parse_displayable(opts: &JsValue) -> DisplayableProps {
    DisplayableProps {
        z: get_f64(opts, "z").unwrap_or(0.0),
        z2: get_f64(opts, "z2").unwrap_or(0.0),
        zlevel: get_f64(opts, "zlevel").unwrap_or(0.0),
        invisible: get_bool(opts, "invisible").unwrap_or(false),
        culling: get_bool(opts, "culling").unwrap_or(false),
    }
}

fn is_stroke_only_path(type_name: &str) -> bool {
    matches!(
        type_name,
        "line" | "polyline" | "bezier-curve" | "arc" | "rose" | "trochoid"
    )
}

pub fn parse_path_style_for(type_name: &str, style: &JsValue) -> PathStyle {
    if style.is_null() || style.is_undefined() {
        return if is_stroke_only_path(type_name) {
            PathStyle::stroke_default()
        } else {
            PathStyle::default()
        };
    }

    let mut path_style = PathStyle {
        fill: parse_fill_stroke(&get_value(style, "fill")),
        stroke: parse_fill_stroke(&get_value(style, "stroke")),
        line_width: get_f64(style, "lineWidth").unwrap_or(1.0) as f32,
        opacity: get_f64(style, "opacity").unwrap_or(1.0) as f32,
        line_dash: parse_line_dash(style),
        line_dash_offset: get_f64(style, "lineDashOffset").unwrap_or(0.0) as f32,
        fill_opacity: get_f64(style, "fillOpacity").unwrap_or(1.0) as f32,
        stroke_opacity: get_f64(style, "strokeOpacity").unwrap_or(1.0) as f32,
        shadow: parse_shadow(style),
        stroke_first: get_bool(style, "strokeFirst").unwrap_or(false),
        line_cap: parse_line_cap(style),
        line_join: parse_line_join(style),
        miter_limit: get_f64(style, "miterLimit").unwrap_or(10.0) as f32,
        ..PathStyle::default()
    };

    let fill_undefined = get_value(style, "fill").is_undefined();
    let stroke_undefined = get_value(style, "stroke").is_undefined();
    if is_stroke_only_path(type_name) {
        if fill_undefined {
            path_style.fill = FillStrokeStyle::None;
        }
        if stroke_undefined {
            path_style.stroke = FillStrokeStyle::Color("#000".into());
        }
    } else {
        if path_style.fill.is_none() && fill_undefined {
            path_style.fill = FillStrokeStyle::Color("#000".into());
        }
        if path_style.stroke.is_none() && stroke_undefined {
            path_style.stroke = FillStrokeStyle::None;
        }
    }

    path_style
}

pub fn parse_path_style_patch(style: &JsValue) -> PathStylePatch {
    if style.is_null() || style.is_undefined() {
        return PathStylePatch::default();
    }

    PathStylePatch {
        fill: parse_optional_fill_stroke(style, "fill"),
        stroke: parse_optional_fill_stroke(style, "stroke"),
        line_width: get_f64(style, "lineWidth").map(|n| n as f32),
        opacity: get_f64(style, "opacity").map(|n| n as f32),
        fill_opacity: get_f64(style, "fillOpacity").map(|n| n as f32),
        stroke_opacity: get_f64(style, "strokeOpacity").map(|n| n as f32),
        shadow: None,
    }
}

fn parse_optional_fill_stroke(obj: &JsValue, key: &str) -> Option<FillStrokeStyle> {
    let value = get_value(obj, key);
    if value.is_undefined() {
        None
    } else {
        Some(parse_fill_stroke(&value))
    }
}

pub fn parse_text_style(style: &JsValue) -> TextStyle {
    if style.is_null() || style.is_undefined() {
        return TextStyle::default();
    }

    TextStyle {
        fill: get_string(style, "fill")
            .or_else(|| get_string(style, "textFill"))
            .unwrap_or_else(|| "#000".into()),
        font_size: get_f64(style, "fontSize").unwrap_or(12.0) as f32,
        align: parse_text_align(style),
        baseline: parse_text_baseline(style),
    }
}

fn parse_line_cap(style: &JsValue) -> LineCap {
    match get_string(style, "lineCap").as_deref() {
        Some("round") => LineCap::Round,
        Some("square") => LineCap::Square,
        _ => LineCap::Butt,
    }
}

fn parse_line_join(style: &JsValue) -> LineJoin {
    match get_string(style, "lineJoin").as_deref() {
        Some("round") => LineJoin::Round,
        Some("bevel") => LineJoin::Bevel,
        _ => LineJoin::Miter,
    }
}

fn parse_line_dash(style: &JsValue) -> Option<Vec<f32>> {
    let dash = get_value(style, "lineDash");
    if dash.is_null() || dash.is_undefined() {
        return None;
    }
    if let Some(s) = dash.as_string() {
        let line_width = get_f64(style, "lineWidth").unwrap_or(1.0) as f32;
        return normalize_line_dash(&s, line_width);
    }
    let arr = Array::from(&dash);
    if arr.length() == 0 {
        return None;
    }
    let mut out = Vec::with_capacity(arr.length() as usize);
    for i in 0..arr.length() {
        let v = arr.get(i);
        if let Some(n) = v.as_f64() {
            out.push(n as f32);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn parse_shadow(style: &JsValue) -> Option<ShadowStyle> {
    let blur = get_f64(style, "shadowBlur").unwrap_or(0.0) as f32;
    let offset_x = get_f64(style, "shadowOffsetX").unwrap_or(0.0) as f32;
    let offset_y = get_f64(style, "shadowOffsetY").unwrap_or(0.0) as f32;
    if blur <= 0.0 && offset_x == 0.0 && offset_y == 0.0 {
        return None;
    }
    Some(ShadowStyle {
        color: get_string(style, "shadowColor").unwrap_or_else(|| "rgba(0,0,0,0.3)".into()),
        blur,
        offset_x,
        offset_y,
    })
}

fn parse_text_align(style: &JsValue) -> TextAlign {
    match get_string(style, "align")
        .or_else(|| get_string(style, "textAlign"))
        .as_deref()
    {
        Some("center") => TextAlign::Center,
        Some("right") => TextAlign::Right,
        _ => TextAlign::Left,
    }
}

fn parse_text_baseline(style: &JsValue) -> TextBaseline {
    match get_string(style, "verticalAlign")
        .or_else(|| get_string(style, "textVerticalAlign"))
        .or_else(|| get_string(style, "baseline"))
        .as_deref()
    {
        Some("top") => TextBaseline::Top,
        Some("middle") => TextBaseline::Middle,
        Some("bottom") => TextBaseline::Bottom,
        _ => TextBaseline::Alphabetic,
    }
}

pub fn parse_draggable(opts: &JsValue) -> DraggableKind {
    let value = get_value(opts, "draggable");
    if value.is_undefined() {
        DraggableKind::None
    } else {
        DraggableKind::from_js(&value)
    }
}

/// 解析官方变换主属性：`x` `y` `scaleX` `scaleY` `rotation` `originX` `originY`。
/// `position: [x, y]` 与顶层 `x` / `y` 等价。
pub fn parse_transform(opts: &JsValue) -> crate::element::pending::PendingTransform {
    let (x, y) = parse_position(opts);
    crate::element::pending::PendingTransform {
        x,
        y,
        scale_x: get_f64(opts, "scaleX").unwrap_or(1.0),
        scale_y: get_f64(opts, "scaleY").unwrap_or(1.0),
        rotation: get_f64(opts, "rotation").unwrap_or(0.0),
        origin_x: get_f64(opts, "originX").unwrap_or(0.0),
        origin_y: get_f64(opts, "originY").unwrap_or(0.0),
    }
}

/// 解析 `position: [x, y]` 或顶层 `x` / `y`。
pub fn parse_position(opts: &JsValue) -> (f64, f64) {
    let pos = get_value(opts, "position");
    if let Some((x, y)) = parse_xy_pair(&pos) {
        return (x, y);
    }
    (
        get_f64(opts, "x").unwrap_or(0.0),
        get_f64(opts, "y").unwrap_or(0.0),
    )
}

pub fn parse_xy_pair(value: &JsValue) -> Option<(f64, f64)> {
    if value.is_null() || value.is_undefined() {
        return None;
    }
    if let Some(arr) = value.dyn_ref::<Array>() {
        if arr.length() >= 2 {
            return Some((
                arr.get(0).as_f64().unwrap_or(0.0),
                arr.get(1).as_f64().unwrap_or(0.0),
            ));
        }
    }
    None
}

pub fn xy_pair_to_js(x: f64, y: f64) -> JsValue {
    let arr = Array::new();
    arr.push(&JsValue::from(x));
    arr.push(&JsValue::from(y));
    arr.into()
}

pub fn get_value(obj: &JsValue, key: &str) -> JsValue {
    Reflect::get(obj, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
}

pub fn get_f64(obj: &JsValue, key: &str) -> Option<f64> {
    get_value(obj, key).as_f64()
}

pub fn get_i32(obj: &JsValue, key: &str) -> Option<i32> {
    get_f64(obj, key).map(|n| n as i32)
}

pub fn get_u32(obj: &JsValue, key: &str) -> Option<u32> {
    get_f64(obj, key).map(|n| n as u32)
}

pub fn get_bool(obj: &JsValue, key: &str) -> Option<bool> {
    get_value(obj, key).as_bool()
}

pub fn get_string(obj: &JsValue, key: &str) -> Option<String> {
    get_value(obj, key).as_string()
}

pub fn get_object(obj: &JsValue, key: &str) -> JsValue {
    let v = get_value(obj, key);
    if v.is_object() && !v.is_null() {
        v
    } else {
        JsValue::UNDEFINED
    }
}

pub fn get_string_array(obj: &JsValue, key: &str) -> Option<Vec<String>> {
    let value = get_value(obj, key);
    if !value.is_instance_of::<Array>() {
        return None;
    }
    let array = Array::from(&value);
    let mut out = Vec::with_capacity(array.length() as usize);
    for item in array.iter() {
        if let Some(s) = item.as_string() {
            out.push(s);
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}
