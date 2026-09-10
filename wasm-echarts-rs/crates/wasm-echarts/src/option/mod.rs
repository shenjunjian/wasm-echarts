//! ECharts option 解析与合并（保留 JsFunction，不用 serde 整包反序列化）

mod media;
mod merge;
mod parse;

pub use merge::{merge_option, MergeMode};
pub use parse::{option_value_to_js, parse_option_value};

/// `setOption` 第二参数（官方 `notMerge | SetOptionOpts`）
#[derive(Debug, Clone, Default)]
pub struct SetOptionFlags {
    pub not_merge: bool,
    pub replace_merge: Vec<String>,
}

use indexmap::IndexMap;
use js_sys::Function;

/// 递归 option 值；function 分支保留 `js_sys::Function` 引用（由 JS GC 管理）
#[derive(Debug, Clone)]
pub enum OptionValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<OptionValue>),
    Object(IndexMap<String, OptionValue>),
    Function(Function),
}

impl OptionValue {
    pub fn as_object(&self) -> Option<&IndexMap<String, OptionValue>> {
        match self {
            OptionValue::Object(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[OptionValue]> {
        match self {
            OptionValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut IndexMap<String, OptionValue>> {
        match self {
            OptionValue::Object(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<OptionValue>> {
        match self {
            OptionValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut OptionValue> {
        self.as_object_mut()?.get_mut(key)
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            OptionValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            OptionValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            OptionValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&OptionValue> {
        self.as_object()?.get(key)
    }

    pub fn is_function(&self) -> bool {
        matches!(self, OptionValue::Function(_))
    }
}

/// 已合并的 option 模型（阶段 5 起由 GlobalModel 消费）
#[derive(Debug, Clone)]
pub struct OptionModel {
    root: OptionValue,
}

impl Default for OptionModel {
    fn default() -> Self {
        Self::new()
    }
}

impl OptionModel {
    pub fn new() -> Self {
        Self {
            root: OptionValue::Object(IndexMap::new()),
        }
    }

    pub fn with_root(root: OptionValue) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &OptionValue {
        &self.root
    }

    pub fn is_empty(&self) -> bool {
        match &self.root {
            OptionValue::Object(map) => map.is_empty(),
            OptionValue::Null => true,
            _ => false,
        }
    }

    /// 解析 JsValue；`notMerge` / `replaceMerge` 只来自第二参数，不从 option 根读取。
    pub fn set_option(
        &mut self,
        option: &wasm_bindgen::JsValue,
        flags: SetOptionFlags,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let incoming = parse_option_value(option)?;
        self.apply(incoming, flags);
        Ok(())
    }

    pub fn apply(&mut self, incoming: OptionValue, flags: SetOptionFlags) {
        if flags.not_merge || self.is_empty() {
            self.root = incoming;
            return;
        }
        self.root = merge_option(
            &self.root,
            &incoming,
            MergeMode {
                replace_merge: flags.replace_merge,
            },
        );
    }

    pub fn to_js(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
        option_value_to_js(&self.root)
    }

    pub fn append_series_data(
        &mut self,
        series_index: usize,
        extra: OptionValue,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let series = self
            .root
            .get_mut("series")
            .and_then(|v| v.as_array_mut())
            .and_then(|arr| arr.get_mut(series_index))
            .ok_or_else(|| wasm_bindgen::JsValue::from_str("appendData: seriesIndex out of range"))?;
        let data = match series.as_object_mut() {
            Some(map) => map
                .entry("data".to_string())
                .or_insert_with(|| OptionValue::Array(Vec::new())),
            None => {
                return Err(wasm_bindgen::JsValue::from_str(
                    "appendData: series is not an object",
                ))
            }
        };
        if data.as_array().is_none() {
            *data = OptionValue::Array(Vec::new());
        }
        let arr = data
            .as_array_mut()
            .ok_or_else(|| wasm_bindgen::JsValue::from_str("appendData: series.data is not an array"))?;
        match extra {
            OptionValue::Array(items) => arr.extend(items),
            other => arr.push(other),
        }
        Ok(())
    }

    pub fn apply_theme(&mut self, theme: OptionValue) {
        match &theme {
            OptionValue::Null => return,
            OptionValue::Object(m) if m.is_empty() => return,
            _ => {}
        }
        self.root = merge_option(
            &theme,
            &self.root,
            MergeMode {
                replace_merge: Vec::new(),
            },
        );
    }

    pub fn clear(&mut self) {
        self.root = OptionValue::Object(IndexMap::new());
    }

    pub fn set_legend_selected(&mut self, name: &str, selected: bool) {
        let legend = match self.root.get_mut("legend") {
            Some(OptionValue::Array(arr)) => arr.first_mut(),
            Some(v) => Some(v),
            None => {
                self.root.as_object_mut().map(|m| {
                    m.insert("legend".into(), OptionValue::Object(IndexMap::new()));
                    m.get_mut("legend").unwrap()
                })
            }
        };
        let Some(legend) = legend else {
            return;
        };
        if legend.as_object().is_none() {
            *legend = OptionValue::Object(IndexMap::new());
        }
        let map = match legend.as_object_mut() {
            Some(m) => m,
            None => return,
        };
        let selected_map = map
            .entry("selected".to_string())
            .or_insert_with(|| OptionValue::Object(IndexMap::new()));
        if selected_map.as_object().is_none() {
            *selected_map = OptionValue::Object(IndexMap::new());
        }
        if let Some(obj) = selected_map.as_object_mut() {
            obj.insert(name.to_string(), OptionValue::Bool(selected));
        }
    }

    pub fn set_visual_map_selected(&mut self, vm_index: usize, piece: usize, selected: bool) {
        let vm = match self.root.get_mut("visualMap") {
            Some(OptionValue::Array(arr)) => arr.get_mut(vm_index),
            Some(v) if vm_index == 0 => Some(v),
            None if vm_index == 0 => self.root.as_object_mut().map(|m| {
                m.insert("visualMap".into(), OptionValue::Object(IndexMap::new()));
                m.get_mut("visualMap").unwrap()
            }),
            _ => None,
        };
        let Some(vm) = vm else {
            return;
        };
        if vm.as_object().is_none() {
            *vm = OptionValue::Object(IndexMap::new());
        }
        let map = match vm.as_object_mut() {
            Some(m) => m,
            None => return,
        };
        let selected_map = map
            .entry("selected".to_string())
            .or_insert_with(|| OptionValue::Object(IndexMap::new()));
        if selected_map.as_object().is_none() {
            *selected_map = OptionValue::Object(IndexMap::new());
        }
        if let Some(obj) = selected_map.as_object_mut() {
            obj.insert(piece.to_string(), OptionValue::Bool(selected));
        }
    }

    pub fn set_timeline_index(&mut self, index: usize) {
        let n = OptionValue::Number(index as f64);
        if let Some(tl) = first_mut_component(&mut self.root, "timeline") {
            if let Some(map) = tl.as_object_mut() {
                map.insert("currentIndex".into(), n);
                return;
            }
        }
        if let Some(base) = self.root.get_mut("baseOption") {
            if let Some(tl) = first_mut_component(base, "timeline") {
                if let Some(map) = tl.as_object_mut() {
                    map.insert("currentIndex".into(), n);
                }
            }
        }
    }

    pub fn set_cartesian_series_type(&mut self, ty: &str) {
        let Some(arr) = self.root.get_mut("series").and_then(|v| v.as_array_mut()) else {
            return;
        };
        for item in arr {
            let Some(map) = item.as_object_mut() else {
                continue;
            };
            let cur = map.get("type").and_then(|v| v.as_str()).unwrap_or("line");
            if matches!(cur, "line" | "bar") {
                map.insert("type".into(), OptionValue::String(ty.into()));
            }
        }
    }

    /// timeline + media：base → timeline `options[index]` → 匹配的 media（后者优先）。
    pub fn effective_root(&self, timeline_index: usize, width: f64, height: f64) -> OptionValue {
        effective_option_root(&self.root, timeline_index, width, height)
    }

    pub fn set_axis_break_expanded(
        &mut self,
        axis_key: &str,
        axis_index: usize,
        start: f64,
        end: f64,
        expanded: Option<bool>,
    ) {
        set_break_expanded(&mut self.root, axis_key, axis_index, start, end, expanded);
    }
}

fn first_mut_component<'a>(root: &'a mut OptionValue, key: &str) -> Option<&'a mut OptionValue> {
    match root.get_mut(key) {
        Some(OptionValue::Array(arr)) => arr.first_mut(),
        Some(v) => Some(v),
        None => None,
    }
}

pub fn effective_timeline_root(root: &OptionValue, timeline_index: usize) -> OptionValue {
    effective_option_root(root, timeline_index, 0.0, 0.0)
}

pub fn effective_option_root(
    root: &OptionValue,
    timeline_index: usize,
    width: f64,
    height: f64,
) -> OptionValue {
    let has_media = root
        .get("media")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    let has_timeline = root
        .get("options")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false)
        || root.get("baseOption").is_some();
    if !has_media && !has_timeline {
        return root.clone();
    }

    let mut base = media::extract_base_option(root);
    if has_timeline {
        if let Some(options) = root.get("options").and_then(|v| v.as_array()) {
            if !options.is_empty() {
                let overlay = &options[timeline_index.min(options.len() - 1)];
                base = merge_option(
                    &base,
                    overlay,
                    MergeMode {
                        replace_merge: Vec::new(),
                    },
                );
            }
        }
    }
    if has_media && width > 0.0 && height > 0.0 {
        base = media::apply_media(base, root, width, height);
    }
    base
}

fn set_break_expanded(
    root: &mut OptionValue,
    axis_key: &str,
    axis_index: usize,
    start: f64,
    end: f64,
    expanded: Option<bool>,
) {
    let axis = match root.get_mut(axis_key) {
        Some(OptionValue::Array(arr)) => arr.get_mut(axis_index),
        Some(v) if axis_index == 0 => Some(v),
        _ => None,
    };
    let Some(axis) = axis else {
        return;
    };
    let Some(breaks) = axis.get_mut("breaks").and_then(|v| v.as_array_mut()) else {
        return;
    };
    for item in breaks {
        let Some(map) = item.as_object_mut() else {
            continue;
        };
        let b_start = map.get("start").and_then(|v| v.as_f64());
        let b_end = map.get("end").and_then(|v| v.as_f64());
        if b_start != Some(start) || b_end != Some(end) {
            continue;
        }
        let next = match expanded {
            Some(v) => v,
            None => !map.get("isExpanded").and_then(|v| v.as_bool()).unwrap_or(false),
        };
        map.insert("isExpanded".into(), OptionValue::Bool(next));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    fn series(name: &str, data: &[f64]) -> OptionValue {
        obj(vec![
            ("name", OptionValue::String(name.into())),
            (
                "data",
                OptionValue::Array(data.iter().copied().map(OptionValue::Number).collect()),
            ),
        ])
    }

    #[test]
    fn merge_deep_object() {
        let base = OptionValue::Object({
            let mut m = IndexMap::new();
            m.insert("a".into(), OptionValue::Number(1.0));
            m.insert(
                "nested".into(),
                OptionValue::Object({
                    let mut n = IndexMap::new();
                    n.insert("x".into(), OptionValue::Number(10.0));
                    n
                }),
            );
            m
        });
        let incoming = OptionValue::Object({
            let mut m = IndexMap::new();
            m.insert(
                "nested".into(),
                OptionValue::Object({
                    let mut n = IndexMap::new();
                    n.insert("y".into(), OptionValue::Number(20.0));
                    n
                }),
            );
            m
        });
        let merged = merge_option(&base, &incoming, MergeMode::default());
        assert_eq!(merged.get("a").and_then(|v| v.as_f64()), Some(1.0));
        assert_eq!(
            merged
                .get("nested")
                .and_then(|v| v.get("x"))
                .and_then(|v| v.as_f64()),
            Some(10.0)
        );
        assert_eq!(
            merged
                .get("nested")
                .and_then(|v| v.get("y"))
                .and_then(|v| v.as_f64()),
            Some(20.0)
        );
    }

    #[test]
    fn not_merge_replaces_entire_tree() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![
                ("title", OptionValue::String("a".into())),
                (
                    "series",
                    OptionValue::Array(vec![series("old", &[1.0, 2.0])]),
                ),
            ]),
            SetOptionFlags::default(),
        );
        model.apply(
            obj(vec![(
                "series",
                OptionValue::Array(vec![series("new", &[9.0])]),
            )]),
            SetOptionFlags {
                not_merge: true,
                ..Default::default()
            },
        );
        assert!(model.root().get("title").is_none());
        assert_eq!(
            model
                .root()
                .get("series")
                .and_then(|v| v.as_array())
                .map(|a| a.len()),
            Some(1)
        );
        assert_eq!(
            model
                .root()
                .get("series")
                .and_then(|v| v.as_array())
                .and_then(|a| a[0].get("name"))
                .and_then(|v| v.as_str()),
            Some("new")
        );
    }

    #[test]
    fn not_merge_field_on_option_is_not_a_flag() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![
                ("title", OptionValue::String("keep".into())),
                (
                    "series",
                    OptionValue::Array(vec![series("old", &[1.0])]),
                ),
            ]),
            SetOptionFlags::default(),
        );
        model.apply(
            obj(vec![
                ("notMerge", OptionValue::Bool(true)),
                (
                    "series",
                    OptionValue::Array(vec![series("patched", &[2.0])]),
                ),
            ]),
            SetOptionFlags::default(),
        );
        assert_eq!(
            model.root().get("title").and_then(|v| v.as_str()),
            Some("keep")
        );
        assert_eq!(
            model.root().get("notMerge").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert_eq!(
            model
                .root()
                .get("series")
                .and_then(|v| v.as_array())
                .and_then(|a| a[0].get("name"))
                .and_then(|v| v.as_str()),
            Some("patched")
        );
    }

    #[test]
    fn replace_merge_replaces_top_level_key() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![(
                "series",
                OptionValue::Array(vec![series("a", &[1.0]), series("b", &[2.0])]),
            )]),
            SetOptionFlags::default(),
        );
        model.apply(
            obj(vec![(
                "series",
                OptionValue::Array(vec![series("only", &[3.0])]),
            )]),
            SetOptionFlags {
                replace_merge: vec!["series".into()],
                ..Default::default()
            },
        );
        let series = model.root().get("series").and_then(|v| v.as_array()).unwrap();
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].get("name").and_then(|v| v.as_str()), Some("only"));
    }

    #[test]
    fn without_replace_merge_series_merges_by_index() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![(
                "series",
                OptionValue::Array(vec![series("a", &[1.0]), series("b", &[2.0])]),
            )]),
            SetOptionFlags::default(),
        );
        model.apply(
            obj(vec![(
                "series",
                OptionValue::Array(vec![series("patched", &[9.0])]),
            )]),
            SetOptionFlags::default(),
        );
        let series = model.root().get("series").and_then(|v| v.as_array()).unwrap();
        assert_eq!(series.len(), 2);
        assert_eq!(
            series[0].get("name").and_then(|v| v.as_str()),
            Some("patched")
        );
        assert_eq!(series[1].get("name").and_then(|v| v.as_str()), Some("b"));
    }

    #[test]
    fn append_series_data_extends_array() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![(
                "series",
                OptionValue::Array(vec![series("a", &[1.0])]),
            )]),
            SetOptionFlags::default(),
        );
        model
            .append_series_data(
                0,
                OptionValue::Array(vec![OptionValue::Number(2.0), OptionValue::Number(3.0)]),
            )
            .unwrap();
        let data = model
            .root()
            .get("series")
            .and_then(|v| v.as_array())
            .and_then(|a| a[0].get("data"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(data[2].as_f64(), Some(3.0));
    }

    #[test]
    fn apply_theme_fills_defaults_under_option() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![("color", OptionValue::Array(vec![OptionValue::String("#f00".into())]))]),
            SetOptionFlags::default(),
        );
        model.apply_theme(obj(vec![
            ("color", OptionValue::Array(vec![OptionValue::String("#00f".into())])),
            ("backgroundColor", OptionValue::String("#fff".into())),
        ]));
        assert_eq!(
            model
                .root()
                .get("color")
                .and_then(|v| v.as_array())
                .and_then(|a| a[0].as_str()),
            Some("#f00")
        );
        assert_eq!(
            model.root().get("backgroundColor").and_then(|v| v.as_str()),
            Some("#fff")
        );
    }

    #[test]
    fn timeline_effective_root_merges_options() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![
                (
                    "baseOption",
                    obj(vec![("title", obj(vec![("text", OptionValue::String("base".into()))]))]),
                ),
                (
                    "options",
                    OptionValue::Array(vec![
                        obj(vec![("title", obj(vec![("text", OptionValue::String("A".into()))]))]),
                        obj(vec![("title", obj(vec![("text", OptionValue::String("B".into()))]))]),
                    ]),
                ),
                (
                    "timeline",
                    obj(vec![("currentIndex", OptionValue::Number(1.0))]),
                ),
            ]),
            SetOptionFlags::default(),
        );
        let root = model.effective_root(1, 400.0, 300.0);
        assert_eq!(
            root.get("title")
                .and_then(|t| t.get("text"))
                .and_then(|v| v.as_str()),
            Some("B")
        );
        model.set_timeline_index(0);
        let idx = model
            .root()
            .get("timeline")
            .and_then(|t| t.get("currentIndex"))
            .and_then(|v| v.as_f64());
        assert_eq!(idx, Some(0.0));
    }

    #[test]
    fn media_effective_root_switches_by_width() {
        let mut model = OptionModel::new();
        model.apply(
            obj(vec![
                (
                    "title",
                    obj(vec![("text", OptionValue::String("base".into()))]),
                ),
                (
                    "media",
                    OptionValue::Array(vec![
                        obj(vec![
                            ("query", obj(vec![("minWidth", OptionValue::Number(700.0))])),
                            (
                                "option",
                                obj(vec![("title", obj(vec![("text", OptionValue::String("wide".into()))]))]),
                            ),
                        ]),
                        obj(vec![(
                            "option",
                            obj(vec![("title", obj(vec![("text", OptionValue::String("narrow".into()))]))]),
                        )]),
                    ]),
                ),
            ]),
            SetOptionFlags::default(),
        );
        let narrow = model.effective_root(0, 400.0, 300.0);
        assert_eq!(
            narrow
                .get("title")
                .and_then(|t| t.get("text"))
                .and_then(|v| v.as_str()),
            Some("narrow")
        );
        let wide = model.effective_root(0, 800.0, 300.0);
        assert_eq!(
            wide.get("title")
                .and_then(|t| t.get("text"))
                .and_then(|v| v.as_str()),
            Some("wide")
        );
    }
}
