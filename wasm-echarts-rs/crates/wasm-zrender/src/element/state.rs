//! Element 状态样式（emphasis / select，无 morph 动画）

use std::collections::HashMap;

use crate::graphic::style::{FillStrokeStyle, PathStyle, ShadowStyle};

pub const STATE_NORMAL: &str = "";
pub const STATE_EMPHASIS: &str = "emphasis";
pub const STATE_SELECT: &str = "select";

/// 状态样式补丁：仅覆盖 Some 字段
#[derive(Debug, Clone, Default)]
pub struct PathStylePatch {
    pub fill: Option<FillStrokeStyle>,
    pub stroke: Option<FillStrokeStyle>,
    pub line_width: Option<f32>,
    pub opacity: Option<f32>,
    pub fill_opacity: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub shadow: Option<Option<ShadowStyle>>,
}

impl PathStylePatch {
    pub fn apply_to(&self, base: &PathStyle) -> PathStyle {
        let mut style = base.clone();
        if let Some(fill) = &self.fill {
            style.fill = fill.clone();
        }
        if let Some(stroke) = &self.stroke {
            style.stroke = stroke.clone();
        }
        if let Some(line_width) = self.line_width {
            style.line_width = line_width;
        }
        if let Some(opacity) = self.opacity {
            style.opacity = opacity;
        }
        if let Some(fill_opacity) = self.fill_opacity {
            style.fill_opacity = fill_opacity;
        }
        if let Some(stroke_opacity) = self.stroke_opacity {
            style.stroke_opacity = stroke_opacity;
        }
        if let Some(shadow) = &self.shadow {
            style.shadow = shadow.clone();
        }
        style
    }
}

#[derive(Debug, Clone, Default)]
pub struct ElementStates {
    pub patches: HashMap<String, PathStylePatch>,
    normal_style: Option<PathStyle>,
    pub current: Vec<String>,
}

impl ElementStates {
    pub fn set_state_patch(&mut self, name: impl Into<String>, patch: PathStylePatch) {
        self.patches.insert(name.into(), patch);
    }

    pub fn use_state(&mut self, style: &mut PathStyle, state_name: &str) {
        if state_name.is_empty() || state_name == STATE_NORMAL {
            if let Some(normal) = self.normal_style.take() {
                *style = normal;
            }
            self.current.clear();
            return;
        }

        if self.normal_style.is_none() {
            self.normal_style = Some(style.clone());
        }

        let base = self.normal_style.as_ref().unwrap_or(style);
        if let Some(patch) = self.patches.get(state_name) {
            *style = patch.apply_to(base);
        }
        self.current = vec![state_name.to_string()];
    }

    pub fn use_states(&mut self, style: &mut PathStyle, state_names: &[&str]) {
        if state_names.is_empty() {
            self.use_state(style, STATE_NORMAL);
            return;
        }

        if self.normal_style.is_none() {
            self.normal_style = Some(style.clone());
        }

        let base = self.normal_style.as_ref().unwrap_or(style);
        let mut merged = base.clone();
        for name in state_names {
            if let Some(patch) = self.patches.get(*name) {
                merged = patch.apply_to(&merged);
            }
        }
        *style = merged;
        self.current = state_names.iter().map(|s| s.to_string()).collect();
    }

    pub fn is_emphasis(&self) -> bool {
        self.current.iter().any(|s| s == STATE_EMPHASIS)
    }

    pub fn is_selected(&self) -> bool {
        self.current.iter().any(|s| s == STATE_SELECT)
    }
}
