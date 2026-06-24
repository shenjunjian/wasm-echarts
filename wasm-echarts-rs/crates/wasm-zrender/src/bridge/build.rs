//! 从 JsValue 构建 pending 图元数据

use std::collections::HashMap;

use rust_zrender::Shape;
use wasm_bindgen::prelude::*;

use super::opts::{
    get_f64, get_object, get_string, parse_element_common, parse_path_style,
    parse_text_style,
};
use super::shape::{
    parse_arc_shape, parse_bezier_curve_shape, parse_circle_shape, parse_droplet_shape,
    parse_ellipse_shape, parse_heart_shape, parse_isogon_shape, parse_line_shape,
    parse_polygon_shape, parse_polyline_shape, parse_rect_shape, parse_ring_shape,
    parse_rose_shape, parse_sector_shape, parse_star_shape, parse_trochoid_shape,
    shape_from_opts,
};
use crate::element::pending::{PendingData, PendingPath, PendingText};

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

fn parse_shape(type_name: &str, shape: &JsValue) -> Result<Shape, JsValue> {
    match type_name {
        "rect" => Ok(Shape::Rect(parse_rect_shape(shape)?)),
        "circle" => Ok(Shape::Circle(parse_circle_shape(shape)?)),
        "line" => Ok(Shape::Line(parse_line_shape(shape)?)),
        "polygon" => Ok(Shape::Polygon(parse_polygon_shape(shape)?)),
        "polyline" => Ok(Shape::Polyline(parse_polyline_shape(shape)?)),
        "sector" => Ok(Shape::Sector(parse_sector_shape(shape)?)),
        "arc" => Ok(Shape::Arc(parse_arc_shape(shape)?)),
        "ellipse" => Ok(Shape::Ellipse(parse_ellipse_shape(shape)?)),
        "ring" => Ok(Shape::Ring(parse_ring_shape(shape)?)),
        "bezier-curve" => Ok(Shape::BezierCurve(parse_bezier_curve_shape(shape)?)),
        "isogon" => Ok(Shape::Isogon(parse_isogon_shape(shape)?)),
        "star" => Ok(Shape::Star(parse_star_shape(shape)?)),
        "heart" => Ok(Shape::Heart(parse_heart_shape(shape)?)),
        "droplet" => Ok(Shape::Droplet(parse_droplet_shape(shape)?)),
        "rose" => Ok(Shape::Rose(parse_rose_shape(shape)?)),
        "trochoid" => Ok(Shape::Trochoid(parse_trochoid_shape(shape)?)),
        other => Err(JsValue::from_str(&format!("unsupported shape: {other}"))),
    }
}
