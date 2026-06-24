//! 从 JsValue 解析各 shape 字段

use rust_zrender::{
    CircleShape, LineShape, PolygonShape, PolylineShape, RectShape, SectorShape,
};
use wasm_bindgen::prelude::*;

use super::opts::{get_f64, get_object, get_value};

pub fn parse_rect_shape(shape: &JsValue) -> Result<RectShape, JsValue> {
    Ok(RectShape {
        x: get_f64(shape, "x").unwrap_or(0.0),
        y: get_f64(shape, "y").unwrap_or(0.0),
        width: get_f64(shape, "width").unwrap_or(0.0),
        height: get_f64(shape, "height").unwrap_or(0.0),
    })
}

pub fn parse_circle_shape(shape: &JsValue) -> Result<CircleShape, JsValue> {
    Ok(CircleShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        r: get_f64(shape, "r").unwrap_or(0.0),
    })
}

pub fn parse_line_shape(shape: &JsValue) -> Result<LineShape, JsValue> {
    Ok(LineShape {
        x1: get_f64(shape, "x1").unwrap_or(0.0),
        y1: get_f64(shape, "y1").unwrap_or(0.0),
        x2: get_f64(shape, "x2").unwrap_or(0.0),
        y2: get_f64(shape, "y2").unwrap_or(0.0),
        percent: get_f64(shape, "percent").unwrap_or(1.0),
    })
}

pub fn parse_polygon_shape(shape: &JsValue) -> Result<PolygonShape, JsValue> {
    let points_val = get_value(shape, "points");
    let mut points = Vec::new();
    if let Some(arr) = points_val.dyn_ref::<js_sys::Array>() {
        for i in 0..arr.length() {
            let pt = arr.get(i);
            if let Some(pair) = pt.dyn_ref::<js_sys::Array>() {
                if pair.length() >= 2 {
                    let x = pair.get(0).as_f64().unwrap_or(0.0);
                    let y = pair.get(1).as_f64().unwrap_or(0.0);
                    points.push((x, y));
                }
            }
        }
    }
    Ok(PolygonShape { points })
}

pub fn parse_polyline_shape(shape: &JsValue) -> Result<PolylineShape, JsValue> {
    let polygon = parse_polygon_shape(shape)?;
    Ok(PolylineShape {
        points: polygon.points,
        percent: get_f64(shape, "percent").unwrap_or(1.0),
    })
}

pub fn parse_sector_shape(shape: &JsValue) -> Result<SectorShape, JsValue> {
    Ok(SectorShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        r: get_f64(shape, "r").unwrap_or(0.0),
        start_angle: get_f64(shape, "startAngle").unwrap_or(0.0),
        end_angle: get_f64(shape, "endAngle").unwrap_or(0.0),
        percent: get_f64(shape, "percent").unwrap_or(1.0),
    })
}

/// 从 element opts 中取 shape 子对象
pub fn shape_from_opts(opts: &JsValue) -> JsValue {
    get_object(opts, "shape")
}
