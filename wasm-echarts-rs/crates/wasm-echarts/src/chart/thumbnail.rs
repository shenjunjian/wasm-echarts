//! thumbnail：cartesian 缩略 + 窗口 roam（拖窗口改 dataZoom）

use rust_zrender::{
    ChildRef, FillStrokeStyle, Path, PathStyle, PolylineShape, Shape, ZRenderer,
};

use crate::chart::layout::{add_rect, component_ec, layout_origin, HIT_THUMBNAIL};
use crate::coord::Cartesian2D;
use crate::interaction::InteractionState;
use crate::model::GlobalModel;
use crate::option::OptionModel;
use crate::utils::first_component;

#[derive(Debug, Clone, Copy)]
pub struct ThumbGeom {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn thumb_geom(model: &GlobalModel, option: &OptionModel) -> Option<ThumbGeom> {
    let th = first_component(option.root().get("thumbnail"))?;
    if th.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return None;
    }
    let width = th.get("width").and_then(|v| v.as_f64()).unwrap_or(120.0);
    let height = th.get("height").and_then(|v| v.as_f64()).unwrap_or(80.0);
    let (x, y) = layout_origin(
        th,
        model.width as f64,
        model.height as f64,
        width,
        height,
        model.width as f64 - width - 12.0,
        40.0,
    );
    Some(ThumbGeom { x, y, width, height })
}

pub fn render_thumbnail(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    let Some(geom) = thumb_geom(model, option) else {
        return;
    };
    add_rect(
        zr,
        group,
        geom.x,
        geom.y,
        geom.width,
        geom.height,
        FillStrokeStyle::color("rgba(255,255,255,0.92)"),
        FillStrokeStyle::color("#ccc"),
        1.0,
        23.0,
        Some(component_ec(HIT_THUMBNAIL, 0)),
    );
    if let Some(series) = model.series.first() {
        let coord = Cartesian2D::for_series(model, series);
        let g = coord.grid();
        let mut pts = Vec::new();
        let n = series.data.len().max(1);
        for (i, p) in series.data.iter().enumerate() {
            if !p.value.is_finite() {
                continue;
            }
            let (px, py) = coord.point_for(i, p.x_value, p.value);
            let tx = geom.x + 4.0 + (px - g.x) / g.width.max(1.0) * (geom.width - 8.0);
            let ty = geom.y + 4.0 + (py - g.y) / g.height.max(1.0) * (geom.height - 8.0);
            pts.push((tx, ty));
            let _ = n;
        }
        if pts.len() >= 2 {
            let line = zr.storage.create_path(Path::new(
                Shape::Polyline(PolylineShape {
                    points: pts,
                    percent: 1.0,
                    ..Default::default()
                }),
                PathStyle {
                    fill: FillStrokeStyle::none(),
                    stroke: FillStrokeStyle::color("#5470c6"),
                    line_width: 1.0,
                    ..PathStyle::stroke_default()
                },
            ));
            zr.storage.group_add_child(group, ChildRef::Path(line));
        }
    }
    let wx = geom.x + geom.width * interaction.data_zoom.start / 100.0;
    let ww = geom.width * (interaction.data_zoom.end - interaction.data_zoom.start) / 100.0;
    add_rect(
        zr,
        group,
        wx,
        geom.y,
        ww.max(8.0),
        geom.height,
        FillStrokeStyle::color("rgba(84,112,198,0.18)"),
        FillStrokeStyle::color("#5470c6"),
        1.0,
        23.2,
        Some(component_ec(HIT_THUMBNAIL, 1)),
    );
}

pub fn apply_thumbnail_drag(
    interaction: &mut InteractionState,
    geom: ThumbGeom,
    x: f64,
) {
    let Some(drag) = interaction.drag else {
        return;
    };
    let dx = (x - drag.start_x) / geom.width.max(1.0) * 100.0;
    interaction.data_zoom = drag.start_zoom;
    interaction.data_zoom.pan(dx);
}
