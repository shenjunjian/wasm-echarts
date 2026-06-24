//! 文本图元（基于 vl-convert fill_text）

use crate::core::bbox::BoundingRect;
use crate::element::EcData;
use crate::element::ElementBase;
use crate::graphic::displayable::DisplayableProps;

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
            silent: false,
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

    pub fn with_ec_data(mut self, ec_data: EcData) -> Self {
        self.ec_data = ec_data;
        self
    }

    pub fn estimate_text_width(content: &str, font_size: f32) -> f64 {
        content.chars().fold(0.0, |width, ch| {
            width
                + if ch.is_ascii() {
                    font_size as f64 * 0.55
                } else {
                    font_size as f64
                }
        })
    }

    pub fn estimate_line_height(font_size: f32) -> f64 {
        font_size as f64
    }

    pub fn bounding_rect(&self) -> BoundingRect {
        let width = Self::estimate_text_width(&self.content, self.style.font_size);
        let height = Self::estimate_line_height(self.style.font_size);
        let x = match self.style.align {
            TextAlign::Left => self.x,
            TextAlign::Center => self.x - width / 2.0,
            TextAlign::Right => self.x - width,
        };
        let y = match self.style.baseline {
            TextBaseline::Top => self.y,
            TextBaseline::Middle => self.y - height / 2.0,
            TextBaseline::Bottom => self.y - height,
            TextBaseline::Alphabetic => self.y - height * 0.8,
        };
        BoundingRect::new(x, y, width, height)
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        if let Some((lx, ly)) = invert_transform_point(self.base.transform(), x, y) {
            let rect = self.bounding_rect();
            lx >= rect.x
                && ly >= rect.y
                && lx <= rect.x + rect.width
                && ly <= rect.y + rect.height
        } else {
            false
        }
    }

    pub fn hit_test(&self, x: f64, y: f64) -> bool {
        self.contains(x, y)
    }
}

fn invert_transform_point(m: &[f32; 6], x: f64, y: f64) -> Option<(f64, f64)> {
    let det = m[0] as f64 * m[3] as f64 - m[2] as f64 * m[1] as f64;
    if det.abs() < 1e-12 {
        return None;
    }
    let inv_det = 1.0 / det;
    let tx = x - m[4] as f64;
    let ty = y - m[5] as f64;
    Some((
        (m[3] as f64 * tx - m[2] as f64 * ty) * inv_det,
        (-m[1] as f64 * tx + m[0] as f64 * ty) * inv_det,
    ))
}
