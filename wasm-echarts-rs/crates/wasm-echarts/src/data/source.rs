//! dataset.source 收成列对齐的表

use crate::option::OptionValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeriesLayoutBy {
    Column,
    Row,
}

impl SeriesLayoutBy {
    pub fn from_option(value: Option<&OptionValue>) -> Self {
        match value.and_then(|v| v.as_str()) {
            Some("row") => SeriesLayoutBy::Row,
            _ => SeriesLayoutBy::Column,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataTable {
    pub id: Option<String>,
    pub dimensions: Vec<String>,
    pub rows: Vec<Vec<OptionValue>>,
}

impl DataTable {
    pub fn empty() -> Self {
        Self {
            id: None,
            dimensions: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn dim_count(&self) -> usize {
        self.dimensions.len()
    }

    pub fn dim_index(&self, spec: &OptionValue) -> Option<usize> {
        if let Some(n) = spec.as_f64() {
            let i = n as i32;
            if i >= 0 && (i as usize) < self.dimensions.len() {
                return Some(i as usize);
            }
            return None;
        }
        let name = spec.as_str()?;
        self.dimensions.iter().position(|d| d == name)
    }

    pub fn dim_index_loose(&self, spec: &OptionValue) -> Option<usize> {
        self.dim_index(spec).or_else(|| {
            spec.as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .and_then(|n| {
                    let i = n as usize;
                    (i < self.dimensions.len()).then_some(i)
                })
        })
    }

    pub fn cell(&self, row: usize, dim: usize) -> Option<&OptionValue> {
        self.rows.get(row)?.get(dim)
    }
}

pub fn parse_dataset_table(dataset: &OptionValue) -> DataTable {
    let id = dataset
        .get("id")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let dim_opt = dataset.get("dimensions");
    let header_opt = dataset.get("sourceHeader");
    let source = dataset.get("source");
    let mut table = match source {
        Some(v) => parse_source(v, dim_opt, header_opt),
        None => DataTable::empty(),
    };
    table.id = id;
    table
}

fn parse_source(
    source: &OptionValue,
    dimensions: Option<&OptionValue>,
    source_header: Option<&OptionValue>,
) -> DataTable {
    match source {
        OptionValue::Object(map) => parse_keyed_columns(map, dimensions),
        OptionValue::Array(arr) => {
            if arr.is_empty() {
                return table_from_dims(dimensions, Vec::new());
            }
            match &arr[0] {
                OptionValue::Object(_) => parse_object_rows(arr, dimensions),
                OptionValue::Array(_) => parse_array_rows(arr, dimensions, source_header),
                _ => {
                    let rows = arr.iter().map(|v| vec![v.clone()]).collect();
                    table_from_dims(dimensions.or(Some(&dummy_dims(1))), rows)
                }
            }
        }
        _ => DataTable::empty(),
    }
}

fn dummy_dims(n: usize) -> OptionValue {
    OptionValue::Array(
        (0..n)
            .map(|i| OptionValue::String(format!("dim{i}")))
            .collect(),
    )
}

fn parse_dimension_names(dimensions: Option<&OptionValue>, fallback: usize) -> Vec<String> {
    if let Some(arr) = dimensions.and_then(|v| v.as_array()) {
        return arr
            .iter()
            .enumerate()
            .map(|(i, d)| match d {
                OptionValue::String(s) => s.clone(),
                OptionValue::Object(map) => map
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("dim{i}")),
                OptionValue::Number(n) => n.to_string(),
                _ => format!("dim{i}"),
            })
            .collect();
    }
    (0..fallback).map(|i| format!("dim{i}")).collect()
}

fn table_from_dims(dimensions: Option<&OptionValue>, rows: Vec<Vec<OptionValue>>) -> DataTable {
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let names = parse_dimension_names(dimensions, width);
    let width = names.len().max(width);
    let rows = pad_rows(rows, width);
    DataTable {
        id: None,
        dimensions: {
            let mut names = names;
            while names.len() < width {
                names.push(format!("dim{}", names.len()));
            }
            names
        },
        rows,
    }
}

fn pad_rows(mut rows: Vec<Vec<OptionValue>>, width: usize) -> Vec<Vec<OptionValue>> {
    for row in &mut rows {
        while row.len() < width {
            row.push(OptionValue::Null);
        }
        if row.len() > width {
            row.truncate(width);
        }
    }
    rows
}

fn parse_keyed_columns(
    map: &indexmap::IndexMap<String, OptionValue>,
    dimensions: Option<&OptionValue>,
) -> DataTable {
    let names = if dimensions.is_some() {
        parse_dimension_names(dimensions, 0)
    } else {
        map.keys().cloned().collect()
    };
    let height = names
        .iter()
        .filter_map(|n| map.get(n).and_then(|v| v.as_array()).map(|a| a.len()))
        .max()
        .unwrap_or(0);
    let mut rows = Vec::with_capacity(height);
    for i in 0..height {
        let mut row = Vec::with_capacity(names.len());
        for name in &names {
            let cell = map
                .get(name)
                .and_then(|v| v.as_array())
                .and_then(|a| a.get(i))
                .cloned()
                .unwrap_or(OptionValue::Null);
            row.push(cell);
        }
        rows.push(row);
    }
    DataTable {
        id: None,
        dimensions: names,
        rows,
    }
}

fn parse_object_rows(arr: &[OptionValue], dimensions: Option<&OptionValue>) -> DataTable {
    let names = if dimensions.is_some() {
        parse_dimension_names(dimensions, 0)
    } else {
        let mut names = Vec::new();
        for item in arr {
            if let Some(map) = item.as_object() {
                for k in map.keys() {
                    if !names.iter().any(|n| n == k) {
                        names.push(k.clone());
                    }
                }
            }
        }
        names
    };
    let rows = arr
        .iter()
        .map(|item| {
            names
                .iter()
                .map(|n| {
                    item.get(n).cloned().unwrap_or(OptionValue::Null)
                })
                .collect()
        })
        .collect();
    DataTable {
        id: None,
        dimensions: names,
        rows,
    }
}

fn parse_array_rows(
    arr: &[OptionValue],
    dimensions: Option<&OptionValue>,
    source_header: Option<&OptionValue>,
) -> DataTable {
    let as_rows: Vec<Vec<OptionValue>> = arr
        .iter()
        .map(|item| match item {
            OptionValue::Array(cells) => cells.clone(),
            other => vec![other.clone()],
        })
        .collect();
    let header_rows = header_row_count(source_header, &as_rows, dimensions.is_some());
    if header_rows > 0 && !as_rows.is_empty() {
        let header = &as_rows[0];
        let names = if dimensions.is_some() {
            parse_dimension_names(dimensions, header.len())
        } else {
            header
                .iter()
                .enumerate()
                .map(|(i, c)| match c {
                    OptionValue::String(s) if !s.is_empty() => s.clone(),
                    OptionValue::Number(n) => n.to_string(),
                    _ => format!("dim{i}"),
                })
                .collect()
        };
        let data_rows = as_rows.into_iter().skip(header_rows).collect();
        let width = names.len();
        DataTable {
            id: None,
            dimensions: names,
            rows: pad_rows(data_rows, width),
        }
    } else {
        table_from_dims(dimensions, as_rows)
    }
}

fn header_row_count(
    source_header: Option<&OptionValue>,
    rows: &[Vec<OptionValue>],
    has_explicit_dims: bool,
) -> usize {
    if let Some(v) = source_header {
        if let Some(b) = v.as_bool() {
            return if b { 1 } else { 0 };
        }
        if let Some(n) = v.as_f64() {
            return n.max(0.0) as usize;
        }
    }
    if has_explicit_dims {
        return 0;
    }
    if rows.len() < 2 {
        return 0;
    }
    let first_all_string = rows[0].iter().all(|c| matches!(c, OptionValue::String(_)));
    let rest_has_number = rows.iter().skip(1).any(|r| {
        r.iter()
            .any(|c| matches!(c, OptionValue::Number(_) | OptionValue::Null))
    });
    if first_all_string && rest_has_number {
        1
    } else {
        0
    }
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
    fn array_rows_with_header() {
        let source = OptionValue::Array(vec![
            OptionValue::Array(vec![
                OptionValue::String("product".into()),
                OptionValue::String("2015".into()),
            ]),
            OptionValue::Array(vec![
                OptionValue::String("Matcha".into()),
                OptionValue::Number(43.3),
            ]),
        ]);
        let table = parse_source(&source, None, None);
        assert_eq!(table.dimensions, vec!["product", "2015"]);
        assert_eq!(table.row_count(), 1);
        assert_eq!(table.cell(0, 1).and_then(|v| v.as_f64()), Some(43.3));
    }

    #[test]
    fn object_rows() {
        let source = OptionValue::Array(vec![obj(vec![
            ("product", OptionValue::String("Matcha".into())),
            ("2015", OptionValue::Number(43.3)),
        ])]);
        let table = parse_source(&source, None, None);
        assert!(table.dimensions.contains(&"product".to_string()));
        assert_eq!(table.row_count(), 1);
    }

    #[test]
    fn keyed_columns() {
        let source = obj(vec![
            (
                "product",
                OptionValue::Array(vec![OptionValue::String("A".into())]),
            ),
            (
                "2015",
                OptionValue::Array(vec![OptionValue::Number(1.0)]),
            ),
        ]);
        let table = parse_source(&source, None, None);
        assert_eq!(table.row_count(), 1);
        assert_eq!(table.dim_count(), 2);
    }
}
