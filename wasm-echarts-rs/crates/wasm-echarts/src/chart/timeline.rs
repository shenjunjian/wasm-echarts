//! timeline：canvas 滑条，currentIndex 切 options 终态

use rust_zrender::{FillStrokeStyle, TextAlign, TextBaseline, ZRenderer};

use crate::chart::layout::{add_rect, add_text, component_ec, layout_origin, HIT_TIMELINE};
use crate::chart::text_opt::parse_chart_text_style;
use crate::interaction::InteractionState;
use crate::model::GlobalModel;
use crate::option::{OptionModel, OptionValue};
use crate::utils::first_component;

pub fn timeline_component(option: &OptionModel) -> Option<&OptionValue> {
    first_component(option.root().get("timeline")).or_else(|| {
        option
            .root()
            .get("baseOption")
            .and_then(|b| first_component(b.get("timeline")))
    })
}

pub fn timeline_count(option: &OptionModel) -> usize {
    if let Some(data) = timeline_component(option)
        .and_then(|t| t.get("data"))
        .and_then(|v| v.as_array())
    {
        return data.len();
    }
    option
        .root()
        .get("options")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0)
}

pub fn render_timeline(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
    interaction: &InteractionState,
) {
    let Some(tl) = timeline_component(option) else {
        return;
    };
    if tl.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }
    let n = timeline_count(option);
    if n == 0 {
        return;
    }
    let labels: Vec<String> = tl
        .get("data")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(item_label).collect())
        .unwrap_or_else(|| (0..n).map(|i| i.to_string()).collect());
    let width = tl
        .get("width")
        .and_then(|v| v.as_f64())
        .unwrap_or(model.width as f64 * 0.7);
    let height = 28.0;
    let (x, y) = layout_origin(
        tl,
        model.width as f64,
        model.height as f64,
        width,
        height,
        (model.width as f64 - width) / 2.0,
        model.height as f64 - height - 6.0,
    );
    add_rect(
        zr,
        group,
        x,
        y + height / 2.0 - 1.0,
        width,
        2.0,
        FillStrokeStyle::color("#d4d7dc"),
        FillStrokeStyle::none(),
        0.0,
        21.0,
        None,
    );
    let style = parse_chart_text_style(tl.get("label"), "#6e7079", 11.0, "sans-serif");
    let current = interaction.timeline_index.min(n.saturating_sub(1));
    for i in 0..n {
        let cx = if n == 1 {
            x + width / 2.0
        } else {
            x + width * i as f64 / (n - 1) as f64
        };
        let r = if i == current { 6.0 } else { 4.0 };
        let color = if i == current { "#5470c6" } else { "#a0a4aa" };
        add_rect(
            zr,
            group,
            cx - r,
            y + height / 2.0 - r,
            r * 2.0,
            r * 2.0,
            FillStrokeStyle::color(color),
            FillStrokeStyle::none(),
            0.0,
            21.2,
            Some(component_ec(HIT_TIMELINE, i as i32)),
        );
        if let Some(label) = labels.get(i) {
            add_text(
                zr,
                group,
                label,
                cx,
                y + height / 2.0 + 8.0,
                style.to_text_style(TextAlign::Center, TextBaseline::Top),
                21.2,
                false,
                Some(component_ec(HIT_TIMELINE, i as i32)),
            );
        }
    }
}

fn item_label(item: &OptionValue) -> String {
    match item {
        OptionValue::String(s) => s.clone(),
        OptionValue::Number(v) => v.to_string(),
        OptionValue::Object(_) => item
            .get("name")
            .or_else(|| item.get("value"))
            .and_then(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| v.as_f64().map(|n| n.to_string()))
            })
            .unwrap_or_default(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::option::SetOptionFlags;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn timeline_hits() {
        let mut option = OptionModel::new();
        option.apply(
            obj(vec![
                (
                    "timeline",
                    obj(vec![(
                        "data",
                        OptionValue::Array(vec![
                            OptionValue::String("2002".into()),
                            OptionValue::String("2003".into()),
                        ]),
                    )]),
                ),
                (
                    "options",
                    OptionValue::Array(vec![
                        obj(vec![("title", obj(vec![("text", OptionValue::String("A".into()))]))]),
                        obj(vec![("title", obj(vec![("text", OptionValue::String("B".into()))]))]),
                    ]),
                ),
            ]),
            SetOptionFlags {
                not_merge: true,
                replace_merge: vec![],
            },
        );
        let interaction = InteractionState::from_option(&option);
        let model = GlobalModel::from_option(&option, 400, 240);
        let mut zr = ZRenderer::new(400, 240).unwrap();
        let group = zr.storage.create_group();
        render_timeline(&mut zr, group, &model, &option, &interaction);
        let n = zr
            .storage
            .paths()
            .iter()
            .filter(|p| p.ec_data.data_type.as_deref() == Some(HIT_TIMELINE))
            .count();
        assert_eq!(n, 2);
    }
}
