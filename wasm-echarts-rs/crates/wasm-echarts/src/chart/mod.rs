mod axis;
mod bar;
mod label;
mod legend;
mod line;
mod text_opt;
mod title;
mod style;
mod symbol;
#[cfg(feature = "chart-pie")]
mod pie;
#[cfg(feature = "chart-scatter")]
mod scatter;

#[cfg(feature = "chart-bar")]
pub use bar::render_bar_series;
#[cfg(feature = "chart-line")]
pub use line::render_line_series;
#[cfg(feature = "chart-pie")]
pub use pie::render_pie_series;
#[cfg(feature = "chart-scatter")]
pub use scatter::render_scatter_series;

use rust_zrender::{
    ChildRef, FillStrokeStyle, LineShape, Path, PathStyle, Shape, ZRenderer,
};

use crate::coord::Cartesian2D;
use crate::interaction::InteractionState;
use crate::model::{GlobalModel, SeriesType};
use crate::option::OptionModel;
use crate::visual::VisualContext;

pub fn render_components(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    let visual = VisualContext::new(option, model);

    if model.has_cartesian_series() {
        for (i, _) in model.grids.iter().enumerate() {
            render_grid_frame(zr, group, model.grid_at(i));
            let x_idx = model
                .x_axes
                .iter()
                .position(|a| a.grid_index == i)
                .unwrap_or(0);
            let y_idx = model
                .y_axes
                .iter()
                .position(|a| a.grid_index == i)
                .unwrap_or(0);
            let coord = Cartesian2D::for_axes(model, x_idx, y_idx);
            render_split_lines(zr, group, model, &coord);
        }
        axis::render_axis_labels(zr, group, model, option);
    }

    for series in &model.series {
        let coord = Cartesian2D::for_series(model, series);
        let (zoom_start, zoom_end) = model.visible_category_range_of(series.x_axis_index);
        match series.series_type {
            #[cfg(feature = "chart-line")]
            SeriesType::Line => {
                render_line_series(zr, group, model, &coord, &visual, series, zoom_start, zoom_end);
            }
            #[cfg(feature = "chart-bar")]
            SeriesType::Bar => {
                render_bar_series(zr, group, model, &coord, &visual, series, zoom_start, zoom_end);
            }
            #[cfg(feature = "chart-pie")]
            SeriesType::Pie => {
                render_pie_series(zr, group, model, &visual, series, interaction);
            }
            #[cfg(feature = "chart-scatter")]
            SeriesType::Scatter => {
                render_scatter_series(zr, group, model, &coord, &visual, series);
            }
            _ => {}
        }
    }

    if model.has_cartesian_series() {
        let coord = Cartesian2D::new(model);
        render_axis_pointer(zr, group, model, &coord, interaction);
    }

    title::render_title(zr, group, model, option);
    legend::render_legend(zr, group, model, option);
}

fn render_grid_frame(zr: &mut ZRenderer, group: usize, g: crate::model::GridRect) {
    use rust_zrender::{
        FillStrokeStyle, LineShape, Path, PathStyle, Shape,
    };
    let y_axis = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1: g.x,
            y1: g.y,
            x2: g.x,
            y2: g.y + g.height,
            percent: 1.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::none(),
            stroke: FillStrokeStyle::color("#333"),
            line_width: 1.0,
            ..Default::default()
        },
    ));
    let x_axis = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1: g.x,
            y1: g.y + g.height,
            x2: g.x + g.width,
            y2: g.y + g.height,
            percent: 1.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::none(),
            stroke: FillStrokeStyle::color("#333"),
            line_width: 1.0,
            ..Default::default()
        },
    ));
    zr.storage.group_add_child(group, ChildRef::Path(y_axis));
    zr.storage.group_add_child(group, ChildRef::Path(x_axis));
}

fn render_split_lines(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &Cartesian2D,
) {
    use rust_zrender::{
        FillStrokeStyle, LineShape, Path, PathStyle, Shape,
    };

    let g = coord.grid();
    let split_count = 5;
    let ymin = coord.y_axis().value_min();
    let ymax = coord.y_axis().value_max();
    let span = ymax - ymin;

    for i in 0..=split_count {
        let value = ymin + span * i as f64 / split_count as f64;
        let (_, y) = coord.value_to_point(coord.x_axis().value_min(), value);
        let line = zr.storage.create_path(Path::new(
            Shape::Line(LineShape {
                x1: g.x,
                y1: y,
                x2: g.x + g.width,
                y2: y,
                percent: 1.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::none(),
                stroke: FillStrokeStyle::color("#eee"),
                line_width: 1.0,
                ..Default::default()
            },
        ));
        zr.storage.group_add_child(group, ChildRef::Path(line));
    }
}

fn render_axis_pointer(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &Cartesian2D,
    interaction: &InteractionState,
) {
    if !interaction.axis_pointer_enabled {
        return;
    }
    let (px, py) = match (interaction.pointer_x, interaction.pointer_y) {
        (Some(x), Some(y)) if model.grid().contains(x, y) => (x, y),
        _ => return,
    };

    let snap_x = interaction
        .axis_pointer_label(model, px, py)
        .map(|(_, _, x)| x)
        .unwrap_or(px);

    let g = model.grid();
    let line = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1: snap_x,
            y1: g.y,
            x2: snap_x,
            y2: g.y + g.height,
            percent: 1.0,
        }),
        PathStyle {
            fill: FillStrokeStyle::none(),
            stroke: FillStrokeStyle::color("#aaa"),
            line_width: 1.0,
            line_dash: Some(vec![4.0, 4.0]),
            ..Default::default()
        },
    ));
    zr.storage.group_add_child(group, ChildRef::Path(line));

    let _ = py;
    let _ = coord;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::option::{OptionModel, OptionValue, SetOptionFlags};
    use indexmap::IndexMap;
    use rust_zrender::ZRenderer;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn title_legend_and_axis_name_keep_font_family() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "title",
                    obj(vec![
                        ("text", OptionValue::String("主标题".into())),
                        ("subtext", OptionValue::String("副标题".into())),
                        ("left", OptionValue::String("center".into())),
                        (
                            "textStyle",
                            obj(vec![(
                                "fontFamily",
                                OptionValue::String("Microsoft YaHei".into()),
                            )]),
                        ),
                        (
                            "subtextStyle",
                            obj(vec![("fontFamily", OptionValue::String("SimSun".into()))]),
                        ),
                    ]),
                ),
                (
                    "legend",
                    obj(vec![(
                        "textStyle",
                        obj(vec![("fontFamily", OptionValue::String("KaiTi".into()))]),
                    )]),
                ),
                (
                    "xAxis",
                    obj(vec![
                        ("type", OptionValue::String("category".into())),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::String("Q1".into())]),
                        ),
                        ("name", OptionValue::String("季度".into())),
                        (
                            "nameTextStyle",
                            obj(vec![("fontFamily", OptionValue::String("SimSun".into()))]),
                        ),
                        (
                            "axisLabel",
                            obj(vec![(
                                "fontFamily",
                                OptionValue::String("Microsoft YaHei".into()),
                            )]),
                        ),
                    ]),
                ),
                (
                    "yAxis",
                    obj(vec![
                        ("type", OptionValue::String("value".into())),
                        ("name", OptionValue::String("单位：万件".into())),
                        (
                            "nameTextStyle",
                            obj(vec![("fontFamily", OptionValue::String("KaiTi".into()))]),
                        ),
                        (
                            "axisLabel",
                            obj(vec![
                                ("fontFamily", OptionValue::String("SimSun".into())),
                                ("formatter", OptionValue::String("{value} 万".into())),
                            ]),
                        ),
                    ]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        ("name", OptionValue::String("华东".into())),
                        (
                            "data",
                            OptionValue::Array(vec![OptionValue::Number(12.0)]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );

        let model = crate::model::GlobalModel::from_option(&option, 480, 360);
        let mut zr = ZRenderer::new(480, 360).unwrap();
        let group = zr.storage.create_group();
        title::render_title(&mut zr, group, &model, &option);
        legend::render_legend(&mut zr, group, &model, &option);
        axis::render_axis_labels(&mut zr, group, &model, &option);
        let texts: Vec<(String, String)> = zr
            .storage
            .texts()
            .iter()
            .map(|t| (t.content.clone(), t.style.font_family.clone()))
            .collect();

        assert!(
            texts
                .iter()
                .any(|(c, f)| c == "主标题" && f == "Microsoft YaHei"),
            "title font: {:?}",
            texts
        );
        assert!(
            texts.iter().any(|(c, f)| c == "副标题" && f == "SimSun"),
            "subtext font: {:?}",
            texts
        );
        assert!(
            texts.iter().any(|(c, f)| c == "华东" && f == "KaiTi"),
            "legend font: {:?}",
            texts
        );
        assert!(
            texts.iter().any(|(c, f)| c == "季度" && f == "SimSun"),
            "xAxis name font: {:?}",
            texts
        );
        assert!(
            texts
                .iter()
                .any(|(c, f)| c == "单位：万件" && f == "KaiTi"),
            "yAxis name font: {:?}",
            texts
        );
        assert!(
            texts
                .iter()
                .any(|(c, f)| c.contains('万') && f == "SimSun"),
            "y axisLabel font: {:?}",
            texts
        );
    }
}
