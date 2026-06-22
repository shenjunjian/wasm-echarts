//! 阶段 2 demo：渐变、虚线、阴影

use super::backend::{BackendError, CanvasBackend};
use crate::canvas::helper::{apply_fill_style, create_linear_gradient};
use crate::core::bbox::BoundingRect;
use crate::graphic::style::{ColorStop, FillStrokeStyle, LinearGradientStyle};

/// 在指定后端上绘制 demo 图形（渐变矩形 + 虚线圆 + 阴影）
pub fn render_demo_shapes(backend: &mut impl CanvasBackend) -> Result<(), BackendError> {
    backend.clear();

    let gradient_rect = BoundingRect::new(40.0, 30.0, 120.0, 80.0);
    let gradient = create_linear_gradient(
        &LinearGradientStyle {
            x: 0.0,
            y: 0.0,
            x2: 1.0,
            y2: 1.0,
            color_stops: vec![
                ColorStop {
                    offset: 0.0,
                    color: "#5470c6".to_string(),
                },
                ColorStop {
                    offset: 1.0,
                    color: "#91cc75".to_string(),
                },
            ],
            global: false,
        },
        &gradient_rect,
    )?;
    backend.set_fill_style_gradient(gradient);
    backend.fill_rect(40.0, 30.0, 120.0, 80.0);

    backend.set_stroke_style("#ee6666")?;
    backend.set_line_width(4.0);
    backend.set_line_dash(vec![8.0, 4.0]);
    backend.stroke_circle(150.0, 100.0, 45.0);

    backend.set_line_dash(vec![]);
    apply_fill_style(
        backend,
        &FillStrokeStyle::color("rgba(145, 204, 117, 0.6)"),
        &BoundingRect::new(120.0, 70.0, 60.0, 60.0),
    )?;
    backend.fill_circle(150.0, 100.0, 30.0);

    Ok(())
}
