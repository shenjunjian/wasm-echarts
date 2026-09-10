//! hover / select / dataZoom / legend / brush / timeline / toolbox 交互状态

use std::collections::{HashMap, HashSet};

use crate::model::GlobalModel;
use crate::option::{OptionModel, OptionValue};
use crate::utils::{as_components, first_component};

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
        let end = ((total as f64 * self.end / 100.0).ceil() as usize)
            .max(start + 1)
            .min(total);
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

    pub fn pan(&mut self, delta_percent: f64) {
        let span = self.end - self.start;
        let mut start = self.start + delta_percent;
        let mut end = self.end + delta_percent;
        if start < 0.0 {
            start = 0.0;
            end = span;
        }
        if end > 100.0 {
            end = 100.0;
            start = 100.0 - span;
        }
        *self = Self::clamped(start, end);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataTarget {
    pub series_index: i32,
    pub data_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragKind {
    ZoomStart,
    ZoomEnd,
    ZoomFiller,
    Brush,
    Thumbnail,
    ToolboxZoom,
}

#[derive(Debug, Clone, Copy)]
pub struct DragState {
    pub kind: DragKind,
    pub start_x: f64,
    #[allow(dead_code)]
    pub start_y: f64,
    pub start_zoom: DataZoomRange,
}

#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    pub hover: Option<DataTarget>,
    pub selected: HashSet<DataTarget>,
    pub data_zoom: DataZoomRange,
    #[allow(dead_code)]
    pub data_zoom_enabled: bool,
    pub data_zoom_from_option: bool,
    pub data_zoom_has_slider: bool,
    pub data_zoom_has_inside: bool,
    pub data_zoom_x_axis_index: Option<usize>,
    pub axis_pointer_enabled: bool,
    pub tooltip_trigger_axis: bool,
    pub pointer_x: Option<f64>,
    pub pointer_y: Option<f64>,
    pub legend_selected: HashMap<String, bool>,
    pub timeline_index: usize,
    pub brush_rect: Option<(f64, f64, f64, f64)>,
    pub toolbox_zoom_select: bool,
    pub drag: Option<DragState>,
    pub restore_snapshot: Option<OptionValue>,
    pub visual_map_selected: HashMap<usize, bool>,
}

impl InteractionState {
    pub fn from_option(option: &OptionModel) -> Self {
        let root = option.root();
        let zoom = parse_data_zoom(root);
        let (axis_pointer_enabled, tooltip_trigger_axis) = parse_axis_pointer(root);
        Self {
            data_zoom: zoom.range,
            data_zoom_enabled: zoom.enabled,
            data_zoom_from_option: zoom.from_option,
            data_zoom_has_slider: zoom.has_slider,
            data_zoom_has_inside: zoom.has_inside,
            data_zoom_x_axis_index: zoom.x_axis_index,
            axis_pointer_enabled: axis_pointer_enabled || tooltip_trigger_axis,
            tooltip_trigger_axis,
            legend_selected: parse_legend_selected(root),
            timeline_index: parse_timeline_index(root),
            visual_map_selected: parse_visual_map_selected(root),
            ..Default::default()
        }
    }

    pub fn absorb_user_option(&mut self, option: &OptionModel, not_merge: bool) {
        let prev_zoom = self.data_zoom;
        let prev_restore = self.restore_snapshot.clone();
        let prev_selected = if not_merge {
            HashSet::new()
        } else {
            self.selected.clone()
        };
        let prev_legend = if not_merge {
            HashMap::new()
        } else {
            self.legend_selected.clone()
        };
        let prev_zoom_select = self.toolbox_zoom_select;
        let prev_vm = if not_merge {
            HashMap::new()
        } else {
            self.visual_map_selected.clone()
        };
        *self = Self::from_option(option);
        if !self.data_zoom_from_option && !not_merge {
            self.data_zoom = prev_zoom;
        }
        self.restore_snapshot = prev_restore;
        self.selected = prev_selected;
        if self.legend_selected.is_empty() {
            self.legend_selected = prev_legend;
        }
        if self.visual_map_selected.is_empty() {
            self.visual_map_selected = prev_vm;
        }
        self.toolbox_zoom_select = prev_zoom_select;
        if self.restore_snapshot.is_none() && toolbox_has_restore(option.root()) {
            self.restore_snapshot = Some(option.root().clone());
        }
    }

    pub fn is_name_selected(&self, name: &str) -> bool {
        if name.is_empty() {
            return true;
        }
        self.legend_selected.get(name).copied().unwrap_or(true)
    }

    pub fn toggle_legend(&mut self, name: &str) -> bool {
        if name.is_empty() {
            return true;
        }
        let next = !self.is_name_selected(name);
        self.legend_selected.insert(name.to_string(), next);
        next
    }

    pub fn toggle_visual_map_piece(&mut self, key: usize) -> bool {
        let next = !self.visual_map_selected.get(&key).copied().unwrap_or(true);
        self.visual_map_selected.insert(key, next);
        next
    }

    pub fn set_legend(&mut self, name: &str, selected: bool) {
        if !name.is_empty() {
            self.legend_selected.insert(name.to_string(), selected);
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

    pub fn begin_drag(&mut self, kind: DragKind, x: f64, y: f64) {
        self.drag = Some(DragState {
            kind,
            start_x: x,
            start_y: y,
            start_zoom: self.data_zoom,
        });
    }

    pub fn end_drag(&mut self) {
        self.drag = None;
    }

    /// 根据指针位置计算 axisPointer 对应的 category 索引与标签
    pub fn axis_pointer_label(
        &self,
        model: &GlobalModel,
        x: f64,
        y: f64,
    ) -> Option<(usize, String, f64, f64)> {
        if !self.axis_pointer_enabled && !self.tooltip_trigger_axis {
            return None;
        }
        if !model.grid().contains(x, y) {
            return None;
        }
        if !model.x_axis().axis_type.is_category() {
            return Some((0, String::new(), x, y));
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
        let grid = model.grid();
        let rel = ((x - grid.x) / grid.width).clamp(0.0, 0.999_999);
        let local = (rel * visible as f64).floor() as usize;
        let global = (start + local).min(total.saturating_sub(1));
        let label = model
            .x_categories()
            .get(global)
            .cloned()
            .unwrap_or_else(|| global.to_string());
        let snap_x = grid.x + (local as f64 + 0.5) / visible as f64 * grid.width;
        Some((global, label, snap_x, y))
    }
}

struct ParsedZoom {
    enabled: bool,
    range: DataZoomRange,
    from_option: bool,
    has_slider: bool,
    has_inside: bool,
    x_axis_index: Option<usize>,
}

fn parse_data_zoom(root: &OptionValue) -> ParsedZoom {
    let comps = as_components(root.get("dataZoom"));
    if comps.is_empty() {
        return ParsedZoom {
            enabled: false,
            range: DataZoomRange::default(),
            from_option: false,
            has_slider: false,
            has_inside: false,
            x_axis_index: None,
        };
    }
    let mut range = DataZoomRange::default();
    let mut from_option = false;
    let mut has_slider = false;
    let mut has_inside = false;
    let mut x_axis_index = None;
    for c in &comps {
        let ty = c.get("type").and_then(|v| v.as_str()).unwrap_or("slider");
        match ty {
            "inside" => has_inside = true,
            _ => has_slider = true,
        }
        if x_axis_index.is_none() {
            x_axis_index = c.get("xAxisIndex").and_then(|v| match v {
                OptionValue::Number(n) => Some(*n as usize),
                OptionValue::Array(arr) => arr.first().and_then(|v| v.as_f64()).map(|n| n as usize),
                _ => None,
            });
        }
        let start = c.get("start").and_then(|v| v.as_f64());
        let end = c.get("end").and_then(|v| v.as_f64());
        if start.is_some() || end.is_some() {
            range = DataZoomRange::clamped(start.unwrap_or(0.0), end.unwrap_or(100.0));
            from_option = true;
        }
    }
    ParsedZoom {
        enabled: true,
        range,
        from_option,
        has_slider,
        has_inside,
        x_axis_index,
    }
}

fn parse_axis_pointer(root: &OptionValue) -> (bool, bool) {
    let from_root = root
        .get("axisPointer")
        .and_then(|v| v.get("show"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let tooltip = first_component(root.get("tooltip"));
    let trigger_axis = tooltip
        .and_then(|t| t.get("trigger"))
        .and_then(|v| v.as_str())
        == Some("axis");
    let from_tooltip = tooltip.and_then(|t| t.get("axisPointer")).is_some() || trigger_axis;
    (from_root || from_tooltip, trigger_axis)
}

fn parse_legend_selected(root: &OptionValue) -> HashMap<String, bool> {
    let mut out = HashMap::new();
    let Some(legend) = first_component(root.get("legend")) else {
        return out;
    };
    let Some(obj) = legend.get("selected").and_then(|v| v.as_object()) else {
        return out;
    };
    for (k, v) in obj {
        if let Some(b) = v.as_bool() {
            out.insert(k.clone(), b);
        }
    }
    out
}

fn parse_timeline_index(root: &OptionValue) -> usize {
    first_component(root.get("timeline"))
        .or_else(|| root.get("baseOption").and_then(|b| first_component(b.get("timeline"))))
        .and_then(|t| t.get("currentIndex"))
        .and_then(|v| v.as_f64())
        .map(|n| n.max(0.0) as usize)
        .unwrap_or(0)
}

fn parse_visual_map_selected(root: &OptionValue) -> HashMap<usize, bool> {
    let mut out = HashMap::new();
    for (vi, vm) in as_components(root.get("visualMap")).into_iter().enumerate() {
        if let Some(obj) = vm.get("selected").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let (Ok(i), Some(b)) = (k.parse::<usize>(), v.as_bool()) {
                    out.insert(vi * 100 + i, b);
                }
            }
        }
    }
    out
}

fn toolbox_has_restore(root: &OptionValue) -> bool {
    first_component(root.get("toolbox"))
        .and_then(|t| t.get("feature"))
        .and_then(|f| f.get("restore"))
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::option::OptionModel;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

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

    #[test]
    fn legend_selected_from_option() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![(
                "legend",
                obj(vec![(
                    "selected",
                    obj(vec![("A", OptionValue::Bool(false))]),
                )]),
            )]),
            crate::option::SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let st = InteractionState::from_option(&option);
        assert!(!st.is_name_selected("A"));
        assert!(st.is_name_selected("B"));
    }

    #[test]
    fn slider_and_start_end_from_option() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![(
                "dataZoom",
                OptionValue::Array(vec![
                    obj(vec![
                        ("type", OptionValue::String("inside".into())),
                        ("start", OptionValue::Number(10.0)),
                        ("end", OptionValue::Number(60.0)),
                        ("xAxisIndex", OptionValue::Number(0.0)),
                    ]),
                    obj(vec![("type", OptionValue::String("slider".into()))]),
                ]),
            )]),
            crate::option::SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let st = InteractionState::from_option(&option);
        assert!(st.data_zoom_has_slider && st.data_zoom_has_inside);
        assert_eq!(st.data_zoom_x_axis_index, Some(0));
        assert!((st.data_zoom.start - 10.0).abs() < 1e-9);
        assert!((st.data_zoom.end - 60.0).abs() < 1e-9);
    }
}
