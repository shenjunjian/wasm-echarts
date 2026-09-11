//! legend：折线用「线 + 圆」，其它系列用色块；默认贴底（ECharts 6）

use rust_zrender::{
    ChildRef, CircleShape, DisplayableProps, FillStrokeStyle, LineShape, Path, PathStyle, Shape,
    TextAlign, TextBaseline, ZRenderer,
};

use crate::bridge::default_series_color;
use crate::chart::layout::{
    add_rect, add_text, component_ec, layout_origin, parse_orient, parse_padding, HIT_LEGEND,
};
use crate::chart::text_opt::{option_component, parse_chart_text_style};
use crate::interaction::InteractionState;
use crate::model::{GlobalModel, SeriesType};
use crate::option::{OptionModel, OptionValue};

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
        .unwrap_or(25.0);
    let item_height = legend
        .get("itemHeight")
        .and_then(|v| v.as_f64())
        .unwrap_or(14.0);
    let item_gap = legend
        .get("itemGap")
        .and_then(|v| v.as_f64())
        .unwrap_or(8.0);
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
    let row_h = item_height.max(style.font_size as f64) + 4.0;
    let legend_h = if horizontal {
        pad.top + pad.bottom + row_h
    } else {
        pad.top + pad.bottom + (row_h + item_gap) * names.len() as f64 - item_gap
    };
    let (origin_x, origin_y) = layout_origin(
        legend,
        width,
        height,
        total_w,
        legend_h,
        ((width - total_w).max(0.0) / 2.0).max(0.0),
        (height - 15.0 - legend_h).max(0.0),
    );

    let mut x = origin_x + pad.left;
    let mut y = origin_y + pad.top;
    for (i, name) in names.iter().enumerate() {
        let series_index = model.series.iter().position(|s| s.name == *name).unwrap_or(i);
        let series_type = model
            .series
            .get(series_index)
            .map(|s| s.series_type)
            .unwrap_or(SeriesType::Other);
        let on = interaction.is_name_selected(name);
        let color = if on {
            default_series_color(series_index).to_string()
        } else {
            "#ccc".to_string()
        };
        let fill = style.fill.clone();
        let text_fill = if on { fill } else { "#ccc".to_string() };
        let hit_w = item_width + 6.0 + widths[i];
        let hit_h = row_h;
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
        if series_type == SeriesType::Line {
            add_line_legend_icon(zr, group, x, y, item_width, item_height, &color);
        } else {
            add_rect(
                zr,
                group,
                x,
                y + (hit_h - item_height).max(0.0) / 2.0,
                item_width,
                item_height,
                FillStrokeStyle::color(&color),
                FillStrokeStyle::none(),
                0.0,
                10.1,
                None,
            );
        }
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

fn add_line_legend_icon(
    zr: &mut ZRenderer,
    group: usize,
    x: f64,
    y: f64,
    item_width: f64,
    item_height: f64,
    color: &str,
) {
    let cy = y + item_height / 2.0;
    let line = zr.storage.create_path(
        Path::new(
            Shape::Line(LineShape {
                x1: x,
                y1: cy,
                x2: x + item_width,
                y2: cy,
                percent: 1.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::none(),
                stroke: FillStrokeStyle::color(color),
                line_width: 2.0,
                ..PathStyle::stroke_default()
            },
        )
        .with_displayable(DisplayableProps {
            z: 10.1,
            ..Default::default()
        }),
    );
    zr.storage.group_add_child(group, ChildRef::Path(line));
    let size = item_height * 0.8;
    let circle = zr.storage.create_path(
        Path::new(
            Shape::Circle(CircleShape {
                cx: x + item_width / 2.0,
                cy,
                r: size / 2.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::color(color),
                stroke: FillStrokeStyle::none(),
                ..Default::default()
            },
        )
        .with_displayable(DisplayableProps {
            z: 10.15,
            ..Default::default()
        }),
    );
    zr.storage.group_add_child(group, ChildRef::Path(circle));
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

    #[test]
    fn line_legend_uses_line_and_circle_at_bottom() {
        let (option, model) = sample_option("horizontal");
        let interaction = InteractionState::from_option(&option);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        render_legend(&mut zr, group, &model, &option, &interaction);
        let has_line = zr
            .storage
            .paths()
            .iter()
            .any(|p| matches!(p.shape, Shape::Line(_)));
        let has_circle = zr
            .storage
            .paths()
            .iter()
            .any(|p| matches!(p.shape, Shape::Circle(_)));
        assert!(has_line, "line series legend should draw a line");
        assert!(has_circle, "line series legend should draw a circle");
        let min_y = zr
            .storage
            .paths()
            .iter()
            .filter_map(|p| match &p.shape {
                Shape::Line(s) => Some(s.y1.min(s.y2)),
                Shape::Circle(s) => Some(s.cy - s.r),
                _ => None,
            })
            .fold(f64::INFINITY, f64::min);
        assert!(
            min_y > 200.0,
            "default legend should sit near the bottom, got y={}",
            min_y
        );
    }
}
