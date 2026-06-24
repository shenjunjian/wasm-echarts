//! 从 JsValue 解析 init / element opts

use js_sys::{Array, Reflect};
use rust_zrender::{
    DisplayableProps, EcData, FillStrokeStyle, PathStyle, PathStylePatch, ShadowStyle, TextAlign,
    TextBaseline, TextStyle,
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct InitOpts {
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ElementCommonOpts {
    pub displayable: DisplayableProps,
    pub silent: bool,
    pub name: Option<String>,
    pub ec_data: EcData,
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
        ec_data: EcData::default(),
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

pub fn parse_path_style(style: &JsValue) -> PathStyle {
    if style.is_null() || style.is_undefined() {
        return PathStyle::default();
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
        ..PathStyle::default()
    };

    if path_style.fill.is_none() && get_value(style, "fill").is_undefined() {
        path_style.fill = FillStrokeStyle::Color("#000".into());
    }
    if path_style.stroke.is_none() && get_value(style, "stroke").is_undefined() {
        path_style.stroke = FillStrokeStyle::None;
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
        fill: get_string(style, "fill").unwrap_or_else(|| "#333".into()),
        font_size: get_f64(style, "fontSize").unwrap_or(12.0) as f32,
        align: parse_text_align(style),
        baseline: parse_text_baseline(style),
    }
}

fn parse_fill_stroke(value: &JsValue) -> FillStrokeStyle {
    if value.is_null() || value.is_undefined() {
        return FillStrokeStyle::None;
    }
    if let Some(s) = value.as_string() {
        if s == "none" || s.is_empty() {
            return FillStrokeStyle::None;
        }
        return FillStrokeStyle::Color(s);
    }
    // 渐变 / Pattern 在 Todo 4 stub 或后续补全；当前按 none 处理
    FillStrokeStyle::None
}

fn parse_line_dash(style: &JsValue) -> Option<Vec<f32>> {
    let dash = get_value(style, "lineDash");
    if dash.is_null() || dash.is_undefined() {
        return None;
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
    match get_string(style, "align").as_deref() {
        Some("center") => TextAlign::Center,
        Some("right") => TextAlign::Right,
        _ => TextAlign::Left,
    }
}

fn parse_text_baseline(style: &JsValue) -> TextBaseline {
    match get_string(style, "verticalAlign")
        .or_else(|| get_string(style, "baseline"))
        .as_deref()
    {
        Some("top") => TextBaseline::Top,
        Some("middle") => TextBaseline::Middle,
        Some("bottom") => TextBaseline::Bottom,
        _ => TextBaseline::Alphabetic,
    }
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
