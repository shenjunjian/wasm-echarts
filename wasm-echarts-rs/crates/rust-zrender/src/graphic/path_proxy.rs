//! PathProxy：记录路径命令并回放到 CanvasContext

use vl_convert_canvas2d::{ArcParams, CubicBezierParams, QuadraticBezierParams, RectParams};

use crate::canvas::backend::CanvasContext;

#[derive(Debug, Clone)]
pub enum PathCmd {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    CubicBezier(CubicBezierParams),
    QuadraticBezier(QuadraticBezierParams),
    Arc(ArcParams),
    Rect(RectParams),
    ClosePath,
}

#[derive(Debug, Clone, Default)]
pub struct PathProxy {
    commands: Vec<PathCmd>,
}

impl PathProxy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_path(&mut self) {
        self.commands.clear();
    }

    pub fn commands(&self) -> &[PathCmd] {
        &self.commands
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCmd::MoveTo(x, y));
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCmd::LineTo(x, y));
    }

    pub fn cubic_bezier_to(&mut self, params: CubicBezierParams) {
        self.commands.push(PathCmd::CubicBezier(params));
    }

    pub fn quadratic_curve_to(&mut self, params: QuadraticBezierParams) {
        self.commands.push(PathCmd::QuadraticBezier(params));
    }

    pub fn arc(&mut self, params: ArcParams) {
        self.commands.push(PathCmd::Arc(params));
    }

    pub fn rect(&mut self, params: &RectParams) {
        self.commands.push(PathCmd::Rect(*params));
    }

    pub fn close_path(&mut self) {
        self.commands.push(PathCmd::ClosePath);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn replay(&self, ctx: &mut dyn CanvasContext) {
        for cmd in &self.commands {
            match cmd {
                PathCmd::MoveTo(x, y) => ctx.move_to(*x, *y),
                PathCmd::LineTo(x, y) => ctx.line_to(*x, *y),
                PathCmd::CubicBezier(p) => ctx.cubic_bezier_to(p),
                PathCmd::QuadraticBezier(p) => ctx.quadratic_curve_to(p),
                PathCmd::Arc(p) => ctx.arc(p),
                PathCmd::Rect(p) => ctx.rect(p),
                PathCmd::ClosePath => ctx.close_path(),
            }
        }
    }
}
