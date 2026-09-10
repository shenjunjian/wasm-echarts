//! 柱状图 ChartView：barWidth / barGap / barCategoryGap / borderRadius / barMinHeight

use rust_zrender::{
    ChildRef, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle, PathStylePatch, RectShape,
    SectorShape, Shape, STATE_EMPHASIS, STATE_SELECT, TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_label;
use crate::coord::{Cartesian2D, SeriesCoord};
use crate::model::{GlobalModel, SeriesModel, SeriesType};
use crate::option::OptionValue;
use crate::utils::parse_percent;
use crate::visual::VisualContext;

#[derive(Debug, Clone, Copy)]
pub struct BarColumnLayout {
    pub offset: f64,
    pub width: f64,
}

pub fn bar_column_layout(
    model: &GlobalModel,
    series: &SeriesModel,
    visual: &VisualContext,
    band: f64,
) -> BarColumnLayout {
    let peers: Vec<&SeriesModel> = model
        .series
        .iter()
        .filter(|s| {
            s.series_type == SeriesType::Bar
                && s.x_axis_index == series.x_axis_index
                && s.y_axis_index == series.y_axis_index
        })
        .collect();
    if peers.is_empty() {
        return BarColumnLayout {
            offset: -band * 0.3,
            width: band * 0.6,
        };
    }

    let mut columns: Vec<(String, Option<f64>)> = Vec::new();
    let mut bar_gap = OptionValue::String("10%".into());
    let mut bar_category_gap: Option<&OptionValue> = None;
    for s in &peers {
        let opt = visual.series_option(s.index);
        if let Some(g) = opt.and_then(|o| o.get("barGap")) {
            bar_gap = g.clone();
        }
        if let Some(g) = opt.and_then(|o| o.get("barCategoryGap")) {
            bar_category_gap = Some(g);
        }
        let stack_id = s
            .stack
            .clone()
            .unwrap_or_else(|| format!("__ec_stack_{}", s.index));
        if !columns.iter().any(|(id, _)| id == &stack_id) {
            let width = opt.and_then(|o| o.get("barWidth")).map(|w| parse_percent(Some(w), band, 0.0));
            columns.push((stack_id, width.filter(|n| *n > 0.0)));
        }
    }

    let n = columns.len().max(1);
    let category_gap = match bar_category_gap {
        Some(v) => parse_percent(Some(v), band, 0.0),
        None => {
            let pct = (35.0 - n as f64 * 4.0).max(15.0);
            band * pct / 100.0
        }
    };
    let gap_ratio = parse_percent(Some(&bar_gap), 1.0, 0.1);
    let auto_count = columns.iter().filter(|(_, w)| w.is_none()).count() as f64;
    let mut remained = (band - category_gap).max(0.0);
    for (_, w) in &columns {
        if let Some(width) = w {
            remained -= *width;
        }
    }
    let auto_width = if auto_count > 0.0 {
        (remained / (auto_count + (auto_count - 1.0).max(0.0) * gap_ratio)).max(0.0)
    } else {
        0.0
    };

    let widths: Vec<f64> = columns
        .iter()
        .map(|(_, w)| w.unwrap_or(auto_width).max(0.0))
        .collect();
    let mut width_sum = 0.0;
    for (i, w) in widths.iter().enumerate() {
        width_sum += *w;
        if i + 1 < widths.len() {
            width_sum += *w * gap_ratio;
        }
    }
    let mut offset = -width_sum / 2.0;
    let own_id = series
        .stack
        .clone()
        .unwrap_or_else(|| format!("__ec_stack_{}", series.index));
    for (i, (id, _)) in columns.iter().enumerate() {
        let w = widths[i];
        if *id == own_id {
            return BarColumnLayout {
                offset,
                width: w,
            };
        }
        offset += w * (1.0 + gap_ratio);
    }
    BarColumnLayout {
        offset: -band * 0.3,
        width: band * 0.6,
    }
}

fn border_radius(item_style: Option<&OptionValue>) -> Vec<f64> {
    match item_style.and_then(|s| s.get("borderRadius")) {
        Some(OptionValue::Number(n)) if *n > 0.0 => vec![*n],
        Some(OptionValue::Array(arr)) => arr.iter().filter_map(|v| v.as_f64()).collect(),
        _ => Vec::new(),
    }
}

pub fn render_bar_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
    zoom_start: usize,
    zoom_end: usize,
) {
    if series.data.is_empty() {
        return;
    }
    if let Some(polar) = coord.as_polar() {
        render_polar_bars(zr, group, polar, visual, series, zoom_start, zoom_end);
        return;
    }
    let Some(coord) = coord.as_cartesian() else {
        return;
    };
    render_cartesian_bars(zr, group, model, coord, visual, series, zoom_start, zoom_end);
}

fn render_cartesian_bars(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &Cartesian2D,
    visual: &VisualContext,
    series: &SeriesModel,
    zoom_start: usize,
    zoom_end: usize,
) {
    let series_opt = visual.series_option(series.index);
    let band = coord.category_band_width();
    let layout = bar_column_layout(model, series, visual, band);
    let bar_w = layout.width.max(0.5);
    let min_height = parse_percent(series_opt.and_then(|s| s.get("barMinHeight")), 1.0, 0.0);
    let radius = border_radius(series_opt.and_then(|s| s.get("itemStyle")));
    let horizontal = coord.is_horizontal();
    let zero_y = coord.base_y().min(coord.grid().y + coord.grid().height);
    let zero_x = coord.base_x().max(coord.grid().x);

    for (i, point) in series.data.iter().enumerate() {
        if i < zoom_start || i >= zoom_end {
            continue;
        }
        if !point.value.is_finite() && !point.stacked_value.is_finite() {
            continue;
        }
        let color = visual.resolve_item_color(series.index, i);
        if horizontal {
            let (top_x, cy) = coord.point_for(i, point.x_value, point.stacked_value);
            let base_x = if point.stack_base.abs() > f64::EPSILON || series.stack.is_some() {
                coord.point_for(i, point.x_value, point.stack_base).0
            } else {
                zero_x
            };
            let y = cy + layout.offset;
            let mut x = top_x.min(base_x);
            let mut w = (base_x - top_x).abs();
            if w < min_height {
                w = min_height;
                if top_x <= base_x {
                    x = base_x - w;
                }
            }
            w = w.max(1.0);
            add_bar_rect(
                zr,
                group,
                series,
                i,
                x,
                y,
                w,
                bar_w,
                radius.clone(),
                &color,
            );
            add_label(
                zr,
                group,
                visual,
                series.index,
                i,
                x + w + 4.0,
                y + bar_w / 2.0,
                TextAlign::Left,
                TextBaseline::Middle,
                &color,
                series.index as f64 + 0.2,
            );
            continue;
        }
        let (cx, top_y) = coord.point_for(i, point.x_value, point.stacked_value);
        let base_y = if point.stack_base.abs() > f64::EPSILON || series.stack.is_some() {
            coord.point_for(i, point.x_value, point.stack_base).1
        } else {
            zero_y
        };
        let x = cx + layout.offset;
        let mut y = top_y.min(base_y);
        let mut h = (base_y - top_y).abs();
        if h < min_height {
            h = min_height;
            if top_y <= base_y {
                y = base_y - h;
            }
        }
        h = h.max(1.0);
        add_bar_rect(
            zr,
            group,
            series,
            i,
            x,
            y,
            bar_w,
            h,
            radius.clone(),
            &color,
        );
        add_label(
            zr,
            group,
            visual,
            series.index,
            i,
            x + bar_w / 2.0,
            y - 4.0,
            TextAlign::Center,
            TextBaseline::Bottom,
            &color,
            series.index as f64 + 0.2,
        );
    }
}

fn add_bar_rect(
    zr: &mut ZRenderer,
    group: usize,
    series: &SeriesModel,
    i: usize,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radius: Vec<f64>,
    color: &str,
) {
    let bar = zr.storage.create_path(
        Path::new(
            Shape::Rect(RectShape {
                x,
                y,
                width,
                height,
                r: radius,
            }),
            PathStyle {
                fill: FillStrokeStyle::color(color),
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
            fill: Some(FillStrokeStyle::color(color)),
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

fn render_polar_bars(
    zr: &mut ZRenderer,
    group: usize,
    polar: &crate::coord::PolarCoord,
    visual: &VisualContext,
    series: &SeriesModel,
    zoom_start: usize,
    zoom_end: usize,
) {
    let band = polar.angle_band();
    let half = band * 0.35;
    for (i, point) in series.data.iter().enumerate() {
        if i < zoom_start || i >= zoom_end {
            continue;
        }
        if !point.stacked_value.is_finite() {
            continue;
        }
        let (_, angle_val) = crate::model::polar_data_pair(
            point,
            i,
            polar.angle_axis().axis_type.is_category(),
        );
        let r1 = polar.radius_to_pixel(point.stacked_value);
        let r0 = if series.stack.is_some() || point.stack_base.abs() > f64::EPSILON {
            polar.radius_to_pixel(point.stack_base)
        } else {
            polar.spec.r0
        };
        let mid = polar.angle_to_degree(angle_val);
        let start = (mid - half) * std::f64::consts::PI / 180.0;
        let end = (mid + half) * std::f64::consts::PI / 180.0;
        let color = visual.resolve_item_color(series.index, i);
        let bar = zr.storage.create_path(
            Path::new(
                Shape::Sector(SectorShape {
                    cx: polar.spec.center_x,
                    cy: polar.spec.center_y,
                    r: r1.max(r0),
                    r0: r0.min(r1),
                    start_angle: start,
                    end_angle: end,
                    clockwise: true,
                    corner_radius: Vec::new(),
                    percent: 1.0,
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

    fn render(root: OptionValue) -> (ZRenderer, GlobalModel) {
        let mut option = OptionModel::new();
        option.apply(
            root,
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 400, 300);
        let visual = VisualContext::new(&option, &model);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        for series in &model.series {
            if series.series_type != SeriesType::Bar {
                continue;
            }
            let coord = SeriesCoord::for_series(&model, series);
            render_bar_series(
                &mut zr,
                group,
                &model,
                &coord,
                &visual,
                series,
                0,
                series.data.len(),
            );
        }
        (zr, model)
    }

    fn axes_and_series(series: Vec<OptionValue>) -> OptionValue {
        obj(vec![
            (
                "xAxis",
                obj(vec![
                    ("type", OptionValue::String("category".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::String("A".into())]),
                    ),
                ]),
            ),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            ("series", OptionValue::Array(series)),
        ])
    }

    fn bar_data(extra: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut pairs = vec![
            ("type", OptionValue::String("bar".into())),
            ("data", OptionValue::Array(vec![OptionValue::Number(10.0)])),
        ];
        pairs.extend(extra);
        obj(pairs)
    }

    fn first_rect(zr: &ZRenderer) -> &RectShape {
        zr.storage
            .paths()
            .iter()
            .find_map(|p| match &p.shape {
                Shape::Rect(r) => Some(r),
                _ => None,
            })
            .expect("rect")
    }

    #[test]
    fn bar_width_pixels() {
        let (zr, _) = render(axes_and_series(vec![bar_data(vec![(
            "barWidth",
            OptionValue::Number(20.0),
        )])]));
        assert!((first_rect(&zr).width - 20.0).abs() < 1e-6);
    }

    #[test]
    fn two_series_side_by_side_not_same_x() {
        let (zr, _) = render(axes_and_series(vec![
            bar_data(vec![]),
            bar_data(vec![]),
        ]));
        let xs: Vec<f64> = zr
            .storage
            .paths()
            .iter()
            .filter_map(|p| match &p.shape {
                Shape::Rect(r) => Some(r.x),
                _ => None,
            })
            .collect();
        assert_eq!(xs.len(), 2);
        assert!(
            (xs[0] - xs[1]).abs() > 1.0,
            "bars should sit side by side: {:?}",
            xs
        );
    }

    #[test]
    fn stacked_bars_share_x() {
        let (zr, _) = render(axes_and_series(vec![
            bar_data(vec![("stack", OptionValue::String("t".into()))]),
            bar_data(vec![("stack", OptionValue::String("t".into()))]),
        ]));
        let xs: Vec<f64> = zr
            .storage
            .paths()
            .iter()
            .filter_map(|p| match &p.shape {
                Shape::Rect(r) => Some(r.x),
                _ => None,
            })
            .collect();
        assert_eq!(xs.len(), 2);
        assert!(
            (xs[0] - xs[1]).abs() < 1e-6,
            "stacked bars share x: {:?}",
            xs
        );
    }

    #[test]
    fn border_radius_and_min_height() {
        let (zr, _) = render(obj(vec![
            (
                "xAxis",
                obj(vec![
                    ("type", OptionValue::String("category".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::String("A".into())]),
                    ),
                ]),
            ),
            (
                "yAxis",
                obj(vec![
                    ("type", OptionValue::String("value".into())),
                    ("min", OptionValue::Number(0.0)),
                    ("max", OptionValue::Number(100.0)),
                ]),
            ),
            (
                "series",
                OptionValue::Array(vec![bar_data(vec![
                    ("barMinHeight", OptionValue::Number(12.0)),
                    (
                        "itemStyle",
                        obj(vec![("borderRadius", OptionValue::Number(4.0))]),
                    ),
                    ("data", OptionValue::Array(vec![OptionValue::Number(0.01)])),
                ])]),
            ),
        ]));
        let rect = first_rect(&zr);
        assert!(rect.height >= 12.0 - 1e-6);
        assert_eq!(rect.r, vec![4.0]);
    }
}
