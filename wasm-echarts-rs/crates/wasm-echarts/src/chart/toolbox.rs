//! toolbox canvas 按钮：restore / magicType / dataZoom；DataView / SaveAsImage 只 warn

use rust_zrender::{
    ChildRef, CircleShape, DisplayableProps, FillStrokeStyle, LineShape, Path, PathStyle, RectShape,
    Shape, TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_plain_label;
use crate::chart::layout::{add_rect, component_ec, layout_origin, HIT_TOOLBOX};
use crate::model::GlobalModel;
use crate::option::OptionModel;
use crate::utils::first_component;

pub const TB_RESTORE: i32 = 0;
pub const TB_MAGIC: i32 = 1;
pub const TB_DATA_ZOOM: i32 = 2;
pub const TB_DATA_VIEW: i32 = 3;
pub const TB_SAVE: i32 = 4;
pub const TB_BRUSH: i32 = 5;

pub fn render_toolbox(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    option: &OptionModel,
) {
    let Some(tb) = first_component(option.root().get("toolbox")) else {
        return;
    };
    if tb.get("show").and_then(|v| v.as_bool()) == Some(false) {
        return;
    }
    let feature = tb.get("feature");
    let mut items: Vec<(i32, &'static str)> = Vec::new();
    if feature.and_then(|f| f.get("dataZoom")).is_some() {
        items.push((TB_DATA_ZOOM, "zoom"));
    }
    if feature.and_then(|f| f.get("dataView")).is_some() {
        items.push((TB_DATA_VIEW, "view"));
    }
    if feature.and_then(|f| f.get("magicType")).is_some() {
        items.push((TB_MAGIC, "type"));
    }
    if feature.and_then(|f| f.get("restore")).is_some() {
        items.push((TB_RESTORE, "restore"));
    }
    if feature.and_then(|f| f.get("saveAsImage")).is_some() {
        items.push((TB_SAVE, "save"));
    }
    if feature.and_then(|f| f.get("brush")).is_some() {
        items.push((TB_BRUSH, "brush"));
    }
    if items.is_empty() {
        return;
    }
    let size = tb
        .get("itemSize")
        .and_then(|v| v.as_f64())
        .unwrap_or(15.0);
    let gap = tb
        .get("itemGap")
        .and_then(|v| v.as_f64())
        .unwrap_or(8.0);
    let total_w = items.len() as f64 * (size + gap) - gap;
    let (ox, oy) = layout_origin(
        tb,
        model.width as f64,
        model.height as f64,
        total_w,
        size,
        model.width as f64 - total_w - 16.0,
        8.0,
    );
    for (i, (id, name)) in items.iter().enumerate() {
        let x = ox + i as f64 * (size + gap);
        let y = oy;
        add_rect(
            zr,
            group,
            x - 4.0,
            y - 4.0,
            size + 8.0,
            size + 8.0,
            FillStrokeStyle::none(),
            FillStrokeStyle::none(),
            0.0,
            22.0,
            Some(component_ec(HIT_TOOLBOX, *id)),
        );
        draw_icon(zr, group, *id, x, y, size);
        add_plain_label(
            zr,
            group,
            (*name).to_string(),
            x + size / 2.0,
            y + size + 2.0,
            TextAlign::Center,
            TextBaseline::Top,
            "#999",
            22.0,
        );
    }
}

fn draw_icon(zr: &mut ZRenderer, group: usize, id: i32, x: f64, y: f64, size: f64) {
    let stroke = PathStyle {
        fill: FillStrokeStyle::none(),
        stroke: FillStrokeStyle::color("#5470c6"),
        line_width: 1.2,
        ..PathStyle::stroke_default()
    };
    let fill = PathStyle {
        fill: FillStrokeStyle::color("#5470c6"),
        ..Default::default()
    };
    let z = DisplayableProps {
        z: 22.1,
        ..Default::default()
    };
    match id {
        TB_RESTORE => {
            let c = zr.storage.create_path(
                Path::new(
                    Shape::Circle(CircleShape {
                        cx: x + size / 2.0,
                        cy: y + size / 2.0,
                        r: size / 2.5,
                    }),
                    stroke,
                )
                .with_displayable(z),
            );
            zr.storage.group_add_child(group, ChildRef::Path(c));
        }
        TB_MAGIC => {
            let bar = zr.storage.create_path(
                Path::new(
                    Shape::Rect(RectShape {
                        x: x + 2.0,
                        y: y + size * 0.35,
                        width: size * 0.3,
                        height: size * 0.5,
                        ..Default::default()
                    }),
                    fill.clone(),
                )
                .with_displayable(z.clone()),
            );
            let line = zr.storage.create_path(
                Path::new(
                    Shape::Line(LineShape {
                        x1: x + size * 0.5,
                        y1: y + size * 0.8,
                        x2: x + size - 2.0,
                        y2: y + 2.0,
                        percent: 1.0,
                    }),
                    stroke,
                )
                .with_displayable(z),
            );
            zr.storage.group_add_child(group, ChildRef::Path(bar));
            zr.storage.group_add_child(group, ChildRef::Path(line));
        }
        TB_DATA_ZOOM => {
            let c = zr.storage.create_path(
                Path::new(
                    Shape::Circle(CircleShape {
                        cx: x + size * 0.4,
                        cy: y + size * 0.4,
                        r: size * 0.28,
                    }),
                    stroke.clone(),
                )
                .with_displayable(z.clone()),
            );
            let handle = zr.storage.create_path(
                Path::new(
                    Shape::Line(LineShape {
                        x1: x + size * 0.58,
                        y1: y + size * 0.58,
                        x2: x + size - 1.0,
                        y2: y + size - 1.0,
                        percent: 1.0,
                    }),
                    stroke,
                )
                .with_displayable(z),
            );
            zr.storage.group_add_child(group, ChildRef::Path(c));
            zr.storage.group_add_child(group, ChildRef::Path(handle));
        }
        _ => {
            let r = zr.storage.create_path(
                Path::new(
                    Shape::Rect(RectShape {
                        x: x + 2.0,
                        y: y + 2.0,
                        width: size - 4.0,
                        height: size - 4.0,
                        r: vec![2.0],
                    }),
                    stroke,
                )
                .with_displayable(z),
            );
            zr.storage.group_add_child(group, ChildRef::Path(r));
        }
    }
}

pub fn magic_types(option: &OptionModel) -> Vec<String> {
    first_component(option.root().get("toolbox"))
        .and_then(|t| t.get("feature"))
        .and_then(|f| f.get("magicType"))
        .and_then(|m| m.get("type"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(|| vec!["line".into(), "bar".into()])
}

pub fn current_magic_type(option: &OptionModel) -> String {
    option
        .root()
        .get("series")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|s| s.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("line")
        .to_string()
}

pub fn next_magic_type(option: &OptionModel) -> String {
    let types = magic_types(option);
    let cur = current_magic_type(option);
    if types.is_empty() {
        return if cur == "line" { "bar".into() } else { "line".into() };
    }
    let idx = types.iter().position(|t| t == &cur).unwrap_or(0);
    types[(idx + 1) % types.len()].clone()
}
