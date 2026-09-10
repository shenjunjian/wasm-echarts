//! 漏斗：按 value 宽、从上到下画梯形

use rust_zrender::{PolygonShape, Shape, TextAlign, TextBaseline, ZRenderer};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke, opt_f64, opt_str};
use crate::model::{GlobalModel, SeriesModel};
use crate::utils::parse_percent;
use crate::visual::VisualContext;

pub fn render_funnel_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    if series.data.is_empty() {
        return;
    }
    let series_opt = visual.series_option(series.index);
    let w = model.width as f64;
    let h = model.height as f64;
    let left = parse_percent(series_opt.and_then(|s| s.get("left")), w, w * 0.15);
    let top = parse_percent(series_opt.and_then(|s| s.get("top")), h, h * 0.12);
    let width = parse_percent(series_opt.and_then(|s| s.get("width")), w, w * 0.7);
    let height = parse_percent(series_opt.and_then(|s| s.get("height")), h, h * 0.76);
    let gap = opt_f64(series_opt, "gap", 2.0);
    let min_size = parse_percent(series_opt.and_then(|s| s.get("minSize")), width, 0.0);
    let max_size = parse_percent(series_opt.and_then(|s| s.get("maxSize")), width, width);
    let sort_desc = opt_str(series_opt, "sort") != Some("ascending");
    let mut items: Vec<(usize, f64)> = series
        .data
        .iter()
        .enumerate()
        .filter(|(_, p)| p.value.is_finite() && p.value >= 0.0)
        .map(|(i, p)| (i, p.value))
        .collect();
    items.sort_by(|a, b| {
        if sort_desc {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
        }
    });
    let max_v = items.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max).max(1e-9);
    let n = items.len().max(1) as f64;
    let row_h = ((height - gap * (n - 1.0)) / n).max(2.0);
    let mut y = top;
    let mut prev_w = width_of(items.first().map(|(_, v)| *v).unwrap_or(0.0), max_v, min_size, max_size);
    for (k, (i, value)) in items.iter().enumerate() {
        let next_w = if k + 1 < items.len() {
            width_of(items[k + 1].1, max_v, min_size, max_size)
        } else {
            width_of(*value, max_v, min_size, max_size).min(prev_w)
        };
        let top_w = prev_w;
        let bot_w = next_w;
        let cx = left + width / 2.0;
        let color = visual.resolve_item_color(series.index, *i);
        add_ec_path(
            zr,
            group,
            Shape::Polygon(PolygonShape {
                points: vec![
                    (cx - top_w / 2.0, y),
                    (cx + top_w / 2.0, y),
                    (cx + bot_w / 2.0, y + row_h),
                    (cx - bot_w / 2.0, y + row_h),
                ],
                ..Default::default()
            }),
            fill_stroke(&color, "#fff", 1.0, 1.0),
            series.index,
            *i,
            series.index as f64,
            true,
        );
        add_label(
            zr,
            group,
            visual,
            series.index,
            *i,
            cx,
            y + row_h / 2.0,
            TextAlign::Center,
            TextBaseline::Middle,
            "#333",
            series.index as f64 + 0.2,
        );
        y += row_h + gap;
        prev_w = bot_w;
    }
}

fn width_of(value: f64, max_v: f64, min_size: f64, max_size: f64) -> f64 {
    min_size + (max_size - min_size) * (value / max_v).clamp(0.0, 1.0)
}
