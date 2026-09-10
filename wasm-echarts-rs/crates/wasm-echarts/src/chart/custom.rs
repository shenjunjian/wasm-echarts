//! custom：`renderItem(params, api)` → graphic 进同一 Zr

use js_sys::{Array, Object, Reflect};
use rust_zrender::ZRenderer;
use wasm_bindgen::prelude::*;

use crate::bridge::JsCallback;
use crate::chart::graphic::add_graphic_element;
use crate::chart::path_util::raw_numbers;
use crate::coord::{CalendarCoord, GeoCoord, ParallelCoord, PolarCoord, SeriesCoord, SingleCoord};
use crate::model::{CoordSysKind, GlobalModel, SeriesModel};
use crate::option::{parse_option_value, OptionValue};
use crate::visual::VisualContext;

const KIND_CART: u8 = 0;
const KIND_POLAR: u8 = 1;
const KIND_GEO: u8 = 2;
const KIND_CAL: u8 = 3;
const KIND_SINGLE: u8 = 4;

#[wasm_bindgen]
pub struct CustomSeriesApi {
    kind: u8,
    gx: f64,
    gy: f64,
    gw: f64,
    gh: f64,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    xcat: bool,
    xn: f64,
    cx: f64,
    cy: f64,
    r: f64,
    r0: f64,
    cal_start: f64,
    cal_cell_w: f64,
    cal_cell_h: f64,
    cal_first: f64,
    cal_horizontal: bool,
    values: Vec<f64>,
    fill: String,
}

#[wasm_bindgen]
impl CustomSeriesApi {
    #[wasm_bindgen]
    pub fn coord(&self, data: JsValue) -> Array {
        let (x, y) = js_xy(&data);
        let (px, py) = self.data_to_point(x, y);
        let arr = Array::new();
        arr.push(&JsValue::from(px));
        arr.push(&JsValue::from(py));
        arr
    }

    #[wasm_bindgen]
    pub fn size(&self, data_size: JsValue, data_item: JsValue) -> Array {
        let (dw, dh) = js_xy(&data_size);
        let (x, y) = js_xy(&data_item);
        let (x0, y0) = self.data_to_point(x - dw / 2.0, y - dh / 2.0);
        let (x1, y1) = self.data_to_point(x + dw / 2.0, y + dh / 2.0);
        let arr = Array::new();
        arr.push(&JsValue::from((x1 - x0).abs().max(1.0)));
        arr.push(&JsValue::from((y1 - y0).abs().max(1.0)));
        arr
    }

    #[wasm_bindgen]
    pub fn style(&self, extra: JsValue) -> JsValue {
        let obj = Object::new();
        let _ = Reflect::set(&obj, &JsValue::from_str("fill"), &JsValue::from_str(&self.fill));
        if extra.is_object() {
            if let Ok(keys) = Reflect::own_keys(&extra) {
                for i in 0..keys.length() {
                    let k = keys.get(i);
                    if let Ok(v) = Reflect::get(&extra, &k) {
                        let _ = Reflect::set(&obj, &k, &v);
                    }
                }
            }
        }
        obj.into()
    }

    #[wasm_bindgen]
    pub fn value(&self, dim: JsValue) -> f64 {
        let i = dim.as_f64().unwrap_or(0.0).max(0.0) as usize;
        self.values.get(i).copied().unwrap_or(0.0)
    }
}

impl CustomSeriesApi {
    pub(crate) fn data_to_point(&self, x: f64, y: f64) -> (f64, f64) {
        match self.kind {
            KIND_POLAR => {
                let t = ((x - self.xmin) / (self.xmax - self.xmin).abs().max(1e-9)).clamp(0.0, 1.0);
                let radius = self.r0 + t * (self.r - self.r0);
                let ang = self.ymin + (y - self.ymin) / (self.ymax - self.ymin).abs().max(1e-9)
                    * (self.ymax - self.ymin);
                let rad = -ang * std::f64::consts::PI / 180.0;
                (self.cx + radius * rad.cos(), self.cy + radius * rad.sin())
            }
            KIND_GEO => {
                let px = self.gx + ((x - self.xmin) / (self.xmax - self.xmin).abs().max(1e-9)) * self.gw;
                let py = self.gy + self.gh
                    - ((y - self.ymin) / (self.ymax - self.ymin).abs().max(1e-9)) * self.gh;
                (px, py)
            }
            KIND_CAL => {
                let day = ((x - self.cal_start) / 86_400_000.0).floor();
                let loc = self.cal_first + day;
                let week = (loc / 7.0).floor();
                let weekday = loc.rem_euclid(7.0);
                if self.cal_horizontal {
                    (
                        self.gx + (week + 0.5) * self.cal_cell_w,
                        self.gy + (weekday + 0.5) * self.cal_cell_h,
                    )
                } else {
                    (
                        self.gx + (weekday + 0.5) * self.cal_cell_w,
                        self.gy + (week + 0.5) * self.cal_cell_h,
                    )
                }
            }
            KIND_SINGLE => {
                let t = ((x - self.xmin) / (self.xmax - self.xmin).abs().max(1e-9)).clamp(0.0, 1.0);
                (self.gx + t * self.gw, self.gy + self.gh * 0.5)
            }
            _ => {
                let px = if self.xcat && self.xn > 0.0 {
                    self.gx + (x + 0.5) / self.xn * self.gw
                } else {
                    self.gx + ((x - self.xmin) / (self.xmax - self.xmin).abs().max(1e-9)) * self.gw
                };
                let py = self.gy + self.gh
                    - ((y - self.ymin) / (self.ymax - self.ymin).abs().max(1e-9)) * self.gh;
                (px, py)
            }
        }
    }
}

pub fn render_custom_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let Some(OptionValue::Function(func)) = visual
        .series_option(series.index)
        .and_then(|s| s.get("renderItem"))
    else {
        return;
    };
    let callback = JsCallback::new(func.clone());
    let pw = model.width as f64;
    let ph = model.height as f64;
    for (i, point) in series.data.iter().enumerate() {
        let mut api = api_from_coord(model, series, coord, visual, i);
        api.values = raw_numbers(&point.raw);
        if api.values.is_empty() {
            api.values = vec![point.x_value.unwrap_or(i as f64), point.value];
        }
        api.fill = visual.resolve_item_color(series.index, i);
        let params = visual.data_params(series.index, i);
        attach_coord_sys(&params, &api);
        let api_js: JsValue = api.into();
        match callback.call_render_item(&params, &api_js) {
            Ok(spec) => match parse_option_value(&spec) {
                Ok(value) => add_custom_output(zr, group, &value, pw, ph),
                Err(err) => {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::error_2(&JsValue::from_str("custom renderItem parse:"), &err);
                    let _ = err;
                }
            },
            Err(err) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::error_2(&JsValue::from_str("custom renderItem:"), &err);
                let _ = err;
            }
        }
    }
}

fn add_custom_output(zr: &mut ZRenderer, group: usize, value: &OptionValue, pw: f64, ph: f64) {
    match value {
        OptionValue::Array(arr) => {
            for el in arr {
                add_custom_output(zr, group, el, pw, ph);
            }
        }
        OptionValue::Object(_) => {
            add_graphic_element(zr, group, value, pw, ph);
        }
        _ => {}
    }
}

fn attach_coord_sys(params: &JsValue, api: &CustomSeriesApi) {
    let obj = Object::new();
    let ty = match api.kind {
        KIND_POLAR => "polar",
        KIND_GEO => "geo",
        KIND_CAL => "calendar",
        KIND_SINGLE => "singleAxis",
        _ => "cartesian2d",
    };
    let _ = Reflect::set(&obj, &JsValue::from_str("type"), &JsValue::from_str(ty));
    let _ = Reflect::set(&obj, &JsValue::from_str("x"), &JsValue::from(api.gx));
    let _ = Reflect::set(&obj, &JsValue::from_str("y"), &JsValue::from(api.gy));
    let _ = Reflect::set(&obj, &JsValue::from_str("width"), &JsValue::from(api.gw));
    let _ = Reflect::set(&obj, &JsValue::from_str("height"), &JsValue::from(api.gh));
    let _ = Reflect::set(params, &JsValue::from_str("coordSys"), &obj);
}

fn api_from_coord(
    model: &GlobalModel,
    series: &SeriesModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    data_index: usize,
) -> CustomSeriesApi {
    let mut api = CustomSeriesApi {
        kind: KIND_CART,
        gx: 0.0,
        gy: 0.0,
        gw: model.width as f64,
        gh: model.height as f64,
        xmin: 0.0,
        xmax: 1.0,
        ymin: 0.0,
        ymax: 1.0,
        xcat: false,
        xn: 1.0,
        cx: model.width as f64 * 0.5,
        cy: model.height as f64 * 0.5,
        r: 1.0,
        r0: 0.0,
        cal_start: 0.0,
        cal_cell_w: 10.0,
        cal_cell_h: 10.0,
        cal_first: 0.0,
        cal_horizontal: true,
        values: Vec::new(),
        fill: visual.resolve_item_color(series.index, data_index),
    };
    match series.coord_sys {
        CoordSysKind::Polar => {
            if let Some(p) = PolarCoord::for_series(model, series) {
                api.kind = KIND_POLAR;
                api.cx = p.spec.center_x;
                api.cy = p.spec.center_y;
                api.r = p.spec.r;
                api.r0 = p.spec.r0;
                api.xmin = p.spec.radius_axis.value_min();
                api.xmax = p.spec.radius_axis.value_max();
                api.ymin = p.spec.angle_axis.value_min();
                api.ymax = p.spec.angle_axis.value_max();
                api.gx = p.spec.center_x - p.spec.r;
                api.gy = p.spec.center_y - p.spec.r;
                api.gw = p.spec.r * 2.0;
                api.gh = p.spec.r * 2.0;
            }
        }
        CoordSysKind::Geo => {
            if let Some(g) = GeoCoord::for_series(model, series) {
                api.kind = KIND_GEO;
                api.gx = g.spec.rect.x;
                api.gy = g.spec.rect.y;
                api.gw = g.spec.rect.width;
                api.gh = g.spec.rect.height;
                api.xmin = -180.0;
                api.xmax = 180.0;
                api.ymin = -90.0;
                api.ymax = 90.0;
            }
        }
        CoordSysKind::Calendar => {
            if let Some(c) = CalendarCoord::for_series(model, series) {
                api.kind = KIND_CAL;
                api.gx = c.spec.rect.x;
                api.gy = c.spec.rect.y;
                api.gw = c.spec.rect.width;
                api.gh = c.spec.rect.height;
                api.cal_start = c.spec.start_ms;
                api.cal_cell_w = c.spec.cell_w;
                api.cal_cell_h = c.spec.cell_h;
                api.cal_first = crate::data::weekday_sun0(c.spec.start_ms) as f64 - c.spec.first_day as f64;
                api.cal_horizontal = c.spec.horizontal;
            }
        }
        CoordSysKind::Single => {
            if let Some(s) = SingleCoord::for_series(model, series) {
                api.kind = KIND_SINGLE;
                api.gx = s.spec.rect.x;
                api.gy = s.spec.rect.y;
                api.gw = s.spec.rect.width;
                api.gh = s.spec.rect.height;
                api.xmin = s.spec.axis.value_min();
                api.xmax = s.spec.axis.value_max();
            }
        }
        CoordSysKind::Parallel => {
            if let Some(p) = ParallelCoord::for_series(model, series) {
                api.gx = p.spec.rect.x;
                api.gy = p.spec.rect.y;
                api.gw = p.spec.rect.width;
                api.gh = p.spec.rect.height;
            }
        }
        _ => {
            if let Some(c) = coord.as_cartesian() {
                let g = c.grid();
                api.gx = g.x;
                api.gy = g.y;
                api.gw = g.width;
                api.gh = g.height;
                api.xmin = c.x_axis().value_min();
                api.xmax = c.x_axis().value_max();
                api.ymin = c.y_axis().value_min();
                api.ymax = c.y_axis().value_max();
                api.xcat = c.x_axis().axis_type.is_category();
                api.xn = c.x_axis().category_data.len().max(1) as f64;
            }
        }
    }
    api
}

fn js_xy(value: &JsValue) -> (f64, f64) {
    if Array::is_array(value) {
        let arr = Array::from(value);
        return (
            arr.get(0).as_f64().unwrap_or(0.0),
            arr.get(1).as_f64().unwrap_or(0.0),
        );
    }
    let n = value.as_f64().unwrap_or(0.0);
    (n, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian_coord_maps_corners() {
        let api = CustomSeriesApi {
            kind: KIND_CART,
            gx: 10.0,
            gy: 20.0,
            gw: 100.0,
            gh: 50.0,
            xmin: 0.0,
            xmax: 10.0,
            ymin: 0.0,
            ymax: 10.0,
            xcat: false,
            xn: 1.0,
            cx: 0.0,
            cy: 0.0,
            r: 1.0,
            r0: 0.0,
            cal_start: 0.0,
            cal_cell_w: 10.0,
            cal_cell_h: 10.0,
            cal_first: 0.0,
            cal_horizontal: true,
            values: vec![0.0, 10.0],
            fill: "#5470c6".into(),
        };
        let (x0, y0) = api.data_to_point(0.0, 0.0);
        let (x1, y1) = api.data_to_point(10.0, 10.0);
        assert!((x0 - 10.0).abs() < 1e-6);
        assert!((y0 - 70.0).abs() < 1e-6);
        assert!((x1 - 110.0).abs() < 1e-6);
        assert!((y1 - 20.0).abs() < 1e-6);
    }
}
