//! 文本图元（基于 vl-convert fill_text）

use crate::element::ElementBase;
use crate::graphic::displayable::DisplayableProps;
use crate::element::EcData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBaseline {
    Top,
    Middle,
    Bottom,
    Alphabetic,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub fill: String,
    pub font_size: f32,
    pub align: TextAlign,
    pub baseline: TextBaseline,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            fill: "#333".into(),
            font_size: 12.0,
            align: TextAlign::Left,
            baseline: TextBaseline::Alphabetic,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub base: ElementBase,
    pub displayable: DisplayableProps,
    pub ec_data: EcData,
    pub content: String,
    pub x: f64,
    pub y: f64,
    pub style: TextStyle,
    /// 不参与命中检测（轴标签等）
    pub silent: bool,
}

impl Text {
    pub fn new(content: impl Into<String>, x: f64, y: f64) -> Self {
        Self {
            base: ElementBase::default(),
            displayable: DisplayableProps::default(),
            ec_data: EcData::default(),
            content: content.into(),
            x,
            y,
            style: TextStyle::default(),
            silent: true,
        }
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_displayable(mut self, props: DisplayableProps) -> Self {
        self.displayable = props;
        self
    }
}
