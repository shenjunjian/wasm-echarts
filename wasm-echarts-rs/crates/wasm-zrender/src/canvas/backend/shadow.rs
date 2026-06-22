//! 阴影 pass：vl-convert 不支持 shadow 时，用 tiny-skia 手动实现

use csscolorparser::parse;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use crate::canvas::backend::BackendError;
use crate::core::matrix::Matrix;
use crate::graphic::path_proxy::{PathCmd, PathProxy};
use crate::graphic::style::ShadowStyle;

/// 阴影渲染器
pub struct ShadowPass;

impl ShadowPass {
    /// 将路径阴影渲染到 RGBA buffer（非预乘 alpha），供 draw_image_rgba 合成
    pub fn render(
        width: u32,
        height: u32,
        path: &PathProxy,
        transform: &Matrix,
        shadow: &ShadowStyle,
        fill: bool,
        stroke: bool,
        line_width: f32,
    ) -> Result<Vec<u8>, BackendError> {
        if !shadow.is_active() {
            return Ok(vec![0u8; (width * height * 4) as usize]);
        }

        let mut pixmap = Pixmap::new(width, height)
            .ok_or_else(|| BackendError::Canvas("failed to create shadow pixmap".into()))?;

        let sk_transform = matrix_to_skia(transform);
        let sk_path = build_skia_path(path, sk_transform)
            .ok_or_else(|| BackendError::Canvas("failed to build shadow path".into()))?;

        let color = parse_shadow_color(&shadow.color)?;
        let mut paint = Paint::default();
        paint.set_color(color);

        if fill {
            pixmap.fill_path(&sk_path, &paint, FillRule::Winding, Transform::identity(), None);
        }

        if stroke && line_width > 0.0 {
            let stroke = tiny_skia::Stroke {
                width: line_width,
                ..Default::default()
            };
            pixmap.stroke_path(&sk_path, &paint, &stroke, Transform::identity(), None);
        }

        let blur_radius = shadow.blur.max(0.0) as u32;
        if blur_radius > 0 {
            box_blur_rgba(pixmap.data_mut(), width, height, blur_radius);
        }

        apply_offset(pixmap.data_mut(), width, height, shadow.offset_x, shadow.offset_y);

        Ok(pixmap.data().to_vec())
    }
}

fn parse_shadow_color(s: &str) -> Result<tiny_skia::Color, BackendError> {
    let parsed = parse(s).map_err(|e| BackendError::Canvas(format!("shadow color: {e}")))?;
    let [r, g, b, a] = parsed.to_array();
    Ok(tiny_skia::Color::from_rgba8(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ))
}

fn matrix_to_skia(m: &Matrix) -> Transform {
    Transform::from_row(
        m[0] as f32,
        m[1] as f32,
        m[2] as f32,
        m[3] as f32,
        m[4] as f32,
        m[5] as f32,
    )
}

fn map_xy(t: Transform, x: f32, y: f32) -> (f32, f32) {
    let mut p = tiny_skia::Point { x, y };
    t.map_point(&mut p);
    (p.x, p.y)
}

fn build_skia_path(path: &PathProxy, transform: Transform) -> Option<tiny_skia::Path> {
    let mut builder = PathBuilder::new();
    let mut has_subpath = false;

    for cmd in path.commands() {
        match cmd {
            PathCmd::MoveTo(x, y) => {
                let (x, y) = map_xy(transform, *x, *y);
                builder.move_to(x, y);
                has_subpath = true;
            }
            PathCmd::LineTo(x, y) => {
                let (x, y) = map_xy(transform, *x, *y);
                builder.line_to(x, y);
            }
            PathCmd::CubicBezier(params) => {
                let (cp1x, cp1y) = map_xy(transform, params.cp1x, params.cp1y);
                let (cp2x, cp2y) = map_xy(transform, params.cp2x, params.cp2y);
                let (x, y) = map_xy(transform, params.x, params.y);
                builder.cubic_to(cp1x, cp1y, cp2x, cp2y, x, y);
            }
            PathCmd::QuadraticBezier(params) => {
                let (cpx, cpy) = map_xy(transform, params.cpx, params.cpy);
                let (x, y) = map_xy(transform, params.x, params.y);
                builder.quad_to(cpx, cpy, x, y);
            }
            PathCmd::Arc(params) => {
                append_arc(&mut builder, params, transform);
            }
            PathCmd::Rect(params) => {
                let (x0, y0) = map_xy(transform, params.x, params.y);
                let (x1, y1) = map_xy(transform, params.x + params.width, params.y);
                let (x2, y2) = map_xy(transform, params.x + params.width, params.y + params.height);
                let (x3, y3) = map_xy(transform, params.x, params.y + params.height);
                builder.move_to(x0, y0);
                builder.line_to(x1, y1);
                builder.line_to(x2, y2);
                builder.line_to(x3, y3);
                builder.close();
                has_subpath = true;
            }
            PathCmd::ClosePath => {
                builder.close();
            }
        }
    }

    if !has_subpath {
        return None;
    }
    builder.finish()
}

fn append_arc(
    builder: &mut PathBuilder,
    params: &vl_convert_canvas2d::ArcParams,
    transform: Transform,
) {
    let steps = 32;
    let start = params.start_angle;
    let end = params.end_angle;
    let mut delta = end - start;
    if params.anticlockwise {
        if delta > 0.0 {
            delta -= std::f32::consts::TAU;
        }
    } else if delta < 0.0 {
        delta += std::f32::consts::TAU;
    }

    for i in 0..=steps {
        let t = start + delta * (i as f32 / steps as f32);
        let x = params.x + params.radius * t.cos();
        let y = params.y + params.radius * t.sin();
        let (x, y) = map_xy(transform, x, y);
        if i == 0 {
            builder.move_to(x, y);
        } else {
            builder.line_to(x, y);
        }
    }
}

/// 可分离 box blur（近似 shadowBlur）
fn box_blur_rgba(data: &mut [u8], width: u32, height: u32, radius: u32) {
    if radius == 0 {
        return;
    }
    let w = width as usize;
    let h = height as usize;
    let r = radius as usize;
    let mut tmp = data.to_vec();

    for y in 0..h {
        for x in 0..w {
            let mut sum = [0u32; 4];
            let mut count = 0u32;
            for dx in -(r as i32)..=(r as i32) {
                let sx = x as i32 + dx;
                if sx >= 0 && (sx as usize) < w {
                    let idx = (y * w + sx as usize) * 4;
                    for c in 0..4 {
                        sum[c] += tmp[idx + c] as u32;
                    }
                    count += 1;
                }
            }
            let idx = (y * w + x) * 4;
            for c in 0..4 {
                data[idx + c] = (sum[c] / count.max(1)) as u8;
            }
        }
    }

    tmp.copy_from_slice(data);

    for y in 0..h {
        for x in 0..w {
            let mut sum = [0u32; 4];
            let mut count = 0u32;
            for dy in -(r as i32)..=(r as i32) {
                let sy = y as i32 + dy;
                if sy >= 0 && (sy as usize) < h {
                    let idx = (sy as usize * w + x) * 4;
                    for c in 0..4 {
                        sum[c] += tmp[idx + c] as u32;
                    }
                    count += 1;
                }
            }
            let idx = (y * w + x) * 4;
            for c in 0..4 {
                data[idx + c] = (sum[c] / count.max(1)) as u8;
            }
        }
    }
}

fn apply_offset(data: &mut [u8], width: u32, height: u32, offset_x: f32, offset_y: f32) {
    let dx = offset_x.round() as i32;
    let dy = offset_y.round() as i32;
    if dx == 0 && dy == 0 {
        return;
    }

    let w = width as usize;
    let h = height as usize;
    let mut shifted = vec![0u8; data.len()];

    for y in 0..h {
        for x in 0..w {
            let tx = x as i32 + dx;
            let ty = y as i32 + dy;
            if tx >= 0 && tx < w as i32 && ty >= 0 && ty < h as i32 {
                let src = (y * w + x) * 4;
                let dst = (ty as usize * w + tx as usize) * 4;
                shifted[dst..dst + 4].copy_from_slice(&data[src..src + 4]);
            }
        }
    }

    data.copy_from_slice(&shifted);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic::path_proxy::PathProxy;
    use vl_convert_canvas2d::RectParams;

    #[test]
    fn shadow_pass_produces_non_empty_pixels() {
        let mut proxy = PathProxy::new();
        proxy.rect(&RectParams {
            x: 50.0,
            y: 50.0,
            width: 100.0,
            height: 60.0,
        });

        let rgba = ShadowPass::render(
            200,
            150,
            &proxy,
            &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            &ShadowStyle::default(),
            true,
            false,
            0.0,
        )
        .unwrap();

        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }
}
