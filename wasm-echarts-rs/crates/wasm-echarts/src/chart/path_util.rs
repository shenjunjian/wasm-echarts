//! 第 6 波图表共用：挂 Path / 从 raw 抽数值

use rust_zrender::{
    ChildRef, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle, PathStylePatch, Shape,
    STATE_EMPHASIS, STATE_SELECT, ZRenderer,
};

use crate::data::numeric_or_time;
use crate::model::DataPoint;
use crate::option::OptionValue;

pub fn add_ec_path(
    zr: &mut ZRenderer,
    group: usize,
    shape: Shape,
    style: PathStyle,
    series_index: usize,
    data_index: usize,
    z: f64,
    attach_states: bool,
) -> usize {
    let idx = zr.storage.create_path(
        Path::new(shape, style)
            .with_displayable(DisplayableProps {
                z,
                ..Default::default()
            })
            .with_ec_data(EcData::new(series_index as i32, data_index as i32)),
    );
    zr.storage.group_add_child(group, ChildRef::Path(idx));
    if attach_states {
        zr.set_path_state_style(
            idx,
            STATE_EMPHASIS,
            PathStylePatch {
                line_width: Some(2.0),
                ..Default::default()
            },
        );
        zr.set_path_state_style(
            idx,
            STATE_SELECT,
            PathStylePatch {
                stroke: Some(FillStrokeStyle::color("#333")),
                line_width: Some(2.0),
                ..Default::default()
            },
        );
    }
    idx
}

pub fn fill_stroke(fill: &str, stroke: &str, width: f32, opacity: f32) -> PathStyle {
    PathStyle {
        fill: if fill == "none" {
            FillStrokeStyle::none()
        } else {
            FillStrokeStyle::color(fill)
        },
        stroke: if stroke == "none" {
            FillStrokeStyle::none()
        } else {
            FillStrokeStyle::color(stroke)
        },
        line_width: width,
        opacity,
        ..Default::default()
    }
}

pub fn raw_numbers(raw: &OptionValue) -> Vec<f64> {
    let arr = match raw {
        OptionValue::Array(a) => a.as_slice(),
        OptionValue::Object(obj) => match obj.get("value") {
            Some(OptionValue::Array(a)) => a.as_slice(),
            Some(v) => {
                return numeric_or_time(v)
                    .or_else(|| v.as_f64())
                    .into_iter()
                    .collect()
            }
            None => return Vec::new(),
        },
        OptionValue::Number(n) => return vec![*n],
        _ => return Vec::new(),
    };
    arr.iter()
        .filter_map(|v| numeric_or_time(v).or_else(|| v.as_f64()))
        .collect()
}

pub fn item_name(point: &DataPoint, fallback_index: usize) -> String {
    point
        .name
        .clone()
        .or_else(|| {
            if let OptionValue::Object(obj) = &point.raw {
                obj.get("name")
                    .and_then(|v| v.as_str())
                    .map(str::to_string)
            } else {
                None
            }
        })
        .unwrap_or_else(|| fallback_index.to_string())
}

pub fn opt_f64(opt: Option<&OptionValue>, key: &str, default: f64) -> f64 {
    opt.and_then(|o| o.get(key))
        .and_then(|v| v.as_f64())
        .filter(|n| n.is_finite())
        .unwrap_or(default)
}

pub fn opt_str<'a>(opt: Option<&'a OptionValue>, key: &str) -> Option<&'a str> {
    opt.and_then(|o| o.get(key)).and_then(|v| v.as_str())
}
