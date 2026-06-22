//! ECharts option 解析与合并（保留 JsFunction，不用 serde 整包反序列化）

mod merge;
mod parse;

pub use merge::{merge_option, MergeMode};
pub use parse::parse_option_value;

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

    /// 解析 JsValue 并按 ECharts 规则合并
    pub fn set_option(&mut self, option: &wasm_bindgen::JsValue) -> Result<(), wasm_bindgen::JsValue> {
        let incoming = parse_option_value(option)?;
        let not_merge = incoming
            .get("notMerge")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if not_merge || self.is_empty() {
            self.root = strip_meta_keys(incoming);
        } else {
            self.root = merge_option(&self.root, &incoming, MergeMode::default());
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.root = OptionValue::Object(IndexMap::new());
    }
}

/// 移除 setOption 元字段（notMerge / lazyUpdate / replaceMerge 等）
fn strip_meta_keys(value: OptionValue) -> OptionValue {
    const META_KEYS: &[&str] = &["notMerge", "lazyUpdate", "replaceMerge", "transition"];
    match value {
        OptionValue::Object(mut map) => {
            for key in META_KEYS {
                map.shift_remove(*key);
            }
            OptionValue::Object(map)
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
