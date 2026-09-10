//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_echarts::{register_font, EChartsInstance};

const TEST_FONT: &[u8] = include_bytes!("../../rust-zrender/tests/fixtures/NotoSansSC-Regular.ttf");

wasm_bindgen_test_configure!(run_in_browser);

fn js_obj(pairs: &[(&str, JsValue)]) -> JsValue {
    let obj = Object::new();
    for (key, value) in pairs {
        Reflect::set(&obj, &JsValue::from_str(key), value).unwrap();
    }
    obj.into()
}

fn register_test_font() {
    let opts = js_obj(&[
        ("familyName", JsValue::from_str("Noto Sans SC")),
        (
            "sansSerif",
            {
                let names = Array::new();
                names.push(&JsValue::from_str("Noto Sans SC"));
                names.into()
            },
        ),
    ]);
    register_font(TEST_FONT, opts).unwrap();
}

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn axis_labels_refresh_after_register_font() {
    register_test_font();
    let mut chart = EChartsInstance::new(240, 180, 1.0).unwrap();
    let option = js_obj(&[
        (
            "xAxis",
            js_obj(&[
                ("type", JsValue::from_str("category")),
                (
                    "data",
                    {
                        let data = Array::new();
                        data.push(&JsValue::from_str("A"));
                        data.push(&JsValue::from_str("B"));
                        data.into()
                    },
                ),
            ]),
        ),
        ("yAxis", js_obj(&[("type", JsValue::from_str("value"))])),
        (
            "series",
            {
                let series = Array::new();
                series.push(&js_obj(&[
                    ("type", JsValue::from_str("bar")),
                    (
                        "data",
                        {
                            let data = Array::new();
                            data.push(&JsValue::from(10));
                            data.push(&JsValue::from(20));
                            data.into()
                        },
                    ),
                ]));
                series.into()
            },
        ),
    ]);
    chart.set_option(option, None).unwrap();
    let rgba = chart.refresh().expect("refresh should not panic after registerFont");
    assert!(!rgba.is_empty());
}

fn js_dirty(value: &JsValue) -> bool {
    Reflect::get(value, &JsValue::from_str("dirty"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

#[wasm_bindgen_test]
fn pointer_move_skips_repaint_when_hover_unchanged() {
    register_test_font();
    let mut chart = EChartsInstance::new(200, 200, 1.0).unwrap();
    let option = js_obj(&[
        (
            "series",
            {
                let series = Array::new();
                series.push(&js_obj(&[
                    ("type", JsValue::from_str("pie")),
                    (
                        "data",
                        {
                            let data = Array::new();
                            data.push(&js_obj(&[
                                ("name", JsValue::from_str("A")),
                                ("value", JsValue::from(40)),
                            ]));
                            data.push(&js_obj(&[
                                ("name", JsValue::from_str("B")),
                                ("value", JsValue::from(60)),
                            ]));
                            data.into()
                        },
                    ),
                ]));
                series.into()
            },
        ),
    ]);
    chart.set_option(option, None).unwrap();
    let first = chart.handle_pointer_move(140.0, 100.0);
    let second = chart.handle_pointer_move(141.0, 101.0);
    assert!(
        !js_dirty(&second),
        "same hover target should not mark dirty; first dirty={}",
        js_dirty(&first)
    );
}
