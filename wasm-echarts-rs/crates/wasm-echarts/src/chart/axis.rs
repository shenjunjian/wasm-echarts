//! 坐标轴刻度标签

use rust_zrender::{
    ChildRef, Text, TextAlign, TextBaseline, TextStyle, ZRenderer,
};

use crate::coord::Cartesian2D;
use crate::model::GlobalModel;

pub fn render_axis_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &Cartesian2D,
    zoom_start: usize,
    zoom_end: usize,
) {
    render_x_labels(zr, group, model, coord, zoom_start, zoom_end);
    render_y_labels(zr, group, model, coord);
}

fn render_x_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &Cartesian2D,
    zoom_start: usize,
    zoom_end: usize,
) {
    if model.x_axis.axis_type == crate::model::AxisType::Value {
        render_x_value_labels(zr, group, model, coord);
        return;
    }
    let g = model.grid;
    let visible = (zoom_end - zoom_start).max(1);
    for i in zoom_start..zoom_end {
        let label = model
            .x_categories
            .get(i)
            .map(|s| s.as_str())
            .unwrap_or("");
        if label.is_empty() {
            continue;
        }
        let local = i - zoom_start;
        let x = g.x + (local as f64 + 0.5) / visible as f64 * g.width;
        let y = g.y + g.height + 14.0;
        let text_idx = zr.storage.create_text(
            Text::new(label, x, y)
                .with_style(TextStyle {
                    fill: "#666".into(),
                    font_size: 11.0,
                    align: TextAlign::Center,
                    baseline: TextBaseline::Top,
                }),
        );
        zr.storage.text_mut(text_idx).silent = true;
        zr.storage.group_add_child(group, ChildRef::Text(text_idx));
    }
}

fn render_x_value_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &Cartesian2D,
) {
    let g = model.grid;
    let split_count = 5;
    let xmin = model.x_axis.value_min();
    let xmax = model.x_axis.value_max();
    let span = xmax - xmin;
    for i in 0..=split_count {
        let value = xmin + span * i as f64 / split_count as f64;
        let label = format_axis_number(value);
        let (x, _) = coord.value_to_point(value, model.y_axis.value_min());
        let y = g.y + g.height + 14.0;
        let text_idx = zr.storage.create_text(
            Text::new(&label, x, y).with_style(TextStyle {
                fill: "#666".into(),
                font_size: 11.0,
                align: TextAlign::Center,
                baseline: TextBaseline::Top,
            }),
        );
        zr.storage.text_mut(text_idx).silent = true;
        zr.storage.group_add_child(group, ChildRef::Text(text_idx));
    }
}

fn render_y_labels(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &Cartesian2D,
) {
    let g = model.grid;
    let split_count = 5;
    let ymin = model.y_axis.value_min();
    let ymax = model.y_axis.value_max();
    let span = ymax - ymin;

    for i in 0..=split_count {
        let value = ymin + span * i as f64 / split_count as f64;
        let label = format_axis_number(value);
        let (_, y) = coord.data_to_point(0, value);
        let x = g.x - 8.0;
        let text_idx = zr.storage.create_text(
            Text::new(&label, x, y).with_style(TextStyle {
                fill: "#666".into(),
                font_size: 11.0,
                align: TextAlign::Right,
                baseline: TextBaseline::Middle,
            }),
        );
        zr.storage.text_mut(text_idx).silent = true;
        zr.storage.group_add_child(group, ChildRef::Text(text_idx));
    }
}

fn format_axis_number(v: f64) -> String {
    if v.abs() >= 1000.0 || (v.abs() < 0.01 && v != 0.0) {
        format!("{v:.1e}")
    } else if (v - v.round()).abs() < 0.001 {
        format!("{}", v.round() as i64)
    } else {
        format!("{v:.1}")
    }
}
