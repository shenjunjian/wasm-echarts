//! 阶段 0 demo 场景：绘制 Rect + Circle，验证后端闭环

use super::backend::{BackendError, CanvasBackend};

/// 在指定后端上绘制 demo 图形（填充矩形 + 描边圆）
pub fn render_demo_shapes(backend: &mut impl CanvasBackend) -> Result<(), BackendError> {
    backend.clear();

    backend.set_fill_style("#5470c6")?;
    backend.fill_rect(40.0, 30.0, 120.0, 80.0);

    backend.set_stroke_style("#ee6666")?;
    backend.set_line_width(4.0);
    backend.stroke_circle(150.0, 100.0, 45.0);

    backend.set_fill_style("rgba(145, 204, 117, 0.6)")?;
    backend.fill_circle(150.0, 100.0, 30.0);

    Ok(())
}
