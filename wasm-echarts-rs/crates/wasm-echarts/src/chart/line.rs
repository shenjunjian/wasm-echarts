//! 折线图 ChartView：smooth / step / connectNulls / areaStyle / endLabel / lineStyle

use rust_zrender::{
    ChildRef, DisplayableProps, FillStrokeStyle, Path, PathStyle, PolygonShape, PolylineShape, Shape,
    TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::{add_label, add_plain_label};
use crate::chart::style::{
    line_dash_from_style, line_segments, option_to_fill, parse_connect_nulls, parse_smooth, parse_step,
    style_opacity, style_width, turn_points_into_step,
};
use crate::chart::symbol::{add_symbol, SymbolSpec};
use crate::coord::{Cartesian2D, SeriesCoord};
use crate::model::{GlobalModel, SeriesModel};
use crate::option::OptionValue;
use crate::visual::VisualContext;

pub fn render_line_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
    zoom_start: usize,
    zoom_end: usize,
) {
    if series.data.is_empty() {
        return;
    }

    let series_opt = visual.series_option(series.index);
    let step = parse_step(series_opt.and_then(|s| s.get("step")));
    let smooth = if step.is_some() {
        0.0
    } else {
        parse_smooth(series_opt.and_then(|s| s.get("smooth")))
    };
    let connect_nulls = parse_connect_nulls(series_opt.and_then(|s| s.get("connectNulls")));
    let area_opt = series_opt.and_then(|s| s.get("areaStyle"));
    let has_area = matches!(area_opt, Some(OptionValue::Object(_)));
    let line_style = series_opt.and_then(|s| s.get("lineStyle"));
    let line_width = style_width(line_style, 2.0);
    let line_opacity = style_opacity(line_style, 1.0);
    let origin_opt = area_opt.and_then(|a| a.get("origin"));
    let origin_y = coord
        .as_cartesian()
        .map(|c| area_origin_y(c, origin_opt))
        .unwrap_or(0.0);

    let mut raw_line: Vec<(f64, f64)> = Vec::new();
    let mut raw_base: Vec<(f64, f64)> = Vec::new();
    let mut raw_index: Vec<usize> = Vec::new();
    for (i, p) in series.data.iter().enumerate() {
        if i < zoom_start || i >= zoom_end {
            continue;
        }
        let y_val = p.stacked_value;
        let (cx, cy) = coord.map_point(p, i);
        let finite = p.value.is_finite() && y_val.is_finite() && cx.is_finite() && cy.is_finite();
        let (px, py) = if finite { (cx, cy) } else { (f64::NAN, f64::NAN) };
        let (bx, by) = if finite {
            if series.stack.is_some() {
                coord.point_for(i, p.x_value, p.stack_base)
            } else {
                coord.area_base(i, p.x_value, origin_opt, origin_y)
            }
        } else {
            (f64::NAN, f64::NAN)
        };
        raw_line.push((px, py));
        raw_base.push((bx, by));
        raw_index.push(i);
    }

    let line_color = visual.resolve_item_color(series.index, raw_index.first().copied().unwrap_or(0));
    let stroke = option_to_fill(line_style.and_then(|ls| ls.get("color")), &line_color);
    let dash = line_dash_from_style(line_style, line_width);

    let line_segs = line_segments(&raw_line, connect_nulls);
    let base_segs = line_segments(&raw_base, connect_nulls);
    for (seg_i, mut pts) in line_segs.into_iter().enumerate() {
        if let Some(mode) = step {
            pts = turn_points_into_step(&pts, mode);
        }
        if pts.len() < 2 {
            continue;
        }
        let polyline = zr.storage.create_path(
            Path::new(
                Shape::Polyline(PolylineShape {
                    points: pts.clone(),
                    percent: 1.0,
                    smooth,
                    ..Default::default()
                }),
                PathStyle {
                    fill: FillStrokeStyle::none(),
                    stroke: stroke.clone(),
                    line_width,
                    opacity: line_opacity,
                    line_dash: dash.clone(),
                    ..PathStyle::stroke_default()
                },
            )
            .with_displayable(DisplayableProps {
                z: series.index as f64,
                ..Default::default()
            }),
        );
        zr.storage.group_add_child(group, ChildRef::Path(polyline));

        if has_area {
            let mut base_pts = base_segs.get(seg_i).cloned().unwrap_or_default();
            if let Some(mode) = step {
                base_pts = turn_points_into_step(&base_pts, mode);
            }
            if let Some(poly_pts) = area_polygon_points(&pts, &base_pts) {
                let fill_fallback = line_color.clone();
                let fill = option_to_fill(area_opt.and_then(|a| a.get("color")), &fill_fallback);
                let area_opacity = style_opacity(area_opt, 0.7);
                let polygon = zr.storage.create_path(
                    Path::new(
                        Shape::Polygon(PolygonShape {
                            points: poly_pts,
                            smooth,
                            ..Default::default()
                        }),
                        PathStyle {
                            fill,
                            stroke: FillStrokeStyle::none(),
                            opacity: area_opacity,
                            ..Default::default()
                        },
                    )
                    .with_displayable(DisplayableProps {
                        z: series.index as f64 - 0.05,
                        ..Default::default()
                    }),
                );
                zr.storage.group_add_child(group, ChildRef::Path(polygon));
            }
        }
    }

    let show_symbol = visual.show_symbol(series.index);
    let kind = visual.resolve_symbol(series.index);
    let mut last_finite: Option<(usize, f64, f64, String)> = None;

    for (local, &i) in raw_index.iter().enumerate() {
        let (cx, cy) = raw_line[local];
        if !cx.is_finite() || !cy.is_finite() {
            continue;
        }
        let color = visual.resolve_item_color(series.index, i);
        let size = visual.resolve_symbol_size_of(series.index, i);
        if show_symbol {
            add_symbol(
                zr,
                group,
                &SymbolSpec {
                    kind: kind.clone(),
                    size,
                    cx,
                    cy,
                    color: color.clone(),
                    series_index: series.index,
                    data_index: i,
                    attach_states: true,
                },
            );
        }
        add_label(
            zr,
            group,
            visual,
            series.index,
            i,
            cx,
            cy - size / 2.0 - 4.0,
            TextAlign::Center,
            TextBaseline::Bottom,
            &color,
            series.index as f64 + 0.2,
        );
        last_finite = Some((i, cx, cy, color));
    }

    if let Some((i, cx, cy, color)) = last_finite {
        render_end_label(zr, group, visual, series, series_opt, i, cx, cy, &color);
    }
}

fn area_origin_y(coord: &Cartesian2D, origin: Option<&OptionValue>) -> f64 {
    let ymin = coord.y_axis().value_min();
    let ymax = coord.y_axis().value_max();
    let value = match origin {
        Some(OptionValue::String(s)) if s == "start" => ymin,
        Some(OptionValue::String(s)) if s == "end" => ymax,
        Some(OptionValue::Number(n)) if n.is_finite() => *n,
        _ => {
            if ymin > 0.0 {
                ymin
            } else if ymax < 0.0 {
                ymax
            } else {
                0.0
            }
        }
    };
    coord.y_value_to_pixel(value)
}

fn area_polygon_points(line: &[(f64, f64)], base: &[(f64, f64)]) -> Option<Vec<(f64, f64)>> {
    if line.len() < 2 {
        return None;
    }
    let mut pts = line.to_vec();
    if base.len() == line.len() {
        pts.extend(base.iter().rev().copied());
    } else if let (Some(&(x0, _)), Some(&(x1, _))) = (line.first(), line.last()) {
        let yb = base.last().map(|p| p.1).unwrap_or(line.last().unwrap().1);
        pts.push((x1, yb));
        pts.push((x0, yb));
    } else {
        return None;
    }
    Some(pts)
}

fn render_end_label(
    zr: &mut ZRenderer,
    group: usize,
    visual: &VisualContext,
    series: &SeriesModel,
    series_opt: Option<&OptionValue>,
    data_index: usize,
    x: f64,
    y: f64,
    color: &str,
) {
    let end = series_opt.and_then(|s| s.get("endLabel"));
    let show = end.and_then(|e| e.get("show")).and_then(|v| v.as_bool()).unwrap_or(false);
    if !show {
        return;
    }
    let text = visual
        .resolve_end_label(series.index, data_index)
        .unwrap_or_else(|| {
            series
                .data
                .get(data_index)
                .map(|p| crate::utils::format_axis_number(p.value))
                .unwrap_or_default()
        });
    if text.is_empty() {
        return;
    }
    let fill = end
        .and_then(|e| e.get("color"))
        .and_then(|v| v.as_str())
        .unwrap_or(color);
    add_plain_label(
        zr,
        group,
        text,
        x + 6.0,
        y,
        TextAlign::Left,
        TextBaseline::Middle,
        fill,
        series.index as f64 + 0.3,
    );
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

    fn render(root: OptionValue) -> ZRenderer {
        let mut option = OptionModel::new();
        option.apply(
            root,
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = crate::model::GlobalModel::from_option(&option, 400, 300);
        let visual = VisualContext::new(&option, &model);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        let series = &model.series[0];
        let coord = SeriesCoord::for_series(&model, series);
        render_line_series(&mut zr, group, &model, &coord, &visual, series, 0, series.data.len());
        zr
    }

    fn line_series(extra: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut pairs = vec![
            ("type", OptionValue::String("line".into())),
            (
                "data",
                OptionValue::Array(vec![
                    OptionValue::Number(1.0),
                    OptionValue::Number(3.0),
                    OptionValue::Number(2.0),
                ]),
            ),
        ];
        pairs.extend(extra);
        obj(pairs)
    }

    fn chart(series: OptionValue) -> OptionValue {
        obj(vec![
            (
                "xAxis",
                obj(vec![
                    ("type", OptionValue::String("category".into())),
                    (
                        "data",
                        OptionValue::Array(vec![
                            OptionValue::String("a".into()),
                            OptionValue::String("b".into()),
                            OptionValue::String("c".into()),
                        ]),
                    ),
                ]),
            ),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            ("series", OptionValue::Array(vec![series])),
        ])
    }

    fn polyline_smooth(zr: &ZRenderer) -> f64 {
        zr.storage
            .paths()
            .iter()
            .find_map(|p| match &p.shape {
                Shape::Polyline(s) => Some(s.smooth),
                _ => None,
            })
            .unwrap_or(-1.0)
    }

    #[test]
    fn smooth_true_sets_polyline_smooth() {
        let zr = render(chart(line_series(vec![("smooth", OptionValue::Bool(true))])));
        assert!((polyline_smooth(&zr) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn area_style_creates_polygon() {
        let zr = render(chart(line_series(vec![("areaStyle", obj(vec![]))])));
        let has_poly = zr
            .storage
            .paths()
            .iter()
            .any(|p| matches!(p.shape, Shape::Polygon(_)));
        assert!(has_poly);
    }

    #[test]
    fn dashed_line_style_sets_dash() {
        let zr = render(chart(line_series(vec![(
            "lineStyle",
            obj(vec![
                ("width", OptionValue::Number(4.0)),
                ("type", OptionValue::String("dashed".into())),
            ]),
        )])));
        let line = zr
            .storage
            .paths()
            .iter()
            .find(|p| matches!(p.shape, Shape::Polyline(_)))
            .unwrap();
        assert!((line.style.line_width - 4.0).abs() < 1e-6);
        assert_eq!(line.style.line_dash.as_ref().map(|d| d.len()), Some(2));
    }

    #[test]
    fn end_label_draws_text() {
        let zr = render(chart(line_series(vec![(
            "endLabel",
            obj(vec![("show", OptionValue::Bool(true))]),
        )])));
        assert!(
            zr.storage.texts().iter().any(|t| t.content == "2"),
            "endLabel texts: {:?}",
            zr.storage.texts().iter().map(|t| t.content.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn connect_nulls_false_splits_polyline() {
        let series = obj(vec![
            ("type", OptionValue::String("line".into())),
            (
                "data",
                OptionValue::Array(vec![
                    OptionValue::Number(1.0),
                    OptionValue::Number(2.0),
                    OptionValue::Null,
                    OptionValue::Number(3.0),
                    OptionValue::Number(4.0),
                ]),
            ),
        ]);
        let zr = render(chart(series));
        let n = zr
            .storage
            .paths()
            .iter()
            .filter(|p| matches!(p.shape, Shape::Polyline(_)))
            .count();
        assert_eq!(n, 2);
    }

    #[test]
    fn connect_nulls_true_joins_polyline() {
        let series = obj(vec![
            ("type", OptionValue::String("line".into())),
            ("connectNulls", OptionValue::Bool(true)),
            (
                "data",
                OptionValue::Array(vec![
                    OptionValue::Number(1.0),
                    OptionValue::Null,
                    OptionValue::Number(3.0),
                ]),
            ),
        ]);
        let zr = render(chart(series));
        let n = zr
            .storage
            .paths()
            .iter()
            .filter(|p| matches!(p.shape, Shape::Polyline(_)))
            .count();
        assert_eq!(n, 1);
    }

    #[test]
    fn polar_line_draws_polyline() {
        let zr = render(obj(vec![
            ("polar", obj(vec![])),
            (
                "angleAxis",
                obj(vec![
                    ("type", OptionValue::String("value".into())),
                    ("startAngle", OptionValue::Number(0.0)),
                ]),
            ),
            ("radiusAxis", obj(vec![])),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("line".into())),
                    ("coordinateSystem", OptionValue::String("polar".into())),
                    (
                        "data",
                        OptionValue::Array(vec![
                            OptionValue::Array(vec![
                                OptionValue::Number(1.0),
                                OptionValue::Number(0.0),
                            ]),
                            OptionValue::Array(vec![
                                OptionValue::Number(2.0),
                                OptionValue::Number(90.0),
                            ]),
                            OptionValue::Array(vec![
                                OptionValue::Number(3.0),
                                OptionValue::Number(180.0),
                            ]),
                        ]),
                    ),
                ])]),
            ),
        ]));
        let n = zr
            .storage
            .paths()
            .iter()
            .filter(|p| matches!(p.shape, Shape::Polyline(_)))
            .count();
        assert!(n >= 1, "polar line should draw a polyline");
    }
}
