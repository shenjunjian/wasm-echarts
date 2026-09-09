//! legend 最小绘制：色块 + 系列名（读 textStyle.fontFamily）

use rust_zrender::{
    ChildRef, FillStrokeStyle, Path, PathStyle, RectShape, Shape, TextAlign, TextBaseline,
    Text, ZRenderer,
};

use crate::bridge::default_series_color;
use crate::chart::text_opt::{
    add_silent_text, option_component, parse_chart_text_style,
};
use crate::model::GlobalModel;
use crate::option::{OptionModel, OptionValue};
use crate::utils::parse_percent;

pub fn render_legend(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
) {
    let Some(legend) = option_component(option.root(), "legend") else {
        return;
    };
    if legend.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }

    let names = legend_names(legend, model);
    if names.is_empty() {
        return;
    }

    let style = parse_chart_text_style(legend.get("textStyle"), "#333", 12.0, "sans-serif");
    let item_width = 14.0;
    let item_gap = 18.0;
    let mut total = 0.0;
    let mut widths = Vec::with_capacity(names.len());
    for name in &names {
        let w = Text::estimate_text_width(name, style.font_size);
        widths.push(w);
        total += item_width + 6.0 + w + item_gap;
    }
    total -= item_gap;

    let width = model.width as f64;
    let (origin_x, _) = parse_legend_left(legend.get("left"), width, total);
    let y = parse_percent(legend.get("top"), model.height as f64, 40.0);

    let mut x = origin_x;
    for (i, name) in names.iter().enumerate() {
        let color = default_series_color(i);
        let marker = zr.storage.create_path(Path::new(
            Shape::Rect(RectShape {
                x,
                y: y + 2.0,
                width: item_width,
                height: 10.0,
                ..Default::default()
            }),
            PathStyle {
                fill: FillStrokeStyle::color(color),
                ..Default::default()
            },
        ));
        zr.storage.group_add_child(group, ChildRef::Path(marker));

        add_silent_text(
            zr,
            group,
            name,
            x + item_width + 6.0,
            y,
            style.to_text_style(TextAlign::Left, TextBaseline::Top),
            10.0,
        );
        x += item_width + 6.0 + widths[i] + item_gap;
    }
}

fn legend_names(legend: &OptionValue, model: &GlobalModel) -> Vec<String> {
    if let Some(data) = legend.get("data").and_then(|v| v.as_array()) {
        return data
            .iter()
            .filter_map(|item| match item {
                OptionValue::String(s) if !s.is_empty() => Some(s.clone()),
                OptionValue::Object(_) => item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                _ => None,
            })
            .collect();
    }
    model
        .series
        .iter()
        .map(|s| s.name.clone())
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_legend_left(value: Option<&OptionValue>, width: f64, total: f64) -> (f64, rust_zrender::TextAlign) {
    match value {
        Some(OptionValue::String(s)) => match s.as_str() {
            "center" => ((width - total).max(0.0) / 2.0, rust_zrender::TextAlign::Left),
            "right" => ((width - total - 12.0).max(0.0), rust_zrender::TextAlign::Left),
            "left" => (12.0, rust_zrender::TextAlign::Left),
            _ => (parse_percent(value, width, 12.0), rust_zrender::TextAlign::Left),
        },
        Some(OptionValue::Number(n)) => (*n, rust_zrender::TextAlign::Left),
        _ => ((width - total).max(0.0) / 2.0, rust_zrender::TextAlign::Left),
    }
}
