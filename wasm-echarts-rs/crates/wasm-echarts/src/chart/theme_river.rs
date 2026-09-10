//! 主题河流：singleAxis 上按名称堆叠的面积带

use rust_zrender::{PolygonShape, Shape, ZRenderer};

use crate::chart::path_util::{add_ec_path, fill_stroke, raw_numbers};
use crate::coord::SingleCoord;
use crate::model::{GlobalModel, SeriesModel};
use crate::option::OptionValue;
use crate::visual::VisualContext;

pub fn render_theme_river_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let Some(coord) = SingleCoord::for_series(model, series) else {
        return;
    };
    let mut groups: Vec<(String, Vec<(f64, f64)>)> = Vec::new();
    for (i, point) in series.data.iter().enumerate() {
        let nums = raw_numbers(&point.raw);
        let time = if nums.len() >= 2 {
            nums[0]
        } else {
            point.x_value.unwrap_or(i as f64)
        };
        let value = if nums.len() >= 2 { nums[1] } else { point.value };
        let name = point
            .name
            .clone()
            .or_else(|| {
                if let OptionValue::Array(arr) = &point.raw {
                    arr.get(2).and_then(|v| v.as_str()).map(str::to_string)
                } else if let OptionValue::Object(obj) = &point.raw {
                    obj.get("name").and_then(|v| v.as_str()).map(str::to_string)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "stream".into());
        if let Some((_, pts)) = groups.iter_mut().find(|(n, _)| n == &name) {
            pts.push((time, value));
        } else {
            groups.push((name, vec![(time, value)]));
        }
    }
    for pts in groups.iter_mut() {
        pts.1.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    }
    let mut times: Vec<f64> = groups.iter().flat_map(|(_, p)| p.iter().map(|t| t.0)).collect();
    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    times.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
    if times.is_empty() {
        return;
    }
    let mut stack = vec![0.0; times.len()];
    for (gi, (_name, pts)) in groups.iter().enumerate() {
        let mut top = Vec::new();
        let mut bottom = Vec::new();
        for (ti, t) in times.iter().enumerate() {
            let v = pts
                .iter()
                .find(|(tt, _)| (*tt - *t).abs() < 1e-6)
                .map(|(_, v)| *v)
                .unwrap_or(0.0);
            let base = stack[ti];
            stack[ti] += v.max(0.0);
            let (x, _) = coord.data_to_point(*t);
            let y0 = river_y(coord.spec.rect.y, coord.spec.rect.height, base);
            let y1 = river_y(coord.spec.rect.y, coord.spec.rect.height, stack[ti]);
            bottom.push((x, y0));
            top.push((x, y1));
        }
        let mut poly = top;
        poly.extend(bottom.into_iter().rev());
        let color = visual.resolve_item_color(series.index, gi.min(series.data.len().saturating_sub(1)));
        add_ec_path(
            zr,
            group,
            Shape::Polygon(PolygonShape {
                points: poly,
                smooth: 0.4,
                ..Default::default()
            }),
            fill_stroke(&color, "none", 0.0, 0.7),
            series.index,
            gi,
            series.index as f64,
            true,
        );
    }
}

fn river_y(top: f64, height: f64, value: f64) -> f64 {
    top + height * 0.5 - value
}
