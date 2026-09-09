//! 坐标轴刻度标签

use rust_zrender::{
    ChildRef, Text, TextAlign, TextBaseline, TextStyle, ZRenderer,
};
use wasm_bindgen::JsValue;

use crate::bridge::resolve_axis_formatter;
use crate::coord::Cartesian2D;
use crate::model::{AxisType, GlobalModel};
use crate::option::{OptionModel, OptionValue};
use crate::utils::format_axis_number;

pub fn render_axis_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    coord: &Cartesian2D,
    zoom_start: usize,
    zoom_end: usize,
) {
    render_x_labels(zr, group, model, option, coord, zoom_start, zoom_end);
    render_y_labels(zr, group, model, option, coord);
}

fn axis_component<'a>(option: &'a OptionModel, key: &str) -> Option<&'a OptionValue> {
    match option.root().get(key) {
        Some(OptionValue::Array(arr)) => arr.first(),
        Some(v) => Some(v),
        None => None,
    }
}

fn axis_formatter<'a>(option: &'a OptionModel, key: &str) -> Option<&'a OptionValue> {
    axis_component(option, key)
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
) {
    let text_idx = zr.storage.create_text(Text::new(content, x, y).with_style(TextStyle {
        fill: "#666".into(),
        font_size: 11.0,
        align,
        baseline,
    }));
    zr.storage.text_mut(text_idx).silent = true;
    zr.storage.group_add_child(group, ChildRef::Text(text_idx));
}

fn format_label(formatter: Option<&OptionValue>, raw: &JsValue, index: u32, fallback: &str) -> String {
    resolve_axis_formatter(formatter, raw, index, fallback)
}

fn render_x_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    coord: &Cartesian2D,
    zoom_start: usize,
    zoom_end: usize,
) {
    if model.x_axis.axis_type == AxisType::Value {
        render_x_value_labels(zr, group, model, option, coord);
        return;
    }
    let g = model.grid;
    let visible = (zoom_end - zoom_start).max(1);
    let formatter = axis_formatter(option, "xAxis");
    for i in zoom_start..zoom_end {
        let raw_label = model
            .x_categories
            .get(i)
            .map(|s| s.as_str())
            .unwrap_or("");
        if raw_label.is_empty() {
            continue;
        }
        let local = i - zoom_start;
        let x = g.x + (local as f64 + 0.5) / visible as f64 * g.width;
        let y = g.y + g.height + 14.0;
        let label = format_label(
            formatter,
            &JsValue::from_str(raw_label),
            i as u32,
            raw_label,
        );
        add_axis_text(zr, group, &label, x, y, TextAlign::Center, TextBaseline::Top);
    }
}

fn render_x_value_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    coord: &Cartesian2D,
) {
    let g = model.grid;
    let split_count = 5;
    let xmin = model.x_axis.value_min();
    let xmax = model.x_axis.value_max();
    let span = xmax - xmin;
    let formatter = axis_formatter(option, "xAxis");
    for i in 0..=split_count {
        let value = xmin + span * i as f64 / split_count as f64;
        let fallback = format_axis_number(value);
        let label = format_label(
            formatter,
            &JsValue::from(value),
            i as u32,
            &fallback,
        );
        let (x, _) = coord.value_to_point(value, model.y_axis.value_min());
        let y = g.y + g.height + 14.0;
        add_axis_text(zr, group, &label, x, y, TextAlign::Center, TextBaseline::Top);
    }
}

fn render_y_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    coord: &Cartesian2D,
) {
    let g = model.grid;
    let split_count = 5;
    let ymin = model.y_axis.value_min();
    let ymax = model.y_axis.value_max();
    let span = ymax - ymin;
    let formatter = axis_formatter(option, "yAxis");

    for i in 0..=split_count {
        let value = ymin + span * i as f64 / split_count as f64;
        let fallback = format_axis_number(value);
        let label = format_label(
            formatter,
            &JsValue::from(value),
            i as u32,
            &fallback,
        );
        let (_, y) = coord.data_to_point(0, value);
        let x = g.x - 8.0;
        add_axis_text(zr, group, &label, x, y, TextAlign::Right, TextBaseline::Middle);
    }
}
