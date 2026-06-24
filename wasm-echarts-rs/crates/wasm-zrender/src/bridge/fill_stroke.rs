//! fill / stroke 样式解析：纯色、渐变、Pattern

use std::sync::Arc;

use js_sys::{Array, Uint8Array};
use rust_zrender::{
    ColorStop, FillStrokeStyle, LinearGradientStyle, PatternStyle, RadialGradientStyle,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::bridge::opts::{get_bool, get_f64, get_string, get_value};

pub fn parse_fill_stroke(value: &JsValue) -> FillStrokeStyle {
    if value.is_null() || value.is_undefined() {
        return FillStrokeStyle::None;
    }
    if let Some(s) = value.as_string() {
        if s == "none" || s.is_empty() {
            return FillStrokeStyle::None;
        }
        return FillStrokeStyle::Color(s);
    }
    if !value.is_object() {
        return FillStrokeStyle::None;
    }

    let type_name = get_string(value, "type");
    match type_name.as_deref() {
        Some("linear") => parse_linear_gradient(value),
        Some("radial") => parse_radial_gradient(value),
        Some("pattern") => parse_pattern(value),
        _ => {
            if get_value(value, "image").is_object() || get_value(value, "image").is_string() {
                parse_pattern(value)
            } else if get_f64(value, "r").is_some() {
                parse_radial_gradient(value)
            } else if get_f64(value, "x2").is_some() || get_f64(value, "y2").is_some() {
                parse_linear_gradient(value)
            } else {
                FillStrokeStyle::None
            }
        }
    }
}

fn parse_linear_gradient(obj: &JsValue) -> FillStrokeStyle {
    let stops = parse_color_stops(&get_value(obj, "colorStops"));
    if stops.is_empty() {
        return FillStrokeStyle::None;
    }
    FillStrokeStyle::LinearGradient(LinearGradientStyle {
        x: get_f64(obj, "x").unwrap_or(0.0),
        y: get_f64(obj, "y").unwrap_or(0.0),
        x2: get_f64(obj, "x2").unwrap_or(1.0),
        y2: get_f64(obj, "y2").unwrap_or(0.0),
        color_stops: stops,
        global: get_bool(obj, "global").unwrap_or(false),
    })
}

fn parse_radial_gradient(obj: &JsValue) -> FillStrokeStyle {
    let stops = parse_color_stops(&get_value(obj, "colorStops"));
    if stops.is_empty() {
        return FillStrokeStyle::None;
    }
    FillStrokeStyle::RadialGradient(RadialGradientStyle {
        x: get_f64(obj, "x").unwrap_or(0.5),
        y: get_f64(obj, "y").unwrap_or(0.5),
        r: get_f64(obj, "r").unwrap_or(0.5),
        r0: get_f64(obj, "r0").unwrap_or(0.0),
        color_stops: stops,
        global: get_bool(obj, "global").unwrap_or(false),
    })
}

fn parse_pattern(obj: &JsValue) -> FillStrokeStyle {
    let image = get_value(obj, "image");
    let width = get_u32(obj, "imageWidth");
    let height = get_u32(obj, "imageHeight");

    let (data, w, h) = if !image.is_undefined() && !image.is_null() {
        match decode_pattern_image(&image, width, height) {
            Ok(v) => v,
            Err(_) => return FillStrokeStyle::None,
        }
    } else {
        let image_data = get_value(obj, "imageData");
        match decode_pattern_image(&image_data, width, height) {
            Ok(v) => v,
            Err(_) => return FillStrokeStyle::None,
        }
    };

    if data.is_empty() || w == 0 || h == 0 {
        return FillStrokeStyle::None;
    }

    FillStrokeStyle::Pattern(PatternStyle {
        data,
        width: w,
        height: h,
        repeat: get_string(obj, "repeat").unwrap_or_else(|| "repeat".into()),
        x: get_f64(obj, "x").unwrap_or(0.0),
        y: get_f64(obj, "y").unwrap_or(0.0),
        scale_x: get_f64(obj, "scaleX").unwrap_or(1.0),
        scale_y: get_f64(obj, "scaleY").unwrap_or(1.0),
        rotation: get_f64(obj, "rotation").unwrap_or(0.0),
    })
}

pub fn parse_color_stops(value: &JsValue) -> Vec<ColorStop> {
    if !value.is_instance_of::<Array>() {
        return Vec::new();
    }
    let array = Array::from(value);
    let mut out = Vec::with_capacity(array.length() as usize);
    for item in array.iter() {
        if !item.is_object() {
            continue;
        }
        let offset = get_f64(&item, "offset").unwrap_or(0.0);
        let color = get_string(&item, "color").unwrap_or_default();
        if color.is_empty() {
            continue;
        }
        out.push(ColorStop { offset, color });
    }
    out
}

fn get_u32(obj: &JsValue, key: &str) -> Option<u32> {
    get_f64(obj, key).map(|n| n as u32)
}

/// 将 JS image 源解码为 RGBA bytes（HTMLImageElement / Canvas / Uint8Array / URL 暂不支持同步）
pub fn decode_pattern_image(
    image: &JsValue,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<(Arc<[u8]>, u32, u32), JsValue> {
    if let Ok(array) = image.clone().dyn_into::<Uint8Array>() {
        let w = width
            .filter(|v| *v > 0)
            .ok_or_else(|| JsValue::from_str("imageWidth required for Uint8Array pattern image"))?;
        let h = height
            .filter(|v| *v > 0)
            .ok_or_else(|| JsValue::from_str("imageHeight required for Uint8Array pattern image"))?;
        let expected = (w as usize) * (h as usize) * 4;
        if array.length() as usize != expected {
            return Err(JsValue::from_str(
                "Uint8Array length does not match imageWidth * imageHeight * 4",
            ));
        }
        let mut data = vec![0u8; expected];
        array.copy_to(&mut data);
        return Ok((Arc::from(data), w, h));
    }

    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::{HtmlCanvasElement, HtmlImageElement};

        if let Ok(canvas) = image.clone().dyn_into::<HtmlCanvasElement>() {
            return canvas_to_rgba(&canvas);
        }
        if let Ok(img) = image.clone().dyn_into::<HtmlImageElement>() {
            return image_element_to_rgba(&img);
        }
    }

    Err(JsValue::from_str("unsupported pattern image source"))
}

#[cfg(target_arch = "wasm32")]
fn image_element_to_rgba(img: &web_sys::HtmlImageElement) -> Result<(Arc<[u8]>, u32, u32), JsValue> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    let w = img.natural_width();
    let h = img.natural_height();
    if w == 0 || h == 0 {
        return Err(JsValue::from_str("image not ready"));
    }

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("no document"))?;
    let canvas = document
        .create_element("canvas")
        .map_err(|_| JsValue::from_str("failed to create canvas"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("canvas cast failed"))?;
    canvas.set_width(w);
    canvas.set_height(h);

    let ctx = canvas
        .get_context("2d")
        .map_err(|_| JsValue::from_str("getContext failed"))?
        .ok_or_else(|| JsValue::from_str("2d context unavailable"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| JsValue::from_str("2d context cast failed"))?;

    ctx.draw_image_with_html_image_element(img, 0.0, 0.0)
        .map_err(|_| JsValue::from_str("drawImage failed"))?;

    canvas_to_rgba(&canvas)
}

#[cfg(target_arch = "wasm32")]
fn canvas_to_rgba(canvas: &web_sys::HtmlCanvasElement) -> Result<(Arc<[u8]>, u32, u32), JsValue> {
    use wasm_bindgen::JsCast;

    let w = canvas.width();
    let h = canvas.height();
    if w == 0 || h == 0 {
        return Err(JsValue::from_str("canvas has zero size"));
    }

    let ctx = canvas
        .get_context("2d")
        .map_err(|_| JsValue::from_str("getContext failed"))?
        .ok_or_else(|| JsValue::from_str("2d context unavailable"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| JsValue::from_str("2d context cast failed"))?;

    let image_data = ctx
        .get_image_data(0.0, 0.0, w as f64, h as f64)
        .map_err(|_| JsValue::from_str("getImageData failed"))?;

    let clamped = image_data.data().0;
    Ok((Arc::from(clamped), w, h))
}
