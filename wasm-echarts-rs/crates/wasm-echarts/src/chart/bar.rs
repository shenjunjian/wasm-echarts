//! 柱状图 ChartView

use wasm_zrender::{
    ChildRef, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle, RectShape, Shape,
    PathStylePatch, STATE_EMPHASIS, STATE_SELECT, ZRenderer,
};

use crate::coord::Cartesian2D;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

const BAR_WIDTH_RATIO: f64 = 0.6;

pub fn render_bar_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &Cartesian2D,
    visual: &VisualContext,
    series: &SeriesModel,
    zoom_start: usize,
    zoom_end: usize,
) {
    if series.data.is_empty() {
        return;
    }

    let band = coord.category_band_width();
    let bar_w = band * BAR_WIDTH_RATIO;
    let base_y = coord.base_y().min(coord.grid().y + coord.grid().height);

    for (i, point) in series.data.iter().enumerate() {
        if i < zoom_start || i >= zoom_end {
            continue;
        }
        let (cx, top_y) = coord.data_to_point(i, point.value);
        let x = cx - bar_w / 2.0;
        let y = top_y.min(base_y);
        let h = (base_y - top_y).abs().max(1.0);

        let color = visual.resolve_item_color(series.index, i);
        let bar = zr.storage.create_path(
            Path::new(
                Shape::Rect(RectShape {
                    x,
                    y,
                    width: bar_w,
                    height: h,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color(&color),
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                z: series.index as f64,
                ..Default::default()
            })
            .with_ec_data(EcData::new(series.index as i32, i as i32)),
        );
        zr.storage.group_add_child(group, ChildRef::Path(bar));

        zr.set_path_state_style(
            bar,
            STATE_EMPHASIS,
            PathStylePatch {
                fill: Some(FillStrokeStyle::color(&color)),
                line_width: Some(2.0),
                ..Default::default()
            },
        );
        zr.set_path_state_style(
            bar,
            STATE_SELECT,
            PathStylePatch {
                stroke: Some(FillStrokeStyle::color("#333")),
                line_width: Some(2.0),
                ..Default::default()
            },
        );
    }
}
