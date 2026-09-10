//! 内置 filter / sort；外部 registerTransform

use std::cell::RefCell;
use std::collections::HashMap;

use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::data::source::DataTable;
use crate::option::{option_value_to_js, parse_option_value, OptionValue};

thread_local! {
    static CUSTOM_TRANSFORMS: RefCell<HashMap<String, Function>> = RefCell::new(HashMap::new());
}

pub fn register_transform(ty: String, func: Function) {
    let key = normalize_transform_type(&ty);
    CUSTOM_TRANSFORMS.with(|m| {
        m.borrow_mut().insert(key, func);
    });
}

pub fn normalize_transform_type(ty: &str) -> String {
    let t = ty.trim();
    if t.contains(':') {
        t.to_string()
    } else {
        format!("echarts:{t}")
    }
}

pub fn apply_transforms(mut table: DataTable, transform: Option<&OptionValue>) -> DataTable {
    let Some(transform) = transform else {
        return table;
    };
    let list: Vec<&OptionValue> = match transform {
        OptionValue::Array(arr) => arr.iter().collect(),
        other => vec![other],
    };
    for item in list {
        table = apply_one_transform(table, item);
    }
    table
}

fn apply_one_transform(table: DataTable, spec: &OptionValue) -> DataTable {
    let ty = spec
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let key = normalize_transform_type(ty);
    let config = spec.get("config");
    match key.as_str() {
        "echarts:filter" | "filter" => filter_table(&table, config),
        "echarts:sort" | "sort" => sort_table(&table, config),
        _ => apply_custom(&key, table, config),
    }
}

fn apply_custom(ty: &str, table: DataTable, config: Option<&OptionValue>) -> DataTable {
    let func = CUSTOM_TRANSFORMS.with(|m| m.borrow().get(ty).cloned());
    let Some(func) = func else {
        return table;
    };
    let Ok(upstream) = table_to_js(&table) else {
        return table;
    };
    let params = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&params, &JsValue::from_str("upstream"), &upstream);
    let config_js = config
        .and_then(|c| option_value_to_js(c).ok())
        .unwrap_or(JsValue::NULL);
    let _ = js_sys::Reflect::set(&params, &JsValue::from_str("config"), &config_js);
    match func.call1(&JsValue::UNDEFINED, &params.into()) {
        Ok(result) => parse_transform_result(&table, &result),
        Err(_) => table,
    }
}

fn table_to_js(table: &DataTable) -> Result<JsValue, JsValue> {
    let obj = js_sys::Object::new();
    let dims = js_sys::Array::new_with_length(table.dimensions.len() as u32);
    for (i, d) in table.dimensions.iter().enumerate() {
        dims.set(i as u32, JsValue::from_str(d));
    }
    let data = js_sys::Array::new_with_length(table.rows.len() as u32);
    for (i, row) in table.rows.iter().enumerate() {
        data.set(i as u32, option_value_to_js(&OptionValue::Array(row.clone()))?);
    }
    js_sys::Reflect::set(&obj, &JsValue::from_str("dimensions"), &dims)?;
    js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &data)?;
    js_sys::Reflect::set(&obj, &JsValue::from_str("source"), &data)?;
    Ok(obj.into())
}

fn parse_transform_result(fallback: &DataTable, result: &JsValue) -> DataTable {
    let Ok(parsed) = parse_option_value(result) else {
        return fallback.clone();
    };
    let (dims, rows) = match &parsed {
        OptionValue::Object(map) => {
            let dims = map.get("dimensions").cloned();
            let data = map
                .get("data")
                .or_else(|| map.get("source"))
                .cloned()
                .unwrap_or(OptionValue::Array(Vec::new()));
            (dims, data)
        }
        OptionValue::Array(_) => (None, parsed),
        _ => return fallback.clone(),
    };
    let mut out = crate::data::source::parse_dataset_table(&{
        let mut m = indexmap::IndexMap::new();
        if let Some(d) = dims {
            m.insert("dimensions".into(), d);
        } else {
            m.insert(
                "dimensions".into(),
                OptionValue::Array(
                    fallback
                        .dimensions
                        .iter()
                        .cloned()
                        .map(OptionValue::String)
                        .collect(),
                ),
            );
        }
        m.insert("source".into(), rows);
        m.insert("sourceHeader".into(), OptionValue::Bool(false));
        OptionValue::Object(m)
    });
    out.id = fallback.id.clone();
    if out.dimensions.is_empty() {
        out.dimensions = fallback.dimensions.clone();
    }
    out
}

fn filter_table(table: &DataTable, config: Option<&OptionValue>) -> DataTable {
    let Some(config) = config else {
        return DataTable {
            id: table.id.clone(),
            dimensions: table.dimensions.clone(),
            rows: Vec::new(),
        };
    };
    let rows = table
        .rows
        .iter()
        .filter(|row| eval_condition(config, table, row))
        .cloned()
        .collect();
    DataTable {
        id: table.id.clone(),
        dimensions: table.dimensions.clone(),
        rows,
    }
}

fn eval_condition(expr: &OptionValue, table: &DataTable, row: &[OptionValue]) -> bool {
    if let Some(and_list) = expr.get("and").and_then(|v| v.as_array()) {
        return and_list.iter().all(|c| eval_condition(c, table, row));
    }
    if let Some(or_list) = expr.get("or").and_then(|v| v.as_array()) {
        return or_list.iter().any(|c| eval_condition(c, table, row));
    }
    if let Some(not) = expr.get("not") {
        return !eval_condition(not, table, row);
    }
    let Some(dim) = expr.get("dimension") else {
        return false;
    };
    let Some(idx) = table.dim_index_loose(dim) else {
        return false;
    };
    let cell = row.get(idx).unwrap_or(&OptionValue::Null);
    let mut ok = true;
    let mut has_op = false;
    for (key, rhs) in expr.as_object().into_iter().flatten() {
        if key == "dimension" {
            continue;
        }
        has_op = true;
        ok = ok && compare_cell(cell, key, rhs);
    }
    has_op && ok
}

fn compare_cell(cell: &OptionValue, op: &str, rhs: &OptionValue) -> bool {
    match op {
        "=" | "eq" | "==" => cells_equal(cell, rhs),
        "!=" | "ne" | "<>" => !cells_equal(cell, rhs),
        ">" | "gt" => cmp_num(cell, rhs).map(|c| c.is_gt()).unwrap_or(false),
        ">=" | "gte" => cmp_num(cell, rhs).map(|c| c.is_ge()).unwrap_or(false),
        "<" | "lt" => cmp_num(cell, rhs).map(|c| c.is_lt()).unwrap_or(false),
        "<=" | "lte" => cmp_num(cell, rhs).map(|c| c.is_le()).unwrap_or(false),
        _ => true,
    }
}

fn cells_equal(a: &OptionValue, b: &OptionValue) -> bool {
    match (a, b) {
        (OptionValue::Number(x), OptionValue::Number(y)) => (x - y).abs() < f64::EPSILON,
        (OptionValue::String(x), OptionValue::String(y)) => x == y,
        (OptionValue::Bool(x), OptionValue::Bool(y)) => x == y,
        (OptionValue::Null, OptionValue::Null) => true,
        (OptionValue::Number(x), OptionValue::String(y)) => y.parse::<f64>().ok() == Some(*x),
        (OptionValue::String(x), OptionValue::Number(y)) => x.parse::<f64>().ok() == Some(*y),
        _ => cell_text(a) == cell_text(b),
    }
}

fn cmp_num(cell: &OptionValue, rhs: &OptionValue) -> Option<std::cmp::Ordering> {
    let a = cell_number(cell)?;
    let b = cell_number(rhs)?;
    a.partial_cmp(&b)
}

pub fn cell_number(v: &OptionValue) -> Option<f64> {
    match v {
        OptionValue::Number(n) if n.is_finite() => Some(*n),
        OptionValue::String(s) => {
            s.parse().ok().or_else(|| crate::data::time::parse_time_string(s))
        }
        OptionValue::Bool(true) => Some(1.0),
        OptionValue::Bool(false) => Some(0.0),
        _ => None,
    }
}

pub fn cell_text(v: &OptionValue) -> String {
    match v {
        OptionValue::String(s) => s.clone(),
        OptionValue::Number(n) => {
            if (n - n.round()).abs() < f64::EPSILON {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        OptionValue::Bool(b) => b.to_string(),
        OptionValue::Null => String::new(),
        _ => String::new(),
    }
}

fn sort_table(table: &DataTable, config: Option<&OptionValue>) -> DataTable {
    let Some(config) = config else {
        return table.clone();
    };
    let orders: Vec<&OptionValue> = match config {
        OptionValue::Array(arr) => arr.iter().collect(),
        other => vec![other],
    };
    if orders.is_empty() {
        return table.clone();
    }
    let mut rows = table.rows.clone();
    rows.sort_by(|a, b| {
        for spec in &orders {
            let Some(dim) = spec.get("dimension") else {
                continue;
            };
            let Some(idx) = table.dim_index_loose(dim) else {
                continue;
            };
            let va = a.get(idx).unwrap_or(&OptionValue::Null);
            let vb = b.get(idx).unwrap_or(&OptionValue::Null);
            let mut ord = compare_sort_cells(va, vb);
            let desc = spec
                .get("order")
                .and_then(|v| v.as_str())
                == Some("desc");
            if desc {
                ord = ord.reverse();
            }
            if ord != std::cmp::Ordering::Equal {
                return ord;
            }
        }
        std::cmp::Ordering::Equal
    });
    DataTable {
        id: table.id.clone(),
        dimensions: table.dimensions.clone(),
        rows,
    }
}

fn compare_sort_cells(a: &OptionValue, b: &OptionValue) -> std::cmp::Ordering {
    match (cell_number(a), cell_number(b)) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => cell_text(a).cmp(&cell_text(b)),
    }
}

pub fn resolve_datasets(root: &OptionValue) -> Vec<DataTable> {
    let specs: Vec<&OptionValue> = match root.get("dataset") {
        Some(OptionValue::Array(arr)) => arr.iter().collect(),
        Some(v) => vec![v],
        None => Vec::new(),
    };
    if specs.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<Option<DataTable>> = vec![None; specs.len()];
    for _ in 0..specs.len().saturating_add(1) {
        let mut progress = false;
        for (i, spec) in specs.iter().enumerate() {
            if out[i].is_some() {
                continue;
            }
            let has_transform = spec.get("transform").is_some()
                || spec.get("fromTransformResult").is_some();
            if has_transform {
                let upstream_idx = spec
                    .get("fromDatasetIndex")
                    .and_then(|v| v.as_f64())
                    .map(|n| n as usize)
                    .or_else(|| {
                        spec.get("fromDatasetId")
                            .and_then(|v| v.as_str())
                            .and_then(|id| {
                                specs.iter().position(|s| {
                                    s.get("id").and_then(|v| v.as_str()) == Some(id)
                                })
                            })
                    })
                    .unwrap_or(0);
                if upstream_idx == i {
                    if spec.get("source").is_none() {
                        continue;
                    }
                    let mut table = crate::data::source::parse_dataset_table(spec);
                    table = apply_transforms(table, spec.get("transform"));
                    out[i] = Some(table);
                    progress = true;
                    continue;
                }
                if let Some(up) = out.get(upstream_idx).and_then(|t| t.as_ref()) {
                    let mut table = up.clone();
                    if let Some(id) = spec.get("id").and_then(|v| v.as_str()) {
                        table.id = Some(id.to_string());
                    }
                    table = apply_transforms(table, spec.get("transform"));
                    out[i] = Some(table);
                    progress = true;
                }
            } else {
                out[i] = Some(crate::data::source::parse_dataset_table(spec));
                progress = true;
            }
        }
        if !progress {
            break;
        }
    }
    out.into_iter()
        .map(|t| t.unwrap_or_else(DataTable::empty))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::source::parse_dataset_table;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    fn table() -> DataTable {
        parse_dataset_table(&obj(vec![(
            "source",
            OptionValue::Array(vec![
                OptionValue::Array(vec![
                    OptionValue::String("Year".into()),
                    OptionValue::String("Score".into()),
                ]),
                OptionValue::Array(vec![OptionValue::Number(2010.0), OptionValue::Number(1.0)]),
                OptionValue::Array(vec![OptionValue::Number(1950.0), OptionValue::Number(2.0)]),
                OptionValue::Array(vec![OptionValue::Number(2020.0), OptionValue::Number(3.0)]),
            ]),
        )]))
    }

    #[test]
    fn filter_gt() {
        let t = table();
        let cfg = obj(vec![
            ("dimension", OptionValue::String("Year".into())),
            (">", OptionValue::Number(1950.0)),
        ]);
        let out = filter_table(&t, Some(&cfg));
        assert_eq!(out.row_count(), 2);
    }

    #[test]
    fn sort_desc() {
        let t = table();
        let cfg = obj(vec![
            ("dimension", OptionValue::String("Year".into())),
            ("order", OptionValue::String("desc".into())),
        ]);
        let out = sort_table(&t, Some(&cfg));
        assert_eq!(out.cell(0, 0).and_then(|v| v.as_f64()), Some(2020.0));
        assert_eq!(out.cell(2, 0).and_then(|v| v.as_f64()), Some(1950.0));
    }
}
