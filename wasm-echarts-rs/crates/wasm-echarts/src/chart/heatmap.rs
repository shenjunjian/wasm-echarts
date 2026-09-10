//! 热力图：cartesian / calendar / matrix / geo 上按值上色的格子

use rust_zrender::{RectShape, Shape, TextAlign, TextBaseline, ZRenderer};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke, raw_numbers};
use crate::chart::visual_map;
use crate::coord::{CalendarCoord, MatrixCoord, SeriesCoord};
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_heatmap_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let (cell_w, cell_h) = cell_size(model, series, coord);
    for (i, point) in series.data.iter().enumerate() {
        let nums = raw_numbers(&point.raw);
        let heat = if nums.len() >= 3 {
            nums[2]
        } else {
            point.value
        };
        if !heat.is_finite() {
            continue;
        }
        let (cx, cy) = coord.map_point(point, i);
        if !cx.is_finite() || !cy.is_finite() {
            continue;
        }
        let color = visual_map::map_color(visual.option(), series.index, i, series)
            .or_else(|| visual_map::color_for_value(visual.option(), series.index, heat))
            .unwrap_or_else(|| visual.resolve_item_color(series.index, i));
        add_ec_path(
            zr,
            group,
            Shape::Rect(RectShape {
                x: cx - cell_w / 2.0,
                y: cy - cell_h / 2.0,
                width: cell_w.max(1.0),
                height: cell_h.max(1.0),
                ..Default::default()
            }),
            fill_stroke(&color, "none", 0.0, 0.9),
            series.index,
            i,
            series.index as f64,
            true,
        );
        add_label(
            zr,
            group,
            visual,
            series.index,
            i,
            cx,
            cy,
            TextAlign::Center,
            TextBaseline::Middle,
            "#333",
            series.index as f64 + 0.2,
        );
    }
}

fn cell_size(model: &GlobalModel, series: &SeriesModel, coord: &SeriesCoord) -> (f64, f64) {
    if let Some(cart) = coord.as_cartesian() {
        let band = cart.category_band_width();
        return (band.max(2.0), band.max(2.0));
    }
    if let Some(cal) = CalendarCoord::for_series(model, series) {
        return (cal.spec.cell_w.max(2.0), cal.spec.cell_h.max(2.0));
    }
    if let Some(mx) = MatrixCoord::for_series(model, series) {
        return mx.cell_size();
    }
    (12.0, 12.0)
}
