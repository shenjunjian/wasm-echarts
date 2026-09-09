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
