//! visual 阶段：构造 CallbackDataParams 并调用 JsCallback

use wasm_bindgen::JsValue;

use crate::bridge::{
    build_data_params, default_series_color, resolve_color, resolve_formatter, resolve_symbol_size,
    DataParamsInput,
};
use crate::data::{numeric_or_time, data_point_from_parsed};
use crate::model::{DataPoint, GlobalModel, SeriesType};
use crate::option::{option_value_to_js, OptionModel, OptionValue};
use crate::utils::format_axis_number;

pub struct VisualContext<'a> {
    option: &'a OptionModel,
    model: &'a GlobalModel,
}

impl<'a> VisualContext<'a> {
    pub fn new(option: &'a OptionModel, model: &'a GlobalModel) -> Self {
        Self { option, model }
    }

    pub fn series_option(&self, series_index: usize) -> Option<&OptionValue> {
        self.option
            .root()
            .get("series")?
            .as_array()?
            .get(series_index)
    }

    pub fn data_params(&self, series_index: usize, data_index: usize) -> JsValue {
        let series = &self.model.series[series_index];
        let point = &series.data[data_index];
        let name = point
            .name
            .as_deref()
            .or_else(|| {
                self.model
                    .x_axis_at(series.x_axis_index)
                    .category_data
                    .get(data_index)
                    .map(|s| s.as_str())
            })
            .unwrap_or("");
        let color = default_series_color(series_index);
        let data = option_value_to_js(&point.raw).unwrap_or(JsValue::NULL);
        let value = params_value(&point.raw, point.value, &data);
        let percent = if series.series_type == SeriesType::Pie {
            let total: f64 = series.data.iter().map(|p| p.value).sum();
            Some(if total > 0.0 {
                point.value / total * 100.0
            } else {
                0.0
            })
        } else {
            None
        };
        build_data_params(DataParamsInput {
            series_index: series_index as u32,
            data_index: data_index as u32,
            series_name: &series.name,
            series_type: series.series_type.as_str(),
            name,
            value: &value,
            data: &data,
            color: Some(color),
            percent,
        })
    }

    pub fn resolve_item_color(&self, series_index: usize, data_index: usize) -> String {
        let fallback = default_series_color(series_index).to_string();
        let params = self.data_params(series_index, data_index);
        let series_opt = self.series_option(series_index);
        let color_val = series_opt
            .and_then(|s| s.get("itemStyle"))
            .and_then(|is| is.get("color"))
            .or_else(|| {
                series_opt
                    .and_then(|s| s.get("lineStyle"))
                    .and_then(|ls| ls.get("color"))
            });
        resolve_color(color_val, &params, &fallback)
    }

    pub fn label_visible(&self, series_index: usize) -> bool {
        let series = self.model.series.get(series_index);
        let default_show = series
            .map(|s| s.series_type == SeriesType::Pie)
            .unwrap_or(false);
        self.series_option(series_index)
            .and_then(|s| s.get("label"))
            .and_then(|l| l.get("show"))
            .and_then(|v| v.as_bool())
            .unwrap_or(default_show)
    }

    pub fn resolve_label(&self, series_index: usize, data_index: usize) -> Option<String> {
        if !self.label_visible(series_index) {
            return None;
        }
        let params = self.data_params(series_index, data_index);
        let formatter = self
            .series_option(series_index)
            .and_then(|s| s.get("label"))
            .and_then(|l| l.get("formatter"));
        if formatter.is_some() {
            return resolve_formatter(formatter, &params);
        }
        let series = self.model.series.get(series_index)?;
        let point = series.data.get(data_index)?;
        if series.series_type == SeriesType::Pie {
            Some(
                point
                    .name
                    .clone()
                    .unwrap_or_else(|| format_axis_number(point.value)),
            )
        } else {
            Some(format_axis_number(point.value))
        }
    }

    pub fn resolve_symbol(&self, series_index: usize) -> String {
        if let Some(s) = self
            .series_option(series_index)
            .and_then(|s| s.get("symbol"))
            .and_then(|v| v.as_str())
        {
            return s.to_string();
        }
        match self.model.series.get(series_index).map(|s| s.series_type) {
            Some(SeriesType::Line) => "emptyCircle".into(),
            _ => "circle".into(),
        }
    }

    pub fn show_symbol(&self, series_index: usize) -> bool {
        self.series_option(series_index)
            .and_then(|s| s.get("showSymbol"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    pub fn resolve_symbol_size_of(&self, series_index: usize, data_index: usize) -> f64 {
        let default = match self.model.series.get(series_index).map(|s| s.series_type) {
            Some(SeriesType::Scatter) => 10.0,
            Some(SeriesType::Line) => 6.0,
            _ => 6.0,
        };
        let params = self.data_params(series_index, data_index);
        let point = &self.model.series[series_index].data[data_index];
        let raw_value = option_value_to_js(&point.raw).unwrap_or_else(|_| JsValue::from(point.value));
        let size_opt = self
            .series_option(series_index)
            .and_then(|s| s.get("symbolSize"));
        resolve_symbol_size(size_opt, &raw_value, &params, default)
    }

    pub fn resolve_tooltip(&self, series_index: usize, data_index: usize) -> Option<String> {
        let params = self.data_params(series_index, data_index);
        let root_tip = self.option.root().get("tooltip").and_then(|t| t.get("formatter"));
        if let Some(text) = resolve_formatter(root_tip, &params) {
            return Some(text);
        }
        let series_tip = self
            .series_option(series_index)
            .and_then(|s| s.get("tooltip"))
            .and_then(|t| t.get("formatter"));
        resolve_formatter(series_tip, &params).or_else(|| {
            let s = &self.model.series[series_index];
            let p = &s.data[data_index];
            Some(format!("{}: {}", s.name, p.value))
        })
    }
}

fn params_value(raw: &OptionValue, numeric: f64, data_js: &JsValue) -> JsValue {
    match raw {
        OptionValue::Array(_) => data_js.clone(),
        OptionValue::Object(obj) => obj
            .get("value")
            .and_then(|v| option_value_to_js(v).ok())
            .unwrap_or_else(|| JsValue::from(numeric)),
        _ => JsValue::from(numeric),
    }
}

/// 从 series.data 解析数值点
pub fn parse_series_data(data: Option<&OptionValue>) -> Vec<DataPoint> {
    let Some(OptionValue::Array(arr)) = data else {
        return Vec::new();
    };
    arr.iter()
        .enumerate()
        .filter_map(|(i, item)| parse_data_point(item, i))
        .collect()
}

fn parse_data_point(item: &OptionValue, index: usize) -> Option<DataPoint> {
    match item {
        OptionValue::Number(n) => Some(data_point_from_parsed(*n, None, None, index, item.clone())),
        OptionValue::Object(obj) => {
            let value_field = obj.get("value");
            let (value, x_value) = match value_field {
                Some(OptionValue::Number(n)) => (*n, None),
                Some(OptionValue::Array(pair)) => {
                    let y = pair
                        .get(1)
                        .and_then(numeric_or_time)
                        .or_else(|| pair.first().and_then(numeric_or_time))?;
                    let x = pair.first().and_then(numeric_or_time);
                    (y, x)
                }
                _ => return None,
            };
            let name = obj.get("name").and_then(|v| v.as_str()).map(str::to_string);
            Some(data_point_from_parsed(value, x_value, name, index, item.clone()))
        }
        OptionValue::Array(pair) if pair.len() >= 2 => {
            let x = numeric_or_time(&pair[0]);
            let value = numeric_or_time(&pair[1])?;
            let name = pair[0].as_str().map(str::to_string);
            Some(data_point_from_parsed(value, x, name, index, item.clone()))
        }
        _ => None,
    }
}

pub fn series_type_of(opt: &OptionValue) -> &str {
    opt.get("type").and_then(|v| v.as_str()).unwrap_or("line")
}

pub fn series_name_of(opt: &OptionValue, index: usize) -> String {
    opt.get("name")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| format!("series{index}"))
}
