//! Path 样式

use vl_convert_canvas2d::{LineCap, LineJoin};

#[derive(Debug, Clone)]
pub struct PathStyle {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub line_width: f32,
    pub opacity: f32,
    pub line_dash: Option<Vec<f32>>,
    pub line_dash_offset: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub fill_opacity: f32,
    pub stroke_opacity: f32,
}

impl Default for PathStyle {
    fn default() -> Self {
        Self {
            fill: Some("#000".to_string()),
            stroke: None,
            line_width: 1.0,
            opacity: 1.0,
            line_dash: None,
            line_dash_offset: 0.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            fill_opacity: 1.0,
            stroke_opacity: 1.0,
        }
    }
}

impl PathStyle {
    pub fn has_fill(&self) -> bool {
        self.fill
            .as_ref()
            .map(|f| f != "none" && !f.is_empty())
            .unwrap_or(false)
    }

    pub fn has_stroke(&self) -> bool {
        self.stroke
            .as_ref()
            .map(|s| s != "none" && !s.is_empty())
            .unwrap_or(false)
            && self.line_width > 0.0
    }

    pub fn effective_fill_opacity(&self) -> f32 {
        self.opacity * self.fill_opacity
    }

    pub fn effective_stroke_opacity(&self) -> f32 {
        self.opacity * self.stroke_opacity
    }
}
