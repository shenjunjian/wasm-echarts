//! dataZoom slider（canvas）+ inside 已由滚轮消费 start/end/xAxisIndex

use rust_zrender::{FillStrokeStyle, ZRenderer};

use crate::chart::layout::{add_rect, component_ec, HIT_DATA_ZOOM};
use crate::interaction::InteractionState;
use crate::model::GlobalModel;
use crate::option::OptionModel;
use crate::utils::{as_components, parse_percent};

/// 滑条几何：filler 为 data_index=2，start handle=0，end handle=1。
#[derive(Debug, Clone, Copy)]
pub struct SliderGeom {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn slider_geom(model: &GlobalModel, option: &OptionModel) -> Option<SliderGeom> {
    let slider = as_components(option.root().get("dataZoom"))
        .into_iter()
        .find(|c| c.get("type").and_then(|v| v.as_str()).unwrap_or("slider") != "inside")?;
    if slider.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return None;
    }
    let g = model.grid();
    let height = slider
        .get("height")
        .and_then(|v| v.as_f64())
        .unwrap_or(24.0);
    let width = slider
        .get("width")
        .and_then(|v| v.as_f64())
        .unwrap_or(g.width);
    let x = parse_percent(slider.get("left"), model.width as f64, g.x);
    let y = if slider.get("top").is_some() {
        parse_percent(slider.get("top"), model.height as f64, g.y + g.height + 8.0)
    } else {
        parse_percent(
            slider.get("bottom").or(Some(&crate::option::OptionValue::Number(8.0))),
            model.height as f64,
            8.0,
        );
        model.height as f64 - height - 8.0
    };
    Some(SliderGeom { x, y, width, height })
}

pub fn render_data_zoom_slider(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    if !interaction.data_zoom_has_slider {
        return;
    }
    let Some(geom) = slider_geom(model, option).or_else(|| {
        let g = model.grid();
        Some(SliderGeom {
            x: g.x,
            y: model.height as f64 - 32.0,
            width: g.width,
            height: 24.0,
        })
    }) else {
        return;
    };
    add_rect(
        zr,
        group,
        geom.x,
        geom.y,
        geom.width,
        geom.height,
        FillStrokeStyle::color("rgba(47,69,84,0.08)"),
        FillStrokeStyle::color("#d4d7dc"),
        1.0,
        20.0,
        Some(component_ec(HIT_DATA_ZOOM, 2)),
    );
    let start_x = geom.x + geom.width * interaction.data_zoom.start / 100.0;
    let end_x = geom.x + geom.width * interaction.data_zoom.end / 100.0;
    let filler_w = (end_x - start_x).max(4.0);
    add_rect(
        zr,
        group,
        start_x,
        geom.y,
        filler_w,
        geom.height,
        FillStrokeStyle::color("rgba(84,112,198,0.25)"),
        FillStrokeStyle::none(),
        0.0,
        20.1,
        Some(component_ec(HIT_DATA_ZOOM, 2)),
    );
    let handle_w = 8.0;
    add_rect(
        zr,
        group,
        start_x - handle_w / 2.0,
        geom.y - 2.0,
        handle_w,
        geom.height + 4.0,
        FillStrokeStyle::color("#5470c6"),
        FillStrokeStyle::none(),
        0.0,
        20.2,
        Some(component_ec(HIT_DATA_ZOOM, 0)),
    );
    add_rect(
        zr,
        group,
        end_x - handle_w / 2.0,
        geom.y - 2.0,
        handle_w,
        geom.height + 4.0,
        FillStrokeStyle::color("#5470c6"),
        FillStrokeStyle::none(),
        0.0,
        20.2,
        Some(component_ec(HIT_DATA_ZOOM, 1)),
    );
}

pub fn hit_slider_kind(data_index: i32) -> Option<crate::interaction::DragKind> {
    match data_index {
        0 => Some(crate::interaction::DragKind::ZoomStart),
        1 => Some(crate::interaction::DragKind::ZoomEnd),
        2 => Some(crate::interaction::DragKind::ZoomFiller),
        _ => None,
    }
}

pub fn apply_slider_drag(
    interaction: &mut InteractionState,
    geom: SliderGeom,
    x: f64,
) {
    let Some(drag) = interaction.drag else {
        return;
    };
    let ratio = if geom.width > 0.0 {
        ((x - geom.x) / geom.width).clamp(0.0, 1.0) * 100.0
    } else {
        0.0
    };
    match drag.kind {
        crate::interaction::DragKind::ZoomStart => {
            interaction.data_zoom.start = ratio.min(interaction.data_zoom.end - 1.0).max(0.0);
        }
        crate::interaction::DragKind::ZoomEnd => {
            interaction.data_zoom.end = ratio.max(interaction.data_zoom.start + 1.0).min(100.0);
        }
        crate::interaction::DragKind::ZoomFiller => {
            let dx = (x - drag.start_x) / geom.width.max(1.0) * 100.0;
            interaction.data_zoom = drag.start_zoom;
            interaction.data_zoom.pan(dx);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::InteractionState;
    use crate::option::{OptionValue, SetOptionFlags};
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn slider_draws_handles() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "dataZoom",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("slider".into())),
                        ("start", OptionValue::Number(20.0)),
                        ("end", OptionValue::Number(80.0)),
                    ])]),
                ),
                (
                    "xAxis",
                    obj(vec![("type", OptionValue::String("category".into()))]),
                ),
                (
                    "yAxis",
                    obj(vec![("type", OptionValue::String("value".into()))]),
                ),
                (
                    "series",
                    OptionValue::Array(vec![obj(vec![
                        ("type", OptionValue::String("line".into())),
                        ("data", OptionValue::Array(vec![OptionValue::Number(1.0)])),
                    ])]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let interaction = InteractionState::from_option(&option);
        let model = GlobalModel::from_option_with_zoom(&option, 400, 300, interaction.data_zoom);
        let mut zr = ZRenderer::new(400, 300).unwrap();
        let group = zr.storage.create_group();
        render_data_zoom_slider(&mut zr, group, &model, &option, &interaction);
        let handles = zr
            .storage
            .paths()
            .iter()
            .filter(|p| p.ec_data.data_type.as_deref() == Some(HIT_DATA_ZOOM))
            .count();
        assert!(handles >= 3, "slider hits {}", handles);
    }
}
