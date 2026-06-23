//! drawImage 辅助（JS 传入 RGBA bytes）

use crate::canvas::backend::{BackendError, CanvasContext};

/// 绘制 RGBA 图像到 canvas
pub fn draw_image_rgba(
    ctx: &mut dyn CanvasContext,
    data: &[u8],
    width: u32,
    height: u32,
    dx: f32,
    dy: f32,
    dw: Option<f32>,
    dh: Option<f32>,
) -> Result<(), BackendError> {
    let dw = dw.unwrap_or(width as f32);
    let dh = dh.unwrap_or(height as f32);
    ctx.draw_image_rgba(data, width, height, dx, dy, dw, dh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::backend::{CanvasBackend, VlConvertBackend};

    #[test]
    fn draw_small_image() {
        let mut backend = VlConvertBackend::new(50, 50).unwrap();
        backend.clear();
        let mut data = vec![0u8; 4 * 4 * 4];
        for px in data.chunks_mut(4) {
            px[0] = 255;
            px[1] = 0;
            px[2] = 0;
            px[3] = 255;
        }
        draw_image_rgba(&mut backend, &data, 4, 4, 10.0, 10.0, None, None).unwrap();
        let rgba = backend.get_rgba();
        assert!(rgba.chunks(4).any(|px| px[0] > 200 && px[3] > 0));
    }
}
