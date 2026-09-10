//! 散点图 ChartView：large 终态一次画完 + 其余 symbol

use rust_zrender::{TextAlign, TextBaseline, ZRenderer};

use crate::chart::label::add_label;
use crate::chart::symbol::{add_symbol, SymbolSpec};
use crate::coord::SeriesCoord;
use crate::model::{GlobalModel, SeriesModel};
use crate::visual::VisualContext;

pub fn is_large_scatter(visual: &VisualContext, series: &SeriesModel) -> bool {
    let opt = visual.series_option(series.index);
    let large = opt
        .and_then(|s| s.get("large"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let threshold = opt
        .and_then(|s| s.get("largeThreshold"))
        .and_then(|v| v.as_f64())
        .unwrap_or(2000.0) as usize;
    large && series.data.len() >= threshold
}

pub fn render_scatter_series(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    coord: &SeriesCoord,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let kind = visual.resolve_symbol(series.index);
    let large = is_large_scatter(visual, series);
    let mut jitter = crate::chart::jitter::JitterState::default();
    for (i, point) in series.data.iter().enumerate() {
        if !point.value.is_finite() {
            continue;
        }
        let (cx, cy) = coord.map_point(point, i);
        let size = visual.resolve_symbol_size_of(series.index, i);
        let (cx, cy) = crate::chart::jitter::apply_scatter_jitter(
            &mut jitter, coord, series, i, size, cx, cy,
        );
        if !cx.is_finite() || !cy.is_finite() {
            continue;
        }
        let color = visual.resolve_item_color(series.index, i);
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
                attach_states: !large,
            },
        );
        if !large {
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
        }
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

    fn render(root: OptionValue) -> ZRenderer {
        let mut option = OptionModel::new();
        option.apply(
            root,
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = crate::model::GlobalModel::from_option(&option, 200, 160);
        let visual = VisualContext::new(&option, &model);
        let mut zr = ZRenderer::new(200, 160).unwrap();
        let group = zr.storage.create_group();
        let series = &model.series[0];
        let coord = SeriesCoord::for_series(&model, series);
        render_scatter_series(&mut zr, group, &model, &coord, &visual, series);
        zr
    }

    #[test]
    fn diamond_symbol_is_polygon() {
        let zr = render(obj(vec![
            ("xAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("scatter".into())),
                    ("symbol", OptionValue::String("diamond".into())),
                    (
                        "data",
                        OptionValue::Array(vec![OptionValue::Array(vec![
                            OptionValue::Number(1.0),
                            OptionValue::Number(2.0),
                        ])]),
                    ),
                ])]),
            ),
        ]));
        assert!(zr.storage.paths().iter().any(|p| matches!(
            p.shape,
            rust_zrender::Shape::Polygon(_)
        )));
    }

    #[test]
    fn large_mode_skips_emphasis_states() {
        let mut data = Vec::new();
        for i in 0..8 {
            data.push(OptionValue::Array(vec![
                OptionValue::Number(i as f64),
                OptionValue::Number((i % 3) as f64),
            ]));
        }
        let zr = render(obj(vec![
            ("xAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            ("yAxis", obj(vec![("type", OptionValue::String("value".into()))])),
            (
                "series",
                OptionValue::Array(vec![obj(vec![
                    ("type", OptionValue::String("scatter".into())),
                    ("large", OptionValue::Bool(true)),
                    ("largeThreshold", OptionValue::Number(5.0)),
                    ("data", OptionValue::Array(data)),
                ])]),
            ),
        ]));
        assert_eq!(zr.storage.paths().len(), 8);
        assert!(
            zr.storage.paths().iter().all(|p| !p.states.patches.contains_key("emphasis")),
            "large mode should not attach emphasis states"
        );
    }
}
