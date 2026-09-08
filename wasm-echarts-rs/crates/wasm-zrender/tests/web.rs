//! wasm-bindgen 浏览器端集成测试

use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use wasm_zrender::{
    clear_fonts, dispose_all, init, register_font, Arc, BezierCurve, BoundingRect, Circle, CompoundPath,
    Displayable, Droplet, Ellipse, Group, Heart, Image, Isogon, Line, LinearGradient, OrientedBoundingRect, Path,
    Point, Polygon, RadialGradient, Rect, Ring, Rose, Sector, Star, Text, Trochoid, TSpan,
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
fn text_in_group_refresh_outputs_rgba() {
    reset_registry();
    register_test_font();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let g = Group::new();

    let style = Object::new();
    Reflect::set(&style, &"text".into(), &JsValue::from_str("Group Text")).unwrap();
    Reflect::set(&style, &"x".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&style, &"y".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#5470c6")).unwrap();
    Reflect::set(&style, &"fontSize".into(), &JsValue::from(16.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    let text = Text::new(opts.into());
    g.add(JsValue::from(text)).unwrap();
    zr.add(JsValue::from(g)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn tspan_refresh_outputs_rgba() {
    reset_registry();
    register_test_font();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();

    let style = Object::new();
    Reflect::set(&style, &"text".into(), &JsValue::from_str("TSpan MVP")).unwrap();
    Reflect::set(&style, &"x".into(), &JsValue::from(30.0)).unwrap();
    Reflect::set(&style, &"y".into(), &JsValue::from(70.0)).unwrap();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#ee6666")).unwrap();
    Reflect::set(&style, &"fontSize".into(), &JsValue::from(18.0)).unwrap();

    let opts = Object::new();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    let tspan = TSpan::new(opts.into());
    zr.add(JsValue::from(tspan)).unwrap();

    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 320 * 160 * 4);
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn displayable_is_abstract() {
    reset_registry();
    let result = Displayable::new(JsValue::NULL);
    assert!(result.is_err());
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

#[wasm_bindgen_test]
fn rect_use_states() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(200, 100)).unwrap();
    let rect = Rect::new(rect_opts()).unwrap();
    rect.set_state_style("selected", emphasis_style()).unwrap();
    let states = Array::new();
    states.push(&JsValue::from_str("selected"));
    rect.use_states(states.into()).unwrap();
    zr.add(JsValue::from(rect)).unwrap();
    let rgba = zr.refresh().unwrap();
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

#[wasm_bindgen_test]
fn group_clip_path_and_remove() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(200, 100)).unwrap();
    let group = Group::new();
    let clip = Rect::new(rect_opts()).unwrap();
    group.set_clip_path(JsValue::from(clip));
    group.remove_clip_path();
    let child = Rect::new(rect_opts()).unwrap();
    group.add(JsValue::from(child)).unwrap();
    zr.add(JsValue::from(group)).unwrap();
    let rgba = zr.refresh().unwrap();
    assert_eq!(rgba.len(), 200 * 100 * 4);
}

#[wasm_bindgen_test]
fn zr_set_cursor_style_and_config_layer() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(80, 40)).unwrap();
    zr.set_cursor_style("crosshair");
    zr.config_layer(JsValue::from(0), JsValue::UNDEFINED).unwrap();
    let _ = zr.refresh().unwrap();
}

fn emphasis_style() -> JsValue {
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#ee6666")).unwrap();
    Reflect::set(&style, &"lineWidth".into(), &JsValue::from(4.0)).unwrap();
    style.into()
}

#[wasm_bindgen_test]
fn point_basic_ops() {
    let mut p = Point::new(Some(3.0), Some(4.0));
    assert_eq!(p.len(), 5.0);
    p.add(&Point::new(Some(1.0), Some(2.0)));
    assert_eq!(p.x(), 4.0);
    assert_eq!(p.y(), 6.0);
    assert_eq!(p.distance(&Point::new(Some(4.0), Some(6.0))), 0.0);
}

#[wasm_bindgen_test]
fn bounding_rect_calculate_transform() {
    let a = BoundingRect::new(10.0, 20.0, 20.0, 40.0);
    let b = BoundingRect::new(0.0, 0.0, 40.0, 80.0);
    let m = a.calculate_transform(&b);
    assert_eq!(m[0], 2.0);
    assert_eq!(m[3], 2.0);
    assert_eq!(m[4], -20.0);
    assert_eq!(m[5], -40.0);
}

#[wasm_bindgen_test]
fn bounding_rect_contain_union_intersect() {
    let rect = BoundingRect::new(10.0, 20.0, 30.0, 40.0);
    assert!(rect.contain(10.0, 20.0));
    assert!(!rect.contain(0.0, 0.0));

    let mut merged = BoundingRect::new(0.0, 0.0, 10.0, 10.0);
    merged.union(&BoundingRect::new(5.0, 5.0, 10.0, 10.0));
    assert_eq!(merged.width(), 15.0);
    assert_eq!(merged.height(), 15.0);

    assert!(rect.intersect(
        &BoundingRect::new(35.0, 50.0, 10.0, 10.0),
        JsValue::UNDEFINED,
    ));
    assert!(!rect.intersect(
        &BoundingRect::new(100.0, 100.0, 10.0, 10.0),
        JsValue::UNDEFINED,
    ));
}

#[wasm_bindgen_test]
fn oriented_bounding_rect_intersect() {
    let a = OrientedBoundingRect::new(
        JsValue::from(BoundingRect::new(0.0, 0.0, 20.0, 20.0)),
        JsValue::UNDEFINED,
    );
    let b = OrientedBoundingRect::new(
        JsValue::from(BoundingRect::new(5.0, 5.0, 10.0, 10.0)),
        JsValue::UNDEFINED,
    );
    assert!(a.intersect(&b, JsValue::UNDEFINED, JsValue::UNDEFINED));
}

#[wasm_bindgen_test]
fn official_example_apis_do_not_throw() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    assert_eq!(zr.get_width(), 320);
    assert_eq!(zr.get_height(), 160);
    let _ = zr.on("mousedown", JsValue::UNDEFINED);
    zr.animation().on("frame", JsValue::UNDEFINED);

    let pos = Array::new();
    pos.push(&JsValue::from(80.0));
    pos.push(&JsValue::from(70.0));
    let opts = circle_opts();
    Reflect::set(&opts, &"position".into(), &pos).unwrap();

    let circle = Circle::new(opts).unwrap();
    circle
        .animate(JsValue::from_str("shape"), JsValue::TRUE)
        .when(1000.0, JsValue::UNDEFINED)
        .during(JsValue::UNDEFINED)
        .done(JsValue::UNDEFINED)
        .start(JsValue::UNDEFINED);
    let _ = circle.on("mousemove", JsValue::UNDEFINED);

    let heart_opts = Object::new();
    let heart_shape = Object::new();
    Reflect::set(&heart_shape, &"cx".into(), &JsValue::from(80.0)).unwrap();
    Reflect::set(&heart_shape, &"cy".into(), &JsValue::from(70.0)).unwrap();
    Reflect::set(&heart_shape, &"width".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&heart_shape, &"height".into(), &JsValue::from(50.0)).unwrap();
    Reflect::set(&heart_opts, &"shape".into(), &heart_shape).unwrap();
    let heart = Heart::new(heart_opts.into()).unwrap();
    let _ = circle.set_clip_path(JsValue::from(heart));
    zr.add(JsValue::from(circle)).unwrap();

    let group = Group::new();
    let child = Circle::new(circle_opts()).unwrap();
    let child_pos = Array::new();
    child_pos.push(&JsValue::from(40.0));
    child_pos.push(&JsValue::from(50.0));
    child.set_position(child_pos.into());
    group.add(JsValue::from(child)).unwrap();
    let bbox = group.get_bounding_rect();
    assert!(bbox.width() > 0.0);
    zr.add(JsValue::from(group)).unwrap();

    let _ = zr.refresh().unwrap();
}

fn pointer_at(x: f64, y: f64) -> JsValue {
    let obj = Object::new();
    Reflect::set(&obj, &"zrX".into(), &JsValue::from(x)).unwrap();
    Reflect::set(&obj, &"zrY".into(), &JsValue::from(y)).unwrap();
    obj.into()
}

#[wasm_bindgen_test]
fn handler_dispatch_drags_draggable_circle() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(400, 300)).unwrap();

    let opts = Object::new();
    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("white")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();
    Reflect::set(&opts, &"draggable".into(), &JsValue::TRUE).unwrap();
    let pos = Array::new();
    pos.push(&JsValue::from(100.0));
    pos.push(&JsValue::from(80.0));
    Reflect::set(&opts, &"position".into(), &pos).unwrap();

    let circle = Circle::new(opts.into()).unwrap();
    zr.add(JsValue::from(circle)).unwrap();
    zr.refresh().unwrap();
    assert!(zr.find_hover(100.0, 80.0).is_some());

    zr.handler()
        .dispatch("mousedown", pointer_at(100.0, 80.0))
        .unwrap();
    zr.handler()
        .dispatch("mousemove", pointer_at(140.0, 110.0))
        .unwrap();
    zr.handler()
        .dispatch("mouseup", pointer_at(140.0, 110.0))
        .unwrap();

    assert!(zr.find_hover(100.0, 80.0).is_none());
    let hover = zr.find_hover(140.0, 110.0).expect("circle should follow drag");
    assert_eq!(hover.target().element_type(), "circle");
}

#[wasm_bindgen_test]
fn attr_transform_moves_and_scales_rect() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();

    let opts = Object::new();
    let shape = Object::new();
    Reflect::set(&shape, &"x".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"y".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"width".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&shape, &"height".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#000")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();
    Reflect::set(&opts, &"x".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&opts, &"y".into(), &JsValue::from(30.0)).unwrap();
    Reflect::set(&opts, &"scaleX".into(), &JsValue::from(2.0)).unwrap();

    let rect = Rect::new(opts.into()).unwrap();
    let pos = Array::from(&rect.position());
    assert_eq!(pos.get(0).as_f64().unwrap(), 40.0);
    assert_eq!(pos.get(1).as_f64().unwrap(), 30.0);

    rect.attr(JsValue::from_str("y"), JsValue::from(10.0));
    let pos = Array::from(&rect.position());
    assert_eq!(pos.get(1).as_f64().unwrap(), 10.0);

    let shape_patch = Object::new();
    Reflect::set(&shape_patch, &"height".into(), &JsValue::from(30.0)).unwrap();
    rect.set_shape(shape_patch.into());

    zr.add(JsValue::from(rect)).unwrap();
    zr.refresh().unwrap();
    // shape 20×30 at (0,0), x=40 scaleX=2 y=10 → 覆盖 [40,80] × [10,40]
    assert!(zr.find_hover(50.0, 20.0).is_some());
    assert!(zr.find_hover(10.0, 20.0).is_none());
}

#[wasm_bindgen_test]
fn group_add_before_keeps_child_order() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(320, 160)).unwrap();
    let group = Group::new();
    group.attr(JsValue::from_str("x"), JsValue::from(10.0));

    let first = Rect::new(rect_opts()).unwrap();
    let second = Circle::new(circle_opts()).unwrap();
    let first_js = JsValue::from(first);
    let second_js = JsValue::from(second);
    group.add(second_js.clone()).unwrap();
    group.add_before(first_js, second_js).unwrap();
    zr.add(JsValue::from(group)).unwrap();
    let rgba = zr.refresh().unwrap();
    assert!(!rgba.is_empty());
}

#[wasm_bindgen_test]
fn line_default_style_strokes_without_fill() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(200, 80)).unwrap();
    let opts = Object::new();
    let shape = Object::new();
    Reflect::set(&shape, &"x1".into(), &JsValue::from(10.0)).unwrap();
    Reflect::set(&shape, &"y1".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&shape, &"x2".into(), &JsValue::from(190.0)).unwrap();
    Reflect::set(&shape, &"y2".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"lineWidth".into(), &JsValue::from(6.0)).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    zr.add(JsValue::from(Line::new(opts.into()).unwrap())).unwrap();
    let rgba = zr.refresh().unwrap();
    assert!(rgba.chunks(4).any(|px| px[3] > 0), "Line default stroke should paint");
}

#[wasm_bindgen_test]
fn rounded_rect_corner_is_transparent() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(80, 80)).unwrap();
    let opts = Object::new();
    let shape = Object::new();
    Reflect::set(&shape, &"x".into(), &JsValue::from(10.0)).unwrap();
    Reflect::set(&shape, &"y".into(), &JsValue::from(10.0)).unwrap();
    Reflect::set(&shape, &"width".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&shape, &"height".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(30.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#000")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    zr.add(JsValue::from(Rect::new(opts.into()).unwrap())).unwrap();
    let rgba = zr.refresh().unwrap();
    let corner = (10 * 80 + 10) * 4;
    assert_eq!(rgba[corner + 3], 0);
    let center = (40 * 80 + 40) * 4;
    assert!(rgba[center + 3] > 0);
}

#[wasm_bindgen_test]
fn sector_ring_has_transparent_hole() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(100, 100)).unwrap();
    let opts = Object::new();
    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(50.0)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(50.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&shape, &"r0".into(), &JsValue::from(18.0)).unwrap();
    Reflect::set(&shape, &"startAngle".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"endAngle".into(), &JsValue::from(std::f64::consts::PI * 2.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#000")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    zr.add(JsValue::from(Sector::new(opts.into()).unwrap())).unwrap();
    let rgba = zr.refresh().unwrap();
    let hole = (50 * 100 + 50) * 4;
    assert_eq!(rgba[hole + 3], 0);
    let ring = (50 * 100 + 80) * 4;
    assert!(rgba[ring + 3] > 0);
}

#[wasm_bindgen_test]
fn linear_gradient_add_color_stop_paints() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(120, 60)).unwrap();
    let mut gradient = LinearGradient::new(0.0, 0.0, 1.0, 0.0, None, None);
    gradient.add_color_stop(0.0, "#ff0000".into());
    gradient.add_color_stop(1.0, "#00ff00".into());

    let opts = Object::new();
    let shape = Object::new();
    Reflect::set(&shape, &"x".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"y".into(), &JsValue::from(0.0)).unwrap();
    Reflect::set(&shape, &"width".into(), &JsValue::from(120.0)).unwrap();
    Reflect::set(&shape, &"height".into(), &JsValue::from(60.0)).unwrap();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from(gradient)).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();

    zr.add(JsValue::from(Rect::new(opts.into()).unwrap())).unwrap();
    let rgba = zr.refresh().unwrap();
    assert!(rgba.chunks(4).any(|px| px[0] > 200 && px[3] > 0));
    assert!(rgba.chunks(4).any(|px| px[1] > 200 && px[3] > 0));
}

#[wasm_bindgen_test]
fn radial_gradient_constructor_sets_r0() {
    let gradient = RadialGradient::new(0.5, 0.5, 0.5, None, None, Some(0.2));
    assert!((gradient.r0() - 0.2).abs() < 1e-9);
}

#[wasm_bindgen_test]
fn text_default_fill_is_black() {
    reset_registry();
    register_test_font();
    let mut zr = init(JsValue::NULL, init_opts(200, 80)).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"text".into(), &JsValue::from_str("A")).unwrap();
    Reflect::set(&style, &"x".into(), &JsValue::from(20.0)).unwrap();
    Reflect::set(&style, &"y".into(), &JsValue::from(40.0)).unwrap();
    Reflect::set(&style, &"fontSize".into(), &JsValue::from(32.0)).unwrap();
    let opts = Object::new();
    Reflect::set(&opts, &"style".into(), &style).unwrap();
    zr.add(JsValue::from(Text::new(opts.into()))).unwrap();
    let rgba = zr.refresh().unwrap();
    let dark = rgba.chunks(4).any(|px| px[3] > 0 && px[0] < 40 && px[1] < 40 && px[2] < 40);
    assert!(dark, "default Text fill should be near #000");
}

#[wasm_bindgen_test]
fn polygon_smooth_paints() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(120, 80)).unwrap();
    let points = Array::new();
    for (x, y) in [(10.0, 10.0), (110.0, 20.0), (80.0, 70.0), (20.0, 60.0)] {
        let pt = Array::new();
        pt.push(&JsValue::from(x));
        pt.push(&JsValue::from(y));
        points.push(&pt);
    }
    let shape = Object::new();
    Reflect::set(&shape, &"points".into(), &points).unwrap();
    Reflect::set(&shape, &"smooth".into(), &JsValue::from(0.4)).unwrap();
    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("#5470c6")).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();
    zr.add(JsValue::from(Polygon::new(opts.into()).unwrap())).unwrap();
    let rgba = zr.refresh().unwrap();
    assert!(rgba.chunks(4).any(|px| px[3] > 0));
}

/// 对齐 site/zrender/examples/animation.js：透明 fill + stroke，animate 终态后再 add/refresh。
#[wasm_bindgen_test]
fn animation_example_stroke_circle_paints_at_last_when() {
    reset_registry();
    let width = 800u32;
    let height = 600u32;
    let r = 30.0;
    let mut zr = init(JsValue::NULL, init_opts(width, height)).unwrap();

    let shape = Object::new();
    Reflect::set(&shape, &"cx".into(), &JsValue::from(r)).unwrap();
    Reflect::set(&shape, &"cy".into(), &JsValue::from(height as f64 / 2.0)).unwrap();
    Reflect::set(&shape, &"r".into(), &JsValue::from(r)).unwrap();
    let style = Object::new();
    Reflect::set(&style, &"fill".into(), &JsValue::from_str("transparent")).unwrap();
    Reflect::set(&style, &"stroke".into(), &JsValue::from_str("#FF6EBE")).unwrap();
    let opts = Object::new();
    Reflect::set(&opts, &"shape".into(), &shape).unwrap();
    Reflect::set(&opts, &"style".into(), &style).unwrap();
    Reflect::set(&opts, &"silent".into(), &JsValue::TRUE).unwrap();

    let circle = Circle::new(opts.into()).unwrap();
    let mid = Object::new();
    Reflect::set(&mid, &"cx".into(), &JsValue::from(r)).unwrap();
    let last = Object::new();
    Reflect::set(&last, &"cx".into(), &JsValue::from(width as f64 - r)).unwrap();
    circle
        .animate(JsValue::from_str("shape"), JsValue::TRUE)
        .when(5000.0, mid.into())
        .when(10000.0, last.into())
        .start(JsValue::UNDEFINED);

    let bbox = circle.get_bounding_rect();
    assert!(
        (bbox.x() - (width as f64 - r * 2.0)).abs() < 1.0,
        "last when should move circle to the right, bbox.x={}, expected ~{}",
        bbox.x(),
        width as f64 - r * 2.0
    );

    zr.add(JsValue::from(circle)).unwrap();
    let rgba = zr.refresh().unwrap();
    assert!(
        rgba.chunks(4).any(|px| px[3] > 0),
        "animation example circle should paint at least one pixel"
    );
}

#[wasm_bindgen_test]
fn animator_start_writes_last_when_shape() {
    reset_registry();
    let circle = Circle::new(circle_opts()).unwrap();
    let mid = Object::new();
    Reflect::set(&mid, &"cx".into(), &JsValue::from(300.0)).unwrap();
    let last = Object::new();
    Reflect::set(&last, &"cx".into(), &JsValue::from(50.0)).unwrap();
    circle
        .animate(JsValue::from_str("shape"), JsValue::FALSE)
        .when(1000.0, mid.into())
        .when(2000.0, last.into())
        .start(JsValue::UNDEFINED);
    let bbox = circle.get_bounding_rect();
    assert!(
        (bbox.x() - 10.0).abs() < 1.0,
        "last when cx=50, r=40 => bbox.x ~= 10, got {}",
        bbox.x()
    );
    assert!((bbox.width() - 80.0).abs() < 1.0);
}

#[wasm_bindgen_test]
fn hide_show_and_clear_and_background() {
    reset_registry();
    let mut zr = init(JsValue::NULL, init_opts(400, 200)).unwrap();
    zr.set_background_color(JsValue::from_str("#ff0000")).unwrap();
    assert_eq!(
        zr.get_background_color().as_string().as_deref(),
        Some("#ff0000")
    );

    let circle = Circle::new(circle_opts()).unwrap();
    zr.add(JsValue::from(circle.clone())).unwrap();
    assert!(zr.find_hover(180.0, 80.0).is_some());
    circle.hide();
    assert!(zr.find_hover(180.0, 80.0).is_none());
    circle.show();
    assert!(zr.find_hover(180.0, 80.0).is_some());

    zr.clear().unwrap();
    assert!(zr.find_hover(180.0, 80.0).is_none());
    let rgba = zr.refresh().unwrap();
    assert!(rgba[0] > 200 && rgba[1] < 40 && rgba[2] < 40);
}

#[wasm_bindgen_test]
fn zr_off_removes_handler_and_trigger_fires() {
    reset_registry();
    let zr = init(JsValue::NULL, init_opts(80, 80)).unwrap();
    let hits = Object::new();
    Reflect::set(&hits, &"n".into(), &JsValue::from(0.0)).unwrap();
    js_sys::Reflect::set(&js_sys::global(), &"__zr_off_hits".into(), &hits).unwrap();
    let handler = js_sys::Function::new_no_args(
        "var h = globalThis.__zr_off_hits; h.n = (h.n || 0) + 1;",
    );
    let _ = zr.on("custom", handler.clone().into());
    let _ = zr.trigger("custom", JsValue::UNDEFINED);
    assert_eq!(
        Reflect::get(&hits, &"n".into())
            .unwrap()
            .as_f64()
            .unwrap(),
        1.0
    );
    let _ = zr.off(JsValue::from_str("custom"), handler.into());
    let _ = zr.trigger("custom", JsValue::UNDEFINED);
    assert_eq!(
        Reflect::get(&hits, &"n".into())
            .unwrap()
            .as_f64()
            .unwrap(),
        1.0
    );
}

#[wasm_bindgen_test]
fn instance_dispose_removes_from_registry() {
    reset_registry();
    let zr = init(JsValue::NULL, init_opts(40, 40)).unwrap();
    let id = zr.id();
    zr.dispose();
    assert!(wasm_zrender::get_instance(id).is_none());
}
