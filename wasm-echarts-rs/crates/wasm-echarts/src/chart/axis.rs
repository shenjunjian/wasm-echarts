//! 坐标轴刻度标签与轴名称（含多轴、time / log）

use rust_zrender::{
    ChildRef, FillStrokeStyle, LineShape, Path, PathStyle, Shape, TextAlign, TextBaseline,
    ZRenderer,
};
use wasm_bindgen::JsValue;

use crate::bridge::resolve_axis_formatter;
use crate::chart::text_opt::{
    add_silent_text, parse_chart_text_style, ChartTextStyle,
};
use crate::coord::{format_axis_tick, Cartesian2D};
use crate::model::{AxisModel, AxisType, GlobalModel};
use crate::option::{OptionModel, OptionValue};
use crate::utils::as_components;

pub fn render_axis_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
) {
    for (i, axis) in model.x_axes.iter().enumerate() {
        let y_idx = model
            .y_axes
            .iter()
            .position(|y| y.grid_index == axis.grid_index)
            .unwrap_or(0);
        let coord = Cartesian2D::for_axes(model, i, y_idx);
        let (zoom_start, zoom_end) = model.visible_category_range_of(i);
        let opt = axis_option_at(option, "xAxis", i);
        render_one_x(zr, group, axis, &coord, opt, zoom_start, zoom_end);
        render_one_name(
            zr,
            group,
            coord.grid(),
            opt,
            true,
        );
    }
    for (i, axis) in model.y_axes.iter().enumerate() {
        let x_idx = model
            .x_axes
            .iter()
            .position(|x| x.grid_index == axis.grid_index)
            .unwrap_or(0);
        let coord = Cartesian2D::for_axes(model, x_idx, i);
        let opt = axis_option_at(option, "yAxis", i);
        render_one_y(zr, group, axis, &coord, opt);
        render_one_name(zr, group, coord.grid(), opt, false);
    }
    render_axis_breaks(zr, group, model);
}

fn axis_option_at<'a>(option: &'a OptionModel, key: &str, index: usize) -> Option<&'a OptionValue> {
    as_components(option.root().get(key)).get(index).copied()
}

fn axis_label_style(axis_opt: Option<&OptionValue>) -> ChartTextStyle {
    parse_chart_text_style(
        axis_opt.and_then(|axis| axis.get("axisLabel")),
        "#666",
        11.0,
        "sans-serif",
    )
}

fn axis_formatter(axis_opt: Option<&OptionValue>) -> Option<&OptionValue> {
    axis_opt
        .and_then(|axis| axis.get("axisLabel"))
        .and_then(|label| label.get("formatter"))
}

fn add_axis_text(
    zr: &mut ZRenderer,
    group: usize,
    content: &str,
    x: f64,
    y: f64,
    align: TextAlign,
    baseline: TextBaseline,
    style: &ChartTextStyle,
) {
    add_silent_text(
        zr,
        group,
        content,
        x,
        y,
        style.to_text_style(align, baseline),
        2.0,
    );
}

fn format_tick(formatter: Option<&OptionValue>, fallback: &str, index: u32, numeric: Option<f64>) -> String {
    match formatter {
        Some(OptionValue::String(s)) => s.replace("{value}", fallback),
        Some(OptionValue::Function(_)) => {
            let axis_value = match numeric {
                Some(n) => JsValue::from(n),
                None => JsValue::from_str(fallback),
            };
            resolve_axis_formatter(formatter, &axis_value, index, fallback)
        }
        _ => fallback.to_string(),
    }
}

fn render_one_x(
    zr: &mut ZRenderer,
    group: usize,
    axis: &AxisModel,
    coord: &Cartesian2D,
    axis_opt: Option<&OptionValue>,
    zoom_start: usize,
    zoom_end: usize,
) {
    let g = coord.grid();
    let formatter = axis_formatter(axis_opt);
    let style = axis_label_style(axis_opt);
    if axis.axis_type.is_category() {
        let visible = (zoom_end - zoom_start).max(1);
        for i in zoom_start..zoom_end {
            let raw_label = axis
                .category_data
                .get(i)
                .map(|s| s.as_str())
                .unwrap_or("");
            if raw_label.is_empty() {
                continue;
            }
            let local = i - zoom_start;
            let x = g.x + (local as f64 + 0.5) / visible as f64 * g.width;
            let y = g.y + g.height + 14.0;
            let label = format_tick(formatter, raw_label, i as u32, None);
            add_axis_text(zr, group, &label, x, y, TextAlign::Center, TextBaseline::Top, &style);
        }
        return;
    }
    let ticks = axis_ticks(axis);
    for (i, value) in ticks.iter().enumerate() {
        if axis.is_inside_break(*value) {
            continue;
        }
        let fallback = format_axis_tick(axis, *value, None);
        let label = format_tick(formatter, &fallback, i as u32, Some(*value));
        let (x, _) = coord.value_to_point(*value, coord.y_axis().value_min());
        let y = g.y + g.height + 14.0;
        add_axis_text(zr, group, &label, x, y, TextAlign::Center, TextBaseline::Top, &style);
    }
}

fn render_one_y(
    zr: &mut ZRenderer,
    group: usize,
    axis: &AxisModel,
    coord: &Cartesian2D,
    axis_opt: Option<&OptionValue>,
) {
    let g = coord.grid();
    let formatter = axis_formatter(axis_opt);
    let style = axis_label_style(axis_opt);
    let ticks = axis_ticks(axis);
    for (i, value) in ticks.iter().enumerate() {
        if axis.is_inside_break(*value) {
            continue;
        }
        let fallback = format_axis_tick(axis, *value, None);
        let label = format_tick(formatter, &fallback, i as u32, Some(*value));
        let (_, y) = coord.value_to_point(coord.x_axis().value_min(), *value);
        let x = g.x - 8.0;
        add_axis_text(zr, group, &label, x, y, TextAlign::Right, TextBaseline::Middle, &style);
    }
}

fn axis_ticks(axis: &AxisModel) -> Vec<f64> {
    if axis.axis_type == AxisType::Log {
        return log_ticks(axis.value_min(), axis.value_max(), axis.log_base);
    }
    let min = axis.value_min();
    let max = axis.value_max();
    let span = max - min;
    (0..=5)
        .map(|i| min + span * i as f64 / 5.0)
        .collect()
}

fn log_ticks(min: f64, max: f64, base: f64) -> Vec<f64> {
    let base = base.max(1.000_000_1);
    let min = min.max(f64::MIN_POSITIVE);
    let max = max.max(min * base);
    let start = min.log(base).floor() as i32;
    let end = max.log(base).ceil() as i32;
    let mut ticks = Vec::new();
    for i in start..=end {
        let v = base.powi(i);
        if v >= min / base && v <= max * base {
            ticks.push(v);
        }
    }
    if ticks.len() < 2 {
        ticks = vec![min, max];
    }
    ticks
}

fn render_axis_breaks(zr: &mut ZRenderer, group: usize, model: &GlobalModel) {
    for (i, axis) in model.x_axes.iter().enumerate() {
        if !axis.has_breaks() {
            continue;
        }
        let y_idx = model
            .y_axes
            .iter()
            .position(|y| y.grid_index == axis.grid_index)
            .unwrap_or(0);
        let coord = Cartesian2D::for_axes(model, i, y_idx);
        let g = coord.grid();
        for brk in axis.active_breaks() {
            let x = coord.x_value_to_pixel(brk.start);
            add_break_mark(zr, group, x - 4.0, g.y + g.height - 6.0, x + 4.0, g.y + g.height + 6.0);
        }
    }
    for (i, axis) in model.y_axes.iter().enumerate() {
        if !axis.has_breaks() {
            continue;
        }
        let x_idx = model
            .x_axes
            .iter()
            .position(|x| x.grid_index == axis.grid_index)
            .unwrap_or(0);
        let coord = Cartesian2D::for_axes(model, x_idx, i);
        let g = coord.grid();
        for brk in axis.active_breaks() {
            let y = coord.y_value_to_pixel(brk.start);
            add_break_mark(zr, group, g.x - 6.0, y - 4.0, g.x + 6.0, y + 4.0);
        }
    }
}

fn add_break_mark(zr: &mut ZRenderer, group: usize, x1: f64, y1: f64, x2: f64, y2: f64) {
    let mid_x = (x1 + x2) * 0.5;
    let idx1 = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1,
            y1: y2,
            x2: mid_x,
            y2: y1,
            percent: 1.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::none(),
            stroke: FillStrokeStyle::color("#333"),
            line_width: 1.5,
            ..Default::default()
        },
    ));
    let idx2 = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1: mid_x,
            y1: y2,
            x2,
            y2: y1,
            percent: 1.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::none(),
            stroke: FillStrokeStyle::color("#333"),
            line_width: 1.5,
            ..Default::default()
        },
    ));
    zr.storage.group_add_child(group, ChildRef::Path(idx1));
    zr.storage.group_add_child(group, ChildRef::Path(idx2));
}

fn render_one_name(
    zr: &mut ZRenderer,
    group: usize,
    g: crate::model::GridRect,
    axis_opt: Option<&OptionValue>,
    is_x: bool,
) {
    let Some(name) = axis_opt
        .and_then(|axis| axis.get("name"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
    else {
        return;
    };
    let style = parse_chart_text_style(
        axis_opt.and_then(|axis| axis.get("nameTextStyle")),
        "#666",
        12.0,
        "sans-serif",
    );
    if is_x {
        add_silent_text(
            zr,
            group,
            &name,
            g.x + g.width + 6.0,
            g.y + g.height,
            style.to_text_style(TextAlign::Left, TextBaseline::Middle),
            3.0,
        );
    } else {
        add_silent_text(
            zr,
            group,
            &name,
            g.x,
            g.y - 8.0,
            style.to_text_style(TextAlign::Center, TextBaseline::Bottom),
            3.0,
        );
    }
}
