//! 从 dataset 按 encode / seriesLayoutBy 抽出系列点

use crate::data::source::{DataTable, SeriesLayoutBy};
use crate::data::time::parse_time_value;
use crate::data::transform::{cell_number, cell_text};
use crate::model::{DataPoint, SeriesType};
use crate::option::OptionValue;

#[derive(Debug, Clone, Default)]
pub struct EncodeMap {
    pub x: Option<usize>,
    pub y: Option<usize>,
    pub item_name: Option<usize>,
    pub value: Option<usize>,
}

#[derive(Debug, Default)]
pub struct DatasetCursor {
    pub category_way_dim: usize,
    pub value_way_dim: usize,
}

pub fn parse_encode(table: &DataTable, encode: Option<&OptionValue>) -> EncodeMap {
    let Some(encode) = encode else {
        return EncodeMap::default();
    };
    EncodeMap {
        x: first_dim(table, encode.get("x")),
        y: first_dim(table, encode.get("y")),
        item_name: first_dim(table, encode.get("itemName")),
        value: first_dim(table, encode.get("value")),
    }
}

fn first_dim(table: &DataTable, spec: Option<&OptionValue>) -> Option<usize> {
    let spec = spec?;
    match spec {
        OptionValue::Array(arr) => arr.first().and_then(|v| table.dim_index_loose(v)),
        other => table.dim_index_loose(other),
    }
}

pub fn default_encode(
    table: &DataTable,
    series_type: SeriesType,
    x_is_category: bool,
    cursor: &mut DatasetCursor,
) -> EncodeMap {
    if table.dim_count() == 0 {
        return EncodeMap::default();
    }
    if series_type == SeriesType::Pie {
        return pie_default_encode(table);
    }
    if x_is_category {
        if cursor.category_way_dim == 0 {
            cursor.category_way_dim = 1;
        }
        let y = if cursor.category_way_dim < table.dim_count() {
            let y = cursor.category_way_dim;
            cursor.category_way_dim += 1;
            Some(y)
        } else {
            table.dim_count().checked_sub(1)
        };
        EncodeMap {
            x: Some(0),
            y,
            item_name: Some(0),
            value: y,
        }
    } else {
        let start = cursor.value_way_dim;
        if start + 1 < table.dim_count() {
            cursor.value_way_dim = start + 2;
            EncodeMap {
                x: Some(start),
                y: Some(start + 1),
                item_name: None,
                value: Some(start + 1),
            }
        } else if table.dim_count() >= 2 {
            EncodeMap {
                x: Some(0),
                y: Some(1),
                item_name: None,
                value: Some(1),
            }
        } else {
            EncodeMap {
                x: Some(0),
                y: Some(0),
                item_name: None,
                value: Some(0),
            }
        }
    }
}

fn pie_default_encode(table: &DataTable) -> EncodeMap {
    let mut name = 0usize;
    let mut value = 0usize;
    for (i, _) in table.dimensions.iter().enumerate() {
        let numeric = (0..table.row_count().min(5))
            .filter_map(|r| table.cell(r, i))
            .filter(|c| cell_number(c).is_some())
            .count();
        if numeric >= 1 {
            value = i;
            break;
        }
    }
    for (i, name_hint) in table.dimensions.iter().enumerate() {
        if name_hint == "name" || i != value {
            name = i;
            if name_hint == "name" {
                break;
            }
        }
    }
    EncodeMap {
        x: None,
        y: Some(value),
        item_name: Some(name),
        value: Some(value),
    }
}

pub fn table_to_points(
    table: &DataTable,
    encode: &EncodeMap,
    layout: SeriesLayoutBy,
    series_slot: usize,
) -> Vec<DataPoint> {
    match layout {
        SeriesLayoutBy::Column => column_points(table, encode),
        SeriesLayoutBy::Row => row_points(table, encode, series_slot),
    }
}

fn column_points(table: &DataTable, encode: &EncodeMap) -> Vec<DataPoint> {
    let mut out = Vec::with_capacity(table.row_count());
    for (i, row) in table.rows.iter().enumerate() {
        if let Some(p) = row_to_point(table, encode, row, i) {
            out.push(p);
        }
    }
    out
}

fn row_points(table: &DataTable, encode: &EncodeMap, series_slot: usize) -> Vec<DataPoint> {
    if table.row_count() == 0 || table.dim_count() == 0 {
        return Vec::new();
    }
    let row_idx = series_slot.min(table.row_count().saturating_sub(1));
    let row = &table.rows[row_idx];
    let name_dim = encode.item_name.or(Some(0)).unwrap_or(0);
    let start = if table.dim_count() > 1 { 1 } else { 0 };
    let mut out = Vec::new();
    for dim in start..table.dim_count() {
        let cell = row.get(dim).unwrap_or(&OptionValue::Null);
        let value = cell_number(cell).unwrap_or(f64::NAN);
        let name = Some(table.dimensions[dim].clone());
        let series_name = row
            .get(name_dim)
            .map(cell_text)
            .filter(|s| !s.is_empty());
        out.push(DataPoint {
            value,
            x_value: Some((dim - start) as f64),
            name: name.or(series_name),
            raw_index: dim - start,
            raw: cell.clone(),
            stack_base: 0.0,
            stacked_value: value,
        });
    }
    out
}

fn row_to_point(
    table: &DataTable,
    encode: &EncodeMap,
    row: &[OptionValue],
    index: usize,
) -> Option<DataPoint> {
    let y_dim = encode.y.or(encode.value).unwrap_or_else(|| {
        if table.dim_count() > 1 {
            1
        } else {
            0
        }
    });
    let y_cell = row.get(y_dim).unwrap_or(&OptionValue::Null);
    let value = cell_number(y_cell).unwrap_or(f64::NAN);
    let x_value = encode.x.and_then(|d| {
        let cell = row.get(d)?;
        cell_number(cell).or_else(|| parse_time_value(cell))
    });
    let name = encode
        .item_name
        .or(encode.x)
        .and_then(|d| row.get(d))
        .map(cell_text)
        .filter(|s| !s.is_empty());
    let raw = if table.dim_count() == 1 {
        y_cell.clone()
    } else {
        OptionValue::Array(row.to_vec())
    };
    Some(DataPoint {
        value,
        x_value,
        name,
        raw_index: index,
        raw,
        stack_base: 0.0,
        stacked_value: value,
    })
}

pub fn series_name_from_table(table: &DataTable, encode: &EncodeMap) -> Option<String> {
    encode
        .y
        .or(encode.value)
        .and_then(|i| table.dimensions.get(i).cloned())
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

    #[test]
    fn category_way_default_columns() {
        let table = parse_dataset_table(&obj(vec![(
            "source",
            OptionValue::Array(vec![
                OptionValue::Array(vec![
                    OptionValue::String("product".into()),
                    OptionValue::String("2015".into()),
                    OptionValue::String("2016".into()),
                ]),
                OptionValue::Array(vec![
                    OptionValue::String("A".into()),
                    OptionValue::Number(10.0),
                    OptionValue::Number(20.0),
                ]),
            ]),
        )]));
        let mut cursor = DatasetCursor::default();
        let e0 = default_encode(&table, SeriesType::Bar, true, &mut cursor);
        let e1 = default_encode(&table, SeriesType::Bar, true, &mut cursor);
        let p0 = table_to_points(&table, &e0, SeriesLayoutBy::Column, 0);
        let p1 = table_to_points(&table, &e1, SeriesLayoutBy::Column, 1);
        assert_eq!(p0[0].value, 10.0);
        assert_eq!(p1[0].value, 20.0);
        assert_eq!(p0[0].name.as_deref(), Some("A"));
    }
}
