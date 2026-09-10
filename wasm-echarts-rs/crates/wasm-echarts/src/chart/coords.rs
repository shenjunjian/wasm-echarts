//! 第 5 波坐标系底图：polar / radar / single / parallel / calendar / matrix / geo

use rust_zrender::{
    ChildRef, CircleShape, FillStrokeStyle, LineShape, Path, PathStyle, PolygonShape, RectShape, Shape,
    TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::text_opt::{add_silent_text, parse_chart_text_style};
use crate::coord::{
    format_axis_tick, CalendarCoord, GeoCoord, MatrixCoord, ParallelCoord, PolarCoord, RadarCoord,
    SingleCoord,
};
use crate::model::GlobalModel;
use crate::option::OptionModel;

pub fn render_extra_coords(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
) {
    for i in 0..model.polars.len() {
        if let Some(coord) = PolarCoord::new(model, i) {
            render_polar(zr, group, &coord);
        }
    }
    for i in 0..model.radars.len() {
        if let Some(coord) = RadarCoord::new(model, i) {
            render_radar(zr, group, &coord);
        }
    }
    for i in 0..model.single_axes.len() {
        if let Some(coord) = SingleCoord::new(model, i) {
            render_single(zr, group, &coord);
        }
    }
    for i in 0..model.parallels.len() {
        if let Some(coord) = ParallelCoord::new(model, i) {
            render_parallel(zr, group, &coord);
        }
    }
    for i in 0..model.calendars.len() {
        if let Some(coord) = CalendarCoord::new(model, i) {
            render_calendar(zr, group, &coord);
        }
    }
    for i in 0..model.matrices.len() {
        if let Some(coord) = MatrixCoord::new(model, i) {
            render_matrix(zr, group, &coord);
        }
    }
    for i in 0..model.geos.len() {
        if let Some(coord) = GeoCoord::new(model, i) {
            render_geo(zr, group, &coord);
        }
    }
    let _ = option;
}

fn stroke_style(color: &str, width: f64) -> PathStyle {
    PathStyle {
        fill: FillStrokeStyle::none(),
        stroke: FillStrokeStyle::color(color),
        line_width: width as f32,
        ..Default::default()
    }
}

fn add_line(zr: &mut ZRenderer, group: usize, x1: f64, y1: f64, x2: f64, y2: f64, color: &str) {
    let id = zr.storage.create_path(Path::new(
        Shape::Line(LineShape {
            x1,
            y1,
            x2,
            y2,
            percent: 1.0,
        }),
        stroke_style(color, 1.0),
    ));
    zr.storage.group_add_child(group, ChildRef::Path(id));
}

fn render_polar(zr: &mut ZRenderer, group: usize, coord: &PolarCoord) {
    let spec = coord.spec;
    let ring = zr.storage.create_path(Path::new(
        Shape::Circle(CircleShape {
            cx: spec.center_x,
            cy: spec.center_y,
            r: spec.r,
        }),
        stroke_style("#333", 1.0),
    ));
    zr.storage.group_add_child(group, ChildRef::Path(ring));
    if spec.r0 > 1.0 {
        let inner = zr.storage.create_path(Path::new(
            Shape::Circle(CircleShape {
                cx: spec.center_x,
                cy: spec.center_y,
                r: spec.r0,
            }),
            stroke_style("#333", 1.0),
        ));
        zr.storage.group_add_child(group, ChildRef::Path(inner));
    }
    for i in 1..5 {
        let t = i as f64 / 5.0;
        let r = spec.r0 + t * (spec.r - spec.r0);
        let c = zr.storage.create_path(Path::new(
            Shape::Circle(CircleShape {
                cx: spec.center_x,
                cy: spec.center_y,
                r,
            }),
            stroke_style("#eee", 1.0),
        ));
        zr.storage.group_add_child(group, ChildRef::Path(c));
    }
    let spokes = if spec.angle_axis.axis_type.is_category() {
        spec.angle_axis.category_data.len().max(1)
    } else {
        12
    };
    for i in 0..spokes {
        let deg = coord.angle_to_degree(i as f64);
        let (x, y) = coord.coord_to_point(spec.r, deg);
        add_line(zr, group, spec.center_x, spec.center_y, x, y, "#eee");
        if spec.angle_axis.axis_type.is_category() {
            if let Some(label) = spec.angle_axis.category_data.get(i) {
                let (lx, ly) = coord.coord_to_point(spec.r + 14.0, deg);
                let style = parse_chart_text_style(None, "#666", 11.0, "sans-serif");
                add_silent_text(
                    zr,
                    group,
                    label,
                    lx,
                    ly,
                    style.to_text_style(TextAlign::Center, TextBaseline::Middle),
                    2.0,
                );
            }
        }
    }
    if !spec.radius_axis.axis_type.is_category() {
        let style = parse_chart_text_style(None, "#666", 11.0, "sans-serif");
        for i in 0..=4 {
            let t = i as f64 / 4.0;
            let v = spec.radius_axis.value_min()
                + t * (spec.radius_axis.value_max() - spec.radius_axis.value_min());
            let r = coord.radius_to_pixel(v);
            let (x, y) = coord.coord_to_point(r, spec.start_angle);
            let label = format_axis_tick(&spec.radius_axis, v, None);
            add_silent_text(
                zr,
                group,
                &label,
                x + 4.0,
                y,
                style.to_text_style(TextAlign::Left, TextBaseline::Middle),
                2.0,
            );
        }
    }
}

fn render_radar(zr: &mut ZRenderer, group: usize, coord: &RadarCoord) {
    let spec = coord.spec;
    let n = spec.indicators.len().max(1);
    for ring in 1..=spec.split_number.max(1) {
        let t = ring as f64 / spec.split_number.max(1) as f64;
        let radius = spec.r0 + t * (spec.r - spec.r0);
        if spec.shape_circle {
            let c = zr.storage.create_path(Path::new(
                Shape::Circle(CircleShape {
                    cx: spec.center_x,
                    cy: spec.center_y,
                    r: radius,
                }),
                stroke_style("#ccc", 1.0),
            ));
            zr.storage.group_add_child(group, ChildRef::Path(c));
        } else {
            let pts: Vec<(f64, f64)> = (0..n).map(|i| coord.coord_to_point(radius, i)).collect();
            let poly = zr.storage.create_path(Path::new(
                Shape::Polygon(PolygonShape {
                    points: pts,
                    smooth: 0.0,
                    smooth_constraint: None,
                }),
                stroke_style("#ccc", 1.0),
            ));
            zr.storage.group_add_child(group, ChildRef::Path(poly));
        }
    }
    let style = parse_chart_text_style(None, "#333", 12.0, "sans-serif");
    for i in 0..n {
        let (x, y) = coord.coord_to_point(spec.r, i);
        add_line(zr, group, spec.center_x, spec.center_y, x, y, "#ccc");
        if let Some(ind) = spec.indicators.get(i) {
            let (lx, ly) = coord.coord_to_point(spec.r + 16.0, i);
            add_silent_text(
                zr,
                group,
                &ind.name,
                lx,
                ly,
                style.to_text_style(TextAlign::Center, TextBaseline::Middle),
                2.0,
            );
        }
    }
}

fn render_single(zr: &mut ZRenderer, group: usize, coord: &SingleCoord) {
    let r = coord.spec.rect;
    if coord.spec.horizontal {
        add_line(zr, group, r.x, r.y + r.height * 0.5, r.x + r.width, r.y + r.height * 0.5, "#333");
    } else {
        add_line(zr, group, r.x + r.width * 0.5, r.y, r.x + r.width * 0.5, r.y + r.height, "#333");
    }
    let style = parse_chart_text_style(None, "#666", 11.0, "sans-serif");
    if coord.spec.axis.axis_type.is_category() {
        for (i, label) in coord.spec.axis.category_data.iter().enumerate() {
            let (x, y) = coord.data_to_point(i as f64);
            add_silent_text(
                zr,
                group,
                label,
                x,
                y + 14.0,
                style.to_text_style(TextAlign::Center, TextBaseline::Top),
                2.0,
            );
        }
    } else {
        for i in 0..=4 {
            let t = i as f64 / 4.0;
            let v = coord.spec.axis.value_min() + t * (coord.spec.axis.value_max() - coord.spec.axis.value_min());
            let (x, y) = coord.data_to_point(v);
            let label = format_axis_tick(&coord.spec.axis, v, None);
            add_silent_text(
                zr,
                group,
                &label,
                x,
                y + 14.0,
                style.to_text_style(TextAlign::Center, TextBaseline::Top),
                2.0,
            );
        }
    }
}

fn render_parallel(zr: &mut ZRenderer, group: usize, coord: &ParallelCoord) {
    let r = coord.spec.rect;
    let n = coord.spec.axes.len().max(1);
    let style = parse_chart_text_style(None, "#333", 11.0, "sans-serif");
    for (i, axis) in coord.spec.axes.iter().enumerate() {
        let t = if n <= 1 { 0.5 } else { i as f64 / (n - 1) as f64 };
        if coord.spec.horizontal {
            let x = r.x + t * r.width;
            add_line(zr, group, x, r.y, x, r.y + r.height, "#333");
            add_silent_text(
                zr,
                group,
                &axis.name,
                x,
                r.y - 8.0,
                style.to_text_style(TextAlign::Center, TextBaseline::Bottom),
                2.0,
            );
        } else {
            let y = r.y + t * r.height;
            add_line(zr, group, r.x, y, r.x + r.width, y, "#333");
            add_silent_text(
                zr,
                group,
                &axis.name,
                r.x - 8.0,
                y,
                style.to_text_style(TextAlign::Right, TextBaseline::Middle),
                2.0,
            );
        }
    }
}

fn render_calendar(zr: &mut ZRenderer, group: usize, coord: &CalendarCoord) {
    let spec = coord.spec;
    let days = ((spec.end_ms - spec.start_ms) / 86_400_000.0).ceil().max(1.0) as i64;
    for d in 0..=days {
        let ms = spec.start_ms + d as f64 * 86_400_000.0;
        if ms > spec.end_ms + 86_400_000.0 {
            break;
        }
        let (cx, cy) = coord.data_to_point(ms);
        if !cx.is_finite() {
            continue;
        }
        let rect = zr.storage.create_path(Path::new(
            Shape::Rect(RectShape {
                x: cx - spec.cell_w * 0.5,
                y: cy - spec.cell_h * 0.5,
                width: spec.cell_w,
                height: spec.cell_h,
                r: vec![],
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#fff"),
                stroke: FillStrokeStyle::color("#ccc"),
                line_width: 1.0,
                ..Default::default()
            },
        ));
        zr.storage.group_add_child(group, ChildRef::Path(rect));
    }
}

fn render_matrix(zr: &mut ZRenderer, group: usize, coord: &MatrixCoord) {
    let (cw, ch) = coord.cell_size();
    let r = coord.spec.rect;
    let nx = coord.spec.x_data.len().max(1);
    let ny = coord.spec.y_data.len().max(1);
    let style = parse_chart_text_style(None, "#666", 11.0, "sans-serif");
    for yi in 0..ny {
        for xi in 0..nx {
            let x = r.x + xi as f64 * cw;
            let y = r.y + yi as f64 * ch;
            let rect = zr.storage.create_path(Path::new(
                Shape::Rect(RectShape {
                    x,
                    y,
                    width: cw,
                    height: ch,
                    r: vec![],
                }),
                PathStyle {
                    fill: FillStrokeStyle::none(),
                    stroke: FillStrokeStyle::color("#ddd"),
                    line_width: 1.0,
                    ..Default::default()
                },
            ));
            zr.storage.group_add_child(group, ChildRef::Path(rect));
        }
    }
    for (i, name) in coord.spec.x_data.iter().enumerate() {
        add_silent_text(
            zr,
            group,
            name,
            r.x + (i as f64 + 0.5) * cw,
            r.y + r.height + 12.0,
            style.to_text_style(TextAlign::Center, TextBaseline::Top),
            2.0,
        );
    }
    for (i, name) in coord.spec.y_data.iter().enumerate() {
        add_silent_text(
            zr,
            group,
            name,
            r.x - 8.0,
            r.y + (i as f64 + 0.5) * ch,
            style.to_text_style(TextAlign::Right, TextBaseline::Middle),
            2.0,
        );
    }
}

fn render_geo(zr: &mut ZRenderer, group: usize, coord: &GeoCoord) {
    for region in coord.regions() {
        for poly in &region.polygons {
            let pts = coord.project_polygon(poly);
            if pts.len() < 3 {
                continue;
            }
            let id = zr.storage.create_path(Path::new(
                Shape::Polygon(PolygonShape {
                    points: pts,
                    smooth: 0.0,
                    smooth_constraint: None,
                }),
                PathStyle {
                    fill: FillStrokeStyle::color("#eee"),
                    stroke: FillStrokeStyle::color("#999"),
                    line_width: 1.0,
                    ..Default::default()
                },
            ));
            zr.storage.group_add_child(group, ChildRef::Path(id));
        }
    }
}
