//! 散点图 ChartView

use wasm_zrender::{
    ChildRef, CircleShape, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle, Shape,
    PathStylePatch, STATE_EMPHASIS, STATE_SELECT, ZRenderer,
};

use crate::coord::Cartesian2D;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

const SYMBOL_SIZE: f64 = 6.0;

pub fn render_scatter_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &Cartesian2D,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    for (i, point) in series.data.iter().enumerate() {
        let x_val = point.x_value.unwrap_or(i as f64);
        let (cx, cy) = coord.value_to_point(x_val, point.value);
        let color = visual.resolve_item_color(series.index, i);

        let symbol = zr.storage.create_path(
            Path::new(
                Shape::Circle(CircleShape {
                    cx,
                    cy,
                    r: SYMBOL_SIZE,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color(&color),
                    stroke: FillStrokeStyle::color("#fff"),
                    line_width: 1.0,
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                z: series.index as f64 + 0.1,
                ..Default::default()
            })
            .with_ec_data(EcData::new(series.index as i32, i as i32)),
        );
        zr.storage.group_add_child(group, ChildRef::Path(symbol));

        zr.set_path_state_style(
            symbol,
            STATE_EMPHASIS,
            PathStylePatch {
                fill: Some(FillStrokeStyle::color(&color)),
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
    }
}
