//! title / subtext 最小绘制（读 textStyle.fontFamily）

use rust_zrender::{TextAlign, TextBaseline, ZRenderer};

use crate::chart::text_opt::{
    add_silent_text, option_component, parse_chart_text_style,
};
use crate::model::GlobalModel;
use crate::option::{OptionModel, OptionValue};
use crate::utils::parse_percent;

pub fn render_title(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
) {
    let Some(title) = option_component(option.root(), "title") else {
        return;
    };
    if title.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }

    let text = title_string(title, "text");
    let subtext = title_string(title, "subtext");
    if text.is_empty() && subtext.is_empty() {
        return;
    }

    let width = model.width as f64;
    let (x, align) = parse_horizontal(title.get("left"), width, 12.0);
    let top = parse_percent(title.get("top"), model.height as f64, 10.0);

    let title_style = parse_chart_text_style(
        title.get("textStyle"),
        "#464646",
        18.0,
        "sans-serif",
    );
    let sub_style = parse_chart_text_style(
        title.get("subtextStyle"),
        "#6e7079",
        12.0,
        "sans-serif",
    );

    if !text.is_empty() {
        add_silent_text(
            zr,
            group,
            &text,
            x,
            top,
            title_style.to_text_style(align, TextBaseline::Top),
            10.0,
        );
    }
    if !subtext.is_empty() {
        let sub_y = if text.is_empty() {
            top
        } else {
            top + title_style.font_size as f64 + 6.0
        };
        add_silent_text(
            zr,
            group,
            &subtext,
            x,
            sub_y,
            sub_style.to_text_style(align, TextBaseline::Top),
            10.0,
        );
    }
}

fn title_string(title: &OptionValue, key: &str) -> String {
    if key == "text" {
        if let Some(s) = title.as_str() {
            return s.to_string();
        }
    }
    title
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn parse_horizontal(
    value: Option<&OptionValue>,
    width: f64,
    default: f64,
) -> (f64, TextAlign) {
    match value {
        Some(OptionValue::String(s)) => match s.as_str() {
            "center" => (width / 2.0, TextAlign::Center),
            "right" => (width - 12.0, TextAlign::Right),
            "left" => (12.0, TextAlign::Left),
            _ => (parse_percent(value, width, default), TextAlign::Left),
        },
        Some(OptionValue::Number(n)) => (*n, TextAlign::Left),
        _ => (default, TextAlign::Left),
    }
}
