//! wasm-bindgen 浏览器端集成测试

use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use wasm_zrender::{
    clear_fonts, dispose_all, init, register_font, Arc, BezierCurve, Circle, CompoundPath, Droplet,
    Ellipse, Group, Heart, Image, Isogon, LinearGradient, Path, Rect, Ring, Rose, Star, Text,
    Trochoid,
};

const TEST_FONT: &[u8] = include_bytes!("../tests/fixtures/NotoSansSC-Regular.ttf");

wasm_bindgen_test_configure!(run_in_node_experimental);

fn reset_registry() {
    dispose_all();
    clear_fonts().unwrap();
}

fn register_test_font() {
    let opts = Object::new();
    Reflect::set(&opts, &"familyName".into(), &JsValue::from_str("Noto Sans SC")).unwrap();
    register_font(TEST_FONT, opts.into()).unwrap();
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
fn rect_linear_gradient_fill_outputs_non_monochrome() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let color_stops = Array::new();
    let stop0 = Object::new();
    Reflect::set(&stop0, &"offset".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&stop0, &"color".into(), &JsValue::from_str("#5470c6")).unwrap();
    color_stops.push(&stop0);
    let stop1 = Object::new();
    Reflect::set(&stop1, &"offset".into(), &JsValue::from(1.0)).unwrap();
    Reflect::set(&stop1, &"color".into(), &JsValue::from_str("#91cc75")).unwrap();
    color_stops.push(&stop1);

    let gradient = LinearGradient::new(0.0, 0.0, 1.0, 0.0, Some(color_stops.into()), None);

    let opts = Object::new();
    let shape = Object::new();
    Reflect::set(&shape, &"x".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&shape, &"y".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&shape, &"width".into(), &JsValue::from(100.0)).unwrap();
    Reflect::set(&shape, &"height".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();

    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from(gradient)).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    let rect = Rect::new(opts.into()).unwrap();
    g.add(JsValue::from(rect)).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);

    let mut left_red = 0u8;
    let mut right_green = 0u8;
    for y in 20..80 {
        for x in 20..120 {
            let i = (y * 320 + x) * 4;
            if rgba[i + 3] == 0 {
                continue;
            }
            if x < 60 {
                left_red = rgba[i];
            } else if x > 80 {
                right_green = rgba[i + 1];
            }
        }
    }
    assert!(left_red > 80, "left side should be bluish");
    assert!(right_green > 80, "right side should be greenish");
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
    register_test_font();
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
fn arc_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&shape, &"startAngle".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"endAngle".into(), &JsValue::from(std::f64::consts::PI)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"stroke".into(), &JsValue::from_str("#5470c6")).unwrap();
    Reflect::set(&style, &"lineWidth".into(), &JsValue::from(4.0)).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Arc::new(opts.into()).unwrap())).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn ellipse_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"rx".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"ry".into(), &JsValue::from(50.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#91cc75")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Ellipse::new(opts.into()).unwrap())).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn ring_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&shape, &"r0".into(), &JsValue::from(30.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#fac858")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Ring::new(opts.into()).unwrap())).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn bezier_curve_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"x1".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&shape, &"y1".into(), &JsValue::from(120.0)).unwrap();
    Reflect::set(&shape, &"x2".into(), &JsValue::from(300.0)).unwrap();
    Reflect::set(&shape, &"y2".into(), &JsValue::from(120.0)).unwrap();
    Reflect::set(&shape, &"cpx1".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cpy1".into(), &JsValue::from(20.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"stroke".into(), &JsValue::from_str("#ee6666")).unwrap();
    Reflect::set(&style, &"lineWidth".into(), &JsValue::from(3.0)).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(BezierCurve::new(opts.into()).unwrap()))
        .unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn isogon_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"x".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"y".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(50.0)).unwrap();
    Reflect::set(&shape, &"n".into(), &JsValue::from(6)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#5470c6")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Isogon::new(opts.into()).unwrap()))
        .unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn star_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"n".into(), &JsValue::from(5)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(50.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#91cc75")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Star::new(opts.into()).unwrap())).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn heart_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(70.0)).unwrap();
    Reflect::set(&shape, &"width".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&shape, &"height".into(), &JsValue::from(50.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#ee6666")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Heart::new(opts.into()).unwrap())).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn droplet_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"width".into(), &JsValue::from(30.0)).unwrap();
    Reflect::set(&shape, &"height".into(), &JsValue::from(60.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#73c0de")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Droplet::new(opts.into()).unwrap()))
        .unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn rose_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let r_arr = Array::new();
    r_arr.push(&JsValue::from(40.0));

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &r_arr).unwrap();
    Reflect::set(&shape, &"k".into(), &JsValue::from(3.0)).unwrap();
    Reflect::set(&shape, &"n".into(), &JsValue::from(1)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"stroke".into(), &JsValue::from_str("#5470c6")).unwrap();
    Reflect::set(&style, &"lineWidth".into(), &JsValue::from(2.0)).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Rose::new(opts.into()).unwrap())).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn trochoid_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(160.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&shape, &"r0".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&shape, &"d".into(), &JsValue::from(30.0)).unwrap();
    Reflect::set(&shape, &"location".into(), &JsValue::from_str("out")).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"stroke".into(), &JsValue::from_str("#fac858")).unwrap();
    Reflect::set(&style, &"lineWidth".into(), &JsValue::from(2.0)).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Trochoid::new(opts.into()).unwrap()))
        .unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

fn solid_rgba_bytes(width: u32, height: u32, r: u8, g: u8, b: u8) -> Uint8Array {
    let mut data = vec![0u8; (width * height * 4) as usize];
    for px in data.chunks_mut(4) {
        px[0] = r;
        px[1] = g;
        px[2] = b;
        px[3] = 255;
    }
    Uint8Array::from(data.as_slice())
}

#[wasm_bindgen_test]
fn generic_path_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let shape = Object::new();
    Reflect::set(
        &shape,
        &"pathData".into(),
        &JsValue::from_str("M 40 40 L 140 40 L 140 100 Z"),
    )
    .unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#5470c6")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Path::new(opts.into()).unwrap()))
        .unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn compound_path_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let paths = Array::new();
    let sub0 = Object::new();
    Reflect::set(
        &sub0,
        &"pathData".into(),
        &JsValue::from_str("M 40 40 L 120 40 L 80 100 Z"),
    )
    .unwrap();
    paths.push(&sub0);
    let sub1 = Object::new();
    Reflect::set(
        &sub1,
        &"pathData".into(),
        &JsValue::from_str("M 160 60 L 220 60 L 190 120 Z"),
    )
    .unwrap();
    paths.push(&sub1);

    let shape = Object::new();
    Reflect::set(&shape, &"paths".into(), &paths).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#91cc75")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(CompoundPath::new(opts.into()).unwrap()))
        .unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn image_refresh_outputs_rgba() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let style = Object::new();
    Reflect::set(&style, &"x".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&style, &"y".into(), &JsValue::from(30.0)).unwrap();
    Reflect::set(&style, &"width".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&style, &"height".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&style, &"image".into(), &solid_rgba_bytes(8, 8, 238, 102, 102)).unwrap();
    Reflect::set(&style, &"imageWidth".into(), &JsValue::from(8)).unwrap();
    Reflect::set(&style, &"imageHeight".into(), &JsValue::from(8)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    g.add(JsValue::from(Image::new(opts.into()).unwrap()))
        .unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[0] > 200 && px[3] > 0));
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
