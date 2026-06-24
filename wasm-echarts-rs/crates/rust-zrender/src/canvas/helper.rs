//! Canvas 绘制辅助：渐变坐标映射（对齐 zrender canvas/helper.ts）

use std::sync::Arc;

use vl_convert_canvas2d::{CanvasColor, CanvasGradient, RadialGradientParams, CanvasPattern};

use crate::canvas::backend::BackendError;
use crate::core::bbox::BoundingRect;
use crate::graphic::style::{
    FillStrokeStyle, LinearGradientStyle, PatternStyle, RadialGradientStyle,
};

fn is_safe_num(n: f64) -> bool {
    n.is_finite()
}

pub fn create_linear_gradient(
    obj: &LinearGradientStyle,
    rect: &BoundingRect,
) -> Result<CanvasGradient, BackendError> {
    let mut x = obj.x;
    let mut x2 = obj.x2;
    let mut y = obj.y;
    let mut y2 = obj.y2;

    if !obj.global {
        x = x * rect.width + rect.x;
        x2 = x2 * rect.width + rect.x;
        y = y * rect.height + rect.y;
        y2 = y2 * rect.height + rect.y;
    }

    let x = if is_safe_num(x) { x } else { 0.0 } as f32;
    let x2 = if is_safe_num(x2) { x2 } else { 1.0 } as f32;
    let y = if is_safe_num(y) { y } else { 0.0 } as f32;
    let y2 = if is_safe_num(y2) { y2 } else { 0.0 } as f32;

    let mut gradient = CanvasGradient::new_linear(x, y, x2, y2);
    for stop in &obj.color_stops {
        gradient.add_color_stop(stop.offset, parse_canvas_color(&stop.color)?);
    }
    Ok(gradient)
}

pub fn create_radial_gradient(
    obj: &RadialGradientStyle,
    rect: &BoundingRect,
) -> Result<CanvasGradient, BackendError> {
    let min = rect.width.min(rect.height);

    let mut x = obj.x;
    let mut y = obj.y;
    let mut r = obj.r;
    let mut r0 = obj.r0;

    if !obj.global {
        x = x * rect.width + rect.x;
        y = y * rect.height + rect.y;
        r = r * min;
        r0 = r0 * min;
    }

    let x = if is_safe_num(x) { x } else { 0.5 } as f32;
    let y = if is_safe_num(y) { y } else { 0.5 } as f32;
    let r = if r >= 0.0 && is_safe_num(r) { r } else { 0.5 } as f32;
    let r0 = if r0 >= 0.0 && is_safe_num(r0) { r0 } else { 0.0 } as f32;

    let params = RadialGradientParams {
        x0: x,
        y0: y,
        r0,
        x1: x,
        y1: y,
        r1: r,
    };

    let mut gradient = CanvasGradient::new_radial(&params);
    for stop in &obj.color_stops {
        gradient.add_color_stop(stop.offset, parse_canvas_color(&stop.color)?);
    }
    Ok(gradient)
}

fn parse_canvas_color(s: &str) -> Result<CanvasColor, BackendError> {
    let parsed = csscolorparser::parse(s)
        .map_err(|e| BackendError::Canvas(format!("color parse {s}: {e}")))?;
    let [r, g, b, a] = parsed.to_array();
    Ok(CanvasColor::from_rgba_f32(r, g, b, a))
}

pub fn create_pattern(
    ctx: &vl_convert_canvas2d::Canvas2dContext,
    pattern: &PatternStyle,
) -> Result<Arc<CanvasPattern>, BackendError> {
    ctx.create_pattern(
        &pattern.data,
        pattern.width,
        pattern.height,
        &pattern.repeat,
    )
    .map_err(|e| BackendError::Canvas(e.to_string()))
}

pub fn apply_fill_style(
    ctx: &mut dyn crate::canvas::backend::CanvasContext,
    fill: &FillStrokeStyle,
    rect: &BoundingRect,
) -> Result<bool, BackendError> {
    match fill {
        FillStrokeStyle::None => Ok(false),
        FillStrokeStyle::Color(c) if c == "none" || c.is_empty() => Ok(false),
        FillStrokeStyle::Color(c) => {
            ctx.set_fill_style(c)?;
            Ok(true)
        }
        FillStrokeStyle::LinearGradient(g) => {
            let gradient = create_linear_gradient(g, rect)?;
            ctx.set_fill_style_gradient(gradient);
            Ok(true)
        }
        FillStrokeStyle::RadialGradient(g) => {
            let gradient = create_radial_gradient(g, rect)?;
            ctx.set_fill_style_gradient(gradient);
            Ok(true)
        }
        FillStrokeStyle::Pattern(p) => {
            let pattern = create_pattern_from_style(ctx, p)?;
            ctx.set_fill_style_pattern(pattern);
            Ok(true)
        }
    }
}

pub fn apply_stroke_style(
    ctx: &mut dyn crate::canvas::backend::CanvasContext,
    stroke: &FillStrokeStyle,
    rect: &BoundingRect,
) -> Result<bool, BackendError> {
    match stroke {
        FillStrokeStyle::None => Ok(false),
        FillStrokeStyle::Color(c) if c == "none" || c.is_empty() => Ok(false),
        FillStrokeStyle::Color(c) => {
            ctx.set_stroke_style(c)?;
            Ok(true)
        }
        FillStrokeStyle::LinearGradient(g) => {
            let gradient = create_linear_gradient(g, rect)?;
            ctx.set_stroke_style_gradient(gradient);
            Ok(true)
        }
        FillStrokeStyle::RadialGradient(g) => {
            let gradient = create_radial_gradient(g, rect)?;
            ctx.set_stroke_style_gradient(gradient);
            Ok(true)
        }
        FillStrokeStyle::Pattern(p) => {
            let pattern = create_pattern_from_style(ctx, p)?;
            ctx.set_stroke_style_pattern(pattern);
            Ok(true)
        }
    }
}

fn create_pattern_from_style(
    ctx: &dyn crate::canvas::backend::CanvasContext,
    pattern: &PatternStyle,
) -> Result<Arc<CanvasPattern>, BackendError> {
    let canvas_pattern = ctx.create_pattern(
        &pattern.data,
        pattern.width,
        pattern.height,
        &pattern.repeat,
    )?;

    let mut owned = (*canvas_pattern).clone();
    if pattern_needs_transform(pattern) {
        owned.set_transform(pattern_transform_matrix(pattern));
    }
    Ok(Arc::new(owned))
}

fn pattern_needs_transform(p: &PatternStyle) -> bool {
    p.x != 0.0
        || p.y != 0.0
        || p.rotation != 0.0
        || p.scale_x != 1.0
        || p.scale_y != 1.0
}

fn pattern_transform_matrix(p: &PatternStyle) -> vl_convert_canvas2d::DOMMatrix {
    use crate::core::matrix::{create, rotate, scale, to_dom_matrix, translate};

    let mut m = create();
    let mut tmp = create();

    translate(&mut tmp, &m, p.x as f32, p.y as f32);
    m = tmp;

    if p.rotation != 0.0 {
        rotate(&mut tmp, &m, p.rotation as f32);
        m = tmp;
    }

    if p.scale_x != 1.0 || p.scale_y != 1.0 {
        scale(&mut tmp, &m, p.scale_x as f32, p.scale_y as f32);
        m = tmp;
    }

    to_dom_matrix(&m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::canvas::backend::{CanvasBackend, CanvasContext, VlConvertBackend};
    use crate::graphic::style::{ColorStop, FillStrokeStyle, PatternStyle, RadialGradientStyle};

    #[test]
    fn radial_gradient_with_r0() {
        let mut backend = VlConvertBackend::new(80, 80).unwrap();
        backend.clear();
        let rect = BoundingRect::new(0.0, 0.0, 80.0, 80.0);
        let gradient = create_radial_gradient(
            &RadialGradientStyle {
                x: 0.5,
                y: 0.5,
                r: 0.5,
                r0: 0.2,
                color_stops: vec![
                    ColorStop {
                        offset: 0.0,
                        color: "#fff".to_string(),
                    },
                    ColorStop {
                        offset: 1.0,
                        color: "#5470c6".to_string(),
                    },
                ],
                global: false,
            },
            &rect,
        )
        .unwrap();
        backend.set_fill_style_gradient(gradient);
        backend.fill_rect(0.0, 0.0, 80.0, 80.0);
        let rgba = backend.get_rgba();
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }

    #[test]
    fn pattern_fill() {
        let mut backend = VlConvertBackend::new(40, 40).unwrap();
        backend.clear();
        let mut data = vec![0u8; 8 * 8 * 4];
        for (i, px) in data.chunks_mut(4).enumerate() {
            px[0] = if i % 2 == 0 { 200 } else { 50 };
            px[1] = 100;
            px[2] = 50;
            px[3] = 255;
        }
        let pattern = PatternStyle {
            data: Arc::from(data),
            width: 8,
            height: 8,
            repeat: "repeat".to_string(),
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
        };
        apply_fill_style(
            &mut backend,
            &FillStrokeStyle::Pattern(pattern),
            &BoundingRect::new(0.0, 0.0, 40.0, 40.0),
        )
        .unwrap();
        backend.fill_rect(0.0, 0.0, 40.0, 40.0);
        let rgba = backend.get_rgba();
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }
}
