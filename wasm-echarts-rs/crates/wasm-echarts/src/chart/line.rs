//! 折线图 ChartView

use wasm_zrender::{
    ChildRef, CircleShape, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle,
    PolylineShape, Shape, PathStylePatch, STATE_EMPHASIS, ZRenderer,
};

use crate::coord::Cartesian2D;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_line_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &Cartesian2D,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    if series.data.is_empty() {
        return;
    }

    let points: Vec<(f64, f64)> = series
        .data
        .iter()
        .enumerate()
        .map(|(i, p)| coord.data_to_point(i, p.value))
        .collect();

    let line_color = visual.resolve_item_color(series.index, 0);

    let polyline = zr.storage.create_path(
        Path::new(
            Shape::Polyline(PolylineShape {
                points: points.clone(),
                percent: 1.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::none(),
                stroke: FillStrokeStyle::color(&line_color),
                line_width: 2.0,
                ..Default::default()
            },
        )
        .with_displayable(DisplayableProps {
            z: series.index as f64,
            ..Default::default()
        }),
    );
    zr.storage.group_add_child(group, ChildRef::Path(polyline));

    for (i, p) in series.data.iter().enumerate() {
        let (cx, cy) = points[i];
        let color = visual.resolve_item_color(series.index, i);
        let symbol = zr.storage.create_path(
            Path::new(
                Shape::Circle(CircleShape {
                    cx,
                    cy,
                    r: 4.0,
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

        // emphasis 样式补丁
        let emphasis_color = visual.resolve_item_color(series.index, i);
        zr.set_path_state_style(
            symbol,
            STATE_EMPHASIS,
            PathStylePatch {
                fill: Some(FillStrokeStyle::color(&emphasis_color)),
                line_width: Some(2.0),
                ..Default::default()
            },
        );

        let _ = p;
    }
}
