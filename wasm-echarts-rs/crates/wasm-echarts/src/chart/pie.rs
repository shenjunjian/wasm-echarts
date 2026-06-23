//! 饼图 ChartView

use std::f64::consts::PI;

use rust_zrender::{
    ChildRef, DisplayableProps, EcData, FillStrokeStyle, Path, PathStyle, SectorShape, Shape,
    PathStylePatch, STATE_EMPHASIS, STATE_SELECT, ZRenderer,
};

use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

/// ECharts 默认 startAngle=90°，顺时针
const DEFAULT_START_ANGLE: f64 = PI / 2.0;

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

    let g = model.grid;
    let cx = g.x + g.width / 2.0;
    let cy = g.y + g.height / 2.0;
    let r = g.width.min(g.height) * 0.38;

    let mut angle = -DEFAULT_START_ANGLE;

    for (i, point) in series.data.iter().enumerate() {
        let sweep = point.value / total * PI * 2.0;
        if sweep <= f64::EPSILON {
            continue;
        }
        let start = angle;
        let end = angle + sweep;
        angle = end;

        let color = visual.resolve_item_color(series.index, i);
        let sector = zr.storage.create_path(
            Path::new(
                Shape::Sector(SectorShape {
                    cx,
                    cy,
                    r,
                    start_angle: start,
                    end_angle: end,
                    percent: 1.0,
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

    #[test]
    fn sectors_sum_to_full_circle() {
        let angles = pie_sector_angles(&[30.0, 70.0, 100.0], 0.0);
        assert_eq!(angles.len(), 3);
        let last_end = angles.last().unwrap().1;
        assert!((last_end - PI * 2.0).abs() < 0.001);
    }
}
