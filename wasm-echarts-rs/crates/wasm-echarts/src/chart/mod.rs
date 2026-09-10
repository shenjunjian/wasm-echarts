mod axis;
pub(crate) mod axis_pointer;
mod bar;
pub(crate) mod brush;
mod candlestick;
mod coords;
#[cfg(feature = "chart-custom")]
mod custom;
pub(crate) mod data_zoom;
mod effect_scatter;
mod funnel;
mod geo_charts;
mod gauge;
mod graphic;
mod heatmap;
mod hierarchy;
mod label;
pub(crate) mod layout;
pub(crate) mod legend;
mod line;
mod mark;
mod network;
mod parallel_series;
mod path_util;
mod pictorial;
mod radar;
#[cfg(feature = "chart-pie")]
mod pie;
mod scatter;
mod style;
mod symbol;
mod theme_river;
mod title;
pub(crate) mod text_opt;
pub(crate) mod thumbnail;
pub(crate) mod timeline;
pub(crate) mod toolbox;
pub(crate) mod visual_map;

#[cfg(feature = "chart-bar")]
pub use bar::render_bar_series;
#[cfg(feature = "chart-line")]
pub use line::render_line_series;
#[cfg(feature = "chart-pie")]
pub use pie::render_pie_series;
#[cfg(feature = "chart-scatter")]
pub use scatter::render_scatter_series;
#[cfg(feature = "chart-custom")]
pub use custom::CustomSeriesApi;

pub use layout::{
    HIT_DATA_ZOOM, HIT_LEGEND, HIT_THUMBNAIL, HIT_TIMELINE, HIT_TOOLBOX, HIT_VISUAL_MAP,
};
pub use toolbox::{next_magic_type, TB_BRUSH, TB_DATA_VIEW, TB_DATA_ZOOM, TB_MAGIC, TB_RESTORE, TB_SAVE};

use crate::coord::{Cartesian2D, SeriesCoord};
use crate::interaction::InteractionState;
use crate::model::{GlobalModel, SeriesType};
use crate::option::OptionModel;
use crate::visual::VisualContext;

use rust_zrender::{
    ChildRef, FillStrokeStyle, LineShape, Path, PathStyle, Shape, ZRenderer,
};

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

    coords::render_extra_coords(zr, group, model, option);

    for series in &model.series {
        if !interaction.is_name_selected(&series.name) {
            continue;
        }
        let coord = SeriesCoord::for_series(model, series);
        let (zoom_start, zoom_end) = if series.coord_sys.is_cartesian() {
            model.visible_category_range_of(series.x_axis_index)
        } else {
            (0, series.data.len())
        };
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
            #[cfg(feature = "chart-radar")]
            SeriesType::Radar => {
                radar::render_radar_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-gauge")]
            SeriesType::Gauge => {
                gauge::render_gauge_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-candlestick")]
            SeriesType::Candlestick => {
                candlestick::render_candlestick_series(zr, group, model, &coord, &visual, series);
            }
            #[cfg(feature = "chart-boxplot")]
            SeriesType::Boxplot => {
                candlestick::render_boxplot_series(zr, group, model, &coord, &visual, series);
            }
            #[cfg(feature = "chart-heatmap")]
            SeriesType::Heatmap => {
                heatmap::render_heatmap_series(zr, group, model, &coord, &visual, series);
            }
            #[cfg(feature = "chart-pictorial")]
            SeriesType::PictorialBar => {
                pictorial::render_pictorial_bar_series(zr, group, model, &coord, &visual, series);
            }
            #[cfg(feature = "chart-effect-scatter")]
            SeriesType::EffectScatter => {
                effect_scatter::render_effect_scatter_series(zr, group, model, &coord, &visual, series);
            }
            #[cfg(feature = "chart-funnel")]
            SeriesType::Funnel => {
                funnel::render_funnel_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-tree")]
            SeriesType::Tree => {
                hierarchy::render_tree_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-treemap")]
            SeriesType::Treemap => {
                hierarchy::render_treemap_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-sunburst")]
            SeriesType::Sunburst => {
                hierarchy::render_sunburst_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-graph")]
            SeriesType::Graph => {
                network::render_graph_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-chord")]
            SeriesType::Chord => {
                network::render_chord_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-sankey")]
            SeriesType::Sankey => {
                network::render_sankey_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-theme-river")]
            SeriesType::ThemeRiver => {
                theme_river::render_theme_river_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-map")]
            SeriesType::Map => {
                geo_charts::render_map_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-lines")]
            SeriesType::Lines => {
                geo_charts::render_lines_series(zr, group, model, &coord, &visual, series);
            }
            #[cfg(feature = "chart-parallel")]
            SeriesType::Parallel => {
                parallel_series::render_parallel_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-custom")]
            SeriesType::Custom => {
                custom::render_custom_series(zr, group, model, &coord, &visual, series);
            }
            _ => {}
        }
        mark::render_marks(zr, group, model, option, &visual, series);
    }

    if model.has_cartesian_series() {
        axis_pointer::render_axis_pointer(zr, group, model, option, interaction);
    }

    title::render_title(zr, group, model, option);
    legend::render_legend(zr, group, model, option, interaction);
    visual_map::render_visual_map(zr, group, model, option, interaction);
    data_zoom::render_data_zoom_slider(zr, group, model, option, interaction);
    toolbox::render_toolbox(zr, group, model, option);
    timeline::render_timeline(zr, group, model, option, interaction);
    brush::render_brush(zr, group, model, option, interaction);
    thumbnail::render_thumbnail(zr, group, model, option, interaction);
    graphic::render_graphic(zr, group, model, option);
}

fn render_grid_frame(zr: &mut ZRenderer, group: usize, g: crate::model::GridRect) {
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
        let interaction = InteractionState::default();
        title::render_title(&mut zr, group, &model, &option);
        legend::render_legend(&mut zr, group, &model, &option, &interaction);
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

    fn render_root(root: OptionValue) -> (crate::model::GlobalModel, ZRenderer) {
        let mut option = OptionModel::new();
        option.apply(
            root,
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = crate::model::GlobalModel::from_option(&option, 400, 300);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        render_components(&mut zr, group, &model, &option, &InteractionState::default());
        (model, zr)
    }

    #[test]
    fn wave6_series_types_are_not_other() {
        for ty in [
            "radar",
            "gauge",
            "candlestick",
            "boxplot",
            "heatmap",
            "pictorialBar",
            "effectScatter",
            "funnel",
            "chord",
            "sunburst",
            "tree",
            "treemap",
            "graph",
            "sankey",
            "themeRiver",
            "map",
            "lines",
            "parallel",
            "custom",
        ] {
            assert_ne!(
                SeriesType::from_str(ty),
                SeriesType::Other,
                "{ty} must not collapse to Other"
            );
        }
        assert_eq!(SeriesType::from_str("not-a-chart"), SeriesType::Other);
    }

    #[test]
    fn wave6_radar_gauge_funnel_draw_paths() {
        let (_, zr) = render_root(obj(vec![
            (
                "radar",
                obj(vec![(
                    "indicator",
                    OptionValue::Array(vec![
                        obj(vec![
                            ("name", OptionValue::String("A".into())),
                            ("max", OptionValue::Number(100.0)),
                        ]),
                        obj(vec![
                            ("name", OptionValue::String("B".into())),
                            ("max", OptionValue::Number(100.0)),
                        ]),
                        obj(vec![
                            ("name", OptionValue::String("C".into())),
                            ("max", OptionValue::Number(100.0)),
                        ]),
                    ]),
                )]),
            ),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("radar".into())),
                    (
                        "data",
                        OptionValue::Array(vec![obj(vec![(
                            "value",
                            OptionValue::Array(vec![
                                OptionValue::Number(60.0),
                                OptionValue::Number(80.0),
                                OptionValue::Number(40.0),
                            ]),
                        )])]),
                    ),
                ])]),
            ),
        ]));
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, rust_zrender::Shape::Polygon(_))));

        let (_, zr) = render_root(obj(vec![(
            "series",
            OptionValue::Array(vec![obj(vec![
                ("type", OptionValue::String("gauge".into())),
                (
                    "data",
                    OptionValue::Array(vec![obj(vec![("value", OptionValue::Number(42.0))])]),
                ),
            ])]),
        )]));
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, rust_zrender::Shape::Sector(_))));

        let (_, zr) = render_root(obj(vec![(
            "series",
            OptionValue::Array(vec![obj(vec![
                ("type", OptionValue::String("funnel".into())),
                (
                    "data",
                    OptionValue::Array(vec![
                        obj(vec![
                            ("name", OptionValue::String("A".into())),
                            ("value", OptionValue::Number(80.0)),
                        ]),
                        obj(vec![
                            ("name", OptionValue::String("B".into())),
                            ("value", OptionValue::Number(40.0)),
                        ]),
                    ]),
                ),
            ])]),
        )]));
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, rust_zrender::Shape::Polygon(_))));
    }

    #[test]
    fn wave6_cartesian_extras_draw() {
        let cats = OptionValue::Array(vec![
            OptionValue::String("a".into()),
            OptionValue::String("b".into()),
        ]);
        let (_, zr) = render_root(obj(vec![
            (
                "xAxis",
                obj(vec![
                    ("type", OptionValue::String("category".into())),
                    ("data", cats.clone()),
                ]),
            ),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("candlestick".into())),
                    (
                        "data",
                        OptionValue::Array(vec![
                            OptionValue::Array(vec![
                                OptionValue::Number(20.0),
                                OptionValue::Number(34.0),
                                OptionValue::Number(10.0),
                                OptionValue::Number(38.0),
                            ]),
                            OptionValue::Array(vec![
                                OptionValue::Number(40.0),
                                OptionValue::Number(35.0),
                                OptionValue::Number(30.0),
                                OptionValue::Number(50.0),
                            ]),
                        ]),
                    ),
                ])]),
            ),
        ]));
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, rust_zrender::Shape::Rect(_))));

        let (_, zr) = render_root(obj(vec![
            (
                "xAxis",
                obj(vec![
                    ("type", OptionValue::String("category".into())),
                    ("data", cats),
                ]),
            ),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("heatmap".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::Array(vec![
                            OptionValue::Number(0.0),
                            OptionValue::Number(0.0),
                            OptionValue::Number(5.0),
                        ])]),
                    ),
                ])]),
            ),
        ]));
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, rust_zrender::Shape::Rect(_))));
    }

    #[test]
    fn wave6_layout_and_network_draw() {
        let tree = obj(vec![
            ("name", OptionValue::String("root".into())),
            (
                "children",
                OptionValue::Array(vec![
                    obj(vec![
                        ("name", OptionValue::String("L".into())),
                        ("value", OptionValue::Number(1.0)),
                    ]),
                    obj(vec![
                        ("name", OptionValue::String("R".into())),
                        ("value", OptionValue::Number(2.0)),
                    ]),
                ]),
            ),
        ]);
        let (_, zr) = render_root(obj(vec![(
            "series",
            OptionValue::Array(vec![obj(vec![
                ("type", OptionValue::String("treemap".into())),
                ("data", OptionValue::Array(vec![tree.clone()])),
            ])]),
        )]));
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, rust_zrender::Shape::Rect(_))));

        let (_, zr) = render_root(obj(vec![(
            "series",
            OptionValue::Array(vec![obj(vec![
                ("type", OptionValue::String("sunburst".into())),
                ("data", OptionValue::Array(vec![tree])),
            ])]),
        )]));
        assert!(zr.storage.paths().iter().any(|p| matches!(p.shape, rust_zrender::Shape::Sector(_))));

        let (_, zr) = render_root(obj(vec![(
            "series",
            OptionValue::Array(vec![obj(vec![
                ("type", OptionValue::String("graph".into())),
                (
                    "data",
                    OptionValue::Array(vec![
                        obj(vec![("name", OptionValue::String("A".into()))]),
                        obj(vec![("name", OptionValue::String("B".into()))]),
                    ]),
                ),
                (
                    "links",
                    OptionValue::Array(vec![obj(vec![
                        ("source", OptionValue::String("A".into())),
                        ("target", OptionValue::String("B".into())),
                    ])]),
                ),
            ])]),
        )]));
        assert!(!zr.storage.paths().is_empty());

        let (_, zr) = render_root(obj(vec![
            (
                "parallelAxis",
                OptionValue::Array(vec![
                    obj(vec![("dim", OptionValue::Number(0.0))]),
                    obj(vec![("dim", OptionValue::Number(1.0))]),
                ]),
            ),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("parallel".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::Array(vec![
                            OptionValue::Number(1.0),
                            OptionValue::Number(2.0),
                        ])]),
                    ),
                ])]),
            ),
        ]));
        assert!(zr.storage.paths().iter().any(|p| matches!(
            p.shape,
            rust_zrender::Shape::Polyline(_)
        )));
    }
}
