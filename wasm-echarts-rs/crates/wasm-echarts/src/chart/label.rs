//! series.label Text 图元

use rust_zrender::{
    ChildRef, Text, TextAlign, TextBaseline, TextStyle, ZRenderer,
};

use crate::visual::VisualContext;

pub fn add_label(
    zr: &mut ZRenderer,
    group: usize,
    visual: &VisualContext,
    series_index: usize,
    data_index: usize,
    x: f64,
    y: f64,
    align: TextAlign,
    baseline: TextBaseline,
    color: &str,
    z: f64,
) {
    let Some(text) = visual.resolve_label(series_index, data_index) else {
        return;
    };
    if text.is_empty() {
        return;
    }
    let layout = crate::chart::label_layout::resolve_label_layout(visual, series_index, data_index);
    let x = layout.x.unwrap_or(x) + layout.dx;
    let y = layout.y.unwrap_or(y) + layout.dy;
    let align = layout.align.unwrap_or(align);
    let baseline = layout.baseline.unwrap_or(baseline);
    let idx = zr.storage.create_text(
        Text::new(text, x, y)
            .with_style(TextStyle {
                fill: color.into(),
                font_size: 12.0,
                align,
                baseline,
                ..Default::default()
            })
            .with_displayable(rust_zrender::DisplayableProps {
                z,
                ..Default::default()
            }),
    );
    zr.storage.text_mut(idx).silent = true;
    zr.storage.text_mut(idx).base.name = crate::chart::label_layout::series_label_name(series_index);
    zr.storage.group_add_child(group, ChildRef::Text(idx));
}

pub fn add_plain_label(
    zr: &mut ZRenderer,
    group: usize,
    text: String,
    x: f64,
    y: f64,
    align: TextAlign,
    baseline: TextBaseline,
    color: &str,
    z: f64,
) {
    if text.is_empty() {
        return;
    }
    let idx = zr.storage.create_text(
        Text::new(text, x, y)
            .with_style(TextStyle {
                fill: color.into(),
                font_size: 12.0,
                align,
                baseline,
                ..Default::default()
            })
            .with_displayable(rust_zrender::DisplayableProps {
                z,
                ..Default::default()
            }),
    );
    zr.storage.text_mut(idx).silent = true;
    zr.storage.group_add_child(group, ChildRef::Text(idx));
}
