//! 饼图 ChartView

use std::f64::consts::PI;

use rust_zrender::{
    ChildRef, DisplayableProps, EcData, FillStrokeStyle, LineShape, Path, PathStyle, SectorShape,
    Shape, PathStylePatch, STATE_EMPHASIS, STATE_SELECT, TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_label;
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

pub fn render_pie_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    if series.data.is_empty() {
        return;
    }

    let total: f64 = series.data.iter().map(|p| p.value).sum();
    if total <= 0.0 {
        return;
    }

    let series_opt = visual.series_option(series.index);
    let layout = parse_pie_layout(series_opt, model.width as f64, model.height as f64);
    let dir = if layout.clockwise { 1.0 } else { -1.0 };
    let mut angle = layout.start_angle;
    let show_label = visual.label_visible(series.index);
    let show_label_line = show_label
        && series_opt
            .and_then(|s| s.get("labelLine"))
            .and_then(|l| l.get("show"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

    for (i, point) in series.data.iter().enumerate() {
        let sweep = point.value / total * PI * 2.0;
        if sweep <= f64::EPSILON {
            continue;
        }
        let start = angle;
        let end = angle + dir * sweep;
        angle = end;

        let color = visual.resolve_item_color(series.index, i);
        let sector = zr.storage.create_path(
            Path::new(
                Shape::Sector(SectorShape {
                    cx: layout.cx,
                    cy: layout.cy,
                    r: layout.r,
                    r0: layout.r0,
                    start_angle: start,
                    end_angle: end,
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

        let mid = (start + end) / 2.0;
        let cos = mid.cos();
        let sin = mid.sin();
        let label_r = layout.r + 16.0;
        let lx = layout.cx + label_r * cos;
        let ly = layout.cy + label_r * sin;
        if show_label_line {
            let x1 = layout.cx + layout.r * cos;
            let y1 = layout.cy + layout.r * sin;
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
        let _ = point;
    }
}

/// 计算扇区角度（供单元测试）
pub fn pie_sector_angles(values: &[f64], start: f64) -> Vec<(f64, f64)> {
    let total: f64 = values.iter().sum();
    if total <= 0.0 {
        return Vec::new();
    }
    let mut angle = start;
    values
        .iter()
        .filter(|&&v| v > 0.0)
        .map(|&v| {
            let sweep = v / total * PI * 2.0;
            let s = angle;
            angle += sweep;
            (s, angle)
        })
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
}
