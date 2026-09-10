//! legend：色块 + 系列名；`orient` / `selected`；点击筛选（命中 dataType=legend）

use rust_zrender::{
    FillStrokeStyle, TextAlign, TextBaseline, ZRenderer,
};

use crate::bridge::default_series_color;
use crate::chart::layout::{
    add_rect, add_text, component_ec, parse_orient, parse_padding, HIT_LEGEND,
};
use crate::chart::text_opt::{option_component, parse_chart_text_style};
use crate::interaction::InteractionState;
use crate::model::{GlobalModel, SeriesType};
use crate::option::{OptionModel, OptionValue};
use crate::utils::parse_percent;

pub fn render_legend(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    let Some(legend) = option_component(option.root(), "legend") else {
        return;
    };
    if legend.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }

    let names = legend_names(legend, model);
    if names.is_empty() {
        return;
    }

    let style = parse_chart_text_style(legend.get("textStyle"), "#333", 12.0, "sans-serif");
    let item_width = legend
        .get("itemWidth")
        .and_then(|v| v.as_f64())
        .unwrap_or(14.0);
    let item_height = legend
        .get("itemHeight")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let item_gap = legend
        .get("itemGap")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let pad = parse_padding(legend.get("padding"), 5.0);
    let horizontal = parse_orient(legend.get("orient"));
    let mut total_w = pad.left + pad.right;
    let mut widths = Vec::with_capacity(names.len());
    for name in &names {
        let w = rust_zrender::Text::estimate_text_width(name, style.font_size);
        widths.push(w);
        if horizontal {
            total_w += item_width + 6.0 + w + item_gap;
        } else {
            total_w = total_w.max(pad.left + pad.right + item_width + 6.0 + w);
        }
    }
    if horizontal {
        total_w -= item_gap;
    }

    let width = model.width as f64;
    let height = model.height as f64;
    let (origin_x, _) = parse_legend_left(legend.get("left"), width, total_w);
    let origin_y = parse_percent(legend.get("top"), height, 12.0);

    let mut x = origin_x + pad.left;
    let mut y = origin_y + pad.top;
    for (i, name) in names.iter().enumerate() {
        let on = interaction.is_name_selected(name);
        let color = if on {
            default_series_color(i).to_string()
        } else {
            "#ccc".to_string()
        };
        let fill = style.fill.clone();
        let text_fill = if on { fill } else { "#ccc".to_string() };
        let hit_w = item_width + 6.0 + widths[i];
        let hit_h = item_height.max(style.font_size as f64) + 4.0;
        add_rect(
            zr,
            group,
            x - 2.0,
            y - 2.0,
            hit_w + 4.0,
            hit_h,
            FillStrokeStyle::none(),
            FillStrokeStyle::none(),
            0.0,
            10.0,
            Some(component_ec(HIT_LEGEND, i as i32)),
        );
        add_rect(
            zr,
            group,
            x,
            y + 2.0,
            item_width,
            item_height,
            FillStrokeStyle::color(color),
            FillStrokeStyle::none(),
            0.0,
            10.1,
            None,
        );
        let mut ts = style.to_text_style(TextAlign::Left, TextBaseline::Top);
        ts.fill = text_fill;
        add_text(
            zr,
            group,
            name,
            x + item_width + 6.0,
            y,
            ts,
            10.1,
            false,
            Some(component_ec(HIT_LEGEND, i as i32)),
        );
        if horizontal {
            x += item_width + 6.0 + widths[i] + item_gap;
        } else {
            y += hit_h + item_gap;
        }
    }
}

pub fn legend_item_name(legend: &OptionValue, model: &GlobalModel, index: usize) -> Option<String> {
    legend_names(legend, model).get(index).cloned()
}

pub fn legend_names(legend: &OptionValue, model: &GlobalModel) -> Vec<String> {
    if let Some(data) = legend.get("data").and_then(|v| v.as_array()) {
        return data
            .iter()
            .filter_map(|item| match item {
                OptionValue::String(s) if !s.is_empty() => Some(s.clone()),
                OptionValue::Object(_) => item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                _ => None,
            })
            .collect();
    }
    let series_names: Vec<String> = model
        .series
        .iter()
        .map(|s| s.name.clone())
        .filter(|s| !s.is_empty())
        .collect();
    if !series_names.is_empty() {
        return series_names;
    }
    model
        .series
        .iter()
        .filter(|s| s.series_type == SeriesType::Pie)
        .flat_map(|s| {
            s.data
                .iter()
                .filter_map(|p| p.name.clone())
                .filter(|n| !n.is_empty())
        })
        .collect()
}

fn parse_legend_left(value: Option<&OptionValue>, width: f64, total: f64) -> (f64, rust_zrender::TextAlign) {
    match value {
        Some(OptionValue::String(s)) => match s.as_str() {
            "center" => ((width - total).max(0.0) / 2.0, rust_zrender::TextAlign::Left),
            "right" => ((width - total - 12.0).max(0.0), rust_zrender::TextAlign::Left),
            "left" => (12.0, rust_zrender::TextAlign::Left),
            _ => (parse_percent(value, width, 12.0), rust_zrender::TextAlign::Left),
        },
        Some(OptionValue::Number(n)) => (*n, rust_zrender::TextAlign::Left),
        _ => ((width - total).max(0.0) / 2.0, rust_zrender::TextAlign::Left),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::InteractionState;
    use crate::option::{OptionModel, SetOptionFlags};
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    fn sample_option(orient: &str) -> (OptionModel, GlobalModel) {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "legend",
                    obj(vec![
                        ("orient", OptionValue::String(orient.into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::String("A".into()),
                                OptionValue::String("B".into()),
                            ]),
                        ),
                    ]),
                ),
                (
                    "xAxis",
                    obj(vec![("type", OptionValue::String("category".into()))]),
                ),
                (
                    "yAxis",
                    obj(vec![("type", OptionValue::String("value".into()))]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![
                        obj(vec![
                            ("type", OptionValue::String("line".into())),
                            ("name", OptionValue::String("A".into())),
                            ("data", OptionValue::Array(vec![OptionValue::Number(1.0)])),
                        ]),
                        obj(vec![
                            ("type", OptionValue::String("line".into())),
                            ("name", OptionValue::String("B".into())),
                            ("data", OptionValue::Array(vec![OptionValue::Number(2.0)])),
                        ]),
                    ]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 400, 300);
        (option, model)
    }

    #[test]
    fn legend_hit_and_vertical_layout() {
        let (option, model) = sample_option("vertical");
        let interaction = InteractionState::from_option(&option);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        render_legend(&mut zr, group, &model, &option, &interaction);
        let hits = zr
            .storage
            .paths()
            .iter()
            .filter(|p| p.ec_data.data_type.as_deref() == Some(HIT_LEGEND))
            .count();
        assert!(hits >= 2, "legend hit targets: {}", hits);
        let legend = option.root().get("legend").unwrap();
        assert_eq!(legend_names(legend, &model), vec!["A".to_string(), "B".to_string()]);
    }
}
