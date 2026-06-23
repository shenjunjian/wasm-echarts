//! 文本 brush

use crate::canvas::backend::{BackendError, CanvasContext};
use crate::graphic::text::Text;

pub fn brush_text(ctx: &mut dyn CanvasContext, text: &Text) -> Result<(), BackendError> {
    ctx.save();
    ctx.set_transform(text.base.transform());

    let font = format!("{}px sans-serif", text.style.font_size);
    ctx.set_font(&font);
    ctx.set_text_align(text.style.align);
    ctx.set_text_baseline(text.style.baseline);
    ctx.set_fill_style(&text.style.fill)?;
    let _ = ctx.fill_text(&text.content, text.x as f32, text.y as f32);

    ctx.restore();
    Ok(())
}

pub fn text_sort_key(text: &Text) -> (f64, f64, f64) {
    (
        text.displayable.zlevel,
        text.displayable.z,
        text.displayable.z2,
    )
}
