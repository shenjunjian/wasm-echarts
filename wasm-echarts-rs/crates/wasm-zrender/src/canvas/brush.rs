//! brush：将 Path 绘制到 CanvasContext

use crate::canvas::backend::{BackendError, CanvasContext};
use crate::canvas::helper::{apply_fill_style, apply_stroke_style};
use crate::core::bbox::BoundingRect;
use crate::element::REDRAW_BIT;
use crate::graphic::path::Path;

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

pub fn brush(ctx: &mut dyn CanvasContext, path: &mut Path, scope: &BrushScope) -> Result<(), BackendError> {
    if !path.should_be_painted(scope.view_width, scope.view_height) {
        path.base.dirty &= !REDRAW_BIT;
        return Ok(());
    }

    path.ensure_path();

    ctx.save();
    ctx.set_transform(path.base.transform());

    let mut has_fill = path.style.has_fill();
    let mut has_stroke = path.style.has_stroke();

    let bbox = path
        .bounding_rect()
        .cloned()
        .unwrap_or_else(|| BoundingRect::new(0.0, 0.0, scope.view_width, scope.view_height));

    if let Some(shadow) = &path.style.shadow {
        if shadow.is_active() && (has_fill || has_stroke) {
            ctx.draw_shadow(
                path.path_proxy(),
                path.base.transform(),
                shadow,
                has_fill,
                has_stroke,
                path.style.line_width,
            )?;
        }
    }

    if has_fill {
        has_fill = apply_fill_style(ctx, &path.style.fill, &bbox)?;
        if has_fill {
            ctx.set_global_alpha(path.style.effective_fill_opacity());
        }
    }

    if has_stroke {
        has_stroke = apply_stroke_style(ctx, &path.style.stroke, &bbox)?;
        if has_stroke {
            ctx.set_line_width(path.style.line_width);
            ctx.set_line_cap(path.style.line_cap);
            ctx.set_line_join(path.style.line_join);
            if let Some(dash) = &path.style.line_dash {
                ctx.set_line_dash(dash.clone());
                ctx.set_line_dash_offset(path.style.line_dash_offset);
            }
            ctx.set_global_alpha(path.style.effective_stroke_opacity());
        }
    }

    if path.style.stroke_first {
        if has_stroke {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.stroke();
        }
        if has_fill {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.fill();
        }
    } else {
        if has_fill {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.fill();
        }
        if has_stroke {
            ctx.begin_path();
            path.path_proxy().replay(ctx);
            ctx.stroke();
        }
    }

    ctx.restore();

    path.base.dirty = 0;
    Ok(())
}

pub fn brush_single(ctx: &mut dyn CanvasContext, path: &mut Path, scope: &BrushScope) -> Result<(), BackendError> {
    brush(ctx, path, scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::backend::{CanvasBackend, VlConvertBackend};
    use crate::graphic::shapes::{RectShape, Shape};
    use crate::graphic::style::{FillStrokeStyle, PathStyle, ShadowStyle};

    #[test]
    fn brush_with_shadow_and_gradient() {
        let mut backend = VlConvertBackend::new(120, 80).unwrap();
        backend.clear();

        let mut path = Path::new(
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
        );

        let scope = BrushScope::new(120.0, 80.0);
        brush(&mut backend, &mut path, &scope).unwrap();
        let rgba = backend.get_rgba();
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
    }
}
