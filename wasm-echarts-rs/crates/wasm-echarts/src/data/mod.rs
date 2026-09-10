//! 数据管线：dataset / transform / encode / stack / sampling / time

mod encode;
mod sampling;
mod source;
mod stack;
mod time;
mod transform;

pub use encode::{default_encode, parse_encode, series_name_from_table, table_to_points, DatasetCursor};
pub use sampling::{apply_sampling, Sampling};
pub use source::{DataTable, SeriesLayoutBy};
pub use stack::{apply_stack, StackStrategy};
pub use time::{format_time_label, parse_time_value, weekday_sun0};
pub use transform::{cell_number, cell_text, register_transform, resolve_datasets};

use crate::model::{option_index, parse_coord_kind, DataPoint, SeriesModel, SeriesType};
use crate::option::OptionValue;
use crate::visual::{parse_series_data, series_name_of, series_type_of};

pub fn build_series(
    root: &OptionValue,
    x_axes: &[crate::model::AxisModel],
    pixel_width_for: impl Fn(usize, usize) -> f64,
) -> Vec<SeriesModel> {
    let datasets = resolve_datasets(root);
    let series_opts: Vec<&OptionValue> = root
        .get("series")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().collect())
        .unwrap_or_default();

    let mut cursors: std::collections::HashMap<(usize, SeriesLayoutBy), DatasetCursor> =
        std::collections::HashMap::new();
    let mut row_slots: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

    let mut series: Vec<SeriesModel> = series_opts
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let st = SeriesType::from_str(series_type_of(s));
            let x_axis_index = s
                .get("xAxisIndex")
                .and_then(|v| v.as_f64())
                .map(|n| n.max(0.0) as usize)
                .unwrap_or(0);
            let y_axis_index = s
                .get("yAxisIndex")
                .and_then(|v| v.as_f64())
                .map(|n| n.max(0.0) as usize)
                .unwrap_or(0);
            let stack = s
                .get("stack")
                .and_then(|v| v.as_str())
                .filter(|v| !v.is_empty())
                .map(str::to_string);
            let stack_strategy = StackStrategy::from_option(s.get("stackStrategy").and_then(|v| v.as_str()));
            let sampling = Sampling::from_option(s.get("sampling").and_then(|v| v.as_str()));
            let layout = SeriesLayoutBy::from_option(s.get("seriesLayoutBy"));
            let coord_sys = parse_coord_kind(s, st.as_str());
            let polar_index = option_index(s, "polarIndex");
            let radar_index = option_index(s, "radarIndex");
            let geo_index = option_index(s, "geoIndex");
            let calendar_index = option_index(s, "calendarIndex");
            let single_axis_index = option_index(s, "singleAxisIndex");
            let parallel_index = option_index(s, "parallelIndex");
            let matrix_index = option_index(s, "matrixIndex");

            let mut data = if s.get("data").and_then(|v| v.as_array()).is_some() {
                parse_series_data(s.get("data"))
            } else if let Some(table) = pick_dataset(&datasets, s) {
                let x_is_category = x_axes
                    .get(x_axis_index)
                    .map(|a| a.axis_type.is_category())
                    .unwrap_or(true)
                    && st != SeriesType::Scatter
                    && st != SeriesType::Pie;
                let cursor = cursors.entry((table_index(&datasets, s), layout)).or_default();
                let encode = if s.get("encode").is_some() {
                    parse_encode(table, s.get("encode"))
                } else {
                    default_encode(table, st, x_is_category, cursor)
                };
                let slot = row_slots.entry(table_index(&datasets, s)).or_insert(0);
                let points = table_to_points(table, &encode, layout, *slot);
                if layout == SeriesLayoutBy::Row {
                    *slot += 1;
                }
                let name_from_dim = series_name_from_table(table, &encode);
                let mut model = SeriesModel {
                    index: i,
                    name: series_name_of(s, i),
                    series_type: st,
                    data: points,
                    x_axis_index,
                    y_axis_index,
                    coord_sys,
                    polar_index,
                    radar_index,
                    geo_index,
                    calendar_index,
                    single_axis_index,
                    parallel_index,
                    matrix_index,
                    stack,
                    stack_strategy,
                    sampling,
                };
                if model.name == format!("series{i}") {
                    if let Some(n) = name_from_dim {
                        model.name = n;
                    }
                }
                return model;
            } else {
                parse_series_data(s.get("data"))
            };

            for p in &mut data {
                p.stacked_value = p.value;
                p.stack_base = 0.0;
            }

            SeriesModel {
                index: i,
                name: series_name_of(s, i),
                series_type: st,
                data,
                x_axis_index,
                y_axis_index,
                coord_sys,
                polar_index,
                radar_index,
                geo_index,
                calendar_index,
                single_axis_index,
                parallel_index,
                matrix_index,
                stack,
                stack_strategy,
                sampling,
            }
        })
        .collect();

    apply_stack(&mut series);

    for s in &mut series {
        if s.sampling == Sampling::None {
            continue;
        }
        let width = pixel_width_for(s.x_axis_index, s.y_axis_index);
        s.data = apply_sampling(&s.data, s.sampling, width);
    }

    series
}

fn table_index(datasets: &[DataTable], series: &OptionValue) -> usize {
    if let Some(id) = series.get("datasetId").and_then(|v| v.as_str()) {
        if let Some(i) = datasets.iter().position(|d| d.id.as_deref() == Some(id)) {
            return i;
        }
    }
    series
        .get("datasetIndex")
        .and_then(|v| v.as_f64())
        .map(|n| n.max(0.0) as usize)
        .unwrap_or(0)
}

fn pick_dataset<'a>(datasets: &'a [DataTable], series: &'a OptionValue) -> Option<&'a DataTable> {
    if datasets.is_empty() {
        return None;
    }
    datasets.get(table_index(datasets, series))
}

pub fn numeric_or_time(value: &OptionValue) -> Option<f64> {
    cell_number(value).or_else(|| parse_time_value(value))
}

pub fn data_point_from_parsed(
    value: f64,
    x_value: Option<f64>,
    name: Option<String>,
    raw_index: usize,
    raw: OptionValue,
) -> DataPoint {
    DataPoint {
        value,
        x_value,
        name,
        raw_index,
        raw,
        stack_base: 0.0,
        stacked_value: value,
    }
}
