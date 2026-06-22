mod bar;
mod line;

pub use bar::render_bar_series;
pub use line::render_line_series;

use wasm_zrender::{ChildRef, ZRenderer};

use crate::coord::Cartesian2D;
use crate::model::GlobalModel;
use crate::option::OptionModel;
use crate::visual::VisualContext;

pub fn render_components(
    zr: &mut ZRenderer,
    model: &GlobalModel,
    option: &OptionModel,
) {
    let group = zr.storage.create_group();
    let coord = Cartesian2D::new(model);
    let visual = VisualContext::new(option, model);

    render_grid_frame(zr, group, model);
    render_split_lines(zr, group, model, &coord);

    for series in &model.series {
        match series.series_type {
            crate::model::SeriesType::Line => {
                render_line_series(zr, group, model, &coord, &visual, series);
            }
            crate::model::SeriesType::Bar => {
                render_bar_series(zr, group, model, &coord, &visual, series);
            }
            _ => {}
        }
    }

    zr.storage.add_root(ChildRef::Group(group));
}

fn render_grid_frame(zr: &mut ZRenderer, group: usize, model: &GlobalModel) {
    use wasm_zrender::{
        FillStrokeStyle, LineShape, Path, PathStyle, Shape,
    };

    let g = model.grid;
    // 左边 Y 轴
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
    // 底边 X 轴
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
