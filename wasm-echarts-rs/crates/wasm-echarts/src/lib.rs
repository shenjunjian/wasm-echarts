mod bridge;
mod chart;
mod coord;
mod data;
mod instance;
mod interaction;
mod maps;
mod model;
mod option;
mod render;
mod scheduler;
mod utils;
mod visual;

use wasm_bindgen::prelude::*;
#[cfg(feature = "chart-custom")]
pub use chart::CustomSeriesApi;
pub use instance::EChartsInstance;
pub use wasm_zrender::{
    clear_fonts, register_font, Animation, Animator, Arc, BezierCurve, BoundingRect, Circle,
    CompoundPath, Displayable, Droplet, Ellipse, Group, Handler, Heart, HoverResult, Image,
    IncrementalDisplayable, Isogon, Line, LinearGradient, OrientedBoundingRect, Path, Pattern,
    Point, Polygon, Polyline, RadialGradient, Rect, Ring, Rose, Sector, Star, Text, Trochoid,
    TSpan, ZRender,
};

#[wasm_bindgen(js_name = registerTransform)]
pub fn register_transform(ty: String, func: js_sys::Function) {
    crate::data::register_transform(ty, func);
}

#[wasm_bindgen(js_name = registerMap)]
pub fn register_map(name: String, geo_json: JsValue, special_areas: JsValue) {
    let geo = crate::option::parse_option_value(&geo_json).unwrap_or(crate::option::OptionValue::Null);
    let special =
        crate::option::parse_option_value(&special_areas).unwrap_or(crate::option::OptionValue::Null);
    crate::maps::register_map(name, geo, special);
}

#[wasm_bindgen(js_name = parseGeoJSON)]
pub fn parse_geo_json(geo_json: JsValue, name_property: JsValue) -> JsValue {
    let Ok(geo) = crate::option::parse_option_value(&geo_json) else {
        return js_sys::Array::new().into();
    };
    let name_prop = name_property.as_string().unwrap_or_else(|| "name".into());
    let regions = crate::maps::parse_geojson(&geo, &name_prop);
    let arr = js_sys::Array::new();
    for region in regions {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("name"), &JsValue::from_str(&region.name));
        if let Some((lng, lat)) = region.center {
            let cp = js_sys::Array::new();
            cp.push(&JsValue::from(lng));
            cp.push(&JsValue::from(lat));
            let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("center"), &cp);
        }
        let polys = js_sys::Array::new();
        for poly in region.polygons {
            let ring = js_sys::Array::new();
            for (lng, lat) in poly {
                let pt = js_sys::Array::new();
                pt.push(&JsValue::from(lng));
                pt.push(&JsValue::from(lat));
                ring.push(&pt);
            }
            polys.push(&ring);
        }
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("polygons"), &polys);
        arr.push(&obj);
    }
    arr.into()
}

#[wasm_bindgen(js_name = parseGeoJson)]
pub fn parse_geo_json_alias(geo_json: JsValue, name_property: JsValue) -> JsValue {
    parse_geo_json(geo_json, name_property)
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}
