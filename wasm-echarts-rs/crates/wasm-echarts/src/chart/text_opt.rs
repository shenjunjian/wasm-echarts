//! 从 option 读 textStyle / nameTextStyle / axisLabel 的字体字段

use rust_zrender::{
    ChildRef, DisplayableProps, Text, TextAlign, TextBaseline, TextStyle, ZRenderer,
};

use crate::option::OptionValue;
use crate::utils::first_component;

#[derive(Debug, Clone)]
pub struct ChartTextStyle {
    pub fill: String,
    pub font_size: f32,
    pub font_family: String,
}

impl ChartTextStyle {
    pub fn to_text_style(&self, align: TextAlign, baseline: TextBaseline) -> TextStyle {
        TextStyle {
            fill: self.fill.clone(),
            font_size: self.font_size,
            font_family: self.font_family.clone(),
            align,
            baseline,
        }
    }
}

pub fn parse_chart_text_style(
    style: Option<&OptionValue>,
    default_fill: &str,
    default_size: f32,
    default_family: &str,
) -> ChartTextStyle {
    ChartTextStyle {
        fill: style
            .and_then(|s| s.get("color"))
            .and_then(|v| v.as_str())
            .unwrap_or(default_fill)
            .to_string(),
        font_size: style
            .and_then(|s| s.get("fontSize"))
            .and_then(|v| v.as_f64())
            .unwrap_or(default_size as f64) as f32,
        font_family: style
            .and_then(|s| s.get("fontFamily"))
            .and_then(|v| v.as_str())
            .unwrap_or(default_family)
            .to_string(),
    }
}

pub fn option_component<'a>(root: &'a OptionValue, key: &str) -> Option<&'a OptionValue> {
    first_component(root.get(key))
}

pub fn add_silent_text(
    zr: &mut ZRenderer,
    group: usize,
    content: &str,
    x: f64,
    y: f64,
    style: TextStyle,
    z: f64,
) {
    if content.is_empty() {
        return;
    }
    let idx = zr.storage.create_text(
        Text::new(content, x, y)
            .with_style(style)
            .with_displayable(DisplayableProps {
                z,
                ..Default::default()
            }),
    );
    zr.storage.text_mut(idx).silent = true;
    zr.storage.group_add_child(group, ChildRef::Text(idx));
}
