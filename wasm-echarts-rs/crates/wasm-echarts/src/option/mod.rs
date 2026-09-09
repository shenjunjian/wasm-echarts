//! ECharts option 解析与合并（保留 JsFunction，不用 serde 整包反序列化）

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

    pub fn clear(&mut self) {
        self.root = OptionValue::Object(IndexMap::new());
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
}
