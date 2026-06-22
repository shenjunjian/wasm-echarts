mod axis;
mod bar;
mod line;
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

use wasm_zrender::{
    ChildRef, FillStrokeStyle, LineShape, Path, PathStyle, Shape, ZRenderer,
};

use crate::coord::Cartesian2D;
use crate::interaction::InteractionState;
use crate::model::{GlobalModel, SeriesType};
use crate::option::OptionModel;
use crate::visual::VisualContext;

pub fn render_components(
    zr: &mut ZRenderer,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    let group = zr.storage.create_group();
    let coord = Cartesian2D::new(model);
    let visual = VisualContext::new(option, model);
    let (zoom_start, zoom_end) = model.visible_category_range();

    if model.has_cartesian_series() {
        render_grid_frame(zr, group, model);
        render_split_lines(zr, group, model, &coord);
        axis::render_axis_labels(zr, group, model, &coord, zoom_start, zoom_end);
    }

    for series in &model.series {
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
                render_pie_series(zr, group, model, &visual, series);
            }
            #[cfg(feature = "chart-scatter")]
            SeriesType::Scatter => {
                render_scatter_series(zr, group, model, &coord, &visual, series);
            }
            _ => {}
        }
    }

    if model.has_cartesian_series() {
        render_axis_pointer(zr, group, model, &coord, interaction);
    }

    zr.storage.add_root(ChildRef::Group(group));
}

fn render_grid_frame(zr: &mut ZRenderer, group: usize, model: &GlobalModel) {
    use wasm_zrender::{
        FillStrokeStyle, LineShape, Path, PathStyle, Shape,
    };

    let g = model.grid;
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
    model: &GlobalModel,
    coord: &Cartesian2D,
) {
    use wasm_zrender::{
        FillStrokeStyle, LineShape, Path, PathStyle, Shape,
    };

    let g = model.grid;
    let split_count = 5;
    let ymin = model.y_axis.value_min();
    let ymax = model.y_axis.value_max();
    let span = ymax - ymin;

    for i in 0..=split_count {
        let value = ymin + span * i as f64 / split_count as f64;
        let (_, y) = coord.data_to_point(0, value);
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
        (Some(x), Some(y)) if model.grid.contains(x, y) => (x, y),
        _ => return,
    };

    let snap_x = interaction
        .axis_pointer_label(model, px, py)
        .map(|(_, _, x)| x)
        .unwrap_or(px);

    let g = model.grid;
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
