//! `option.media`：按实例宽高匹配 query，叠到 base / timeline 之上。

use crate::option::{merge_option, MergeMode, OptionValue};

/// 从原始 option 抽出 base（去掉 options/media，或使用 `baseOption`）。
pub fn extract_base_option(root: &OptionValue) -> OptionValue {
    if let Some(base) = root.get("baseOption") {
        let mut base = base.clone();
        if base.get("timeline").is_none() {
            if let Some(tl) = root.get("timeline") {
                if let Some(map) = base.as_object_mut() {
                    map.insert("timeline".into(), tl.clone());
                }
            }
        }
        return base;
    }
    let mut base = root.clone();
    if let Some(map) = base.as_object_mut() {
        map.shift_remove("options");
        map.shift_remove("media");
    }
    base
}

pub fn parse_media_list(root: &OptionValue) -> (Vec<(OptionValue, OptionValue)>, Option<OptionValue>) {
    let Some(media) = root.get("media").and_then(|v| v.as_array()) else {
        return (Vec::new(), None);
    };
    let mut list = Vec::new();
    let mut default = None;
    for unit in media {
        let Some(option) = unit.get("option") else {
            continue;
        };
        if unit.get("query").is_some() {
            list.push((unit.get("query").cloned().unwrap_or(OptionValue::Null), option.clone()));
        } else if default.is_none() {
            default = Some(option.clone());
        }
    }
    (list, default)
}

/// `minWidth` / `maxWidth` / `width` / `height` / `aspectRatio` 及对应 min/max。
pub fn apply_media_query(query: &OptionValue, width: f64, height: f64) -> bool {
    let Some(map) = query.as_object() else {
        return false;
    };
    let aspect = if height.abs() < f64::EPSILON {
        0.0
    } else {
        width / height
    };
    for (key, expect) in map {
        let Some(expect) = expect.as_f64() else {
            continue;
        };
        let (op, attr) = split_query_key(key);
        let real = match attr.as_str() {
            "width" => width,
            "height" => height,
            "aspectratio" => aspect,
            _ => continue,
        };
        if !compare_query(real, expect, op) {
            return false;
        }
    }
    true
}

fn split_query_key(key: &str) -> (&'static str, String) {
    let lower = key.to_ascii_lowercase();
    if let Some(rest) = lower.strip_prefix("min") {
        ("min", rest.to_string())
    } else if let Some(rest) = lower.strip_prefix("max") {
        ("max", rest.to_string())
    } else {
        ("eq", lower)
    }
}

fn compare_query(real: f64, expect: f64, op: &str) -> bool {
    match op {
        "min" => real >= expect,
        "max" => real <= expect,
        _ => (real - expect).abs() < f64::EPSILON,
    }
}

/// 匹配到的 media option（后者优先）。无匹配时用无 query 的默认项。
pub fn matching_media_options(root: &OptionValue, width: f64, height: f64) -> Vec<OptionValue> {
    let (list, default) = parse_media_list(root);
    if list.is_empty() && default.is_none() {
        return Vec::new();
    }
    let mut matched = Vec::new();
    for (query, option) in &list {
        if apply_media_query(query, width, height) {
            matched.push(option.clone());
        }
    }
    if matched.is_empty() {
        if let Some(default) = default {
            matched.push(default);
        }
    }
    matched
}

pub fn apply_media(base: OptionValue, root: &OptionValue, width: f64, height: f64) -> OptionValue {
    let overlays = matching_media_options(root, width, height);
    let mut out = base;
    for overlay in overlays {
        out = merge_option(
            &out,
            &overlay,
            MergeMode {
                replace_merge: Vec::new(),
            },
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn media_query_min_width() {
        let q = obj(vec![("minWidth", OptionValue::Number(600.0))]);
        assert!(!apply_media_query(&q, 400.0, 300.0));
        assert!(apply_media_query(&q, 600.0, 300.0));
        assert!(apply_media_query(&q, 800.0, 300.0));
    }

    #[test]
    fn media_picks_default_when_none_match() {
        let root = obj(vec![(
            "media",
            OptionValue::Array(vec![
                obj(vec![
                    ("query", obj(vec![("minWidth", OptionValue::Number(900.0))])),
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
        )]);
        let matched = matching_media_options(&root, 400.0, 300.0);
        assert_eq!(
            matched[0]
                .get("title")
                .and_then(|t| t.get("text"))
                .and_then(|v| v.as_str()),
            Some("narrow")
        );
    }
}
