//! 从 JsValue 构建 pending 图元数据

use std::collections::HashMap;

use rust_zrender::{ImageStyle, Shape};
use wasm_bindgen::prelude::*;

use super::fill_stroke::decode_pattern_image;
use super::opts::{
    get_f64, get_object, get_string, get_u32, get_value, parse_element_common, parse_path_style,
    parse_text_style,
};
use super::shape::{
    parse_compound_path_shape, parse_path_data_shape, parse_shape_by_type, shape_from_opts,
};
use crate::element::pending::{PendingData, PendingImage, PendingPath, PendingText};

pub fn build_pending_path(type_name: &str, opts: &JsValue) -> Result<PendingData, JsValue> {
    let common = parse_element_common(opts);
    let style = parse_path_style(&get_object(opts, "style"));
    let shape_js = shape_from_opts(opts);
    let shape = parse_shape(type_name, &shape_js)?;
    Ok(PendingData::Path(PendingPath {
        shape,
        style,
        displayable: common.displayable,
        silent: common.silent,
        name: common.name.unwrap_or_default(),
        ec_data: common.ec_data,
        state_patches: HashMap::new(),
        active_states: Vec::new(),
    }))
}

pub fn build_pending_image(opts: &JsValue) -> Result<PendingData, JsValue> {
    let common = parse_element_common(opts);
    let style_obj = get_object(opts, "style");
    let style = parse_image_style(&style_obj, opts)?;
    Ok(PendingData::Image(PendingImage {
        style,
        displayable: common.displayable,
        silent: common.silent,
        name: common.name.unwrap_or_default(),
        ec_data: common.ec_data,
    }))
}

pub fn build_pending_text(opts: &JsValue) -> PendingData {
    let common = parse_element_common(opts);
    let style_obj = get_object(opts, "style");
    let content = get_string(&style_obj, "text").unwrap_or_default();
    let x = get_f64(&style_obj, "x")
        .or_else(|| get_f64(opts, "x"))
        .unwrap_or(0.0);
    let y = get_f64(&style_obj, "y")
        .or_else(|| get_f64(opts, "y"))
        .unwrap_or(0.0);
    let style = parse_text_style(&style_obj);
    PendingData::Text(PendingText {
        content,
        x,
        y,
        style,
        displayable: common.displayable,
        silent: common.silent,
        name: common.name.unwrap_or_default(),
        ec_data: common.ec_data,
    })
}

fn parse_image_style(style: &JsValue, opts: &JsValue) -> Result<ImageStyle, JsValue> {
    let image_val = get_value(style, "image");
    let (data, source_width, source_height) =
        decode_pattern_image(&image_val, get_u32(style, "imageWidth"), get_u32(style, "imageHeight"))?;

    Ok(ImageStyle {
        x: get_f64(style, "x").unwrap_or(0.0),
        y: get_f64(style, "y").unwrap_or(0.0),
        width: get_f64(style, "width"),
        height: get_f64(style, "height"),
        sx: get_f64(style, "sx").unwrap_or(0.0),
        sy: get_f64(style, "sy").unwrap_or(0.0),
        s_width: get_f64(style, "sWidth"),
        s_height: get_f64(style, "sHeight"),
        opacity: get_f64(style, "opacity")
            .or_else(|| get_f64(opts, "opacity"))
            .unwrap_or(1.0) as f32,
        data,
        source_width,
        source_height,
    })
}

fn parse_shape(type_name: &str, shape: &JsValue) -> Result<Shape, JsValue> {
    match type_name {
        "compound" => Ok(Shape::Compound(parse_compound_path_shape(shape)?)),
        "path" => Ok(Shape::PathData(parse_path_data_shape(shape)?)),
        other => parse_shape_by_type(other, shape),
    }
}
