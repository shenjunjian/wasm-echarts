//! K 线 / 箱线图：cartesian 上的 OHLC 与五数概括

use rust_zrender::{LineShape, RectShape, Shape, TextAlign, TextBaseline, ZRenderer};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke, raw_numbers};
use crate::coord::SeriesCoord;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn render_candlestick_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    render_ohlc(zr, group, coord, visual, series, false);
}

pub fn render_boxplot_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    render_ohlc(zr, group, coord, visual, series, true);
}

fn render_ohlc(
    zr: &mut ZRenderer,
    group: usize,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
    boxplot: bool,
) {
    let Some(cart) = coord.as_cartesian() else {
        return;
    };
    let band = cart.category_band_width();
    let body_w = (band * 0.55).max(2.0);
    let series_opt = visual.series_option(series.index);
    let up = series_opt
        .and_then(|s| s.get("itemStyle"))
        .and_then(|s| s.get("color"))
        .and_then(|v| v.as_str())
        .unwrap_or("#ec0000")
        .to_string();
    let down = series_opt
        .and_then(|s| s.get("itemStyle"))
        .and_then(|s| s.get("color0"))
        .and_then(|v| v.as_str())
        .unwrap_or("#00da3c")
        .to_string();
    for (i, point) in series.data.iter().enumerate() {
        let nums = raw_numbers(&point.raw);
        if nums.len() < 4 {
            continue;
        }
        let (open, close, low, high) = if boxplot {
            (nums[1], nums[3], nums[0], nums[4.min(nums.len() - 1)])
        } else if nums.len() >= 5 {
            (nums[1], nums[2], nums[3], nums[4])
        } else {
            (nums[0], nums[1], nums[2], nums[3])
        };
        let (cx, _) = coord.point_for(i, point.x_value, close);
        let y_open = cart.y_value_to_pixel(open);
        let y_close = cart.y_value_to_pixel(close);
        let y_low = cart.y_value_to_pixel(low);
        let y_high = cart.y_value_to_pixel(high);
        let color = if close >= open { &up } else { &down };
        add_ec_path(
            zr,
            group,
            Shape::Line(LineShape {
                x1: cx,
                y1: y_high,
                x2: cx,
                y2: y_low,
                percent: 1.0,
            }),
            fill_stroke("none", color, 1.0, 1.0),
            series.index,
            i,
            series.index as f64,
            false,
        );
        let top = y_open.min(y_close);
        let h = (y_open - y_close).abs().max(1.0);
        add_ec_path(
            zr,
            group,
            Shape::Rect(RectShape {
                x: cx - body_w / 2.0,
                y: top,
                width: body_w,
                height: h,
                ..Default::default()
            }),
            fill_stroke(color, color, 1.0, 1.0),
            series.index,
            i,
            series.index as f64 + 0.1,
            true,
        );
        if boxplot && nums.len() >= 5 {
            let y_med = cart.y_value_to_pixel(nums[2]);
            add_ec_path(
                zr,
                group,
                Shape::Line(LineShape {
                    x1: cx - body_w / 2.0,
                    y1: y_med,
                    x2: cx + body_w / 2.0,
                    y2: y_med,
                    percent: 1.0,
                }),
                fill_stroke("none", "#333", 1.5, 1.0),
                series.index,
                i,
                series.index as f64 + 0.2,
                false,
            );
        }
        add_label(
            zr,
            group,
            visual,
            series.index,
            i,
            cx,
            top - 4.0,
            TextAlign::Center,
            TextBaseline::Bottom,
            color,
            series.index as f64 + 0.3,
        );
    }
}
