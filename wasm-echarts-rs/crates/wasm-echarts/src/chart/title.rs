//! title / subtext：padding、textAlign / textVerticalAlign、left/right/top/bottom

use rust_zrender::{TextAlign, TextBaseline, ZRenderer};

use crate::chart::layout::{add_text, parse_align, parse_baseline, parse_padding};
use crate::chart::text_opt::{
    option_component, parse_chart_text_style,
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

    let pad = parse_padding(title.get("padding"), 5.0);
    let width = model.width as f64;
    let height = model.height as f64;
    let (mut x, mut align) = parse_horizontal(title.get("left"), title.get("right"), width, 12.0);
    if let Some(ta) = title.get("textAlign").and_then(|v| v.as_str()) {
        align = parse_align(Some(&OptionValue::String(ta.into())), align);
        if ta == "center" {
            x = width / 2.0;
        } else if ta == "right" {
            x = width - 12.0;
        }
    }
    match align {
        TextAlign::Left => x += pad.left,
        TextAlign::Right => x -= pad.right,
        TextAlign::Center => {}
    }
    let mut top = parse_percent(title.get("top"), height, 10.0);
    if title.get("top").is_none() {
        if let Some(b) = title.get("bottom") {
            top = height - parse_percent(Some(b), height, 0.0) - 24.0;
        }
    }
    top += pad.top;
    let baseline = parse_baseline(title.get("textVerticalAlign"), TextBaseline::Top);

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
    let gap = title
        .get("itemGap")
        .and_then(|v| v.as_f64())
        .unwrap_or(6.0);

    if !text.is_empty() {
        add_text(
            zr,
            group,
            &text,
            x,
            top,
            title_style.to_text_style(align, baseline),
            10.0,
            true,
            None,
        );
    }
    if !subtext.is_empty() {
        let sub_y = if text.is_empty() {
            top
        } else {
            top + title_style.font_size as f64 + gap
        };
        add_text(
            zr,
            group,
            &subtext,
            x,
            sub_y,
            sub_style.to_text_style(align, baseline),
            10.0,
            true,
            None,
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
    left: Option<&OptionValue>,
    right: Option<&OptionValue>,
    width: f64,
    default: f64,
) -> (f64, TextAlign) {
    if let Some(value) = left {
        return match value {
            OptionValue::String(s) => match s.as_str() {
                "center" => (width / 2.0, TextAlign::Center),
                "right" => (width - 12.0, TextAlign::Right),
                "left" => (12.0, TextAlign::Left),
                _ => (parse_percent(Some(value), width, default), TextAlign::Left),
            },
            OptionValue::Number(n) => (*n, TextAlign::Left),
            _ => (default, TextAlign::Left),
        };
    }
    if let Some(value) = right {
        let r = parse_percent(Some(value), width, 12.0);
        return (width - r, TextAlign::Right);
    }
    (default, TextAlign::Left)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::option::SetOptionFlags;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn padding_shifts_title() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![(
                "title",
                obj(vec![
                    ("text", OptionValue::String("Hello".into())),
                    (
                        "padding",
                        OptionValue::Array(vec![
                            OptionValue::Number(20.0),
                            OptionValue::Number(0.0),
                            OptionValue::Number(0.0),
                            OptionValue::Number(30.0),
                        ]),
                    ),
                    ("left", OptionValue::Number(0.0)),
                    ("top", OptionValue::Number(0.0)),
                ]),
            )]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 400, 300);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        render_title(&mut zr, group, &model, &option);
        let t = zr.storage.texts().iter().find(|t| t.content == "Hello").unwrap();
        assert!((t.x - 30.0).abs() < 1e-9, "x={}", t.x);
        assert!((t.y - 20.0).abs() < 1e-9, "y={}", t.y);
    }
}
