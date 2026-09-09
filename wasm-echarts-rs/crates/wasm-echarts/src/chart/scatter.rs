//! 散点图 ChartView

use rust_zrender::{TextAlign, TextBaseline, ZRenderer};

use crate::chart::label::add_label;
use crate::chart::symbol::{add_symbol, SymbolSpec};
use crate::coord::Cartesian2D;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_scatter_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &Cartesian2D,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let kind = visual.resolve_symbol(series.index);
    for (i, point) in series.data.iter().enumerate() {
        let x_val = point.x_value.unwrap_or(i as f64);
        let (cx, cy) = coord.value_to_point(x_val, point.value);
        let color = visual.resolve_item_color(series.index, i);
        let size = visual.resolve_symbol_size_of(series.index, i);
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
    }
}
