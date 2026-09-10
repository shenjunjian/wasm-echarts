//! 平行坐标 series：每条数据一条折线

use rust_zrender::{PolylineShape, Shape, ZRenderer};

use crate::chart::path_util::{add_ec_path, fill_stroke, raw_numbers};
use crate::chart::style::style_width;
use crate::coord::ParallelCoord;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_parallel_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let Some(coord) = ParallelCoord::for_series(model, series) else {
        return;
    };
    let lw = style_width(
        visual
            .series_option(series.index)
            .and_then(|s| s.get("lineStyle")),
        1.2,
    );
    let dims = coord.spec.axes.len().max(1);
    for (i, point) in series.data.iter().enumerate() {
        let values = raw_numbers(&point.raw);
        if values.is_empty() {
            continue;
        }
        let mut pts = Vec::new();
        for dim in 0..dims.min(values.len()) {
            let (x, y) = coord.data_to_point(values[dim], dim);
            if x.is_finite() && y.is_finite() {
                pts.push((x, y));
            }
        }
        if pts.len() < 2 {
            continue;
        }
        let color = visual.resolve_item_color(series.index, i);
        add_ec_path(
            zr,
            group,
            Shape::Polyline(PolylineShape {
                points: pts,
                percent: 1.0,
                ..Default::default()
            }),
            fill_stroke("none", &color, lw, 0.8),
            series.index,
            i,
            series.index as f64,
            true,
        );
    }
}
