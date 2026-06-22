//! 阶段 6：hover / select / dataZoom / axisPointer 交互状态

use std::collections::HashSet;

use crate::model::GlobalModel;
use crate::option::{OptionModel, OptionValue};

/// dataZoom 可视范围（百分比 0–100）
#[derive(Debug, Clone, Copy)]
pub struct DataZoomRange {
    pub start: f64,
    pub end: f64,
}

impl Default for DataZoomRange {
    fn default() -> Self {
        Self {
            start: 0.0,
            end: 100.0,
        }
    }
}

impl DataZoomRange {
    pub fn clamped(start: f64, end: f64) -> Self {
        let start = start.clamp(0.0, 100.0);
        let end = end.clamp(0.0, 100.0);
        if start >= end {
            Self {
                start: 0.0,
                end: 100.0,
            }
        } else {
            Self { start, end }
        }
    }

    pub fn category_window(&self, total: usize) -> (usize, usize) {
        if total == 0 {
            return (0, 0);
        }
        let start = ((total as f64 * self.start / 100.0).floor() as usize).min(total.saturating_sub(1));
        let end = ((total as f64 * self.end / 100.0).ceil() as usize).max(start + 1).min(total);
        (start, end)
    }

    pub fn zoom_wheel(&mut self, delta_y: f64, anchor_ratio: f64) {
        let span = (self.end - self.start).max(1.0);
        let factor = if delta_y > 0.0 { 1.12 } else { 0.88 };
        let new_span = (span * factor).clamp(5.0, 100.0);
        let anchor = self.start + span * anchor_ratio.clamp(0.0, 1.0);
        let mut start = anchor - new_span * anchor_ratio;
        let mut end = start + new_span;
        if start < 0.0 {
            start = 0.0;
            end = new_span;
        }
        if end > 100.0 {
            end = 100.0;
            start = 100.0 - new_span;
        }
        *self = Self::clamped(start, end);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataTarget {
    pub series_index: i32,
    pub data_index: i32,
}

#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    pub hover: Option<DataTarget>,
    pub selected: HashSet<DataTarget>,
    pub data_zoom: DataZoomRange,
    pub data_zoom_enabled: bool,
    pub axis_pointer_enabled: bool,
    pub pointer_x: Option<f64>,
    pub pointer_y: Option<f64>,
}

impl InteractionState {
    pub fn from_option(option: &OptionModel) -> Self {
        let root = option.root();
        let data_zoom_enabled = parse_data_zoom_enabled(root);
        let axis_pointer_enabled = parse_axis_pointer_enabled(root);
        Self {
            data_zoom_enabled,
            axis_pointer_enabled,
            ..Default::default()
        }
    }

    pub fn set_hover(&mut self, target: Option<DataTarget>) {
        self.hover = target;
    }

    pub fn toggle_select(&mut self, target: DataTarget) {
        if self.selected.contains(&target) {
            self.selected.remove(&target);
        } else {
            self.selected.insert(target);
        }
    }

    pub fn select(&mut self, target: DataTarget) {
        self.selected.insert(target);
    }

    pub fn unselect(&mut self, target: DataTarget) {
        self.selected.remove(&target);
    }

    pub fn clear_select(&mut self) {
        self.selected.clear();
    }

    pub fn set_data_zoom_range(&mut self, start: f64, end: f64) {
        self.data_zoom = DataZoomRange::clamped(start, end);
    }

    pub fn set_pointer(&mut self, x: Option<f64>, y: Option<f64>) {
        self.pointer_x = x;
        self.pointer_y = y;
    }

    /// 根据指针位置计算 axisPointer 对应的 category 索引与标签
    pub fn axis_pointer_label(&self, model: &GlobalModel, x: f64, y: f64) -> Option<(usize, String, f64)> {
        if !self.axis_pointer_enabled || !model.grid.contains(x, y) {
            return None;
        }
        let total = model.category_count();
        if total == 0 {
            return None;
        }
        let (start, end) = model.data_zoom.category_window(total);
        let visible = end - start;
        if visible == 0 {
            return None;
        }
        let grid = model.grid;
        let rel = ((x - grid.x) / grid.width).clamp(0.0, 0.999_999);
        let local = (rel * visible as f64).floor() as usize;
        let global = (start + local).min(total.saturating_sub(1));
        let label = model
            .x_categories
            .get(global)
            .cloned()
            .unwrap_or_else(|| global.to_string());
        let snap_x = grid.x + (local as f64 + 0.5) / visible as f64 * grid.width;
        Some((global, label, snap_x))
    }
}

fn parse_data_zoom_enabled(root: &OptionValue) -> bool {
    root.get("dataZoom")
        .and_then(|v| v.as_array())
        .map(|arr| !arr.is_empty())
        .unwrap_or(false)
        || root.get("dataZoom").and_then(|v| v.as_object()).is_some()
}

fn parse_axis_pointer_enabled(root: &OptionValue) -> bool {
    let from_root = root
        .get("axisPointer")
        .and_then(|v| v.get("show"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let from_tooltip = root
        .get("tooltip")
        .and_then(|t| t.get("axisPointer"))
        .is_some();
    from_root || from_tooltip
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_zoom_window() {
        let range = DataZoomRange::clamped(20.0, 80.0);
        assert_eq!(range.category_window(10), (2, 8));
    }

    #[test]
    fn wheel_zoom_clamps() {
        let mut range = DataZoomRange::default();
        for _ in 0..20 {
            range.zoom_wheel(100.0, 0.5);
        }
        assert!(range.end - range.start >= 5.0);
    }
}
