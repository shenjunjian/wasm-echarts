//! option 深合并（function 按 ECharts 规则：新值覆盖旧值）

use super::OptionValue;

/// setOption 合并模式
#[derive(Debug, Clone, Default)]
pub struct MergeMode {
    /// replaceMerge 指定的组件 id 列表（简化：按顶层 key 名匹配）
    pub replace_merge: Vec<String>,
}

pub fn merge_option(base: &OptionValue, incoming: &OptionValue, mode: MergeMode) -> OptionValue {
    match (base, incoming) {
        (_, OptionValue::Null) => base.clone(),
        (OptionValue::Object(base_map), OptionValue::Object(in_map)) => {
            let mut out = base_map.clone();
            for (key, val) in in_map {
                if mode.replace_merge.iter().any(|r| r == key) {
                    out.insert(key.clone(), val.clone());
                    continue;
                }
                match out.get(key) {
                    Some(existing) => {
                        out.insert(key.clone(), merge_option(existing, val, mode.clone()));
                    }
                    None => {
                        out.insert(key.clone(), val.clone());
                    }
                }
            }
            OptionValue::Object(out)
        }
        // series 等数组：按 index 合并对象元素，否则替换
        (OptionValue::Array(base_arr), OptionValue::Array(in_arr)) => {
            merge_arrays(base_arr, in_arr, mode)
        }
        (_, incoming) => incoming.clone(),
    }
}

fn merge_arrays(base: &[OptionValue], incoming: &[OptionValue], mode: MergeMode) -> OptionValue {
    let mut out: Vec<OptionValue> = base.to_vec();
    for (i, item) in incoming.iter().enumerate() {
        if i < out.len() {
            out[i] = merge_option(&out[i], item, mode.clone());
        } else {
            out.push(item.clone());
        }
    }
    OptionValue::Array(out)
}
