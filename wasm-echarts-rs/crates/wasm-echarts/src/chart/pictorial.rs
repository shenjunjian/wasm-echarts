//! 象形柱：按柱高堆叠或缩放 symbol（终态）

use rust_zrender::{TextAlign, TextBaseline, ZRenderer};

use crate::chart::bar::bar_column_layout;
use crate::chart::label::add_label;
use crate::chart::symbol::{add_symbol, SymbolSpec};
use crate::coord::SeriesCoord;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_pictorial_bar_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let Some(cart) = coord.as_cartesian() else {
        return;
    };
    let band = cart.category_band_width();
    let layout = bar_column_layout(model, series, visual, band);
    let kind = visual.resolve_symbol(series.index);
    let repeat = visual
        .series_option(series.index)
        .and_then(|s| s.get("symbolRepeat"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    for (i, point) in series.data.iter().enumerate() {
        if !point.stacked_value.is_finite() {
            continue;
        }
        let (px, py) = coord.point_for(i, point.x_value, point.stacked_value);
        let base = if coord.is_horizontal() {
            cart.base_x()
        } else {
            cart.base_y()
        };
        let color = visual.resolve_item_color(series.index, i);
        if coord.is_horizontal() {
            let len = (px - base).abs();
            let cy = py + layout.offset + layout.width / 2.0;
            draw_symbols(zr, group, series, i, &kind, &color, base.min(px), cy, len, layout.width, true, repeat);
        } else {
            let len = (base - py).abs();
            let cx = px + layout.offset + layout.width / 2.0;
            draw_symbols(zr, group, series, i, &kind, &color, cx, py.min(base), layout.width, len, false, repeat);
        }
        add_label(
            zr,
            group,
            visual,
            series.index,
            i,
            px,
            py - 4.0,
            TextAlign::Center,
            TextBaseline::Bottom,
            &color,
            series.index as f64 + 0.2,
        );
    }
}

fn draw_symbols(
    zr: &mut ZRenderer,
    group: usize,
    series: &SeriesModel,
    data_index: usize,
    kind: &str,
    color: &str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    horizontal: bool,
    repeat: bool,
) {
    if repeat {
        let step = if horizontal { h.max(6.0) } else { w.max(6.0) };
        let count = if horizontal {
            (w / step).floor().max(1.0) as usize
        } else {
            (h / step).floor().max(1.0) as usize
        };
        for k in 0..count {
            let (cx, cy) = if horizontal {
                (x + (k as f64 + 0.5) * step, y)
            } else {
                (x, y + h - (k as f64 + 0.5) * step)
            };
            add_symbol(
                zr,
                group,
                &SymbolSpec {
                    kind: kind.to_string(),
                    size: step * 0.9,
                    cx,
                    cy,
                    color: color.to_string(),
                    series_index: series.index,
                    data_index,
                    attach_states: k == count - 1,
                },
            );
        }
    } else {
        let size = w.min(h).max(4.0);
        add_symbol(
            zr,
            group,
            &SymbolSpec {
                kind: kind.to_string(),
                size,
                cx: x + w / 2.0,
                cy: y + h / 2.0,
                color: color.to_string(),
                series_index: series.index,
                data_index,
                attach_states: true,
            },
        );
    }
}
