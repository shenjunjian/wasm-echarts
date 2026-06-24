//! wasm-bindgen 浏览器端集成测试

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use wasm_zrender::{dispose_all, init, Arc, Circle, Group, Rect, Text};

wasm_bindgen_test_configure!(run_in_node_experimental);

fn reset_registry() {
    dispose_all();
}

fn init_opts(width: u32, height: u32) -> JsValue {
    let opts = Object::new();
    Reflect::set(&opts, &"width".into(), &JsValue::from(width)).unwrap();
    Reflect::set(&opts, &"height".into(), &JsValue::from(height)).unwrap();
    Reflect::set(&opts, &"devicePixelRatio".into(), &JsValue::from(1.0)).unwrap();
    opts.into()
}

fn rect_opts() -> JsValue {
    let opts = Object::new();

    let shape = Object::new();
    Reflect::set(&shape, &"x".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&shape, &"y".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&shape, &"width".into(), &JsValue::from(100.0)).unwrap();
    Reflect::set(&shape, &"height".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();

    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#5470c6")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    opts.into()
}

fn circle_opts() -> JsValue {
    let opts = Object::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(180.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();

    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#91cc75")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();
    Reflect::set(&opts, &"seriesIndex".into(), &JsValue::from(0)).unwrap();
    Reflect::set(&opts, &"dataIndex".into(), &JsValue::from(1)).unwrap();

    opts.into()
}

#[wasm_bindgen_test]
fn init_group_rect_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();
    let rect = Rect::new(rect_opts()).unwrap();
    g.add(JsValue::from(rect)).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn find_hover_returns_element_handle() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();
    let circle = Circle::new(circle_opts()).unwrap();
    g.add(JsValue::from(circle)).unwrap();
    zr.add(JsValue::from(g)).unwrap();
    zr.refresh().unwrap();

    let hover = zr.find_hover(180.0, 80.0).expect("hover result");
    assert_eq!(hover.target().element_type(), "circle");
    assert_eq!(hover.top_target().element_type(), "circle");
}

#[wasm_bindgen_test]
fn text_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(480, 180)).unwrap();

    let style = Object::new();
    Reflect::set(&style, &"text".into(), &JsValue::from_str("wasm-zrender 文本")).unwrap();
    Reflect::set(&style, &"x".into(), &JsValue::from(24.0)).unwrap();
    Reflect::set(&style, &"y".into(), &JsValue::from(48.0)).unwrap();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#333")).unwrap();
    Reflect::set(&style, &"fontSize".into(), &JsValue::from(18.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    let text = Text::new(opts.into());
    zr.add(JsValue::from(text)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 480 * 180 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn stub_arc_constructor_throws() {
    reset_registry();
    match Arc::new(JsValue::NULL) {
        Err(err) => {
            let message = err.as_string().unwrap_or_default();
            assert!(message.contains("Arc"));
            assert!(message.contains("not implemented"));
        }
        Ok(_) => panic!("Arc constructor should throw"),
    }
}

#[wasm_bindgen_test]
fn rect_set_state_style_and_use_state() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(200, 100)).unwrap();
    let rect = Rect::new(rect_opts()).unwrap();
    rect.set_state_style("emphasis", emphasis_style()).unwrap();
    rect.use_state("emphasis").unwrap();
    zr.add(JsValue::from(rect)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

fn emphasis_style() -> JsValue {
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#ee6666")).unwrap();
    Reflect::set(&style, &"lineWidth".into(), &JsValue::from(4.0)).unwrap();
    style.into()
}
