//! axisPointer 十字线 + 轴上 label；tooltip `trigger: 'axis'` 的文案由 instance 组装

use rust_zrender::{
    ChildRef, DisplayableProps, FillStrokeStyle, LineShape, Path, PathStyle, RectShape, Shape,
    TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_plain_label;
use crate::coord::Cartesian2D;
use crate::interaction::InteractionState;
use crate::model::GlobalModel;
use crate::option::OptionModel;
use crate::utils::format_axis_number;

pub fn render_axis_pointer(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    if !interaction.axis_pointer_enabled && !interaction.tooltip_trigger_axis {
        return;
    }
    let (px, py) = match (interaction.pointer_x, interaction.pointer_y) {
        (Some(x), Some(y)) if model.grid().contains(x, y) => (x, y),
        _ => return,
    };
    let info = interaction.axis_pointer_label(model, px, py);
    let (snap_x, snap_y, xlabel, ylabel) = if let Some((_, label, sx, sy)) = info {
        let coord = Cartesian2D::new(model);
        let yv = coord.pixel_to_y_value(sy);
        (sx, sy, label, format_axis_number(yv))
    } else {
        (px, py, String::new(), String::new())
    };

    let ty = option
        .root()
        .get("tooltip")
        .and_then(|t| t.get("axisPointer"))
        .and_then(|a| a.get("type"))
        .or_else(|| {
            option
                .root()
                .get("axisPointer")
                .and_then(|a| a.get("type"))
        })
        .and_then(|v| v.as_str())
        .unwrap_or("line");

    let g = model.grid();
    let color = "#aaa";
    add_dash_line(zr, group, snap_x, g.y, snap_x, g.y + g.height, color);
    if ty == "cross" || interaction.tooltip_trigger_axis {
        add_dash_line(zr, group, g.x, snap_y, g.x + g.width, snap_y, color);
    }

    if !xlabel.is_empty() {
        let tw = rust_zrender::Text::estimate_text_width(&xlabel, 11.0);
        add_label_box(
            zr,
            group,
            snap_x - tw / 2.0 - 3.0,
            g.y + g.height + 2.0,
            tw + 6.0,
            16.0,
        );
        add_plain_label(
            zr,
            group,
            xlabel,
            snap_x,
            g.y + g.height + 4.0,
            TextAlign::Center,
            TextBaseline::Top,
            "#fff",
            21.0,
        );
    }
    if ty == "cross" || interaction.tooltip_trigger_axis {
        let tw = rust_zrender::Text::estimate_text_width(&ylabel, 11.0);
        add_label_box(
            zr,
            group,
            g.x - tw - 10.0,
            snap_y - 8.0,
            tw + 8.0,
            16.0,
        );
        add_plain_label(
            zr,
            group,
            ylabel,
            g.x - 6.0,
            snap_y,
            TextAlign::Right,
            TextBaseline::Middle,
            "#fff",
            21.0,
        );
    }
}

fn add_dash_line(
    zr: &mut ZRenderer,
    group: usize,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: &str,
) {
    let line = zr.storage.create_path(
        Path::new(
            Shape::Line(LineShape {
                x1,
                y1,
                x2,
                y2,
                percent: 1.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::none(),
                stroke: FillStrokeStyle::color(color),
                line_width: 1.0,
                line_dash: Some(vec![4.0, 4.0]),
                ..PathStyle::stroke_default()
            },
        )
        .with_displayable(DisplayableProps {
            z: 20.0,
            ..Default::default()
        }),
    );
    zr.storage.group_add_child(group, ChildRef::Path(line));
}

fn add_label_box(zr: &mut ZRenderer, group: usize, x: f64, y: f64, w: f64, h: f64) {
    let rect = zr.storage.create_path(
        Path::new(
            Shape::Rect(RectShape {
                x,
                y,
                width: w,
                height: h,
                r: vec![2.0],
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#6e7079"),
                ..Default::default()
            },
        )
        .with_displayable(DisplayableProps {
            z: 20.5,
            ..Default::default()
        }),
    );
    zr.storage.group_add_child(group, ChildRef::Path(rect));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::InteractionState;
    use crate::model::GlobalModel;
    use crate::option::{OptionModel, OptionValue, SetOptionFlags};
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn cross_draws_two_lines() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "tooltip",
                    obj(vec![
                        ("trigger", OptionValue::String("axis".into())),
                        (
                            "axisPointer",
                            obj(vec![("type", OptionValue::String("cross".into()))]),
                        ),
                    ]),
                ),
                (
                    "xAxis",
                    obj(vec![
                        ("type", OptionValue::String("category".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::String("a".into()),
                                OptionValue::String("b".into()),
                            ]),
                        ),
                    ]),
                ),
                (
                    "yAxis",
                    obj(vec![("type", OptionValue::String("value".into()))]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::Number(1.0),
                                OptionValue::Number(2.0),
                            ]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let mut interaction = InteractionState::from_option(&option);
        let model = GlobalModel::from_option(&option, 400, 300);
        let g = model.grid();
        interaction.set_pointer(Some(g.x + g.width / 2.0), Some(g.y + g.height / 2.0));
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        render_axis_pointer(&mut zr, group, &model, &option, &interaction);
        let lines = zr
            .storage
            .paths()
            .iter()
            .filter(|p| matches!(p.shape, Shape::Line(_)))
            .count();
        assert!(lines >= 2, "cross lines {}", lines);
        assert!(zr.storage.texts().iter().any(|t| t.content == "a" || t.content == "b"));
    }
}
