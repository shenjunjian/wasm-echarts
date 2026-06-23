//! Path 样式：纯色、渐变、Pattern、阴影

use std::sync::Arc;

use vl_convert_canvas2d::{LineCap, LineJoin};

#[derive(Debug, Clone)]
pub struct ColorStop {
    pub offset: f64,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct LinearGradientStyle {
    pub x: f64,
    pub y: f64,
    pub x2: f64,
    pub y2: f64,
    pub color_stops: Vec<ColorStop>,
    pub global: bool,
}

impl Default for LinearGradientStyle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            x2: 1.0,
            y2: 0.0,
            color_stops: Vec::new(),
            global: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadialGradientStyle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    /// 内圆半径（vl-convert RadialGradientParams.r0）
    pub r0: f64,
    pub color_stops: Vec<ColorStop>,
    pub global: bool,
}

impl Default for RadialGradientStyle {
    fn default() -> Self {
        Self {
            x: 0.5,
            y: 0.5,
            r: 0.5,
            r0: 0.0,
            color_stops: Vec::new(),
            global: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatternStyle {
    pub data: Arc<[u8]>,
    pub width: u32,
    pub height: u32,
    pub repeat: String,
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone)]
pub enum FillStrokeStyle {
    None,
    Color(String),
    LinearGradient(LinearGradientStyle),
    RadialGradient(RadialGradientStyle),
    Pattern(PatternStyle),
}

impl Default for FillStrokeStyle {
    fn default() -> Self {
        Self::Color("#000".to_string())
    }
}

impl FillStrokeStyle {
    pub fn none() -> Self {
        Self::None
    }

    pub fn color(value: impl Into<String>) -> Self {
        Self::Color(value.into())
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Self::None => false,
            Self::Color(c) => c != "none" && !c.is_empty(),
            Self::LinearGradient(g) => !g.color_stops.is_empty(),
            Self::RadialGradient(g) => !g.color_stops.is_empty(),
            Self::Pattern(p) => !p.data.is_empty() && p.width > 0 && p.height > 0,
        }
    }

    pub fn as_color(&self) -> Option<&str> {
        match self {
            Self::Color(c) => Some(c.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShadowStyle {
    pub color: String,
    pub blur: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Default for ShadowStyle {
    fn default() -> Self {
        Self {
            color: "rgba(0, 0, 0, 0.3)".to_string(),
            blur: 4.0,
            offset_x: 2.0,
            offset_y: 2.0,
        }
    }
}

impl ShadowStyle {
    pub fn is_active(&self) -> bool {
        self.blur > 0.0 || self.offset_x != 0.0 || self.offset_y != 0.0
    }
}

#[derive(Debug, Clone)]
pub struct PathStyle {
    pub fill: FillStrokeStyle,
    pub stroke: FillStrokeStyle,
    pub line_width: f32,
    pub opacity: f32,
    pub line_dash: Option<Vec<f32>>,
    pub line_dash_offset: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub fill_opacity: f32,
    pub stroke_opacity: f32,
    pub shadow: Option<ShadowStyle>,
    pub stroke_first: bool,
}

impl Default for PathStyle {
    fn default() -> Self {
        Self {
            fill: FillStrokeStyle::Color("#000".to_string()),
            stroke: FillStrokeStyle::None,
            line_width: 1.0,
            opacity: 1.0,
            line_dash: None,
            line_dash_offset: 0.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            fill_opacity: 1.0,
            stroke_opacity: 1.0,
            shadow: None,
            stroke_first: false,
        }
    }
}

impl PathStyle {
    pub fn has_fill(&self) -> bool {
        self.fill.is_visible()
    }

    pub fn has_stroke(&self) -> bool {
        self.stroke.is_visible() && self.line_width > 0.0
    }

    pub fn effective_fill_opacity(&self) -> f32 {
        self.opacity * self.fill_opacity
    }

    pub fn effective_stroke_opacity(&self) -> f32 {
        self.opacity * self.stroke_opacity
    }
}
