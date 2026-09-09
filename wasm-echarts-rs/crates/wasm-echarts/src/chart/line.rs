//! 折线图 ChartView

use rust_zrender::{
    ChildRef, DisplayableProps, FillStrokeStyle, Path, PathStyle, PolylineShape, Shape,
    TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_label;
use crate::chart::symbol::{add_symbol, SymbolSpec};
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
    zoom_start: usize,
    zoom_end: usize,
) {
    if series.data.is_empty() {
        return;
    }

    let points: Vec<(f64, f64)> = series
        .data
        .iter()
        .enumerate()
        .filter(|(i, _)| *i >= zoom_start && *i < zoom_end)
        .map(|(i, p)| coord.data_to_point(i, p.value))
        .collect();

    if points.is_empty() {
        return;
    }

    let line_color = visual.resolve_item_color(series.index, 0);

    let polyline = zr.storage.create_path(
        Path::new(
            Shape::Polyline(PolylineShape {
                points: points.clone(),
                percent: 1.0,
                ..Default::default()
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

    let show_symbol = visual.show_symbol(series.index);
    let kind = visual.resolve_symbol(series.index);

    for (i, p) in series.data.iter().enumerate() {
        if i < zoom_start || i >= zoom_end {
            continue;
        }
        let (cx, cy) = points[i - zoom_start];
        let color = visual.resolve_item_color(series.index, i);
        let size = visual.resolve_symbol_size_of(series.index, i);
        if show_symbol {
            add_symbol(
                zr,
                group,
                &SymbolSpec {
                    kind: kind.clone(),
                    size,
                    cx,
                    cy,
                    color: color.clone(),
                    series_index: series.index,
                    data_index: i,
                },
            );
        }
        add_label(
            zr,
            group,
            visual,
            series.index,
            i,
            cx,
            cy - size / 2.0 - 4.0,
            TextAlign::Center,
            TextBaseline::Bottom,
            &color,
            series.index as f64 + 0.2,
        );
        let _ = p;
    }
}
