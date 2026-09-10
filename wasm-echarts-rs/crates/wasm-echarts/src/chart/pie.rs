//! 饼图 ChartView：roseType / selectedMode / label 避让最小集

use std::f64::consts::PI;

use rust_zrender::{
    ChildRef, DisplayableProps, EcData, FillStrokeStyle, LineShape, Path, PathStyle, PathStylePatch,
    SectorShape, Shape, STATE_EMPHASIS, STATE_SELECT, TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_label;
use crate::interaction::{DataTarget, InteractionState};
use crate::model::{GlobalModel, SeriesModel};
use crate::option::OptionValue;
use crate::utils::parse_percent;
use crate::visual::VisualContext;

/// ECharts 默认 startAngle=90°，顺时针
const DEFAULT_START_ANGLE_DEG: f64 = 90.0;

pub struct PieLayout {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub r0: f64,
    pub start_angle: f64,
    pub clockwise: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct PieSectorLayout {
    pub start_angle: f64,
    pub end_angle: f64,
    pub r: f64,
    pub r0: f64,
}

pub fn parse_pie_layout(series: Option<&OptionValue>, width: f64, height: f64) -> PieLayout {
    let (cx_v, cy_v) = match series.and_then(|s| s.get("center")) {
        Some(OptionValue::Array(arr)) => (arr.first(), arr.get(1)),
        Some(v) => (Some(v), Some(v)),
        None => (None, None),
    };
    let cx = parse_percent(cx_v, width, width * 0.5);
    let cy = parse_percent(cy_v, height, height * 0.5);
    let size = width.min(height);
    let half = size / 2.0;
    let (r0, r) = match series.and_then(|s| s.get("radius")) {
        Some(OptionValue::Array(arr)) => (
            parse_percent(arr.first(), half, 0.0),
            parse_percent(arr.get(1), half, half * 0.5),
        ),
        Some(v) => (0.0, parse_percent(Some(v), half, half * 0.5)),
        None => (0.0, half * 0.5),
    };
    let start_deg = series
        .and_then(|s| s.get("startAngle"))
        .and_then(|v| v.as_f64())
        .unwrap_or(DEFAULT_START_ANGLE_DEG);
    let clockwise = series
        .and_then(|s| s.get("clockwise"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    PieLayout {
        cx,
        cy,
        r: r.max(0.0),
        r0: r0.max(0.0),
        start_angle: -start_deg * PI / 180.0,
        clockwise,
    }
}

pub fn parse_rose_type(series: Option<&OptionValue>) -> Option<&'static str> {
    match series.and_then(|s| s.get("roseType")).and_then(|v| v.as_str()) {
        Some("radius") => Some("radius"),
        Some("area") => Some("area"),
        Some("true") => Some("radius"),
        _ => {
            if series
                .and_then(|s| s.get("roseType"))
                .and_then(|v| v.as_bool())
                == Some(true)
            {
                Some("radius")
            } else {
                None
            }
        }
    }
}

pub fn layout_pie_sectors(values: &[f64], layout: &PieLayout, rose: Option<&str>) -> Vec<PieSectorLayout> {
    let finite: Vec<(usize, f64)> = values
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_finite() && **v >= 0.0)
        .map(|(i, v)| (i, *v))
        .collect();
    let valid = finite.len().max(1);
    let sum: f64 = finite.iter().map(|(_, v)| *v).sum();
    let max_v = finite.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max);
    let dir = if layout.clockwise { 1.0 } else { -1.0 };
    let mut angle = layout.start_angle;
    let mut out = vec![
        PieSectorLayout {
            start_angle: f64::NAN,
            end_angle: f64::NAN,
            r: layout.r,
            r0: layout.r0,
        };
        values.len()
    ];
    for (i, value) in values.iter().enumerate() {
        if !value.is_finite() || *value < 0.0 {
            continue;
        }
        let sweep = if rose == Some("area") {
            PI * 2.0 / valid as f64
        } else if sum > 0.0 {
            *value / sum * PI * 2.0
        } else {
            PI * 2.0 / valid as f64
        };
        let start = angle;
        let end = angle + dir * sweep;
        angle = end;
        let r = if rose.is_some() {
            if max_v <= 0.0 {
                layout.r0
            } else {
                layout.r0 + (layout.r - layout.r0) * (*value / max_v)
            }
        } else {
            layout.r
        };
        out[i] = PieSectorLayout {
            start_angle: start,
            end_angle: end,
            r,
            r0: layout.r0,
        };
    }
    out
}

fn selected_mode_on(series_opt: Option<&OptionValue>) -> bool {
    match series_opt.and_then(|s| s.get("selectedMode")) {
        Some(OptionValue::Bool(b)) => *b,
        Some(OptionValue::String(s)) if s != "false" && !s.is_empty() => true,
        _ => false,
    }
}

fn item_selected(
    series: &SeriesModel,
    data_index: usize,
    interaction: &InteractionState,
    mode_on: bool,
) -> bool {
    if !mode_on {
        return false;
    }
    if interaction.selected.contains(&DataTarget {
        series_index: series.index as i32,
        data_index: data_index as i32,
    }) {
        return true;
    }
    series
        .data
        .get(data_index)
        .and_then(|p| p.raw.get("selected"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub fn render_pie_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
    interaction: &InteractionState,
) {
    if series.data.is_empty() {
        return;
    }

    let series_opt = visual.series_option(series.index);
    let layout = parse_pie_layout(series_opt, model.width as f64, model.height as f64);
    let rose = parse_rose_type(series_opt);
    let values: Vec<f64> = series
        .data
        .iter()
        .map(|p| {
            let name = p.name.as_deref().unwrap_or("");
            if !name.is_empty() && !interaction.is_name_selected(name) {
                0.0
            } else {
                p.value
            }
        })
        .collect();
    let sectors = layout_pie_sectors(&values, &layout, rose);
    let mode_on = selected_mode_on(series_opt);
    let selected_offset = series_opt
        .and_then(|s| s.get("selectedOffset"))
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let min_label_angle = series_opt
        .and_then(|s| s.get("minShowLabelAngle"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
        * PI
        / 180.0;
    let show_label = visual.label_visible(series.index);
    let show_label_line = show_label
        && series_opt
            .and_then(|s| s.get("labelLine"))
            .and_then(|l| l.get("show"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

    let mut occupied: Vec<(i8, f64)> = Vec::new();

    for (i, point) in series.data.iter().enumerate() {
        let sec = sectors[i];
        if !sec.start_angle.is_finite() {
            continue;
        }
        let sweep = (sec.end_angle - sec.start_angle).abs();
        if sweep <= f64::EPSILON {
            continue;
        }
        let selected = item_selected(series, i, interaction, mode_on);
        let mid = (sec.start_angle + sec.end_angle) / 2.0;
        let (dx, dy) = if selected {
            (mid.cos() * selected_offset, mid.sin() * selected_offset)
        } else {
            (0.0, 0.0)
        };
        let cx = layout.cx + dx;
        let cy = layout.cy + dy;
        let color = visual.resolve_item_color(series.index, i);
        let sector = zr.storage.create_path(
            Path::new(
                Shape::Sector(SectorShape {
                    cx,
                    cy,
                    r: sec.r,
                    r0: sec.r0,
                    start_angle: sec.start_angle,
                    end_angle: sec.end_angle,
                    clockwise: layout.clockwise,
                    ..Default::default()
                }),
                PathStyle {
                    fill: FillStrokeStyle::color(&color),
                    stroke: FillStrokeStyle::color("#fff"),
                    line_width: 1.0,
                    ..Default::default()
                },
            )
            .with_displayable(DisplayableProps {
                z: series.index as f64,
                ..Default::default()
            })
            .with_ec_data(EcData::new(series.index as i32, i as i32)),
        );
        zr.storage.group_add_child(group, ChildRef::Path(sector));

        zr.set_path_state_style(
            sector,
            STATE_EMPHASIS,
            PathStylePatch {
                fill: Some(FillStrokeStyle::color(&color)),
                line_width: Some(2.0),
                ..Default::default()
            },
        );
        zr.set_path_state_style(
            sector,
            STATE_SELECT,
            PathStylePatch {
                stroke: Some(FillStrokeStyle::color("#333")),
                line_width: Some(2.0),
                ..Default::default()
            },
        );

        let show_this_label = show_label && sweep + 1e-9 >= min_label_angle;
        let cos = mid.cos();
        let sin = mid.sin();
        let label_r = sec.r + 16.0;
        let lx = cx + label_r * cos;
        let ly = cy + label_r * sin;
        let side: i8 = if cos >= 0.0 { 1 } else { -1 };
        let overlapping = occupied
            .iter()
            .any(|(s, y)| *s == side && (ly - *y).abs() < 14.0);
        if show_this_label && !overlapping {
            occupied.push((side, ly));
            if show_label_line {
                let x1 = cx + sec.r * cos;
                let y1 = cy + sec.r * sin;
                let line = zr.storage.create_path(Path::new(
                    Shape::Line(LineShape {
                        x1,
                        y1,
                        x2: lx,
                        y2: ly,
                        percent: 1.0,
                    }),
                    PathStyle {
                        fill: FillStrokeStyle::none(),
                        stroke: FillStrokeStyle::color("#999"),
                        line_width: 1.0,
                        ..Default::default()
                    },
                ));
                zr.storage.group_add_child(group, ChildRef::Path(line));
            }
            let align = if cos > 0.15 {
                TextAlign::Left
            } else if cos < -0.15 {
                TextAlign::Right
            } else {
                TextAlign::Center
            };
            add_label(
                zr,
                group,
                visual,
                series.index,
                i,
                lx,
                ly,
                align,
                TextBaseline::Middle,
                &color,
                series.index as f64 + 0.2,
            );
        }
        let _ = point;
    }
}

/// 计算扇区角度（供单元测试）
pub fn pie_sector_angles(values: &[f64], start: f64) -> Vec<(f64, f64)> {
    let layout = PieLayout {
        cx: 0.0,
        cy: 0.0,
        r: 1.0,
        r0: 0.0,
        start_angle: start,
        clockwise: true,
    };
    layout_pie_sectors(values, &layout, None)
        .into_iter()
        .filter(|s| s.start_angle.is_finite())
        .map(|s| (s.start_angle, s.end_angle))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::option::OptionValue;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn sectors_sum_to_full_circle() {
        let angles = pie_sector_angles(&[30.0, 70.0, 100.0], 0.0);
        assert_eq!(angles.len(), 3);
        let last_end = angles.last().unwrap().1;
        assert!((last_end - PI * 2.0).abs() < 0.001);
    }

    #[test]
    fn radius_percent_uses_half_min_side() {
        let series = obj(vec![("radius", OptionValue::String("50%".into()))]);
        let layout = parse_pie_layout(Some(&series), 400.0, 200.0);
        assert!((layout.r - 50.0).abs() < 1e-9);
        assert!((layout.r0 - 0.0).abs() < 1e-9);
        assert!((layout.cx - 200.0).abs() < 1e-9);
        assert!((layout.cy - 100.0).abs() < 1e-9);
    }

    #[test]
    fn donut_and_start_angle() {
        let series = obj(vec![
            (
                "radius",
                OptionValue::Array(vec![
                    OptionValue::String("30%".into()),
                    OptionValue::String("60%".into()),
                ]),
            ),
            (
                "center",
                OptionValue::Array(vec![
                    OptionValue::String("40%".into()),
                    OptionValue::Number(80.0),
                ]),
            ),
            ("startAngle", OptionValue::Number(90.0)),
            ("clockwise", OptionValue::Bool(false)),
        ]);
        let layout = parse_pie_layout(Some(&series), 200.0, 200.0);
        assert!((layout.r0 - 30.0).abs() < 1e-9);
        assert!((layout.r - 60.0).abs() < 1e-9);
        assert!((layout.cx - 80.0).abs() < 1e-9);
        assert!((layout.cy - 80.0).abs() < 1e-9);
        assert!((layout.start_angle + PI / 2.0).abs() < 1e-9);
        assert!(!layout.clockwise);
    }

    #[test]
    fn rose_radius_maps_r_by_value() {
        let layout = PieLayout {
            cx: 0.0,
            cy: 0.0,
            r: 100.0,
            r0: 0.0,
            start_angle: 0.0,
            clockwise: true,
        };
        let secs = layout_pie_sectors(&[10.0, 20.0], &layout, Some("radius"));
        assert!((secs[0].r - 50.0).abs() < 1e-9);
        assert!((secs[1].r - 100.0).abs() < 1e-9);
        assert!((secs[0].end_angle - secs[0].start_angle).abs() < (secs[1].end_angle - secs[1].start_angle).abs());
    }

    #[test]
    fn rose_area_equal_angles() {
        let layout = PieLayout {
            cx: 0.0,
            cy: 0.0,
            r: 80.0,
            r0: 20.0,
            start_angle: 0.0,
            clockwise: true,
        };
        let secs = layout_pie_sectors(&[1.0, 3.0], &layout, Some("area"));
        let a0 = (secs[0].end_angle - secs[0].start_angle).abs();
        let a1 = (secs[1].end_angle - secs[1].start_angle).abs();
        assert!((a0 - a1).abs() < 1e-9);
        assert!(secs[1].r > secs[0].r);
    }

    #[test]
    fn selected_mode_offsets_sector_center() {
        use crate::option::{OptionModel, SetOptionFlags};
        use rust_zrender::ZRenderer;

        let root = obj(vec![
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("pie".into())),
                    ("selectedMode", OptionValue::Bool(true)),
                    ("selectedOffset", OptionValue::Number(20.0)),
                    ("label", obj(vec![("show", OptionValue::Bool(false))])),
                    (
                        "data",
                        OptionValue::Array(vec![
                            obj(vec![
                                ("value", OptionValue::Number(40.0)),
                                ("name", OptionValue::String("A".into())),
                                ("selected", OptionValue::Bool(true)),
                            ]),
                            obj(vec![
                                ("value", OptionValue::Number(60.0)),
                                ("name", OptionValue::String("B".into())),
                            ]),
                        ]),
                    ),
                ])]),
            ),
        ]);
        let mut option = OptionModel::new();
        option.apply(
            root,
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = crate::model::GlobalModel::from_option(&option, 200, 200);
        let visual = crate::visual::VisualContext::new(&option, &model);
        let interaction = InteractionState::default();
        let mut zr = ZRenderer::new(200, 200).unwrap();
        let group = zr.storage.create_group();
        render_pie_series(&mut zr, group, &model, &visual, &model.series[0], &interaction);
        let sectors: Vec<&SectorShape> = zr
            .storage
            .paths()
            .iter()
            .filter_map(|p| match &p.shape {
                Shape::Sector(s) => Some(s),
                _ => None,
            })
            .collect();
        assert_eq!(sectors.len(), 2);
        assert!(
            (sectors[0].cx - 100.0).abs() > 1.0 || (sectors[0].cy - 100.0).abs() > 1.0,
            "selected slice should leave center"
        );
        assert!((sectors[1].cx - 100.0).abs() < 1e-6);
        assert!((sectors[1].cy - 100.0).abs() < 1e-6);
    }
}
