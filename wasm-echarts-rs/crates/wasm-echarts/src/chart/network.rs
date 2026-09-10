//! 关系图 / 和弦 / 桑基：nodes + links 终态布局

use std::f64::consts::PI;

use rust_zrender::{
    BezierCurveShape, LineShape, PolygonShape, Shape, TextAlign, TextBaseline, ZRenderer,
};

use crate::chart::label::add_label;
use crate::chart::path_util::{add_ec_path, fill_stroke};
use crate::chart::symbol::{add_symbol, SymbolSpec};
use crate::model::{GlobalModel, SeriesModel};
use crate::option::OptionValue;
use crate::utils::parse_percent;
use crate::visual::VisualContext;

struct Node {
    name: String,
    x: f64,
    y: f64,
    value: f64,
    data_index: usize,
}

struct Link {
    source: usize,
    target: usize,
    value: f64,
}

pub fn render_graph_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let (nodes, links) = parse_graph(visual, series, model);
    for link in &links {
        let Some(a) = nodes.get(link.source) else { continue };
        let Some(b) = nodes.get(link.target) else { continue };
        add_ec_path(
            zr,
            group,
            Shape::Line(LineShape {
                x1: a.x,
                y1: a.y,
                x2: b.x,
                y2: b.y,
                percent: 1.0,
            }),
            fill_stroke("none", "#aaa", 1.0, 0.7),
            series.index,
            link.source,
            series.index as f64,
            false,
        );
    }
    for node in &nodes {
        let color = visual.resolve_item_color(series.index, node.data_index);
        add_symbol(
            zr,
            group,
            &SymbolSpec {
                kind: "circle".into(),
                size: (8.0 + node.value.abs().sqrt()).min(28.0),
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
            node.y - 10.0,
            TextAlign::Center,
            TextBaseline::Bottom,
            &color,
            series.index as f64 + 0.2,
        );
    }
}

pub fn render_chord_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let (mut nodes, links) = parse_graph(visual, series, model);
    let cx = model.width as f64 * 0.5;
    let cy = model.height as f64 * 0.5;
    let r = model.width.min(model.height) as f64 * 0.32;
    let n = nodes.len().max(1) as f64;
    for (i, node) in nodes.iter_mut().enumerate() {
        let a = -PI / 2.0 + i as f64 * PI * 2.0 / n;
        node.x = cx + r * a.cos();
        node.y = cy + r * a.sin();
    }
    for link in &links {
        let Some(a) = nodes.get(link.source) else { continue };
        let Some(b) = nodes.get(link.target) else { continue };
        add_ec_path(
            zr,
            group,
            Shape::BezierCurve(BezierCurveShape {
                x1: a.x,
                y1: a.y,
                x2: b.x,
                y2: b.y,
                cpx1: cx,
                cpy1: cy,
                cpx2: Some(cx),
                cpy2: Some(cy),
                percent: 1.0,
            }),
            fill_stroke("none", &visual.resolve_item_color(series.index, link.source), 1.5, 0.55),
            series.index,
            link.source,
            series.index as f64,
            false,
        );
    }
    for node in &nodes {
        let color = visual.resolve_item_color(series.index, node.data_index);
        add_symbol(
            zr,
            group,
            &SymbolSpec {
                kind: "circle".into(),
                size: 10.0,
                cx: node.x,
                cy: node.y,
                color,
                series_index: series.index,
                data_index: node.data_index,
                attach_states: true,
            },
        );
    }
}

pub fn render_sankey_series(
    zr: &mut ZRenderer,
    group: usize,
    model: &GlobalModel,
    visual: &VisualContext,
    series: &SeriesModel,
) {
    let (mut nodes, links) = parse_graph(visual, series, model);
    if nodes.is_empty() {
        return;
    }
    let left = 40.0;
    let top = 30.0;
    let w = (model.width as f64 - 80.0).max(40.0);
    let h = (model.height as f64 - 60.0).max(40.0);
    let incoming: Vec<usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, _)| links.iter().filter(|l| l.target == i).count())
        .collect();
    let mut cols = vec![0usize; nodes.len()];
    for i in 0..nodes.len() {
        cols[i] = if incoming[i] == 0 { 0 } else { 1 };
    }
    for link in &links {
        cols[link.target] = cols[link.target].max(cols[link.source] + 1);
    }
    let max_col = cols.iter().copied().max().unwrap_or(0).max(1);
    let mut col_nodes: Vec<Vec<usize>> = vec![Vec::new(); max_col + 1];
    for (i, c) in cols.iter().enumerate() {
        col_nodes[*c].push(i);
    }
    let node_w = 16.0;
    for (c, ids) in col_nodes.iter().enumerate() {
        let n = ids.len().max(1) as f64;
        let gap = h / n;
        for (k, &i) in ids.iter().enumerate() {
            nodes[i].x = left + c as f64 * (w / max_col as f64);
            nodes[i].y = top + (k as f64 + 0.5) * gap;
        }
    }
    for link in &links {
        let Some(a) = nodes.get(link.source) else { continue };
        let Some(b) = nodes.get(link.target) else { continue };
        let mid = (a.x + b.x) / 2.0;
        add_ec_path(
            zr,
            group,
            Shape::BezierCurve(BezierCurveShape {
                x1: a.x + node_w,
                y1: a.y,
                x2: b.x,
                y2: b.y,
                cpx1: mid,
                cpy1: a.y,
                cpx2: Some(mid),
                cpy2: Some(b.y),
                percent: 1.0,
            }),
            fill_stroke("none", &visual.resolve_item_color(series.index, link.source), (2.0 + link.value.abs().sqrt()).min(10.0) as f32, 0.45),
            series.index,
            link.source,
            series.index as f64,
            false,
        );
    }
    for node in &nodes {
        let color = visual.resolve_item_color(series.index, node.data_index);
        add_ec_path(
            zr,
            group,
            Shape::Polygon(PolygonShape {
                points: vec![
                    (node.x, node.y - 10.0),
                    (node.x + node_w, node.y - 10.0),
                    (node.x + node_w, node.y + 10.0),
                    (node.x, node.y + 10.0),
                ],
                ..Default::default()
            }),
            fill_stroke(&color, "none", 0.0, 1.0),
            series.index,
            node.data_index,
            series.index as f64 + 0.2,
            true,
        );
        add_label(
            zr,
            group,
            visual,
            series.index,
            node.data_index,
            node.x + node_w + 4.0,
            node.y,
            TextAlign::Left,
            TextBaseline::Middle,
            "#333",
            series.index as f64 + 0.3,
        );
    }
}

fn parse_graph(visual: &VisualContext, series: &SeriesModel, model: &GlobalModel) -> (Vec<Node>, Vec<Link>) {
    let opt = visual.series_option(series.index);
    let data = opt.and_then(|s| s.get("data")).or_else(|| opt.and_then(|s| s.get("nodes")));
    let mut nodes = Vec::new();
    match data {
        Some(OptionValue::Array(arr)) => {
            for (i, item) in arr.iter().enumerate() {
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let value = item.get("value").and_then(|v| v.as_f64()).unwrap_or(1.0);
                let x = item.get("x").and_then(|v| v.as_f64());
                let y = item.get("y").and_then(|v| v.as_f64());
                nodes.push(Node {
                    name,
                    x: x.unwrap_or(f64::NAN),
                    y: y.unwrap_or(f64::NAN),
                    value,
                    data_index: i,
                });
            }
        }
        _ => {
            for (i, p) in series.data.iter().enumerate() {
                nodes.push(Node {
                    name: p.name.clone().unwrap_or_else(|| i.to_string()),
                    x: f64::NAN,
                    y: f64::NAN,
                    value: p.value.abs().max(1.0),
                    data_index: i,
                });
            }
        }
    }
    let w = model.width as f64;
    let h = model.height as f64;
    let n = nodes.len().max(1) as f64;
    let layout = opt
        .and_then(|s| s.get("layout"))
        .and_then(|v| v.as_str())
        .unwrap_or("circular");
    for (i, node) in nodes.iter_mut().enumerate() {
        if node.x.is_finite() && node.y.is_finite() {
            if node.x <= 1.0 && node.y <= 1.0 && node.x >= 0.0 {
                node.x *= w;
                node.y *= h;
            }
            continue;
        }
        if layout == "none" {
            node.x = parse_percent(None, w, w * 0.5);
            node.y = parse_percent(None, h, h * 0.5);
        } else {
            let a = -PI / 2.0 + i as f64 * PI * 2.0 / n;
            let r = w.min(h) * 0.32;
            node.x = w * 0.5 + r * a.cos();
            node.y = h * 0.5 + r * a.sin();
        }
    }
    let mut links = Vec::new();
    if let Some(OptionValue::Array(arr)) = opt.and_then(|s| s.get("links")).or_else(|| opt.and_then(|s| s.get("edges"))) {
        for item in arr {
            let source = resolve_end(&nodes, item.get("source"));
            let target = resolve_end(&nodes, item.get("target"));
            if let (Some(s), Some(t)) = (source, target) {
                links.push(Link {
                    source: s,
                    target: t,
                    value: item.get("value").and_then(|v| v.as_f64()).unwrap_or(1.0),
                });
            }
        }
    }
    (nodes, links)
}

fn resolve_end(nodes: &[Node], value: Option<&OptionValue>) -> Option<usize> {
    match value? {
        OptionValue::Number(n) => {
            let i = *n as usize;
            (i < nodes.len()).then_some(i)
        }
        OptionValue::String(s) => nodes.iter().position(|n| n.name == *s),
        _ => None,
    }
}
