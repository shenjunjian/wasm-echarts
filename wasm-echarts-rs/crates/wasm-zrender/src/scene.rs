//! 内置 Demo 场景（供浏览器 Demo 与调试）

use rust_zrender::{
    ChildRef, CircleShape, DisplayableProps, EcData, FillStrokeStyle, LineShape, Path,
    PathStyle, PolygonShape, RectShape, SectorShape, Shape, Text, TextAlign,
    TextBaseline, TextStyle, ZRenderer,
};
use std::f64::consts::PI;

pub fn load_scene(zr: &mut ZRenderer, scene: &str) {
    zr.storage = rust_zrender::Storage::new();
    let group = zr.storage.create_group();

    match scene {
        "text" => build_text_scene(zr, group),
        "sector" => build_sector_scene(zr, group),
        "hit" => build_hit_scene(zr, group),
        "state" => build_state_scene(zr, group),
        _ => build_shapes_scene(zr, group),
    }

    zr.storage.add_root(ChildRef::Group(group));
}

fn build_shapes_scene(zr: &mut ZRenderer, group: usize) {
    let rect = zr.storage.create_path(
        Path::new(
            Shape::Rect(RectShape {
                x: 20.0,
                y: 20.0,
                width: 100.0,
                height: 60.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#5470c6"),
                ..Default::default()
            },
        )
        .with_ec_data(EcData::new(0, 0)),
    );

    let circle = zr.storage.create_path(
        Path::new(
            Shape::Circle(CircleShape {
                cx: 180.0,
                cy: 80.0,
                r: 40.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::color("rgba(145, 204, 117, 0.8)"),
                stroke: FillStrokeStyle::color("#ee6666"),
                line_width: 3.0,
                ..Default::default()
            },
        )
        .with_displayable(DisplayableProps {
            z: 1.0,
            ..Default::default()
        })
        .with_ec_data(EcData::new(0, 1)),
    );

    let line = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1: 20.0,
            y1: 120.0,
            x2: 280.0,
            y2: 120.0,
            percent: 1.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::none(),
            stroke: FillStrokeStyle::color("#333"),
            line_width: 2.0,
            line_dash: Some(vec![6.0, 4.0]),
            ..Default::default()
        },
    ));

    let polygon = zr.storage.create_path(Path::new(
        Shape::Polygon(PolygonShape {
            points: vec![(240.0, 30.0), (300.0, 60.0), (270.0, 100.0)],
        }),
        PathStyle {
            fill: FillStrokeStyle::color("#fac858"),
            ..Default::default()
        },
    ));

    zr.storage.group_add_child(group, ChildRef::Path(rect));
    zr.storage.group_add_child(group, ChildRef::Path(circle));
    zr.storage.group_add_child(group, ChildRef::Path(line));
    zr.storage.group_add_child(group, ChildRef::Path(polygon));
}

fn build_text_scene(zr: &mut ZRenderer, group: usize) {
    let _ = group;
    zr.storage.create_text(
        Text::new("rust-zrender", 40.0, 50.0).with_style(TextStyle {
            fill: "#333".into(),
            font_size: 20.0,
            align: TextAlign::Left,
            baseline: TextBaseline::Alphabetic,
        }),
    );
    zr.storage.create_text(
        Text::new("fillText / 中文", 40.0, 90.0).with_style(TextStyle {
            fill: "#5470c6".into(),
            font_size: 16.0,
            align: TextAlign::Left,
            baseline: TextBaseline::Alphabetic,
        }),
    );
}

fn build_sector_scene(zr: &mut ZRenderer, group: usize) {
    let cx = 160.0;
    let cy = 100.0;
    let r = 70.0;
    let values = [30.0, 50.0, 80.0];
    let total: f64 = values.iter().sum();
    let colors = ["#5470c6", "#91cc75", "#fac858"];
    let mut angle = -PI / 2.0;

    for (i, &v) in values.iter().enumerate() {
        let sweep = v / total * PI * 2.0;
        let start = angle;
        angle += sweep;
        let sector = zr.storage.create_path(
            Path::new(
                Shape::Sector(SectorShape {
                    cx,
                    cy,
                    r,
                    start_angle: start,
                    end_angle: angle,
                    percent: 1.0,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color(colors[i]),
                    stroke: FillStrokeStyle::color("#fff"),
                    line_width: 1.0,
                    ..Default::default()
                },
            )
            .with_ec_data(EcData::new(0, i as i32)),
        );
        zr.storage.group_add_child(group, ChildRef::Path(sector));
    }
}

fn build_hit_scene(zr: &mut ZRenderer, group: usize) {
    build_shapes_scene(zr, group);
}

fn build_state_scene(zr: &mut ZRenderer, group: usize) {
    use rust_zrender::{PathStylePatch, STATE_EMPHASIS};

    let idx = zr.storage.create_path(Path::new(
        Shape::Rect(RectShape {
            x: 60.0,
            y: 40.0,
            width: 120.0,
            height: 80.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::color("#5470c6"),
            ..Default::default()
        },
    ));
    zr.set_path_state_style(
        idx,
        STATE_EMPHASIS,
        PathStylePatch {
            fill: Some(FillStrokeStyle::color("#ee6666")),
            line_width: Some(3.0),
            ..Default::default()
        },
    );
    zr.storage.group_add_child(group, ChildRef::Path(idx));
}
