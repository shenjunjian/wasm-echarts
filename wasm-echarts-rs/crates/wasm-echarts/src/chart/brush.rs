//! brush：canvas 框选矩形；拖拽期间画选区，松开后高亮落入点

use rust_zrender::{FillStrokeStyle, ZRenderer};

use crate::chart::layout::add_rect;
use crate::interaction::InteractionState;
use crate::model::GlobalModel;
use crate::option::OptionModel;
use crate::utils::first_component;

pub fn brush_enabled(option: &OptionModel) -> bool {
    first_component(option.root().get("brush")).is_some()
}

pub fn render_brush(
    zr: &mut ZRenderer,
    group: usize,
    _model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    if !brush_enabled(option) && interaction.brush_rect.is_none() {
        return;
    }
    let Some((x0, y0, x1, y1)) = interaction.brush_rect else {
        return;
    };
    let x = x0.min(x1);
    let y = y0.min(y1);
    let w = (x0 - x1).abs().max(1.0);
    let h = (y0 - y1).abs().max(1.0);
    add_rect(
        zr,
        group,
        x,
        y,
        w,
        h,
        FillStrokeStyle::color("rgba(120,170,255,0.2)"),
        FillStrokeStyle::color("#5470c6"),
        1.0,
        25.0,
        None,
    );
}

pub fn points_in_brush(
    model: &GlobalModel,
    rect: (f64, f64, f64, f64),
) -> Vec<crate::interaction::DataTarget> {
    let (x0, y0, x1, y1) = rect;
    let min_x = x0.min(x1);
    let max_x = x0.max(x1);
    let min_y = y0.min(y1);
    let max_y = y0.max(y1);
    let mut out = Vec::new();
    for series in &model.series {
        let coord = crate::coord::Cartesian2D::for_series(model, series);
        for (i, p) in series.data.iter().enumerate() {
            if !p.value.is_finite() {
                continue;
            }
            let (px, py) = coord.point_for(i, p.x_value, p.stacked_value);
            if px >= min_x && px <= max_x && py >= min_y && py <= max_y {
                out.push(crate::interaction::DataTarget {
                    series_index: series.index as i32,
                    data_index: i as i32,
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::GlobalModel;
    use crate::option::{OptionModel, OptionValue, SetOptionFlags};
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn brush_selects_points_in_rect() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                ("brush", obj(vec![])),
                (
                    "xAxis",
                    obj(vec![
                        ("type", OptionValue::String("category".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::String("a".into()),
                                OptionValue::String("b".into()),
                            ]),
                        ),
                    ]),
                ),
                (
                    "yAxis",
                    obj(vec![("type", OptionValue::String("value".into()))]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        (
                            "data",
                            OptionValue::Array(vec![
                                OptionValue::Number(1.0),
                                OptionValue::Number(2.0),
                            ]),
                        ),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let model = GlobalModel::from_option(&option, 400, 300);
        let g = model.grid();
        let hits = points_in_brush(&model, (g.x, g.y, g.x + g.width, g.y + g.height));
        assert_eq!(hits.len(), 2);
        assert!(brush_enabled(&option));
    }
}
