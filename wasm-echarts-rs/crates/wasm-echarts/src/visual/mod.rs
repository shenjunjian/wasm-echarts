//! visual 阶段：构造 CallbackDataParams 并调用 JsCallback

use wasm_bindgen::JsValue;

use crate::bridge::{build_data_params, default_series_color, resolve_color, resolve_formatter};
use crate::model::{DataPoint, GlobalModel};
use crate::option::{OptionModel, OptionValue};

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
                self.model.x_categories.get(data_index).map(|s| s.as_str())
            })
            .unwrap_or("");
        // 使用默认色板，避免与 resolve_item_color 互相递归
        let color = default_series_color(series_index);
        build_data_params(
            series_index as u32,
            data_index as u32,
            &series.name,
            name,
            &JsValue::from(point.value),
            Some(color),
        )
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

    pub fn resolve_label(&self, series_index: usize, data_index: usize) -> Option<String> {
        let params = self.data_params(series_index, data_index);
        let formatter = self
            .series_option(series_index)
            .and_then(|s| s.get("label"))
            .and_then(|l| l.get("formatter"));
        resolve_formatter(formatter, &params)
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
        OptionValue::Number(n) => Some(DataPoint {
            value: *n,
            name: None,
            raw_index: index,
        }),
        OptionValue::Object(obj) => {
            let value = obj.get("value").and_then(|v| v.as_f64())?;
            let name = obj.get("name").and_then(|v| v.as_str()).map(str::to_string);
            Some(DataPoint {
                value,
                name,
                raw_index: index,
            })
        }
        OptionValue::Array(pair) if pair.len() >= 2 => {
            let value = pair[1].as_f64()?;
            let name = pair[0].as_str().map(str::to_string);
            Some(DataPoint {
                value,
                name,
                raw_index: index,
            })
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
