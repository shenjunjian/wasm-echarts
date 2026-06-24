//! LinearGradient / RadialGradient / Pattern 样式对象（对齐 zrender export.ts）

use std::sync::Arc;

use js_sys::Array;
use js_sys::Uint8Array;
use rust_zrender::ColorStop;
use wasm_bindgen::prelude::*;

use crate::bridge::fill_stroke::{decode_pattern_image, parse_color_stops};
#[wasm_bindgen]
pub struct LinearGradient {
    x: f64,
    y: f64,
    x2: f64,
    y2: f64,
    color_stops: Vec<ColorStop>,
    global: bool,
}

#[wasm_bindgen]
impl LinearGradient {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f64,
        y: f64,
        x2: f64,
        y2: f64,
        color_stops: Option<JsValue>,
        global_coord: Option<bool>,
    ) -> LinearGradient {
        let stops_value = color_stops.unwrap_or(JsValue::UNDEFINED);
        LinearGradient {
            x: if x.is_nan() { 0.0 } else { x },
            y: if y.is_nan() { 0.0 } else { y },
            x2: if x2.is_nan() { 1.0 } else { x2 },
            y2: if y2.is_nan() { 0.0 } else { y2 },
            color_stops: parse_color_stops(&stops_value),
            global: global_coord.unwrap_or(false),
        }
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn gradient_type(&self) -> String {
        "linear".into()
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn x2(&self) -> f64 {
        self.x2
    }

    #[wasm_bindgen(getter)]
    pub fn y2(&self) -> f64 {
        self.y2
    }

    #[wasm_bindgen(getter)]
    pub fn global(&self) -> bool {
        self.global
    }

    #[wasm_bindgen(getter, js_name = colorStops)]
    pub fn color_stops_js(&self) -> JsValue {
        color_stops_to_js(&self.color_stops)
    }
}

#[wasm_bindgen]
pub struct RadialGradient {
    x: f64,
    y: f64,
    r: f64,
    r0: f64,
    color_stops: Vec<ColorStop>,
    global: bool,
}

#[wasm_bindgen]
impl RadialGradient {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f64,
        y: f64,
        r: f64,
        color_stops: Option<JsValue>,
        global_coord: Option<bool>,
    ) -> RadialGradient {
        let stops_value = color_stops.unwrap_or(JsValue::UNDEFINED);
        RadialGradient {
            x: if x.is_nan() { 0.5 } else { x },
            y: if y.is_nan() { 0.5 } else { y },
            r: if r.is_nan() { 0.5 } else { r },
            r0: 0.0,
            color_stops: parse_color_stops(&stops_value),
            global: global_coord.unwrap_or(false),
        }
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn gradient_type(&self) -> String {
        "radial".into()
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn r(&self) -> f64 {
        self.r
    }

    #[wasm_bindgen(getter)]
    pub fn r0(&self) -> f64 {
        self.r0
    }

    #[wasm_bindgen(getter)]
    pub fn global(&self) -> bool {
        self.global
    }

    #[wasm_bindgen(getter, js_name = colorStops)]
    pub fn color_stops_js(&self) -> JsValue {
        color_stops_to_js(&self.color_stops)
    }
}

#[wasm_bindgen]
pub struct Pattern {
    data: Arc<[u8]>,
    width: u32,
    height: u32,
    repeat: String,
    x: f64,
    y: f64,
    scale_x: f64,
    scale_y: f64,
    rotation: f64,
}

#[wasm_bindgen]
impl Pattern {
    #[wasm_bindgen(constructor)]
    pub fn new(image: JsValue, repeat: Option<String>) -> Result<Pattern, JsValue> {
        let (data, width, height) = decode_pattern_image(&image, None, None)?;
        Ok(Pattern {
            data,
            width,
            height,
            repeat: repeat.unwrap_or_else(|| "repeat".into()),
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
        })
    }

    #[wasm_bindgen(getter, js_name = type)]
    pub fn pattern_type(&self) -> String {
        "pattern".into()
    }

    #[wasm_bindgen(getter)]
    pub fn repeat(&self) -> String {
        self.repeat.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[wasm_bindgen(getter, js_name = scaleX)]
    pub fn scale_x(&self) -> f64 {
        self.scale_x
    }

    #[wasm_bindgen(getter, js_name = scaleY)]
    pub fn scale_y(&self) -> f64 {
        self.scale_y
    }

    #[wasm_bindgen(getter)]
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    #[wasm_bindgen(getter, js_name = imageWidth)]
    pub fn image_width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter, js_name = imageHeight)]
    pub fn image_height(&self) -> u32 {
        self.height
    }

    #[wasm_bindgen(getter, js_name = imageData)]
    pub fn image_data_js(&self) -> Uint8Array {
        Uint8Array::from(self.data.as_ref())
    }
}

fn color_stops_to_js(stops: &[ColorStop]) -> JsValue {
    let array = Array::new();
    for stop in stops {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"offset".into(), &JsValue::from(stop.offset)).unwrap();
        js_sys::Reflect::set(&obj, &"color".into(), &JsValue::from_str(&stop.color)).unwrap();
        array.push(&obj);
    }
    array.into()
}
