//! brush：将 Path 绘制到 CanvasContext

use crate::canvas::backend::{BackendError, CanvasContext};
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

    let has_fill = path.style.has_fill();
    let has_stroke = path.style.has_stroke();

    if has_fill {
        if let Some(fill) = &path.style.fill {
            ctx.set_fill_style(fill)?;
        }
        ctx.set_global_alpha(path.style.effective_fill_opacity());
    }

    ctx.begin_path();
    path.path_proxy().replay(ctx);

    if has_fill {
        ctx.fill();
    }

    if has_stroke {
        if let Some(stroke) = &path.style.stroke {
            ctx.set_stroke_style(stroke)?;
        }
        ctx.set_line_width(path.style.line_width);
        ctx.set_line_cap(path.style.line_cap);
        ctx.set_line_join(path.style.line_join);
        if let Some(dash) = &path.style.line_dash {
            ctx.set_line_dash(dash.clone());
            ctx.set_line_dash_offset(path.style.line_dash_offset);
        }
        ctx.set_global_alpha(path.style.effective_stroke_opacity());
        ctx.begin_path();
        path.path_proxy().replay(ctx);
        ctx.stroke();
    }

    ctx.restore();

    path.base.dirty = 0;
    Ok(())
}

pub fn brush_single(ctx: &mut dyn CanvasContext, path: &mut Path, scope: &BrushScope) -> Result<(), BackendError> {
    brush(ctx, path, scope)
}
