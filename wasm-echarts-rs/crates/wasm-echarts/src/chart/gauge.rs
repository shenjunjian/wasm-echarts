//! 仪表盘：轴环 + 指针终态 + detail 文案

use std::f64::consts::PI;

use rust_zrender::{
    FillStrokeStyle, PolygonShape, SectorShape, Shape, TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke, opt_f64};
use crate::model::{GlobalModel, SeriesModel};
use crate::utils::parse_percent;
use crate::visual::VisualContext;

pub fn render_gauge_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let series_opt = visual.series_option(series.index);
    let w = model.width as f64;
    let h = model.height as f64;
    let (cx, cy) = match series_opt.and_then(|s| s.get("center")) {
        Some(crate::option::OptionValue::Array(arr)) => (
            parse_percent(arr.first(), w, w * 0.5),
            parse_percent(arr.get(1), h, h * 0.5),
        ),
        _ => (w * 0.5, h * 0.5),
    };
    let half = w.min(h) / 2.0;
    let (r0, r) = match series_opt.and_then(|s| s.get("radius")) {
        Some(crate::option::OptionValue::Array(arr)) => (
            parse_percent(arr.first(), half, half * 0.75),
            parse_percent(arr.get(1), half, half * 0.9),
        ),
        Some(v) => (half * 0.75, parse_percent(Some(v), half, half * 0.9)),
        None => (half * 0.75, half * 0.9),
    };
    let start_deg = opt_f64(series_opt, "startAngle", 225.0);
    let end_deg = opt_f64(series_opt, "endAngle", -45.0);
    let min = opt_f64(series_opt, "min", 0.0);
    let max = opt_f64(series_opt, "max", 100.0);
    let start = -start_deg * PI / 180.0;
    let end = -end_deg * PI / 180.0;
    let span = end - start;
    let color = visual.resolve_item_color(series.index, 0);
    add_ec_path(
        zr,
        group,
        Shape::Sector(SectorShape {
            cx,
            cy,
            r,
            r0,
            start_angle: start,
            end_angle: end,
            clockwise: span >= 0.0,
            ..Default::default()
        }),
        fill_stroke("#e6e6e6", "none", 0.0, 1.0),
        series.index,
        0,
        series.index as f64,
        false,
    );
    let value = series
        .data
        .first()
        .map(|p| p.value)
        .filter(|n| n.is_finite())
        .unwrap_or(0.0);
    let t = if (max - min).abs() < 1e-12 {
        0.0
    } else {
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    };
    let pointer_end = start + span * t;
    add_ec_path(
        zr,
        group,
        Shape::Sector(SectorShape {
            cx,
            cy,
            r,
            r0,
            start_angle: start,
            end_angle: pointer_end,
            clockwise: span >= 0.0,
            ..Default::default()
        }),
        fill_stroke(&color, "none", 0.0, 1.0),
        series.index,
        0,
        series.index as f64 + 0.1,
        true,
    );
    let pr = r0.max(8.0) * 0.85;
    let tip_x = cx + pr * pointer_end.cos();
    let tip_y = cy + pr * pointer_end.sin();
    let nx = -(pointer_end.sin());
    let ny = pointer_end.cos();
    let hw = (r - r0).abs().max(6.0) * 0.15;
    add_ec_path(
        zr,
        group,
        Shape::Polygon(PolygonShape {
            points: vec![
                (tip_x, tip_y),
                (cx + nx * hw, cy + ny * hw),
                (cx - nx * hw, cy - ny * hw),
            ],
            ..Default::default()
        }),
        rust_zrender::PathStyle {
            fill: FillStrokeStyle::color(&color),
            stroke: FillStrokeStyle::none(),
            ..Default::default()
        },
        series.index,
        0,
        series.index as f64 + 0.2,
        false,
    );
    add_label(
        zr,
        group,
        visual,
        series.index,
        0,
        cx,
        cy + r0 * 0.35,
        TextAlign::Center,
        TextBaseline::Top,
        &color,
        series.index as f64 + 0.3,
    );
}
