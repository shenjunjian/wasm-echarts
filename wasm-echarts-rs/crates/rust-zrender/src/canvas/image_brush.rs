//! Image 绘制

use crate::canvas::brush::BrushScope;
use crate::canvas::image::draw_image_rgba;
use crate::canvas::backend::{BackendError, CanvasContext};
use crate::element::REDRAW_BIT;
use crate::storage::Storage;

pub fn brush_image(
    ctx: &mut dyn CanvasContext,
    storage: &mut Storage,
    image_index: usize,
    scope: &BrushScope,
) -> Result<(), BackendError> {
    let image = storage.image(image_index);
    if !image.should_be_painted(scope.view_width, scope.view_height) {
        storage.image_mut(image_index).base.dirty &= !REDRAW_BIT;
        return Ok(());
    }

    let transform = *image.base.transform();
    let x = image.style.x;
    let y = image.style.y;
    let dw = image.draw_width() as f32;
    let dh = image.draw_height() as f32;
    let opacity = image.style.opacity;

    ctx.save();
    ctx.set_transform(&transform);
    ctx.set_global_alpha(opacity);

    draw_image_rgba(
        ctx,
        &image.style.data,
        image.style.source_width,
        image.style.source_height,
        x as f32,
        y as f32,
        Some(dw),
        Some(dh),
    )?;

    ctx.restore();
    storage.image_mut(image_index).base.dirty = 0;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::canvas::backend::{CanvasBackend, VlConvertBackend};
    use crate::graphic::group::ChildRef;
    use crate::graphic::image::{Image, ImageStyle};

    #[test]
    fn brush_image_outputs_pixels() {
        let mut storage = Storage::new();
        let mut data = vec![255u8, 0, 0, 255];
        data.extend(std::iter::repeat_n(0u8, 4 * 4 * 4 - 4));
        let idx = storage.create_image(Image::new(ImageStyle {
            x: 10.0,
            y: 10.0,
            width: Some(40.0),
            height: Some(40.0),
            data: Arc::from(data),
            source_width: 4,
            source_height: 4,
            ..Default::default()
        }));
        storage.add_root(ChildRef::Image(idx));

        let mut backend = VlConvertBackend::new(80, 80).unwrap();
        backend.clear();
        let scope = BrushScope::new(80.0, 80.0);
        brush_image(
            &mut backend as &mut dyn CanvasContext,
            &mut storage,
            idx,
            &scope,
        )
        .unwrap();

        let rgba = backend.get_rgba();
        assert!(rgba.chunks(4).any(|px| px[0] > 200 && px[3] > 0));
    }
}
