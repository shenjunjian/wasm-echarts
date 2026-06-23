//! brush：将 Path 绘制到 CanvasContext

use crate::canvas::backend::{BackendError, CanvasContext};
use crate::canvas::helper::{apply_fill_style, apply_stroke_style};
use crate::core::bbox::BoundingRect;
use crate::element::REDRAW_BIT;
use crate::storage::Storage;

pub struct BrushScope {
    pub view_width: f64,
    pub view_height: f64,
}

impl BrushScope {
    pub fn new(view_width: f64, view_height: f64) -> Self {
        Self {
            view_width,
            view_height,
        }
    }
}

pub fn brush(
    ctx: &mut dyn CanvasContext,
    storage: &mut Storage,
    path_index: usize,
    scope: &BrushScope,
) -> Result<(), BackendError> {
    if !storage.path_mut(path_index).should_be_painted(scope.view_width, scope.view_height) {
        storage.path_mut(path_index).base.dirty &= !REDRAW_BIT;
        return Ok(());
    }

    storage.path_mut(path_index).ensure_path();

    if let Some(clip_idx) = storage.path(path_index).clip_path {
        apply_clip_path(ctx, storage, clip_idx)?;
    }

    let transform = *storage.path(path_index).base.transform();
    ctx.save();
    ctx.set_transform(&transform);

    let bbox = storage
        .path_mut(path_index)
        .bounding_rect()
        .cloned()
        .unwrap_or_else(|| BoundingRect::new(0.0, 0.0, scope.view_width, scope.view_height));

    let shadow = storage.path(path_index).style.shadow.clone();
    let has_fill = storage.path(path_index).style.has_fill();
    let has_stroke = storage.path(path_index).style.has_stroke();
    let line_width = storage.path(path_index).style.line_width;
    let stroke_first = storage.path(path_index).style.stroke_first;

    if let Some(shadow) = &shadow {
        if shadow.is_active() && (has_fill || has_stroke) {
            ctx.draw_shadow(
                storage.path(path_index).path_proxy(),
                &transform,
                shadow,
                has_fill,
                has_stroke,
                line_width,
            )?;
        }
    }

    let mut draw_fill = has_fill;
    let mut draw_stroke = has_stroke;

    if draw_fill {
        let fill = storage.path(path_index).style.fill.clone();
        draw_fill = apply_fill_style(ctx, &fill, &bbox)?;
        if draw_fill {
            let alpha = storage.path(path_index).style.effective_fill_opacity();
            ctx.set_global_alpha(alpha);
        }
    }

    if draw_stroke {
        let stroke = storage.path(path_index).style.stroke.clone();
        draw_stroke = apply_stroke_style(ctx, &stroke, &bbox)?;
        if draw_stroke {
            let style = &storage.path(path_index).style;
            ctx.set_line_width(style.line_width);
            ctx.set_line_cap(style.line_cap);
            ctx.set_line_join(style.line_join);
            if let Some(dash) = &style.line_dash {
                ctx.set_line_dash(dash.clone());
                ctx.set_line_dash_offset(style.line_dash_offset);
            }
            ctx.set_global_alpha(style.effective_stroke_opacity());
        }
    }

    let path = storage.path(path_index);
    if stroke_first {
        if draw_stroke {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.stroke();
        }
        if draw_fill {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.fill();
        }
    } else {
        if draw_fill {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.fill();
        }
        if draw_stroke {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.stroke();
        }
    }

    ctx.restore();

    storage.path_mut(path_index).base.dirty = 0;
    Ok(())
}

fn apply_clip_path(
    ctx: &mut dyn CanvasContext,
    storage: &mut Storage,
    clip_idx: usize,
) -> Result<(), BackendError> {
    storage.path_mut(clip_idx).ensure_path();
    let transform = *storage.path(clip_idx).base.transform();
    ctx.save();
    ctx.set_transform(&transform);
    ctx.begin_path();
    storage.path(clip_idx).path_proxy().replay(ctx);
    ctx.clip();
    ctx.restore();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::backend::{CanvasBackend, VlConvertBackend};
    use crate::graphic::path::Path;
    use crate::graphic::shapes::{RectShape, Shape};
    use crate::graphic::style::{FillStrokeStyle, PathStyle, ShadowStyle};

    #[test]
    fn brush_with_shadow_and_gradient() {
        let mut storage = Storage::new();
        let mut backend = VlConvertBackend::new(120, 80).unwrap();
        backend.clear();

        let idx = storage.create_path(Path::new(
            Shape::Rect(RectShape {
                x: 20.0,
                y: 15.0,
                width: 80.0,
                height: 50.0,
            }),
            PathStyle {
                fill: FillStrokeStyle::color("#5470c6"),
                shadow: Some(ShadowStyle {
                    color: "rgba(0,0,0,0.4)".to_string(),
                    blur: 6.0,
                    offset_x: 3.0,
                    offset_y: 3.0,
                }),
                ..Default::default()
            },
        ));

        let scope = BrushScope::new(120.0, 80.0);
        brush(&mut backend, &mut storage, idx, &scope).unwrap();
        let rgba = backend.get_rgba();
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }
}
