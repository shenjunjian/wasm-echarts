//! series.labelLayout：终态 hideOverlap / moveOverlap / dx dy。

use rust_zrender::{TextAlign, TextBaseline, ZRenderer};

use crate::bridge::JsCallback;
use crate::option::{parse_option_value, OptionValue};
use crate::visual::VisualContext;

const LABEL_NAME_PREFIX: &str = "__ec_series_label:";

#[derive(Debug, Clone, Default)]
pub struct LabelLayoutSpec {
    pub hide_overlap: bool,
    pub move_overlap: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub dx: f64,
    pub dy: f64,
    pub align: Option<TextAlign>,
    pub baseline: Option<TextBaseline>,
}

pub fn series_label_name(series_index: usize) -> String {
    format!("{LABEL_NAME_PREFIX}{series_index}")
}

pub fn resolve_label_layout(
    visual: &VisualContext,
    series_index: usize,
    data_index: usize,
) -> LabelLayoutSpec {
    let raw = visual
        .series_option(series_index)
        .and_then(|s| s.get("labelLayout"));
    let Some(raw) = raw else {
        return LabelLayoutSpec::default();
    };
    let resolved = match raw {
        OptionValue::Function(func) => {
            let params = visual.data_params(series_index, data_index);
            match JsCallback::new(func.clone()).inner().call1(&wasm_bindgen::JsValue::NULL, &params) {
                Ok(ret) => parse_option_value(&ret).ok(),
                Err(_) => None,
            }
        }
        other => Some(other.clone()),
    };
    parse_layout_spec(resolved.as_ref())
}

fn parse_layout_spec(value: Option<&OptionValue>) -> LabelLayoutSpec {
    let Some(value) = value else {
        return LabelLayoutSpec::default();
    };
    LabelLayoutSpec {
        hide_overlap: value
            .get("hideOverlap")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        move_overlap: value
            .get("moveOverlap")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        x: value.get("x").and_then(|v| v.as_f64()),
        y: value.get("y").and_then(|v| v.as_f64()),
        dx: value.get("dx").and_then(|v| v.as_f64()).unwrap_or(0.0),
        dy: value.get("dy").and_then(|v| v.as_f64()).unwrap_or(0.0),
        align: value.get("align").and_then(|v| v.as_str()).and_then(parse_align),
        baseline: value
            .get("verticalAlign")
            .and_then(|v| v.as_str())
            .and_then(parse_baseline),
    }
}

fn parse_align(s: &str) -> Option<TextAlign> {
    match s {
        "left" => Some(TextAlign::Left),
        "center" => Some(TextAlign::Center),
        "right" => Some(TextAlign::Right),
        _ => None,
    }
}

fn parse_baseline(s: &str) -> Option<TextBaseline> {
    match s {
        "top" => Some(TextBaseline::Top),
        "middle" => Some(TextBaseline::Middle),
        "bottom" => Some(TextBaseline::Bottom),
        _ => None,
    }
}

#[derive(Clone)]
struct LabelBox {
    index: usize,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    hide_overlap: bool,
    move_overlap: Option<String>,
    series_index: usize,
}

fn text_box(text: &rust_zrender::Text, hide_overlap: bool, move_overlap: Option<String>, series_index: usize, index: usize) -> LabelBox {
    let w = (text.content.chars().count() as f64 * text.style.font_size as f64 * 0.6).max(4.0);
    let h = text.style.font_size as f64;
    let mut x = text.x;
    let mut y = text.y;
    match text.style.align {
        TextAlign::Center => x -= w / 2.0,
        TextAlign::Right => x -= w,
        TextAlign::Left => {}
    }
    match text.style.baseline {
        TextBaseline::Middle => y -= h / 2.0,
        TextBaseline::Bottom | TextBaseline::Alphabetic => y -= h,
        TextBaseline::Top => {}
    }
    LabelBox {
        index,
        x,
        y,
        w,
        h,
        hide_overlap,
        move_overlap,
        series_index,
    }
}

fn overlaps(a: &LabelBox, b: &LabelBox) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

/// 对已挂上的 series label 做终态避让。
pub fn apply_label_layout(zr: &mut ZRenderer, visual: &VisualContext) {
    let mut boxes = Vec::new();
    for (i, text) in zr.storage.texts().iter().enumerate() {
        let Some(rest) = text.base.name.strip_prefix(LABEL_NAME_PREFIX) else {
            continue;
        };
        let Ok(series_index) = rest.parse::<usize>() else {
            continue;
        };
        let layout = resolve_label_layout(visual, series_index, 0);
        if !layout.hide_overlap && layout.move_overlap.is_none() {
            continue;
        }
        boxes.push(text_box(
            text,
            layout.hide_overlap,
            layout.move_overlap.clone(),
            series_index,
            i,
        ));
    }
    if boxes.is_empty() {
        return;
    }

    boxes.sort_by(|a, b| {
        a.series_index
            .cmp(&b.series_index)
            .then(a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });

    for i in 0..boxes.len() {
        let Some(kind) = boxes[i].move_overlap.clone() else {
            continue;
        };
        for j in 0..i {
            if !overlaps(&boxes[i], &boxes[j]) {
                continue;
            }
            match kind.as_str() {
                "shiftY" => {
                    let ny = boxes[j].y + boxes[j].h + 2.0;
                    let dy = ny - boxes[i].y;
                    boxes[i].y = ny;
                    zr.storage.text_mut(boxes[i].index).y += dy;
                }
                "shiftX" => {
                    let nx = boxes[j].x + boxes[j].w + 2.0;
                    let dx = nx - boxes[i].x;
                    boxes[i].x = nx;
                    zr.storage.text_mut(boxes[i].index).x += dx;
                }
                "shuffleY" => {
                    let dy = ((boxes[i].index as f64 * 13.0) % 11.0) - 5.0;
                    boxes[i].y += dy;
                    zr.storage.text_mut(boxes[i].index).y += dy;
                }
                "shuffleX" => {
                    let dx = ((boxes[i].index as f64 * 17.0) % 11.0) - 5.0;
                    boxes[i].x += dx;
                    zr.storage.text_mut(boxes[i].index).x += dx;
                }
                _ => {}
            }
        }
    }

    let mut kept: Vec<usize> = Vec::new();
    for i in 0..boxes.len() {
        if !boxes[i].hide_overlap {
            kept.push(i);
            continue;
        }
        let hit = kept.iter().any(|&k| overlaps(&boxes[i], &boxes[k]));
        if hit {
            zr.storage.text_mut(boxes[i].index).base.ignore = true;
        } else {
            kept.push(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_detects_aabb() {
        let a = LabelBox {
            index: 0,
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 10.0,
            hide_overlap: true,
            move_overlap: None,
            series_index: 0,
        };
        let mut b = a.clone();
        b.x = 20.0;
        assert!(!overlaps(&a, &b));
        b.x = 5.0;
        assert!(overlaps(&a, &b));
    }
}
