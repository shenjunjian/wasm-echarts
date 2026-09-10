//! 雷达图：按 indicator 把 value[] 投到蛛网并画 Polygon

use rust_zrender::{FillStrokeStyle, PathStyle, PolygonShape, Shape, TextAlign, TextBaseline, ZRenderer};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke, raw_numbers};
use crate::chart::style::{option_to_fill, style_opacity, style_width};
use crate::coord::RadarCoord;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_radar_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let Some(coord) = RadarCoord::for_series(model, series) else {
        return;
    };
    let n = coord.indicator_count();
    if n == 0 {
        return;
    }
    let series_opt = visual.series_option(series.index);
    let line_style = series_opt.and_then(|s| s.get("lineStyle"));
    let area_style = series_opt.and_then(|s| s.get("areaStyle"));
    let lw = style_width(line_style, 2.0);
    for (i, point) in series.data.iter().enumerate() {
        let values = raw_numbers(&point.raw);
        if values.is_empty() {
            continue;
        }
        let mut pts = Vec::new();
        for (dim, value) in values.iter().take(n).enumerate() {
            let (x, y) = coord.data_to_point(*value, dim);
            if x.is_finite() && y.is_finite() {
                pts.push((x, y));
            }
        }
        if pts.len() < 3 {
            continue;
        }
        let color = visual.resolve_item_color(series.index, i);
        if area_style.is_some() {
            let fill = option_to_fill(area_style.and_then(|s| s.get("color")), &color);
            let op = style_opacity(area_style, 0.25);
            add_ec_path(
                zr,
                group,
                Shape::Polygon(PolygonShape {
                    points: pts.clone(),
                    ..Default::default()
                }),
                PathStyle {
                    fill,
                    stroke: FillStrokeStyle::none(),
                    opacity: op,
                    ..Default::default()
                },
                series.index,
                i,
                series.index as f64,
                false,
            );
        }
        add_ec_path(
            zr,
            group,
            Shape::Polygon(PolygonShape {
                points: pts.clone(),
                ..Default::default()
            }),
            fill_stroke("none", &color, lw, 1.0),
            series.index,
            i,
            series.index as f64 + 0.1,
            true,
        );
        if let Some((x, y)) = pts.first() {
            add_label(
                zr,
                group,
                visual,
                series.index,
                i,
                *x,
                *y - 8.0,
                TextAlign::Center,
                TextBaseline::Bottom,
                &color,
                series.index as f64 + 0.2,
            );
        }
    }
}
