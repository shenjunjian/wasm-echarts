//! 树 / 矩形树图 / 旭日：从 series.data 递归布局

use std::f64::consts::PI;

use rust_zrender::{
    LineShape, RectShape, SectorShape, Shape, TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke, item_name};
use crate::chart::symbol::{add_symbol, SymbolSpec};
use crate::model::{GlobalModel, SeriesModel};
use crate::option::OptionValue;
use crate::utils::parse_percent;
use crate::visual::VisualContext;

struct HNode {
    #[allow(dead_code)]
    name: String,
    value: f64,
    data_index: usize,
    children: Vec<HNode>,
}

pub fn render_tree_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let roots = collect_roots(visual, series);
    if roots.is_empty() {
        return;
    }
    let rect = content_rect(model, visual.series_option(series.index));
    let mut leaves = 0.0;
    count_leaves(&roots, &mut leaves);
    let mut cursor = 0.0;
    let mut placed = Vec::new();
    place_tree(&roots, 0, &rect, leaves.max(1.0), &mut cursor, &mut placed);
    for (parent, child) in links_of(&placed) {
        add_ec_path(
            zr,
            group,
            Shape::Line(LineShape {
                x1: parent.0,
                y1: parent.1,
                x2: child.0,
                y2: child.1,
                percent: 1.0,
            }),
            fill_stroke("none", "#ccc", 1.0, 1.0),
            series.index,
            0,
            series.index as f64,
            false,
        );
    }
    for node in &placed {
        let color = visual.resolve_item_color(series.index, node.data_index);
        add_symbol(
            zr,
            group,
            &SymbolSpec {
                kind: "circle".into(),
                size: 10.0,
                cx: node.x,
                cy: node.y,
                color: color.clone(),
                series_index: series.index,
                data_index: node.data_index,
                attach_states: true,
            },
        );
        add_label(
            zr,
            group,
            visual,
            series.index,
            node.data_index,
            node.x,
            node.y - 8.0,
            TextAlign::Center,
            TextBaseline::Bottom,
            &color,
            series.index as f64 + 0.2,
        );
    }
}

pub fn render_treemap_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let roots = collect_roots(visual, series);
    let rect = content_rect(model, visual.series_option(series.index));
    let mut boxes = Vec::new();
    squarify(&roots, rect.0, rect.1, rect.2, rect.3, &mut boxes);
    for (i, b) in boxes.iter().enumerate() {
        let color = visual.resolve_item_color(series.index, b.data_index.max(i));
        add_ec_path(
            zr,
            group,
            Shape::Rect(RectShape {
                x: b.x,
                y: b.y,
                width: b.w.max(1.0),
                height: b.h.max(1.0),
                ..Default::default()
            }),
            fill_stroke(&color, "#fff", 1.0, 0.9),
            series.index,
            b.data_index,
            series.index as f64,
            true,
        );
        if b.w > 24.0 && b.h > 14.0 {
            add_label(
                zr,
                group,
                visual,
                series.index,
                b.data_index,
                b.x + b.w / 2.0,
                b.y + b.h / 2.0,
                TextAlign::Center,
                TextBaseline::Middle,
                "#333",
                series.index as f64 + 0.2,
            );
        }
    }
}

pub fn render_sunburst_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let roots = collect_roots(visual, series);
    let w = model.width as f64;
    let h = model.height as f64;
    let series_opt = visual.series_option(series.index);
    let (cx, cy) = match series_opt.and_then(|s| s.get("center")) {
        Some(OptionValue::Array(arr)) => (
            parse_percent(arr.first(), w, w * 0.5),
            parse_percent(arr.get(1), h, h * 0.5),
        ),
        _ => (w * 0.5, h * 0.5),
    };
    let half = w.min(h) / 2.0;
    let (r0, r) = match series_opt.and_then(|s| s.get("radius")) {
        Some(OptionValue::Array(arr)) => (
            parse_percent(arr.first(), half, 0.0),
            parse_percent(arr.get(1), half, half * 0.8),
        ),
        Some(v) => (0.0, parse_percent(Some(v), half, half * 0.8)),
        None => (0.0, half * 0.8),
    };
    let depth = max_depth(&roots).max(1);
    let ring = ((r - r0) / depth as f64).max(4.0);
    paint_sunburst(
        zr,
        group,
        visual,
        series,
        &roots,
        cx,
        cy,
        r0,
        ring,
        -PI / 2.0,
        PI * 2.0,
        0,
    );
}

struct Placed {
    x: f64,
    y: f64,
    data_index: usize,
    depth: usize,
    parent: Option<usize>,
}

struct BoxRec {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    data_index: usize,
}

fn collect_roots(visual: &VisualContext, series: &SeriesModel) -> Vec<HNode> {
    let data = visual
        .series_option(series.index)
        .and_then(|s| s.get("data"));
    let mut idx = 0;
    match data {
        Some(OptionValue::Array(arr)) => arr.iter().filter_map(|v| parse_node(v, &mut idx)).collect(),
        Some(v) => parse_node(v, &mut idx).into_iter().collect(),
        None => series
            .data
            .iter()
            .enumerate()
            .map(|(i, p)| HNode {
                name: item_name(p, i),
                value: p.value.abs().max(1.0),
                data_index: i,
                children: Vec::new(),
            })
            .collect(),
    }
}

fn parse_node(value: &OptionValue, idx: &mut usize) -> Option<HNode> {
    let data_index = *idx;
    *idx += 1;
    match value {
        OptionValue::Object(obj) => {
            let name = obj
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let children: Vec<HNode> = obj
                .get("children")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|c| parse_node(c, idx)).collect())
                .unwrap_or_default();
            let value = obj
                .get("value")
                .and_then(|v| v.as_f64())
                .unwrap_or_else(|| children.iter().map(|c| c.value).sum::<f64>().max(1.0));
            Some(HNode {
                name,
                value: value.abs().max(0.0),
                data_index,
                children,
            })
        }
        OptionValue::Number(n) => Some(HNode {
            name: data_index.to_string(),
            value: n.abs(),
            data_index,
            children: Vec::new(),
        }),
        _ => None,
    }
}

fn count_leaves(nodes: &[HNode], n: &mut f64) {
    for node in nodes {
        if node.children.is_empty() {
            *n += 1.0;
        } else {
            count_leaves(&node.children, n);
        }
    }
}

fn place_tree(
    nodes: &[HNode],
    depth: usize,
    rect: &(f64, f64, f64, f64),
    leaves: f64,
    cursor: &mut f64,
    out: &mut Vec<Placed>,
) {
    for node in nodes {
        let parent = out.len();
        if node.children.is_empty() {
            let t = (*cursor + 0.5) / leaves;
            *cursor += 1.0;
            out.push(Placed {
                x: rect.0 + t * rect.2,
                y: rect.1 + depth as f64 * (rect.3 / 6.0).max(28.0),
                data_index: node.data_index,
                depth,
                parent: None,
            });
        } else {
            let start = out.len();
            place_tree(&node.children, depth + 1, rect, leaves, cursor, out);
            let kids = &out[start..];
            let x = if kids.is_empty() {
                rect.0 + rect.2 * 0.5
            } else {
                kids.iter().map(|k| k.x).sum::<f64>() / kids.len() as f64
            };
            out.insert(
                parent,
                Placed {
                    x,
                    y: rect.1 + depth as f64 * (rect.3 / 6.0).max(28.0),
                    data_index: node.data_index,
                    depth,
                    parent: None,
                },
            );
            for child in out.iter_mut().skip(parent + 1) {
                if child.depth == depth + 1 && child.parent.is_none() {
                    child.parent = Some(parent);
                }
            }
        }
    }
}

fn links_of(placed: &[Placed]) -> Vec<((f64, f64), (f64, f64))> {
    placed
        .iter()
        .filter_map(|n| {
            let p = n.parent?;
            let parent = placed.get(p)?;
            Some(((parent.x, parent.y), (n.x, n.y)))
        })
        .collect()
}

fn squarify(nodes: &[HNode], x: f64, y: f64, w: f64, h: f64, out: &mut Vec<BoxRec>) {
    if w < 0.5 || h < 0.5 || nodes.is_empty() {
        return;
    }
    let total: f64 = nodes.iter().map(|n| n.value.max(0.1)).sum::<f64>().max(1e-9);
    let mut cx = x;
    let mut cy = y;
    let horizontal = w >= h;
    for node in nodes {
        let frac = node.value.max(0.1) / total;
        if !node.children.is_empty() {
            if horizontal {
                let nw = w * frac;
                squarify(&node.children, cx, cy, nw, h, out);
                cx += nw;
            } else {
                let nh = h * frac;
                squarify(&node.children, cx, cy, w, nh, out);
                cy += nh;
            }
        } else if horizontal {
            let nw = w * frac;
            out.push(BoxRec {
                x: cx,
                y: cy,
                w: nw,
                h,
                data_index: node.data_index,
            });
            cx += nw;
        } else {
            let nh = h * frac;
            out.push(BoxRec {
                x: cx,
                y: cy,
                w,
                h: nh,
                data_index: node.data_index,
            });
            cy += nh;
        }
    }
}

fn max_depth(nodes: &[HNode]) -> usize {
    nodes
        .iter()
        .map(|n| 1 + max_depth(&n.children))
        .max()
        .unwrap_or(0)
}

fn paint_sunburst(
    zr: &mut ZRenderer,
    group: usize,
    visual: &VisualContext,
    series: &SeriesModel,
    nodes: &[HNode],
    cx: f64,
    cy: f64,
    r0: f64,
    ring: f64,
    start: f64,
    sweep: f64,
    depth: usize,
) {
    let total: f64 = nodes.iter().map(|n| n.value.max(0.1)).sum::<f64>().max(1e-9);
    let mut a = start;
    for node in nodes {
        let frac = node.value.max(0.1) / total;
        let sw = sweep * frac;
        let color = visual.resolve_item_color(series.index, node.data_index);
        add_ec_path(
            zr,
            group,
            Shape::Sector(SectorShape {
                cx,
                cy,
                r: r0 + ring,
                r0,
                start_angle: a,
                end_angle: a + sw,
                clockwise: true,
                ..Default::default()
            }),
            fill_stroke(&color, "#fff", 1.0, 1.0),
            series.index,
            node.data_index,
            series.index as f64 + depth as f64 * 0.05,
            true,
        );
        let mid = a + sw / 2.0;
        let lx = cx + (r0 + ring * 0.55) * mid.cos();
        let ly = cy + (r0 + ring * 0.55) * mid.sin();
        if sw.abs() > 0.12 {
            add_label(
                zr,
                group,
                visual,
                series.index,
                node.data_index,
                lx,
                ly,
                TextAlign::Center,
                TextBaseline::Middle,
                "#333",
                series.index as f64 + 0.4,
            );
        }
        if !node.children.is_empty() {
            paint_sunburst(
                zr,
                group,
                visual,
                series,
                &node.children,
                cx,
                cy,
                r0 + ring,
                ring,
                a,
                sw,
                depth + 1,
            );
        }
        a += sw;
    }
}

fn content_rect(model: &GlobalModel, series_opt: Option<&OptionValue>) -> (f64, f64, f64, f64) {
    let w = model.width as f64;
    let h = model.height as f64;
    let left = parse_percent(series_opt.and_then(|s| s.get("left")), w, 40.0);
    let top = parse_percent(series_opt.and_then(|s| s.get("top")), h, 40.0);
    let right = parse_percent(series_opt.and_then(|s| s.get("right")), w, 20.0);
    let bottom = parse_percent(series_opt.and_then(|s| s.get("bottom")), h, 20.0);
    (left, top, (w - left - right).max(10.0), (h - top - bottom).max(10.0))
}
