//! 阶段 4 占位场景：option 解析完成前仍用 zrender demo 图形验证管线

use wasm_zrender::{
    ChildRef, DisplayableProps, FillStrokeStyle, Path, PathStyle, PolygonShape, RectShape,
    Shape, Storage, ZRenderer,
};
use wasm_zrender::{CircleShape, LineShape};

use crate::option::OptionModel;

/// 根据已合并 option 构建占位场景（阶段 5 替换为 ChartView）
pub fn build_placeholder_scene(zr: &mut ZRenderer, option: &OptionModel) {
    zr.storage = Storage::new();
    let group = zr.storage.create_group();

    let rect = zr.storage.create_path(Path::new(
        Shape::Rect(RectShape {
            x: 40.0,
            y: 30.0,
            width: 120.0,
            height: 80.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::color("#5470c6"),
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
                fill: FillStrokeStyle::color("rgba(145, 204, 117, 0.6)"),
                stroke: FillStrokeStyle::color("#ee6666"),
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
            fill: FillStrokeStyle::none(),
            stroke: FillStrokeStyle::color("#333"),
            line_width: 2.0,
            ..Default::default()
        },
    ));

    let polygon = zr.storage.create_path(Path::new(
        Shape::Polygon(PolygonShape {
            points: vec![(300.0, 40.0), (360.0, 80.0), (330.0, 140.0)],
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
    zr.storage.add_root(ChildRef::Group(group));

    // 若 option 含 series，写入 ECData 供 hover 演示
    if let Some(series) = option.root().get("series").and_then(|v| v.as_array()) {
        if let Some(first) = series.first() {
            if let Some(data) = first.get("data").and_then(|v| v.as_array()) {
                let path_indices: Vec<usize> = (0..zr.storage.paths().len())
                    .filter(|&i| {
                        let p = zr.storage.path(i);
                        !p.base.ignore && !p.displayable.invisible
                    })
                    .collect();
                for (i, path_idx) in path_indices.iter().enumerate().take(data.len()) {
                    zr.set_path_ec_data(
                        *path_idx,
                        wasm_zrender::EcData::new(0, i as i32),
                    );
                }
            }
        }
    }
}
