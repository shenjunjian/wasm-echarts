//! 地图填充 + 航线：挂 geo / cartesian

use rust_zrender::{PolygonShape, PolylineShape, Shape, TextAlign, TextBaseline, ZRenderer};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke};
use crate::chart::visual_map;
use crate::coord::{GeoCoord, SeriesCoord};
use crate::model::{GlobalModel, SeriesModel};
use crate::option::OptionValue;
use crate::visual::VisualContext;

pub fn render_map_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let Some(coord) = GeoCoord::for_series(model, series) else {
        return;
    };
    let mut by_name: std::collections::HashMap<String, (usize, f64)> = std::collections::HashMap::new();
    for (i, point) in series.data.iter().enumerate() {
        if let Some(name) = point.name.clone() {
            by_name.insert(name, (i, point.value));
        }
    }
    for region in coord.regions() {
        let (data_index, value) = by_name
            .get(&region.name)
            .copied()
            .unwrap_or((0, f64::NAN));
        let color = if value.is_finite() {
            visual_map::map_color(visual.option(), series.index, data_index, series)
                .or_else(|| visual_map::color_for_value(visual.option(), series.index, value))
                .unwrap_or_else(|| visual.resolve_item_color(series.index, data_index))
        } else {
            visual
                .series_option(series.index)
                .and_then(|s| s.get("itemStyle"))
                .and_then(|s| s.get("areaColor").or_else(|| s.get("color")))
                .and_then(|v| v.as_str())
                .unwrap_or("#eee")
                .to_string()
        };
        for poly in &region.polygons {
            let pts = coord.project_polygon(poly);
            if pts.len() < 3 {
                continue;
            }
            add_ec_path(
                zr,
                group,
                Shape::Polygon(PolygonShape {
                    points: pts,
                    ..Default::default()
                }),
                fill_stroke(&color, "#fff", 1.0, 1.0),
                series.index,
                data_index,
                series.index as f64,
                true,
            );
        }
        if let Some((px, py)) = coord.named_point(&region.name) {
            add_label(
                zr,
                group,
                visual,
                series.index,
                data_index,
                px,
                py,
                TextAlign::Center,
                TextBaseline::Middle,
                "#333",
                series.index as f64 + 0.2,
            );
        }
    }
}

pub fn render_lines_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let geo = GeoCoord::for_series(model, series);
    let series_opt = visual.series_option(series.index);
    let lw = series_opt
        .and_then(|s| s.get("lineStyle"))
        .and_then(|s| s.get("width"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.5) as f32;
    for (i, point) in series.data.iter().enumerate() {
        let pts = line_points(&point.raw, coord, geo.as_ref());
        if pts.len() < 2 {
            continue;
        }
        let color = visual.resolve_item_color(series.index, i);
        add_ec_path(
            zr,
            group,
            Shape::Polyline(PolylineShape {
                points: pts,
                percent: 1.0,
                smooth: 0.0,
                ..Default::default()
            }),
            fill_stroke("none", &color, lw, 0.85),
            series.index,
            i,
            series.index as f64,
            true,
        );
    }
}

fn line_points(
    raw: &OptionValue,
    coord: &SeriesCoord,
    geo: Option<&GeoCoord>,
) -> Vec<(f64, f64)> {
    let coords = match raw {
        OptionValue::Object(obj) => obj.get("coords"),
        OptionValue::Array(arr) if arr.first().and_then(|v| v.as_array()).is_some() => Some(raw),
        _ => None,
    };
    let Some(OptionValue::Array(arr)) = coords else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|p| {
            let pair = p.as_array()?;
            let a = pair.first().and_then(|v| v.as_f64())?;
            let b = pair.get(1).and_then(|v| v.as_f64())?;
            if let Some(geo) = geo {
                return Some(geo.data_to_point(a, b));
            }
            Some(coord.point_for(0, Some(a), b))
        })
        .collect()
}
