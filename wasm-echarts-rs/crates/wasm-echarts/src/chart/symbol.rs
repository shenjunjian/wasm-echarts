//! line / scatter 的 symbol 图元：circle / rect / emptyCircle

use rust_zrender::{
    ChildRef, CircleShape, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle,
    PathStylePatch, RectShape, Shape, STATE_EMPHASIS, STATE_SELECT, ZRenderer,
};

pub struct SymbolSpec {
    pub kind: String,
    pub size: f64,
    pub cx: f64,
    pub cy: f64,
    pub color: String,
    pub series_index: usize,
    pub data_index: usize,
}

pub fn add_symbol(zr: &mut ZRenderer, group: usize, spec: &SymbolSpec) -> Option<usize> {
    if spec.size <= 0.0 {
        return None;
    }
    let r = spec.size / 2.0;
    let (shape, fill, stroke, line_width) = match spec.kind.as_str() {
        "none" => return None,
        "rect" | "square" => (
            Shape::Rect(RectShape {
                x: spec.cx - r,
                y: spec.cy - r,
                width: spec.size,
                height: spec.size,
                ..Default::default()
            }),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "emptyCircle" => (
            Shape::Circle(CircleShape {
                cx: spec.cx,
                cy: spec.cy,
                r,
            }),
            FillStrokeStyle::none(),
            FillStrokeStyle::color(&spec.color),
            1.5,
        ),
        _ => (
            Shape::Circle(CircleShape {
                cx: spec.cx,
                cy: spec.cy,
                r,
            }),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
    };
    let symbol = zr.storage.create_path(
        Path::new(
            shape,
            PathStyle {
                fill,
                stroke,
                line_width,
                ..Default::default()
            },
        )
        .with_displayable(DisplayableProps {
            z: spec.series_index as f64 + 0.1,
            ..Default::default()
        })
        .with_ec_data(EcData::new(spec.series_index as i32, spec.data_index as i32)),
    );
    zr.storage.group_add_child(group, ChildRef::Path(symbol));
    zr.set_path_state_style(
        symbol,
        STATE_EMPHASIS,
        PathStylePatch {
            fill: Some(FillStrokeStyle::color(&spec.color)),
            line_width: Some(2.0),
            ..Default::default()
        },
    );
    zr.set_path_state_style(
        symbol,
        STATE_SELECT,
        PathStylePatch {
            stroke: Some(FillStrokeStyle::color("#333")),
            line_width: Some(3.0),
            ..Default::default()
        },
    );
    Some(symbol)
}
