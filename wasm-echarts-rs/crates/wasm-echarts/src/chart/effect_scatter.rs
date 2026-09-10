//! 带涟漪散点：ripple 只画终态同心圆

use rust_zrender::{CircleShape, Shape, ZRenderer};

use crate::chart::path_util::{add_ec_path, fill_stroke, opt_f64};
use crate::chart::scatter::render_scatter_series;
use crate::coord::SeriesCoord;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_effect_scatter_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    render_scatter_series(zr, group, model, coord, visual, series);
    let series_opt = visual.series_option(series.index);
    let scale = series_opt
        .and_then(|s| s.get("rippleEffect"))
        .and_then(|r| r.get("scale"))
        .and_then(|v| v.as_f64())
        .unwrap_or(2.5)
        .max(1.0);
    let period_rings = series_opt
        .and_then(|s| s.get("rippleEffect"))
        .and_then(|r| r.get("number"))
        .and_then(|v| v.as_f64())
        .unwrap_or(2.0)
        .clamp(1.0, 4.0) as usize;
    let mut jitter = crate::chart::jitter::JitterState::default();
    for (i, point) in series.data.iter().enumerate() {
        if !point.value.is_finite() {
            continue;
        }
        let (cx, cy) = coord.map_point(point, i);
        let size = visual.resolve_symbol_size_of(series.index, i);
        let (cx, cy) = crate::chart::jitter::apply_scatter_jitter(
            &mut jitter, coord, series, i, size, cx, cy,
        );
        if !cx.is_finite() || !cy.is_finite() {
            continue;
        }
        let color = visual.resolve_item_color(series.index, i);
        for k in 1..=period_rings {
            let t = k as f64 / period_rings as f64;
            let r = size * 0.5 * (1.0 + (scale - 1.0) * t);
            add_ec_path(
                zr,
                group,
                Shape::Circle(CircleShape { cx, cy, r }),
                fill_stroke("none", &color, 1.0, (1.0 - t) as f32 * 0.45),
                series.index,
                i,
                series.index as f64 - 0.1,
                false,
            );
        }
    }
    let _ = opt_f64(series_opt, "z", 0.0);
}
