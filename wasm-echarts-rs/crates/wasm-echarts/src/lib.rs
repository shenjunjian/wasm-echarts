mod utils;

use wasm_bindgen::prelude::*;
use wasm_zrender::ZRenderer;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}

/// 阶段 1 渲染器：Storage → Painter → RGBA
#[wasm_bindgen]
pub struct DemoRenderer {
    zr: ZRenderer,
}

#[wasm_bindgen]
impl DemoRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<DemoRenderer, JsValue> {
        utils::set_panic_hook();
        let zr = ZRenderer::new(width, height).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(DemoRenderer { zr })
    }

    pub fn width(&self) -> u32 {
        self.zr.width()
    }

    pub fn height(&self) -> u32 {
        self.zr.height()
    }

    /// 构建 demo 场景并返回 RGBA 像素缓冲
    pub fn render(&mut self) -> Result<Vec<u8>, JsValue> {
        build_demo_scene(&mut self.zr);
        self.zr
            .refresh()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

fn build_demo_scene(zr: &mut ZRenderer) {
    use wasm_zrender::{
        ChildRef, DisplayableProps, Path, PathStyle, PolygonShape, RectShape, Shape,
    };
    use wasm_zrender::{CircleShape, LineShape};

    zr.storage = wasm_zrender::Storage::new();
    let group = zr.storage.create_group();

    let rect = zr.storage.create_path(Path::new(
        Shape::Rect(RectShape {
            x: 40.0,
            y: 30.0,
            width: 120.0,
            height: 80.0,
        }),
        PathStyle {
            fill: Some("#5470c6".to_string()),
            ..Default::default()
        },
    ));

    let circle = zr.storage.create_path(
        Path::new(
            Shape::Circle(CircleShape {
                cx: 220.0,
                cy: 100.0,
                r: 45.0,
            }),
            PathStyle {
                fill: Some("rgba(145, 204, 117, 0.6)".to_string()),
                stroke: Some("#ee6666".to_string()),
                line_width: 4.0,
                ..Default::default()
            },
        )
        .with_displayable(DisplayableProps {
            z: 1.0,
            ..Default::default()
        }),
    );

    let line = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1: 40.0,
            y1: 200.0,
            x2: 360.0,
            y2: 200.0,
            percent: 1.0,
        }),
        PathStyle {
            fill: None,
            stroke: Some("#333".to_string()),
            line_width: 2.0,
            ..Default::default()
        },
    ));

    let polygon = zr.storage.create_path(Path::new(
        Shape::Polygon(PolygonShape {
            points: vec![(300.0, 40.0), (360.0, 80.0), (330.0, 140.0)],
        }),
        PathStyle {
            fill: Some("#fac858".to_string()),
            ..Default::default()
        },
    ));

    zr.storage.group_add_child(group, ChildRef::Path(rect));
    zr.storage.group_add_child(group, ChildRef::Path(circle));
    zr.storage.group_add_child(group, ChildRef::Path(line));
    zr.storage.group_add_child(group, ChildRef::Path(polygon));
    zr.storage.add_root(ChildRef::Group(group));
}
