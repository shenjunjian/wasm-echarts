//! line / scatter 的 symbol：官方常用形状 + empty* 空心

use rust_zrender::{
    ChildRef, CircleShape, DisplayableProps, DropletShape, EcData, FillStrokeStyle, LineShape, Path,
    PathStyle, PathStylePatch, PolygonShape, RectShape, Shape, StarShape, STATE_EMPHASIS,
    STATE_SELECT, ZRenderer,
};

pub struct SymbolSpec {
    pub kind: String,
    pub size: f64,
    pub cx: f64,
    pub cy: f64,
    pub color: String,
    pub series_index: usize,
    pub data_index: usize,
    pub attach_states: bool,
}

fn empty_kind(kind: &str) -> (String, bool) {
    if let Some(rest) = kind.strip_prefix("empty") {
        if rest.is_empty() {
            return (kind.to_string(), false);
        }
        let mut chars = rest.chars();
        let first = chars.next().map(|c| c.to_ascii_lowercase()).unwrap_or('c');
        let body: String = std::iter::once(first).chain(chars).collect();
        (body, true)
    } else {
        (kind.to_string(), false)
    }
}

fn polygon(points: Vec<(f64, f64)>) -> Shape {
    Shape::Polygon(PolygonShape {
        points,
        ..Default::default()
    })
}

pub fn add_symbol(zr: &mut ZRenderer, group: usize, spec: &SymbolSpec) -> Option<usize> {
    if spec.size <= 0.0 {
        return None;
    }
    let r = spec.size / 2.0;
    let (base_kind, empty) = empty_kind(&spec.kind);
    if base_kind == "none" {
        return None;
    }
    let (shape, mut fill, mut stroke, mut line_width) = match base_kind.as_str() {
        "rect" | "square" => (
            Shape::Rect(RectShape {
                x: spec.cx - r,
                y: spec.cy - r,
                width: spec.size,
                height: spec.size,
                ..Default::default()
            }),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "roundRect" => (
            Shape::Rect(RectShape {
                x: spec.cx - r,
                y: spec.cy - r,
                width: spec.size,
                height: spec.size,
                r: vec![spec.size / 4.0],
            }),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "triangle" => (
            polygon(vec![
                (spec.cx, spec.cy - r),
                (spec.cx + r, spec.cy + r),
                (spec.cx - r, spec.cy + r),
            ]),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "diamond" => (
            polygon(vec![
                (spec.cx, spec.cy - r),
                (spec.cx + r, spec.cy),
                (spec.cx, spec.cy + r),
                (spec.cx - r, spec.cy),
            ]),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "arrow" => (
            polygon(vec![
                (spec.cx, spec.cy - r),
                (spec.cx + r * 0.7, spec.cy + r * 0.1),
                (spec.cx + r * 0.22, spec.cy + r * 0.1),
                (spec.cx + r * 0.22, spec.cy + r),
                (spec.cx - r * 0.22, spec.cy + r),
                (spec.cx - r * 0.22, spec.cy + r * 0.1),
                (spec.cx - r * 0.7, spec.cy + r * 0.1),
            ]),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "pin" => (
            Shape::Droplet(DropletShape {
                cx: spec.cx,
                cy: spec.cy,
                width: r,
                height: r * 1.4,
            }),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "star" => (
            Shape::Star(StarShape {
                cx: spec.cx,
                cy: spec.cy,
                n: 5,
                r,
                r0: None,
            }),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
        "line" => (
            Shape::Line(LineShape {
                x1: spec.cx - r,
                y1: spec.cy,
                x2: spec.cx + r,
                y2: spec.cy,
                percent: 1.0,
            }),
            FillStrokeStyle::none(),
            FillStrokeStyle::color(&spec.color),
            2.0,
        ),
        _ => (
            Shape::Circle(CircleShape {
                cx: spec.cx,
                cy: spec.cy,
                r,
            }),
            FillStrokeStyle::color(&spec.color),
            FillStrokeStyle::color("#fff"),
            1.0,
        ),
    };
    if empty {
        // 官方 empty*：白底 + 系列色描边，避免面积色透出来像实心点
        fill = FillStrokeStyle::color("#fff");
        stroke = FillStrokeStyle::color(&spec.color);
        line_width = 2.0;
    }
    let symbol = zr.storage.create_path(
        Path::new(
            shape,
            PathStyle {
                fill,
                stroke,
                line_width,
                ..Default::default()
            },
        )
        .with_displayable(DisplayableProps {
            z: spec.series_index as f64 + 0.1,
            ..Default::default()
        })
        .with_ec_data(EcData::new(spec.series_index as i32, spec.data_index as i32)),
    );
    zr.storage.group_add_child(group, ChildRef::Path(symbol));
    if spec.attach_states {
        zr.set_path_state_style(
            symbol,
            STATE_EMPHASIS,
            PathStylePatch {
                fill: Some(FillStrokeStyle::color(&spec.color)),
                line_width: Some(2.0),
                ..Default::default()
            },
        );
        zr.set_path_state_style(
            symbol,
            STATE_SELECT,
            PathStylePatch {
                stroke: Some(FillStrokeStyle::color("#333")),
                line_width: Some(3.0),
                ..Default::default()
            },
        );
    }
    Some(symbol)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_zrender::ZRenderer;

    fn spec(kind: &str) -> SymbolSpec {
        SymbolSpec {
            kind: kind.into(),
            size: 10.0,
            cx: 20.0,
            cy: 20.0,
            color: "#5470c6".into(),
            series_index: 0,
            data_index: 0,
            attach_states: true,
        }
    }

    #[test]
    fn extra_symbols_create_paths() {
        let mut zr = ZRenderer::new(80, 80).unwrap();
        let group = zr.storage.create_group();
        for kind in ["triangle", "diamond", "roundRect", "star", "pin", "arrow", "emptyDiamond"] {
            add_symbol(&mut zr, group, &spec(kind));
        }
        assert_eq!(zr.storage.paths().len(), 7);
        let empty = zr.storage.paths().last().unwrap();
        assert!(matches!(&empty.style.fill, FillStrokeStyle::Color(c) if c == "#fff"));
        assert!(matches!(&empty.style.stroke, FillStrokeStyle::Color(c) if c == "#5470c6"));
    }

    #[test]
    fn empty_circle_is_white_fill_with_stroke() {
        let mut zr = ZRenderer::new(80, 80).unwrap();
        let group = zr.storage.create_group();
        add_symbol(&mut zr, group, &spec("emptyCircle"));
        let path = zr.storage.paths().last().unwrap();
        assert!(matches!(path.shape, Shape::Circle(_)));
        assert!(matches!(&path.style.fill, FillStrokeStyle::Color(c) if c == "#fff"));
        assert!(matches!(&path.style.stroke, FillStrokeStyle::Color(c) if c == "#5470c6"));
        assert!((path.style.line_width - 2.0).abs() < 1e-6);
    }
}
