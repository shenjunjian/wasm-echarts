//! 从 JsValue 解析各 shape 字段

use rust_zrender::{
    ArcShape, BezierCurveShape, CircleShape, CompoundPathShape, DropletShape, EllipseShape,
    HeartShape, IsogonShape, LineShape, PathDataShape, PolygonShape, PolylineShape, RectShape,
    RingShape, RoseShape, SectorShape, Shape, StarShape, TrochoidShape,
};
use wasm_bindgen::prelude::*;

use super::opts::{get_bool, get_f64, get_object, get_string, get_u32, get_value};

fn get_opt_f64(obj: &JsValue, key: &str) -> Option<f64> {
    let v = get_value(obj, key);
    if v.is_undefined() || v.is_null() {
        None
    } else {
        v.as_f64()
    }
}

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

pub fn parse_arc_shape(shape: &JsValue) -> Result<ArcShape, JsValue> {
    Ok(ArcShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        r: get_f64(shape, "r").unwrap_or(0.0),
        start_angle: get_f64(shape, "startAngle").unwrap_or(0.0),
        end_angle: get_f64(shape, "endAngle").unwrap_or(std::f64::consts::PI * 2.0),
        clockwise: get_bool(shape, "clockwise").unwrap_or(true),
    })
}

pub fn parse_ellipse_shape(shape: &JsValue) -> Result<EllipseShape, JsValue> {
    Ok(EllipseShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        rx: get_f64(shape, "rx").unwrap_or(0.0),
        ry: get_f64(shape, "ry").unwrap_or(0.0),
    })
}

pub fn parse_ring_shape(shape: &JsValue) -> Result<RingShape, JsValue> {
    Ok(RingShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        r: get_f64(shape, "r").unwrap_or(0.0),
        r0: get_f64(shape, "r0").unwrap_or(0.0),
    })
}

pub fn parse_bezier_curve_shape(shape: &JsValue) -> Result<BezierCurveShape, JsValue> {
    Ok(BezierCurveShape {
        x1: get_f64(shape, "x1").unwrap_or(0.0),
        y1: get_f64(shape, "y1").unwrap_or(0.0),
        x2: get_f64(shape, "x2").unwrap_or(0.0),
        y2: get_f64(shape, "y2").unwrap_or(0.0),
        cpx1: get_f64(shape, "cpx1").unwrap_or(0.0),
        cpy1: get_f64(shape, "cpy1").unwrap_or(0.0),
        cpx2: get_opt_f64(shape, "cpx2"),
        cpy2: get_opt_f64(shape, "cpy2"),
        percent: get_f64(shape, "percent").unwrap_or(1.0),
    })
}

fn get_f64_array(obj: &JsValue, key: &str) -> Vec<f64> {
    let val = get_value(obj, key);
    let Some(arr) = val.dyn_ref::<js_sys::Array>() else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(arr.length() as usize);
    for i in 0..arr.length() {
        if let Some(n) = arr.get(i).as_f64() {
            out.push(n);
        }
    }
    out
}

pub fn parse_isogon_shape(shape: &JsValue) -> Result<IsogonShape, JsValue> {
    Ok(IsogonShape {
        x: get_f64(shape, "x").unwrap_or(0.0),
        y: get_f64(shape, "y").unwrap_or(0.0),
        r: get_f64(shape, "r").unwrap_or(0.0),
        n: get_u32(shape, "n").unwrap_or(0),
    })
}

pub fn parse_star_shape(shape: &JsValue) -> Result<StarShape, JsValue> {
    Ok(StarShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        n: get_u32(shape, "n").unwrap_or(3),
        r0: get_opt_f64(shape, "r0"),
        r: get_f64(shape, "r").unwrap_or(0.0),
    })
}

pub fn parse_heart_shape(shape: &JsValue) -> Result<HeartShape, JsValue> {
    Ok(HeartShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        width: get_f64(shape, "width").unwrap_or(0.0),
        height: get_f64(shape, "height").unwrap_or(0.0),
    })
}

pub fn parse_droplet_shape(shape: &JsValue) -> Result<DropletShape, JsValue> {
    Ok(DropletShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        width: get_f64(shape, "width").unwrap_or(0.0),
        height: get_f64(shape, "height").unwrap_or(0.0),
    })
}

pub fn parse_rose_shape(shape: &JsValue) -> Result<RoseShape, JsValue> {
    Ok(RoseShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        r: get_f64_array(shape, "r"),
        k: get_f64(shape, "k").unwrap_or(0.0),
        n: get_u32(shape, "n").unwrap_or(1),
    })
}

pub fn parse_trochoid_shape(shape: &JsValue) -> Result<TrochoidShape, JsValue> {
    Ok(TrochoidShape {
        cx: get_f64(shape, "cx").unwrap_or(0.0),
        cy: get_f64(shape, "cy").unwrap_or(0.0),
        r: get_f64(shape, "r").unwrap_or(0.0),
        r0: get_f64(shape, "r0").unwrap_or(0.0),
        d: get_f64(shape, "d").unwrap_or(0.0),
        location: get_string(shape, "location").unwrap_or_else(|| "out".to_string()),
    })
}

pub fn parse_path_data_shape(shape: &JsValue) -> Result<PathDataShape, JsValue> {
    Ok(PathDataShape {
        path_data: get_string(shape, "pathData").unwrap_or_default(),
    })
}

pub fn parse_compound_path_shape(shape: &JsValue) -> Result<CompoundPathShape, JsValue> {
    let paths_val = get_value(shape, "paths");
    let Some(arr) = paths_val.dyn_ref::<js_sys::Array>() else {
        return Ok(CompoundPathShape { shapes: Vec::new() });
    };
    let mut shapes = Vec::with_capacity(arr.length() as usize);
    for i in 0..arr.length() {
        if let Some(sub) = parse_compound_subpath(&arr.get(i))? {
            shapes.push(sub);
        }
    }
    Ok(CompoundPathShape { shapes })
}

fn parse_compound_subpath(item: &JsValue) -> Result<Option<Shape>, JsValue> {
    if let Some(path_data) = get_string(item, "pathData") {
        return Ok(Some(Shape::PathData(PathDataShape { path_data })));
    }
    let type_name = get_string(item, "type");
    let shape_obj = {
        let nested = get_object(item, "shape");
        if nested.is_undefined() {
            item.clone()
        } else {
            nested
        }
    };
    if let Some(type_name) = type_name {
        return parse_shape_by_type(&type_name, &shape_obj).map(Some);
    }
    if get_string(&shape_obj, "pathData").is_some() {
        return Ok(Some(Shape::PathData(parse_path_data_shape(&shape_obj)?)));
    }
    Ok(None)
}

pub fn parse_shape_by_type(type_name: &str, shape: &JsValue) -> Result<Shape, JsValue> {
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
        "path" => Ok(Shape::PathData(parse_path_data_shape(shape)?)),
        other => Err(JsValue::from_str(&format!("unsupported shape: {other}"))),
    }
}

/// 从 element opts 中取 shape 子对象
pub fn shape_from_opts(opts: &JsValue) -> JsValue {
    get_object(opts, "shape")
}
